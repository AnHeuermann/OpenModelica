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

pub type Type = ArcStr;

pub type Ident = ArcStr;

pub type Label = ArcStr;

/// an Attribute is a pair of name an value.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Attribute {
    /// name
    pub name: ArcStr,
    /// value
    pub value: ArcStr,
}

impl metamodelica::gc::MMTrace for Attribute {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.value, __mmv)?;
        Ok(())
    }
}
impl Default for Attribute {
    fn default() -> Self {
        Self {
            name: Default::default(),
            value: Default::default(),
        }
    }
}

pub type ATTR = Attribute;

pub type Attributes = metamodelica::List<Attribute>;

/// A graphviz Node is a node of the graph.
///    It has a type and attributes and children.
///    It can also have a list of labels, provided by the LNODE
///    constructor.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Node {
    NODE {
        type_: Type,
        attributes: Attributes,
        children: metamodelica::List<metamodelica::Ref<Node>>,
    },
    LNODE {
        type_: Type,
        labelLst: metamodelica::List<ArcStr>,
        attributes: Attributes,
        children: metamodelica::List<metamodelica::Ref<Node>>,
    },
}
impl metamodelica::gc::MMTrace for Node {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Node::NODE {
                type_,
                attributes,
                children,
            } => {
                metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attributes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(children, __mmv)?;
                Ok(())
            }
            Node::LNODE {
                type_,
                labelLst,
                attributes,
                children,
            } => {
                metamodelica::gc::MMTrace::mm_accept(type_, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(labelLst, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(attributes, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(children, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Node {
    fn default() -> Self {
        Self::NODE {
            type_: Default::default(),
            attributes: Default::default(),
            children: Default::default(),
        }
    }
}
pub use self::Node::{LNODE, NODE};

pub type Children = metamodelica::List<metamodelica::Ref<Node>>;

pub static r#box: Attribute = Attribute {
    name: literal!("shape"),
    value: literal!("box"),
};

pub fn dump(mut node: &metamodelica::Ref<Node>) -> Result<()> {
    let mut nm: Label;
    metamodelica::print(literal!("graph AST {\n"));
    nm = dumpNode(node)?;
    metamodelica::print(literal!("}\n"));
    Ok(())
}

fn dumpNode(mut inNode: &metamodelica::Ref<Node>) -> Result<Ident> {
    let mut outIdent: Ident;
    outIdent = (match &**inNode {
        Node::NODE {
            type_: typ,
            attributes: attr,
            children,
        } => {
            let mut nm: Label;
            let mut typlbl: Label;
            let mut out: Label;
            let mut newattr: Attributes;
            nm = nodename(typ);
            typlbl = makeLabel(&(list![typ.clone()]))?;
            newattr = metamodelica::cons(
                Attribute {
                    name: literal!("label"),
                    value: typlbl,
                },
                attr.clone(),
            );
            out = makeNode(nm.clone(), &newattr)?;
            metamodelica::print(out);
            dumpChildren(nm.clone(), children)?;
            nm
        }
        Node::LNODE {
            type_: typ,
            labelLst: lbl,
            attributes: attr,
            children,
        } => {
            let mut nm: Label;
            let mut out: Label;
            let mut lblstr: Label;
            let mut newattr: Attributes;
            let mut lbl_1: metamodelica::List<ArcStr>;
            nm = nodename(typ);
            lbl_1 = metamodelica::cons(typ.clone(), lbl.clone());
            lblstr = makeLabel(&lbl_1)?;
            newattr = metamodelica::cons(
                Attribute {
                    name: literal!("label"),
                    value: lblstr,
                },
                attr.clone(),
            );
            out = makeNode(nm.clone(), &newattr)?;
            metamodelica::print(out);
            dumpChildren(nm.clone(), children)?;
            nm
        }
    });
    Ok(outIdent)
}

fn makeLabel(mut sl: &metamodelica::List<ArcStr>) -> Result<ArcStr> {
    let mut s2: ArcStr;
    let mut s0: Label;
    let mut s1: Label;
    s0 = makeLabelReq(sl, literal!(""))?;
    s1 = stringAppend(literal!("\""), s0);
    s2 = stringAppend(s1, literal!("\""));
    Ok(s2)
}

fn makeLabelReq<'__b>(mut inStringLst: &'__b metamodelica::List<ArcStr>, mut inString: ArcStr) -> Result<ArcStr> {
    '__tco: loop {
        ::match_deref::match_deref! { match inStringLst {
            Deref @ metamodelica::ListNode::Cons { head: s, tail: Deref @ metamodelica::ListNode::Nil } => {
                return Ok(stringAppend(inString, s.clone()))
            },
            Deref @ metamodelica::ListNode::Cons { head: s1, tail: Deref @ metamodelica::ListNode::Cons { head: s2, tail: Deref @ metamodelica::ListNode::Nil } } => {
                let mut s: Label;
                s = stringAppend(inString, s1.clone());
                s = stringAppend(s, literal!("\\n"));
                return Ok(stringAppend(s, s2.clone()))
            },
            Deref @ metamodelica::ListNode::Cons { head: s1, tail: rest } => {
                let mut s: Label;
                s = stringAppend(inString, s1.clone());
                s = stringAppend(s, literal!("\\n"));
                { (inStringLst, inString) = (rest, s); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn dumpChildren(mut inIdent: Ident, mut inChildren: &Children) -> Result<()> {
    let () = (::match_deref::match_deref! { match inChildren {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: node, tail: rest } => {
            let mut parent = inIdent;
            let mut nm: Label;
            nm = dumpNode(metamodelica::AsArg::as_arg(&node))?;
            printEdge(nm, parent.clone());
            dumpChildren(parent, rest)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn nodename(mut r#str: &ArcStr) -> ArcStr {
    let mut s: ArcStr;
    let mut i: i32;
    let mut is: Label;
    i = tick();
    is = intString(i);
    s = stringAppend(literal!("GVNOD"), is);
    s
}

fn printEdge(mut n1: Ident, mut n2: Ident) -> () {
    let mut r#str: Label;
    r#str = makeEdge(n1, n2);
    metamodelica::print(r#str);
    metamodelica::print(literal!(";\n"));
    ()
}

fn makeEdge(mut n1: Ident, mut n2: Ident) -> ArcStr {
    let mut r#str: ArcStr;
    let mut s: Label;
    s = stringAppend(n1, literal!(" -- "));
    r#str = stringAppend(s, n2);
    r#str
}

fn makeNode(mut nm: Ident, mut attr: &Attributes) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut s: Label;
    let mut s_1: Label;
    s = makeAttr(attr)?;
    s_1 = stringAppend(nm, s);
    r#str = stringAppend(s_1, literal!(";"));
    Ok(r#str)
}

fn makeAttr(mut l: &metamodelica::List<Attribute>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut res: Label;
    let mut s: Label;
    res = makeAttrReq(l, literal!(""))?;
    s = stringAppend(literal!("["), res);
    r#str = stringAppend(s, literal!("]"));
    Ok(r#str)
}

fn makeAttrReq<'__b>(mut inAttributeLst: &'__b metamodelica::List<Attribute>, mut inString: ArcStr) -> Result<ArcStr> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAttributeLst {
            Deref @ metamodelica::ListNode::Cons { head: Attribute { name, value: v }, tail: Deref @ metamodelica::ListNode::Nil } => {
                let mut s: Label;
                s = stringAppend(inString, name.clone());
                s = stringAppend(s, literal!("="));
                return Ok(stringAppend(s, v.clone()))
            },
            Deref @ metamodelica::ListNode::Cons { head: Attribute { name, value: v }, tail: rest } => {
                let mut s: Label;
                s = stringAppend(inString, name.clone());
                s = stringAppend(s, literal!("="));
                s = stringAppend(s, v.clone());
                s = stringAppend(s, literal!(","));
                { (inAttributeLst, inString) = (rest, s); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}
