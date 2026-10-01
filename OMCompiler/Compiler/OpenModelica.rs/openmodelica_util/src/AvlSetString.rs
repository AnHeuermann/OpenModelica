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

use crate::BaseAvlSet;

pub type Key = ArcStr;

pub(crate) fn keyStr(mut inKey: Key) -> ArcStr {
    let mut outString: ArcStr;
    outString = inKey;
    outString
}

pub(crate) fn keyCompare(mut inKey1: Key, mut inKey2: Key) -> i32 {
    let mut outResult: i32;
    outResult = stringCompare(&inKey1, &inKey2);
    outResult
}

/// The binary tree data structure.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Tree {
    NODE {
        /// The key of the node.
        key: Key,
        /// Height of tree, used for balancing
        height: i32,
        /// Left subtree.
        left: metamodelica::Ref<Tree>,
        /// Right subtree.
        right: metamodelica::Ref<Tree>,
    },
    LEAF {
        /// The key of the node.
        key: Key,
    },
    EMPTY,
}
impl metamodelica::gc::MMTrace for Tree {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Tree::NODE {
                key,
                height,
                left,
                right,
            } => {
                metamodelica::gc::MMTrace::mm_accept(key, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(height, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(left, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(right, __mmv)?;
                Ok(())
            }
            Tree::LEAF { key } => {
                metamodelica::gc::MMTrace::mm_accept(key, __mmv)?;
                Ok(())
            }
            Tree::EMPTY => Ok(()),
        }
    }
}
impl Tree {
    pub fn interned_EMPTY() -> metamodelica::Ref<Tree> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<Tree>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(Tree::EMPTY));
        (*INTERNED).clone()
    }
}
pub fn interned_EMPTY() -> metamodelica::Ref<Tree> {
    Tree::interned_EMPTY()
}
impl Default for Tree {
    fn default() -> Self {
        Self::EMPTY
    }
}
pub use self::Tree::{EMPTY, LEAF, NODE};

pub type ValueNode = ArcStr;

pub fn add(mut inTree: metamodelica::Ref<Tree>, mut inKey: &Key) -> Result<metamodelica::Ref<Tree>> {
    let mut tree: metamodelica::Ref<Tree> = inTree;
    tree = (match &*tree {
        Tree::EMPTY { .. } => metamodelica::Ref::new(Tree::LEAF { key: inKey.clone() }),
        Tree::NODE { key, .. } => {
            let mut key_comp: i32;
            key_comp = keyCompare(inKey.clone(), key.clone());
            if key_comp == -1 {
                assign_variant_field!(tree => Tree::NODE; left = add(var_field!((*tree).left, Tree::NODE).clone(), inKey)?);
            } else if key_comp == 1 {
                assign_variant_field!(tree => Tree::NODE; right = add(var_field!((*tree).right, Tree::NODE).clone(), inKey)?);
            }
            if (key_comp == 0) { tree } else { balance(tree)? }
        }
        Tree::LEAF { key } => {
            let mut key_comp: i32;
            let mut outTree: metamodelica::Ref<Tree>;
            key_comp = keyCompare(inKey.clone(), key.clone());
            if key_comp == -1 {
                outTree = metamodelica::Ref::new(Tree::NODE {
                    key: var_field!((*tree).key, Tree::LEAF).clone(),
                    height: 2,
                    left: metamodelica::Ref::new(Tree::LEAF { key: inKey.clone() }),
                    right: crate::AvlSetString::Tree::interned_EMPTY(),
                });
            } else if key_comp == 1 {
                outTree = metamodelica::Ref::new(Tree::NODE {
                    key: var_field!((*tree).key, Tree::LEAF).clone(),
                    height: 2,
                    left: crate::AvlSetString::Tree::interned_EMPTY(),
                    right: metamodelica::Ref::new(Tree::LEAF { key: inKey.clone() }),
                });
            } else {
                outTree = tree;
            }
            outTree
        }
    });
    Ok(tree)
}

pub fn addList(
    mut tree: metamodelica::Ref<Tree>,
    mut inValues: &metamodelica::List<ArcStr>,
) -> Result<metamodelica::Ref<Tree>> {
    let mut tree: metamodelica::Ref<Tree> = tree;
    for mut key in &**inValues {
        tree = add(tree, metamodelica::AsArg::as_arg(&key))?;
    }
    Ok(tree)
}

fn balance(mut inTree: metamodelica::Ref<Tree>) -> Result<metamodelica::Ref<Tree>> {
    let mut outTree: metamodelica::Ref<Tree> = inTree.clone();
    outTree = (match &*outTree {
        Tree::LEAF { .. } => inTree,
        Tree::NODE {
            left: __outTree_left,
            right: __outTree_right,
            ..
        } => {
            let mut lh: i32;
            let mut rh: i32;
            let mut diff: i32;
            let mut balanced_tree: metamodelica::Ref<Tree>;
            lh = height(metamodelica::AsArg::as_arg(&__outTree_left));
            rh = height(metamodelica::AsArg::as_arg(&__outTree_right));
            diff = lh - rh;
            if diff < -1 {
                balanced_tree = if (calculateBalance(var_field!((*outTree).right, Tree::NODE)) > 0) {
                    rotateLeft(setTreeLeftRight(
                        outTree.clone(),
                        var_field!((*outTree).left, Tree::NODE).clone(),
                        rotateRight(var_field!((*outTree).right, Tree::NODE).clone())?,
                    )?)?
                } else {
                    rotateLeft(outTree)?
                };
            } else if diff > 1 {
                balanced_tree = if (calculateBalance(var_field!((*outTree).left, Tree::NODE)) < 0) {
                    rotateRight(setTreeLeftRight(
                        outTree.clone(),
                        rotateLeft(var_field!((*outTree).left, Tree::NODE).clone())?,
                        var_field!((*outTree).right, Tree::NODE).clone(),
                    )?)?
                } else {
                    rotateRight(outTree)?
                };
            } else if var_field!((*outTree).height, Tree::NODE).clone() != std::cmp::max(lh, rh) + 1 {
                assign_variant_field!(outTree => Tree::NODE; height = std::cmp::max(lh, rh) + 1);
                balanced_tree = outTree;
            } else {
                balanced_tree = outTree;
            }
            balanced_tree
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outTree)
}

fn calculateBalance(mut inNode: &metamodelica::Ref<Tree>) -> i32 {
    let mut outBalance: i32;
    outBalance = (match &**inNode {
        Tree::NODE {
            left: __inNode_left,
            right: __inNode_right,
            ..
        } => height(metamodelica::AsArg::as_arg(&__inNode_left)) - height(metamodelica::AsArg::as_arg(&__inNode_right)),
        Tree::LEAF { .. } => 0,
        _ => 0,
    });
    outBalance
}

pub fn hasKey(mut inTree: metamodelica::Ref<Tree>, mut inKey: Key) -> Result<bool> {
    let mut comp: bool = false;
    let mut key: Key;
    let mut key_comp: i32;
    let mut tree: metamodelica::Ref<Tree>;
    key = (match &*inTree {
        Tree::NODE { key: __inTree_key, .. } => __inTree_key.clone(),
        Tree::LEAF { key: __inTree_key } => __inTree_key.clone(),
        Tree::EMPTY { .. } => {
            return Ok(comp);
            return Err("fail");
        }
    });
    key_comp = keyCompare(inKey.clone(), key);
    comp = (::match_deref::match_deref! { match &((key_comp, inTree)) {
        (0, _) => true,
        (1, Deref @ Tree::NODE { right: __esc_tree, .. }) => {
            tree = (*__esc_tree).clone();
            hasKey(tree.clone(), inKey)?
        },
        ((-1), Deref @ Tree::NODE { left: __esc_tree, .. }) => {
            tree = (*__esc_tree).clone();
            hasKey(tree.clone(), inKey)?
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(comp)
}

fn height(mut inNode: &metamodelica::Ref<Tree>) -> i32 {
    let mut outHeight: i32;
    outHeight = (match &**inNode {
        Tree::NODE {
            height: __inNode_height,
            ..
        } => __inNode_height.clone(),
        Tree::LEAF { .. } => 1,
        _ => 0,
    });
    outHeight
}

pub(crate) fn intersection(
    mut tree1: metamodelica::Ref<Tree>,
    mut tree2: metamodelica::Ref<Tree>,
) -> Result<(
    metamodelica::Ref<Tree>,
    metamodelica::Ref<Tree>,
    metamodelica::Ref<Tree>,
)> {
    let mut intersect: metamodelica::Ref<Tree> = crate::AvlSetString::Tree::interned_EMPTY();
    let mut rest1: metamodelica::Ref<Tree> = crate::AvlSetString::Tree::interned_EMPTY();
    let mut rest2: metamodelica::Ref<Tree> = crate::AvlSetString::Tree::interned_EMPTY();
    let mut keylist1: metamodelica::List<ArcStr>;
    let mut keylist2: metamodelica::List<ArcStr>;
    let mut k1: Key;
    let mut k2: Key;
    let mut key_comp: i32;
    if isEmpty(&tree1) {
        rest2 = tree2;
        return Ok((intersect, rest1, rest2));
    }
    if isEmpty(&tree2) {
        rest1 = tree1;
        return Ok((intersect, rest1, rest2));
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(listKeys(&tree1, metamodelica::nil())) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    k1 = metamodelica::Own::own(__pa0);
    keylist1 = metamodelica::Own::own(__pa1);
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(listKeys(&tree2, metamodelica::nil())) {
        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    k2 = metamodelica::Own::own(__pa2);
    keylist2 = metamodelica::Own::own(__pa3);
    loop {
        key_comp = keyCompare(k1.clone(), k2.clone());
        if key_comp > 0 {
            if true
            /* isPresent not implemented in Rust */
            {
                rest2 = add(rest2, &k2)?;
            }
            if (keylist2).is_empty() {
                break;
            }
            let (__pa4, __pa5) = ::match_deref::match_deref! { match &(keylist2) {
                Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                _ => return Err("pattern mismatch"),
            } };
            k2 = metamodelica::Own::own(__pa4);
            keylist2 = metamodelica::Own::own(__pa5);
        } else if key_comp < 0 {
            if true
            /* isPresent not implemented in Rust */
            {
                rest1 = add(rest1, &k1)?;
            }
            if (keylist1).is_empty() {
                break;
            }
            let (__pa6, __pa7) = ::match_deref::match_deref! { match &(keylist1) {
                Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: __pa7 } => (__pa6.clone(), __pa7.clone()),
                _ => return Err("pattern mismatch"),
            } };
            k1 = metamodelica::Own::own(__pa6);
            keylist1 = metamodelica::Own::own(__pa7);
        } else {
            intersect = add(intersect, &k1)?;
            if (keylist1).is_empty() || (keylist2).is_empty() {
                break;
            }
            let (__pa8, __pa9) = ::match_deref::match_deref! { match &(keylist1) {
                Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: __pa9 } => (__pa8.clone(), __pa9.clone()),
                _ => return Err("pattern mismatch"),
            } };
            k1 = metamodelica::Own::own(__pa8);
            keylist1 = metamodelica::Own::own(__pa9);
            let (__pa10, __pa11) = ::match_deref::match_deref! { match &(keylist2) {
                Deref @ metamodelica::ListNode::Cons { head: __pa10, tail: __pa11 } => (__pa10.clone(), __pa11.clone()),
                _ => return Err("pattern mismatch"),
            } };
            k2 = metamodelica::Own::own(__pa10);
            keylist2 = metamodelica::Own::own(__pa11);
        }
    }
    if true /* isPresent not implemented in Rust */ && !((keylist1).is_empty()) {
        for mut key in &*keylist1 {
            rest1 = add(rest1, metamodelica::AsArg::as_arg(&key))?;
        }
    }
    if true /* isPresent not implemented in Rust */ && !((keylist2).is_empty()) {
        for mut key in &*keylist2 {
            rest2 = add(rest2, metamodelica::AsArg::as_arg(&key))?;
        }
    }
    Ok((intersect, rest1, rest2))
}

pub fn isEmpty(mut tree: &metamodelica::Ref<Tree>) -> bool {
    let mut isEmpty: bool;
    isEmpty = (match &**tree {
        Tree::EMPTY { .. } => true,
        _ => false,
    });
    isEmpty
}

pub fn join<'__b>(
    mut tree: metamodelica::Ref<Tree>,
    mut treeToJoin: &'__b metamodelica::Ref<Tree>,
) -> Result<metamodelica::Ref<Tree>> {
    '__tco: loop {
        match &**treeToJoin {
            Tree::EMPTY { .. } => return Ok(tree),
            Tree::NODE { .. } => {
                tree = add(tree, var_field!((**treeToJoin).key, Tree::NODE))?;
                tree = join(tree, var_field!((**treeToJoin).left, Tree::NODE))?;
                {
                    (tree, treeToJoin) = (tree, var_field!((**treeToJoin).right, Tree::NODE));
                    continue '__tco;
                }
            }
            Tree::LEAF { .. } => return Ok(add(tree, var_field!((**treeToJoin).key, Tree::LEAF))?),
        }
    }
}

pub fn listKeys<'__b>(
    mut inTree: &'__b metamodelica::Ref<Tree>,
    mut lst: metamodelica::List<ArcStr>,
) -> metamodelica::List<ArcStr> {
    '__tco: loop {
        match &**inTree {
            Tree::LEAF { .. } => return metamodelica::cons(var_field!((**inTree).key, Tree::LEAF).clone(), lst),
            Tree::NODE { .. } => {
                lst = listKeys(var_field!((**inTree).right, Tree::NODE), lst);
                lst = metamodelica::cons(var_field!((**inTree).key, Tree::NODE).clone(), lst);
                {
                    (inTree, lst) = (var_field!((**inTree).left, Tree::NODE), lst);
                    continue '__tco;
                }
            }
            _ => return lst,
        }
    }
}

pub(crate) fn listKeysReverse<'__b>(
    mut inTree: &'__b metamodelica::Ref<Tree>,
    mut lst: metamodelica::List<ArcStr>,
) -> metamodelica::List<ArcStr> {
    '__tco: loop {
        match &**inTree {
            Tree::LEAF { .. } => return metamodelica::cons(var_field!((**inTree).key, Tree::LEAF).clone(), lst),
            Tree::NODE { .. } => {
                lst = listKeysReverse(var_field!((**inTree).left, Tree::NODE), lst);
                lst = metamodelica::cons(var_field!((**inTree).key, Tree::NODE).clone(), lst);
                {
                    (inTree, lst) = (var_field!((**inTree).right, Tree::NODE), lst);
                    continue '__tco;
                }
            }
            _ => return lst,
        }
    }
}

pub fn new() -> metamodelica::Ref<Tree> {
    let mut outTree: metamodelica::Ref<Tree> = crate::AvlSetString::Tree::interned_EMPTY();
    outTree
}

pub(crate) fn printNodeStr(mut inNode: &metamodelica::Ref<Tree>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &**inNode {
        Tree::NODE { key: __inNode_key, .. } => keyStr(__inNode_key.clone()),
        Tree::LEAF { key: __inNode_key } => keyStr(__inNode_key.clone()),
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

pub(crate) fn printTreeStr(mut inTree: &metamodelica::Ref<Tree>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut left: metamodelica::Ref<Tree>;
    let mut right: metamodelica::Ref<Tree>;
    outString = (match &**inTree {
        Tree::EMPTY { .. } => literal!("EMPTY()"),
        Tree::LEAF { .. } => printNodeStr(inTree)?,
        Tree::NODE {
            left: __esc_left,
            right: __esc_right,
            ..
        } => {
            left = (*__esc_left).clone();
            right = (*__esc_right).clone();
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*printTreeStr2(
                    metamodelica::AsArg::as_arg(&left),
                    true,
                    &(literal!("")),
                )?);
                __mm_s.push_str(&*printNodeStr(inTree)?);
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*printTreeStr2(
                    metamodelica::AsArg::as_arg(&right),
                    false,
                    &(literal!("")),
                )?);
                ArcStr::from(__mm_s)
            }
        }
    });
    Ok(outString)
}

fn printTreeStr2(mut inTree: &metamodelica::Ref<Tree>, mut isLeft: bool, mut inIndent: &ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut left: Option<metamodelica::Ref<Tree>>;
    let mut right: Option<metamodelica::Ref<Tree>>;
    outString = (match &**inTree {
        Tree::NODE {
            left: __inTree_left,
            right: __inTree_right,
            ..
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*printTreeStr2(
                metamodelica::AsArg::as_arg(&__inTree_left),
                true,
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*inIndent);
                    __mm_s.push_str(&*if (isLeft) {
                        literal!("     ")
                    } else {
                        literal!(" │   ")
                    });
                    ArcStr::from(__mm_s)
                }),
            )?);
            __mm_s.push_str(&*inIndent);
            __mm_s.push_str(&*if (isLeft) { literal!(" ┌") } else { literal!(" └") });
            __mm_s.push_str(&*literal!("────"));
            __mm_s.push_str(&*printNodeStr(inTree)?);
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*printTreeStr2(
                metamodelica::AsArg::as_arg(&__inTree_right),
                false,
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*inIndent);
                    __mm_s.push_str(&*if (isLeft) {
                        literal!(" │   ")
                    } else {
                        literal!("     ")
                    });
                    ArcStr::from(__mm_s)
                }),
            )?);
            ArcStr::from(__mm_s)
        }
        Tree::LEAF { .. } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*inIndent);
            __mm_s.push_str(&*if (isLeft) { literal!(" ┌") } else { literal!(" └") });
            __mm_s.push_str(&*literal!("────"));
            __mm_s.push_str(&*printNodeStr(inTree)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        }
        _ => literal!(""),
    });
    Ok(outString)
}

fn referenceEqOrEmpty(mut t1: &metamodelica::Ref<Tree>, mut t2: &metamodelica::Ref<Tree>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match (t1, t2) {
        (Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => true,
        _ => referenceEq(&*(&**t1),&*(&**t2)),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn rotateLeft(mut inNode: metamodelica::Ref<Tree>) -> Result<metamodelica::Ref<Tree>> {
    let mut outNode: metamodelica::Ref<Tree> = inNode.clone();
    outNode = (::match_deref::match_deref! { match &(outNode.clone()) {
        Deref @ Tree::NODE { right: child @ Deref @ Tree::NODE { .. }, left: __outNode_left, .. } => {
            let mut node: metamodelica::Ref<Tree>;
            node = setTreeLeftRight(outNode, __outNode_left.clone(), var_field!((**child).left, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), node, var_field!((**child).right, Tree::NODE).clone())?
        },
        Deref @ Tree::NODE { right: child @ Deref @ Tree::LEAF { .. }, left: __outNode_left, .. } => {
            let mut node: metamodelica::Ref<Tree>;
            node = setTreeLeftRight(outNode, __outNode_left.clone(), crate::AvlSetString::Tree::interned_EMPTY())?;
            setTreeLeftRight(child.clone(), node, crate::AvlSetString::Tree::interned_EMPTY())?
        },
        _ => {
            inNode
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outNode)
}

fn rotateRight(mut inNode: metamodelica::Ref<Tree>) -> Result<metamodelica::Ref<Tree>> {
    let mut outNode: metamodelica::Ref<Tree> = inNode.clone();
    outNode = (::match_deref::match_deref! { match &(outNode.clone()) {
        Deref @ Tree::NODE { left: child @ Deref @ Tree::NODE { .. }, right: __outNode_right, .. } => {
            let mut node: metamodelica::Ref<Tree>;
            node = setTreeLeftRight(outNode, var_field!((**child).right, Tree::NODE).clone(), __outNode_right.clone())?;
            setTreeLeftRight(child.clone(), var_field!((**child).left, Tree::NODE).clone(), node)?
        },
        Deref @ Tree::NODE { left: child @ Deref @ Tree::LEAF { .. }, right: __outNode_right, .. } => {
            let mut node: metamodelica::Ref<Tree>;
            node = setTreeLeftRight(outNode, crate::AvlSetString::Tree::interned_EMPTY(), __outNode_right.clone())?;
            setTreeLeftRight(child.clone(), crate::AvlSetString::Tree::interned_EMPTY(), node)?
        },
        _ => {
            inNode
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outNode)
}

pub(crate) fn setTreeLeftRight(
    mut orig: metamodelica::Ref<Tree>,
    mut left: metamodelica::Ref<Tree>,
    mut right: metamodelica::Ref<Tree>,
) -> Result<metamodelica::Ref<Tree>> {
    let mut res: metamodelica::Ref<Tree>;
    res = (::match_deref::match_deref! { match &((orig.clone(), left.clone(), right.clone())) {
        (Deref @ Tree::NODE { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => metamodelica::Ref::new(Tree::LEAF { key: var_field!((*orig).key, Tree::NODE).clone() }),
        (Deref @ Tree::LEAF { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => orig,
        (Deref @ Tree::NODE { .. }, _, _) => if (referenceEqOrEmpty(var_field!((*orig).left, Tree::NODE), &left) && referenceEqOrEmpty(var_field!((*orig).right, Tree::NODE), &right)) {orig} else {metamodelica::Ref::new(Tree::NODE { key: var_field!((*orig).key, Tree::NODE).clone(), height: std::cmp::max(height(&left), height(&right)) + 1, left: left, right: right })},
        (Deref @ Tree::LEAF { .. }, _, _) => metamodelica::Ref::new(Tree::NODE { key: var_field!((*orig).key, Tree::LEAF).clone(), height: std::cmp::max(height(&left), height(&right)) + 1, left: left, right: right }),
        _ => return Err("match: no arm matched"),
    } });
    Ok(res)
}

pub(crate) fn smallestKey<'__b>(mut tree: &'__b metamodelica::Ref<Tree>) -> Result<Key> {
    '__tco: loop {
        ::match_deref::match_deref! { match tree {
            Deref @ Tree::NODE { right: Deref @ Tree::EMPTY { .. }, .. } => return Ok(var_field!((**tree).key, Tree::NODE).clone()),
            Deref @ Tree::NODE { .. } => { tree = var_field!((**tree).right, Tree::NODE); continue '__tco; },
            Deref @ Tree::LEAF { .. } => return Ok(var_field!((**tree).key, Tree::LEAF).clone()),
            _ => return Err("match: no arm matched"),
        } }
    }
}
