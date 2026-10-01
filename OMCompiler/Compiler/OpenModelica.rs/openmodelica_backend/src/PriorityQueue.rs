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

use openmodelica_simcode_types::SimCode;

/* protected */
/* TODO: Hide when RML is killed */
/* This specific version... */
pub type Priority = i32;

pub type Data = metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;

/* Replaceable types */
pub(crate) fn compareElement(mut el1: &Element, mut el2: &Element) -> bool {
    let mut b: bool;
    let mut p1: Priority;
    let mut p2: Priority;
    (p1, _) = el1.clone();
    (p2, _) = el2.clone();
    b = p1 <= p2;
    b
}

pub type Element = (i32, metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>);

pub type T = metamodelica::List<metamodelica::Ref<Tree>>;

thread_local! { static __empty_TLS: metamodelica::List<metamodelica::Ref<Tree>> = metamodelica::nil(); }
pub(crate) fn empty() -> metamodelica::List<metamodelica::Ref<Tree>> {
    __empty_TLS.with(|__t| __t.clone())
}

/*
function isEmpty = listEmpty;
*/
pub(crate) fn isEmpty(mut ts: &T) -> bool {
    let mut isEmpty: bool;
    isEmpty = (ts).is_empty();
    isEmpty
}

pub(crate) fn insert(mut elt: Element, mut ts: T) -> Result<T> {
    let mut ots: T;
    ots = ins(
        metamodelica::Ref::new(Tree {
            elt: elt,
            rank: 0,
            trees: metamodelica::nil(),
        }),
        ts,
    )?;
    Ok(ots)
}

pub(crate) fn meld(mut its1: T, mut its2: T) -> Result<T> {
    let mut ts: T;
    ts = (::match_deref::match_deref! { match &((its1, its2)) {
        (ts1, Deref @ metamodelica::ListNode::Nil) => {
            ts1.clone()
        },
        (Deref @ metamodelica::ListNode::Nil, ts2) => {
            ts2.clone()
        },
        (Deref @ metamodelica::ListNode::Cons { head: t1, tail: ts1 }, Deref @ metamodelica::ListNode::Cons { head: t2, tail: ts2 }) => {
            meld2(rank(metamodelica::AsArg::as_arg(&t1)) < rank(metamodelica::AsArg::as_arg(&t2)), rank(metamodelica::AsArg::as_arg(&t2)) < rank(metamodelica::AsArg::as_arg(&t1)), t1.clone(), ts1.clone(), t2.clone(), ts2.clone())?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(ts)
}

pub(crate) fn meld2(
    mut b1: bool,
    mut b2: bool,
    mut t1: metamodelica::Ref<Tree>,
    mut inTs1: T,
    mut t2: metamodelica::Ref<Tree>,
    mut inTs2: T,
) -> Result<T> {
    let mut ts: T;
    ts = (match (b1, b2) {
        (true, _) => {
            let mut ts1 = inTs1.clone();
            let mut ts2 = inTs2.clone();
            ts = meld(ts1, metamodelica::cons(t2, ts2))?;
            metamodelica::cons(t1, ts)
        }
        (_, true) => {
            let mut ts1 = inTs1.clone();
            let mut ts2 = inTs2.clone();
            ts = meld(metamodelica::cons(t1, ts1), ts2)?;
            metamodelica::cons(t2, ts)
        }
        _ => ins(link(t1, t2)?, meld(inTs1, inTs2)?)?,
    });
    Ok(ts)
}

pub(crate) fn findMin(mut inTs: &T) -> Result<Element> {
    let mut elt: Element;
    elt = (::match_deref::match_deref! { match inTs {
        Deref @ metamodelica::ListNode::Cons { head: t, tail: Deref @ metamodelica::ListNode::Nil } => {
            root(metamodelica::AsArg::as_arg(&t))
        },
        Deref @ metamodelica::ListNode::Cons { head: t, tail: ts } => {
            let mut x: Element;
            let mut y: Element;
            x = root(metamodelica::AsArg::as_arg(&t));
            y = findMin(ts)?;
            if (compareElement(&x, &y)) {x} else {y}
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(elt)
}

pub(crate) fn deleteMin(mut ts: &T) -> Result<T> {
    let mut ots: T;
    let mut ts1: T;
    let mut ts2: T;
    let (__t2, __pa1) = getMin(ts)?;
    let __arc3 = __t2.clone();
    let Tree { trees: __pa0, .. } = &*__arc3;
    ts1 = metamodelica::Own::own(__pa0);
    ts2 = metamodelica::Own::own(__pa1);
    ots = meld(ts1.reverse(), ts2)?;
    Ok(ots)
}

pub(crate) fn deleteAndReturnMin(mut ts: &T) -> Result<(T, Element)> {
    let mut ots: T;
    let mut elt: Element;
    let mut ts1: T;
    let mut ts2: T;
    let (__t3, __pa2) = getMin(ts)?;
    let __arc4 = __t3.clone();
    let Tree {
        elt: __pa0,
        trees: __pa1,
        ..
    } = &*__arc4;
    elt = metamodelica::Own::own(__pa0);
    ts1 = metamodelica::Own::own(__pa1);
    ts2 = metamodelica::Own::own(__pa2);
    ots = meld(ts1.reverse(), ts2)?;
    Ok((ots, elt))
}

pub(crate) fn elements(
    mut ts: T,
) -> Result<metamodelica::List<(i32, metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>)>> {
    let mut elts: metamodelica::List<(i32, metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>)>;
    elts = elements2(ts, metamodelica::nil())?;
    Ok(elts)
}

pub(crate) fn elements2(
    mut its: T,
    mut acc: metamodelica::List<(i32, metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>)>,
) -> Result<metamodelica::List<(i32, metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>)>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(its) {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(acc.reverse())
            },
            ts => {
                let mut elt: Element;
                let mut ts = (*ts).clone();
                (ts, elt) = deleteAndReturnMin(metamodelica::AsArg::as_arg(&ts))?;
                { (its, acc) = (ts.clone(), metamodelica::cons(elt, acc)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

/* TODO: Hide from user when we remove RML... */
pub type Rank = i32;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Tree {
    pub elt: Element,
    pub rank: Rank,
    pub trees: T,
}

impl metamodelica::gc::MMTrace for Tree {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.elt, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.rank, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.trees, __mmv)?;
        Ok(())
    }
}
impl Default for Tree {
    fn default() -> Self {
        Self {
            elt: Default::default(),
            rank: Default::default(),
            trees: Default::default(),
        }
    }
}

pub type NODE = Tree;

fn root(mut tree: &metamodelica::Ref<Tree>) -> Element {
    let mut elt: Element;
    let __arc1 = &(*tree);
    let Tree { elt: __pa0, .. } = &**__arc1;
    elt = metamodelica::Own::own(__pa0);
    elt
}

fn rank(mut tree: &metamodelica::Ref<Tree>) -> Rank {
    let mut rank: Rank;
    let __arc1 = &(*tree);
    let Tree { rank: __pa0, .. } = &**__arc1;
    rank = metamodelica::Own::own(__pa0);
    rank
}

fn link(mut t1: metamodelica::Ref<Tree>, mut t2: metamodelica::Ref<Tree>) -> Result<metamodelica::Ref<Tree>> {
    let mut t: metamodelica::Ref<Tree>;
    t = (::match_deref::match_deref! { match &((t1.clone(), t2.clone())) {
        (Deref @ Tree { elt: e1, rank: r1, trees: ts1 }, Deref @ Tree { elt: e2, rank: r2, trees: ts2 }) => {
            let mut r1 = (*r1).clone();
            let mut ts1 = (*ts1).clone();
            let mut r2 = (*r2).clone();
            let mut ts2 = (*ts2).clone();
            r1 = r1.clone() + 1;
            r2 = r2.clone() + 1;
            ts1 = metamodelica::cons(t2.clone(), ts1.clone());
            ts2 = metamodelica::cons(t1.clone(), ts2.clone());
            if (compareElement(&(root(&t1)), &(root(&t2)))) {metamodelica::Ref::new(Tree { elt: e1.clone(), rank: r1.clone(), trees: ts1.clone() })} else {metamodelica::Ref::new(Tree { elt: e2.clone(), rank: r2.clone(), trees: ts2.clone() })}
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(t)
}

fn ins(mut t: metamodelica::Ref<Tree>, mut its: T) -> Result<T> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((t.clone(), its)) {
            (_, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(list![t])
            },
            (t1, Deref @ metamodelica::ListNode::Cons { head: t2, tail: ts }) => {
                if (rank(metamodelica::AsArg::as_arg(&t1)) < rank(metamodelica::AsArg::as_arg(&t2))) {return Ok(metamodelica::cons(t1.clone(), metamodelica::cons(t2.clone(), ts.clone())))} else {{ (t, its) = (link(t1.clone(), t2.clone())?, ts.clone()); continue '__tco; }}
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getMin(mut ts: &T) -> Result<(metamodelica::Ref<Tree>, T)> {
    let mut min: metamodelica::Ref<Tree>;
    let mut ots: T;
    (min, ots) = (::match_deref::match_deref! { match ts {
        Deref @ metamodelica::ListNode::Cons { head: t, tail: Deref @ metamodelica::ListNode::Nil } => {
            (t.clone(), metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: t1, tail: ts1 } => {
            let mut t2: metamodelica::Ref<Tree>;
            let mut ts2: T;
            let mut b: bool;
            (t2, ts2) = getMin(ts1)?;
            b = compareElement(&(root(metamodelica::AsArg::as_arg(&t1))), &(root(&t2)));
            (if (b) {t1.clone()} else {t2}, if (b) {ts1.clone()} else {metamodelica::cons(t1.clone(), ts2)})
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((min, ots))
}
