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

use crate::NFConnector as Connector;
use openmodelica_util::Error;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct NFConnection {
    pub lhs: metamodelica::Ref<Connector::NFConnector>,
    pub rhs: metamodelica::Ref<Connector::NFConnector>,
}

impl metamodelica::gc::MMTrace for NFConnection {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.lhs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.rhs, __mmv)?;
        Ok(())
    }
}
impl Default for NFConnection {
    fn default() -> Self {
        Self {
            lhs: Default::default(),
            rhs: Default::default(),
        }
    }
}

pub type CONNECTION = NFConnection;

pub(crate) fn split(
    mut conn: &metamodelica::Ref<NFConnection>,
) -> Result<metamodelica::List<metamodelica::Ref<NFConnection>>> {
    let mut conns: metamodelica::List<metamodelica::Ref<NFConnection>> = metamodelica::nil();
    let mut cls: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    let mut crs: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    let mut cr: metamodelica::Ref<Connector::NFConnector>;
    cls = Connector::split(&conn.lhs)?;
    crs = Connector::split(&conn.rhs)?;
    checkBalance(cls.clone(), crs.clone(), conn)?;
    for mut cl in &*cls {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crs) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cr = metamodelica::Own::own(__pa0);
        crs = metamodelica::Own::own(__pa1);
        if !(Connector::isDeleted(metamodelica::AsArg::as_arg(&cl))? || Connector::isDeleted(&cr)?) {
            conns = metamodelica::cons(
                metamodelica::Ref::new(NFConnection {
                    lhs: cl.clone(),
                    rhs: cr,
                }),
                conns,
            );
        }
    }
    conns = metamodelica::Dangerous::listReverseInPlace(conns);
    Ok(conns)
}

pub(crate) fn scalarize(
    mut conn: metamodelica::Ref<NFConnection>,
) -> Result<metamodelica::List<metamodelica::Ref<NFConnection>>> {
    let mut conns: metamodelica::List<metamodelica::Ref<NFConnection>> = metamodelica::nil();
    let mut cls: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    let mut crs: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    let mut cr: metamodelica::Ref<Connector::NFConnector>;
    if !(Connector::isArray(&conn.lhs)) {
        conns = list![conn];
        return Ok(conns);
    }
    cls = Connector::scalarize(&conn.lhs)?;
    crs = Connector::scalarize(&conn.rhs)?;
    checkBalance(cls.clone(), crs.clone(), &conn)?;
    for mut cl in &*cls {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crs) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cr = metamodelica::Own::own(__pa0);
        crs = metamodelica::Own::own(__pa1);
        conns = metamodelica::cons(
            metamodelica::Ref::new(NFConnection {
                lhs: cl.clone(),
                rhs: cr,
            }),
            conns,
        );
    }
    conns = metamodelica::Dangerous::listReverseInPlace(conns);
    Ok(conns)
}

pub(crate) fn scalarizePrefix(
    mut conn: metamodelica::Ref<NFConnection>,
) -> Result<metamodelica::List<metamodelica::Ref<NFConnection>>> {
    let mut conns: metamodelica::List<metamodelica::Ref<NFConnection>> = metamodelica::nil();
    let mut cls: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    let mut crs: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>;
    let mut cr: metamodelica::Ref<Connector::NFConnector>;
    if !(Connector::isArray(&conn.lhs)) {
        conns = list![conn];
        return Ok(conns);
    }
    cls = Connector::scalarizePrefix(conn.lhs.clone())?;
    crs = Connector::scalarizePrefix(conn.rhs.clone())?;
    checkBalance(cls.clone(), crs.clone(), &conn)?;
    for mut cl in &*cls {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crs) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cr = metamodelica::Own::own(__pa0);
        crs = metamodelica::Own::own(__pa1);
        conns = metamodelica::cons(
            metamodelica::Ref::new(NFConnection {
                lhs: cl.clone(),
                rhs: cr,
            }),
            conns,
        );
    }
    conns = metamodelica::Dangerous::listReverseInPlace(conns);
    Ok(conns)
}

pub(crate) fn toString(mut conn: &metamodelica::Ref<NFConnection>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("connect("));
        __mm_s.push_str(&*Connector::toString(&conn.lhs)?);
        __mm_s.push_str(&*literal!(", "));
        __mm_s.push_str(&*Connector::toString(&conn.rhs)?);
        __mm_s.push_str(&*literal!(")"));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

fn checkBalance(
    mut leftConnectors: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
    mut rightConnectors: metamodelica::List<metamodelica::Ref<Connector::NFConnector>>,
    mut conn: &metamodelica::Ref<NFConnection>,
) -> Result<()> {
    if ((leftConnectors).len() as i32) != ((rightConnectors).len() as i32) {
        Error::terminate(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFConnection.checkBalance"));
                __mm_s.push_str(&*literal!(" got unbalanced connection "));
                __mm_s.push_str(&*toString(conn)?);
                __mm_s.push_str(&*literal!(":"));
                __mm_s.push_str(&*List::toStringCustom(
                    leftConnectors,
                    &move |__a0: metamodelica::Ref<Connector::NFConnector>| Connector::toString(&__a0),
                    literal!("\n  lhs: "),
                    literal!("{"),
                    literal!(", "),
                    literal!("}"),
                    true,
                    0,
                )?);
                __mm_s.push_str(&*List::toStringCustom(
                    rightConnectors,
                    &move |__a0: metamodelica::Ref<Connector::NFConnector>| Connector::toString(&__a0),
                    literal!("\n  rhs: "),
                    literal!("{"),
                    literal!(", "),
                    literal!("}"),
                    true,
                    0,
                )?);
                ArcStr::from(__mm_s)
            },
            &(metamodelica::sourceInfo!("NFFrontEnd/NFConnection.mo")),
        )?;
        return Err("fail");
    }
    Ok(())
}
