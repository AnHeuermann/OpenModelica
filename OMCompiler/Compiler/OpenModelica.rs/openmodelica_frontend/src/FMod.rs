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
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

// public imports
// protected imports
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

pub type Scope = metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;

pub type ImportTable = FCore::ImportTable;

pub type Graph = FCore::Graph;

pub type Extra = FCore::Extra;

pub type Visited = FCore::Visited;

pub type Import = Absyn::Import;

pub type ModScope = FCore::ModScope;

pub(crate) fn merge(
    mut inParentRef: Ref,
    mut inOuterModRef: Ref,
    mut inInnerModRef: Ref,
    mut inGraph: Graph,
) -> (Graph, Ref) {
    let mut outGraph: Graph;
    let mut outMergedModRef: Ref;
    (outGraph, outMergedModRef) = (match (inParentRef, inGraph) {
        (mut r, mut g) => (g, r),
    });
    (outGraph, outMergedModRef)
}

pub(crate) fn apply(mut inTargetRef: Ref, mut inModRef: Ref, mut inGraph: Graph) -> (Graph, Ref) {
    let mut outGraph: Graph;
    let mut outNodeRef: Ref;
    (outGraph, outNodeRef) = (match (inTargetRef, inGraph) {
        (mut r, mut g) => (g, r),
    });
    (outGraph, outNodeRef)
}

pub(crate) fn compactSubMods(
    mut inSubMods: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    mut inModScope: ModScope,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::SubMod>>> {
    let mut outSubMods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    let mut submods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    submods = List::fold2(
        inSubMods,
        &move |__a0: metamodelica::Ref<SCode::SubMod>,
               __a1: FCore::ModScope,
               __a2: metamodelica::List<ArcStr>,
               __a3: metamodelica::List<metamodelica::Ref<SCode::SubMod>>| {
            compactSubMod(__a0, &__a1, &__a2, __a3)
        },
        inModScope,
        metamodelica::nil(),
        metamodelica::nil(),
    )?;
    outSubMods = submods.reverse();
    Ok(outSubMods)
}

fn compactSubMod(
    mut inSubMod: metamodelica::Ref<SCode::SubMod>,
    mut inModScope: &ModScope,
    mut inName: &metamodelica::List<ArcStr>,
    mut inAccumMods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::SubMod>>> {
    let mut outSubMods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    let mut name: ArcStr;
    let mut submods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    let mut found: bool;
    let __arc1 = inSubMod.clone();
    let SCode::NAMEMOD { ident: __pa0, r#mod: _ } = &*__arc1;
    name = metamodelica::Own::own(__pa0);
    (submods, found) = List::findMap(
        inAccumMods,
        &({
            let __pe_b1 = inSubMod.clone();
            let __pe_b2 = inModScope.clone();
            let __pe_b3 = inName.clone();
            move |__pe_a0| compactSubMod2(__pe_a0, &__pe_b1, __pe_b2.clone(), __pe_b3.clone())
        }),
    )?;
    outSubMods = List::consOnTrue(!(found), inSubMod, submods);
    Ok(outSubMods)
}

fn compactSubMod2(
    mut inExistingMod: metamodelica::Ref<SCode::SubMod>,
    mut inNewMod: &metamodelica::Ref<SCode::SubMod>,
    mut inModScope: ModScope,
    mut inName: metamodelica::List<ArcStr>,
) -> Result<(metamodelica::Ref<SCode::SubMod>, bool)> {
    let mut outMod: metamodelica::Ref<SCode::SubMod>;
    let mut outFound: bool;
    (outMod, outFound) = (::match_deref::match_deref! { match &((inExistingMod.clone(), inNewMod.clone())) {
        (Deref @ SCode::SubMod { ident: name1, .. }, Deref @ SCode::SubMod { ident: name2, .. }) if (!(stringEqual(&name1, &name2))) => {
            (inExistingMod, false)
        },
        (Deref @ SCode::SubMod { ident: name1, .. }, _) => {
            let mut submod: metamodelica::Ref<SCode::SubMod>;
            submod = mergeSubModsInSameScope(&inExistingMod, inNewMod, metamodelica::cons(name1.clone(), inName), inModScope)?;
            (submod, true)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outMod, outFound))
}

fn mergeSubModsInSameScope(
    mut inMod1: &metamodelica::Ref<SCode::SubMod>,
    mut inMod2: &metamodelica::Ref<SCode::SubMod>,
    mut inElementName: metamodelica::List<ArcStr>,
    mut inModScope: ModScope,
) -> Result<metamodelica::Ref<SCode::SubMod>> {
    let mut outMod: metamodelica::Ref<SCode::SubMod>;
    let mut scope: ArcStr;
    let mut name: ArcStr;
    let mut submods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    let mut info1: SourceInfo;
    let mut info2: SourceInfo;
    let mut mod1: metamodelica::Ref<SCode::Mod> = inMod1.r#mod.clone();
    let mut mod2: metamodelica::Ref<SCode::Mod> = inMod2.r#mod.clone();
    outMod = (::match_deref::match_deref! { match &((mod1.clone(), mod2.clone())) {
        (Deref @ SCode::Mod::MOD { .. }, Deref @ SCode::Mod::MOD { binding: None, .. }) => {
            submods = List::fold2(var_field!((*mod1).subModLst, SCode::Mod::MOD), &move |__a0: metamodelica::Ref<SCode::SubMod>, __a1: FCore::ModScope, __a2: metamodelica::List<ArcStr>, __a3: metamodelica::List<metamodelica::Ref<SCode::SubMod>>| compactSubMod(__a0, &__a1, &__a2, __a3), inModScope, inElementName, var_field!((*mod2).subModLst, SCode::Mod::MOD).clone())?;
            metamodelica::Ref::new(SCode::SubMod { ident: inMod1.ident.clone(), r#mod: metamodelica::Ref::new(SCode::Mod::MOD { finalPrefix: var_field!((*mod1).finalPrefix, SCode::Mod::MOD).clone(), eachPrefix: var_field!((*mod1).eachPrefix, SCode::Mod::MOD).clone(), subModLst: submods, binding: var_field!((*mod1).binding, SCode::Mod::MOD).clone(), comment: var_field!((*mod1).comment, SCode::Mod::MOD).clone(), info: var_field!((*mod1).info, SCode::Mod::MOD).clone() }) })
        },
        (Deref @ SCode::Mod::MOD { binding: None, .. }, Deref @ SCode::Mod::MOD { .. }) => {
            submods = List::fold2(var_field!((*mod1).subModLst, SCode::Mod::MOD), &move |__a0: metamodelica::Ref<SCode::SubMod>, __a1: FCore::ModScope, __a2: metamodelica::List<ArcStr>, __a3: metamodelica::List<metamodelica::Ref<SCode::SubMod>>| compactSubMod(__a0, &__a1, &__a2, __a3), inModScope, inElementName, var_field!((*mod2).subModLst, SCode::Mod::MOD).clone())?;
            metamodelica::Ref::new(SCode::SubMod { ident: inMod2.ident.clone(), r#mod: metamodelica::Ref::new(SCode::Mod::MOD { finalPrefix: var_field!((*mod2).finalPrefix, SCode::Mod::MOD).clone(), eachPrefix: var_field!((*mod2).eachPrefix, SCode::Mod::MOD).clone(), subModLst: submods, binding: var_field!((*mod2).binding, SCode::Mod::MOD).clone(), comment: var_field!((*mod2).comment, SCode::Mod::MOD).clone(), info: var_field!((*mod2).info, SCode::Mod::MOD).clone() }) })
        },
        _ => {
            info1 = SCodeUtil::getModifierInfo(&mod1);
            info2 = SCodeUtil::getModifierInfo(&mod2);
            scope = printModScope(&inModScope)?;
            name = stringDelimitList(inElementName.reverse(), literal!("."));
            Error::addMultiSourceMessage(&(Error::DUPLICATE_MODIFICATIONS.clone()), &(list![name, scope]), &(list![info2, info1]))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMod)
}

fn printModScope(mut inModScope: &ModScope) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inModScope.clone() {
        FCore::ModScope::MS_COMPONENT { name: mut name } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("component "));
            __mm_s.push_str(&*name);
            ArcStr::from(__mm_s)
        }
        FCore::ModScope::MS_EXTENDS { path: mut path } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("extends "));
            __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), literal!("."), true, false)?);
            ArcStr::from(__mm_s)
        }
        FCore::ModScope::MS_DERIVED { path: mut path } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("inherited class "));
            __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), literal!("."), true, false)?);
            ArcStr::from(__mm_s)
        }
        FCore::ModScope::MS_CLASS_EXTENDS { name: mut name } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("class extends class "));
            __mm_s.push_str(&*name);
            ArcStr::from(__mm_s)
        }
        FCore::ModScope::MS_CONSTRAINEDBY { path: mut path } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("constrainedby class "));
            __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), literal!("."), true, false)?);
            ArcStr::from(__mm_s)
        }
    });
    Ok(outString)
}
