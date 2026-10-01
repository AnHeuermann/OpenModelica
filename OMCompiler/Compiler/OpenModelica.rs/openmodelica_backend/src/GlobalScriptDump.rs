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

use crate::SymbolTable;
use openmodelica_ast::Absyn;
use openmodelica_ast::GlobalScript;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_util_datatypes_basic::List;

pub fn printIstmtsStr(mut inStatements: &GlobalScript::Statements) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inStatements.clone() {
        GlobalScript::Statements {
            interactiveStmtLst: ref stmts,
            ..
        } => stringDelimitList(
            List::map(stmts.clone(), &move |__a0: GlobalScript::Statement| {
                printIstmtStr(&__a0)
            })?,
            literal!("; "),
        ),
        _ => {
            literal!("printIstmtsStr: unknown")
        }
    });
    Ok(outString)
}

pub fn printIstmtStr(mut inStatement: &GlobalScript::Statement) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inStatement.clone() {
        GlobalScript::Statement::IALG { algItem: ref alg } => Dump::unparseAlgorithmStr(alg.clone())?,
        GlobalScript::Statement::IEXP { exp: ref expr, .. } => Dump::printExpStr(expr.clone())?,
        _ => {
            literal!("printIstmtStr: unknown")
        }
    });
    Ok(outString)
}

pub(crate) fn printAST(mut pr: Absyn::Program) -> Result<()> {
    let mut s: ArcStr = literal!("");
    let mut class_: metamodelica::Ref<Absyn::Class> =
        <metamodelica::Ref<Absyn::Class> as ::std::default::Default>::default();
    let mut classes: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
    let mut within_: Absyn::Within;
    let Absyn::PROGRAM {
        classes: __pa0,
        within_: __pa1,
    } = pr;
    classes = metamodelica::Own::own(__pa0);
    within_ = metamodelica::Own::own(__pa1);
    for mut class_ in &*classes {
        let mut class_ = class_.clone();
        s = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*s);
            __mm_s.push_str(&*classString(&class_)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
    }
    metamodelica::print(s);
    Ok(())
}

pub(crate) fn printGlobalScript(mut st: &metamodelica::Ref<SymbolTable::SymbolTable>) -> Result<()> {
    metamodelica::print(literal!("AST\n"));
    printAST(st.ast.clone())?;
    Ok(())
}

fn classString(mut cl: &metamodelica::Ref<Absyn::Class>) -> Result<ArcStr> {
    let mut s: ArcStr;
    let mut id: ArcStr;
    let __arc1 = &(*cl);
    let Absyn::CLASS { name: __pa0, .. } = &**__arc1;
    id = metamodelica::Own::own(__pa0);
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*id);
        __mm_s.push_str(&*literal!(": "));
        __mm_s.push_str(&*AbsynUtil::classFilename(cl)?);
        ArcStr::from(__mm_s)
    };
    Ok(s)
}
