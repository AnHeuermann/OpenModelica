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

use crate::FBuiltin;
use crate::FExpand;
use crate::FGraph;
use crate::FGraphBuild;
use crate::FGraphDump;
use crate::FNode;
use crate::InstUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Flags;
use openmodelica_util::System;
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

pub type Graph = FCore::Graph;

pub type Extra = FCore::Extra;

pub type Visited = FCore::Visited;

pub type Import = Absyn::Import;

pub type Msg = Option<SourceInfo>;

pub(crate) fn inst(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> DAE::DAElist {
    let mut dae: DAE::DAElist;
    dae = 'mc: {
        let __mc_input = &*inProgram;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut g: Graph;
                    let mut p: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut lst: metamodelica::List<metamodelica::Real>;
                    p = doSCodeDep(inProgram.clone(), inPath.clone());
                    lst = metamodelica::nil();
                    System::realtimeTick(ClockIndexes::RT_CLOCK_FINST.clone())?;
                    (_, g) = FBuiltin::initialGraph(FCore::emptyCache())?;
                    g = FGraphBuild::mkProgramGraph(&p, openmodelica_frontend_dump::FCore::Kind::USERDEFINED, g.clone())?;
                    lst = List::consr(lst.clone(), System::realtimeTock(ClockIndexes::RT_CLOCK_FINST.clone())?);
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SCode->FGraph:  ")); __mm_s.push_str(&*realString((lst).head().cloned()?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    System::realtimeTick(ClockIndexes::RT_CLOCK_FINST.clone())?;
                    g = FExpand::all(g.clone())?;
                    lst = List::consr(lst.clone(), System::realtimeTock(ClockIndexes::RT_CLOCK_FINST.clone())?);
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Total time:     ")); __mm_s.push_str(&*realString(List::fold(&lst, &fnptr!(realAdd, metamodelica::Real, metamodelica::Real), metamodelica::OrderedFloat(0.0_f64))?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    FGraphDump::dumpGraph(&g, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("F:\\dev\\")); __mm_s.push_str(&*AbsynUtil::pathString(inPath.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(".graph.graphml")); ArcStr::from(__mm_s) })?;
                    System::realtimeTick(ClockIndexes::RT_CLOCK_FINST.clone())?;
                    FGraph::clone(g.clone())?;
                    lst = List::consr(lst.clone(), System::realtimeTock(ClockIndexes::RT_CLOCK_FINST.clone())?);
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FGraph->clone:  ")); __mm_s.push_str(&*realString((lst).head().cloned()?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(DAE::emptyDae().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("FInst.inst failed!\n"));
                    Ok(DAE::emptyDae().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    dae
}

pub fn instPath(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> DAE::DAElist {
    let mut dae: DAE::DAElist;
    dae = 'mc: {
        let __mc_input = &*inProgram;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inst(inPath.clone(), inProgram.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut g: Graph;
                    let mut p: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                    let mut lst: metamodelica::List<metamodelica::Real>;
                    lst = metamodelica::nil();
                    System::realtimeTick(ClockIndexes::RT_CLOCK_FINST.clone())?;
                    p = doSCodeDep(inProgram.clone(), inPath.clone());
                    lst = List::consr(lst.clone(), System::realtimeTock(ClockIndexes::RT_CLOCK_FINST.clone())?);
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SCode depend:   ")); __mm_s.push_str(&*realString((lst).head().cloned()?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    System::realtimeTick(ClockIndexes::RT_CLOCK_FINST.clone())?;
                    (_, g) = FBuiltin::initialGraph(FCore::emptyCache())?;
                    lst = List::consr(lst.clone(), System::realtimeTock(ClockIndexes::RT_CLOCK_FINST.clone())?);
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Initial graph:  ")); __mm_s.push_str(&*realString((lst).head().cloned()?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    System::realtimeTick(ClockIndexes::RT_CLOCK_FINST.clone())?;
                    g = FGraphBuild::mkProgramGraph(&p, openmodelica_frontend_dump::FCore::Kind::USERDEFINED, g.clone())?;
                    lst = List::consr(lst.clone(), System::realtimeTock(ClockIndexes::RT_CLOCK_FINST.clone())?);
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SCode->FGraph:  ")); __mm_s.push_str(&*realString((lst).head().cloned()?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    System::realtimeTick(ClockIndexes::RT_CLOCK_FINST.clone())?;
                    (g, _) = FExpand::path(g.clone(), &inPath)?;
                    lst = List::consr(lst.clone(), System::realtimeTock(ClockIndexes::RT_CLOCK_FINST.clone())?);
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FExpand.path:   ")); __mm_s.push_str(&*realString((lst).head().cloned()?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Total time:     ")); __mm_s.push_str(&*realString(List::fold(&lst, &fnptr!(realAdd, metamodelica::Real, metamodelica::Real), metamodelica::OrderedFloat(0.0_f64))?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    FGraphDump::dumpGraph(&g, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("F:\\dev\\")); __mm_s.push_str(&*AbsynUtil::pathString(inPath.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(".graph.graphml")); ArcStr::from(__mm_s) })?;
                    Ok(DAE::emptyDae().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("FInst.inst failed!\n"));
                    Ok(DAE::emptyDae().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    dae
}

fn doSCodeDep(
    mut inProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
) -> metamodelica::List<metamodelica::Ref<SCode::Element>> {
    let mut outProgram: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    outProgram = 'mc: {
        let __mc_input = &*inPath;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut outProgram: metamodelica::List<metamodelica::Ref<SCode::Element>> = outProgram.clone();
                    let true = (Flags::isSet(Flags::GRAPH_INST_RUN_DEP.clone())?) else { return Err("pattern mismatch") };
                    outProgram = InstUtil::scodeFlatten(inProgram.clone(), inPath.clone())?;
                    Ok((outProgram.clone(), outProgram.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outProgram = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inProgram.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outProgram
}
