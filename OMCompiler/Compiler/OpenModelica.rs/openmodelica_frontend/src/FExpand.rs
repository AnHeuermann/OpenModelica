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
use crate::FResolve;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::FCore;
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

pub type Scope = metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;

pub type ImportTable = FCore::ImportTable;

pub type Graph = FCore::Graph;

pub type Extra = FCore::Extra;

pub type Visited = FCore::Visited;

pub type Import = Absyn::Import;

pub type Msg = Option<SourceInfo>;

pub(crate) fn path(mut inGraph: Graph, mut inPath: &metamodelica::Ref<Absyn::Path>) -> Result<(Graph, Ref)> {
    let mut outGraph: Graph;
    let mut outRef: Ref;
    (outGraph, outRef) = (match inGraph {
        mut g => {
            let mut r: Ref;
            let mut t: Ref;
            t = FGraph::top(&g)?;
            r = t;
            (g, r)
        }
    });
    Ok((outGraph, outRef))
}

pub(crate) fn all(mut inGraph: Graph) -> Result<Graph> {
    let mut outGraph: Graph;
    outGraph = (match inGraph {
        mut g => {
            let mut lst: metamodelica::List<metamodelica::Real>;
            lst = metamodelica::nil();
            System::startTimer();
            g = FResolve::ext(FGraph::top(&g)?, g)?;
            System::stopTimer();
            lst = List::consr(lst, System::getTimerIntervalTime());
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Extends:        "));
                __mm_s.push_str(&*realString((lst).head().cloned()?));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            System::startTimer();
            g = FResolve::derived(FGraph::top(&g)?, g)?;
            System::stopTimer();
            lst = List::consr(lst, System::getTimerIntervalTime());
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Derived:        "));
                __mm_s.push_str(&*realString((lst).head().cloned()?));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            System::startTimer();
            g = FResolve::cc(FGraph::top(&g)?, g)?;
            System::stopTimer();
            lst = List::consr(lst, System::getTimerIntervalTime());
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("ConstrainedBy:  "));
                __mm_s.push_str(&*realString((lst).head().cloned()?));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            System::startTimer();
            g = FResolve::clsext(FGraph::top(&g)?, g)?;
            System::stopTimer();
            lst = List::consr(lst, System::getTimerIntervalTime());
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("ClassExtends:   "));
                __mm_s.push_str(&*realString((lst).head().cloned()?));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            System::startTimer();
            g = FResolve::ty(FGraph::top(&g)?, g)?;
            System::stopTimer();
            lst = List::consr(lst, System::getTimerIntervalTime());
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("ComponentTypes: "));
                __mm_s.push_str(&*realString((lst).head().cloned()?));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            System::startTimer();
            g = FResolve::cr(FGraph::top(&g)?, g)?;
            System::stopTimer();
            lst = List::consr(lst, System::getTimerIntervalTime());
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Comp Refs:      "));
                __mm_s.push_str(&*realString((lst).head().cloned()?));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            System::startTimer();
            g = FResolve::r#mod(FGraph::top(&g)?, g)?;
            System::stopTimer();
            lst = List::consr(lst, System::getTimerIntervalTime());
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Modifiers:      "));
                __mm_s.push_str(&*realString((lst).head().cloned()?));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("FExpand.all:    "));
                __mm_s.push_str(&*realString(List::fold(
                    &lst,
                    &fnptr!(realAdd, metamodelica::Real, metamodelica::Real),
                    metamodelica::OrderedFloat(0.0_f64),
                )?));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            g
        }
    });
    Ok(outGraph)
}
