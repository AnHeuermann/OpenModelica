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

use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

/* *************************
 imports
**************************/
/* *************************
 types
**************************/
/// Generic Binary tree implementation
///  - Binary Tree
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct BinTree {
    /// Value
    pub value: Option<TreeValue>,
    /// left subtree
    pub leftSubTree: Option<metamodelica::Ref<BinTree>>,
    /// right subtree
    pub rightSubTree: Option<metamodelica::Ref<BinTree>>,
}

impl metamodelica::gc::MMTrace for BinTree {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.value, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.leftSubTree, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.rightSubTree, __mmv)?;
        Ok(())
    }
}
impl Default for BinTree {
    fn default() -> Self {
        Self {
            value: Default::default(),
            leftSubTree: Default::default(),
            rightSubTree: Default::default(),
        }
    }
}

pub type TREENODE = BinTree;

/// Each node in the binary tree can have a value associated with it.
///  - Tree Value
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct TreeValue {
    /// Key
    pub key: Key,
    pub r#str: ArcStr,
    pub hash: i32,
    /// Value
    pub value: Value,
}

impl metamodelica::gc::MMTrace for TreeValue {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.key, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.r#str, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.hash, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.value, __mmv)?;
        Ok(())
    }
}
impl Default for TreeValue {
    fn default() -> Self {
        Self {
            key: Default::default(),
            r#str: Default::default(),
            hash: Default::default(),
            value: Default::default(),
        }
    }
}

pub type TREEVALUE = TreeValue;

/// A key is a Component Reference
pub type Key = metamodelica::Ref<DAE::ComponentRef>;

/// - Value
pub type Value = i32;

thread_local! { static __emptyBinTree_TLS: metamodelica::Ref<BinTree> = metamodelica::Ref::new(BinTree { value: None, leftSubTree: None, rightSubTree: None }); }
pub(crate) fn emptyBinTree() -> metamodelica::Ref<BinTree> {
    __emptyBinTree_TLS.with(|__t| __t.clone())
}

/* *************************
 implementation
**************************/
fn keyCompareNinjaSecretHashTricks(mut lstr: &ArcStr, mut lhash: i32, mut rstr: &ArcStr, mut rhash: i32) -> i32 {
    let mut cmp: i32;
    cmp = Util::intSign(lhash - rhash);
    cmp = if (cmp == 0) { stringCompare(&lstr, &rstr) } else { cmp };
    cmp
}

pub(crate) fn treeGet(mut bt: metamodelica::Ref<BinTree>, mut key: &Key) -> Result<Value> {
    let mut v: Value;
    let mut keystr: ArcStr;
    let mut keyhash: i32;
    keystr = ComponentReferenceBasics::printComponentRefStr(key)?;
    keyhash = stringHashDjb2Mod(&keystr, BaseHashTable::hugeBucketSize.clone());
    v = treeGet3(bt.clone(), &(keystr.clone()), keyhash, treeGet2(&bt, &keystr, keyhash)?)?;
    Ok(v)
}

fn treeGet2(mut inBinTree: &metamodelica::Ref<BinTree>, mut keystr: &ArcStr, mut keyhash: i32) -> Result<i32> {
    let mut compResult: i32;
    compResult = (match &**inBinTree {
        BinTree {
            value:
                Some(TreeValue {
                    r#str: rkeystr,
                    hash: rkeyhash,
                    ..
                }),
            ..
        } => keyCompareNinjaSecretHashTricks(metamodelica::AsArg::as_arg(&rkeystr), rkeyhash.clone(), keystr, keyhash),
        _ => return Err("match: no arm matched"),
    });
    Ok(compResult)
}

fn treeGet3<'__b>(
    mut inBinTree: metamodelica::Ref<BinTree>,
    mut keystr: &'__b ArcStr,
    mut keyhash: i32,
    mut inCompResult: i32,
) -> Result<Value> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inBinTree, inCompResult)) {
            (Deref @ BinTree { value: Some(TreeValue { value: rval, .. }), .. }, 0) => {
                return Ok(rval.clone())
            },
            (Deref @ BinTree { rightSubTree: Some(right), .. }, 1) => {
                let mut compResult: i32;
                compResult = treeGet2(metamodelica::AsArg::as_arg(&right), keystr, keyhash)?;
                { (inBinTree, keystr, keyhash, inCompResult) = (right.clone(), keystr, keyhash, compResult); continue '__tco; }
            },
            (Deref @ BinTree { leftSubTree: Some(left), .. }, (-1)) => {
                let mut compResult: i32;
                compResult = treeGet2(metamodelica::AsArg::as_arg(&left), keystr, keyhash)?;
                { (inBinTree, keystr, keyhash, inCompResult) = (left.clone(), keystr, keyhash, compResult); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn treeAddList(
    mut inBinTree: metamodelica::Ref<BinTree>,
    mut inKeyLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::Ref<BinTree>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inBinTree, inKeyLst)) {
            (bt, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(bt.clone())
            },
            (bt, Deref @ metamodelica::ListNode::Cons { head: key, tail: res }) => {
                let mut bt_1: metamodelica::Ref<BinTree>;
                let mut bt_2: metamodelica::Ref<BinTree>;
                bt_1 = treeAdd(metamodelica::AsArg::as_arg(&bt), key.clone(), 0)?;
                { (inBinTree, inKeyLst) = (bt_1, res.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn treeAdd(
    mut inBinTree: &metamodelica::Ref<BinTree>,
    mut inKey: Key,
    mut inValue: Value,
) -> Result<metamodelica::Ref<BinTree>> {
    let mut outBinTree: metamodelica::Ref<BinTree>;
    let mut r#str: ArcStr;
    r#str = ComponentReferenceBasics::printComponentRefStr(&inKey)?;
    outBinTree = treeAdd2(
        inBinTree,
        inKey,
        stringHashDjb2Mod(&r#str, BaseHashTable::hugeBucketSize.clone()),
        &(r#str),
        inValue,
    )?;
    Ok(outBinTree)
}

fn treeAdd2(
    mut inBinTree: &metamodelica::Ref<BinTree>,
    mut inKey: Key,
    mut keyhash: i32,
    mut keystr: &ArcStr,
    mut inValue: Value,
) -> Result<metamodelica::Ref<BinTree>> {
    let mut outBinTree: metamodelica::Ref<BinTree>;
    outBinTree = 'mc: {
        let __mc_input = (&**inBinTree, inKey, inValue);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BinTree { value: None, leftSubTree: None, rightSubTree: None }, key, value) => {
                    Ok(metamodelica::Ref::new(BinTree { value: Some(TreeValue { key: key.clone(), r#str: keystr.clone(), hash: keyhash, value: value.clone() }), leftSubTree: None, rightSubTree: None }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BinTree { value: Some(TreeValue { key: rkey, r#str: rkeystr, hash: rhash, value: _ }), leftSubTree: left, rightSubTree: right }, _, value) => {
                    let 0 = (keyCompareNinjaSecretHashTricks(metamodelica::AsArg::as_arg(&rkeystr), rhash.clone(), keystr, keyhash)) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(BinTree { value: Some(TreeValue { key: rkey.clone(), r#str: rkeystr.clone(), hash: rhash.clone(), value: value.clone() }), leftSubTree: left.clone(), rightSubTree: right.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BinTree { value: optVal @ Some(TreeValue { key: _, r#str: rkeystr, hash: rhash, value: _ }), leftSubTree: left, rightSubTree: Some(t) }, key, value) => {
                    let mut t_1: metamodelica::Ref<BinTree>;
                    let 1 = (keyCompareNinjaSecretHashTricks(metamodelica::AsArg::as_arg(&rkeystr), rhash.clone(), keystr, keyhash)) else { return Err("pattern mismatch") };
                    t_1 = treeAdd2(metamodelica::AsArg::as_arg(&t), key.clone(), keyhash, keystr, value.clone())?;
                    Ok(metamodelica::Ref::new(BinTree { value: optVal.clone(), leftSubTree: left.clone(), rightSubTree: Some(t_1.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BinTree { value: optVal @ Some(TreeValue { key: _, r#str: rkeystr, hash: rhash, value: _ }), leftSubTree: left, rightSubTree: None }, key, value) => {
                    let mut right_1: metamodelica::Ref<BinTree>;
                    let 1 = (keyCompareNinjaSecretHashTricks(metamodelica::AsArg::as_arg(&rkeystr), rhash.clone(), keystr, keyhash)) else { return Err("pattern mismatch") };
                    right_1 = treeAdd2(&(metamodelica::Ref::new(BinTree { value: None, leftSubTree: None, rightSubTree: None })), key.clone(), keyhash, keystr, value.clone())?;
                    Ok(metamodelica::Ref::new(BinTree { value: optVal.clone(), leftSubTree: left.clone(), rightSubTree: Some(right_1.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BinTree { value: optVal @ Some(TreeValue { key: _, r#str: rkeystr, hash: rhash, value: _ }), leftSubTree: Some(t), rightSubTree: right }, key, value) => {
                    let mut t_1: metamodelica::Ref<BinTree>;
                    let (-1) = (keyCompareNinjaSecretHashTricks(metamodelica::AsArg::as_arg(&rkeystr), rhash.clone(), keystr, keyhash)) else { return Err("pattern mismatch") };
                    t_1 = treeAdd2(metamodelica::AsArg::as_arg(&t), key.clone(), keyhash, keystr, value.clone())?;
                    Ok(metamodelica::Ref::new(BinTree { value: optVal.clone(), leftSubTree: Some(t_1.clone()), rightSubTree: right.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BinTree { value: optVal @ Some(TreeValue { key: _, r#str: rkeystr, hash: rhash, value: _ }), leftSubTree: None, rightSubTree: right }, key, value) => {
                    let mut left_1: metamodelica::Ref<BinTree>;
                    let (-1) = (keyCompareNinjaSecretHashTricks(metamodelica::AsArg::as_arg(&rkeystr), rhash.clone(), keystr, keyhash)) else { return Err("pattern mismatch") };
                    left_1 = treeAdd2(&(metamodelica::Ref::new(BinTree { value: None, leftSubTree: None, rightSubTree: None })), key.clone(), keyhash, keystr, value.clone())?;
                    Ok(metamodelica::Ref::new(BinTree { value: optVal.clone(), leftSubTree: Some(left_1.clone()), rightSubTree: right.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("- BinaryTree.treeAdd2 failed\n")])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outBinTree)
}

// protected function treeDelete2 "author: PA
//   This function deletes an entry from the BinTree."
//   input BinTree inBinTree;
//   input String keystr;
//   input Integer keyhash;
//   output BinTree outBinTree;
// algorithm
//   outBinTree := matchcontinue (inBinTree,keystr,keyhash)
//     local
//       BinTree bt,right,left,t;
//       DAE.ComponentRef key,rkey;
//       String rkeystr;
//       TreeValue rightmost;
//       Option<BinTree> optRight,optLeft,optTree;
//       Value rval;
//       Option<TreeValue> optVal;
//       Integer rhash;
//
//     case ((bt as TREENODE(value = NONE(),leftSubTree = NONE(),rightSubTree = NONE())),_,_)
//       then bt;
//
//     case (TREENODE(value = SOME(TREEVALUE(rkey,rkeystr,rhash,rval)),leftSubTree = optLeft,rightSubTree = SOME(right)),_,_)
//       equation
//         0 = keyCompareNinjaSecretHashTricks(rkeystr, rhash, keystr, keyhash);
//         (rightmost,right) = treeDeleteRightmostValue(right);
//         optRight = treePruneEmptyNodes(right);
//       then
//         TREENODE(SOME(rightmost),optLeft,optRight);
//
//     case (TREENODE(value = SOME(TREEVALUE(rkey,rkeystr,rhash,rval)),leftSubTree = SOME(left as TREENODE(value=_)),rightSubTree = NONE()),_,_)
//       equation
//         0 = keyCompareNinjaSecretHashTricks(rkeystr, rhash, keystr, keyhash);
//       then
//         left;
//
//     case (TREENODE(value = SOME(TREEVALUE(rkey,rkeystr,rhash,rval)),leftSubTree = NONE(),rightSubTree = NONE()),_,_)
//       equation
//         0 = keyCompareNinjaSecretHashTricks(rkeystr, rhash, keystr, keyhash);
//       then
//         TREENODE(NONE(),NONE(),NONE());
//
//     case (TREENODE(value = optVal as SOME(TREEVALUE(rkey,rkeystr,rhash,rval)),leftSubTree = optLeft,rightSubTree = SOME(t)),_,_)
//       equation
//         1 = keyCompareNinjaSecretHashTricks(rkeystr, rhash, keystr, keyhash);
//         t = treeDelete2(t, keystr, keyhash);
//         optTree = treePruneEmptyNodes(t);
//       then
//         TREENODE(optVal,optLeft,optTree);
//
//     case (TREENODE(value = optVal as SOME(TREEVALUE(rkey,rkeystr,rhash,rval)),leftSubTree =  SOME(t),rightSubTree = optRight),_,_)
//       equation
//         -1 = keyCompareNinjaSecretHashTricks(rkeystr, rhash, keystr, keyhash);
//         t = treeDelete2(t, keystr, keyhash);
//         optTree = treePruneEmptyNodes(t);
//       then
//         TREENODE(optVal,optTree,optRight);
//
//     else
//       equation
//         Error.addMessage(Error.INTERNAL_ERROR,{"-BinaryTree.treeDelete failed\n"});
//       then
//         fail();
//   end matchcontinue;
// end treeDelete2;
// protected function treeDeleteRightmostValue "author: PA
//   This function takes a BinTree and deletes the rightmost value of the tree.
//   Tt returns this value and the updated BinTree. This function is used in
//   the binary tree deletion function \'tree_delete\'.
//   inputs:  (BinTree)
//   outputs: (TreeValue, /* deleted value */
//               BinTree    /* updated bintree */)
// "
//   input BinTree inBinTree;
//   output TreeValue outTreeValue;
//   output BinTree outBinTree;
// algorithm
//   (outTreeValue,outBinTree) := matchcontinue (inBinTree)
//     local
//       TreeValue treeVal,value;
//       BinTree left,right,bt;
//       Option<BinTree> optRight, optLeft;
//       Option<TreeValue> optTreeVal;
//
//     case (TREENODE(value = SOME(treeVal),leftSubTree = NONE(),rightSubTree = NONE()))
//       then (treeVal,TREENODE(NONE(),NONE(),NONE()));
//
//     case (TREENODE(value = SOME(treeVal),leftSubTree = SOME(left),rightSubTree = NONE()))
//       then (treeVal,left);
//
//     case (TREENODE(value = optTreeVal,leftSubTree = optLeft,rightSubTree = SOME(right)))
//       equation
//         (value,right) = treeDeleteRightmostValue(right);
//         optRight = treePruneEmptyNodes(right);
//       then
//         (value,TREENODE(optTreeVal,optLeft,optRight));
//
//     case (TREENODE(value = SOME(treeVal),leftSubTree = NONE(),rightSubTree = SOME(right)))
//       equation
//         failure((_,_) = treeDeleteRightmostValue(right));
//         print("- BinaryTree.treeDeleteRightmostValue: right value was empty, left NONE\n");
//       then
//         (treeVal,TREENODE(NONE(),NONE(),NONE()));
//
//     else
//       equation
//         Error.addMessage(Error.INTERNAL_ERROR,{"- BinaryTree.treeDeleteRightmostValue failed\n"});
//       then
//         fail();
//   end matchcontinue;
// end treeDeleteRightmostValue;
// protected function treePruneEmptyNodes "author: PA
//   This function is a helper function to tree_delete
//   It is used to delete empty nodes of the BinTree
//   representation, that might be introduced when deleting nodes."
//   input BinTree inBinTree;
//   output Option<BinTree> outBinTreeOption;
// algorithm
//   outBinTreeOption := matchcontinue (inBinTree)
//     local BinTree bt;
//     case TREENODE(value = NONE(),leftSubTree = NONE(),rightSubTree = NONE()) then NONE();
//     case bt then SOME(bt);
//   end matchcontinue;
// end treePruneEmptyNodes;
// protected function bintreeDepth "author: PA
//   This function calculates the depth of the Binary Tree given
//   as input. It can be used for debugging purposes to investigate
//   how balanced binary trees are."
//   input BinTree inBinTree;
//   output Integer outInteger;
// algorithm
//   outInteger := matchcontinue (inBinTree)
//     local
//       Value ld,rd,res;
//       BinTree left,right;
//
//     case (TREENODE(leftSubTree = NONE(),rightSubTree = NONE())) then 1;
//
//     case (TREENODE(leftSubTree = SOME(left),rightSubTree = SOME(right)))
//       equation
//         ld = bintreeDepth(left);
//         rd = bintreeDepth(right);
//         res = intMax(ld, rd);
//       then
//         res + 1;
//
//     case (TREENODE(leftSubTree = SOME(left),rightSubTree = NONE()))
//       equation
//         ld = bintreeDepth(left);
//       then
//         ld;
//
//     case (TREENODE(leftSubTree = NONE(),rightSubTree = SOME(right)))
//       equation
//         rd = bintreeDepth(right);
//       then
//         rd;
//   end matchcontinue;
// end bintreeDepth;
pub(crate) fn bintreeToList(
    mut inBinTree: &metamodelica::Ref<BinTree>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    metamodelica::List<i32>,
)> {
    let mut outKeyLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut outValueLst: metamodelica::List<i32>;
    (outKeyLst, outValueLst) = 'mc: {
        let __mc_input = &**inBinTree;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut klst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut vlst: metamodelica::List<i32>;
                    (klst, vlst) = bintreeToList2(inBinTree, metamodelica::nil(), metamodelica::nil())?;
                    Ok((klst.clone(), vlst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("- BackendDAEUtil.bintreeToList failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outKeyLst, outValueLst))
}

fn bintreeToList2(
    mut inBinTree: &metamodelica::Ref<BinTree>,
    mut inKeyLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inValueLst: metamodelica::List<i32>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    metamodelica::List<i32>,
)> {
    let mut outKeyLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut outValueLst: metamodelica::List<i32>;
    (outKeyLst, outValueLst) = 'mc: {
        let __mc_input = &**inBinTree;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BinTree { value: None, leftSubTree: None, rightSubTree: None } => {
                    Ok((inKeyLst.clone(), inValueLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BinTree { value: Some(TreeValue { key, value, .. }), leftSubTree: left, rightSubTree: right } => {
                    let mut klst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut vlst: metamodelica::List<i32>;
                    (klst, vlst) = bintreeToListOpt(left.clone(), metamodelica::cons(key.clone(), inKeyLst.clone()), metamodelica::cons(value.clone(), inValueLst.clone()))?;
                    (klst, vlst) = bintreeToListOpt(right.clone(), klst.clone(), vlst.clone())?;
                    Ok((klst.clone(), vlst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BinTree { value: None, leftSubTree: left, .. } => {
                    let mut klst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut vlst: metamodelica::List<i32>;
                    (klst, vlst) = bintreeToListOpt(left.clone(), inKeyLst.clone(), inValueLst.clone())?;
                    (klst, vlst) = bintreeToListOpt(left.clone(), klst.clone(), vlst.clone())?;
                    Ok((klst.clone(), vlst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outKeyLst, outValueLst))
}

fn bintreeToListOpt(
    mut inBinTreeOption: Option<metamodelica::Ref<BinTree>>,
    mut inKeyLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inValueLst: metamodelica::List<i32>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    metamodelica::List<i32>,
)> {
    let mut outKeyLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut outValueLst: metamodelica::List<i32>;
    (outKeyLst, outValueLst) = (::match_deref::match_deref! { match &(inBinTreeOption) {
        None => {
            (inKeyLst, inValueLst)
        },
        Some(bt) => {
            let mut klst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut vlst: metamodelica::List<i32>;
            (klst, vlst) = bintreeToList2(metamodelica::AsArg::as_arg(&bt), inKeyLst, inValueLst)?;
            (klst, vlst)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outKeyLst, outValueLst))
}

pub(crate) fn binTreeintersection(
    mut bt1: &metamodelica::Ref<BinTree>,
    mut bt2: metamodelica::Ref<BinTree>,
    mut iBt: metamodelica::Ref<BinTree>,
) -> Result<metamodelica::Ref<BinTree>> {
    let mut oBt: metamodelica::Ref<BinTree>;
    let mut keys: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (keys, _) = bintreeToList(bt1)?;
    oBt = List::fold1(
        &keys,
        &fnptr!(
            binTreeintersection1,
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::Ref<BinTree>,
            metamodelica::Ref<BinTree>
        ),
        bt2,
        iBt,
    )?;
    Ok(oBt)
}

fn binTreeintersection1(
    mut key: metamodelica::Ref<DAE::ComponentRef>,
    mut bt2: metamodelica::Ref<BinTree>,
    mut iBt: metamodelica::Ref<BinTree>,
) -> metamodelica::Ref<BinTree> {
    let mut oBt: metamodelica::Ref<BinTree>;
    oBt = 'mc: {
        let __mc_input = &*iBt;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut bt: metamodelica::Ref<BinTree>;
                    treeGet(bt2.clone(), &key)?;
                    bt = treeAdd(&iBt, key.clone(), 0)?;
                    Ok(bt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iBt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oBt
}
