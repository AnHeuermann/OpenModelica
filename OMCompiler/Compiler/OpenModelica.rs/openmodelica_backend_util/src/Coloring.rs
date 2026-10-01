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

use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::Graph;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

pub fn createColoring(
    mut sparseArray: metamodelica::Array<metamodelica::List<i32>>,
    mut sparseArrayT: metamodelica::Array<metamodelica::List<i32>>,
    mut sizeVars: i32,
    mut sizeVarswithDep: i32,
) -> Result<metamodelica::Array<metamodelica::List<i32>>> {
    let mut coloredArray: metamodelica::Array<metamodelica::List<i32>>;
    let debug: bool = false;
    let mut colored: metamodelica::Array<i32>;
    let mut sparseGraphT: metamodelica::List<(i32, metamodelica::List<i32>)>;
    let mut maxColor: i32;
    match '__try0: {
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_SPARSE_VERBOSE.clone()), '__try0) {
            metamodelica::print(literal!("analytical Jacobians[SPARSE] -> build sparse graph.\n"));
        }
        sparseGraphT = unwrap_break_err!(Graph::buildGraph(List::intRange2(1, sizeVars), &createBipartiteGraph, sparseArrayT.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_SPARSE_VERBOSE.clone()), '__try0) {
            metamodelica::print(literal!("sparse graph: \n"));
            unwrap_break_err!(Graph::printGraphInt(&(unwrap_break_err!(Graph::buildGraph(List::intRange2(1, sizeVarswithDep), &createBipartiteGraph, sparseArray.clone()), '__try0))), '__try0);
            metamodelica::print(literal!("transposed sparse graph: \n"));
            unwrap_break_err!(Graph::printGraphInt(&sparseGraphT), '__try0);
            metamodelica::print(literal!(
                "analytical Jacobians[SPARSE] -> builded graph for coloring.\n"
            ));
        }
        colored = arrayCreate(sizeVars, 0);
        if debug {
            unwrap_break_err!(execStat(&(literal!("generateSparsePattern -> coloring start "))), '__try0);
        }
        if sizeVars > 0 {
            unwrap_break_err!(Graph::partialDistance2colorInt(&sparseGraphT, sizeVarswithDep, colored.clone()), '__try0);
        }
        if debug {
            unwrap_break_err!(execStat(&(literal!("generateSparsePattern -> coloring end "))), '__try0);
        }
        maxColor = unwrap_break_err!(Array::fold(colored.clone(), &fnptr!(intMax, i32, i32), 0), '__try0);
        coloredArray = arrayCreate(maxColor, metamodelica::nil());
        unwrap_break_err!(mapIndexColors(colored.clone(), sizeVars, coloredArray.clone()), '__try0);
        GCExt::free(colored.clone());
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_SPARSE_VERBOSE.clone()), '__try0) {
            metamodelica::print(literal!("Print Coloring Cols: \n"));
            unwrap_break_err!(dumpColoring(&(coloredArray.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>())), '__try0);
        }
        Ok::<_, &'static str>((
            colored.clone(),
            coloredArray.clone(),
            maxColor.clone(),
            sparseGraphT.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            colored = __try0_o0;
            coloredArray = __try0_o1;
            maxColor = __try0_o2;
            sparseGraphT = __try0_o3;
        }
        Err(__try0_err) => {
            Error::addInternalError(
                literal!("function createColoring failed"),
                metamodelica::sourceInfo!("BackEnd/Coloring.mo"),
            )?;
            return Err(__try0_err);
        }
    }
    Ok(coloredArray)
}

fn createBipartiteGraph(
    mut inNode: i32,
    mut inSparsePattern: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<metamodelica::List<i32>> {
    let mut outEdges: metamodelica::List<i32> = metamodelica::nil();
    if inNode >= 1 && inNode <= metamodelica::arrayLength(inSparsePattern.clone()) {
        outEdges = metamodelica::arrayGet(inSparsePattern.clone(), inNode)?;
    } else {
        outEdges = metamodelica::nil();
    }
    Ok(outEdges)
}

fn mapIndexColors(
    mut inColors: metamodelica::Array<i32>,
    mut inMaxIndex: i32,
    mut inArray: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<()> {
    let mut index: i32;
    match '__try0: {
        for mut i in 1..=inMaxIndex {
            index = unwrap_break_err!(metamodelica::arrayGet(inColors.clone(), i), '__try0);
            unwrap_break_err!(metamodelica::arrayUpdate(inArray.clone(), index, metamodelica::cons(i, unwrap_break_err!(metamodelica::arrayGet(inArray.clone(), index), '__try0))), '__try0);
        }
        Ok::<(), &'static str>(())
    } {
        Ok(()) => {}
        Err(__try0_err) => {
            Error::addInternalError(
                literal!("function mapIndexColors failed"),
                metamodelica::sourceInfo!("BackEnd/Coloring.mo"),
            )?;
            return Err(__try0_err);
        }
    }
    Ok(())
}

fn dumpColoring(mut pattern: &metamodelica::List<metamodelica::List<i32>>) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Print sparse pattern: "));
        __mm_s.push_str(&*intString(((pattern).len() as i32)));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    for mut row in &**pattern {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("{"));
            __mm_s.push_str(&*stringDelimitList(
                List::map(row.clone(), &fnptr!(intString, i32))?,
                literal!(", "),
            ));
            __mm_s.push_str(&*literal!("}\n"));
            ArcStr::from(__mm_s)
        });
    }
    metamodelica::print(literal!("\n"));
    Ok(())
}
