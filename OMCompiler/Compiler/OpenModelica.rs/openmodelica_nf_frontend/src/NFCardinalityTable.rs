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

use crate::NFConnection as Connection;
use crate::NFConnections as Connections;
use crate::NFConnector as Connector;
use crate::NFExpression as Expression;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;

pub type Table = metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>>;

pub(crate) fn emptyCardinalityTable(mut size: i32) -> Table {
    let mut table: Table;
    table = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        size,
    );
    table
}

pub(crate) fn fromConnections(mut conns: &metamodelica::Ref<Connections::NFConnections>) -> Result<Table> {
    let mut table: Table;
    if System::getUsesCardinality() {
        table = emptyCardinalityTable(std::cmp::max(1, Util::nextPrime(((conns.connections).len() as i32))));
        for mut conn in &*conns.connections.clone() {
            addConnector(&conn.lhs, table.clone())?;
            addConnector(&conn.rhs, table.clone())?;
        }
    } else {
        table = emptyCardinalityTable(1);
    }
    Ok(table)
}

pub(crate) fn addConnector(mut conn: &metamodelica::Ref<Connector::NFConnector>, mut table: Table) -> Result<()> {
    fn update(mut count: Option<i32>) -> i32 {
        let mut outCount: i32;
        outCount = (match count {
            Some(mut __esc_outCount) => {
                outCount = __esc_outCount.clone();
                outCount + 1
            }
            _ => 1,
        });
        outCount
    }

    let mut conn_str: ArcStr;
    conn_str = Connector::toString(conn)?;
    UnorderedMap::addUpdate(conn_str, &fnptr!(update, Option<i32>), table)?;
    Ok(())
}

pub(crate) fn evaluateCardinality(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
    mut table: Table,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut res: metamodelica::Ref<Expression::NFExpression>;
    let mut count: i32;
    count = UnorderedMap::getOrDefault(Expression::toString(arg)?, table, 0)?;
    res = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: count });
    Ok(res)
}

pub(crate) fn print(mut table: Table) -> () {
    for mut e in &*UnorderedMap::toList(table) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Util::tuple21(e.clone()));
            __mm_s.push_str(&*literal!(": "));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", Util::tuple22(e.clone()))));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    ()
}
