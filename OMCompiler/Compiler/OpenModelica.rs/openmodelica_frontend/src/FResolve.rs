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

use crate::FGraphBuild;
use crate::FLookup;
use crate::FNode;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::SCode;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

// public imports
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

pub type Extra = FCore::Extra;

pub type Visited = FCore::Visited;

pub type Import = Absyn::Import;

pub type Graph = FCore::Graph;

pub type Msg = Option<SourceInfo>;

pub(crate) fn ext(mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = (match ig {
        mut g => {
            g = FNode::apply1(
                inRef,
                &move |__a0: ArcStr,
                       __a1: Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
                       __a2: FCore::Graph|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(ext_one(&__a0, __a1, __a2))
                },
                g,
            )?;
            g
        }
    });
    Ok(og)
}

pub(crate) fn ext_one(mut name: &Name, mut inRef: Ref, mut ig: Graph) -> Graph {
    let mut og: Graph;
    og = 'mc: {
        let __mc_input = (inRef, ig.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (FNode::isRefExtends(r.clone())) else {
                return Err("pattern mismatch");
            };
            let false = (FNode::isRefDerived(r.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (FNode::isRefRefResolved(r.clone())) else {
                return Err("pattern mismatch");
            };
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut rr: Ref;
            let mut p: metamodelica::Ref<Absyn::Path>;
            let mut e: metamodelica::Ref<SCode::Element>;
            let true = (FNode::isRefExtends(r.clone())) else {
                return Err("pattern mismatch");
            };
            let false = (FNode::isRefDerived(r.clone())) else {
                return Err("pattern mismatch");
            };
            let __pa0 = ::match_deref::match_deref! { match &(FNode::refData(r.clone())) {
                Deref @ FCore::Data::EX { e: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            p = SCodeUtil::getBaseClassPath(&e)?;
            (g, rr) = FLookup::name(
                g.clone(),
                r.clone(),
                &p,
                FLookup::ignoreNothing.clone(),
                FLookup::dummyLookupOption.clone(),
            )?;
            g = FGraphBuild::mkRefNode(
                arcstr::literal!(FNode::refNodeName),
                list![rr.clone()],
                r.clone(),
                g.clone(),
            )?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut p: metamodelica::Ref<Absyn::Path>;
            let mut e: metamodelica::Ref<SCode::Element>;
            let true = (FNode::isRefExtends(r.clone())) else {
                return Err("pattern mismatch");
            };
            let false = (FNode::isRefDerived(r.clone())) else {
                return Err("pattern mismatch");
            };
            let __pa0 = ::match_deref::match_deref! { match &(FNode::refData(r.clone())) {
                Deref @ FCore::Data::EX { e: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            p = SCodeUtil::getBaseClassPath(&e)?;
            if '__try1: {
                unwrap_break_err!(FLookup::name(g.clone(), r.clone(), &p, FLookup::ignoreNothing.clone(), FLookup::dummyLookupOption.clone()), '__try1);
                Ok::<(), &'static str>(())
            }.is_ok() { return Err("failure(): body succeeded") }
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("FResolve.ext_one: baseclass: "));
                __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), literal!("."), true, false)?);
                __mm_s.push_str(&*literal!(" not found in: "));
                __mm_s.push_str(&*FNode::toPathStr(&(FNode::fromRef(r.clone())))?);
                __mm_s.push_str(&*literal!("!\n"));
                ArcStr::from(__mm_s)
            });
            g = FGraphBuild::mkRefNode(
                arcstr::literal!(FNode::refNodeName),
                metamodelica::nil(),
                r.clone(),
                g.clone(),
            )?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(ig.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    og
}

pub(crate) fn derived(mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = (match ig {
        mut g => {
            g = FNode::apply1(
                inRef,
                &move |__a0: ArcStr,
                       __a1: Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
                       __a2: FCore::Graph|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(derived_one(&__a0, __a1, __a2))
                },
                g,
            )?;
            g
        }
    });
    Ok(og)
}

pub(crate) fn derived_one(mut name: &Name, mut inRef: Ref, mut ig: Graph) -> Graph {
    let mut og: Graph;
    og = 'mc: {
        let __mc_input = (inRef, ig.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (FNode::isRefDerived(r.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (FNode::isRefRefResolved(r.clone())) else {
                return Err("pattern mismatch");
            };
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut rr: Ref;
            let mut p: metamodelica::Ref<Absyn::Path>;
            let true = (FNode::isRefDerived(r.clone())) else {
                return Err("pattern mismatch");
            };
            let __pa0 = ::match_deref::match_deref! { match &(FNode::refData(r.clone())) {
                Deref @ FCore::Data::CL { e: Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __pa0, arrayDim: _ }, .. }, .. }, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            p = metamodelica::Own::own(__pa0);
            (g, rr) = FLookup::name(
                g.clone(),
                r.clone(),
                &p,
                FLookup::ignoreNothing.clone(),
                FLookup::dummyLookupOption.clone(),
            )?;
            g = FGraphBuild::mkRefNode(
                arcstr::literal!(FNode::refNodeName),
                list![rr.clone()],
                r.clone(),
                g.clone(),
            )?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut p: metamodelica::Ref<Absyn::Path>;
            let true = (FNode::isRefDerived(r.clone())) else {
                return Err("pattern mismatch");
            };
            let __pa0 = ::match_deref::match_deref! { match &(FNode::refData(r.clone())) {
                Deref @ FCore::Data::CL { e: Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __pa0, arrayDim: _ }, .. }, .. }, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            p = metamodelica::Own::own(__pa0);
            if '__try2: {
                unwrap_break_err!(FLookup::name(g.clone(), r.clone(), &p, FLookup::ignoreNothing.clone(), FLookup::dummyLookupOption.clone()), '__try2);
                Ok::<(), &'static str>(())
            }.is_ok() { return Err("failure(): body succeeded") }
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("FResolve.derived_one: baseclass: "));
                __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), literal!("."), true, false)?);
                __mm_s.push_str(&*literal!(" not found in: "));
                __mm_s.push_str(&*FNode::toPathStr(&(FNode::fromRef(r.clone())))?);
                __mm_s.push_str(&*literal!("!\n"));
                ArcStr::from(__mm_s)
            });
            g = FGraphBuild::mkRefNode(
                arcstr::literal!(FNode::refNodeName),
                metamodelica::nil(),
                r.clone(),
                g.clone(),
            )?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(ig.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    og
}

pub(crate) fn ty(mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = (match ig {
        mut g => {
            g = FNode::apply1(
                inRef,
                &move |__a0: ArcStr,
                       __a1: Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
                       __a2: FCore::Graph|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(ty_one(&__a0, __a1, __a2))
                },
                g,
            )?;
            g
        }
    });
    Ok(og)
}

pub(crate) fn ty_one(mut name: &Name, mut inRef: Ref, mut ig: Graph) -> Graph {
    let mut og: Graph;
    og = 'mc: {
        let __mc_input = (inRef, ig.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (FNode::isRefComponent(r.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (FNode::isRefRefResolved(r.clone())) else {
                return Err("pattern mismatch");
            };
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut rr: Ref;
            let mut p: metamodelica::Ref<Absyn::Path>;
            let mut e: metamodelica::Ref<SCode::Element>;
            let true = (FNode::isRefComponent(r.clone())) else {
                return Err("pattern mismatch");
            };
            let __pa0 = ::match_deref::match_deref! { match &(FNode::refData(r.clone())) {
                Deref @ FCore::Data::CO { e: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            p = SCodeUtil::getElementTypePath(&e)?;
            (g, rr) = FLookup::name(
                g.clone(),
                r.clone(),
                &p,
                FLookup::ignoreNothing.clone(),
                FLookup::dummyLookupOption.clone(),
            )?;
            g = FGraphBuild::mkRefNode(
                arcstr::literal!(FNode::refNodeName),
                list![rr.clone()],
                r.clone(),
                g.clone(),
            )?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut p: metamodelica::Ref<Absyn::Path>;
            let mut e: metamodelica::Ref<SCode::Element>;
            let true = (FNode::isRefComponent(r.clone())) else {
                return Err("pattern mismatch");
            };
            let __pa0 = ::match_deref::match_deref! { match &(FNode::refData(r.clone())) {
                Deref @ FCore::Data::CO { e: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            p = SCodeUtil::getElementTypePath(&e)?;
            if '__try1: {
                unwrap_break_err!(FLookup::name(g.clone(), r.clone(), &p, FLookup::ignoreNothing.clone(), FLookup::dummyLookupOption.clone()), '__try1);
                Ok::<(), &'static str>(())
            }.is_ok() { return Err("failure(): body succeeded") }
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("FResolve.ty_one: component type path: "));
                __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), literal!("."), true, false)?);
                __mm_s.push_str(&*literal!(" not found in: "));
                __mm_s.push_str(&*FNode::toPathStr(&(FNode::fromRef(r.clone())))?);
                __mm_s.push_str(&*literal!("!\n"));
                ArcStr::from(__mm_s)
            });
            g = FGraphBuild::mkRefNode(
                arcstr::literal!(FNode::refNodeName),
                metamodelica::nil(),
                r.clone(),
                g.clone(),
            )?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(ig.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    og
}

pub(crate) fn cc(mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = (match ig {
        mut g => {
            g = FNode::apply1(
                inRef,
                &move |__a0: ArcStr,
                       __a1: Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
                       __a2: FCore::Graph|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(cc_one(&__a0, __a1, __a2))
                },
                g,
            )?;
            g
        }
    });
    Ok(og)
}

pub(crate) fn cc_one(mut name: &Name, mut inRef: Ref, mut ig: Graph) -> Graph {
    let mut og: Graph;
    og = 'mc: {
        let __mc_input = (inRef, ig.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (FNode::isRefConstrainClass(r.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (FNode::isRefRefResolved(r.clone())) else {
                return Err("pattern mismatch");
            };
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut rr: Ref;
            let mut p: metamodelica::Ref<Absyn::Path>;
            let true = (FNode::isRefConstrainClass(r.clone())) else {
                return Err("pattern mismatch");
            };
            let __pa0 = ::match_deref::match_deref! { match &(FNode::refData(r.clone())) {
                Deref @ FCore::Data::CC { cc: Deref @ SCode::ConstrainClass { constrainingClass: __pa0, .. } } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            p = metamodelica::Own::own(__pa0);
            (g, rr) = FLookup::name(
                g.clone(),
                r.clone(),
                &p,
                FLookup::ignoreNothing.clone(),
                FLookup::dummyLookupOption.clone(),
            )?;
            g = FGraphBuild::mkRefNode(
                arcstr::literal!(FNode::refNodeName),
                list![rr.clone()],
                r.clone(),
                g.clone(),
            )?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut p: metamodelica::Ref<Absyn::Path>;
            let true = (FNode::isRefConstrainClass(r.clone())) else {
                return Err("pattern mismatch");
            };
            let __pa0 = ::match_deref::match_deref! { match &(FNode::refData(r.clone())) {
                Deref @ FCore::Data::CC { cc: Deref @ SCode::ConstrainClass { constrainingClass: __pa0, .. } } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            p = metamodelica::Own::own(__pa0);
            if '__try2: {
                unwrap_break_err!(FLookup::name(g.clone(), r.clone(), &p, FLookup::ignoreNothing.clone(), FLookup::dummyLookupOption.clone()), '__try2);
                Ok::<(), &'static str>(())
            }.is_ok() { return Err("failure(): body succeeded") }
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("FResolve.cc_one: constrained class: "));
                __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), literal!("."), true, false)?);
                __mm_s.push_str(&*literal!(" not found in: "));
                __mm_s.push_str(&*FNode::toPathStr(&(FNode::fromRef(r.clone())))?);
                __mm_s.push_str(&*literal!("!\n"));
                ArcStr::from(__mm_s)
            });
            g = FGraphBuild::mkRefNode(
                arcstr::literal!(FNode::refNodeName),
                metamodelica::nil(),
                r.clone(),
                g.clone(),
            )?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(ig.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    og
}

pub(crate) fn clsext(mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = (match ig {
        mut g => {
            g = FNode::apply1(
                inRef,
                &move |__a0: ArcStr,
                       __a1: Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
                       __a2: FCore::Graph|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(clsext_one(&__a0, __a1, __a2))
                },
                g,
            )?;
            g
        }
    });
    Ok(og)
}

pub(crate) fn clsext_one(mut name: &Name, mut inRef: Ref, mut ig: Graph) -> Graph {
    let mut og: Graph;
    og = 'mc: {
        let __mc_input = (inRef, ig.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (FNode::isRefClassExtends(r.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (FNode::isRefRefResolved(r.clone())) else {
                return Err("pattern mismatch");
            };
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut rr: Ref;
            let mut p: Ref;
            let mut id: Name;
            let true = (FNode::isRefClassExtends(r.clone())) else {
                return Err("pattern mismatch");
            };
            let __pa0 = ::match_deref::match_deref! { match &(FNode::refData(r.clone())) {
                Deref @ FCore::Data::CL { e: Deref @ SCode::Element::CLASS { name: __pa0, .. }, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            id = metamodelica::Own::own(__pa0);
            p = FNode::contextualParent(&(FNode::fromRef(r.clone())))?;
            (g, rr) = FLookup::ext(
                g.clone(),
                p.clone(),
                &id,
                FLookup::ignoreParentsAndImports.clone(),
                FLookup::dummyLookupOption.clone(),
            )?;
            g = FGraphBuild::mkRefNode(
                arcstr::literal!(FNode::refNodeName),
                list![rr.clone()],
                r.clone(),
                g.clone(),
            )?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut p: Ref;
            let mut id: Name;
            let true = (FNode::isRefClassExtends(r.clone())) else {
                return Err("pattern mismatch");
            };
            let __pa0 = ::match_deref::match_deref! { match &(FNode::refData(r.clone())) {
                Deref @ FCore::Data::CL { e: Deref @ SCode::Element::CLASS { name: __pa0, .. }, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            id = metamodelica::Own::own(__pa0);
            p = FNode::contextualParent(&(FNode::fromRef(r.clone())))?;
            if '__try2: {
                unwrap_break_err!(FLookup::ext(g.clone(), p.clone(), &id, FLookup::ignoreParentsAndImports.clone(), FLookup::dummyLookupOption.clone()), '__try2);
                Ok::<(), &'static str>(())
            }.is_ok() { return Err("failure(): body succeeded") }
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("FResolve.clsext_one: class extends: "));
                __mm_s.push_str(&*id);
                __mm_s.push_str(&*literal!(" scope: "));
                __mm_s.push_str(&*FNode::toPathStr(&(FNode::fromRef(r.clone())))?);
                __mm_s.push_str(&*literal!(" not found in extends of: "));
                __mm_s.push_str(&*FNode::toPathStr(&(FNode::fromRef(p.clone())))?);
                __mm_s.push_str(&*literal!(":\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\t"));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(
                        List::map(
                            FNode::extendsRefs(p.clone())?,
                            &fnptr!(FNode::fromRef, Mutable::Mutable<metamodelica::Ref<FCore::Node>>),
                        )?,
                        &move |__a0: metamodelica::Ref<FCore::Node>| FNode::toPathStr(&__a0),
                    )?,
                    literal!("\n\t"),
                ));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            g = FGraphBuild::mkRefNode(
                arcstr::literal!(FNode::refNodeName),
                metamodelica::nil(),
                r.clone(),
                g.clone(),
            )?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(ig.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    og
}

pub(crate) fn cr(mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = (match ig {
        mut g => {
            g = FNode::apply1(
                inRef,
                &move |__a0: ArcStr,
                       __a1: Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
                       __a2: FCore::Graph|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(cr_one(&__a0, __a1, __a2))
                },
                g,
            )?;
            g
        }
    });
    Ok(og)
}

pub(crate) fn cr_one(mut name: &Name, mut inRef: Ref, mut ig: Graph) -> Graph {
    let mut og: Graph;
    og = 'mc: {
        let __mc_input = (inRef, ig.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (FNode::isRefCref(r.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (FNode::isRefRefResolved(r.clone())) else {
                return Err("pattern mismatch");
            };
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut rr: Ref;
            let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
            let true = (FNode::isRefCref(r.clone())) else {
                return Err("pattern mismatch");
            };
            let __pa0 = ::match_deref::match_deref! { match &(FNode::refData(r.clone())) {
                Deref @ FCore::Data::CR { r: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa0);
            (g, rr) = FLookup::cr(
                g.clone(),
                r.clone(),
                &cr,
                FLookup::ignoreNothing.clone(),
                FLookup::dummyLookupOption.clone(),
            )?;
            g = FGraphBuild::mkRefNode(
                arcstr::literal!(FNode::refNodeName),
                list![rr.clone()],
                r.clone(),
                g.clone(),
            )?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
            let true = (FNode::isRefCref(r.clone())) else {
                return Err("pattern mismatch");
            };
            let __pa0 = ::match_deref::match_deref! { match &(FNode::refData(r.clone())) {
                Deref @ FCore::Data::CR { r: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa0);
            if '__try1: {
                unwrap_break_err!(FLookup::cr(g.clone(), r.clone(), &cr, FLookup::ignoreNothing.clone(), FLookup::dummyLookupOption.clone()), '__try1);
                Ok::<(), &'static str>(())
            }.is_ok() { return Err("failure(): body succeeded") }
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("FResolve.cr_one: component reference: "));
                __mm_s.push_str(&*AbsynUtil::crefString(&cr)?);
                __mm_s.push_str(&*literal!(" not found in: "));
                __mm_s.push_str(&*FNode::toPathStr(&(FNode::fromRef(r.clone())))?);
                __mm_s.push_str(&*literal!("!\n"));
                ArcStr::from(__mm_s)
            });
            g = FGraphBuild::mkRefNode(
                arcstr::literal!(FNode::refNodeName),
                metamodelica::nil(),
                r.clone(),
                g.clone(),
            )?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(ig.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    og
}

pub(crate) fn r#mod(mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = (match ig {
        mut g => {
            g = FNode::apply1(
                inRef,
                &move |__a0: ArcStr,
                       __a1: Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
                       __a2: FCore::Graph|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(mod_one(&__a0, __a1, __a2))
                },
                g,
            )?;
            g
        }
    });
    Ok(og)
}

pub(crate) fn mod_one(mut name: &Name, mut inRef: Ref, mut ig: Graph) -> Graph {
    let mut og: Graph;
    og = 'mc: {
        let __mc_input = (inRef, ig.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (FNode::isRefMod(r.clone())
                && !(FNode::isRefModHolder(r.clone()))
                && !(ClassInfUtil::isBasicTypeComponentName(FNode::refName(r.clone()))))
            else {
                return Err("pattern mismatch");
            };
            let true = (FNode::isRefRefResolved(r.clone())) else {
                return Err("pattern mismatch");
            };
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut rr: Ref;
            let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
            let true = (FNode::isRefMod(r.clone())
                && !(FNode::isRefModHolder(r.clone()))
                && !(ClassInfUtil::isBasicTypeComponentName(FNode::refName(r.clone()))))
            else {
                return Err("pattern mismatch");
            };
            cr = AbsynUtil::pathToCref(
                &(AbsynUtil::stringListPath(FNode::namesUpToParentName(
                    r.clone(),
                    arcstr::literal!(FNode::modNodeName),
                )?)?),
            );
            (g, rr) = FLookup::cr(
                g.clone(),
                FNode::getModifierTarget(r.clone())?,
                &cr,
                FLookup::ignoreNothing.clone(),
                FLookup::dummyLookupOption.clone(),
            )?;
            g = FGraphBuild::mkRefNode(
                arcstr::literal!(FNode::refNodeName),
                list![rr.clone()],
                r.clone(),
                g.clone(),
            )?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
            let true = (FNode::isRefMod(r.clone())
                && !(FNode::isRefModHolder(r.clone()))
                && !(ClassInfUtil::isBasicTypeComponentName(FNode::refName(r.clone()))))
            else {
                return Err("pattern mismatch");
            };
            cr = AbsynUtil::pathToCref(
                &(AbsynUtil::stringListPath(FNode::namesUpToParentName(
                    r.clone(),
                    arcstr::literal!(FNode::modNodeName),
                )?)?),
            );
            if '__try0: {
                unwrap_break_err!(FLookup::cr(g.clone(), unwrap_break_err!(FNode::getModifierTarget(r.clone()), '__try0), &cr, FLookup::ignoreNothing.clone(), FLookup::dummyLookupOption.clone()), '__try0);
                Ok::<(), &'static str>(())
            }.is_ok() { return Err("failure(): body succeeded") }
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("FResolve.mod_one: modifier: "));
                __mm_s.push_str(&*AbsynUtil::crefString(&cr)?);
                __mm_s.push_str(&*literal!(" not found in: "));
                __mm_s.push_str(&*FNode::toPathStr(&(FNode::fromRef(r.clone())))?);
                __mm_s.push_str(&*literal!("!\n"));
                ArcStr::from(__mm_s)
            });
            g = FGraphBuild::mkRefNode(
                arcstr::literal!(FNode::refNodeName),
                metamodelica::nil(),
                r.clone(),
                g.clone(),
            )?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(ig.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    og
}

pub(crate) fn elred(mut inRef: Ref, mut ig: Graph) -> Result<Graph> {
    let mut og: Graph;
    og = (match ig {
        mut g => {
            g = FNode::apply1(
                inRef,
                &move |__a0: ArcStr,
                       __a1: Mutable::Mutable<metamodelica::Ref<FCore::Node>>,
                       __a2: FCore::Graph|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(elred_one(&__a0, __a1, __a2))
                },
                g,
            )?;
            g
        }
    });
    Ok(og)
}

pub(crate) fn elred_one(mut name: &Name, mut inRef: Ref, mut ig: Graph) -> Graph {
    let mut og: Graph;
    og = 'mc: {
        let __mc_input = (inRef, ig.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (FNode::isRefRedeclare(r.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (FNode::isRefClass(r.clone()) && !(FNode::isRefClassExtends(r.clone()))
                || FNode::isRefComponent(r.clone()))
            else {
                return Err("pattern mismatch");
            };
            let true = (FNode::isRefRefResolved(r.clone())) else {
                return Err("pattern mismatch");
            };
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut rr: Ref;
            let mut p: Ref;
            let mut id: Name;
            let true = (FNode::isRefRedeclare(r.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (FNode::isRefClass(r.clone()) && !(FNode::isRefClassExtends(r.clone()))
                || FNode::isRefComponent(r.clone()))
            else {
                return Err("pattern mismatch");
            };
            id = SCodeUtil::elementName(&(FNode::getElement(&(FNode::fromRef(r.clone())))?))?;
            p = FNode::contextualParent(&(FNode::fromRef(r.clone())))?;
            (g, rr) = FLookup::ext(
                g.clone(),
                p.clone(),
                &id,
                FLookup::ignoreParentsAndImports.clone(),
                FLookup::dummyLookupOption.clone(),
            )?;
            g = FGraphBuild::mkRefNode(
                arcstr::literal!(FNode::refNodeName),
                list![rr.clone()],
                r.clone(),
                g.clone(),
            )?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut r, mut g) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut p: Ref;
            let mut id: Name;
            let true = (FNode::isRefRedeclare(r.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (FNode::isRefClass(r.clone()) && !(FNode::isRefClassExtends(r.clone()))
                || FNode::isRefComponent(r.clone()))
            else {
                return Err("pattern mismatch");
            };
            id = SCodeUtil::elementName(&(FNode::getElement(&(FNode::fromRef(r.clone())))?))?;
            p = FNode::contextualParent(&(FNode::fromRef(r.clone())))?;
            if '__try0: {
                unwrap_break_err!(FLookup::ext(g.clone(), p.clone(), &id, FLookup::ignoreParentsAndImports.clone(), FLookup::dummyLookupOption.clone()), '__try0);
                Ok::<(), &'static str>(())
            }.is_ok() { return Err("failure(): body succeeded") }
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("FResolve.elred_one: redeclare as element: "));
                __mm_s.push_str(&*id);
                __mm_s.push_str(&*literal!(" scope: "));
                __mm_s.push_str(&*FNode::toPathStr(&(FNode::fromRef(r.clone())))?);
                __mm_s.push_str(&*literal!(" not found in extends of: "));
                __mm_s.push_str(&*FNode::toPathStr(&(FNode::fromRef(p.clone())))?);
                __mm_s.push_str(&*literal!(":\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\t"));
                __mm_s.push_str(&*stringDelimitList(
                    List::map(
                        List::map(
                            FNode::extendsRefs(p.clone())?,
                            &fnptr!(FNode::fromRef, Mutable::Mutable<metamodelica::Ref<FCore::Node>>),
                        )?,
                        &move |__a0: metamodelica::Ref<FCore::Node>| FNode::toPathStr(&__a0),
                    )?,
                    literal!("\n\t"),
                ));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            g = FGraphBuild::mkRefNode(
                arcstr::literal!(FNode::refNodeName),
                metamodelica::nil(),
                r.clone(),
                g.clone(),
            )?;
            Ok(g.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(ig.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    og
}
