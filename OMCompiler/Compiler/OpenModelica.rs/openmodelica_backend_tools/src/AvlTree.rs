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

pub type FuncTypeKeyToStr<Key: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key) -> Result<ArcStr> + 'static>;

pub type FuncTypeValToStr<Val: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Val) -> Result<ArcStr> + 'static>;

pub type FuncTypeItemUpdateCheck<Key: Clone + 'static, Val: Clone + 'static> =
    std::sync::Arc<dyn ::std::ops::Fn(Item<Key, Val>, Item<Key, Val>) -> Result<bool> + 'static>;

pub type FuncTypeKeyCompare<Key: Clone + 'static> =
    std::sync::Arc<dyn ::std::ops::Fn(Key, Key) -> Result<i32> + 'static>;

/// a tree is a node and two optional printing functions
/// a tree is a node and two optional printing functions
#[derive(Clone, metamodelica::MMCtor, metamodelica::ReferenceEq)]
pub struct Tree<Key: Clone, Val: Clone> {
    pub root: metamodelica::Ref<Node<Key, Val>>,
    /// function to compare keys, should return -1, 0, 1 ONLY!
    pub keyCompareFunc: FuncTypeKeyCompare<Key>,
    /// optional function for printing Key
    pub keyStrFuncOpt: Option<FuncTypeKeyToStr<Key>>,
    /// optional function for printing Val
    pub valStrFuncOpt: Option<FuncTypeValToStr<Val>>,
    /// optional function for reporting error on an update of the same item
    ///       if this function is NONE() then updates of items with the same key is allowed!
    ///       this function gets the new item and the old item for easy reporting,
    ///       and should return:
    ///       - true if update is allowed
    ///       - false if update should not be done
    ///       - should print an error message and fail if it wants to fail the update
    pub updateCheckFuncOpt: Option<FuncTypeItemUpdateCheck<Key, Val>>,
    /// a name for this tree so you know which one it is if you have more
    pub name: ArcStr,
}

impl<Key: Clone + metamodelica::gc::MMTrace, Val: Clone + metamodelica::gc::MMTrace> metamodelica::gc::MMTrace
    for Tree<Key, Val>
{
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.root, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.keyCompareFunc, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.keyStrFuncOpt, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.valStrFuncOpt, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.updateCheckFuncOpt, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        Ok(())
    }
}
impl<Key: Clone + 'static + PartialEq, Val: Clone + 'static + PartialEq> PartialEq for Tree<Key, Val> {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
            && std::sync::Arc::ptr_eq((&self.keyCompareFunc), (&other.keyCompareFunc))
            && (match ((&self.keyStrFuncOpt), (&other.keyStrFuncOpt)) {
                (Some(__lo), Some(__ro)) => std::sync::Arc::ptr_eq(__lo, __ro),
                (None, None) => true,
                _ => false,
            })
            && (match ((&self.valStrFuncOpt), (&other.valStrFuncOpt)) {
                (Some(__lo), Some(__ro)) => std::sync::Arc::ptr_eq(__lo, __ro),
                (None, None) => true,
                _ => false,
            })
            && (match ((&self.updateCheckFuncOpt), (&other.updateCheckFuncOpt)) {
                (Some(__lo), Some(__ro)) => std::sync::Arc::ptr_eq(__lo, __ro),
                (None, None) => true,
                _ => false,
            })
            && self.name == other.name
    }
}
impl<Key: Clone + 'static + PartialEq + Eq, Val: Clone + 'static + PartialEq + Eq> Eq for Tree<Key, Val> {}
impl<Key: Clone + 'static + PartialEq + Eq + PartialOrd + Ord, Val: Clone + 'static + PartialEq + Eq + PartialOrd + Ord>
    PartialOrd for Tree<Key, Val>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<Key: Clone + 'static + PartialEq + Eq + PartialOrd + Ord, Val: Clone + 'static + PartialEq + Eq + PartialOrd + Ord>
    Ord for Tree<Key, Val>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.root
            .cmp(&other.root)
            .then_with(|| {
                (std::sync::Arc::as_ptr((&self.keyCompareFunc)) as *const ())
                    .cmp(&(std::sync::Arc::as_ptr((&other.keyCompareFunc)) as *const ()))
            })
            .then_with(|| {
                (match ((&self.keyStrFuncOpt), (&other.keyStrFuncOpt)) {
                    (Some(__lo), Some(__ro)) => {
                        (std::sync::Arc::as_ptr(__lo) as *const ()).cmp(&(std::sync::Arc::as_ptr(__ro) as *const ()))
                    }
                    (None, None) => std::cmp::Ordering::Equal,
                    (None, Some(_)) => std::cmp::Ordering::Less,
                    (Some(_), None) => std::cmp::Ordering::Greater,
                })
            })
            .then_with(|| {
                (match ((&self.valStrFuncOpt), (&other.valStrFuncOpt)) {
                    (Some(__lo), Some(__ro)) => {
                        (std::sync::Arc::as_ptr(__lo) as *const ()).cmp(&(std::sync::Arc::as_ptr(__ro) as *const ()))
                    }
                    (None, None) => std::cmp::Ordering::Equal,
                    (None, Some(_)) => std::cmp::Ordering::Less,
                    (Some(_), None) => std::cmp::Ordering::Greater,
                })
            })
            .then_with(|| {
                (match ((&self.updateCheckFuncOpt), (&other.updateCheckFuncOpt)) {
                    (Some(__lo), Some(__ro)) => {
                        (std::sync::Arc::as_ptr(__lo) as *const ()).cmp(&(std::sync::Arc::as_ptr(__ro) as *const ()))
                    }
                    (None, None) => std::cmp::Ordering::Equal,
                    (None, Some(_)) => std::cmp::Ordering::Less,
                    (Some(_), None) => std::cmp::Ordering::Greater,
                })
            })
            .then_with(|| self.name.cmp(&other.name))
    }
}
impl<Key: Clone + 'static + std::hash::Hash, Val: Clone + 'static + std::hash::Hash> std::hash::Hash
    for Tree<Key, Val>
{
    fn hash<__H: std::hash::Hasher>(&self, __state: &mut __H) {
        self.root.hash(__state);
        (std::sync::Arc::as_ptr((&self.keyCompareFunc)) as *const ()).hash(__state);
        match (&self.keyStrFuncOpt) {
            Some(__ho) => {
                1u8.hash(__state);
                (std::sync::Arc::as_ptr(__ho) as *const ()).hash(__state);
            }
            None => 0u8.hash(__state),
        }
        match (&self.valStrFuncOpt) {
            Some(__ho) => {
                1u8.hash(__state);
                (std::sync::Arc::as_ptr(__ho) as *const ()).hash(__state);
            }
            None => 0u8.hash(__state),
        }
        match (&self.updateCheckFuncOpt) {
            Some(__ho) => {
                1u8.hash(__state);
                (std::sync::Arc::as_ptr(__ho) as *const ()).hash(__state);
            }
            None => 0u8.hash(__state),
        }
        self.name.hash(__state);
    }
}
impl<Key: Clone + 'static + std::fmt::Debug, Val: Clone + 'static + std::fmt::Debug> std::fmt::Debug
    for Tree<Key, Val>
{
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut __ds = __f.debug_struct("Tree");
        __ds.field("root", &self.root);
        __ds.field(
            "keyCompareFunc",
            &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr((&self.keyCompareFunc))),
        );
        __ds.field(
            "keyStrFuncOpt",
            &format_args!("<dyn-fn-container@{:p}>", (&self.keyStrFuncOpt) as *const _),
        );
        __ds.field(
            "valStrFuncOpt",
            &format_args!("<dyn-fn-container@{:p}>", (&self.valStrFuncOpt) as *const _),
        );
        __ds.field(
            "updateCheckFuncOpt",
            &format_args!("<dyn-fn-container@{:p}>", (&self.updateCheckFuncOpt) as *const _),
        );
        __ds.field("name", &self.name);
        __ds.finish()
    }
}

pub type TREE<Key, Val> = Tree<Key, Val>;

/// The binary tree data structure
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Node<Key, Val> {
    NODE {
        /// Val
        item: Item<Key, Val>,
        /// height of tree, used for balancing
        height: i32,
        /// left subtree
        left: metamodelica::Ref<Node<Key, Val>>,
        /// right subtree
        right: metamodelica::Ref<Node<Key, Val>>,
    },
    /// no node, empty tree
    NO_NODE,
}
impl<Key: Clone + metamodelica::gc::MMTrace, Val: Clone + metamodelica::gc::MMTrace> metamodelica::gc::MMTrace
    for Node<Key, Val>
{
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Node::NODE {
                item,
                height,
                left,
                right,
            } => {
                metamodelica::gc::MMTrace::mm_accept(item, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(height, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(left, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(right, __mmv)?;
                Ok(())
            }
            Node::NO_NODE => Ok(()),
        }
    }
}
pub(crate) use self::Node::{NO_NODE, NODE};

/// Each node in the binary tree can have an item associated with it.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Item<Key, Val> {
    ITEM {
        /// Key
        key: Key,
        /// Val
        val: Val,
    },
    /// no item
    NO_ITEM,
}
impl<Key: Clone + metamodelica::gc::MMTrace, Val: Clone + metamodelica::gc::MMTrace> metamodelica::gc::MMTrace
    for Item<Key, Val>
{
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Item::ITEM { key, val } => {
                metamodelica::gc::MMTrace::mm_accept(key, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(val, __mmv)?;
                Ok(())
            }
            Item::NO_ITEM => Ok(()),
        }
    }
}
pub(crate) use self::Item::{ITEM, NO_ITEM};

pub(crate) fn name<
    Key: Clone + 'static + metamodelica::gc::MMTrace,
    Val: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut tree: Tree<Key, Val>,
) -> ArcStr {
    let mut name: ArcStr;
    let Tree { name: __pa0, .. } = tree;
    name = metamodelica::Own::own(__pa0);
    name
}

pub(crate) fn create<
    Key: Clone + 'static + metamodelica::gc::MMTrace,
    Val: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut name: ArcStr,
    mut inKeyCompareFunc: Arc<dyn ::std::ops::Fn(Key, Key) -> Result<i32> + 'static>,
    mut inKeyStrFuncOpt: Option<Arc<dyn ::std::ops::Fn(Key) -> Result<ArcStr> + 'static>>,
    mut inValStrFuncOpt: Option<Arc<dyn ::std::ops::Fn(Val) -> Result<ArcStr> + 'static>>,
    mut inUpdateCheckFuncOpt: Option<Arc<dyn ::std::ops::Fn(Item<Key, Val>, Item<Key, Val>) -> Result<bool> + 'static>>,
) -> Tree<Key, Val> {
    let mut tree: Tree<Key, Val>;
    tree = Tree {
        root: metamodelica::Ref::new(Node::NODE {
            item: crate::AvlTree::Item::NO_ITEM,
            height: 0,
            left: metamodelica::Ref::new(crate::AvlTree::Node::NO_NODE),
            right: metamodelica::Ref::new(crate::AvlTree::Node::NO_NODE),
        }),
        keyCompareFunc: inKeyCompareFunc.clone(),
        keyStrFuncOpt: inKeyStrFuncOpt,
        valStrFuncOpt: inValStrFuncOpt,
        updateCheckFuncOpt: inUpdateCheckFuncOpt,
        name: name,
    };
    tree
}

pub(crate) fn hasPrintingFunctions<
    Key: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
    Val: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
>(
    mut tree: Tree<Key, Val>,
) -> bool {
    let mut hasPrinting: bool;
    let mut kf: Option<FuncTypeKeyToStr<Key>>;
    let mut vf: Option<FuncTypeValToStr<Val>>;
    let Tree {
        keyStrFuncOpt: __pa0,
        valStrFuncOpt: __pa1,
        ..
    } = tree;
    kf = metamodelica::Own::own(__pa0);
    vf = metamodelica::Own::own(__pa1);
    hasPrinting = boolNot(boolOr((kf).is_none(), (vf).is_none()));
    hasPrinting
}

pub(crate) fn hasUpdateCheckFunction<
    Key: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
    Val: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
>(
    mut tree: Tree<Key, Val>,
) -> bool {
    let mut hasUpdateCheck: bool;
    let mut uf: Option<FuncTypeItemUpdateCheck<Key, Val>>;
    let Tree {
        updateCheckFuncOpt: __pa0,
        ..
    } = tree;
    uf = metamodelica::Own::own(__pa0);
    hasUpdateCheck = boolNot((uf).is_none());
    hasUpdateCheck
}

pub(crate) fn getUpdateCheckFunc<
    Key: Clone + 'static + metamodelica::gc::MMTrace,
    Val: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut tree: Tree<Key, Val>,
) -> Result<Arc<dyn ::std::ops::Fn(Item<Key, Val>, Item<Key, Val>) -> Result<bool> + 'static>> {
    let mut outUpdateCheckFunc: FuncTypeItemUpdateCheck<Key, Val>;
    let __pa0 = ::match_deref::match_deref! { match &(tree) {
        Tree { updateCheckFuncOpt: Some(__pa0), .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outUpdateCheckFunc = metamodelica::Own::own(__pa0);
    Ok(outUpdateCheckFunc)
}

pub(crate) fn getKeyCompareFunc<
    Key: Clone + 'static + metamodelica::gc::MMTrace,
    Val: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut tree: Tree<Key, Val>,
) -> Arc<dyn ::std::ops::Fn(Key, Key) -> Result<i32> + 'static> {
    let mut outKeyCompareFunc: FuncTypeKeyCompare<Key>;
    let Tree {
        keyCompareFunc: __pa0, ..
    } = tree;
    outKeyCompareFunc = metamodelica::Own::own(__pa0);
    outKeyCompareFunc
}

pub(crate) fn getKeyToStrFunc<
    Key: Clone + 'static + metamodelica::gc::MMTrace,
    Val: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut tree: Tree<Key, Val>,
) -> Result<Arc<dyn ::std::ops::Fn(Key) -> Result<ArcStr> + 'static>> {
    let mut outKey2StrFunc: FuncTypeKeyToStr<Key>;
    let __pa0 = ::match_deref::match_deref! { match &(tree) {
        Tree { keyStrFuncOpt: Some(__pa0), .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outKey2StrFunc = metamodelica::Own::own(__pa0);
    Ok(outKey2StrFunc)
}

pub(crate) fn getValToStrFunc<
    Key: Clone + 'static + metamodelica::gc::MMTrace,
    Val: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut tree: Tree<Key, Val>,
) -> Result<Arc<dyn ::std::ops::Fn(Val) -> Result<ArcStr> + 'static>> {
    let mut outVal2StrFunc: FuncTypeValToStr<Val>;
    let __pa0 = ::match_deref::match_deref! { match &(tree) {
        Tree { valStrFuncOpt: Some(__pa0), .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outVal2StrFunc = metamodelica::Own::own(__pa0);
    Ok(outVal2StrFunc)
}

fn newLeafNode<Key: Clone + 'static + metamodelica::gc::MMTrace, Val: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inItem: Item<Key, Val>,
    mut height: i32,
) -> metamodelica::Ref<Node<Key, Val>> {
    let mut outNode: metamodelica::Ref<Node<Key, Val>>;
    outNode = metamodelica::Ref::new(Node::NODE {
        item: inItem,
        height: 1,
        left: metamodelica::Ref::new(crate::AvlTree::Node::NO_NODE),
        right: metamodelica::Ref::new(crate::AvlTree::Node::NO_NODE),
    });
    outNode
}

pub(crate) fn add<
    Key: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
    Val: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
>(
    mut inTree: Tree<Key, Val>,
    mut inKey: Key,
    mut inVal: Val,
) -> Result<Tree<Key, Val>> {
    let mut outTree: Tree<Key, Val>;
    outTree = 'mc: {
        let __mc_input = (inTree.clone(), inKey, inVal);
        if let Ok(__v) = (|| -> Result<_> {
            let (
                Tree {
                    root: ref node,
                    keyCompareFunc: mut cf,
                    keyStrFuncOpt: mut kf,
                    valStrFuncOpt: mut vf,
                    updateCheckFuncOpt: mut uf,
                    name: mut n,
                },
                mut key,
                mut val,
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut node = node.clone();
            node = addNode(inTree.clone(), node.clone(), key.clone(), val.clone())?;
            Ok(Tree {
                root: node.clone(),
                keyCompareFunc: cf.clone(),
                keyStrFuncOpt: kf.clone(),
                valStrFuncOpt: vf.clone(),
                updateCheckFuncOpt: uf.clone(),
                name: n.clone(),
            })
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut r#str: ArcStr;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("AvlTree.add name: "));
                __mm_s.push_str(&*name(inTree.clone()));
                __mm_s.push_str(&*literal!(" failed!"));
                ArcStr::from(__mm_s)
            };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str.clone()])?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTree)
}

fn addNode<
    Key: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
    Val: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
>(
    mut inTree: Tree<Key, Val>,
    mut inNode: metamodelica::Ref<Node<Key, Val>>,
    mut inKey: Key,
    mut inVal: Val,
) -> Result<metamodelica::Ref<Node<Key, Val>>> {
    let mut outNode: metamodelica::Ref<Node<Key, Val>>;
    outNode = (::match_deref::match_deref! { match &((inTree.clone(), inNode.clone())) {
        (_, Deref @ Node::NO_NODE { .. }) => {
            let mut n: metamodelica::Ref<Node<Key, Val>>;
            n = newLeafNode(Item::ITEM { key: inKey, val: inVal }, 1);
            n
        },
        (_, Deref @ Node::NODE { item: Item::NO_ITEM { .. }, left: Deref @ Node::NO_NODE { .. }, right: Deref @ Node::NO_NODE { .. }, .. }) => {
            let mut key = inKey.clone();
            let mut val = inVal.clone();
            let mut n: metamodelica::Ref<Node<Key, Val>>;
            n = newLeafNode(Item::ITEM { key: key, val: val }, 1);
            n
        },
        (Tree { keyCompareFunc, .. }, Deref @ Node::NODE { item: Item::ITEM { key: rkey, .. }, .. }) => {
            let mut key = inKey.clone();
            let mut val = inVal.clone();
            let mut n: metamodelica::Ref<Node<Key, Val>>;
            let mut order: i32;
            order = keyCompareFunc(key.clone(), rkey.clone())?;
            n = balance(addNode_dispatch(inTree, inNode, order, key, val)?)?;
            n
        },
        _ => {
            let mut r#str: ArcStr;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("AvlTree.addNode name: ")); __mm_s.push_str(&*name(inTree)); __mm_s.push_str(&*literal!(" failed!")); ArcStr::from(__mm_s) };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outNode)
}

fn addNode_dispatch<
    Key: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
    Val: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
>(
    mut inTree: Tree<Key, Val>,
    mut inNode: metamodelica::Ref<Node<Key, Val>>,
    mut inKeyComp: i32,
    mut inKey: Key,
    mut inVal: Val,
) -> Result<metamodelica::Ref<Node<Key, Val>>> {
    let mut outNode: metamodelica::Ref<Node<Key, Val>>;
    outNode = 'mc: {
        let __mc_input = (&*inNode, inKeyComp, inKey, inVal);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Node::NODE { item: _, height: h, left: l, right: r }, 0, key, val) => {
                    let false = (hasUpdateCheckFunction(inTree.clone())) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(Node::NODE { item: Item::ITEM { key: key.clone(), val: val.clone() }, height: h.clone(), left: l.clone(), right: r.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Node::NODE { item: i, height: h, left: l, right: r }, 0, key, val) => {
                    let mut updateCheckFunc: FuncTypeItemUpdateCheck<Key, Val>;
                    let true = (hasUpdateCheckFunction(inTree.clone())) else { return Err("pattern mismatch") };
                    updateCheckFunc = getUpdateCheckFunc(inTree.clone())?;
                    let true = (updateCheckFunc(i.clone(), Item::ITEM { key: key.clone(), val: val.clone() })?) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(Node::NODE { item: Item::ITEM { key: key.clone(), val: val.clone() }, height: h.clone(), left: l.clone(), right: r.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Node::NODE { item: i, height: _, left: _, right: _ }, 0, key, val) => {
                    let mut updateCheckFunc: FuncTypeItemUpdateCheck<Key, Val>;
                    let true = (hasUpdateCheckFunction(inTree.clone())) else { return Err("pattern mismatch") };
                    updateCheckFunc = getUpdateCheckFunc(inTree.clone())?;
                    let false = (updateCheckFunc(i.clone(), Item::ITEM { key: key.clone(), val: val.clone() })?) else { return Err("pattern mismatch") };
                    Ok(inNode.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Node::NODE { item: i, height: h, left: l, right: r }, 1, key, val) => {
                    let mut n: metamodelica::Ref<Node<Key, Val>>;
                    n = emptyNodeIfNoNode(r.clone());
                    n = addNode(inTree.clone(), n.clone(), key.clone(), val.clone())?;
                    Ok(metamodelica::Ref::new(Node::NODE { item: i.clone(), height: h.clone(), left: l.clone(), right: n.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Node::NODE { item: i, height: h, left: l, right: r }, (-1), key, val) => {
                    let mut n: metamodelica::Ref<Node<Key, Val>>;
                    n = emptyNodeIfNoNode(l.clone());
                    n = addNode(inTree.clone(), n.clone(), key.clone(), val.clone())?;
                    Ok(metamodelica::Ref::new(Node::NODE { item: i.clone(), height: h.clone(), left: n.clone(), right: r.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outNode)
}

pub(crate) fn get<
    Key: Clone + 'static + metamodelica::gc::MMTrace,
    Val: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inTree: Tree<Key, Val>,
    mut inKey: Key,
) -> Result<Val> {
    let mut outVal: Val;
    let mut node: metamodelica::Ref<Node<Key, Val>>;
    let Tree { root: __pa0, .. } = &inTree;
    node = metamodelica::Own::own(__pa0);
    outVal = getNode(inTree, node, inKey)?;
    Ok(outVal)
}

fn getNode<Key: Clone + 'static + metamodelica::gc::MMTrace, Val: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inTree: Tree<Key, Val>,
    mut inNode: metamodelica::Ref<Node<Key, Val>>,
    mut inKey: Key,
) -> Result<Val> {
    let mut outVal: Val;
    let mut rkey: Key;
    let mut keyCompareFunc: FuncTypeKeyCompare<Key>;
    let mut order: i32;
    let __pa0 = ::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Node::NODE { item: Item::ITEM { key: __pa0, .. }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    rkey = metamodelica::Own::own(__pa0);
    keyCompareFunc = getKeyCompareFunc(inTree.clone());
    order = keyCompareFunc(inKey.clone(), rkey)?;
    outVal = getNode_dispatch(inTree, inNode, order, inKey)?;
    Ok(outVal)
}

fn getNode_dispatch<
    Key: Clone + 'static + metamodelica::gc::MMTrace,
    Val: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inTree: Tree<Key, Val>,
    mut inNode: metamodelica::Ref<Node<Key, Val>>,
    mut inKeyComp: i32,
    mut inKey: Key,
) -> Result<Val> {
    let mut outVal: Val;
    outVal = (::match_deref::match_deref! { match &((inNode, inKeyComp)) {
        (Deref @ Node::NODE { item: Item::ITEM { val, .. }, .. }, 0) => {
            val.clone()
        },
        (Deref @ Node::NODE { right: r, .. }, 1) => {
            let mut key = inKey;
            getNode(inTree, r.clone(), key)?
        },
        (Deref @ Node::NODE { left: l, .. }, (-1)) => {
            let mut key = inKey;
            getNode(inTree, l.clone(), key)?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outVal)
}

pub(crate) fn replace<
    Key: Clone + 'static + metamodelica::gc::MMTrace,
    Val: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inTree: Tree<Key, Val>,
    mut inKey: Key,
    mut inVal: Val,
) -> Result<Tree<Key, Val>> {
    let mut outTree: Tree<Key, Val>;
    outTree = (match (inTree.clone(), inKey, inVal) {
        (
            Tree {
                root: ref node,
                keyCompareFunc: mut keyCompareFunc,
                keyStrFuncOpt: mut kf,
                valStrFuncOpt: mut vf,
                updateCheckFuncOpt: mut uf,
                name: mut n,
            },
            mut key,
            mut val,
        ) => {
            let mut node = node.clone();
            node = replaceNode(inTree, node.clone(), key, val)?;
            Tree {
                root: node.clone(),
                keyCompareFunc: keyCompareFunc.clone(),
                keyStrFuncOpt: kf.clone(),
                valStrFuncOpt: vf.clone(),
                updateCheckFuncOpt: uf.clone(),
                name: n.clone(),
            }
        }
        _ => {
            let mut r#str: ArcStr;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("AvlTree.replace name: "));
                __mm_s.push_str(&*name(inTree));
                __mm_s.push_str(&*literal!(" failed!"));
                ArcStr::from(__mm_s)
            };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str])?;
            return Err("fail");
        }
    });
    Ok(outTree)
}

pub(crate) fn replaceNode<
    Key: Clone + 'static + metamodelica::gc::MMTrace,
    Val: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inTree: Tree<Key, Val>,
    mut inNode: metamodelica::Ref<Node<Key, Val>>,
    mut inKey: Key,
    mut inVal: Val,
) -> Result<metamodelica::Ref<Node<Key, Val>>> {
    let mut outNode: metamodelica::Ref<Node<Key, Val>>;
    outNode = (::match_deref::match_deref! { match &((inTree.clone(), inNode.clone())) {
        (Tree { keyCompareFunc, .. }, Deref @ Node::NODE { item: Item::ITEM { key: rkey, .. }, .. }) => {
            let mut key = inKey;
            let mut val = inVal;
            let mut n: metamodelica::Ref<Node<Key, Val>>;
            let mut order: i32;
            order = keyCompareFunc(key.clone(), rkey.clone())?;
            n = replaceNode_dispatch(inTree, inNode, order, key, val)?;
            n
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outNode)
}

fn replaceNode_dispatch<
    Key: Clone + 'static + metamodelica::gc::MMTrace,
    Val: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inTree: Tree<Key, Val>,
    mut inNode: metamodelica::Ref<Node<Key, Val>>,
    mut inKeyComp: i32,
    mut inKey: Key,
    mut inVal: Val,
) -> Result<metamodelica::Ref<Node<Key, Val>>> {
    let mut outNode: metamodelica::Ref<Node<Key, Val>>;
    outNode = (::match_deref::match_deref! { match &((inNode, inKeyComp)) {
        (Deref @ Node::NODE { item: Item::ITEM { .. }, height: h, left: l, right: r }, 0) => {
            let mut key = inKey;
            let mut val = inVal;
            metamodelica::Ref::new(Node::NODE { item: Item::ITEM { key: key, val: val }, height: h.clone(), left: l.clone(), right: r.clone() })
        },
        (Deref @ Node::NODE { item: i, height: h, left: l, right: r }, 1) => {
            let mut key = inKey;
            let mut val = inVal;
            let mut n: metamodelica::Ref<Node<Key, Val>>;
            n = emptyNodeIfNoNode(r.clone());
            n = replaceNode(inTree, n, key, val)?;
            metamodelica::Ref::new(Node::NODE { item: i.clone(), height: h.clone(), left: l.clone(), right: n })
        },
        (Deref @ Node::NODE { item: i, height: h, left: l, right: r }, (-1)) => {
            let mut key = inKey;
            let mut val = inVal;
            let mut n: metamodelica::Ref<Node<Key, Val>>;
            n = emptyNodeIfNoNode(l.clone());
            n = replaceNode(inTree, n, key, val)?;
            metamodelica::Ref::new(Node::NODE { item: i.clone(), height: h.clone(), left: n, right: r.clone() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outNode)
}

fn emptyNodeIfNoNode<
    Key: Clone + 'static + metamodelica::gc::MMTrace,
    Val: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inNode: metamodelica::Ref<Node<Key, Val>>,
) -> metamodelica::Ref<Node<Key, Val>> {
    let mut outNode: metamodelica::Ref<Node<Key, Val>>;
    outNode = (match &*inNode {
        Node::NO_NODE { .. } => metamodelica::Ref::new(Node::NODE {
            item: crate::AvlTree::Item::NO_ITEM,
            height: 0,
            left: metamodelica::Ref::new(crate::AvlTree::Node::NO_NODE),
            right: metamodelica::Ref::new(crate::AvlTree::Node::NO_NODE),
        }),
        Node::NODE { .. } => inNode,
        _ => panic!("match: no arm matched"),
    });
    outNode
}

fn balance<Key: Clone + 'static + metamodelica::gc::MMTrace, Val: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inNode: metamodelica::Ref<Node<Key, Val>>,
) -> Result<metamodelica::Ref<Node<Key, Val>>> {
    let mut outNode: metamodelica::Ref<Node<Key, Val>>;
    let mut d: i32;
    d = differenceInHeight(inNode.clone())?;
    outNode = doBalance(d, inNode)?;
    Ok(outNode)
}

fn doBalance<Key: Clone + 'static + metamodelica::gc::MMTrace, Val: Clone + 'static + metamodelica::gc::MMTrace>(
    mut difference: i32,
    mut inNode: metamodelica::Ref<Node<Key, Val>>,
) -> Result<metamodelica::Ref<Node<Key, Val>>> {
    let mut outNode: metamodelica::Ref<Node<Key, Val>>;
    outNode = (match difference {
        (-1) => computeHeight(inNode)?,
        0 => computeHeight(inNode)?,
        1 => computeHeight(inNode)?,
        _ => doBalance2(difference < 0, inNode)?,
    });
    Ok(outNode)
}

fn doBalance2<Key: Clone + 'static + metamodelica::gc::MMTrace, Val: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inDiffIsNegative: bool,
    mut inNode: metamodelica::Ref<Node<Key, Val>>,
) -> Result<metamodelica::Ref<Node<Key, Val>>> {
    let mut outNode: metamodelica::Ref<Node<Key, Val>>;
    outNode = (match inDiffIsNegative {
        true => {
            let mut n = inNode;
            n = doBalance3(n);
            n = rotateLeft(n)?;
            n
        }
        false => {
            let mut n = inNode;
            n = doBalance4(n);
            n = rotateRight(n)?;
            n
        }
    });
    Ok(outNode)
}

fn doBalance3<Key: Clone + 'static + metamodelica::gc::MMTrace, Val: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inNode: metamodelica::Ref<Node<Key, Val>>,
) -> metamodelica::Ref<Node<Key, Val>> {
    let mut outNode: metamodelica::Ref<Node<Key, Val>>;
    outNode = 'mc: {
        let __mc_input = inNode.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                n => {
                    let mut rr: metamodelica::Ref<Node<Key, Val>>;
                    let mut rN: metamodelica::Ref<Node<Key, Val>>;
                    let mut n = (*n).clone();
                    rN = rightNode(n.clone())?;
                    let true = (differenceInHeight(rN.clone())? > 0) else { return Err("pattern mismatch") };
                    rr = rotateRight(rN.clone())?;
                    n = setRight(n.clone(), rr.clone())?;
                    Ok(n.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inNode.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outNode
}

fn doBalance4<Key: Clone + 'static + metamodelica::gc::MMTrace, Val: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inNode: metamodelica::Ref<Node<Key, Val>>,
) -> metamodelica::Ref<Node<Key, Val>> {
    let mut outNode: metamodelica::Ref<Node<Key, Val>>;
    outNode = 'mc: {
        let __mc_input = inNode.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                n => {
                    let mut rl: metamodelica::Ref<Node<Key, Val>>;
                    let mut lN: metamodelica::Ref<Node<Key, Val>>;
                    let mut n = (*n).clone();
                    lN = leftNode(n.clone())?;
                    let true = (differenceInHeight(lN.clone())? < 0) else { return Err("pattern mismatch") };
                    rl = rotateLeft(lN.clone())?;
                    n = setLeft(n.clone(), rl.clone())?;
                    Ok(n.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inNode.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outNode
}

fn setRight<Key: Clone + 'static + metamodelica::gc::MMTrace, Val: Clone + 'static + metamodelica::gc::MMTrace>(
    mut node: metamodelica::Ref<Node<Key, Val>>,
    mut right: metamodelica::Ref<Node<Key, Val>>,
) -> Result<metamodelica::Ref<Node<Key, Val>>> {
    let mut outNode: metamodelica::Ref<Node<Key, Val>>;
    let mut item: Item<Key, Val>;
    let mut l: metamodelica::Ref<Node<Key, Val>>;
    let mut height: i32;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(node) {
        Deref @ Node::NODE { item: __pa0, height: __pa1, left: __pa2, right: _ } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    item = metamodelica::Own::own(__pa0);
    height = metamodelica::Own::own(__pa1);
    l = metamodelica::Own::own(__pa2);
    outNode = metamodelica::Ref::new(Node::NODE {
        item: item,
        height: height,
        left: l,
        right: right,
    });
    Ok(outNode)
}

fn setLeft<Key: Clone + 'static + metamodelica::gc::MMTrace, Val: Clone + 'static + metamodelica::gc::MMTrace>(
    mut node: metamodelica::Ref<Node<Key, Val>>,
    mut left: metamodelica::Ref<Node<Key, Val>>,
) -> Result<metamodelica::Ref<Node<Key, Val>>> {
    let mut outNode: metamodelica::Ref<Node<Key, Val>>;
    let mut item: Item<Key, Val>;
    let mut r: metamodelica::Ref<Node<Key, Val>>;
    let mut height: i32;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(node) {
        Deref @ Node::NODE { item: __pa0, height: __pa1, left: _, right: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    item = metamodelica::Own::own(__pa0);
    height = metamodelica::Own::own(__pa1);
    r = metamodelica::Own::own(__pa2);
    outNode = metamodelica::Ref::new(Node::NODE {
        item: item,
        height: height,
        left: left,
        right: r,
    });
    Ok(outNode)
}

fn leftNode<Key: Clone + 'static + metamodelica::gc::MMTrace, Val: Clone + 'static + metamodelica::gc::MMTrace>(
    mut node: metamodelica::Ref<Node<Key, Val>>,
) -> Result<metamodelica::Ref<Node<Key, Val>>> {
    let mut subNode: metamodelica::Ref<Node<Key, Val>>;
    let __pa0 = ::match_deref::match_deref! { match &(node) {
        Deref @ Node::NODE { left: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    subNode = metamodelica::Own::own(__pa0);
    Ok(subNode)
}

fn rightNode<Key: Clone + 'static + metamodelica::gc::MMTrace, Val: Clone + 'static + metamodelica::gc::MMTrace>(
    mut node: metamodelica::Ref<Node<Key, Val>>,
) -> Result<metamodelica::Ref<Node<Key, Val>>> {
    let mut subNode: metamodelica::Ref<Node<Key, Val>>;
    let __pa0 = ::match_deref::match_deref! { match &(node) {
        Deref @ Node::NODE { right: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    subNode = metamodelica::Own::own(__pa0);
    Ok(subNode)
}

fn exchangeLeft<Key: Clone + 'static + metamodelica::gc::MMTrace, Val: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inNode: metamodelica::Ref<Node<Key, Val>>,
    mut inParent: metamodelica::Ref<Node<Key, Val>>,
) -> Result<metamodelica::Ref<Node<Key, Val>>> {
    let mut outParent: metamodelica::Ref<Node<Key, Val>>;
    let mut parent: metamodelica::Ref<Node<Key, Val>>;
    let mut node: metamodelica::Ref<Node<Key, Val>>;
    parent = setRight(inParent, leftNode(inNode.clone())?)?;
    parent = balance(parent)?;
    node = setLeft(inNode, parent)?;
    outParent = balance(node)?;
    Ok(outParent)
}

fn exchangeRight<Key: Clone + 'static + metamodelica::gc::MMTrace, Val: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inNode: metamodelica::Ref<Node<Key, Val>>,
    mut inParent: metamodelica::Ref<Node<Key, Val>>,
) -> Result<metamodelica::Ref<Node<Key, Val>>> {
    let mut outParent: metamodelica::Ref<Node<Key, Val>>;
    let mut parent: metamodelica::Ref<Node<Key, Val>>;
    let mut node: metamodelica::Ref<Node<Key, Val>>;
    parent = setLeft(inParent, rightNode(inNode.clone())?)?;
    parent = balance(parent)?;
    node = setRight(inNode, parent)?;
    outParent = balance(node)?;
    Ok(outParent)
}

fn rotateLeft<Key: Clone + 'static + metamodelica::gc::MMTrace, Val: Clone + 'static + metamodelica::gc::MMTrace>(
    mut node: metamodelica::Ref<Node<Key, Val>>,
) -> Result<metamodelica::Ref<Node<Key, Val>>> {
    let mut outNode: metamodelica::Ref<Node<Key, Val>>;
    outNode = exchangeLeft(rightNode(node.clone())?, node)?;
    Ok(outNode)
}

fn rotateRight<Key: Clone + 'static + metamodelica::gc::MMTrace, Val: Clone + 'static + metamodelica::gc::MMTrace>(
    mut node: metamodelica::Ref<Node<Key, Val>>,
) -> Result<metamodelica::Ref<Node<Key, Val>>> {
    let mut outNode: metamodelica::Ref<Node<Key, Val>>;
    outNode = exchangeRight(leftNode(node.clone())?, node)?;
    Ok(outNode)
}

fn differenceInHeight<
    Key: Clone + 'static + metamodelica::gc::MMTrace,
    Val: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut node: metamodelica::Ref<Node<Key, Val>>,
) -> Result<i32> {
    let mut diff: i32;
    let mut l: metamodelica::Ref<Node<Key, Val>>;
    let mut r: metamodelica::Ref<Node<Key, Val>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(node) {
        Deref @ Node::NODE { left: __pa0, right: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    l = metamodelica::Own::own(__pa0);
    r = metamodelica::Own::own(__pa1);
    diff = getHeight(l) - getHeight(r);
    Ok(diff)
}

fn computeHeight<Key: Clone + 'static + metamodelica::gc::MMTrace, Val: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inNode: metamodelica::Ref<Node<Key, Val>>,
) -> Result<metamodelica::Ref<Node<Key, Val>>> {
    let mut outNode: metamodelica::Ref<Node<Key, Val>>;
    let mut l: metamodelica::Ref<Node<Key, Val>>;
    let mut r: metamodelica::Ref<Node<Key, Val>>;
    let mut i: Item<Key, Val>;
    let mut hl: i32;
    let mut hr: i32;
    let mut height: i32;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inNode) {
        Deref @ Node::NODE { item: __pa0 @ Item::ITEM { .. }, left: __pa1, right: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    i = metamodelica::Own::own(__pa0);
    l = metamodelica::Own::own(__pa1);
    r = metamodelica::Own::own(__pa2);
    hl = getHeight(l.clone());
    hr = getHeight(r.clone());
    height = intMax(hl, hr) + 1;
    outNode = metamodelica::Ref::new(Node::NODE {
        item: i,
        height: height,
        left: l,
        right: r,
    });
    Ok(outNode)
}

fn getHeight<Key: Clone + 'static + metamodelica::gc::MMTrace, Val: Clone + 'static + metamodelica::gc::MMTrace>(
    mut bt: metamodelica::Ref<Node<Key, Val>>,
) -> i32 {
    let mut height: i32;
    height = (match &*bt {
        Node::NO_NODE { .. } => 0,
        Node::NODE {
            height: __esc_height, ..
        } => {
            height = (*__esc_height).clone();
            height.clone()
        }
        _ => panic!("match: no arm matched"),
    });
    height
}

pub(crate) fn prettyPrintTreeStr<
    Key: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
    Val: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
>(
    mut inTree: Tree<Key, Val>,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = prettyPrintTreeStr_dispatch(inTree, &(literal!("")))?;
    Ok(outString)
}

fn prettyPrintTreeStr_dispatch<
    Key: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
    Val: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
>(
    mut inTree: Tree<Key, Val>,
    mut inIndent: &ArcStr,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut node: metamodelica::Ref<Node<Key, Val>>;
    if !(hasPrintingFunctions(inTree.clone())) {
        outString = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("TreePrintError<NO_PRINTING_FUNCTIONS_ATTACHED> name["));
            __mm_s.push_str(&*name(inTree));
            __mm_s.push_str(&*literal!("]"));
            ArcStr::from(__mm_s)
        };
        return Ok(outString);
    }
    let Tree { root: __pa0, .. } = &inTree;
    node = metamodelica::Own::own(__pa0);
    outString = prettyPrintNodeStr(inTree, node, inIndent)?;
    Ok(outString)
}

fn prettyPrintNodeStr<
    Key: Clone + 'static + metamodelica::gc::MMTrace,
    Val: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inTree: Tree<Key, Val>,
    mut inNode: metamodelica::Ref<Node<Key, Val>>,
    mut inIndent: &ArcStr,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &*inNode {
        Node::NO_NODE { .. } => {
            literal!("")
        }
        Node::NODE {
            item: Item::NO_ITEM { .. },
            left: l,
            right: r,
            ..
        } => {
            let mut indent: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            indent = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*inIndent);
                __mm_s.push_str(&*literal!("  "));
                ArcStr::from(__mm_s)
            };
            s1 = prettyPrintNodeStr(inTree.clone(), l.clone(), &indent)?;
            s2 = prettyPrintNodeStr(inTree, r.clone(), &indent)?;
            res = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*s1);
                __mm_s.push_str(&*s2);
                ArcStr::from(__mm_s)
            };
            res
        }
        Node::NODE {
            item: item @ Item::ITEM { .. },
            left: l,
            right: r,
            ..
        } => {
            let mut indent: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            indent = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*inIndent);
                __mm_s.push_str(&*literal!("  "));
                ArcStr::from(__mm_s)
            };
            s1 = prettyPrintNodeStr(inTree.clone(), l.clone(), &indent)?;
            s2 = prettyPrintNodeStr(inTree.clone(), r.clone(), &indent)?;
            res = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*inIndent);
                __mm_s.push_str(&*printItemStr(inTree, item.clone())?);
                __mm_s.push_str(&*s1);
                __mm_s.push_str(&*s2);
                ArcStr::from(__mm_s)
            };
            res
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

pub(crate) fn printTreeStr<
    Key: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
    Val: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
>(
    mut inTree: Tree<Key, Val>,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut node: metamodelica::Ref<Node<Key, Val>>;
    if !(hasPrintingFunctions(inTree.clone())) {
        outString = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("TreePrintError<NO_PRINTING_FUNCTIONS_ATTACHED> name["));
            __mm_s.push_str(&*name(inTree));
            __mm_s.push_str(&*literal!("]"));
            ArcStr::from(__mm_s)
        };
        return Ok(outString);
    }
    let Tree { root: __pa0, .. } = &inTree;
    node = metamodelica::Own::own(__pa0);
    outString = printNodeStr(inTree, node)?;
    Ok(outString)
}

fn printNodeStr<Key: Clone + 'static + metamodelica::gc::MMTrace, Val: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inTree: Tree<Key, Val>,
    mut inNode: metamodelica::Ref<Node<Key, Val>>,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &*inNode {
        Node::NO_NODE { .. } => {
            literal!("")
        }
        Node::NODE {
            item: Item::NO_ITEM { .. },
            ..
        } => {
            literal!("")
        }
        Node::NODE {
            item: item @ Item::ITEM { .. },
            left,
            right,
            ..
        } => {
            let mut left_str: ArcStr;
            let mut right_str: ArcStr;
            let mut item_str: ArcStr;
            let mut r#str: ArcStr;
            left_str = printNodeStr(inTree.clone(), left.clone())?;
            right_str = printNodeStr(inTree.clone(), right.clone())?;
            item_str = printItemStr(inTree, item.clone())?;
            r#str = stringAppendList(list![
                literal!("i: "),
                item_str,
                literal!(", l: "),
                left_str,
                literal!(", r: "),
                right_str
            ]);
            r#str
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

pub(crate) fn printItemStr<
    Key: Clone + 'static + metamodelica::gc::MMTrace,
    Val: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inTree: Tree<Key, Val>,
    mut inItem: Item<Key, Val>,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inItem {
        Item::NO_ITEM { .. } => {
            literal!("[]")
        }
        Item::ITEM {
            key: mut key,
            val: mut val,
        } => {
            let mut r#str: ArcStr;
            let mut keyStr: ArcStr;
            let mut valStr: ArcStr;
            let mut key2Str: FuncTypeKeyToStr<Key>;
            let mut val2Str: FuncTypeValToStr<Val>;
            key2Str = getKeyToStrFunc(inTree.clone())?;
            val2Str = getValToStrFunc(inTree)?;
            keyStr = key2Str(key.clone())?;
            valStr = val2Str(val.clone())?;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("["));
                __mm_s.push_str(&*keyStr);
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*valStr);
                __mm_s.push_str(&*literal!("]"));
                ArcStr::from(__mm_s)
            };
            r#str
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

pub(crate) fn getKeyOfVal<
    Key: Clone + 'static + metamodelica::gc::MMTrace,
    Val: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
>(
    mut inTree: Tree<Key, Val>,
    mut inVal: Val,
) -> Result<Key> {
    let mut outKey: Key;
    let mut node: metamodelica::Ref<Node<Key, Val>>;
    let Tree { root: __pa0, .. } = &inTree;
    node = metamodelica::Own::own(__pa0);
    outKey = getKeyOfValNode(inTree, node, inVal)?;
    Ok(outKey)
}

fn getKeyOfValNode<
    Key: Clone + 'static + metamodelica::gc::MMTrace,
    Val: Clone + 'static + metamodelica::gc::MMTrace + PartialEq,
>(
    mut inTree: Tree<Key, Val>,
    mut inNode: metamodelica::Ref<Node<Key, Val>>,
    mut inVal: Val,
) -> Result<Key> {
    let mut outKey: Key;
    outKey = 'mc: {
        let __mc_input = inNode;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Node::NODE { item: Item::ITEM { key: k, val: v }, .. } => {
                    let true = (v.clone() == inVal.clone()) else { return Err("pattern mismatch") };
                    Ok(k.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Node::NODE { item: Item::ITEM { key: _, val: v }, left, .. } => {
                    let mut k: Key;
                    let false = (v.clone() == inVal.clone()) else { return Err("pattern mismatch") };
                    k = getKeyOfValNode(inTree.clone(), left.clone(), inVal.clone())?;
                    Ok(k.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Node::NODE { item: Item::ITEM { key: _, val: v }, right, .. } => {
                    let mut k: Key;
                    let false = (v.clone() == inVal.clone()) else { return Err("pattern mismatch") };
                    k = getKeyOfValNode(inTree.clone(), right.clone(), inVal.clone())?;
                    Ok(k.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outKey)
}

pub(crate) fn addUnique<
    Key: Clone + 'static + metamodelica::gc::MMTrace,
    Val: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inTree: Tree<Key, Val>,
    mut inKey: Key,
    mut inVal: Val,
) -> Result<(Tree<Key, Val>, Item<Key, Val>)> {
    let mut outTree: Tree<Key, Val>;
    let mut outItem: Item<Key, Val>;
    (outTree, outItem) = 'mc: {
        let __mc_input = (inTree.clone(), inKey, inVal);
        if let Ok(__v) = (|| -> Result<_> {
            let (
                Tree {
                    root: ref node,
                    keyCompareFunc: mut cf,
                    keyStrFuncOpt: mut kf,
                    valStrFuncOpt: mut vf,
                    updateCheckFuncOpt: mut uf,
                    name: mut n,
                },
                mut key,
                mut val,
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut item: Item<Key, Val>;
            let mut node = node.clone();
            (node, item) = addNodeUnique(inTree.clone(), node.clone(), key.clone(), val.clone())?;
            Ok((
                Tree {
                    root: node.clone(),
                    keyCompareFunc: cf.clone(),
                    keyStrFuncOpt: kf.clone(),
                    valStrFuncOpt: vf.clone(),
                    updateCheckFuncOpt: uf.clone(),
                    name: n.clone(),
                },
                item.clone(),
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut r#str: ArcStr;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("AvlTree.addUnique name: "));
                __mm_s.push_str(&*name(inTree.clone()));
                __mm_s.push_str(&*literal!(" failed!"));
                ArcStr::from(__mm_s)
            };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str.clone()])?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outTree, outItem))
}

fn addNodeUnique<Key: Clone + 'static + metamodelica::gc::MMTrace, Val: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inTree: Tree<Key, Val>,
    mut inNode: metamodelica::Ref<Node<Key, Val>>,
    mut inKey: Key,
    mut inVal: Val,
) -> Result<(metamodelica::Ref<Node<Key, Val>>, Item<Key, Val>)> {
    let mut outNode: metamodelica::Ref<Node<Key, Val>>;
    let mut outItem: Item<Key, Val>;
    (outNode, outItem) = (::match_deref::match_deref! { match &((inTree.clone(), inNode.clone())) {
        (_, Deref @ Node::NO_NODE { .. }) => {
            let mut item: Item<Key, Val>;
            let mut n: metamodelica::Ref<Node<Key, Val>>;
            item = Item::ITEM { key: inKey, val: inVal };
            n = newLeafNode(item.clone(), 1);
            (n, item)
        },
        (_, Deref @ Node::NODE { item: Item::NO_ITEM { .. }, left: Deref @ Node::NO_NODE { .. }, right: Deref @ Node::NO_NODE { .. }, .. }) => {
            let mut key = inKey.clone();
            let mut val = inVal.clone();
            let mut item: Item<Key, Val>;
            let mut n: metamodelica::Ref<Node<Key, Val>>;
            item = Item::ITEM { key: key, val: val };
            n = newLeafNode(item.clone(), 1);
            (n, item)
        },
        (Tree { keyCompareFunc, .. }, Deref @ Node::NODE { item: Item::ITEM { key: rkey, .. }, .. }) => {
            let mut key = inKey.clone();
            let mut val = inVal.clone();
            let mut item: Item<Key, Val>;
            let mut n: metamodelica::Ref<Node<Key, Val>>;
            let mut order: i32;
            order = keyCompareFunc(key.clone(), rkey.clone())?;
            (n, item) = addNodeUnique_dispatch(inTree, inNode, order, key, val)?;
            n = balance(n)?;
            (n, item)
        },
        _ => {
            let mut r#str: ArcStr;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("AvlTree.addNodeUnique name: ")); __mm_s.push_str(&*name(inTree)); __mm_s.push_str(&*literal!(" failed!")); ArcStr::from(__mm_s) };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outNode, outItem))
}

fn addNodeUnique_dispatch<
    Key: Clone + 'static + metamodelica::gc::MMTrace,
    Val: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut inTree: Tree<Key, Val>,
    mut inNode: metamodelica::Ref<Node<Key, Val>>,
    mut inKeyComp: i32,
    mut inKey: Key,
    mut inVal: Val,
) -> Result<(metamodelica::Ref<Node<Key, Val>>, Item<Key, Val>)> {
    let mut outNode: metamodelica::Ref<Node<Key, Val>>;
    let mut outItem: Item<Key, Val>;
    (outNode, outItem) = (::match_deref::match_deref! { match &((inNode.clone(), inKeyComp)) {
        (Deref @ Node::NODE { item: i, height: _, left: _, right: _ }, 0) => {
            (inNode, i.clone())
        },
        (Deref @ Node::NODE { item: i, height: h, left: l, right: r }, 1) => {
            let mut key = inKey;
            let mut val = inVal;
            let mut n: metamodelica::Ref<Node<Key, Val>>;
            let mut it: Item<Key, Val>;
            n = emptyNodeIfNoNode(r.clone());
            (n, it) = addNodeUnique(inTree, n, key, val)?;
            (metamodelica::Ref::new(Node::NODE { item: i.clone(), height: h.clone(), left: l.clone(), right: n }), it)
        },
        (Deref @ Node::NODE { item: i, height: h, left: l, right: r }, (-1)) => {
            let mut key = inKey;
            let mut val = inVal;
            let mut n: metamodelica::Ref<Node<Key, Val>>;
            let mut it: Item<Key, Val>;
            n = emptyNodeIfNoNode(l.clone());
            (n, it) = addNodeUnique(inTree, n, key, val)?;
            (metamodelica::Ref::new(Node::NODE { item: i.clone(), height: h.clone(), left: n, right: r.clone() }), it)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outNode, outItem))
}
