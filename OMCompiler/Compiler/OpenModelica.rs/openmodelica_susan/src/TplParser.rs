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

use crate::TplAbsyn;
use openmodelica_tpl::Tpl;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;
use openmodelica_util::Debug;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

//protected import Print;
pub(crate) const TabSpaces: i32 = 4;

pub mod CacheTree {
    use super::*;
    pub type Key = ArcStr;

    pub type Value = metamodelica::List<TplAbsyn::ASTDef>;

    pub(crate) fn keyStr(mut inKey: Key) -> ArcStr {
        let mut outString: ArcStr;
        outString = inKey;
        outString
    }

    pub(crate) fn valueStr(mut inValue: Value) -> ArcStr {
        let mut outString: ArcStr;
        outString = literal!("#OPAQUE#");
        outString
    }

    pub(crate) fn keyCompare(mut inKey1: Key, mut inKey2: Key) -> i32 {
        let mut outResult: i32;
        outResult = stringCompare(&inKey1, &inKey2);
        outResult
    }

    pub type ConflictFunc = std::sync::Arc<dyn ::std::ops::Fn(Value, Value, Key) -> Result<Value> + 'static>;

    /// The binary tree data structure.
    #[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub(crate) enum Tree {
        NODE {
            /// The key of the node.
            key: Key,
            value: Value,
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
            value: Value,
        },
        EMPTY,
    }
    impl metamodelica::gc::MMTrace for Tree {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                Tree::NODE {
                    key,
                    value,
                    height,
                    left,
                    right,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(key, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(height, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(left, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(right, __mmv)?;
                    Ok(())
                }
                Tree::LEAF { key, value } => {
                    metamodelica::gc::MMTrace::mm_accept(key, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
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
    pub(crate) use self::Tree::{EMPTY, LEAF, NODE};

    pub type ValueNode = ArcStr;

    pub(crate) fn add(
        mut inTree: metamodelica::Ref<Tree>,
        mut inKey: &Key,
        mut inValue: &Value,
        mut conflictFunc: &dyn ::std::ops::Fn(
            metamodelica::List<TplAbsyn::ASTDef>,
            metamodelica::List<TplAbsyn::ASTDef>,
            ArcStr,
        ) -> Result<metamodelica::List<TplAbsyn::ASTDef>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        let mut tree: metamodelica::Ref<Tree> = inTree;
        tree = (match &*tree.clone() {
            Tree::EMPTY { .. } => metamodelica::Ref::new(Tree::LEAF {
                key: inKey.clone(),
                value: inValue.clone(),
            }),
            Tree::NODE { key, .. } => {
                let mut value: Value;
                let mut key_comp: i32;
                key_comp = keyCompare(inKey.clone(), key.clone());
                if key_comp == -1 {
                    assign_variant_field!(tree => Tree::NODE; left = add(var_field!((*tree).left, Tree::NODE).clone(), inKey, inValue, conflictFunc)?);
                } else if key_comp == 1 {
                    assign_variant_field!(tree => Tree::NODE; right = add(var_field!((*tree).right, Tree::NODE).clone(), inKey, inValue, conflictFunc)?);
                } else {
                    value = conflictFunc(
                        inValue.clone(),
                        var_field!((*tree).value, Tree::NODE).clone(),
                        key.clone(),
                    )?;
                    if !(metamodelica::ReferenceEq::reference_eq(
                        &(var_field!((*tree).value, Tree::NODE).clone()),
                        &(value),
                    )) {
                        assign_variant_field!(tree => Tree::NODE; value = value);
                    }
                }
                if (key_comp == 0) { tree } else { balance(tree)? }
            }
            Tree::LEAF { key: __tree_key, .. } => {
                let mut value: Value;
                let mut key_comp: i32;
                let mut outTree: metamodelica::Ref<Tree>;
                key_comp = keyCompare(inKey.clone(), __tree_key.clone());
                if key_comp == -1 {
                    outTree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: metamodelica::Ref::new(Tree::LEAF {
                            key: inKey.clone(),
                            value: inValue.clone(),
                        }),
                        right: crate::TplParser::CacheTree::Tree::interned_EMPTY(),
                    });
                } else if key_comp == 1 {
                    outTree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: crate::TplParser::CacheTree::Tree::interned_EMPTY(),
                        right: metamodelica::Ref::new(Tree::LEAF {
                            key: inKey.clone(),
                            value: inValue.clone(),
                        }),
                    });
                } else {
                    value = conflictFunc(
                        inValue.clone(),
                        var_field!((*tree).value, Tree::LEAF).clone(),
                        var_field!((*tree).key, Tree::LEAF).clone(),
                    )?;
                    if !(metamodelica::ReferenceEq::reference_eq(
                        &(var_field!((*tree).value, Tree::LEAF).clone()),
                        &(value),
                    )) {
                        assign_variant_field!(tree => Tree::LEAF; value = value);
                    }
                    outTree = tree;
                }
                if (key_comp == 0) { outTree } else { balance(outTree)? }
            }
        });
        Ok(tree)
    }

    pub use addConflictFail as addConflictDefault;

    pub fn addConflictFail(mut newValue: Value, mut oldValue: Value, mut key: Key) -> Result<Value> {
        let mut value: Value;
        return Err("fail");
        Ok(value)
    }

    pub(crate) fn addConflictKeep(mut newValue: &Value, mut oldValue: Value, mut key: &Key) -> Value {
        let mut value: Value = oldValue;
        value
    }

    pub(crate) fn addConflictReplace(mut newValue: Value, mut oldValue: &Value, mut key: &Key) -> Value {
        let mut value: Value = newValue;
        value
    }

    pub(crate) fn addList(
        mut tree: metamodelica::Ref<Tree>,
        mut inValues: &metamodelica::List<(ArcStr, metamodelica::List<TplAbsyn::ASTDef>)>,
        mut conflictFunc: &dyn ::std::ops::Fn(
            metamodelica::List<TplAbsyn::ASTDef>,
            metamodelica::List<TplAbsyn::ASTDef>,
            ArcStr,
        ) -> Result<metamodelica::List<TplAbsyn::ASTDef>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        let mut tree: metamodelica::Ref<Tree> = tree;
        let mut key: Key;
        let mut value: Value;
        for mut t in &**inValues {
            (key, value) = t.clone();
            tree = add(tree, &key, &value, conflictFunc)?;
        }
        Ok(tree)
    }

    pub(crate) fn addUpdate(
        mut tree: metamodelica::Ref<Tree>,
        mut key: &Key,
        mut r#fn: &dyn ::std::ops::Fn(
            Option<metamodelica::List<TplAbsyn::ASTDef>>,
        ) -> Result<metamodelica::List<TplAbsyn::ASTDef>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        pub type UpdateFn =
            std::sync::Arc<dyn ::std::ops::Fn(Option<metamodelica::List<TplAbsyn::ASTDef>>) -> Result<Value> + 'static>;

        let mut tree: metamodelica::Ref<Tree> = tree;
        let mut key_comp: i32;
        let mut new_tree: metamodelica::Ref<Tree>;
        tree = (match &*tree {
            Tree::EMPTY { .. } => metamodelica::Ref::new(Tree::LEAF {
                key: key.clone(),
                value: r#fn(None)?,
            }),
            Tree::NODE { key: __tree_key, .. } => {
                key_comp = keyCompare(key.clone(), __tree_key.clone());
                if key_comp == -1 {
                    assign_variant_field!(tree => Tree::NODE; left = addUpdate(var_field!((*tree).left, Tree::NODE).clone(), key, r#fn)?);
                } else if key_comp == 1 {
                    assign_variant_field!(tree => Tree::NODE; right = addUpdate(var_field!((*tree).right, Tree::NODE).clone(), key, r#fn)?);
                } else {
                    assign_variant_field!(tree => Tree::NODE; value = r#fn(Some(var_field!((*tree).value, Tree::NODE).clone()))?);
                }
                if (key_comp == 0) { tree } else { balance(tree)? }
            }
            Tree::LEAF { key: __tree_key, .. } => {
                key_comp = keyCompare(key.clone(), __tree_key.clone());
                if key_comp == -1 {
                    new_tree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: metamodelica::Ref::new(Tree::LEAF {
                            key: key.clone(),
                            value: r#fn(None)?,
                        }),
                        right: crate::TplParser::CacheTree::Tree::interned_EMPTY(),
                    });
                } else if key_comp == 1 {
                    new_tree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: crate::TplParser::CacheTree::Tree::interned_EMPTY(),
                        right: metamodelica::Ref::new(Tree::LEAF {
                            key: key.clone(),
                            value: r#fn(None)?,
                        }),
                    });
                } else {
                    assign_variant_field!(tree => Tree::LEAF; value = r#fn(Some(var_field!((*tree).value, Tree::LEAF).clone()))?);
                    new_tree = tree;
                }
                if (key_comp == 0) { new_tree } else { balance(new_tree)? }
            }
        });
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
            } => {
                height(metamodelica::AsArg::as_arg(&__inNode_left))
                    - height(metamodelica::AsArg::as_arg(&__inNode_right))
            }
            Tree::LEAF { .. } => 0,
            _ => 0,
        });
        outBalance
    }

    pub(crate) fn fold<'__b, FT: Clone + 'static + metamodelica::gc::MMTrace>(
        mut inTree: &'__b metamodelica::Ref<Tree>,
        mut inFunc: &'__b dyn ::std::ops::Fn(ArcStr, metamodelica::List<TplAbsyn::ASTDef>, FT) -> Result<FT>,
        mut inStartValue: FT,
    ) -> Result<FT> {
        pub type FoldFunc<FT: Clone + 'static> =
            std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<FT> + 'static>;

        let mut outResult: FT = inStartValue;
        outResult = (match &**inTree {
            Tree::NODE { key, value, .. } => {
                outResult = fold(var_field!((**inTree).left, Tree::NODE), inFunc, outResult)?;
                outResult = inFunc(key.clone(), value.clone(), outResult)?;
                outResult = fold(var_field!((**inTree).right, Tree::NODE), inFunc, outResult)?;
                outResult
            }
            Tree::LEAF { key, value } => {
                outResult = inFunc(key.clone(), value.clone(), outResult)?;
                outResult
            }
            _ => outResult,
        });
        Ok(outResult)
    }

    pub(crate) fn foldCond<FT: Clone + 'static + metamodelica::gc::MMTrace>(
        mut tree: &metamodelica::Ref<Tree>,
        mut foldFunc: &dyn ::std::ops::Fn(ArcStr, metamodelica::List<TplAbsyn::ASTDef>, FT) -> Result<(FT, bool)>,
        mut value: FT,
    ) -> Result<FT> {
        pub type FoldFunc<FT: Clone + 'static> =
            std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<(FT, bool)> + 'static>;

        let mut value: FT = value;
        value = (match &**tree {
            Tree::NODE {
                key: __tree_key,
                left: __tree_left,
                right: __tree_right,
                value: __tree_value,
                ..
            } => {
                let mut c: bool;
                (value, c) = foldFunc(__tree_key.clone(), __tree_value.clone(), value)?;
                if c {
                    value = foldCond(metamodelica::AsArg::as_arg(&__tree_left), foldFunc, value)?;
                    value = foldCond(metamodelica::AsArg::as_arg(&__tree_right), foldFunc, value)?;
                }
                value
            }
            Tree::LEAF {
                key: __tree_key,
                value: __tree_value,
            } => {
                let mut c: bool;
                (value, c) = foldFunc(__tree_key.clone(), __tree_value.clone(), value)?;
                value
            }
            _ => value,
        });
        Ok(value)
    }

    pub(crate) fn fold_2<
        FT1: Clone + 'static + metamodelica::gc::MMTrace,
        FT2: Clone + 'static + metamodelica::gc::MMTrace,
    >(
        mut tree: &metamodelica::Ref<Tree>,
        mut foldFunc: &dyn ::std::ops::Fn(ArcStr, metamodelica::List<TplAbsyn::ASTDef>, FT1, FT2) -> Result<(FT1, FT2)>,
        mut foldArg1: FT1,
        mut foldArg2: FT2,
    ) -> Result<(FT1, FT2)> {
        pub type FoldFunc<FT1: Clone + 'static, FT2: Clone + 'static> =
            std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT1, FT2) -> Result<(FT1, FT2)> + 'static>;

        let mut foldArg1: FT1 = foldArg1;
        let mut foldArg2: FT2 = foldArg2;
        let () = (match &**tree {
            Tree::NODE {
                key: __tree_key,
                left: __tree_left,
                right: __tree_right,
                value: __tree_value,
                ..
            } => {
                (foldArg1, foldArg2) = fold_2(metamodelica::AsArg::as_arg(&__tree_left), foldFunc, foldArg1, foldArg2)?;
                (foldArg1, foldArg2) = foldFunc(__tree_key.clone(), __tree_value.clone(), foldArg1, foldArg2)?;
                (foldArg1, foldArg2) =
                    fold_2(metamodelica::AsArg::as_arg(&__tree_right), foldFunc, foldArg1, foldArg2)?;
                ()
            }
            Tree::LEAF {
                key: __tree_key,
                value: __tree_value,
            } => {
                (foldArg1, foldArg2) = foldFunc(__tree_key.clone(), __tree_value.clone(), foldArg1, foldArg2)?;
                ()
            }
            _ => (),
        });
        Ok((foldArg1, foldArg2))
    }

    pub(crate) fn forEach(
        mut tree: &metamodelica::Ref<Tree>,
        mut func: &dyn ::std::ops::Fn(ArcStr, metamodelica::List<TplAbsyn::ASTDef>) -> Result<()>,
    ) -> Result<()> {
        pub type EachFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<()> + 'static>;

        let () = (match &**tree {
            Tree::NODE {
                key: __tree_key,
                left: __tree_left,
                right: __tree_right,
                value: __tree_value,
                ..
            } => {
                forEach(metamodelica::AsArg::as_arg(&__tree_left), func)?;
                func(__tree_key.clone(), __tree_value.clone())?;
                forEach(metamodelica::AsArg::as_arg(&__tree_right), func)?;
                ()
            }
            Tree::LEAF {
                key: __tree_key,
                value: __tree_value,
            } => {
                func(__tree_key.clone(), __tree_value.clone())?;
                ()
            }
            Tree::EMPTY { .. } => (),
        });
        Ok(())
    }

    pub(crate) fn fromList(
        mut inValues: &metamodelica::List<(ArcStr, metamodelica::List<TplAbsyn::ASTDef>)>,
        mut conflictFunc: &dyn ::std::ops::Fn(
            metamodelica::List<TplAbsyn::ASTDef>,
            metamodelica::List<TplAbsyn::ASTDef>,
            ArcStr,
        ) -> Result<metamodelica::List<TplAbsyn::ASTDef>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        let mut tree: metamodelica::Ref<Tree> = crate::TplParser::CacheTree::Tree::interned_EMPTY();
        let mut key: Key;
        let mut value: Value;
        for mut t in &**inValues {
            (key, value) = t.clone();
            tree = add(tree, &key, &value, conflictFunc)?;
        }
        Ok(tree)
    }

    pub(crate) fn get<'__b>(mut tree: &'__b metamodelica::Ref<Tree>, mut key: Key) -> Result<Value> {
        let mut value: Value;
        let mut k: Key;
        k = (match &**tree {
            Tree::NODE { .. } => var_field!((**tree).key, Tree::NODE).clone(),
            Tree::LEAF { .. } => var_field!((**tree).key, Tree::LEAF).clone(),
            _ => return Err("match: no arm matched"),
        });
        value = (::match_deref::match_deref! { match &((keyCompare(key.clone(), k), tree.clone())) {
            (0, Deref @ Tree::LEAF { .. }) => var_field!((**tree).value, Tree::LEAF).clone(),
            (0, Deref @ Tree::NODE { .. }) => var_field!((**tree).value, Tree::NODE).clone(),
            (1, Deref @ Tree::NODE { .. }) => get(var_field!((**tree).right, Tree::NODE), key)?,
            ((-1), Deref @ Tree::NODE { .. }) => get(var_field!((**tree).left, Tree::NODE), key)?,
            _ => return Err("match: no arm matched"),
        } });
        Ok(value)
    }

    pub(crate) fn getOpt<'__b>(
        mut tree: &'__b metamodelica::Ref<Tree>,
        mut key: Key,
    ) -> Option<metamodelica::List<TplAbsyn::ASTDef>> {
        '__tco: loop {
            let mut k: Key;
            k = (match &**tree {
                Tree::NODE { .. } => var_field!((**tree).key, Tree::NODE).clone(),
                Tree::LEAF { .. } => var_field!((**tree).key, Tree::LEAF).clone(),
                _ => key.clone(),
            });
            ::match_deref::match_deref! { match &((keyCompare(key.clone(), k), tree.clone())) {
                (0, Deref @ Tree::LEAF { .. }) => return Some(var_field!((**tree).value, Tree::LEAF).clone()),
                (0, Deref @ Tree::NODE { .. }) => return Some(var_field!((**tree).value, Tree::NODE).clone()),
                (1, Deref @ Tree::NODE { .. }) => { (tree, key) = (var_field!((**tree).right, Tree::NODE), key); continue '__tco; },
                ((-1), Deref @ Tree::NODE { .. }) => { (tree, key) = (var_field!((**tree).left, Tree::NODE), key); continue '__tco; },
                _ => return None,
                _ => unreachable!("tail-call lowered match: no arm matched"),
            } }
        }
    }

    pub(crate) fn hasKey(mut inTree: metamodelica::Ref<Tree>, mut inKey: Key) -> Result<bool> {
        let mut comp: bool = false;
        let mut key: Key;
        let mut key_comp: i32;
        let mut tree: metamodelica::Ref<Tree>;
        key = (match &*inTree {
            Tree::NODE { key: __inTree_key, .. } => __inTree_key.clone(),
            Tree::LEAF { key: __inTree_key, .. } => __inTree_key.clone(),
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

    pub(crate) fn intersection() -> Result<()> {
        return Err("fail");
        Ok(())
    }

    pub(crate) fn isEmpty(mut tree: &metamodelica::Ref<Tree>) -> bool {
        let mut isEmpty: bool;
        isEmpty = (match &**tree {
            Tree::EMPTY { .. } => true,
            _ => false,
        });
        isEmpty
    }

    pub(crate) fn join<'__b>(
        mut tree: metamodelica::Ref<Tree>,
        mut treeToJoin: &'__b metamodelica::Ref<Tree>,
        mut conflictFunc: &'__b dyn ::std::ops::Fn(
            metamodelica::List<TplAbsyn::ASTDef>,
            metamodelica::List<TplAbsyn::ASTDef>,
            ArcStr,
        ) -> Result<metamodelica::List<TplAbsyn::ASTDef>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        '__tco: loop {
            match &**treeToJoin {
                Tree::EMPTY { .. } => return Ok(tree),
                Tree::NODE { .. } => {
                    tree = add(
                        tree,
                        var_field!((**treeToJoin).key, Tree::NODE),
                        var_field!((**treeToJoin).value, Tree::NODE),
                        conflictFunc,
                    )?;
                    tree = join(tree, var_field!((**treeToJoin).left, Tree::NODE), conflictFunc)?;
                    {
                        (tree, treeToJoin, conflictFunc) =
                            (tree, var_field!((**treeToJoin).right, Tree::NODE), conflictFunc);
                        continue '__tco;
                    }
                }
                Tree::LEAF { .. } => {
                    return Ok(add(
                        tree,
                        var_field!((**treeToJoin).key, Tree::LEAF),
                        var_field!((**treeToJoin).value, Tree::LEAF),
                        conflictFunc,
                    )?);
                }
            }
        }
    }

    pub(crate) fn listKeys<'__b>(
        mut tree: &'__b metamodelica::Ref<Tree>,
        mut lst: metamodelica::List<ArcStr>,
    ) -> metamodelica::List<ArcStr> {
        '__tco: loop {
            match &**tree {
                Tree::NODE { key, .. } => {
                    lst = listKeys(var_field!((**tree).right, Tree::NODE), lst);
                    lst = metamodelica::cons(key.clone(), lst);
                    {
                        (tree, lst) = (var_field!((**tree).left, Tree::NODE), lst);
                        continue '__tco;
                    }
                }
                Tree::LEAF { key, .. } => return metamodelica::cons(key.clone(), lst),
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

    pub(crate) fn listValues<'__b>(
        mut tree: &'__b metamodelica::Ref<Tree>,
        mut lst: metamodelica::List<metamodelica::List<TplAbsyn::ASTDef>>,
    ) -> metamodelica::List<metamodelica::List<TplAbsyn::ASTDef>> {
        '__tco: loop {
            match &**tree {
                Tree::NODE { value, .. } => {
                    lst = listValues(var_field!((**tree).right, Tree::NODE), lst);
                    lst = metamodelica::cons(value.clone(), lst);
                    {
                        (tree, lst) = (var_field!((**tree).left, Tree::NODE), lst);
                        continue '__tco;
                    }
                }
                Tree::LEAF { value, .. } => return metamodelica::cons(value.clone(), lst),
                _ => return lst,
            }
        }
    }

    pub(crate) fn map(
        mut inTree: metamodelica::Ref<Tree>,
        mut inFunc: &dyn ::std::ops::Fn(
            ArcStr,
            metamodelica::List<TplAbsyn::ASTDef>,
        ) -> Result<metamodelica::List<TplAbsyn::ASTDef>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<Value> + 'static>;

        let mut outTree: metamodelica::Ref<Tree> = inTree.clone();
        outTree = (match &*outTree.clone() {
            Tree::NODE {
                key,
                value,
                left: __outTree_left,
                right: __outTree_right,
                ..
            } => {
                let mut new_value: Value;
                let mut new_left: metamodelica::Ref<Tree>;
                let mut new_right: metamodelica::Ref<Tree>;
                new_left = map(__outTree_left.clone(), inFunc)?;
                new_value = inFunc(key.clone(), value.clone())?;
                new_right = map(__outTree_right.clone(), inFunc)?;
                if !(referenceEq(&*(&*new_left), &*(var_field!((*outTree).left, Tree::NODE).clone())))
                    || !(metamodelica::ReferenceEq::reference_eq(&(value.clone()), &(new_value)))
                    || !(referenceEq(&*(&*new_right), &*(var_field!((*outTree).right, Tree::NODE).clone())))
                {
                    outTree = metamodelica::Ref::new(Tree::NODE {
                        key: key.clone(),
                        value: new_value,
                        height: var_field!((*outTree).height, Tree::NODE).clone(),
                        left: new_left,
                        right: new_right,
                    });
                }
                outTree
            }
            Tree::LEAF { key, value } => {
                let mut new_value: Value;
                new_value = inFunc(key.clone(), value.clone())?;
                if !(metamodelica::ReferenceEq::reference_eq(&(value.clone()), &(new_value))) {
                    assign_variant_field!(outTree => Tree::LEAF; value = new_value);
                }
                outTree
            }
            _ => inTree,
        });
        Ok(outTree)
    }

    pub(crate) fn mapFold<FT: Clone + 'static + metamodelica::gc::MMTrace>(
        mut inTree: metamodelica::Ref<Tree>,
        mut inFunc: &dyn ::std::ops::Fn(
            ArcStr,
            metamodelica::List<TplAbsyn::ASTDef>,
            FT,
        ) -> Result<(metamodelica::List<TplAbsyn::ASTDef>, FT)>,
        mut inStartValue: FT,
    ) -> Result<(metamodelica::Ref<Tree>, FT)> {
        pub type MapFunc<FT: Clone + 'static> =
            std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<Value> + 'static>;

        let mut outTree: metamodelica::Ref<Tree> = inTree.clone();
        let mut outResult: FT = inStartValue;
        outTree = (match &*outTree.clone() {
            Tree::NODE {
                key,
                value,
                left: __outTree_left,
                right: __outTree_right,
                ..
            } => {
                let mut new_value: Value;
                let mut new_left: metamodelica::Ref<Tree>;
                let mut new_right: metamodelica::Ref<Tree>;
                (new_left, outResult) = mapFold(__outTree_left.clone(), inFunc, outResult)?;
                (new_value, outResult) = inFunc(key.clone(), value.clone(), outResult)?;
                (new_right, outResult) = mapFold(__outTree_right.clone(), inFunc, outResult)?;
                if !(referenceEq(&*(&*new_left), &*(var_field!((*outTree).left, Tree::NODE).clone())))
                    || !(metamodelica::ReferenceEq::reference_eq(&(value.clone()), &(new_value)))
                    || !(referenceEq(&*(&*new_right), &*(var_field!((*outTree).right, Tree::NODE).clone())))
                {
                    outTree = metamodelica::Ref::new(Tree::NODE {
                        key: key.clone(),
                        value: new_value,
                        height: var_field!((*outTree).height, Tree::NODE).clone(),
                        left: new_left,
                        right: new_right,
                    });
                }
                outTree
            }
            Tree::LEAF { key, value } => {
                let mut new_value: Value;
                (new_value, outResult) = inFunc(key.clone(), value.clone(), outResult)?;
                if !(metamodelica::ReferenceEq::reference_eq(&(value.clone()), &(new_value))) {
                    assign_variant_field!(outTree => Tree::LEAF; value = new_value);
                }
                outTree
            }
            _ => inTree,
        });
        Ok((outTree, outResult))
    }

    pub(crate) fn new() -> metamodelica::Ref<Tree> {
        let mut outTree: metamodelica::Ref<Tree> = crate::TplParser::CacheTree::Tree::interned_EMPTY();
        outTree
    }

    pub(crate) fn printNodeStr(mut inNode: &metamodelica::Ref<Tree>) -> Result<ArcStr> {
        let mut outString: ArcStr;
        outString = (match &**inNode {
            Tree::NODE {
                key: __inNode_key,
                value: __inNode_value,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*keyStr(__inNode_key.clone()));
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*valueStr(__inNode_value.clone()));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
            Tree::LEAF {
                key: __inNode_key,
                value: __inNode_value,
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*keyStr(__inNode_key.clone()));
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*valueStr(__inNode_value.clone()));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
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
                node = setTreeLeftRight(outNode, __outNode_left.clone(), crate::TplParser::CacheTree::Tree::interned_EMPTY())?;
                setTreeLeftRight(child.clone(), node, crate::TplParser::CacheTree::Tree::interned_EMPTY())?
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
                node = setTreeLeftRight(outNode, crate::TplParser::CacheTree::Tree::interned_EMPTY(), __outNode_right.clone())?;
                setTreeLeftRight(child.clone(), crate::TplParser::CacheTree::Tree::interned_EMPTY(), node)?
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
            (Deref @ Tree::NODE { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => metamodelica::Ref::new(Tree::LEAF { key: var_field!((*orig).key, Tree::NODE).clone(), value: var_field!((*orig).value, Tree::NODE).clone() }),
            (Deref @ Tree::LEAF { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => orig,
            (Deref @ Tree::NODE { .. }, _, _) => if (referenceEqOrEmpty(var_field!((*orig).left, Tree::NODE), &left) && referenceEqOrEmpty(var_field!((*orig).right, Tree::NODE), &right)) {orig} else {metamodelica::Ref::new(Tree::NODE { key: var_field!((*orig).key, Tree::NODE).clone(), value: var_field!((*orig).value, Tree::NODE).clone(), height: std::cmp::max(height(&left), height(&right)) + 1, left: left, right: right })},
            (Deref @ Tree::LEAF { .. }, _, _) => metamodelica::Ref::new(Tree::NODE { key: var_field!((*orig).key, Tree::LEAF).clone(), value: var_field!((*orig).value, Tree::LEAF).clone(), height: std::cmp::max(height(&left), height(&right)) + 1, left: left, right: right }),
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

    pub(crate) fn toList<'__b>(
        mut inTree: &'__b metamodelica::Ref<Tree>,
        mut lst: metamodelica::List<(ArcStr, metamodelica::List<TplAbsyn::ASTDef>)>,
    ) -> metamodelica::List<(ArcStr, metamodelica::List<TplAbsyn::ASTDef>)> {
        '__tco: loop {
            match &**inTree {
                Tree::NODE { key, value, .. } => {
                    lst = toList(var_field!((**inTree).right, Tree::NODE), lst);
                    lst = metamodelica::cons((key.clone(), value.clone()), lst);
                    {
                        (inTree, lst) = (var_field!((**inTree).left, Tree::NODE), lst);
                        continue '__tco;
                    }
                }
                Tree::LEAF { key, value } => return metamodelica::cons((key.clone(), value.clone()), lst),
                _ => return lst,
            }
        }
    }

    pub(crate) fn update(
        mut tree: metamodelica::Ref<Tree>,
        mut key: &Key,
        mut value: &Value,
    ) -> Result<metamodelica::Ref<Tree>> {
        let mut outTree: metamodelica::Ref<Tree> =
            add(tree.clone(), key, value, &move |__a0: metamodelica::List<
                TplAbsyn::ASTDef,
            >,
                                                 __a1: metamodelica::List<
                TplAbsyn::ASTDef,
            >,
                                                 __a2: ArcStr|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(addConflictReplace(__a0, &__a1, &__a2))
            })?;
        Ok(outTree)
    }
}

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ParseInfo {
    pub fileName: ArcStr,
    pub errors: metamodelica::List<ArcStr>,
    pub wasFatalError: bool,
}

impl metamodelica::gc::MMTrace for ParseInfo {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.fileName, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.errors, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.wasFatalError, __mmv)?;
        Ok(())
    }
}
impl Default for ParseInfo {
    fn default() -> Self {
        Self {
            fileName: Default::default(),
            errors: Default::default(),
            wasFatalError: Default::default(),
        }
    }
}

pub type PARSE_INFO = ParseInfo;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct LineInfo {
    pub parseInfo: ParseInfo,
    pub lineNumber: i32,
    pub lineLength: i32,
    pub startOfLineChars: metamodelica::List<ArcStr>,
}

impl metamodelica::gc::MMTrace for LineInfo {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.parseInfo, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.lineNumber, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.lineLength, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.startOfLineChars, __mmv)?;
        Ok(())
    }
}
impl Default for LineInfo {
    fn default() -> Self {
        Self {
            parseInfo: Default::default(),
            lineNumber: Default::default(),
            lineLength: Default::default(),
            startOfLineChars: Default::default(),
        }
    }
}

pub type LINE_INFO = LineInfo;

pub(crate) fn getPosition(mut inChars: metamodelica::List<ArcStr>, mut inLineInfo: &LineInfo) -> Result<(i32, i32)> {
    let mut outLineNumber: i32;
    let mut outColumnNumber: i32;
    (outLineNumber, outColumnNumber) = (match inLineInfo.clone() {
        LineInfo {
            lineNumber: mut lnum,
            lineLength: mut llen,
            ..
        } => {
            let mut chars = inChars;
            let mut tillEnd: i32;
            tillEnd = charsTillEndOfLine(&chars, 0)?;
            (lnum.clone(), llen.clone() - tillEnd)
        }
    });
    Ok((outLineNumber, outColumnNumber))
}

pub type LineColumnNumber = (i32, i32);

pub(crate) static dummySourceInfo: std::sync::LazyLock<SourceInfo> =
    std::sync::LazyLock::new(|| TplAbsyn::dummySourceInfo.clone());

pub(crate) fn captureStartPosition(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: &LineInfo,
    mut inColumnOffset: i32,
) -> Result<LineColumnNumber> {
    let mut outLineColumnNumber: LineColumnNumber;
    let mut line: i32;
    let mut col: i32;
    (line, col) = getPosition(inChars, inLineInfo)?;
    col = col - inColumnOffset;
    outLineColumnNumber = (line, col);
    Ok(outLineColumnNumber)
}

//TODO: add correct TIME_STAMP
pub(crate) fn tplSourceInfo(
    mut inStartLineColumnNumber: LineColumnNumber,
    mut inEndChars: metamodelica::List<ArcStr>,
    mut inEndLineInfo: LineInfo,
) -> Result<SourceInfo> {
    let mut outSourceInfo: SourceInfo;
    outSourceInfo = (match (inStartLineColumnNumber, inEndLineInfo) {
        (
            (mut startL, mut startC),
            ref endlinfo @ LineInfo {
                parseInfo: ParseInfo {
                    fileName: ref fileName, ..
                },
                ..
            },
        ) => {
            let mut endL: i32;
            let mut endC: i32;
            (endL, endC) = getPosition(inEndChars, metamodelica::AsArg::as_arg(&endlinfo))?;
            outSourceInfo = SourceInfo {
                fileName: fileName.clone(),
                isReadOnly: false,
                lineNumberStart: startL.clone(),
                columnNumberStart: startC.clone(),
                lineNumberEnd: endL,
                columnNumberEnd: endC,
                lastModification: metamodelica::OrderedFloat(0.0_f64),
            };
            outSourceInfo
        }
    });
    Ok(outSourceInfo)
}

pub(crate) fn startPositionFromExp(
    mut inExpression: &(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
) -> Result<LineColumnNumber> {
    let mut outLineColumnNumber: LineColumnNumber;
    outLineColumnNumber = (::match_deref::match_deref! { match &(inExpression) {
        (_, SourceInfo { lineNumberStart: startL, columnNumberStart: startC, .. }) => {
            (startL.clone(), startC.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outLineColumnNumber)
}

pub(crate) fn charsTillEndOfLine(mut inChars: &metamodelica::List<ArcStr>, mut outCharsTillEnd: i32) -> Result<i32> {
    let mut outCharsTillEnd: i32 = outCharsTillEnd;
    let mut i: i32;
    for mut c in &**inChars {
        i = stringCharInt(c.clone())?;
        if i == 10 || i == 13 {
            return Ok(outCharsTillEnd);
        }
        outCharsTillEnd = outCharsTillEnd + if (i == 9) { TabSpaces.clone() } else { 1 };
    }
    Ok(outCharsTillEnd)
}

pub(crate) fn makeStartLineInfo(mut inChars: metamodelica::List<ArcStr>, mut inFileName: ArcStr) -> Result<LineInfo> {
    let mut outLineInfo: LineInfo;
    let mut llen: i32;
    llen = charsTillEndOfLine(&inChars, 1)?;
    outLineInfo = LineInfo {
        parseInfo: ParseInfo {
            fileName: inFileName,
            errors: metamodelica::nil(),
            wasFatalError: false,
        },
        lineNumber: 1,
        lineLength: llen,
        startOfLineChars: inChars,
    };
    Ok(outLineInfo)
}

pub(crate) fn printAndFailIfError(mut inLineInfo: &LineInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inLineInfo) {
        LineInfo { parseInfo: ParseInfo { errors: Deref @ metamodelica::ListNode::Nil, .. }, .. } => {
            metamodelica::print(literal!("\nSusan parsing successful.\n"));
            ()
        },
        LineInfo { parseInfo: ParseInfo { errors: errLst @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }, .. } => {
            metamodelica::print(literal!("\nSusan parse error(s):\n"));
            metamodelica::print(stringDelimitList(errLst.clone().reverse(), literal!("\n")));
            metamodelica::print(literal!("\n"));
            return Err("fail")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

pub(crate) fn parseError(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inErrMessage: ArcStr,
    mut isFatal: bool,
) -> Result<LineInfo> {
    let mut outLineInfo: LineInfo;
    outLineInfo = 'mc: {
        let __mc_input = (inChars, inLineInfo, inErrMessage, isFatal);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo @ LineInfo { parseInfo: ParseInfo { fileName: fname, errors: errLst, wasFatalError: false }, lineNumber: lnum, lineLength: llen, startOfLineChars: solchars }, errMsg, isfatal) => {
                    let mut locStr: ArcStr;
                    let mut colnum: i32;
                    let mut errMsg = (*errMsg).clone();
                    (_, colnum) = getPosition(chars.clone(), metamodelica::AsArg::as_arg(&linfo))?;
                    locStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*intString(lnum.clone())); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*intString(colnum)); ArcStr::from(__mm_s) };
                    errMsg = { let mut __mm_s = String::new(); __mm_s.push_str(&*fname); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*locStr); __mm_s.push_str(&*literal!("-")); __mm_s.push_str(&*locStr); __mm_s.push_str(&*literal!(" Error:(parser)")); __mm_s.push_str(&*errMsg); __mm_s.push_str(&*literal!("(col ")); __mm_s.push_str(&*intString(colnum)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.parseError msg: ")); __mm_s.push_str(&*errMsg); ArcStr::from(__mm_s) })?;
                    }
                    Ok(LineInfo { parseInfo: ParseInfo { fileName: fname.clone(), errors: metamodelica::cons(errMsg.clone(), errLst.clone()), wasFatalError: isfatal.clone() }, lineNumber: lnum.clone(), lineLength: llen.clone(), startOfLineChars: solchars.clone() })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, linfo @ LineInfo { parseInfo: ParseInfo { wasFatalError: true, .. }, .. }, _, _) => {
                    Ok(linfo.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- !!! TplParser.parseError failed.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outLineInfo)
}

pub(crate) fn parseErrorPrevPosition(
    mut inCharsPrevPos: metamodelica::List<ArcStr>,
    mut inLineInfoPrevPos: &LineInfo,
    mut inLineInfo: &LineInfo,
    mut inErrMessage: ArcStr,
    mut isFatal: bool,
) -> Result<LineInfo> {
    let mut outLineInfo: LineInfo;
    outLineInfo = 'mc: {
        let __mc_input = (inCharsPrevPos, inLineInfoPrevPos, inLineInfo, inErrMessage, isFatal);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (charspp, LineInfo { lineNumber: lnumpp, lineLength: llenpp, startOfLineChars: solcharspp, .. }, LineInfo { parseInfo: pinfo, lineNumber: lnum, lineLength: llen, startOfLineChars: solchars }, errMsg, isfatal) => {
                    let mut linfopp: LineInfo;
                    let mut pinfo = (*pinfo).clone();
                    linfopp = LineInfo { parseInfo: pinfo.clone(), lineNumber: lnumpp.clone(), lineLength: llenpp.clone(), startOfLineChars: solcharspp.clone() };
                    let LineInfo { parseInfo: __pa0, .. } = parseError(charspp.clone(), linfopp.clone(), errMsg.clone(), isfatal.clone())?;
                    pinfo = metamodelica::Own::own(__pa0);
                    Ok(LineInfo { parseInfo: pinfo.clone(), lineNumber: lnum.clone(), lineLength: llen.clone(), startOfLineChars: solchars.clone() })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- !!! TplParser.parseErrorPrevPosition failed.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outLineInfo)
}

pub(crate) fn wasFatalError(mut inLineInfo: &LineInfo) -> bool {
    let mut outWasError: bool;
    outWasError = (match inLineInfo.clone() {
        LineInfo {
            parseInfo: ParseInfo {
                wasFatalError: true, ..
            },
            ..
        } => true,
        _ => false,
    });
    outWasError
}

pub(crate) fn mergeErrors(mut inLineInfo: &LineInfo, mut inLineInfoToAddErrorsFrom: &LineInfo) -> Result<LineInfo> {
    let mut outLineInfo: LineInfo;
    outLineInfo = (match (inLineInfo.clone(), inLineInfoToAddErrorsFrom.clone()) {
        (
            LineInfo {
                parseInfo:
                    ParseInfo {
                        fileName: mut fname,
                        errors: ref errLst,
                        wasFatalError: mut wasFatalError,
                    },
                lineNumber: mut lnum,
                lineLength: mut llen,
                startOfLineChars: ref solchars,
            },
            LineInfo {
                parseInfo:
                    ParseInfo {
                        errors: ref errLstToAdd,
                        ..
                    },
                ..
            },
        ) => {
            let mut errLst = errLst.clone();
            errLst = listAppend(errLstToAdd.clone(), errLst.clone());
            LineInfo {
                parseInfo: ParseInfo {
                    fileName: fname.clone(),
                    errors: errLst.clone(),
                    wasFatalError: wasFatalError.clone(),
                },
                lineNumber: lnum.clone(),
                lineLength: llen.clone(),
                startOfLineChars: solchars.clone(),
            }
        }
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("- !!! TplParser.mergeErrors failed.\n"))?;
            return Err("fail");
        }
    });
    Ok(outLineInfo)
}

pub(crate) fn parseErrorPrevPositionOpt(
    mut inCharsPrevPos: metamodelica::List<ArcStr>,
    mut inLineInfoPrevPos: LineInfo,
    mut inLineInfo: LineInfo,
    mut inErrMessage: Option<ArcStr>,
    mut isFatal: bool,
) -> Result<LineInfo> {
    let mut outLineInfo: LineInfo;
    outLineInfo = 'mc: {
        let __mc_input = (inCharsPrevPos, inLineInfoPrevPos, inLineInfo, inErrMessage, isFatal);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, linfo, None, _) => {
                    Ok(linfo.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (charspp, linfopp, linfo, Some(errMsg), isfatal) => {
                    let mut linfo = (*linfo).clone();
                    linfo = parseErrorPrevPosition(charspp.clone(), metamodelica::AsArg::as_arg(&linfopp), metamodelica::AsArg::as_arg(&linfo), errMsg.clone(), isfatal.clone())?;
                    Ok(linfo.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- !!! TplParser.parseErrorPrevPositionOpt failed.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outLineInfo)
}

pub(crate) fn parseErrorPrevPositionOptInfoChars(
    mut inLineInfoPrevPos: LineInfo,
    mut inLineInfo: LineInfo,
    mut inErrMessage: Option<ArcStr>,
    mut isFatal: bool,
) -> Result<LineInfo> {
    let mut outLineInfo: LineInfo;
    let mut sol_chars: metamodelica::List<ArcStr>;
    let LineInfo {
        startOfLineChars: __pa0,
        ..
    } = &inLineInfoPrevPos;
    sol_chars = metamodelica::Own::own(__pa0);
    outLineInfo = parseErrorPrevPositionOpt(sol_chars, inLineInfoPrevPos, inLineInfo, inErrMessage, isFatal)?;
    Ok(outLineInfo)
}

pub(crate) fn expectChar(
    mut chars: metamodelica::List<ArcStr>,
    mut lineInfo: LineInfo,
    mut inExpectedChar: &ArcStr,
) -> Result<(metamodelica::List<ArcStr>, LineInfo)> {
    let mut chars: metamodelica::List<ArcStr> = chars;
    let mut lineInfo: LineInfo = lineInfo;
    chars = (::match_deref::match_deref! { match &(chars.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: c, tail: rest } if (stringEq(&c, &inExpectedChar)) => {
            rest.clone()
        },
        _ => {
            lineInfo = parseError(chars.clone(), lineInfo, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expected character '")); __mm_s.push_str(&*inExpectedChar); __mm_s.push_str(&*literal!("' at the position.")); ArcStr::from(__mm_s) }, false)?;
            chars
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((chars, lineInfo))
}

//intended to say error before the last interleave, but need
//TODO: remember the last position before interleave in the LINE_INFO
pub(crate) fn interleaveExpectChar(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inExpectedChar: ArcStr,
) -> Result<(metamodelica::List<ArcStr>, LineInfo)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars, inLineInfo, inExpectedChar);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, ec) => {
                    let mut c: ArcStr;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    c = metamodelica::Own::own(__pa0);
                    chars = metamodelica::Own::own(__pa1);
                    let true = (stringEq(&c, &ec)) else { return Err("pattern mismatch") };
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, ec) => {
                    let mut linfo = (*linfo).clone();
                    linfo = parseError(chars.clone(), linfo.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expected character '")); __mm_s.push_str(&*ec); __mm_s.push_str(&*literal!("' after the position.")); ArcStr::from(__mm_s) }, false)?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- !!! TplParser.interleaveExpectChar failed.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo))
}

pub(crate) fn takeKeywordChars(
    mut inChars: metamodelica::List<ArcStr>,
    mut inKeywordChars: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<ArcStr>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inChars, inKeywordChars)) {
            (Deref @ metamodelica::ListNode::Cons { head: c, tail: chars }, Deref @ metamodelica::ListNode::Cons { head: kwc, tail: kwchars }) => {
                let true = (stringEq(&c, &kwc)) else { return Err("pattern mismatch") };
                { (inChars, inKeywordChars) = (chars.clone(), kwchars.clone()); continue '__tco; }
            },
            (chars, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(chars.clone())
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn isKeyword(
    mut inChars: metamodelica::List<ArcStr>,
    mut inKeywordChars: metamodelica::List<ArcStr>,
) -> (metamodelica::List<ArcStr>, bool) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut isKeyword: bool;
    (outChars, isKeyword) = 'mc: {
        let __mc_input = (inChars.clone(), inKeywordChars);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, kwchars) => {
                    let mut chars = (*chars).clone();
                    chars = takeKeywordChars(chars.clone(), kwchars.clone())?;
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    Ok((chars.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, isKeyword)
}

pub(crate) fn interleaveExpectKeyWord(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inKeywordChars: metamodelica::List<ArcStr>,
    mut isFatal: bool,
) -> Result<(metamodelica::List<ArcStr>, LineInfo)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars, inLineInfo, inKeywordChars, isFatal);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, kwchars, _) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(isKeyword(chars.clone(), kwchars.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    chars = metamodelica::Own::own(__pa0);
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, kwchars, isfatal) => {
                    let mut kw: ArcStr;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    ::match_deref::match_deref! { match &(isKeyword(chars.clone(), kwchars.clone())) {
                        (_, false) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    kw = stringCharListString(kwchars.clone());
                    linfo = parseError(chars.clone(), linfo.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expected keyword '")); __mm_s.push_str(&*kw); __mm_s.push_str(&*literal!("' at the position.")); ArcStr::from(__mm_s) }, isfatal.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- !!! TplParser.interleaveExpectKeyWord failed.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo))
}

pub(crate) fn interleaveExpectEndOfFile(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(metamodelica::List<ArcStr>, LineInfo)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars, inLineInfo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut linfo = (*linfo).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(interleave(chars.clone(), linfo.clone())) {
                        (Deref @ metamodelica::ListNode::Nil, __pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    linfo = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::nil(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    linfo = parseError(chars.clone(), linfo.clone(), literal!("Expected end of file at the position."), false)?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- !!! TplParser.interleaveExpectEndOfFile failed.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo))
}

pub(crate) fn openFile(mut inFile: ArcStr) -> Result<(metamodelica::List<ArcStr>, LineInfo, Option<ArcStr>)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outErrorOpt: Option<ArcStr>;
    (outChars, outLineInfo, outErrorOpt) = 'mc: {
        let __mc_input = inFile.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut file = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut src: ArcStr;
            let mut chars: metamodelica::List<ArcStr>;
            let mut linfo: LineInfo;
            let true = (System::regularFileExists(file.clone())) else {
                return Err("pattern mismatch");
            };
            src = System::readFile(file.clone())?;
            chars = stringListStringChar(src.clone());
            linfo = makeStartLineInfo(chars.clone(), file.clone())?;
            Ok((chars.clone(), linfo.clone(), None))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let mut file = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut errStr: ArcStr;
            let mut chars: metamodelica::List<ArcStr>;
            let mut linfo: LineInfo;
            let false = (System::regularFileExists(file.clone())) else {
                return Err("pattern mismatch");
            };
            chars = metamodelica::nil();
            linfo = makeStartLineInfo(chars.clone(), file.clone())?;
            errStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("No such file '"));
                __mm_s.push_str(&*file);
                __mm_s.push_str(&*literal!("'."));
                ArcStr::from(__mm_s)
            };
            Ok((chars.clone(), linfo.clone(), Some(errStr.clone())))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Parse error - TplParser.openFile failed for file '"));
                __mm_s.push_str(&*inFile);
                __mm_s.push_str(&*literal!("'.\n"));
                ArcStr::from(__mm_s)
            })?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outErrorOpt))
}

pub(crate) fn templPackageFromFile(mut inFile: ArcStr) -> Result<TplAbsyn::TemplPackage> {
    let mut outTemplPackage: TplAbsyn::TemplPackage;
    outTemplPackage = 'mc: {
        let __mc_input = inFile.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut file = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut errOpt: Option<ArcStr>;
            let mut chars: metamodelica::List<ArcStr>;
            let mut linfo: LineInfo;
            let mut tplPackage: TplAbsyn::TemplPackage;
            (chars, linfo, errOpt) = openFile(file.clone())?;
            linfo = parseErrorPrevPositionOpt(chars.clone(), linfo.clone(), linfo.clone(), errOpt.clone(), true)?;
            (chars, linfo, tplPackage, _) = templPackage(
                chars.clone(),
                linfo.clone(),
                crate::TplParser::CacheTree::Tree::interned_EMPTY(),
            )?;
            (_, linfo) = interleaveExpectEndOfFile(chars.clone(), linfo.clone())?;
            printAndFailIfError(&linfo)?;
            Ok(tplPackage.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "Parse error - TplParser.templPackageFromFile failed for file '"
                ));
                __mm_s.push_str(&*inFile);
                __mm_s.push_str(&*literal!("'.\n"));
                ArcStr::from(__mm_s)
            })?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTemplPackage)
}

fn typeviewDefsFromInterfaceFile(
    mut interfaceName: &metamodelica::Ref<TplAbsyn::PathIdent>,
    mut astDefs: metamodelica::List<TplAbsyn::ASTDef>,
    mut cachedDefs: metamodelica::Ref<CacheTree::Tree>,
) -> Result<(
    metamodelica::List<TplAbsyn::ASTDef>,
    LineInfo,
    Option<ArcStr>,
    metamodelica::Ref<CacheTree::Tree>,
)> {
    let mut astDefs: metamodelica::List<TplAbsyn::ASTDef> = astDefs;
    let mut linfo: LineInfo;
    let mut errOpt: Option<ArcStr>;
    let mut cachedDefs: metamodelica::Ref<CacheTree::Tree> = cachedDefs;
    let mut file: ArcStr;
    let mut dir: ArcStr;
    let mut chars: metamodelica::List<ArcStr>;
    let mut newAstDefs: metamodelica::List<TplAbsyn::ASTDef>;
    file = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*TplAbsyn::pathIdentString(interfaceName)?);
        __mm_s.push_str(&*literal!(".mo"));
        ArcStr::from(__mm_s)
    };
    dir = Flags::getConfigString(Flags::TPL_INTERFACE_DIR.clone())?;
    if !metamodelica::stringEq(&dir, &(literal!("")))
        && System::regularFileExists({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*dir);
            __mm_s.push_str(&*literal!("/"));
            __mm_s.push_str(&*file);
            ArcStr::from(__mm_s)
        })
    {
        file = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*dir);
            __mm_s.push_str(&*literal!("/"));
            __mm_s.push_str(&*file);
            ArcStr::from(__mm_s)
        };
    }
    match '__try0: {
        if unwrap_break_err!(CacheTree::hasKey(cachedDefs.clone(), file.clone()), '__try0) {
            astDefs = listAppend(
                unwrap_break_err!(CacheTree::get(&cachedDefs, file.clone()), '__try0),
                astDefs.clone(),
            );
            linfo = LineInfo {
                parseInfo: ParseInfo {
                    fileName: literal!("cachedResult"),
                    errors: metamodelica::nil(),
                    wasFatalError: false,
                },
                lineNumber: 0,
                lineLength: 0,
                startOfLineChars: metamodelica::nil(),
            };
            errOpt = None;
            return Ok((astDefs, linfo, errOpt, cachedDefs));
        }
        (chars, linfo, errOpt) = unwrap_break_err!(openFile(file.clone()), '__try0);
        (chars, linfo) = interleave(chars.clone(), linfo.clone());
        (chars, linfo, _, newAstDefs) =
            unwrap_break_err!(interfacePackage(chars.clone(), linfo.clone(), metamodelica::nil()), '__try0);
        (_, linfo) = unwrap_break_err!(interleaveExpectEndOfFile(chars.clone(), linfo.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::FAILTRACE.clone()), '__try0) {
            unwrap_break_err!(Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Loaded interface file: ")); __mm_s.push_str(&*file); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }), '__try0);
        }
        cachedDefs = unwrap_break_err!(CacheTree::add(cachedDefs.clone(), &file, &newAstDefs, &*((std::sync::Arc::new(CacheTree::addConflictDefault) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))), '__try0);
        astDefs = listAppend(newAstDefs.clone(), astDefs.clone());
        Ok::<_, &'static str>((
            astDefs.clone(),
            cachedDefs.clone(),
            chars.clone(),
            errOpt.clone(),
            linfo.clone(),
            newAstDefs.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5)) => {
            astDefs = __try0_o0;
            cachedDefs = __try0_o1;
            chars = __try0_o2;
            errOpt = __try0_o3;
            linfo = __try0_o4;
            newAstDefs = __try0_o5;
        }
        Err(__try0_err) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Parse error - TplParser.typeviewDefsFromInterfaceFile "));
                    __mm_s.push_str(&*file);
                    __mm_s.push_str(&*literal!(" failed.\n"));
                    ArcStr::from(__mm_s)
                })?;
            }
            return Err(__try0_err);
        }
    }
    Ok((astDefs, linfo, errOpt, cachedDefs))
}

fn typeviewDefsFromTemplateFile(
    mut packageName: metamodelica::Ref<TplAbsyn::PathIdent>,
    mut isUnqualifiedImport: bool,
    mut astDefs: metamodelica::List<TplAbsyn::ASTDef>,
    mut cachedDefs: metamodelica::Ref<CacheTree::Tree>,
) -> Result<(
    metamodelica::List<TplAbsyn::ASTDef>,
    LineInfo,
    Option<ArcStr>,
    metamodelica::Ref<CacheTree::Tree>,
)> {
    let mut astDefs: metamodelica::List<TplAbsyn::ASTDef> = astDefs;
    let mut linfo: LineInfo;
    let mut errOpt: Option<ArcStr>;
    let mut cachedDefs: metamodelica::Ref<CacheTree::Tree> = cachedDefs;
    let mut file: ArcStr;
    let mut chars: metamodelica::List<ArcStr>;
    let mut newAstDef: TplAbsyn::ASTDef;
    let mut tplPackage: TplAbsyn::TemplPackage;
    let mut templateDefs: metamodelica::List<(ArcStr, TplAbsyn::TemplateDef)>;
    let mut astTypes: metamodelica::List<(ArcStr, TplAbsyn::TypeInfo)>;
    file = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*TplAbsyn::pathIdentString(&packageName)?);
        __mm_s.push_str(&*literal!(".tpl"));
        ArcStr::from(__mm_s)
    };
    match '__try0: {
        if unwrap_break_err!(CacheTree::hasKey(cachedDefs.clone(), file.clone()), '__try0) {
            let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(CacheTree::get(&cachedDefs, file.clone()), '__try0)) {
                Deref @ metamodelica::ListNode::Cons { head: TplAbsyn::ASTDef { types: __pa1, .. }, tail: Deref @ metamodelica::ListNode::Nil } => __pa1.clone(),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            astTypes = metamodelica::Own::own(__pa1);
            astDefs = metamodelica::cons(
                TplAbsyn::ASTDef {
                    importPackage: packageName.clone(),
                    isDefault: isUnqualifiedImport,
                    isInterface: false,
                    types: astTypes.clone(),
                },
                astDefs.clone(),
            );
            linfo = LineInfo {
                parseInfo: ParseInfo {
                    fileName: literal!("cachedResult"),
                    errors: metamodelica::nil(),
                    wasFatalError: false,
                },
                lineNumber: 0,
                lineLength: 0,
                startOfLineChars: metamodelica::nil(),
            };
            errOpt = None;
            return Ok((astDefs, linfo, errOpt, cachedDefs));
        }
        (chars, linfo, errOpt) = unwrap_break_err!(openFile(file.clone()), '__try0);
        (chars, linfo, tplPackage, cachedDefs) =
            unwrap_break_err!(templPackage(chars.clone(), linfo.clone(), cachedDefs.clone()), '__try0);
        (_, linfo) = unwrap_break_err!(interleaveExpectEndOfFile(chars.clone(), linfo.clone()), '__try0);
        let TplAbsyn::TEMPL_PACKAGE {
            templateDefs: __pa3, ..
        } = unwrap_break_err!(TplAbsyn::fullyQualifyTemplatePackage(&tplPackage), '__try0);
        templateDefs = metamodelica::Own::own(__pa3);
        astTypes = unwrap_break_err!(List::map(templateDefs.clone(), &move |__a0: (ArcStr, TplAbsyn::TemplateDef)| templateDefToAstDefType(&__a0)), '__try0);
        newAstDef = TplAbsyn::ASTDef {
            importPackage: packageName.clone(),
            isDefault: isUnqualifiedImport,
            isInterface: false,
            types: astTypes.clone(),
        };
        cachedDefs = unwrap_break_err!(CacheTree::add(cachedDefs.clone(), &file, &(metamodelica::cons(newAstDef.clone(), metamodelica::nil())), &*((std::sync::Arc::new(CacheTree::addConflictDefault) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>))), '__try0);
        astDefs = metamodelica::cons(newAstDef.clone(), astDefs.clone());
        if unwrap_break_err!(Flags::isSet(Flags::FAILTRACE.clone()), '__try0) {
            unwrap_break_err!(Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Loaded typeview from template file: ")); __mm_s.push_str(&*file); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }), '__try0);
        }
        Ok::<_, &'static str>((
            astDefs.clone(),
            astTypes.clone(),
            cachedDefs.clone(),
            chars.clone(),
            errOpt.clone(),
            linfo.clone(),
            newAstDef.clone(),
            templateDefs.clone(),
            tplPackage.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7, __try0_o8)) => {
            astDefs = __try0_o0;
            astTypes = __try0_o1;
            cachedDefs = __try0_o2;
            chars = __try0_o3;
            errOpt = __try0_o4;
            linfo = __try0_o5;
            newAstDef = __try0_o6;
            templateDefs = __try0_o7;
            tplPackage = __try0_o8;
        }
        Err(__try0_err) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Parse error - TplParser.typeviewDefsFromInterfaceFile "));
                    __mm_s.push_str(&*file);
                    __mm_s.push_str(&*literal!(" failed.\n"));
                    ArcStr::from(__mm_s)
                })?;
            }
            return Err(__try0_err);
        }
    }
    Ok((astDefs, linfo, errOpt, cachedDefs))
}

pub(crate) fn templateDefToAstDefType(
    mut inTemplateDef: &(ArcStr, TplAbsyn::TemplateDef),
) -> Result<(ArcStr, TplAbsyn::TypeInfo)> {
    let mut outType: (ArcStr, TplAbsyn::TypeInfo);
    outType = 'mc: {
        let __mc_input = inTemplateDef.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let (mut id, TplAbsyn::TemplateDef::STR_TOKEN_DEF { .. }) = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok((
                id.clone(),
                TplAbsyn::TypeInfo::TI_CONST_TYPE {
                    constType: crate::TplAbsyn::TypeSignature::interned_STRING_TOKEN_TYPE(),
                },
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (
                mut id,
                TplAbsyn::TemplateDef::LITERAL_DEF {
                    litType: mut litType, ..
                },
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            Ok((
                id.clone(),
                TplAbsyn::TypeInfo::TI_CONST_TYPE {
                    constType: litType.clone(),
                },
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut id, TplAbsyn::TemplateDef::TEMPLATE_DEF { args: ref iargs, .. }) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut oargs: metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>;
            let mut iargs = iargs.clone();
            iargs = metamodelica::cons(TplAbsyn::imlicitTxtArg.clone(), iargs.clone());
            oargs = List::filterOnTrue(
                iargs.clone(),
                (std::sync::Arc::new(
                    move |__a0: (ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(TplAbsyn::isText(&__a0))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn((ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)) -> Result<bool>
                            + 'static,
                    >),
            )?;
            Ok((
                id.clone(),
                TplAbsyn::TypeInfo::TI_FUN_TYPE {
                    inArgs: iargs.clone(),
                    outArgs: oargs.clone(),
                    tyVars: metamodelica::nil(),
                },
            ))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("Parse error - TplParser.templateDefToAstDefType failed.\n"))?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outType)
}

/*
newLine:
  \r \n  //CR + LF ... Windows
  |
  \n     //CR only ... Linux
  |
  \r     //LF only ... Mac OS up to 9
*/
pub(crate) fn newLine(
    mut inChars: &metamodelica::List<ArcStr>,
    mut inLineInfo: &LineInfo,
) -> Result<(metamodelica::List<ArcStr>, LineInfo)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = (::match_deref::match_deref! { match &((&**inChars, inLineInfo)) {
        (Deref @ metamodelica::ListNode::Cons { head: c, tail: chars }, LineInfo { parseInfo: pinfo, lineNumber: lnum, .. }) => {
            let mut llen: i32;
            let mut i: i32;
            let mut chars = (*chars).clone();
            let mut lnum = (*lnum).clone();
            i = stringCharInt(c.clone())?;
            if i == 13 {
                chars = (::match_deref::match_deref! { match &(chars.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ "\n", tail: chars } => chars.clone(),
        _ => chars.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            let true = (i == 10 || i == 13) else { return Err("pattern mismatch") };
            llen = charsTillEndOfLine(metamodelica::AsArg::as_arg(&chars), 1)?;
            lnum = lnum.clone() + 1;
            (chars.clone(), LineInfo { parseInfo: pinfo.clone(), lineNumber: lnum.clone(), lineLength: llen, startOfLineChars: chars.clone() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outChars, outLineInfo))
}

/*
// interleave will be applied before every token
interleave:  //i.e. space / comment
  [' '\n\r\t] interleave
  |
  '//' toEndOfLine  interleave
  |
  '/''*' comment  interleave
  |
  _ //just nothing
*/
pub(crate) fn interleave(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> (metamodelica::List<ArcStr>, LineInfo) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ " ", tail: chars }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "\t", tail: chars }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "/", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "/", tail: chars } }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = toEndOfLine(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "/", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "*", tail: chars } }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = comment(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "/", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "*", tail: charsRest } }, linfo) => {
                    let mut linfo = (*linfo).clone();
                    if '__try0: {
                        unwrap_break_err!(comment(charsRest.clone(), linfo.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    linfo = parseError(chars.clone(), linfo.clone(), literal!("Unmatched /* */ comment - reached end of file."), true)?;
                    Ok((metamodelica::nil(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = newLine(metamodelica::AsArg::as_arg(&chars), metamodelica::AsArg::as_arg(&linfo))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo)
}

/*
toEndOfLine:
    \n
    |
    eof  //end of stream ~ {}
    |
    any  toEndOfLine //any is any character
*/
pub(crate) fn toEndOfLine(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(metamodelica::List<ArcStr>, LineInfo)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars, inLineInfo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = newLine(metamodelica::AsArg::as_arg(&chars), metamodelica::AsArg::as_arg(&linfo))?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: chars }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = toEndOfLine(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, linfo) => {
                    Ok((metamodelica::nil(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo))
}

//comment:
//  '*''/'
//  |
//  '/''*' comment comment  //nesting is possible
//  |
//  any  comment
pub(crate) fn comment(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(metamodelica::List<ArcStr>, LineInfo)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars, inLineInfo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "*", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "/", tail: chars } }, linfo) => {
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "/", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "*", tail: chars } }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = comment(chars.clone(), linfo.clone())?;
                    (chars, linfo) = comment(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = newLine(metamodelica::AsArg::as_arg(&chars), metamodelica::AsArg::as_arg(&linfo))?;
                    (chars, linfo) = comment(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars @ Deref @ metamodelica::ListNode::Cons { head: _, tail: charsRest }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    if '__try0: {
                        unwrap_break_err!(newLine(metamodelica::AsArg::as_arg(&chars), metamodelica::AsArg::as_arg(&linfo)), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    (chars, linfo) = comment(charsRest.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo))
}

/*
//afterKeyword must not fail after every keyword to be considered as keyword
afterKeyword:
    [_0-9A-Za-z]  =>  fail  // if it can be an identifier/other keyword
    |
    _  => ()
*/
pub(crate) fn afterKeyword(mut inChars: &metamodelica::List<ArcStr>) -> Result<()> {
    let () = (::match_deref::match_deref! { match inChars {
        Deref @ metamodelica::ListNode::Cons { head: c, tail: _ } => {
            let mut i: i32;
            i = stringCharInt(c.clone())?;
            let false = (i == 95 || 48 <= i && i <= 57 || 65 <= i && i <= 90 || 97 <= i && i <= 122) else { return Err("pattern mismatch") };
            ()
        },
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

/*
identifier:
  [_A-Za-z]:c  identifier_rest:rest     =>  stringCharListString(c::rest)
*/
pub(crate) static keywords: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        literal!("end"),
        literal!("if"),
        literal!("then"),
        literal!("else"),
        literal!("match"),
        literal!("case"),
        literal!("equation"),
        literal!("equality"),
        literal!("failure"),
        literal!("algorithm"),
        literal!("input"),
        literal!("output"),
        literal!("matchcontinue"),
        literal!("local"),
        literal!("constant"),
        literal!("extends"),
        literal!("external"),
        literal!("for"),
        literal!("function"),
        literal!("import"),
        literal!("package"),
        literal!("partial"),
        literal!("protected"),
        literal!("public"),
        literal!("record"),
        literal!("as"),
        literal!("uniontype"),
        literal!("subtypeof")
    ]
});

pub(crate) fn identifier(mut inChars: &metamodelica::List<ArcStr>) -> Result<(metamodelica::List<ArcStr>, ArcStr)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outIdent: ArcStr;
    (outChars, outIdent) = (::match_deref::match_deref! { match inChars {
        Deref @ metamodelica::ListNode::Cons { head: c, tail: chars } => {
            let mut restIdChars: metamodelica::List<ArcStr>;
            let mut ident: ArcStr;
            let mut i: i32;
            let mut chars = (*chars).clone();
            i = stringCharInt(c.clone())?;
            let true = (i == 95 || 65 <= i && i <= 90 || 97 <= i && i <= 122) else { return Err("pattern mismatch") };
            (chars, restIdChars) = identifier_rest(metamodelica::AsArg::as_arg(&chars))?;
            ident = stringCharListString(metamodelica::cons(c.clone(), restIdChars));
            (chars.clone(), ident)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outChars, outIdent))
}

/*
identifier_rest:
    [_0-9A-Za-z]:c  identifier_rest:rest  =>  c::rest
    |
  _  =>  {}
*/
pub(crate) fn identifier_rest(
    mut inChars: &metamodelica::List<ArcStr>,
) -> Result<(metamodelica::List<ArcStr>, metamodelica::List<ArcStr>)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outRestIdentChars: metamodelica::List<ArcStr>;
    (outChars, outRestIdentChars) = (::match_deref::match_deref! { match inChars {
        Deref @ metamodelica::ListNode::Cons { head: c, tail: chars } => {
            let mut restIdChars: metamodelica::List<ArcStr>;
            let mut i: i32;
            let mut chars = (*chars).clone();
            i = stringCharInt(c.clone())?;
            if i == 95 || 48 <= i && i <= 57 || 65 <= i && i <= 90 || 97 <= i && i <= 122 {
                (chars, restIdChars) = identifier_rest(metamodelica::AsArg::as_arg(&chars))?;
                restIdChars = metamodelica::cons(c.clone(), restIdChars);
            } else {
                chars = inChars.clone();
                restIdChars = metamodelica::nil();
            }
            (chars.clone(), restIdChars)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outChars, outRestIdentChars))
}

/*
pathIdent:
  identifier:head  pathIdentPath(head):pid => pid
*/
pub(crate) fn pathIdent(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::Ref<TplAbsyn::PathIdent>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outPathIdent: metamodelica::Ref<TplAbsyn::PathIdent>;
    (outChars, outLineInfo, outPathIdent) = (::match_deref::match_deref! { match &((inChars, inLineInfo)) {
        (chars, linfo) => {
            let mut head: ArcStr;
            let mut pid: metamodelica::Ref<TplAbsyn::PathIdent>;
            let mut chars = (*chars).clone();
            let mut linfo = (*linfo).clone();
            (chars, head) = identifier(metamodelica::AsArg::as_arg(&chars))?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone());
            (chars, linfo, pid) = pathIdentPath(chars.clone(), linfo.clone(), head);
            (chars.clone(), linfo.clone(), pid)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outChars, outLineInfo, outPathIdent))
}

/*
pathIdentPath(head):
  '.' pathIdent:path  =>  PATH_IDENT(head, path)
  |
  '.' error "expecting identifier after dot."
    => PATH_IDENT(head, TplAbsyn.IDENT("#error#"))
  |
  _ =>  IDENT(head)
*/
pub(crate) fn pathIdentPath(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inHeadIdent: ArcStr,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::Ref<TplAbsyn::PathIdent>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outPathIdent: metamodelica::Ref<TplAbsyn::PathIdent>;
    (outChars, outLineInfo, outPathIdent) = 'mc: {
        let __mc_input = (&*inChars, inLineInfo.clone(), inHeadIdent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ ".", tail: chars }, linfo, head) => {
                    let mut pid: metamodelica::Ref<TplAbsyn::PathIdent>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, pid) = pathIdent(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::PathIdent::PATH_IDENT { ident: head.clone(), path: pid.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: inHeadIdent.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outPathIdent)
}

pub(crate) fn identifierNoOpt(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(metamodelica::List<ArcStr>, LineInfo, ArcStr)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outIdent: ArcStr;
    (outChars, outLineInfo, outIdent) = 'mc: {
        let __mc_input = (inChars, inLineInfo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut ident: ArcStr;
                    let mut chars = (*chars).clone();
                    (chars, ident) = identifier(metamodelica::AsArg::as_arg(&chars))?;
                    Ok((chars.clone(), linfo.clone(), ident.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut linfo = (*linfo).clone();
                    if '__try0: {
                        unwrap_break_err!(identifier(metamodelica::AsArg::as_arg(&chars)), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    linfo = parseError(chars.clone(), linfo.clone(), literal!("Expected an identifier at the position."), true)?;
                    Ok((chars.clone(), linfo.clone(), literal!("#error#")))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outIdent))
}

pub(crate) fn pathIdentNoOpt(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::Ref<TplAbsyn::PathIdent>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outPathIdent: metamodelica::Ref<TplAbsyn::PathIdent>;
    (outChars, outLineInfo, outPathIdent) = (::match_deref::match_deref! { match &((inChars, inLineInfo)) {
        (chars, linfo) => {
            let mut head: ArcStr;
            let mut pid: metamodelica::Ref<TplAbsyn::PathIdent>;
            let mut chars = (*chars).clone();
            let mut linfo = (*linfo).clone();
            (chars, linfo, head) = identifierNoOpt(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone());
            (chars, linfo, pid) = pathIdentPath(chars.clone(), linfo.clone(), head);
            (chars.clone(), linfo.clone(), pid)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outChars, outLineInfo, outPathIdent))
}

/*
templPackage:
  'package'  pathIdent:pid  stringComment
    definitions(pid,{},{}):(astDefs,templDefs)
  endDefPathIdent(pid)
  =>   TEMPL_PACKAGE(pid, astDefs,templDefs)
*/
pub(crate) fn templPackage(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut cachedDefs: metamodelica::Ref<CacheTree::Tree>,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    TplAbsyn::TemplPackage,
    metamodelica::Ref<CacheTree::Tree>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outTemplPackage: TplAbsyn::TemplPackage;
    let mut cachedDefs: metamodelica::Ref<CacheTree::Tree> = cachedDefs;
    (outChars, outLineInfo, outTemplPackage) = 'mc: {
        let __mc_input = (inChars, inLineInfo);
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut pid: metamodelica::Ref<TplAbsyn::PathIdent>;
                    let mut astDefs: metamodelica::List<TplAbsyn::ASTDef>;
                    let mut templDefs: metamodelica::List<(ArcStr, TplAbsyn::TemplateDef)>;
                    let mut annotationFooter: ArcStr;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    let mut cachedDefs: metamodelica::Ref<CacheTree::Tree> = cachedDefs.clone();
                    (chars, linfo) = interleaveExpectKeyWord(chars.clone(), linfo.clone(), list![literal!("p"), literal!("a"), literal!("c"), literal!("k"), literal!("a"), literal!("g"), literal!("e")], true)?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, pid) = pathIdentNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone());
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, astDefs, templDefs, cachedDefs) = definitions(chars.clone(), linfo.clone(), metamodelica::nil(), metamodelica::nil(), cachedDefs.clone());
                    astDefs = astDefs.clone().reverse();
                    templDefs = templDefs.clone().reverse();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, annotationFooter) = self::annotationFooter(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = endDefPathIdent(chars.clone(), linfo.clone(), pid.clone())?;
                    Ok(((chars.clone(), linfo.clone(), TplAbsyn::TemplPackage { name: pid.clone(), astDefs: astDefs.clone(), templateDefs: templDefs.clone(), annotationFooter: annotationFooter.clone() }), cachedDefs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cachedDefs = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("!!!Parse error - TplParser.templPackage failed.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outTemplPackage, cachedDefs))
}

/*
definitions(astDefs,templDefs):
  'import' 'interface' pathIdent:pid stringComment ';'
    { ads = typeviewDefsFromInterfaceFile(packageNameToFileName(pid,".mo"), astDefs) }
    definitions(ads, templDefs):(ads,tds)
    => (ads,tds)
  |
  'import' pathIdent:pid unqualImportPostfix:unq stringComment ';'
    { ads = typeviewDefsFromTemplateFile(pid, unq, astDefs) }
    definitions(ads, templDefs):(ads,tds)
    => (ads,tds)
//  |
//  absynDef:ad  definitions(ad::astDefs,templDefs):(ads,tds) => (ads,tds)
  |
  templDef:(name, td)  definitions(astDefs,(name,td)::templDefs):(ads,tds) => (ads,tds)
//  |
//  error "Expecting 'end' | ['public' | 'protected' ] 'package' definition | template definition starting with an identifier."
*/
pub(crate) fn definitions(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inAccASTDefs: metamodelica::List<TplAbsyn::ASTDef>,
    mut inAccTemplDefs: metamodelica::List<(ArcStr, TplAbsyn::TemplateDef)>,
    mut cachedDefs: metamodelica::Ref<CacheTree::Tree>,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<TplAbsyn::ASTDef>,
    metamodelica::List<(ArcStr, TplAbsyn::TemplateDef)>,
    metamodelica::Ref<CacheTree::Tree>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outASTDefs: metamodelica::List<TplAbsyn::ASTDef>;
    let mut outTemplDefs: metamodelica::List<(ArcStr, TplAbsyn::TemplateDef)>;
    let mut cachedDefs: metamodelica::Ref<CacheTree::Tree> = cachedDefs;
    (outChars, outLineInfo, outASTDefs, outTemplDefs) = 'mc: {
        let __mc_input = (
            inChars.clone(),
            inLineInfo.clone(),
            inAccASTDefs.clone(),
            inAccTemplDefs.clone(),
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (startChars @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "d", tail: chars } } }, linfo, astDefs, templDefs) => {
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    Ok((startChars.clone(), linfo.clone(), astDefs.clone(), templDefs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "i", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "m", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "p", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "o", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "r", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: chars } } } } } }, linfo, astDefs, templDefs) => {
                    let mut startChars: metamodelica::List<ArcStr>;
                    let mut errOptTV: Option<ArcStr>;
                    let mut startLinfo: LineInfo;
                    let mut linfoTV: LineInfo;
                    let mut pid: metamodelica::Ref<TplAbsyn::PathIdent>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    let mut astDefs = (*astDefs).clone();
                    let mut templDefs = (*templDefs).clone();
                    let mut cachedDefs: metamodelica::Ref<CacheTree::Tree> = cachedDefs.clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ "i", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "r", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "f", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "c", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: __pa0 } } } } } } } } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    chars = metamodelica::Own::own(__pa0);
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (startChars, startLinfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, pid) = pathIdentNoOpt(startChars.clone(), startLinfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone());
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    (astDefs, linfoTV, errOptTV, cachedDefs) = typeviewDefsFromInterfaceFile(&pid, astDefs.clone(), cachedDefs.clone())?;
                    linfo = parseErrorPrevPositionOpt(startChars.clone(), startLinfo.clone(), linfo.clone(), errOptTV.clone(), false)?;
                    linfo = mergeErrors(metamodelica::AsArg::as_arg(&linfo), &linfoTV)?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, astDefs, templDefs, cachedDefs) = definitions(chars.clone(), linfo.clone(), astDefs.clone(), templDefs.clone(), cachedDefs.clone());
                    Ok(((chars.clone(), linfo.clone(), astDefs.clone(), templDefs.clone()), cachedDefs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cachedDefs = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "i", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "m", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "p", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "o", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "r", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: chars } } } } } }, linfo, astDefs, templDefs) => {
                    let mut startChars: metamodelica::List<ArcStr>;
                    let mut errOptTV: Option<ArcStr>;
                    let mut startLinfo: LineInfo;
                    let mut linfoTV: LineInfo;
                    let mut isUnqual: bool;
                    let mut pid: metamodelica::Ref<TplAbsyn::PathIdent>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    let mut astDefs = (*astDefs).clone();
                    let mut templDefs = (*templDefs).clone();
                    let mut cachedDefs: metamodelica::Ref<CacheTree::Tree> = cachedDefs.clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (startChars, startLinfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, pid) = pathIdentNoOpt(startChars.clone(), startLinfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, isUnqual) = unqualImportPostfix(chars.clone(), linfo.clone());
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone());
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    (astDefs, linfoTV, errOptTV, cachedDefs) = typeviewDefsFromTemplateFile(pid.clone(), isUnqual, astDefs.clone(), cachedDefs.clone())?;
                    linfo = parseErrorPrevPositionOpt(startChars.clone(), startLinfo.clone(), linfo.clone(), errOptTV.clone(), false)?;
                    linfo = mergeErrors(metamodelica::AsArg::as_arg(&linfo), &linfoTV)?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, astDefs, templDefs, cachedDefs) = definitions(chars.clone(), linfo.clone(), astDefs.clone(), templDefs.clone(), cachedDefs.clone());
                    Ok(((chars.clone(), linfo.clone(), astDefs.clone(), templDefs.clone()), cachedDefs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cachedDefs = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, astDefs, templDefs) => {
                    let mut name: ArcStr;
                    let mut td: TplAbsyn::TemplateDef;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    let mut astDefs = (*astDefs).clone();
                    let mut templDefs = (*templDefs).clone();
                    let mut cachedDefs: metamodelica::Ref<CacheTree::Tree> = cachedDefs.clone();
                    (chars, linfo, name, td) = templDef(metamodelica::AsArg::as_arg(&chars), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, astDefs, templDefs, cachedDefs) = definitions(chars.clone(), linfo.clone(), astDefs.clone(), metamodelica::cons((name.clone(), td.clone()), templDefs.clone()), cachedDefs.clone());
                    Ok(((chars.clone(), linfo.clone(), astDefs.clone(), templDefs.clone()), cachedDefs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cachedDefs = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), inAccASTDefs.clone(), inAccTemplDefs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outASTDefs, outTemplDefs, cachedDefs)
}

/*
unqualImportPostfix:
  '.' '*' => true
  |
  _ => false
*/
pub(crate) fn unqualImportPostfix(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> (metamodelica::List<ArcStr>, LineInfo, bool) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outIsUnqual: bool;
    (outChars, outLineInfo, outIsUnqual) = 'mc: {
        let __mc_input = (&*inChars, inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ ".", tail: chars }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ "*", tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    chars = metamodelica::Own::own(__pa0);
                    Ok((chars.clone(), linfo.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outIsUnqual)
}

//optional, may fail
pub(crate) fn typeSig(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::Ref<TplAbsyn::TypeSignature>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outTypeSignature: metamodelica::Ref<TplAbsyn::TypeSignature>;
    (outChars, outLineInfo, outTypeSignature) = (::match_deref::match_deref! { match &((inChars, inLineInfo)) {
        (chars, linfo) => {
            let mut ts: metamodelica::Ref<TplAbsyn::TypeSignature>;
            let mut chars = (*chars).clone();
            let mut linfo = (*linfo).clone();
            (chars, linfo, ts) = typeSig_base(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone());
            (chars.clone(), linfo.clone(), ts)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outChars, outLineInfo, outTypeSignature))
}

//must not fail
pub(crate) fn typeSigNoOpt(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::Ref<TplAbsyn::TypeSignature>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outTypeSignature: metamodelica::Ref<TplAbsyn::TypeSignature>;
    (outChars, outLineInfo, outTypeSignature) = 'mc: {
        let __mc_input = (inChars, inLineInfo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut ts: metamodelica::Ref<TplAbsyn::TypeSignature>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, ts) = typeSig(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), ts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut linfo = (*linfo).clone();
                    linfo = parseError(chars.clone(), linfo.clone(), literal!("Expected a type signature at the position."), true)?;
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::TypeSignature::UNRESOLVED_TYPE { reason: literal!("#parse error#") })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outTypeSignature))
}

/*
typeSig_base:
  'list' '<' typeSig:tof '>'  =>  LIST_TYPE(tof)
  |
  'Option' '<' typeSig '>'   =>  OPTION_TYPE(tof)
  |
  'tuple' '<' typeSig:ts  typeSig_restList:restLst  '>'  => TUPLE_TYPE(ts::restLst)
  |
  'array' '<' typeSig:tof '>'  =>  ARRAY_TYPE(tof)
  |
  pathIdent:pid  =>  NAMED_TYPE(pid)  // +specializations for String, Integer, .... => STRING_TYPE(), ...
*/
pub(crate) fn typeSig_base(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::Ref<TplAbsyn::TypeSignature>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outTypeSignature: metamodelica::Ref<TplAbsyn::TypeSignature>;
    (outChars, outLineInfo, outTypeSignature) = 'mc: {
        let __mc_input = (inChars, inLineInfo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "l", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "i", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "s", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: chars } } } }, linfo) => {
                    let mut tof: metamodelica::Ref<TplAbsyn::TypeSignature>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!("<"))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, tof) = typeSigNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!(">"))?;
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::TypeSignature::LIST_TYPE { ofType: tof.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "O", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "p", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "i", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "o", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: chars } } } } } }, linfo) => {
                    let mut tof: metamodelica::Ref<TplAbsyn::TypeSignature>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!("<"))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, tof) = typeSigNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!(">"))?;
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::TypeSignature::OPTION_TYPE { ofType: tof.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "u", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "p", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "l", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: chars } } } } }, linfo) => {
                    let mut tof: metamodelica::Ref<TplAbsyn::TypeSignature>;
                    let mut restLst: metamodelica::List<metamodelica::Ref<TplAbsyn::TypeSignature>>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!("<"))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, tof) = typeSigNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, restLst) = typeSig_restList(metamodelica::AsArg::as_arg(&chars), linfo.clone());
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!(">"))?;
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::TypeSignature::TUPLE_TYPE { ofTypes: metamodelica::cons(tof.clone(), restLst.clone()) })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "r", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "r", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "y", tail: chars } } } } }, linfo) => {
                    let mut tof: metamodelica::Ref<TplAbsyn::TypeSignature>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!("<"))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, tof) = typeSigNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!(">"))?;
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::TypeSignature::ARRAY_TYPE { ofType: tof.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut ts: metamodelica::Ref<TplAbsyn::TypeSignature>;
                    let mut pid: metamodelica::Ref<TplAbsyn::PathIdent>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, pid) = pathIdent(chars.clone(), linfo.clone())?;
                    ts = typeSigFromPathIdent(pid.clone());
                    Ok((chars.clone(), linfo.clone(), ts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outTypeSignature))
}

/*
typeSig_restList:
    ',' typeSig:ts  typeSig_restList:restLst  =>  ts::restLst
    |
    _  => {}
*/
pub(crate) fn typeSig_restList(
    mut inChars: &metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<metamodelica::Ref<TplAbsyn::TypeSignature>>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outTypeSignatureList: metamodelica::List<metamodelica::Ref<TplAbsyn::TypeSignature>>;
    (outChars, outLineInfo, outTypeSignatureList) = 'mc: {
        let __mc_input = (&**inChars, inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ ",", tail: chars }, linfo) => {
                    let mut ts: metamodelica::Ref<TplAbsyn::TypeSignature>;
                    let mut tsLst: metamodelica::List<metamodelica::Ref<TplAbsyn::TypeSignature>>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, ts) = typeSigNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, tsLst) = typeSig_restList(metamodelica::AsArg::as_arg(&chars), linfo.clone());
                    Ok((chars.clone(), linfo.clone(), metamodelica::cons(ts.clone(), tsLst.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outTypeSignatureList)
}

pub(crate) fn typeSigFromPathIdent(
    mut inPathIdent: metamodelica::Ref<TplAbsyn::PathIdent>,
) -> metamodelica::Ref<TplAbsyn::TypeSignature> {
    let mut outTypeSignature: metamodelica::Ref<TplAbsyn::TypeSignature>;
    outTypeSignature = (::match_deref::match_deref! { match &(inPathIdent.clone()) {
        Deref @ TplAbsyn::PathIdent::IDENT { ident: Deref @ "String" } => crate::TplAbsyn::TypeSignature::interned_STRING_TYPE(),
        Deref @ TplAbsyn::PathIdent::IDENT { ident: Deref @ "Integer" } => crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE(),
        Deref @ TplAbsyn::PathIdent::IDENT { ident: Deref @ "Real" } => crate::TplAbsyn::TypeSignature::interned_REAL_TYPE(),
        Deref @ TplAbsyn::PathIdent::IDENT { ident: Deref @ "Boolean" } => crate::TplAbsyn::TypeSignature::interned_BOOLEAN_TYPE(),
        _ => metamodelica::Ref::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: inPathIdent }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTypeSignature
}

/*
publicProtected:
  'public' => true
  |
  'protected' => false
  |
  _ => true
*/
pub(crate) fn publicProtected(mut inChars: metamodelica::List<ArcStr>) -> (metamodelica::List<ArcStr>, bool) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outIsDefault: bool;
    (outChars, outIsDefault) = 'mc: {
        let __mc_input = &*inChars;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ "p", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "u", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "b", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "l", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "i", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "c", tail: chars } } } } } } => {
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    Ok((chars.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ "p", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "r", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "o", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "c", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "d", tail: chars } } } } } } } } } => {
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    Ok((chars.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outIsDefault)
}

/*
stringComment:
  '"' stringCommentRest
  |
  _
*/
pub(crate) fn stringComment(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> (metamodelica::List<ArcStr>, LineInfo) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (startChars @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "\"", tail: chars }, startLinfo) => {
                    let mut linfo: LineInfo;
                    let mut optErr: Option<ArcStr>;
                    let mut chars = (*chars).clone();
                    (chars, linfo, optErr) = stringCommentRest(chars.clone(), startLinfo.clone())?;
                    linfo = parseErrorPrevPositionOpt(startChars.clone(), startLinfo.clone(), linfo.clone(), optErr.clone(), true)?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo)
}

/*
stringCommentRest:
  '\\"' stringCommentRest
  |
  '\\' stringCommentRest
  |
  ~'"' stringCommentRest
  |
  '"'
*/
pub(crate) fn stringCommentRest(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(metamodelica::List<ArcStr>, LineInfo, Option<ArcStr>)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outError: Option<ArcStr>;
    (outChars, outLineInfo, outError) = 'mc: {
        let __mc_input = (inChars, inLineInfo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "\"", tail: chars }, linfo) => {
                    Ok((chars.clone(), linfo.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "\\", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "\"", tail: chars } }, linfo) => {
                    let mut optErr: Option<ArcStr>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, optErr) = stringCommentRest(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), optErr.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "\\", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "\\", tail: chars } }, linfo) => {
                    let mut optErr: Option<ArcStr>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, optErr) = stringCommentRest(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), optErr.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut optErr: Option<ArcStr>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = newLine(metamodelica::AsArg::as_arg(&chars), metamodelica::AsArg::as_arg(&linfo))?;
                    (chars, linfo, optErr) = stringCommentRest(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), optErr.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (startChars @ Deref @ metamodelica::ListNode::Cons { head: _, tail: chars }, linfo) => {
                    let mut optErr: Option<ArcStr>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    if '__try0: {
                        unwrap_break_err!(newLine(metamodelica::AsArg::as_arg(&startChars), metamodelica::AsArg::as_arg(&linfo)), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    (chars, linfo, optErr) = stringCommentRest(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), optErr.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, linfo) => {
                    let mut strErr: ArcStr;
                    strErr = literal!("Unmatched \" \" comment - reached end of file.");
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Parse error - TplParser.stringCommentRest - ")); __mm_s.push_str(&*strErr); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    Ok((metamodelica::nil(), linfo.clone(), Some(strErr.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outError))
}

pub(crate) fn semicolon(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(metamodelica::List<ArcStr>, LineInfo)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars, inLineInfo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ ";", tail: chars }, linfo) => {
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut linfo = (*linfo).clone();
                    linfo = parseError(chars.clone(), linfo.clone(), literal!("Expected semicolon ';' at the position."), false)?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("!!! TplParser.semicolon failed.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo))
}

/*
interfacePackage(astDefs):
  'interface' 'package'  pathIdent:pid  stringComment
    typeviewDefs(astDefs):ads
  endDefPathIdent(pid)
  =>   (pid, ads)
*/
pub(crate) fn interfacePackage(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inAccASTDefs: metamodelica::List<TplAbsyn::ASTDef>,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::Ref<TplAbsyn::PathIdent>,
    metamodelica::List<TplAbsyn::ASTDef>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outPid: metamodelica::Ref<TplAbsyn::PathIdent>;
    let mut outAccASTDefs: metamodelica::List<TplAbsyn::ASTDef>;
    (outChars, outLineInfo, outPid, outAccASTDefs) = 'mc: {
        let __mc_input = (inChars, inLineInfo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut pid: metamodelica::Ref<TplAbsyn::PathIdent>;
                    let mut astDefs: metamodelica::List<TplAbsyn::ASTDef>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleaveExpectKeyWord(chars.clone(), linfo.clone(), list![literal!("i"), literal!("n"), literal!("t"), literal!("e"), literal!("r"), literal!("f"), literal!("a"), literal!("c"), literal!("e")], true)?;
                    (chars, linfo) = interleaveExpectKeyWord(chars.clone(), linfo.clone(), list![literal!("p"), literal!("a"), literal!("c"), literal!("k"), literal!("a"), literal!("g"), literal!("e")], true)?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, pid) = pathIdentNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone());
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, astDefs) = typeviewDefs(chars.clone(), linfo.clone(), inAccASTDefs.clone());
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = endDefPathIdent(chars.clone(), linfo.clone(), pid.clone())?;
                    Ok((chars.clone(), linfo.clone(), pid.clone(), astDefs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("!!!Parse error - TplParser.interfacePackage failed.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outPid, outAccASTDefs))
}

/*
typeviewDefs(astDefs):
  absynDef:ad  typeviewDefs(ad::astDefs):ads => ads
  |
  _ => astDefs
*/
pub(crate) fn typeviewDefs(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inAccASTDefs: metamodelica::List<TplAbsyn::ASTDef>,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<TplAbsyn::ASTDef>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outASTDefs: metamodelica::List<TplAbsyn::ASTDef>;
    (outChars, outLineInfo, outASTDefs) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inAccASTDefs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, astDefs) => {
                    let mut ad: TplAbsyn::ASTDef;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    let mut astDefs = (*astDefs).clone();
                    (chars, linfo, ad) = absynDef(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, astDefs) = typeviewDefs(chars.clone(), linfo.clone(), metamodelica::cons(ad.clone(), astDefs.clone()));
                    Ok((chars.clone(), linfo.clone(), astDefs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), inAccASTDefs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outASTDefs)
}

/*
absynDef:
  publicProtected:isD  'package' pathIdent:pid  stringComment
    absynTypes:types
  endDefPathIdent(pid)
  =>  AST_DEF(pid, isD, true, types)
*/
pub(crate) fn absynDef(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(metamodelica::List<ArcStr>, LineInfo, TplAbsyn::ASTDef)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outASTDef: TplAbsyn::ASTDef;
    (outChars, outLineInfo, outASTDef) = (::match_deref::match_deref! { match &((inChars, inLineInfo)) {
        (chars, linfo) => {
            let mut isD: bool;
            let mut pid: metamodelica::Ref<TplAbsyn::PathIdent>;
            let mut types: metamodelica::List<(ArcStr, TplAbsyn::TypeInfo)>;
            let mut chars = (*chars).clone();
            let mut linfo = (*linfo).clone();
            (chars, isD) = publicProtected(chars.clone());
            (chars, linfo) = interleave(chars.clone(), linfo.clone());
            let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ "p", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "c", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "k", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "g", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: __pa0 } } } } } } } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            chars = metamodelica::Own::own(__pa0);
            afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone());
            (chars, linfo, pid) = pathIdentNoOpt(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone());
            (chars, linfo) = stringComment(chars.clone(), linfo.clone());
            (chars, linfo) = interleave(chars.clone(), linfo.clone());
            (chars, linfo, types) = absynTypes(chars.clone(), linfo.clone());
            (chars, linfo) = interleave(chars.clone(), linfo.clone());
            (chars, linfo) = endDefPathIdent(chars.clone(), linfo.clone(), pid.clone())?;
            (chars.clone(), linfo.clone(), TplAbsyn::ASTDef { importPackage: pid, isDefault: isD, isInterface: true, types: types })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outChars, outLineInfo, outASTDef))
}

/*
//not optional, must not fail
endDefPathIdent(pid):
  'end' pathIdent:pidEnd ';' // pid == pidEnd | warning
*/
pub(crate) fn endDefPathIdent(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inPathIdentToMatch: metamodelica::Ref<TplAbsyn::PathIdent>,
) -> Result<(metamodelica::List<ArcStr>, LineInfo)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars, inLineInfo, inPathIdentToMatch);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "d", tail: chars } } }, linfo, pidToMatch) => {
                    let mut pid: metamodelica::Ref<TplAbsyn::PathIdent>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, pid) = pathIdentNoOpt(chars.clone(), linfo.clone())?;
                    let true = (pid.clone() == pidToMatch.clone()) else { return Err("pattern mismatch") };
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "d", tail: chars } } }, linfo, pidToMatch) => {
                    let mut startChars: metamodelica::List<ArcStr>;
                    let mut startLinfo: LineInfo;
                    let mut pid: metamodelica::Ref<TplAbsyn::PathIdent>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (startChars, startLinfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, pid) = pathIdentNoOpt(startChars.clone(), startLinfo.clone())?;
                    let false = (pid.clone() == pidToMatch.clone()) else { return Err("pattern mismatch") };
                    linfo = parseErrorPrevPosition(startChars.clone(), &startLinfo, metamodelica::AsArg::as_arg(&linfo), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unmatched ident for 'end'. Expected '")); __mm_s.push_str(&*TplAbsyn::pathIdentString(metamodelica::AsArg::as_arg(&pidToMatch))?); __mm_s.push_str(&*literal!("', but '")); __mm_s.push_str(&*TplAbsyn::pathIdentString(&pid)?); __mm_s.push_str(&*literal!("' found instead.")); ArcStr::from(__mm_s) }, false)?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, _) => {
                    let mut linfo = (*linfo).clone();
                    ::match_deref::match_deref! { match &(isKeyword(chars.clone(), metamodelica::cons(literal!("e"), metamodelica::cons(literal!("n"), metamodelica::cons(literal!("d"), metamodelica::nil()))))) {
                        (_, false) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    linfo = parseError(chars.clone(), linfo.clone(), literal!("Expected 'end' keyword at the position."), true)?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, _) => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(literal!("!!!Parse error - TplParser.endDefPathIdent failed.\n"))?;
                    }
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo))
}

/*
//not optional ... must not fail
endDefIdent(id):
  'end' identifier:idEnd ';' // id == idEnd | warning
*/
pub(crate) fn endDefIdent(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inIdentToMatch: ArcStr,
) -> Result<(metamodelica::List<ArcStr>, LineInfo)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inIdentToMatch);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "d", tail: chars } } }, linfo, idToMatch) => {
                    let mut id: ArcStr;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    let true = (id.clone() == idToMatch.clone()) else { return Err("pattern mismatch") };
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "d", tail: chars } } }, linfo, idToMatch) => {
                    let mut startChars: metamodelica::List<ArcStr>;
                    let mut startLinfo: LineInfo;
                    let mut id: ArcStr;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (startChars, startLinfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, id) = identifierNoOpt(startChars.clone(), startLinfo.clone())?;
                    let false = (id.clone() == idToMatch.clone()) else { return Err("pattern mismatch") };
                    linfo = parseErrorPrevPosition(startChars.clone(), &startLinfo, metamodelica::AsArg::as_arg(&linfo), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unmatched ident for 'end'. Expected '")); __mm_s.push_str(&*idToMatch); __mm_s.push_str(&*literal!("', but '")); __mm_s.push_str(&*id); __mm_s.push_str(&*literal!("' found instead.")); ArcStr::from(__mm_s) }, false)?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, _) => {
                    let mut linfo = (*linfo).clone();
                    ::match_deref::match_deref! { match &(isKeyword(chars.clone(), metamodelica::cons(literal!("e"), metamodelica::cons(literal!("n"), metamodelica::cons(literal!("d"), metamodelica::nil()))))) {
                        (_, false) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    linfo = parseError(chars.clone(), linfo.clone(), literal!("Expected 'end' keyword at the position."), true)?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(literal!("!!!Parse error - TplParser.endDefIdent failed.\n"))?;
                    }
                    Ok((inChars.clone(), inLineInfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo))
}

/*
absynTypes:
  absynType:(id,ti)  absynTypes:types  => (id,ti) :: types
  |
  _ => {}
*/
pub(crate) fn absynTypes(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<(ArcStr, TplAbsyn::TypeInfo)>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outTypes: metamodelica::List<(ArcStr, TplAbsyn::TypeInfo)>;
    (outChars, outLineInfo, outTypes) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut idti: (ArcStr, TplAbsyn::TypeInfo);
                    let mut types: metamodelica::List<(ArcStr, TplAbsyn::TypeInfo)>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, idti) = absynType(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, types) = absynTypes(chars.clone(), linfo.clone());
                    Ok((chars.clone(), linfo.clone(), metamodelica::cons(idti.clone(), types.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outTypes)
}

/*
absynType:
  'uniontype' identifier:id  stringComment
      recordTags(id):rtags
  => (id, TI_UNION_TYPE(rtags))
  |
  recordType:(id,fields)
  => (id, TI_RECORD_TYPE(fields))
  |
  'function' identifier:id  stringComment
    inputFunArgs:inArgs
    outputFunArgs:outArgs
  endDefIdent(id)
  => (id, TI_FUN_TYPE(inArgs,outArgs))
  |
  'constant'  typeSig:ts  identifier:id  stringComment  ';'
  => (id, TI_CONST_TYPE(ts))
  |
  'type' identifier:id '=' typeSig:ts stringComment  ';'
  => (id, TI_ALIAS_TYPE(ts))
*/
pub(crate) fn absynType(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(metamodelica::List<ArcStr>, LineInfo, (ArcStr, TplAbsyn::TypeInfo))> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outType: (ArcStr, TplAbsyn::TypeInfo);
    (outChars, outLineInfo, outType) = 'mc: {
        let __mc_input = (inChars, inLineInfo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "u", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "i", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "o", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "y", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "p", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: chars } } } } } } } } }, linfo) => {
                    let mut id: ArcStr;
                    let mut rtags: metamodelica::List<(ArcStr, metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>)>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone());
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, rtags) = recordTags(chars.clone(), linfo.clone(), &id)?;
                    Ok((chars.clone(), linfo.clone(), (id.clone(), TplAbsyn::TypeInfo::TI_UNION_TYPE { recTags: rtags.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut id: ArcStr;
                    let mut fields: metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    let (__pa0, __pa1, (__pa2, __pa3)) = recordType(metamodelica::AsArg::as_arg(&chars), linfo.clone())?;
                    chars = metamodelica::Own::own(__pa0);
                    linfo = metamodelica::Own::own(__pa1);
                    id = metamodelica::Own::own(__pa2);
                    fields = metamodelica::Own::own(__pa3);
                    Ok((chars.clone(), linfo.clone(), (id.clone(), TplAbsyn::TypeInfo::TI_RECORD_TYPE { fields: fields.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "f", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "u", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "c", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "i", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "o", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: chars } } } } } } } }, linfo) => {
                    let mut id: ArcStr;
                    let mut inargs: metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>;
                    let mut outargs: metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>;
                    let mut tyvars: metamodelica::List<ArcStr>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone());
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, tyvars) = typeVars(metamodelica::AsArg::as_arg(&chars), linfo.clone(), metamodelica::nil());
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, inargs) = inputFunArgs(metamodelica::AsArg::as_arg(&chars), linfo.clone());
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, outargs) = outputFunArgs(metamodelica::AsArg::as_arg(&chars), linfo.clone());
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, tyvars) = typeVars(metamodelica::AsArg::as_arg(&chars), linfo.clone(), tyvars.clone());
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = endDefIdent(chars.clone(), linfo.clone(), id.clone())?;
                    Ok((chars.clone(), linfo.clone(), (id.clone(), TplAbsyn::TypeInfo::TI_FUN_TYPE { inArgs: inargs.clone(), outArgs: outargs.clone(), tyVars: tyvars.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "c", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "o", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "s", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: chars } } } } } } } }, linfo) => {
                    let mut id: ArcStr;
                    let mut ts: metamodelica::Ref<TplAbsyn::TypeSignature>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, ts) = typeSigNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone());
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (id.clone(), TplAbsyn::TypeInfo::TI_CONST_TYPE { constType: ts.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "y", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "p", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: chars } } } }, linfo) => {
                    let mut id: ArcStr;
                    let mut ts: metamodelica::Ref<TplAbsyn::TypeSignature>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!("="))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, ts) = typeSigNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone());
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (id.clone(), TplAbsyn::TypeInfo::TI_ALIAS_TYPE { aliasType: ts.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outType))
}

/*
recordType:
  'record' identifier:id  stringComment
      typeDecls:tids
  'end' identifier:idEnd ';' // id == idEnd
  => (id,tids)
*/
pub(crate) fn recordType(
    mut inChars: &metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    (
        ArcStr,
        metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>,
    ),
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outRecordType: (
        ArcStr,
        metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>,
    );
    (outChars, outLineInfo, outRecordType) = (::match_deref::match_deref! { match inChars {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ "r", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "c", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "o", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "r", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "d", tail: chars } } } } } } => {
            let mut linfo = inLineInfo;
            let mut id: ArcStr;
            let mut fields: metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>;
            let mut chars = (*chars).clone();
            afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
            (chars, linfo) = interleave(chars.clone(), linfo);
            (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo)?;
            (chars, linfo) = interleave(chars.clone(), linfo);
            (chars, linfo) = stringComment(chars.clone(), linfo);
            (chars, linfo) = interleave(chars.clone(), linfo);
            (chars, linfo, fields) = typeDecls(chars.clone(), linfo);
            (chars, linfo) = interleave(chars.clone(), linfo);
            (chars, linfo) = endDefIdent(chars.clone(), linfo, id.clone())?;
            (chars.clone(), linfo, (id, fields))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outChars, outLineInfo, outRecordType))
}

/*
typeDecls:
  typeSig:ts  identifier:id  stringComment ';'
  typeDecls:tids
  => (id,ts) :: tids
  |
  _ => {}
*/
pub(crate) fn typeDecls(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outTypeDecls: metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>;
    (outChars, outLineInfo, outTypeDecls) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (startChars @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "d", tail: chars } } }, linfo) => {
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    Ok((startChars.clone(), linfo.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut id: ArcStr;
                    let mut fields: metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>;
                    let mut ts: metamodelica::Ref<TplAbsyn::TypeSignature>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, ts) = typeSig(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone());
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, fields) = typeDecls(chars.clone(), linfo.clone());
                    Ok((chars.clone(), linfo.clone(), metamodelica::cons((id.clone(), ts.clone()), fields.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outTypeDecls)
}

/*
recordTags:
  recordType:(id,tids)  recordTags:rtags  => (id,tids) :: rtags
  |
  _ => {}
*/
pub(crate) fn recordTags(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut id: &ArcStr,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<(
        ArcStr,
        metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>,
    )>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outRecordTags: metamodelica::List<(
        ArcStr,
        metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>,
    )>;
    (outChars, outLineInfo, outRecordTags) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut rtag: (ArcStr, metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>);
                    let mut rtags: metamodelica::List<(ArcStr, metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>)>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, rtag) = recordType(metamodelica::AsArg::as_arg(&chars), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, rtags) = recordTags(chars.clone(), linfo.clone(), id)?;
                    Ok((chars.clone(), linfo.clone(), metamodelica::cons(rtag.clone(), rtags.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = endDefIdent(chars.clone(), linfo.clone(), id.clone())?;
                    Ok((chars.clone(), linfo.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("!!!Parse error - TplParser.recordTags failed at ")); __mm_s.push_str(&*inLineInfo.parseInfo.fileName); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", inLineInfo.lineNumber.clone()))); __mm_s.push_str(&*literal!(".\n")); ArcStr::from(__mm_s) })?;
                    }
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outRecordTags))
}

/*
inputFunArgs:
  'input' typeSig:ts  identifier:id  stringComment
  inputFunArgs:iargs
  => (id,ts) :: iargs
  |
  _ => {}
*/
pub(crate) fn inputFunArgs(
    mut inChars: &metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outTypedIdents: metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>;
    (outChars, outLineInfo, outTypedIdents) = 'mc: {
        let __mc_input = (&**inChars, inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "i", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "p", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "u", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: chars } } } } }, linfo) => {
                    let mut id: ArcStr;
                    let mut inargs: metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>;
                    let mut ts: metamodelica::Ref<TplAbsyn::TypeSignature>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, ts) = typeSigNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone());
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, inargs) = inputFunArgs(metamodelica::AsArg::as_arg(&chars), linfo.clone());
                    Ok((chars.clone(), linfo.clone(), metamodelica::cons((id.clone(), ts.clone()), inargs.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outTypedIdents)
}

/*
outputFunArgs:
  'output' typeSig:ts  identifier:id  stringComment ';'
  outputFunArgs:oargs
  => (id,ts) :: oargs
  |
  _ => {}
*/
pub(crate) fn outputFunArgs(
    mut inChars: &metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outTypedIdents: metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>;
    (outChars, outLineInfo, outTypedIdents) = 'mc: {
        let __mc_input = (&**inChars, inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "o", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "u", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "p", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "u", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: chars } } } } } }, linfo) => {
                    let mut id: ArcStr;
                    let mut outargs: metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>;
                    let mut ts: metamodelica::Ref<TplAbsyn::TypeSignature>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, ts) = typeSigNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = stringComment(chars.clone(), linfo.clone());
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, outargs) = outputFunArgs(metamodelica::AsArg::as_arg(&chars), linfo.clone());
                    Ok((chars.clone(), linfo.clone(), metamodelica::cons((id.clone(), ts.clone()), outargs.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outTypedIdents)
}

/*
typeVars(tyvars):
  'replaceable' 'type'  identifier:id  'subtypeof' 'Any' ';'
  typeVars(id :: tyvars):tyvars
  => tyvars
  |
  _ => tyvars
*/
pub(crate) fn typeVars(
    mut inChars: &metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inTyVars: metamodelica::List<ArcStr>,
) -> (metamodelica::List<ArcStr>, LineInfo, metamodelica::List<ArcStr>) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outTyVars: metamodelica::List<ArcStr>;
    (outChars, outLineInfo, outTyVars) = 'mc: {
        let __mc_input = (&**inChars, inLineInfo.clone(), inTyVars.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "r", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "p", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "l", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "c", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "b", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "l", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: chars } } } } } } } } } } }, linfo, tyvars) => {
                    let mut id: ArcStr;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    let mut tyvars = (*tyvars).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleaveExpectKeyWord(chars.clone(), linfo.clone(), list![literal!("t"), literal!("y"), literal!("p"), literal!("e")], true)?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleaveExpectKeyWord(chars.clone(), linfo.clone(), list![literal!("s"), literal!("u"), literal!("b"), literal!("t"), literal!("y"), literal!("p"), literal!("e"), literal!("o"), literal!("f")], true)?;
                    (chars, linfo) = interleaveExpectKeyWord(chars.clone(), linfo.clone(), list![literal!("A"), literal!("n"), literal!("y")], true)?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = semicolon(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, tyvars) = typeVars(metamodelica::AsArg::as_arg(&chars), linfo.clone(), metamodelica::cons(id.clone(), tyvars.clone()));
                    Ok((chars.clone(), linfo.clone(), tyvars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), inTyVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outTyVars)
}

/*
templDef:
    'template' identifier:name
      '(' templArgs:args ')' stringComment
      templDef_Templ:(exp,lesc,resc)
    endDefIdent(name)
      =>  (name, TEMPLATE_DEF(args,lesc,resc,exp))
    |
    'constant' constantType:ctype  identifier:name templDef_Const:td //check ctype
      stringComment ';'
      => (name, td)
*/
pub(crate) fn templDef(
    mut inChars: &metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(metamodelica::List<ArcStr>, LineInfo, ArcStr, TplAbsyn::TemplateDef)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outTemplName: ArcStr;
    let mut outTemplDef: TplAbsyn::TemplateDef;
    (outChars, outLineInfo, outTemplName, outTemplDef) = (::match_deref::match_deref! { match inChars {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "m", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "p", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "l", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: chars } } } } } } } } => {
            let mut linfo = inLineInfo;
            let mut lesc: ArcStr;
            let mut resc: ArcStr;
            let mut name: ArcStr;
            let mut args: metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>;
            let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
            let mut chars = (*chars).clone();
            afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
            (chars, linfo) = interleave(chars.clone(), linfo);
            (chars, linfo, name) = identifierNoOpt(chars.clone(), linfo)?;
            (chars, linfo) = interleaveExpectChar(chars.clone(), linfo, literal!("("))?;
            (chars, linfo) = interleave(chars.clone(), linfo);
            (chars, linfo, args) = templArgs(chars.clone(), linfo);
            (chars, linfo) = interleaveExpectChar(chars.clone(), linfo, literal!(")"))?;
            (chars, linfo) = interleave(chars.clone(), linfo);
            (chars, linfo) = stringComment(chars.clone(), linfo);
            (chars, linfo) = interleave(chars.clone(), linfo);
            (chars, linfo, exp, lesc, resc) = templDef_Templ(chars.clone(), linfo)?;
            (chars, linfo) = interleave(chars.clone(), linfo);
            (chars, linfo) = endDefIdent(chars.clone(), linfo, name.clone())?;
            (chars.clone(), linfo, name, TplAbsyn::TemplateDef::TEMPLATE_DEF { args: args, lesc: lesc, resc: resc, exp: exp })
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ "c", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "o", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "s", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: chars } } } } } } } } => {
            let mut linfo = inLineInfo;
            let mut name: ArcStr;
            let mut td: TplAbsyn::TemplateDef;
            let mut ctype: metamodelica::Ref<TplAbsyn::TypeSignature>;
            let mut ctypeLit: metamodelica::Ref<TplAbsyn::TypeSignature>;
            let mut chars = (*chars).clone();
            afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
            (chars, linfo) = interleave(chars.clone(), linfo);
            (chars, linfo, ctype) = constantType(chars.clone(), linfo)?;
            (chars, linfo) = interleave(chars.clone(), linfo);
            (chars, linfo, name) = identifierNoOpt(chars.clone(), linfo)?;
            (chars, linfo) = interleave(chars.clone(), linfo);
            (chars, linfo, td, ctypeLit) = templDef_Const(chars.clone(), linfo)?;
            (chars, linfo) = checkConstantType(chars.clone(), linfo, ctype, ctypeLit)?;
            (chars, linfo) = interleave(chars.clone(), linfo);
            (chars, linfo) = stringComment(chars.clone(), linfo);
            (chars, linfo) = interleave(chars.clone(), linfo);
            (chars, linfo) = semicolon(chars.clone(), linfo)?;
            (chars.clone(), linfo, name, td)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outChars, outLineInfo, outTemplName, outTemplDef))
}

/*
templDef_Const:
  '=' stringConstant:strRevList
    =>  STR_TOKEN_DEF(makeStrTokFromRevStrList(strRevList))
  |
  '=' literalConstant:(str,litType)
    =>  LITERAL_DEF(str, litType)
*/
pub(crate) fn templDef_Const(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    TplAbsyn::TemplateDef,
    metamodelica::Ref<TplAbsyn::TypeSignature>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outTemplDef: TplAbsyn::TemplateDef;
    let mut outConstType: metamodelica::Ref<TplAbsyn::TypeSignature>;
    (outChars, outLineInfo, outTemplDef, outConstType) = 'mc: {
        let __mc_input = (inChars, inLineInfo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "=", tail: chars }, linfo) => {
                    let mut strRevList: metamodelica::List<ArcStr>;
                    let mut st: metamodelica::Ref<Tpl::StringToken>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, strRevList) = stringConstant(chars.clone(), linfo.clone())?;
                    st = makeStrTokFromRevStrList(strRevList.clone())?;
                    Ok((chars.clone(), linfo.clone(), TplAbsyn::TemplateDef::STR_TOKEN_DEF { value: st.clone() }, crate::TplAbsyn::TypeSignature::interned_STRING_TYPE()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "=", tail: chars }, linfo) => {
                    let mut r#str: ArcStr;
                    let mut litType: metamodelica::Ref<TplAbsyn::TypeSignature>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, r#str, litType) = literalConstant(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), TplAbsyn::TemplateDef::LITERAL_DEF { value: r#str.clone(), litType: litType.clone() }, litType.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "=", tail: chars }, linfo) => {
                    let mut litType: metamodelica::Ref<TplAbsyn::TypeSignature>;
                    let mut linfo = (*linfo).clone();
                    linfo = parseError(chars.clone(), linfo.clone(), literal!("Expected a constant definition after the '='."), true)?;
                    litType = metamodelica::Ref::new(TplAbsyn::TypeSignature::UNRESOLVED_TYPE { reason: literal!("#Error#") });
                    Ok((chars.clone(), linfo.clone(), TplAbsyn::TemplateDef::LITERAL_DEF { value: literal!("#error#"), litType: litType.clone() }, litType.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut litType: metamodelica::Ref<TplAbsyn::TypeSignature>;
                    let mut linfo = (*linfo).clone();
                    linfo = parseError(chars.clone(), linfo.clone(), literal!("Expected a constant definition after the position."), true)?;
                    litType = metamodelica::Ref::new(TplAbsyn::TypeSignature::UNRESOLVED_TYPE { reason: literal!("#Error#") });
                    Ok((chars.clone(), linfo.clone(), TplAbsyn::TemplateDef::TEMPLATE_DEF { args: metamodelica::nil(), lesc: literal!(""), resc: literal!(""), exp: (crate::TplAbsyn::ExpressionBase::interned_ERROR_EXP(), dummySourceInfo.clone()) }, litType.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outTemplDef, outConstType))
}

/*
constantType:
  'String'  => STRING_TYPE()
  'Integer' => INTEGER_TYPE()
  'Real'    => REAL_TYPE()
  'Boolean' => BOOLEAN_TYPE()
*/
pub(crate) fn constantType(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::Ref<TplAbsyn::TypeSignature>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outConstType: metamodelica::Ref<TplAbsyn::TypeSignature>;
    (outChars, outLineInfo, outConstType) = 'mc: {
        let __mc_input = (inChars, inLineInfo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "S", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "r", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "i", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "g", tail: chars } } } } } }, linfo) => {
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    Ok((chars.clone(), linfo.clone(), crate::TplAbsyn::TypeSignature::interned_STRING_TYPE()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "I", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "g", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "r", tail: chars } } } } } } }, linfo) => {
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    Ok((chars.clone(), linfo.clone(), crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "R", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "l", tail: chars } } } }, linfo) => {
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    Ok((chars.clone(), linfo.clone(), crate::TplAbsyn::TypeSignature::interned_REAL_TYPE()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "B", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "o", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "o", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "l", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: chars } } } } } } }, linfo) => {
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    Ok((chars.clone(), linfo.clone(), crate::TplAbsyn::TypeSignature::interned_BOOLEAN_TYPE()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut linfo = (*linfo).clone();
                    linfo = parseError(chars.clone(), linfo.clone(), literal!("Expected 'String', 'Integer', 'Real' or 'Boolean' type specification for the constant definition after the position."), false)?;
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::TypeSignature::UNRESOLVED_TYPE { reason: literal!("#Error#") })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outConstType))
}

pub(crate) fn checkConstantType(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inConstType: metamodelica::Ref<TplAbsyn::TypeSignature>,
    mut inConstTypeLiteral: metamodelica::Ref<TplAbsyn::TypeSignature>,
) -> Result<(metamodelica::List<ArcStr>, LineInfo)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = (::match_deref::match_deref! { match &((inConstType, inConstTypeLiteral)) {
        (Deref @ TplAbsyn::TypeSignature::UNRESOLVED_TYPE { reason: _ }, _) => {
            let mut chars = inChars.clone();
            let mut linfo = inLineInfo.clone();
            (chars, linfo)
        },
        (_, Deref @ TplAbsyn::TypeSignature::UNRESOLVED_TYPE { reason: _ }) => {
            let mut chars = inChars.clone();
            let mut linfo = inLineInfo.clone();
            (chars, linfo)
        },
        (ctype, litType) if (!(ctype.clone() == litType.clone())) => {
            let mut chars = inChars.clone();
            let mut linfo = inLineInfo.clone();
            linfo = parseError(chars.clone(), linfo, literal!("Declared constant type and the type of the constant's definition literal are different."), false)?;
            (chars, linfo)
        },
        _ => {
            (inChars, inLineInfo)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outChars, outLineInfo))
}

/*
templDef_Templ:
  '::='  expression(LEsc = '<',REsc = '>'):exp   => (exp,'<','>')
  ///|
  //'$$='  expression(LEsc = '$',REsc = '$'):exp   => (exp,'$','$')
*/
pub(crate) fn templDef_Templ(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
    ArcStr,
    ArcStr,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outExpression: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
    let mut outLeftEsc: ArcStr;
    let mut outRightEsc: ArcStr;
    (outChars, outLineInfo, outExpression, outLeftEsc, outRightEsc) = 'mc: {
        let __mc_input = (inChars, inLineInfo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ ":", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ ":", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "=", tail: chars } } }, linfo) => {
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, exp) = expression(chars.clone(), linfo.clone(), literal!("<"), literal!(">"), false)?;
                    Ok((chars.clone(), linfo.clone(), exp.clone(), literal!("<"), literal!(">")))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    if '__try0: {
                        ::match_deref::match_deref! { match &(chars.clone()) {
                            Deref @ metamodelica::ListNode::Cons { head: Deref @ ":", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ ":", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "=", tail: _ } } } => (),
                            _ => break '__try0 Err::<_, _>("pattern mismatch"),
                        } };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    linfo = parseError(chars.clone(), linfo.clone(), literal!("Expected '::=' symbol before a template definition body at the position."), false)?;
                    (chars, linfo, exp) = expression(chars.clone(), linfo.clone(), literal!("<"), literal!(">"), false)?;
                    Ok((chars.clone(), linfo.clone(), exp.clone(), literal!("<"), literal!(">")))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("!!!Parse error - TplParser.templDef_Templ failed.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outExpression, outLeftEsc, outRightEsc))
}

/*
templArgs:
    //TODO: to be TEXT_REF ... for now only syntax
    'Text' '&' identifier:name  templArgs_rest:args  =>  (name,TEXT_TYPE())::args
    |
    typeSig:ts  identifier:name  templArgs_rest:args  =>  (name,ts)::args
    |
    _  => {}
*/
pub(crate) fn templArgs(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outArgs: metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>;
    (outChars, outLineInfo, outArgs) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "T", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "x", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: chars } } } }, linfo) => {
                    let mut name: ArcStr;
                    let mut args: metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ "&", tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    chars = metamodelica::Own::own(__pa0);
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, name) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, args) = templArgs_rest(metamodelica::AsArg::as_arg(&chars), linfo.clone());
                    Ok((chars.clone(), linfo.clone(), metamodelica::cons((name.clone(), crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE()), args.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut name: ArcStr;
                    let mut args: metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>;
                    let mut ts: metamodelica::Ref<TplAbsyn::TypeSignature>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, ts) = typeSig(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, name) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, args) = templArgs_rest(metamodelica::AsArg::as_arg(&chars), linfo.clone());
                    Ok((chars.clone(), linfo.clone(), metamodelica::cons((name.clone(), ts.clone()), args.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outArgs)
}

/*
templArg0:
    typeSig:ts  implicitArgName:name  =>  (name,ts)

*/
/*
public function templArg0
  input list<String> inChars;
  input LineInfo inLineInfo;

  output list<String> outChars;
  output LineInfo outLineInfo;
  output tuple<TplAbsyn.Ident, TplAbsyn.TypeSignature> outArg;
algorithm
  (outChars, outLineInfo, outArg) := matchcontinue (inChars, inLineInfo)
    local
      String lesc, resc;
      list<String> chars;
      LineInfo linfo;
      Boolean isD;
      TplAbsyn.PathIdent pid;
      list<tuple<TplAbsyn.Ident, TplAbsyn.TypeInfo>> types;
      TplAbsyn.Ident name;
      TplAbsyn.TemplateDef td;
      TplAbsyn.TypedIdents args;
      TplAbsyn.Expression exp;
      TplAbsyn.TypeSignature ts;

    case (chars, linfo)
      algorithm
        (chars, linfo, ts) = typeSig(chars, linfo);
        (chars, linfo) = interleave(chars, linfo);
        (chars, name) = implicitArgName(chars);
      then (chars, linfo, (name,ts));

  end matchcontinue;
end templArg0;
*/
/*
implicitArgName:
      IDENT:id  => id  //maybe 'it' explicitly
      |
      _  => 'it'


public function implicitArgName
  input list<String> inChars;

  output list<String> outChars;
  output TplAbsyn.Ident outArgName;
algorithm
  (outChars, outArgName) := matchcontinue (inChars)
    local
      String lesc, resc;
      list<String> chars;
      Boolean isD;
      TplAbsyn.PathIdent pid;
      list<tuple<TplAbsyn.Ident, TplAbsyn.TypeInfo>> types;
      TplAbsyn.Ident name;
      TplAbsyn.TemplateDef td;
      TplAbsyn.TypedIdents args;
      TplAbsyn.Expression exp;
      TplAbsyn.TypeSignature ts;

    case (chars)
      algorithm
        (chars, name) = identifier(chars);
      then (chars, name);

    else (inChars, "it");

  end matchcontinue;
end implicitArgName;
*/
/*
templArgs_rest
  ',' typeSig:ts  argName_nonIt:name  templArgs_rest:rest  =>  (name,ts)::rest
  |
  _  => {}
*/
pub(crate) fn templArgs_rest(
    mut inChars: &metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outArgs: metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>;
    (outChars, outLineInfo, outArgs) = 'mc: {
        let __mc_input = (&**inChars, inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ ",", tail: chars }, linfo) => {
                    let mut name: ArcStr;
                    let mut args: metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ "T", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "x", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: __pa0 } } } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    chars = metamodelica::Own::own(__pa0);
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    let __pa2 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ "&", tail: __pa2 } => __pa2.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    chars = metamodelica::Own::own(__pa2);
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, name) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, args) = templArgs_rest(metamodelica::AsArg::as_arg(&chars), linfo.clone());
                    Ok((chars.clone(), linfo.clone(), metamodelica::cons((name.clone(), crate::TplAbsyn::TypeSignature::interned_TEXT_TYPE()), args.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ ",", tail: chars }, linfo) => {
                    let mut name: ArcStr;
                    let mut ts: metamodelica::Ref<TplAbsyn::TypeSignature>;
                    let mut rest: metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::TypeSignature>)>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, ts) = typeSigNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, name) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, rest) = templArgs_rest(metamodelica::AsArg::as_arg(&chars), linfo.clone());
                    Ok((chars.clone(), linfo.clone(), metamodelica::cons((name.clone(), ts.clone()), rest.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outArgs)
}

/*
argName_nonIt:
      'it'  =>  Error("Implicit argument 'it' appeared at non-fist position in the template argument list. 'it' can be explicitly only as the first argument.")
      |
      IDENT:id  => id
*/
/*
public function argName_nonIt
  input list<String> inChars;
  input LineInfo inLineInfo;

  output list<String> outChars;
  output LineInfo outLineInfo;
  output TplAbsyn.Ident outArgName;
algorithm
  (outChars, outLineInfo, outArgName) := matchcontinue (inChars, inLineInfo)
    local
      String lesc, resc;
      list<String> chars, startChars;
      LineInfo linfo;
      Boolean isD;
      TplAbsyn.PathIdent pid;
      list<tuple<TplAbsyn.Ident, TplAbsyn.TypeInfo>> types;
      TplAbsyn.Ident name;
      TplAbsyn.TemplateDef td;
      TplAbsyn.TypedIdents args;
      TplAbsyn.Expression exp;
      TplAbsyn.TypeSignature ts;
      TplAbsyn.TypedIdents rest;

    case (startChars as ("i"::"t":: chars), linfo)
      algorithm
        afterKeyword(chars);
        (linfo) = parseError(startChars, linfo, "Implicit argument 'it' appeared at non-first position in the template argument list. 'it' can be explicitly only as the first argument.",
        false);
        //true = Flags.isSet(Flags.FAILTRACE); Debug.trace("Parse error - implicit argument 'it' appeared at non-first position in the template argument list. 'it' can be explicitly only as the first argument.\n");
      then (chars, linfo, "#Error-displaced it#");

    case (chars, linfo)
      algorithm
        (chars, linfo, name) = identifierNoOpt(chars, linfo);
      then (chars, linfo, name);

  end matchcontinue;
end argName_nonIt;
*/
/*
expression(lesc,resc):
  expressionNoOptions(lesc,resc):exp  escapedOptions:opts
    => makeEscapedExp(exp, opts)
*/
pub(crate) fn expression(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
    mut isOptional: bool,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outExpression: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = 'mc: {
        let __mc_input = (inChars, inLineInfo, inLeftEsc, inRightEsc, isOptional);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc, _) => {
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut opts: metamodelica::List<(ArcStr, Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>)>;
                    let mut indexOffsetOption: metamodelica::List<(ArcStr, Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>)>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, exp, indexOffsetOption) = expressionNoOptions(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
                    (chars, linfo, opts) = escapedOptions(metamodelica::AsArg::as_arg(&chars), linfo.clone(), lesc.clone(), resc.clone());
                    opts = listAppend(indexOffsetOption.clone(), opts.clone());
                    exp = makeEscapedExp(chars.clone(), linfo.clone(), exp.clone(), opts.clone())?;
                    Ok((chars.clone(), linfo.clone(), exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, _, _, false) => {
                    let mut linfo = (*linfo).clone();
                    linfo = parseError(chars.clone(), linfo.clone(), literal!("Expecting an expression - not able to parse from this point."), true)?;
                    Ok((chars.clone(), linfo.clone(), (crate::TplAbsyn::ExpressionBase::interned_ERROR_EXP(), dummySourceInfo.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outExpression))
}

pub(crate) fn makeEscapedExp(
    mut inEndChars: metamodelica::List<ArcStr>,
    mut inEndLineInfo: LineInfo,
    mut inExpression: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
    mut inOptions: metamodelica::List<(
        ArcStr,
        Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )>,
) -> Result<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)> {
    let mut outExpression: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
    outExpression = (::match_deref::match_deref! { match &(inOptions) {
        Deref @ metamodelica::ListNode::Nil => {
            let mut exp = inExpression;
            exp
        },
        opts @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => {
            let mut exp = inExpression;
            let mut sinfo: SourceInfo;
            sinfo = tplSourceInfo(startPositionFromExp(&exp)?, inEndChars, inEndLineInfo)?;
            (metamodelica::Ref::new(TplAbsyn::ExpressionBase::ESCAPED { exp: exp, options: opts.clone() }), sinfo)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExpression)
}

/*
escapedOptions(lesc,resc):
  ';' identifier:id  escOptionExp(lesc,resc):expOpt  escapedOptions(lesc,resc):opts
  => (id, expOpt) :: opts
  |
  _ => {}

*/
pub(crate) fn escapedOptions(
    mut inChars: &metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<(
        ArcStr,
        Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outOptions: metamodelica::List<(
        ArcStr,
        Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )>;
    (outChars, outLineInfo, outOptions) = 'mc: {
        let __mc_input = (&**inChars, inLineInfo.clone(), inLeftEsc, inRightEsc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ ";", tail: chars }, linfo, lesc, resc) => {
                    let mut id: ArcStr;
                    let mut expOpt: Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>;
                    let mut opts: metamodelica::List<(ArcStr, Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>)>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, expOpt) = escOptionExp(chars.clone(), linfo.clone(), lesc.clone(), resc.clone());
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, opts) = escapedOptions(metamodelica::AsArg::as_arg(&chars), linfo.clone(), lesc.clone(), resc.clone());
                    Ok((chars.clone(), linfo.clone(), metamodelica::cons((id.clone(), expOpt.clone()), opts.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outOptions)
}

/*
escOptionExp(lesc,resc):
  '=' expressionLet(lesc,resc):exp
    => SOME(exp)
  |
  _ => NONE
*/
pub(crate) fn escOptionExp(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outExpOption: Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>;
    (outChars, outLineInfo, outExpOption) = 'mc: {
        let __mc_input = (&*inChars, inLineInfo.clone(), inLeftEsc, inRightEsc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "=", tail: chars }, linfo, lesc, resc) => {
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, exp) = expressionLet(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
                    Ok((chars.clone(), linfo.clone(), Some(exp.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outExpOption)
}

/* not optional
expressionNoOptions(lesc,resc):
  expressionLet(lesc,resc):expLet  mapTailOpt(lesc,resc,expLet):exp
    => exp
*/
pub(crate) fn expressionNoOptions(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
    metamodelica::List<(
        ArcStr,
        Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outExpression: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
    let mut outIndexOffsetOption: metamodelica::List<(
        ArcStr,
        Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )>;
    (outChars, outLineInfo, outExpression, outIndexOffsetOption) = (::match_deref::match_deref! { match &((inChars, inLineInfo, inLeftEsc, inRightEsc)) {
        (chars, linfo, lesc, resc) => {
            let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
            let mut chars = (*chars).clone();
            let mut linfo = (*linfo).clone();
            (chars, linfo, exp) = expressionLet(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone());
            (chars, linfo, exp, outIndexOffsetOption) = mapTailOpt(chars.clone(), linfo.clone(), exp, lesc.clone(), resc.clone());
            (chars.clone(), linfo.clone(), exp, outIndexOffsetOption)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outChars, outLineInfo, outExpression, outIndexOffsetOption))
}

/*
mapTailOpt(headExp,lesc,resc):
  '|>' matchBinding:mexp
  indexedByOpt:idxNmOpt
  '=>' expressionLet(lesc,resc):exp  =>  MAP(headExp,mexp,exp)
  |
  _ => headExp
*/
pub(crate) fn mapTailOpt(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inHeadExpression: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
    metamodelica::List<(
        ArcStr,
        Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outExpression: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
    let mut outIndexOffsetOption: metamodelica::List<(
        ArcStr,
        Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )> = metamodelica::nil();
    (outChars, outLineInfo, outExpression, outIndexOffsetOption) = 'mc: {
        let __mc_input = (
            &*inChars,
            inLineInfo.clone(),
            inHeadExpression.clone(),
            inLeftEsc,
            inRightEsc,
        );
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "|", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ ">", tail: chars } }, linfo, headExp, lesc, resc) => {
                    let mut idxNmOpt: Option<ArcStr>;
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut mexp: metamodelica::Ref<TplAbsyn::MatchingExp>;
                    let mut sinfo: SourceInfo;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    let mut outIndexOffsetOption: metamodelica::List<(ArcStr, Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>)> = outIndexOffsetOption.clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, mexp) = matchBinding(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, idxNmOpt, outIndexOffsetOption) = indexedByOpt(chars.clone(), linfo.clone(), lesc.clone(), resc.clone());
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!("="))?;
                    (chars, linfo) = expectChar(chars.clone(), linfo.clone(), &(literal!(">")))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, exp) = expressionLet(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
                    sinfo = tplSourceInfo(startPositionFromExp(&(headExp.clone()))?, chars.clone(), linfo.clone())?;
                    Ok(((chars.clone(), linfo.clone(), (metamodelica::Ref::new(TplAbsyn::ExpressionBase::MAP { argExp: headExp.clone(), ofBinding: mexp.clone(), mapExp: exp.clone(), hasIndexIdentOpt: idxNmOpt.clone() }), sinfo.clone()), outIndexOffsetOption.clone()), outIndexOffsetOption.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outIndexOffsetOption = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), inHeadExpression.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outExpression, outIndexOffsetOption)
}

/*
indexedByOpt:
  'hasindex' identifier:id
    => SOME(id)
  |
  _ => NONE
*/
pub(crate) fn indexedByOpt(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    Option<ArcStr>,
    metamodelica::List<(
        ArcStr,
        Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outIndexNameOpt: Option<ArcStr>;
    let mut outIndexOffsetOption: metamodelica::List<(
        ArcStr,
        Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )> = metamodelica::nil();
    (outChars, outLineInfo, outIndexNameOpt, outIndexOffsetOption) = 'mc: {
        let __mc_input = (&*inChars, inLineInfo.clone(), inLeftEsc, inRightEsc);
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "h", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "s", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "i", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "d", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "x", tail: chars } } } } } } } }, linfo, lesc, resc) => {
                    let mut id: ArcStr;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    let mut outIndexOffsetOption: metamodelica::List<(ArcStr, Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>)> = outIndexOffsetOption.clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, outIndexOffsetOption) = fromOpt(chars.clone(), linfo.clone(), lesc.clone(), resc.clone());
                    Ok(((chars.clone(), linfo.clone(), Some(id.clone()), outIndexOffsetOption.clone()), outIndexOffsetOption.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outIndexOffsetOption = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), None, metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outIndexNameOpt, outIndexOffsetOption)
}

/*
fromOpt:
  'fromindex' expression_base:expFrom
    => { ("$indexOffset", SOME(expFrom)) }
  |
  _ => {}
*/
pub(crate) fn fromOpt(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<(
        ArcStr,
        Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outIndexOffsetOption: metamodelica::List<(
        ArcStr,
        Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )>;
    (outChars, outLineInfo, outIndexOffsetOption) = 'mc: {
        let __mc_input = (&*inChars, inLineInfo.clone(), inLeftEsc, inRightEsc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "f", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "r", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "o", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "m", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "i", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "d", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "x", tail: chars } } } } } } } } }, linfo, lesc, resc) => {
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, exp) = expression_base(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
                    Ok((chars.clone(), linfo.clone(), list![(arcstr::literal!(TplAbsyn::indexOffsetOptionId), Some(exp.clone()))]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "f", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "r", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "o", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "m", tail: chars } } } }, linfo, lesc, resc) => {
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    linfo = parseError(chars.clone(), linfo.clone(), literal!("Keyword 'from' was changed to 'fromindex', please update your source code here."), false)?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, exp) = expression_base(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
                    Ok((chars.clone(), linfo.clone(), list![(arcstr::literal!(TplAbsyn::indexOffsetOptionId), Some(exp.clone()))]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outIndexOffsetOption)
}

/*
expressionLet(lesc,resc):
  'let' letExp(lesc,resc):lexp  concatLetExp_rest(lesc,resc):expLst
     => TEMPLATE(lexp::expLst}, "let", ""); //TODO: should be a LET_EXPRESSION()
  |
  expressionMatch(lesc,resc):exp
*/
pub(crate) fn expressionLet(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outExpression: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = 'mc: {
        let __mc_input = (inChars, inLineInfo, inLeftEsc, inRightEsc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "l", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: startChars } } }, startLInfo, lesc, resc) => {
                    let mut chars: metamodelica::List<ArcStr>;
                    let mut linfo: LineInfo;
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut lexp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut sinfo: SourceInfo;
                    afterKeyword(metamodelica::AsArg::as_arg(&startChars))?;
                    (chars, linfo) = interleave(startChars.clone(), startLInfo.clone());
                    (chars, linfo, lexp) = letExp(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, exp) = expressionLet(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), metamodelica::AsArg::as_arg(&startLInfo), 3)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (metamodelica::Ref::new(TplAbsyn::ExpressionBase::LET { letExp: lexp.clone(), exp: exp.clone() }), sinfo.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc) => {
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, exp) = expressionMatch(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
                    Ok((chars.clone(), linfo.clone(), exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outExpression))
}

/*
concatLetExp_rest(lesc,resc):
  'let' letExp(lesc,resc):lexp  concatLetExp_rest(lesc,resc):expLst
    =>  lexp::expLst
  |
  expression(lesc,resc):exp
    => {exp}
*/
/*
public function concatLetExp_rest
  input list<String> inChars;
  input LineInfo inLineInfo;
  input String inLeftEsc;
  input String inRightEsc;

  output list<String> outChars;
  output LineInfo outLineInfo;
  output list<TplAbsyn.Expression> outExpressionList;
algorithm
  (outChars, outLineInfo, outExpressionList) := matchcontinue (inChars, inLineInfo, inLeftEsc, inRightEsc)
    local
      list<String> chars;
      LineInfo linfo;
      String c, lesc, resc;
      Boolean isD;
      TplAbsyn.Ident id;
      TplAbsyn.PathIdent name;
      TplAbsyn.TypedIdents fields,inargs,outargs;
      TplAbsyn.TypeSignature ts;
      Tpl.StringToken st;
      TplAbsyn.Expression exp, lexp;
      list<TplAbsyn.Expression> expLst;
      TplAbsyn.MatchingExp mexp;

    case ("l"::"e"::"t":: chars, linfo, lesc, resc)
      algorithm
        afterKeyword(chars);
        (chars, linfo) = interleave(chars, linfo);
        (chars, linfo, lexp) = letExp(chars, linfo, lesc, resc);
        (chars, linfo) = interleave(chars, linfo);
        (chars, linfo, expLst) = concatLetExp_rest(chars, linfo, lesc, resc);
      then (chars, linfo, lexp::expLst);

    case (chars, linfo, lesc, resc)
      algorithm
        (chars, linfo, exp) = expressionMatch(chars, linfo, lesc, resc);
      then (chars, linfo, {exp});

  end matchcontinue;
end concatLetExp_rest;
*/
/*
must not fail - not optional, at least one must match
letExp(lesc,resc):
  '&' identifier:id '=' 'buffer' expression(lesc,resc):exp
       => TEXT_CREATE(id,exp)
  |
  '&' identifier:id '+=' expression(lesc,resc):exp
       => TEXT_ADD(id,exp)
  |
  '()' '=' pathIdent:name  funCall(name,lesc,resc):exp
       =>  exp //TODO: noRetCall expression should be here
  |
  identifier:id '=' expression(lesc,resc):exp
    => TEXT_CREATE(id,exp) //TODO: !! a HACK for now

  //TODO:
  |
  letBinding:bd '=' expression(lesc,resc):exp
    =>  LET_BINDING(bd, exp)
*/
pub(crate) fn letExp(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outExpression: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = 'mc: {
        let __mc_input = (inChars, inLineInfo, inLeftEsc, inRightEsc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "&", tail: startChars }, startLInfo, lesc, resc) => {
                    let mut chars: metamodelica::List<ArcStr>;
                    let mut linfo: LineInfo;
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut id: ArcStr;
                    let mut sinfo: SourceInfo;
                    (chars, linfo) = interleave(startChars.clone(), startLInfo.clone());
                    (chars, id) = identifier(&chars)?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), metamodelica::AsArg::as_arg(&startLInfo), 1)?, chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ "=", tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    chars = metamodelica::Own::own(__pa0);
                    (chars, linfo) = interleaveExpectKeyWord(chars.clone(), linfo.clone(), list![literal!("b"), literal!("u"), literal!("f"), literal!("f"), literal!("e"), literal!("r")], false)?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, exp) = expression(chars.clone(), linfo.clone(), lesc.clone(), resc.clone(), false)?;
                    Ok((chars.clone(), linfo.clone(), (metamodelica::Ref::new(TplAbsyn::ExpressionBase::TEXT_CREATE { name: id.clone(), exp: exp.clone() }), sinfo.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "&", tail: startChars }, startLInfo, lesc, resc) => {
                    let mut chars: metamodelica::List<ArcStr>;
                    let mut linfo: LineInfo;
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut id: ArcStr;
                    let mut sinfo: SourceInfo;
                    (chars, linfo) = interleave(startChars.clone(), startLInfo.clone());
                    (chars, id) = identifier(&chars)?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), metamodelica::AsArg::as_arg(&startLInfo), 1)?, chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ "+", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "=", tail: __pa0 } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    chars = metamodelica::Own::own(__pa0);
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, exp) = expression(chars.clone(), linfo.clone(), lesc.clone(), resc.clone(), false)?;
                    Ok((chars.clone(), linfo.clone(), (metamodelica::Ref::new(TplAbsyn::ExpressionBase::TEXT_ADD { name: id.clone(), exp: exp.clone() }), sinfo.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "&", tail: startChars }, startLInfo, _, _) => {
                    let mut chars: metamodelica::List<ArcStr>;
                    let mut linfo: LineInfo;
                    (chars, linfo) = interleave(startChars.clone(), startLInfo.clone());
                    (chars, linfo, _) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    linfo = parseError(chars.clone(), linfo.clone(), literal!("Expecting a '=' or '+=' text variable creation/addition (&var = exp or &var += exp) at the position."), true)?;
                    Ok((chars.clone(), linfo.clone(), (crate::TplAbsyn::ExpressionBase::interned_ERROR_EXP(), dummySourceInfo.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "(", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ ")", tail: startChars } }, startLInfo, lesc, resc) => {
                    let mut chars: metamodelica::List<ArcStr>;
                    let mut linfo: LineInfo;
                    let mut name: metamodelica::Ref<TplAbsyn::PathIdent>;
                    let mut args: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>;
                    let mut sinfo: SourceInfo;
                    (chars, linfo) = interleaveExpectChar(startChars.clone(), startLInfo.clone(), literal!("="))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, name) = pathIdentNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(funCall(&chars, linfo.clone(), name.clone(), lesc.clone(), resc.clone())?) {
                        (__pa0, __pa1, Deref @ TplAbsyn::ExpressionBase::FUN_CALL { name: __pa2, args: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    chars = metamodelica::Own::own(__pa0);
                    linfo = metamodelica::Own::own(__pa1);
                    name = metamodelica::Own::own(__pa2);
                    args = metamodelica::Own::own(__pa3);
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), metamodelica::AsArg::as_arg(&startLInfo), 2)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (metamodelica::Ref::new(TplAbsyn::ExpressionBase::NORET_CALL { name: name.clone(), args: args.clone() }), sinfo.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "(", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ ")", tail: startChars } }, startLInfo, _, _) => {
                    let mut chars: metamodelica::List<ArcStr>;
                    let mut linfo: LineInfo;
                    (chars, linfo) = interleaveExpectChar(startChars.clone(), startLInfo.clone(), literal!("="))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, _) = pathIdentNoOpt(chars.clone(), linfo.clone())?;
                    linfo = parseError(chars.clone(), linfo.clone(), literal!("Expecting a non-return function call( let () = [package.]funName(args,...) ) at the position."), true)?;
                    Ok((chars.clone(), linfo.clone(), (crate::TplAbsyn::ExpressionBase::interned_ERROR_EXP(), dummySourceInfo.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (startChars, startLInfo, lesc, resc) => {
                    let mut chars: metamodelica::List<ArcStr>;
                    let mut linfo: LineInfo;
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut id: ArcStr;
                    let mut sinfo: SourceInfo;
                    (chars, linfo, id) = identifierNoOpt(startChars.clone(), startLInfo.clone())?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), metamodelica::AsArg::as_arg(&startLInfo), 0)?, chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!("="))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, exp) = expression(chars.clone(), linfo.clone(), lesc.clone(), resc.clone(), false)?;
                    Ok((chars.clone(), linfo.clone(), (metamodelica::Ref::new(TplAbsyn::ExpressionBase::TEXT_CREATE { name: id.clone(), exp: exp.clone() }), sinfo.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(literal!("!!!Parse error - TplParser.letExp failed.\n"))?;
                    }
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outExpression))
}

/*
expressionMatch(lesc,resc):
  matchExp(lesc,resc):exp
    => exp
  |
  expressionIf(lesc,resc):exp
    => exp
*/
pub(crate) fn expressionMatch(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outExpression: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = 'mc: {
        let __mc_input = (inChars, inLineInfo, inLeftEsc, inRightEsc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc) => {
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, exp) = matchExp(metamodelica::AsArg::as_arg(&chars), linfo.clone(), lesc.clone(), resc.clone())?;
                    Ok((chars.clone(), linfo.clone(), exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc) => {
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, exp) = expressionIf(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
                    Ok((chars.clone(), linfo.clone(), exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outExpression))
}

/*
expressionIf(lesc,resc):
  conditionExp(lesc,resc):exp
    => exp
  |
  expressionPlus(lesc,resc):exp
    => exp
*/
pub(crate) fn expressionIf(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outExpression: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = 'mc: {
        let __mc_input = (inChars, inLineInfo, inLeftEsc, inRightEsc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc) => {
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, exp) = conditionExp(metamodelica::AsArg::as_arg(&chars), linfo.clone(), lesc.clone(), resc.clone())?;
                    Ok((chars.clone(), linfo.clone(), exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc) => {
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, exp) = expressionPlus(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
                    Ok((chars.clone(), linfo.clone(), exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outExpression))
}

/*
expressionPlus(lesc,resc):
  expression_base(lesc,resc):bexp  plusTailOpt(lesc,resc,bexp):exp
    => exp
*/
pub(crate) fn expressionPlus(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outExpression: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = (::match_deref::match_deref! { match &((inChars, inLineInfo, inLeftEsc, inRightEsc)) {
        (chars, linfo, lesc, resc) => {
            let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
            let mut chars = (*chars).clone();
            let mut linfo = (*linfo).clone();
            (chars, linfo, exp) = expression_base(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone());
            (chars, linfo, exp) = plusTailOpt(chars.clone(), linfo.clone(), exp, lesc.clone(), resc.clone());
            (chars.clone(), linfo.clone(), exp)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outChars, outLineInfo, outExpression))
}

/*
plusTailOpt(lesc,resc,bexp):
  '+' expression_base(lesc,resc):exp  concatExp_rest(lesc,resc):expLst   //  concatenation same as "<expression><expression>"
    => TEMPLATE(bexp::exp::expLst, "+", "");
  |
  _ => bexp
*/
pub(crate) fn plusTailOpt(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inBaseExpression: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outExpression: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = 'mc: {
        let __mc_input = (
            &*inChars,
            inLineInfo.clone(),
            inBaseExpression.clone(),
            inLeftEsc,
            inRightEsc,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "+", tail: chars }, linfo, bexp, lesc, resc) => {
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut expLst: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>;
                    let mut sinfo: SourceInfo;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, exp) = expression_base(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, expLst) = concatExp_rest(metamodelica::AsArg::as_arg(&chars), linfo.clone(), lesc.clone(), resc.clone());
                    sinfo = tplSourceInfo(startPositionFromExp(&(bexp.clone()))?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (metamodelica::Ref::new(TplAbsyn::ExpressionBase::TEMPLATE { items: metamodelica::cons(bexp.clone(), metamodelica::cons(exp.clone(), expLst.clone())), lquote: literal!("+"), rquote: literal!("") }), sinfo.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), inBaseExpression.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outExpression)
}

/*
concatExp_rest(lesc,resc):
  '+' expression_base(lesc,resc):exp  concatExp_rest(lesc,resc):expLst  =>  exp::expLst
  |
  _ => {}
*/
pub(crate) fn concatExp_rest(
    mut inChars: &metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outExpressionList: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>;
    (outChars, outLineInfo, outExpressionList) = 'mc: {
        let __mc_input = (&**inChars, inLineInfo.clone(), inLeftEsc, inRightEsc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "+", tail: chars }, linfo, lesc, resc) => {
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut expLst: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, exp) = expression_base(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, expLst) = concatExp_rest(metamodelica::AsArg::as_arg(&chars), linfo.clone(), lesc.clone(), resc.clone());
                    Ok((chars.clone(), linfo.clone(), metamodelica::cons(exp.clone(), expLst.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outExpressionList)
}

/*
expression_base(lesc,resc):
  stringConstant:strRevList
    => STR_TOKEN(makeStrTokFromRevStrList(strRevList))
  |
  literalConstant:(str,litType)
    => LITERAL(str,litType)
  |
  templateExp(lesc,resc)
  |
  '{' '}'  => MAP_ARG_LIST({})
  |
  '{' expressionPlus(lesc,resc):exp  expressionList_rest(lesc,resc):expLst '}'   //  list construction with possible mixed scalars and lists
                                                             // useful in map/concatenation context
     => MAP_ARG_LIST(exp::expLst)
  |
  '(' expression(lesc,resc):exp ')'
     => exp
  |
  '&' identifier:id
    => BOUND_VALUE(IDENT(name))  //TODO: ref Text buffer
  |// TODO: create an optional/error reporting variant of pathIdent
  pathIdent:name  boundValueOrFunCall(name,lesc,resc):exp  =>  exp
*/
pub(crate) fn expression_base(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outExpression: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = 'mc: {
        let __mc_input = (inChars, inLineInfo, inLeftEsc, inRightEsc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (startChars, startLInfo, _, _) => {
                    let mut chars: metamodelica::List<ArcStr>;
                    let mut strRevList: metamodelica::List<ArcStr>;
                    let mut linfo: LineInfo;
                    let mut st: metamodelica::Ref<Tpl::StringToken>;
                    let mut sinfo: SourceInfo;
                    (chars, linfo, strRevList) = stringConstant(startChars.clone(), startLInfo.clone())?;
                    st = makeStrTokFromRevStrList(strRevList.clone())?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), metamodelica::AsArg::as_arg(&startLInfo), 0)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: st.clone() }), sinfo.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (startChars, startLInfo, _, _) => {
                    let mut chars: metamodelica::List<ArcStr>;
                    let mut linfo: LineInfo;
                    let mut r#str: ArcStr;
                    let mut ts: metamodelica::Ref<TplAbsyn::TypeSignature>;
                    let mut sinfo: SourceInfo;
                    (chars, linfo, r#str, ts) = literalConstant(startChars.clone(), startLInfo.clone())?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), metamodelica::AsArg::as_arg(&startLInfo), 0)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (metamodelica::Ref::new(TplAbsyn::ExpressionBase::LITERAL { value: r#str.clone(), litType: ts.clone() }), sinfo.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc) => {
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, exp) = templateExp(metamodelica::AsArg::as_arg(&chars), linfo.clone(), lesc.clone(), resc.clone())?;
                    Ok((chars.clone(), linfo.clone(), exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "{", tail: startChars }, startLInfo, _, _) => {
                    let mut chars: metamodelica::List<ArcStr>;
                    let mut linfo: LineInfo;
                    let mut sinfo: SourceInfo;
                    (chars, linfo) = interleave(startChars.clone(), startLInfo.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ "}", tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    chars = metamodelica::Own::own(__pa0);
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), metamodelica::AsArg::as_arg(&startLInfo), 1)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (metamodelica::Ref::new(TplAbsyn::ExpressionBase::MAP_ARG_LIST { parts: metamodelica::nil() }), sinfo.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "{", tail: startChars }, startLInfo, lesc, resc) => {
                    let mut chars: metamodelica::List<ArcStr>;
                    let mut linfo: LineInfo;
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut expLst: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>;
                    let mut sinfo: SourceInfo;
                    (chars, linfo) = interleave(startChars.clone(), startLInfo.clone());
                    (chars, linfo, exp) = expressionPlus(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, expLst) = expressionList_rest(&chars, linfo.clone(), lesc.clone(), resc.clone());
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!("}"))?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), metamodelica::AsArg::as_arg(&startLInfo), 1)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (metamodelica::Ref::new(TplAbsyn::ExpressionBase::MAP_ARG_LIST { parts: metamodelica::cons(exp.clone(), expLst.clone()) }), sinfo.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "(", tail: startChars }, startLInfo, lesc, resc) => {
                    let mut chars: metamodelica::List<ArcStr>;
                    let mut linfo: LineInfo;
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    (chars, linfo) = interleave(startChars.clone(), startLInfo.clone());
                    (chars, linfo, exp) = expression(chars.clone(), linfo.clone(), lesc.clone(), resc.clone(), false)?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!(")"))?;
                    Ok((chars.clone(), linfo.clone(), exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "&", tail: startChars }, startLInfo, _, _) => {
                    let mut chars: metamodelica::List<ArcStr>;
                    let mut linfo: LineInfo;
                    let mut id: ArcStr;
                    let mut sinfo: SourceInfo;
                    (chars, linfo) = interleave(startChars.clone(), startLInfo.clone());
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), metamodelica::AsArg::as_arg(&startLInfo), 1)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (metamodelica::Ref::new(TplAbsyn::ExpressionBase::BOUND_VALUE { boundPath: metamodelica::Ref::new(TplAbsyn::PathIdent::IDENT { ident: id.clone() }) }), sinfo.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (startChars, startLInfo, lesc, resc) => {
                    let mut chars: metamodelica::List<ArcStr>;
                    let mut linfo: LineInfo;
                    let mut name: metamodelica::Ref<TplAbsyn::PathIdent>;
                    let mut expB: metamodelica::Ref<TplAbsyn::ExpressionBase>;
                    let mut sinfo: SourceInfo;
                    (chars, linfo, name) = pathIdent(startChars.clone(), startLInfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, expB) = boundValueOrFunCall(chars.clone(), linfo.clone(), name.clone(), lesc.clone(), resc.clone());
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), metamodelica::AsArg::as_arg(&startLInfo), 0)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (expB.clone(), sinfo.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outExpression))
}

/*
boundValueOrFunCall(name,lesc,resc):
  funCall(name,lesc,resc):exp  => exp
  |
  _ => BOUND_VALUE(name)
*/
pub(crate) fn boundValueOrFunCall(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inName: metamodelica::Ref<TplAbsyn::PathIdent>,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::Ref<TplAbsyn::ExpressionBase>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outExpressionBase: metamodelica::Ref<TplAbsyn::ExpressionBase>;
    (outChars, outLineInfo, outExpressionBase) = 'mc: {
        let __mc_input = (
            inChars.clone(),
            inLineInfo.clone(),
            inName.clone(),
            inLeftEsc,
            inRightEsc,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, name, lesc, resc) => {
                    let mut expB: metamodelica::Ref<TplAbsyn::ExpressionBase>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, expB) = funCall(metamodelica::AsArg::as_arg(&chars), linfo.clone(), name.clone(), lesc.clone(), resc.clone())?;
                    Ok((chars.clone(), linfo.clone(), expB.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::Ref::new(TplAbsyn::ExpressionBase::BOUND_VALUE { boundPath: inName.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outExpressionBase)
}

/*
//may fail
funCall(name,lesc,resc):
  '(' ')' => FUN_CALL(name,{})
  |
  '(' expression(lesc,resc):exp  expressionList_rest(lesc,resc):expLst ')'  //template  or  intrinsic function
    => FUN_CALL(name,exp::expLst)
*/
pub(crate) fn funCall(
    mut inChars: &metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inName: metamodelica::Ref<TplAbsyn::PathIdent>,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::Ref<TplAbsyn::ExpressionBase>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outExpressionBase: metamodelica::Ref<TplAbsyn::ExpressionBase>;
    (outChars, outLineInfo, outExpressionBase) = 'mc: {
        let __mc_input = (&**inChars, inLineInfo, inName, inLeftEsc, inRightEsc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "(", tail: chars }, linfo, name, _, _) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ ")", tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    chars = metamodelica::Own::own(__pa0);
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::ExpressionBase::FUN_CALL { name: name.clone(), args: metamodelica::nil() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "(", tail: chars }, linfo, name, lesc, resc) => {
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut expLst: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, exp) = expressionPlus(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, expLst) = expressionList_rest(metamodelica::AsArg::as_arg(&chars), linfo.clone(), lesc.clone(), resc.clone());
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!(")"))?;
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::ExpressionBase::FUN_CALL { name: name.clone(), args: metamodelica::cons(exp.clone(), expLst.clone()) })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outExpressionBase))
}

/*
expressionList_rest(lesc,resc):
  ',' expressionPlus(lesc,resc):exp  expressionList_rest(lesc,resc):expLst => exp::expLst
  |
  _ => {}
*/
pub(crate) fn expressionList_rest(
    mut inChars: &metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outExpressionList: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>;
    (outChars, outLineInfo, outExpressionList) = 'mc: {
        let __mc_input = (&**inChars, inLineInfo.clone(), inLeftEsc, inRightEsc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ ",", tail: chars }, linfo, lesc, resc) => {
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut expLst: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, exp) = expressionPlus(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, expLst) = expressionList_rest(metamodelica::AsArg::as_arg(&chars), linfo.clone(), lesc.clone(), resc.clone());
                    Ok((chars.clone(), linfo.clone(), metamodelica::cons(exp.clone(), expLst.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outExpressionList)
}

/*
stringConstant:
  '"' doubleQuoteConst({},{}):stRevLst
    => stRevLst
  |
  //'%'(lquot) stripFirstNewLine verbatimConst(Rquote(lquot),{},{}):stRevLst
  //  => stRevLst
  //|
  '\\n' escUnquotedChars({}, {"\n"}):stRevLst
    => stRevLst
  |
  '\\' escChar:c  escUnquotedChars({c}, {}):stRevLst
    => stRevLst
*/
pub(crate) fn stringConstant(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(metamodelica::List<ArcStr>, LineInfo, metamodelica::List<ArcStr>)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outStrRevList: metamodelica::List<ArcStr>;
    (outChars, outLineInfo, outStrRevList) = 'mc: {
        let __mc_input = (inChars, inLineInfo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (startChars @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "\"", tail: chars }, startLinfo) => {
                    let mut stRevLst: metamodelica::List<ArcStr>;
                    let mut linfo: LineInfo;
                    let mut optError: Option<ArcStr>;
                    let mut chars = (*chars).clone();
                    (chars, linfo, stRevLst, optError) = doubleQuoteConst(chars.clone(), startLinfo.clone(), metamodelica::nil(), metamodelica::nil())?;
                    linfo = parseErrorPrevPositionOpt(startChars.clone(), startLinfo.clone(), linfo.clone(), optError.clone(), true)?;
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "\\", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: chars } }, linfo) => {
                    let mut stRevLst: metamodelica::List<ArcStr>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, stRevLst) = escUnquotedChars(chars.clone(), linfo.clone(), metamodelica::nil(), list![literal!("\n")]);
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "\\", tail: Deref @ metamodelica::ListNode::Cons { head: c, tail: chars } }, linfo) => {
                    let mut stRevLst: metamodelica::List<ArcStr>;
                    let mut c = (*c).clone();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    c = escChar(metamodelica::AsArg::as_arg(&c))?;
                    (chars, linfo, stRevLst) = escUnquotedChars(chars.clone(), linfo.clone(), list![c.clone()], metamodelica::nil());
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outStrRevList))
}

/*
//not optional, must not fail
literalConstant:
  //(+|-)?d*(.d+)?(('e'|'E')(+|-)?d+)?
  plusMinus:pm digits:ds dotNumber:(dn,ts) exponent(ts):(ex,ts)
  => (pm+ stringCharListString(ds)+dn+ex, ts)  //validate the number - must have integer part or dotpart
  |
  'true' => ("true", BOOLEAN_TYPE())
  |
  'false' => ("false", BOOLEAN_TYPE())
*/
pub(crate) fn literalConstant(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    ArcStr,
    metamodelica::Ref<TplAbsyn::TypeSignature>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outConstantValue: ArcStr;
    let mut outConstantType: metamodelica::Ref<TplAbsyn::TypeSignature>;
    (outChars, outLineInfo, outConstantValue, outConstantType) = 'mc: {
        let __mc_input = (inChars, inLineInfo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut ds: metamodelica::List<ArcStr>;
                    let mut pm: ArcStr;
                    let mut dn: ArcStr;
                    let mut ex: ArcStr;
                    let mut num: ArcStr;
                    let mut ts: metamodelica::Ref<TplAbsyn::TypeSignature>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, pm) = plusMinus(chars.clone());
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, ds) = digits(chars.clone());
                    (chars, dn, ts) = dotNumber(chars.clone());
                    num = { let mut __mm_s = String::new(); __mm_s.push_str(&*stringCharListString(ds.clone())); __mm_s.push_str(&*dn); ArcStr::from(__mm_s) };
                    let true = (((num).len() as i32) > 0) else { return Err("pattern mismatch") };
                    (chars, ex, ts) = exponent(chars.clone(), ts.clone());
                    num = { let mut __mm_s = String::new(); __mm_s.push_str(&*pm); __mm_s.push_str(&*num); __mm_s.push_str(&*ex); ArcStr::from(__mm_s) };
                    Ok((chars.clone(), linfo.clone(), num.clone(), ts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "r", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "u", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: chars } } } }, linfo) => {
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    Ok((chars.clone(), linfo.clone(), literal!("true"), crate::TplAbsyn::TypeSignature::interned_BOOLEAN_TYPE()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "f", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "l", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "s", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: chars } } } } }, linfo) => {
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    Ok((chars.clone(), linfo.clone(), literal!("false"), crate::TplAbsyn::TypeSignature::interned_BOOLEAN_TYPE()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outConstantValue, outConstantType))
}

pub(crate) fn stripFirstNewLine(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> (metamodelica::List<ArcStr>, LineInfo) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = newLine(metamodelica::AsArg::as_arg(&chars), metamodelica::AsArg::as_arg(&linfo))?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo)
}

pub(crate) fn rightVerbatimConstQuote(mut inLeftQuote: ArcStr) -> ArcStr {
    let mut outRightQuote: ArcStr;
    outRightQuote = (::match_deref::match_deref! { match &(inLeftQuote.clone()) {
        Deref @ "(" => literal!(")"),
        Deref @ "{" => literal!("}"),
        Deref @ "<" => literal!(">"),
        Deref @ "[" => literal!("]"),
        _ => inLeftQuote,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outRightQuote
}

/*
doubleQuoteConst(accChars,accStrList):
  '"' => stringCharListString(listReverse(accChars)) :: accStrList
  |
  newLine doubleQuoteConst({}, stringCharListString(listReverse('\n'::accChars))::accStrList):stRevLst
  => stRevLst
  |
  '\\n' doubleQuoteConst({}, stringCharListString(listReverse('\n'::accChars))::accStrList):stRevLst
  => stRevLst
  |
  '\\'escChar:c doubleQuoteConst(c::accChars,accStrList):stRevLst
  => stRevLst
  |
  c doubleQuoteConst(c::accChars,accStrList):stRevLst
  => stRevLst
  |
  Error end of file
*/
pub(crate) fn doubleQuoteConst(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inAccChars: metamodelica::List<ArcStr>,
    mut inAccStrList: metamodelica::List<ArcStr>,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<ArcStr>,
    Option<ArcStr>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outStrRevList: metamodelica::List<ArcStr>;
    let mut outError: Option<ArcStr>;
    (outChars, outLineInfo, outStrRevList, outError) = 'mc: {
        let __mc_input = (inChars, inLineInfo, inAccChars, inAccStrList);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "\"", tail: chars }, linfo, accChars, accStrList) => {
                    let mut r#str: ArcStr;
                    r#str = stringCharListString(accChars.clone().reverse());
                    Ok((chars.clone(), linfo.clone(), metamodelica::cons(r#str.clone(), accStrList.clone()), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "\\", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: chars } }, linfo, accChars, accStrList) => {
                    let mut stRevLst: metamodelica::List<ArcStr>;
                    let mut r#str: ArcStr;
                    let mut optError: Option<ArcStr>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    r#str = stringCharListString(metamodelica::cons(literal!("\n"), accChars.clone()).reverse());
                    (chars, linfo, stRevLst, optError) = doubleQuoteConst(chars.clone(), linfo.clone(), metamodelica::nil(), metamodelica::cons(r#str.clone(), accStrList.clone()))?;
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone(), optError.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "\\", tail: Deref @ metamodelica::ListNode::Cons { head: c, tail: chars } }, linfo, accChars, accStrList) => {
                    let mut stRevLst: metamodelica::List<ArcStr>;
                    let mut optError: Option<ArcStr>;
                    let mut c = (*c).clone();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    c = escChar(metamodelica::AsArg::as_arg(&c))?;
                    (chars, linfo, stRevLst, optError) = doubleQuoteConst(chars.clone(), linfo.clone(), metamodelica::cons(c.clone(), accChars.clone()), accStrList.clone())?;
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone(), optError.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, accChars, accStrList) => {
                    let mut stRevLst: metamodelica::List<ArcStr>;
                    let mut r#str: ArcStr;
                    let mut optError: Option<ArcStr>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = newLine(metamodelica::AsArg::as_arg(&chars), metamodelica::AsArg::as_arg(&linfo))?;
                    r#str = stringCharListString(metamodelica::cons(literal!("\n"), accChars.clone()).reverse());
                    (chars, linfo, stRevLst, optError) = doubleQuoteConst(chars.clone(), linfo.clone(), metamodelica::nil(), metamodelica::cons(r#str.clone(), accStrList.clone()))?;
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone(), optError.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars @ Deref @ metamodelica::ListNode::Cons { head: c, tail: restChars }, linfo, accChars, accStrList) => {
                    let mut stRevLst: metamodelica::List<ArcStr>;
                    let mut optError: Option<ArcStr>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    if '__try0: {
                        unwrap_break_err!(newLine(metamodelica::AsArg::as_arg(&chars), metamodelica::AsArg::as_arg(&linfo)), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    (chars, linfo, stRevLst, optError) = doubleQuoteConst(restChars.clone(), linfo.clone(), metamodelica::cons(c.clone(), accChars.clone()), accStrList.clone())?;
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone(), optError.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, linfo, accChars, accStrList) => {
                    let mut r#str: ArcStr;
                    let mut errStr: ArcStr;
                    r#str = stringCharListString(accChars.clone().reverse());
                    errStr = literal!("Unmatched \" \" quotes for a string constant - reached end of file.");
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Parse error - TplParser.doubleQuoteConst - ")); __mm_s.push_str(&*errStr); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    }
                    Ok((metamodelica::nil(), linfo.clone(), metamodelica::cons(r#str.clone(), accStrList.clone()), Some(errStr.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outStrRevList, outError))
}

/*
escChar:
  ( '\'' | '"' | '?' |  '\\' | 'a' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' | ' ' )
  => the escaped char

*/
pub(crate) fn escChar(mut inEscChar: &ArcStr) -> Result<ArcStr> {
    let mut outTheChar: ArcStr;
    outTheChar = (::match_deref::match_deref! { match &(inEscChar.clone()) {
        Deref @ "'" => literal!("'"),
        Deref @ "\"" => literal!("\""),
        Deref @ "?" => literal!("?"),
        Deref @ "\\" => literal!("\\"),
        Deref @ "n" => literal!("\n"),
        Deref @ "t" => literal!("\t"),
        Deref @ " " => literal!(" "),
        _ => return Err("match: no arm matched"),
    } });
    Ok(outTheChar)
}

/*
verbatimConst(rquot, accChars, accStrList):
  //strip a last inline new line
  newLine (rquot)'%' =>  stringCharListString(listReverse(accChars)) :: accStrList
  |
  (rquot)'%' =>  stringCharListString(listReverse(accChars)) :: accStrList
  |
  newLine verbatimConst(rquot, {}, stringCharListString(listReverse('\n'::accChars))::accStrList):stRevLst
    => stRevLst
  |
  c  verbatimConst(rquot, c::accChars,accStrList):stRevLst
    => stRevLst
  |
  Error end of file
*/
pub(crate) fn verbatimConst(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inRightQuote: ArcStr,
    mut inAccChars: metamodelica::List<ArcStr>,
    mut inAccStrList: metamodelica::List<ArcStr>,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<ArcStr>,
    Option<ArcStr>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outStrRevList: metamodelica::List<ArcStr>;
    let mut outError: Option<ArcStr>;
    (outChars, outLineInfo, outStrRevList, outError) = 'mc: {
        let __mc_input = (inChars, inLineInfo, inRightQuote, inAccChars, inAccStrList);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, rquot, accChars, accStrList) => {
                    let mut c: ArcStr;
                    let mut r#str: ArcStr;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = newLine(metamodelica::AsArg::as_arg(&chars), metamodelica::AsArg::as_arg(&linfo))?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "%", tail: __pa1 } } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    c = metamodelica::Own::own(__pa0);
                    chars = metamodelica::Own::own(__pa1);
                    let true = (stringEq(&c, &rquot)) else { return Err("pattern mismatch") };
                    r#str = stringCharListString(accChars.clone().reverse());
                    Ok((chars.clone(), linfo.clone(), metamodelica::cons(r#str.clone(), accStrList.clone()), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: c, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "%", tail: chars } }, linfo, rquot, accChars, accStrList) => {
                    let mut r#str: ArcStr;
                    let true = (stringEq(&c, &rquot)) else { return Err("pattern mismatch") };
                    r#str = stringCharListString(accChars.clone().reverse());
                    Ok((chars.clone(), linfo.clone(), metamodelica::cons(r#str.clone(), accStrList.clone()), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, rquot, accChars, accStrList) => {
                    let mut stRevLst: metamodelica::List<ArcStr>;
                    let mut r#str: ArcStr;
                    let mut optError: Option<ArcStr>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = newLine(metamodelica::AsArg::as_arg(&chars), metamodelica::AsArg::as_arg(&linfo))?;
                    r#str = stringCharListString(metamodelica::cons(literal!("\n"), accChars.clone()).reverse());
                    (chars, linfo, stRevLst, optError) = verbatimConst(chars.clone(), linfo.clone(), rquot.clone(), metamodelica::nil(), metamodelica::cons(r#str.clone(), accStrList.clone()))?;
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone(), optError.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars @ Deref @ metamodelica::ListNode::Cons { head: c, tail: restChars }, linfo, rquot, accChars, accStrList) => {
                    let mut stRevLst: metamodelica::List<ArcStr>;
                    let mut optError: Option<ArcStr>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    if '__try0: {
                        unwrap_break_err!(newLine(metamodelica::AsArg::as_arg(&chars), metamodelica::AsArg::as_arg(&linfo)), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    (chars, linfo, stRevLst, optError) = verbatimConst(restChars.clone(), linfo.clone(), rquot.clone(), metamodelica::cons(c.clone(), accChars.clone()), accStrList.clone())?;
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone(), optError.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, linfo, rquot, accChars, accStrList) => {
                    let mut r#str: ArcStr;
                    let mut errStr: ArcStr;
                    r#str = stringCharListString(accChars.clone().reverse());
                    errStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unmatched %")); __mm_s.push_str(&*rquot); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*rquot); __mm_s.push_str(&*literal!("% quotes for a verbatim string constant - reached end of file.")); ArcStr::from(__mm_s) };
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Parse error - TplParser.verbatimConst - ")); __mm_s.push_str(&*errStr); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    Ok((metamodelica::nil(), linfo.clone(), metamodelica::cons(r#str.clone(), accStrList.clone()), Some(errStr.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outStrRevList, outError))
}

/*
escUnquotedChars(accChars,accStrList):
  '\\n' escUnquotedChars({}, stringCharListString(listReverse('\n'::accChars)) :: accStrList):stRevLst
  => stRevLst
  |
  '\\' escChar:c  escUnquotedChars(c::accChars, accStrList):stRevLst
  => stRevLst
  |
  _ => stringCharListString(listReverse(accChars)) :: accStrList

*/
pub(crate) fn escUnquotedChars(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inAccChars: metamodelica::List<ArcStr>,
    mut inAccStrList: metamodelica::List<ArcStr>,
) -> (metamodelica::List<ArcStr>, LineInfo, metamodelica::List<ArcStr>) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outStrRevList: metamodelica::List<ArcStr>;
    (outChars, outLineInfo, outStrRevList) = 'mc: {
        let __mc_input = (inChars, inLineInfo, inAccChars, inAccStrList);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "\\", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: chars } }, linfo, accChars, accStrList) => {
                    let mut stRevLst: metamodelica::List<ArcStr>;
                    let mut r#str: ArcStr;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    r#str = stringCharListString(metamodelica::cons(literal!("\n"), accChars.clone()).reverse());
                    (chars, linfo, stRevLst) = escUnquotedChars(chars.clone(), linfo.clone(), metamodelica::nil(), metamodelica::cons(r#str.clone(), accStrList.clone()));
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "\\", tail: Deref @ metamodelica::ListNode::Cons { head: c, tail: chars } }, linfo, accChars, accStrList) => {
                    let mut stRevLst: metamodelica::List<ArcStr>;
                    let mut c = (*c).clone();
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    c = escChar(metamodelica::AsArg::as_arg(&c))?;
                    (chars, linfo, stRevLst) = escUnquotedChars(chars.clone(), linfo.clone(), metamodelica::cons(c.clone(), accChars.clone()), accStrList.clone());
                    Ok((chars.clone(), linfo.clone(), stRevLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, accChars, accStrList) => {
                    let mut r#str: ArcStr;
                    r#str = stringCharListString(accChars.clone().reverse());
                    Ok((chars.clone(), linfo.clone(), metamodelica::cons(r#str.clone(), accStrList.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outStrRevList)
}

pub(crate) fn makeStrTokFromRevStrList(
    mut inRevStrList: metamodelica::List<ArcStr>,
) -> Result<metamodelica::Ref<Tpl::StringToken>> {
    let mut outStringToken: metamodelica::Ref<Tpl::StringToken>;
    outStringToken = (::match_deref::match_deref! { match &(inRevStrList) {
        Deref @ metamodelica::ListNode::Cons { head: r#str, tail: Deref @ metamodelica::ListNode::Nil } => {
            metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: r#str.clone() })
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ "", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "\n", tail: Deref @ metamodelica::ListNode::Nil } } => {
            openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ "", tail: Deref @ metamodelica::ListNode::Cons { head: r#str, tail: Deref @ metamodelica::ListNode::Nil } } => {
            metamodelica::Ref::new(Tpl::StringToken::ST_LINE { line: r#str.clone() })
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ "", tail: strList } => {
            let mut strList = (*strList).clone();
            strList = strList.clone().reverse();
            metamodelica::Ref::new(Tpl::StringToken::ST_STRING_LIST { strList: strList.clone(), lastHasNewLine: true })
        },
        strList @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => {
            let mut strList = (*strList).clone();
            strList = strList.clone().reverse();
            metamodelica::Ref::new(Tpl::StringToken::ST_STRING_LIST { strList: strList.clone(), lastHasNewLine: false })
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::trace(literal!("Parse invalid operation error - TplParser.makeStrTokFromRevStrList failed (an empty string list passed?) .\n"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outStringToken)
}

/*
plusMinus:
  '+' => "+"
  |
  '-' => "-"
  |
  _ => ""
*/
pub(crate) fn plusMinus(mut inChars: metamodelica::List<ArcStr>) -> (metamodelica::List<ArcStr>, ArcStr) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outSign: ArcStr;
    (outChars, outSign) = (::match_deref::match_deref! { match &(inChars.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: char, tail: chars } if (metamodelica::stringEq(&char, &(literal!("+"))) || metamodelica::stringEq(&char, &(literal!("-")))) => {
            (chars.clone(), char.clone())
        },
        _ => {
            (inChars, literal!(""))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outChars, outSign)
}

/*
digits:
  [0-9]:d  digits:ds => d::ds
  |
  _ => {}
*/
pub(crate) fn digits(
    mut inChars: metamodelica::List<ArcStr>,
) -> (metamodelica::List<ArcStr>, metamodelica::List<ArcStr>) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outDigits: metamodelica::List<ArcStr>;
    (outChars, outDigits) = 'mc: {
        let __mc_input = inChars;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: d, tail: chars } => {
                    let mut ds: metamodelica::List<ArcStr>;
                    let mut i: i32;
                    let mut chars = (*chars).clone();
                    i = stringCharInt(d.clone())?;
                    let true = (48 <= i && i <= 57) else { return Err("pattern mismatch") };
                    (chars, ds) = digits(chars.clone());
                    Ok((chars.clone(), metamodelica::cons(d.clone(), ds.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                chars => {
                    Ok((chars.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outDigits)
}

/*
dotNumber:
  '.' digits:ds  =>  (stringCharListString(ds), REAL_TYPE())
  |
  _ => INTEGER_TYPE()
*/
pub(crate) fn dotNumber(
    mut inChars: metamodelica::List<ArcStr>,
) -> (
    metamodelica::List<ArcStr>,
    ArcStr,
    metamodelica::Ref<TplAbsyn::TypeSignature>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outDotNumber: ArcStr;
    let mut outLitType: metamodelica::Ref<TplAbsyn::TypeSignature>;
    (outChars, outDotNumber, outLitType) = 'mc: {
        let __mc_input = &*inChars;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ ".", tail: chars } => {
                    let mut dn: ArcStr;
                    let mut ds: metamodelica::List<ArcStr>;
                    let mut chars = (*chars).clone();
                    (chars, ds) = digits(chars.clone());
                    ::match_deref::match_deref! { match &(ds.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    dn = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*stringCharListString(ds.clone())); ArcStr::from(__mm_s) };
                    Ok((chars.clone(), dn.clone(), crate::TplAbsyn::TypeSignature::interned_REAL_TYPE()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), literal!(""), crate::TplAbsyn::TypeSignature::interned_INTEGER_TYPE()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outDotNumber, outLitType)
}

/*
exponent(typ):
  'e' plusMinus:pm  digits:ds => ("e"+pm+stringCharListString(ds), REAL_TYPE())
  |
  'E' plusMinus:pm  digits:ds => ("E"+pm+stringCharListString(ds), REAL_TYPE())
  |
  => ("",typ)
*/
pub(crate) fn exponent(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLitType: metamodelica::Ref<TplAbsyn::TypeSignature>,
) -> (
    metamodelica::List<ArcStr>,
    ArcStr,
    metamodelica::Ref<TplAbsyn::TypeSignature>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outExponent: ArcStr;
    let mut outLitType: metamodelica::Ref<TplAbsyn::TypeSignature>;
    (outChars, outExponent, outLitType) = 'mc: {
        let __mc_input = &*inChars;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: chars } => {
                    let mut ex: ArcStr;
                    let mut pm: ArcStr;
                    let mut ds: metamodelica::List<ArcStr>;
                    let mut chars = (*chars).clone();
                    (chars, pm) = plusMinus(chars.clone());
                    (chars, ds) = digits(chars.clone());
                    ::match_deref::match_deref! { match &(ds.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    ex = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("e")); __mm_s.push_str(&*pm); __mm_s.push_str(&*stringCharListString(ds.clone())); ArcStr::from(__mm_s) };
                    Ok((chars.clone(), ex.clone(), crate::TplAbsyn::TypeSignature::interned_REAL_TYPE()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ "E", tail: chars } => {
                    let mut ex: ArcStr;
                    let mut pm: ArcStr;
                    let mut ds: metamodelica::List<ArcStr>;
                    let mut chars = (*chars).clone();
                    (chars, pm) = plusMinus(chars.clone());
                    (chars, ds) = digits(chars.clone());
                    ::match_deref::match_deref! { match &(ds.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    ex = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("E")); __mm_s.push_str(&*pm); __mm_s.push_str(&*stringCharListString(ds.clone())); ArcStr::from(__mm_s) };
                    Ok((chars.clone(), ex.clone(), crate::TplAbsyn::TypeSignature::interned_REAL_TYPE()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), literal!(""), inLitType.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outExponent, outLitType)
}

/*
templateExp(lesc, resc):
  "'" stripFirstNewLine  templateBody(lesc, resc, isSingleQuote = true, {},{},0)
  |
  '<<' stripFirstNewLine templateBody(lesc, resc, isSingleQuote = false,{},{},0 )
*/
pub(crate) fn templateExp(
    mut inChars: &metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outExpression: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = 'mc: {
        let __mc_input = (&**inChars, inLineInfo, inLeftEsc, inRightEsc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "'", tail: startChars }, startLInfo, lesc, resc) => {
                    let mut chars: metamodelica::List<ArcStr>;
                    let mut linfo: LineInfo;
                    let mut expB: metamodelica::Ref<TplAbsyn::ExpressionBase>;
                    let mut sinfo: SourceInfo;
                    (chars, linfo, expB) = templateBody(startChars.clone(), startLInfo.clone(), lesc.clone(), resc.clone(), true, metamodelica::nil(), metamodelica::nil(), 0)?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), metamodelica::AsArg::as_arg(&startLInfo), 1)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (expB.clone(), sinfo.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "<", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "<", tail: startChars } }, startLInfo @ LineInfo { startOfLineChars: solChars, .. }, lesc, resc) => {
                    let mut chars: metamodelica::List<ArcStr>;
                    let mut linfo: LineInfo;
                    let mut expB: metamodelica::Ref<TplAbsyn::ExpressionBase>;
                    let mut baseInd: i32;
                    let mut sinfo: SourceInfo;
                    (_, baseInd) = lineIndent(solChars.clone(), 0);
                    (chars, linfo) = takeSpaceAndNewLine(startChars.clone(), startLInfo.clone())?;
                    (chars, linfo, expB) = templateBody(chars.clone(), linfo.clone(), lesc.clone(), resc.clone(), false, metamodelica::nil(), metamodelica::nil(), baseInd)?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), metamodelica::AsArg::as_arg(&startLInfo), 2)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (expB.clone(), sinfo.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "<", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "<", tail: startChars } }, startLInfo @ LineInfo { startOfLineChars: solChars, .. }, lesc, resc) => {
                    let mut chars: metamodelica::List<ArcStr>;
                    let mut linfo: LineInfo;
                    let mut expB: metamodelica::Ref<TplAbsyn::ExpressionBase>;
                    let mut baseInd: i32;
                    let mut lineInd: i32;
                    let mut sinfo: SourceInfo;
                    (_, baseInd) = lineIndent(solChars.clone(), 0);
                    if '__try0: {
                        unwrap_break_err!(takeSpaceAndNewLine(startChars.clone(), startLInfo.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    (chars, lineInd) = lineIndent(startChars.clone(), 0);
                    lineInd = lineInd + baseInd;
                    (chars, linfo, expB) = restOfTemplLine(chars.clone(), startLInfo.clone(), lesc.clone(), resc.clone(), false, metamodelica::nil(), metamodelica::nil(), baseInd, lineInd)?;
                    sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), metamodelica::AsArg::as_arg(&startLInfo), 2)?, chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (expB.clone(), sinfo.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outExpression))
}

/*
//optional, may fail
takeSpaceAndNewLine:
  newLine
  |
  ' ' takeSpaceAndNewLine
  |
  '\t' takeSpaceAndNewLine
*/
pub(crate) fn takeSpaceAndNewLine(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(metamodelica::List<ArcStr>, LineInfo)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (inChars, inLineInfo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = newLine(metamodelica::AsArg::as_arg(&chars), metamodelica::AsArg::as_arg(&linfo))?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: char, tail: chars }, linfo) => {
                    if !((metamodelica::stringEq(&char, &(literal!(" "))) || metamodelica::stringEq(&char, &(literal!("\t"))))) { return Err("guard") }
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = takeSpaceAndNewLine(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo))
}

/*
templateBody(lesc, resc, isSingleQuote, expList, indStack, actInd):
  lineIndent(0):lineInd
    restOfTemplLine(lesc, resc, isSingleQuote, expList, indStack, actInd, lineInd, {}):exp
  => exp
*/
pub(crate) fn templateBody(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
    mut inIsSingleQuote: bool,
    mut inExpressionList: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    mut inIndentStack: metamodelica::List<(
        i32,
        metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )>,
    mut inActualIndent: i32,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::Ref<TplAbsyn::ExpressionBase>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outExpressionBase: metamodelica::Ref<TplAbsyn::ExpressionBase>;
    let mut lindent: i32;
    (outChars, lindent) = lineIndent(inChars, 0);
    (outChars, outLineInfo, outExpressionBase) = restOfTemplLine(
        outChars,
        inLineInfo,
        inLeftEsc,
        inRightEsc,
        inIsSingleQuote,
        inExpressionList,
        inIndentStack,
        inActualIndent,
        lindent,
    )?;
    Ok((outChars, outLineInfo, outExpressionBase))
}

/*
lineIndent(ind):
  ' ' lineIndent(ind+1):n  =>  n
  |
  '\t' lineIndent(ind+4):n  =>  n
  |
  _  =>  ind

*/
pub(crate) fn lineIndent(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineIndent: i32,
) -> (metamodelica::List<ArcStr>, i32) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineIndent: i32;
    (outChars, outLineIndent) = (::match_deref::match_deref! { match &(inChars.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ " ", tail: __esc_outChars } => {
            outChars = (*__esc_outChars).clone();
            lineIndent(outChars.clone(), inLineIndent + 1)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ "\t", tail: __esc_outChars } => {
            outChars = (*__esc_outChars).clone();
            lineIndent(outChars.clone(), inLineIndent + TabSpaces.clone())
        },
        _ => (inChars, inLineIndent),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outChars, outLineIndent)
}

/*
// & ... no interleave
restOfTemplLine(lesc, resc, isSingleQuote, expList, indStack, actInd, lineInd, accStrChars):
  //(lesc)'#' nonTemplateExprWithOpts(lesc,resc):eexp  '#'(resc)
  //   { (expList, indStack, actInd) = onEscapedExp(eexp, expList, indStack, actInd, lineInd, accStrChars) }
  //   & restOfTemplLine(lesc,resc,isSingleQuote, expList, indStack, actInd, actInd, {}):exp
  //   => exp
  //
  //|
  (lesc)  (resc)  // a comment | empty expression ... ignore completely
     & restOfTemplLineAfterEmptyExp(lesc,resc,isSingleQuote, expList, indStack, actInd, lineInd, accStrChars):exp
     => exp
  |
  (lesc) '%' expression(lesc,resc):eexp (resc)
     { (expList, indStack, actInd) = onEscapedExp(eexp, expList, indStack, actInd, lineInd, accStrChars) }
     & restOfTemplLine(lesc,resc,isSingleQuote, expList, indStack, actInd, actInd, {}):exp
     => exp

  | // on \n
  newLine
   { (expList, indStack, actInd) = onNewLine(expList, indStack, actInd, lineInd, accStrChars) }
   & templateBody(lesc, resc, isSingleQuote, expList, indStack, actInd):exp
  => exp

  | //end
  (isSingleQuote = true) "'"
   =>
    onTemplEnd(expList, indStack, actInd, lineInd, accStrChars)

  | //end
  (isSingleQuote = false) '>>'
   =>
   onTemplEnd(expList, indStack, actInd, lineInd, accStrChars)

  |
  '\' & ( '\' | "'" | (lesc) | (resc) ):c
   & restOfTemplLine(lesc, resc, isSingleQuote, expList, indStack, actInd, lineInd, c :: accStrChars) : exp
    => exp
  |
  any:c
    & restOfTemplLine(lesc, resc, isSingleQuote, expList, indStack, actInd, lineInd, c :: accStrChars) : exp
    => exp
*/
pub(crate) fn restOfTemplLine(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
    mut inIsSingleQuote: bool,
    mut inExpressionList: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    mut inIndentStack: metamodelica::List<(
        i32,
        metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )>,
    mut inActualIndent: i32,
    mut inLineIndent: i32,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::Ref<TplAbsyn::ExpressionBase>,
)> {
    let mut outChars: metamodelica::List<ArcStr> = inChars;
    let mut outLineInfo: LineInfo = inLineInfo.clone();
    let mut outExpressionBase: metamodelica::Ref<TplAbsyn::ExpressionBase>;
    let mut expl: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)> = inExpressionList;
    let mut lindent: i32 = inLineIndent;
    let mut aindent: i32 = inActualIndent;
    let mut ind_stack: metamodelica::List<(
        i32,
        metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )> = inIndentStack;
    let mut char: ArcStr;
    let mut next_char: ArcStr;
    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
    let mut chars: metamodelica::List<ArcStr>;
    let mut acc_chars: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut err_opt: Option<ArcStr>;
    let mut linfo: LineInfo;
    if '__try0: {
        loop {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(outChars.clone()) {
                Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            char = metamodelica::Own::own(__pa1);
            outChars = metamodelica::Own::own(__pa2);
            if inIsSingleQuote && metamodelica::stringEq(&char, &(literal!("'"))) {
                expl = unwrap_break_err!(onTemplEnd(false, expl.clone(), ind_stack.clone(), aindent, lindent, acc_chars.clone()), '__try0);
                outExpressionBase = makeTemplateFromExpList(expl.clone(), literal!("'"), literal!("'"));
                return Ok((outChars, outLineInfo, outExpressionBase));
            }
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(outChars.clone()) {
                Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            next_char = metamodelica::Own::own(__pa3);
            chars = metamodelica::Own::own(__pa4);
            if !(inIsSingleQuote) && metamodelica::stringEq(&char, &(literal!(">"))) && metamodelica::stringEq(&next_char, &(literal!(">"))) {
                expl = unwrap_break_err!(onTemplEnd(true, expl.clone(), ind_stack.clone(), aindent, lindent, acc_chars.clone()), '__try0);
                outExpressionBase = makeTemplateFromExpList(expl.clone(), literal!("<<"), literal!(">>"));
                outChars = chars.clone();
                return Ok((outChars, outLineInfo, outExpressionBase));
            } else if metamodelica::stringEq(&char, &(literal!("\r"))) || metamodelica::stringEq(&char, &(literal!("\n"))) {
                (outChars, linfo) = unwrap_break_err!(newLine(&(metamodelica::cons(char.clone(), outChars.clone())), &outLineInfo), '__try0);
                (expl, ind_stack, aindent, err_opt) = unwrap_break_err!(onNewLine(expl.clone(), ind_stack.clone(), aindent, lindent, acc_chars.clone()), '__try0);
                outLineInfo = unwrap_break_err!(parseErrorPrevPositionOptInfoChars(outLineInfo.clone(), linfo.clone(), err_opt.clone(), false), '__try0);
                (outChars, lindent) = lineIndent(outChars.clone(), 0);
                acc_chars = metamodelica::nil();
            } else if metamodelica::stringEq(&char, &inLeftEsc) && metamodelica::stringEq(&next_char, &(literal!("%"))) {
                (outChars, linfo) = interleave(chars.clone(), outLineInfo.clone());
                let (__pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(outChars.clone()) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: __pa7 } } => (__pa5.clone(), __pa6.clone(), __pa7.clone()),
                    _ => break '__try0 Err::<_, _>("pattern mismatch"),
                } };
                char = metamodelica::Own::own(__pa5);
                next_char = metamodelica::Own::own(__pa6);
                chars = metamodelica::Own::own(__pa7);
                if metamodelica::stringEq(&char, &(literal!("%"))) && metamodelica::stringEq(&next_char, &inRightEsc) {
                    (outChars, outLineInfo, lindent) = dropNewLineAfterEmptyExp(chars.clone(), linfo.clone(), lindent, &acc_chars);
                } else {
                    (outChars, linfo, exp) = unwrap_break_err!(expression(outChars.clone(), linfo.clone(), inLeftEsc.clone(), inRightEsc.clone(), false), '__try0);
                    (outChars, linfo) = unwrap_break_err!(interleaveExpectChar(outChars.clone(), linfo.clone(), literal!("%")), '__try0);
                    (outChars, linfo) = unwrap_break_err!(expectChar(outChars.clone(), linfo.clone(), &inRightEsc), '__try0);
                    (expl, ind_stack, aindent, err_opt) = unwrap_break_err!(onEscapedExp(exp.clone(), expl.clone(), ind_stack.clone(), aindent, lindent, acc_chars.clone()), '__try0);
                    outLineInfo = unwrap_break_err!(parseErrorPrevPositionOptInfoChars(outLineInfo.clone(), linfo.clone(), err_opt.clone(), false), '__try0);
                    acc_chars = metamodelica::nil();
                }
            } else {
                acc_chars = metamodelica::cons(char.clone(), acc_chars.clone());
            }
        }
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    outChars = metamodelica::nil();
    outLineInfo = parseError(
        metamodelica::nil(),
        inLineInfo,
        literal!("Not able to parse the text template expression from the point."),
        true,
    )?;
    outExpressionBase = crate::TplAbsyn::ExpressionBase::interned_ERROR_EXP();
    Ok((outChars, outLineInfo, outExpressionBase))
}

pub(crate) fn dropNewLineAfterEmptyExp(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLineIndent: i32,
    mut inAccStringChars: &metamodelica::List<ArcStr>,
) -> (metamodelica::List<ArcStr>, LineInfo, i32) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outLineIndent: i32;
    (outChars, outLineInfo, outLineIndent) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), &**inAccStringChars);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, Deref @ metamodelica::ListNode::Nil) => {
                    let mut lineInd: i32;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = takeSpaceAndNewLine(chars.clone(), linfo.clone())?;
                    (chars, lineInd) = lineIndent(chars.clone(), 0);
                    Ok((chars.clone(), linfo.clone(), lineInd))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), inLineIndent))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outLineIndent)
}

pub(crate) fn makeTemplateFromExpList(
    mut inExpressionList: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    mut inLeftQuote: ArcStr,
    mut inRightQuote: ArcStr,
) -> metamodelica::Ref<TplAbsyn::ExpressionBase> {
    let mut outExpressionBase: metamodelica::Ref<TplAbsyn::ExpressionBase>;
    outExpressionBase = (::match_deref::match_deref! { match &(inExpressionList) {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: literal!("") }) })
        },
        Deref @ metamodelica::ListNode::Cons { head: (expB, _), tail: Deref @ metamodelica::ListNode::Nil } => {
            expB.clone()
        },
        expLst => {
            let mut lquote = inLeftQuote;
            let mut rquote = inRightQuote;
            let mut expLst = (*expLst).clone();
            expLst = expLst.clone().reverse();
            metamodelica::Ref::new(TplAbsyn::ExpressionBase::TEMPLATE { items: expLst.clone(), lquote: lquote, rquote: rquote })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExpressionBase
}

pub(crate) fn onEscapedExp(
    mut inExpression: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
    mut inExpressionList: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    mut inIndentStack: metamodelica::List<(
        i32,
        metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )>,
    mut inActualIndent: i32,
    mut inLineIndent: i32,
    mut inAccStringChars: metamodelica::List<ArcStr>,
) -> Result<(
    metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    metamodelica::List<(
        i32,
        metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )>,
    i32,
    Option<ArcStr>,
)> {
    let mut outExpressionList: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>;
    let mut outIndentStack: metamodelica::List<(
        i32,
        metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )>;
    let mut outActualIndent: i32;
    let mut outError: Option<ArcStr>;
    (outExpressionList, outIndentStack, outActualIndent, outError) = 'mc: {
        let __mc_input = (
            inExpression,
            inExpressionList,
            inIndentStack,
            inActualIndent,
            inLineIndent,
            inAccStringChars,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, expLst, indStack, actInd, lineInd, accChars) => {
                    let mut expLst = (*expLst).clone();
                    let true = (intEq(lineInd.clone(), actInd.clone())) else { return Err("pattern mismatch") };
                    expLst = addAccStringChars(expLst.clone(), accChars.clone())?;
                    expLst = finalizeLastStringToken(metamodelica::AsArg::as_arg(&expLst));
                    expLst = metamodelica::cons(exp.clone(), expLst.clone());
                    Ok((expLst.clone(), indStack.clone(), actInd.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, expLst, indStack, actInd, lineInd, accChars) => {
                    let mut expLst = (*expLst).clone();
                    let mut indStack = (*indStack).clone();
                    let true = (lineInd.clone() > actInd.clone()) else { return Err("pattern mismatch") };
                    expLst = finalizeLastStringToken(metamodelica::AsArg::as_arg(&expLst));
                    indStack = metamodelica::cons((actInd.clone(), expLst.clone()), indStack.clone());
                    expLst = addAccStringChars(metamodelica::nil(), accChars.clone())?;
                    expLst = finalizeLastStringToken(metamodelica::AsArg::as_arg(&expLst));
                    expLst = metamodelica::cons(exp.clone(), expLst.clone());
                    Ok((expLst.clone(), indStack.clone(), lineInd.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, expLst, Deref @ metamodelica::ListNode::Nil, baseInd, lineInd, accChars) => {
                    let mut errStr: ArcStr;
                    let mut errOpt: Option<ArcStr>;
                    let mut actInd: i32;
                    let mut indStack: metamodelica::List<(i32, metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>)>;
                    let mut expLst = (*expLst).clone();
                    let true = (lineInd.clone() < baseInd.clone()) else { return Err("pattern mismatch") };
                    errStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Indent level is under the level of the '<<' determined level (by ")); __mm_s.push_str(&*intString(baseInd.clone() - lineInd.clone())); __mm_s.push_str(&*literal!(" chars).")); ArcStr::from(__mm_s) };
                    errOpt = Some(errStr.clone());
                    (expLst, indStack, actInd, _) = onEscapedExp(exp.clone(), expLst.clone(), metamodelica::nil(), baseInd.clone(), baseInd.clone(), accChars.clone())?;
                    Ok((expLst.clone(), indStack.clone(), actInd, errOpt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, expLst, indStack @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, actInd, lineInd, accChars) => {
                    let mut errOpt: Option<ArcStr>;
                    let mut expLst = (*expLst).clone();
                    let mut indStack = (*indStack).clone();
                    let mut actInd = (*actInd).clone();
                    let true = (lineInd.clone() < actInd.clone()) else { return Err("pattern mismatch") };
                    expLst = finalizeLastStringToken(metamodelica::AsArg::as_arg(&expLst));
                    (expLst, indStack, actInd) = popIndentStack(expLst.clone(), indStack.clone(), actInd.clone(), lineInd.clone())?;
                    (expLst, indStack, actInd, errOpt) = onEscapedExp(exp.clone(), expLst.clone(), indStack.clone(), actInd.clone(), lineInd.clone(), accChars.clone())?;
                    Ok((expLst.clone(), indStack.clone(), lineInd.clone(), errOpt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("Parse unexpected error - TplParser.onEscapedExp failed .\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExpressionList, outIndentStack, outActualIndent, outError))
}

pub(crate) fn onNewLine(
    mut inExpressionList: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    mut inIndentStack: metamodelica::List<(
        i32,
        metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )>,
    mut inActualIndent: i32,
    mut inLineIndent: i32,
    mut inAccStringChars: metamodelica::List<ArcStr>,
) -> Result<(
    metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    metamodelica::List<(
        i32,
        metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )>,
    i32,
    Option<ArcStr>,
)> {
    let mut outExpressionList: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>;
    let mut outIndentStack: metamodelica::List<(
        i32,
        metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )>;
    let mut outActualIndent: i32;
    let mut outError: Option<ArcStr>;
    (outExpressionList, outIndentStack, outActualIndent, outError) = 'mc: {
        let __mc_input = (
            inExpressionList,
            inIndentStack,
            inActualIndent,
            inLineIndent,
            inAccStringChars,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst, indStack, actInd, lineInd, Deref @ metamodelica::ListNode::Cons { head: c, tail: accChars }) => {
                    let mut errOpt: Option<ArcStr>;
                    let mut expLst = (*expLst).clone();
                    let mut indStack = (*indStack).clone();
                    let mut actInd = (*actInd).clone();
                    let true = (metamodelica::stringEq(&c, &(literal!(" "))) || metamodelica::stringEq(&c, &(literal!("\t")))) else { return Err("pattern mismatch") };
                    (expLst, indStack, actInd, errOpt) = onNewLine(expLst.clone(), indStack.clone(), actInd.clone(), lineInd.clone(), accChars.clone())?;
                    Ok((expLst.clone(), indStack.clone(), actInd.clone(), errOpt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, indStack, actInd, _, Deref @ metamodelica::ListNode::Nil) => {
                    let mut expLst: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>;
                    expLst = addAccStringChars(metamodelica::nil(), list![literal!("\n")])?;
                    Ok((expLst.clone(), indStack.clone(), actInd.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst @ Deref @ metamodelica::ListNode::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { strList: Deref @ metamodelica::ListNode::Cons { head: Deref @ "", tail: _ }, .. } }, _), tail: _ }, indStack, actInd, _, Deref @ metamodelica::ListNode::Nil) => {
                    let mut expLst = (*expLst).clone();
                    expLst = addAccStringChars(expLst.clone(), list![literal!("\n")])?;
                    Ok((expLst.clone(), indStack.clone(), actInd.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_NEW_LINE { .. } }, _), tail: expLst }, indStack, actInd, _, Deref @ metamodelica::ListNode::Nil) => {
                    let mut expLst = (*expLst).clone();
                    expLst = addAccStringChars(expLst.clone(), list![literal!("\n")])?;
                    Ok((expLst.clone(), indStack.clone(), actInd.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst @ Deref @ metamodelica::ListNode::Cons { head: (Deref @ TplAbsyn::ExpressionBase::SOFT_NEW_LINE { .. }, _), tail: _ }, indStack, actInd, _, Deref @ metamodelica::ListNode::Nil) => {
                    let mut expLst = (*expLst).clone();
                    expLst = addAccStringChars(expLst.clone(), list![literal!("\n")])?;
                    Ok((expLst.clone(), indStack.clone(), actInd.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, indStack, actInd, _, Deref @ metamodelica::ListNode::Nil) => {
                    let mut expLst = (*expLst).clone();
                    expLst = metamodelica::cons((crate::TplAbsyn::ExpressionBase::interned_SOFT_NEW_LINE(), dummySourceInfo.clone()), expLst.clone());
                    Ok((expLst.clone(), indStack.clone(), actInd.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst, indStack, actInd, lineInd, accChars @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => {
                    let mut strLst: metamodelica::List<ArcStr>;
                    let mut expLst = (*expLst).clone();
                    let mut accChars = (*accChars).clone();
                    let true = (lineInd.clone() >= actInd.clone()) else { return Err("pattern mismatch") };
                    accChars = listAppend(accChars.clone(), List::fill(literal!(" "), lineInd.clone() - actInd.clone()));
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(addAccStringChars(expLst.clone(), accChars.clone())?) {
                        Deref @ metamodelica::ListNode::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { strList: __pa0, lastHasNewLine: false } }, _), tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    strLst = metamodelica::Own::own(__pa0);
                    expLst = metamodelica::Own::own(__pa1);
                    expLst = metamodelica::cons((metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING_LIST { strList: strLst.clone(), lastHasNewLine: true }) }), dummySourceInfo.clone()), expLst.clone());
                    Ok((expLst.clone(), indStack.clone(), actInd.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst, Deref @ metamodelica::ListNode::Nil, baseInd, lineInd, accChars @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => {
                    let mut errStr: ArcStr;
                    let mut errOpt: Option<ArcStr>;
                    let mut actInd: i32;
                    let mut indStack: metamodelica::List<(i32, metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>)>;
                    let mut expLst = (*expLst).clone();
                    let true = (lineInd.clone() < baseInd.clone()) else { return Err("pattern mismatch") };
                    errStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Indent level is under the level of the '<<' determined level (by ")); __mm_s.push_str(&*intString(baseInd.clone() - lineInd.clone())); __mm_s.push_str(&*literal!(" chars).")); ArcStr::from(__mm_s) };
                    errOpt = Some(errStr.clone());
                    (expLst, indStack, actInd, _) = onNewLine(expLst.clone(), metamodelica::nil(), baseInd.clone(), baseInd.clone(), accChars.clone())?;
                    Ok((expLst.clone(), indStack.clone(), actInd, errOpt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst, indStack @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, actInd, lineInd, accChars @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => {
                    let mut errOpt: Option<ArcStr>;
                    let mut expLst = (*expLst).clone();
                    let mut indStack = (*indStack).clone();
                    let mut actInd = (*actInd).clone();
                    let true = (lineInd.clone() < actInd.clone()) else { return Err("pattern mismatch") };
                    expLst = finalizeLastStringToken(metamodelica::AsArg::as_arg(&expLst));
                    (expLst, indStack, actInd) = popIndentStack(expLst.clone(), indStack.clone(), actInd.clone(), lineInd.clone())?;
                    (expLst, indStack, actInd, errOpt) = onNewLine(expLst.clone(), indStack.clone(), actInd.clone(), lineInd.clone(), accChars.clone())?;
                    Ok((expLst.clone(), indStack.clone(), actInd.clone(), errOpt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("Parse unexpected error - TplParser.onNewLine failed .\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExpressionList, outIndentStack, outActualIndent, outError))
}

pub(crate) fn onTemplEnd(
    mut inDropLastNewLine: bool,
    mut inExpressionList: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    mut inIndentStack: metamodelica::List<(
        i32,
        metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )>,
    mut inActualIndent: i32,
    mut inLineIndent: i32,
    mut inAccStringChars: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>> {
    let mut outExpressionList: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>;
    outExpressionList = 'mc: {
        let __mc_input = (
            inDropLastNewLine,
            inExpressionList,
            inIndentStack,
            inActualIndent,
            inLineIndent,
            inAccStringChars,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, baseInd, lineInd, Deref @ metamodelica::ListNode::Nil) => {
                    let mut expLst: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>;
                    let true = (lineInd.clone() >= baseInd.clone()) else { return Err("pattern mismatch") };
                    expLst = addAccStringChars(metamodelica::nil(), List::fill(literal!(" "), lineInd.clone() - baseInd.clone()))?;
                    expLst = finalizeLastStringToken(&expLst);
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, Deref @ metamodelica::ListNode::Cons { head: (Deref @ TplAbsyn::ExpressionBase::SOFT_NEW_LINE { .. }, _), tail: expLst }, indStack, actInd, _, Deref @ metamodelica::ListNode::Nil) => {
                    let mut expLst = (*expLst).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(popIndentStack(expLst.clone(), indStack.clone(), actInd.clone(), 0)?) {
                        (__pa0, Deref @ metamodelica::ListNode::Nil, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    expLst = metamodelica::Own::own(__pa0);
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, Deref @ metamodelica::ListNode::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { strList: strLst @ Deref @ metamodelica::ListNode::Cons { head: Deref @ "", tail: _ }, lastHasNewLine: true } }, _), tail: expLst }, indStack, actInd, _, Deref @ metamodelica::ListNode::Nil) => {
                    let mut expLst = (*expLst).clone();
                    expLst = finalizeLastStringToken(&(metamodelica::cons((metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING_LIST { strList: strLst.clone(), lastHasNewLine: false }) }), dummySourceInfo.clone()), expLst.clone())));
                    let __pa0 = ::match_deref::match_deref! { match &(popIndentStack(expLst.clone(), indStack.clone(), actInd.clone(), 0)?) {
                        (__pa0, Deref @ metamodelica::ListNode::Nil, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    expLst = metamodelica::Own::own(__pa0);
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, expLst, indStack, actInd, _, Deref @ metamodelica::ListNode::Nil) => {
                    let mut expLst = (*expLst).clone();
                    expLst = finalizeLastStringToken(metamodelica::AsArg::as_arg(&expLst));
                    let __pa0 = ::match_deref::match_deref! { match &(popIndentStack(expLst.clone(), indStack.clone(), actInd.clone(), 0)?) {
                        (__pa0, Deref @ metamodelica::ListNode::Nil, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    expLst = metamodelica::Own::own(__pa0);
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, expLst, indStack, actInd, lineInd, accChars) => {
                    let mut expLst = (*expLst).clone();
                    let mut accChars = (*accChars).clone();
                    let true = (lineInd.clone() >= actInd.clone()) else { return Err("pattern mismatch") };
                    accChars = listAppend(accChars.clone(), List::fill(literal!(" "), lineInd.clone() - actInd.clone()));
                    expLst = addAccStringChars(expLst.clone(), accChars.clone())?;
                    expLst = finalizeLastStringToken(metamodelica::AsArg::as_arg(&expLst));
                    let __pa0 = ::match_deref::match_deref! { match &(popIndentStack(expLst.clone(), indStack.clone(), actInd.clone(), 0)?) {
                        (__pa0, Deref @ metamodelica::ListNode::Nil, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    expLst = metamodelica::Own::own(__pa0);
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (dropLastNL, expLst, Deref @ metamodelica::ListNode::Nil, baseInd, lineInd, accChars) => {
                    let mut expLst = (*expLst).clone();
                    let true = (lineInd.clone() < baseInd.clone()) else { return Err("pattern mismatch") };
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("Parse warning onTemplEnd() - indent level is under the level of the '<<' determined level.\n"))?;
                    expLst = onTemplEnd(dropLastNL.clone(), expLst.clone(), metamodelica::nil(), baseInd.clone(), baseInd.clone(), accChars.clone())?;
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (dropLastNL, expLst, indStack @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, actInd, lineInd, accChars) => {
                    let mut expLst = (*expLst).clone();
                    let mut indStack = (*indStack).clone();
                    let mut actInd = (*actInd).clone();
                    let true = (lineInd.clone() < actInd.clone()) else { return Err("pattern mismatch") };
                    expLst = finalizeLastStringToken(metamodelica::AsArg::as_arg(&expLst));
                    (expLst, indStack, actInd) = popIndentStack(expLst.clone(), indStack.clone(), actInd.clone(), lineInd.clone())?;
                    expLst = onTemplEnd(dropLastNL.clone(), expLst.clone(), indStack.clone(), actInd.clone(), lineInd.clone(), accChars.clone())?;
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("!!!Parse error - TplParser.onTemplEnd failed .\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExpressionList)
}

pub(crate) fn popIndentStack(
    mut inExpressionList: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    mut inIndentStack: metamodelica::List<(
        i32,
        metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )>,
    mut inActualIndent: i32,
    mut inLineIndent: i32,
) -> Result<(
    metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    metamodelica::List<(
        i32,
        metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )>,
    i32,
)> {
    let mut outExpressionList: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>;
    let mut outIndentStack: metamodelica::List<(
        i32,
        metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    )>;
    let mut outActualIndent: i32;
    (outExpressionList, outIndentStack, outActualIndent) = 'mc: {
        let __mc_input = (inExpressionList, inIndentStack, inActualIndent, inLineIndent);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst, Deref @ metamodelica::ListNode::Cons { head: (prevInd, prevExpLst), tail: indStack }, actInd, lineInd) => {
                    let mut d: i32;
                    let mut expLst = (*expLst).clone();
                    let mut indStack = (*indStack).clone();
                    let mut actInd = (*actInd).clone();
                    let true = (lineInd.clone() < actInd.clone()) else { return Err("pattern mismatch") };
                    d = actInd.clone() - prevInd.clone();
                    expLst = expLst.clone().reverse();
                    expLst = metamodelica::cons((metamodelica::Ref::new(TplAbsyn::ExpressionBase::INDENTATION { width: d, items: expLst.clone() }), dummySourceInfo.clone()), prevExpLst.clone());
                    (expLst, indStack, actInd) = popIndentStack(expLst.clone(), indStack.clone(), prevInd.clone(), lineInd.clone())?;
                    Ok((expLst.clone(), indStack.clone(), actInd.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst, indStack, actInd, lineInd) => {
                    let true = (lineInd.clone() >= actInd.clone()) else { return Err("pattern mismatch") };
                    Ok((expLst.clone(), indStack.clone(), actInd.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst, Deref @ metamodelica::ListNode::Nil, baseInd, _) => {
                    Ok((expLst.clone(), metamodelica::nil(), baseInd.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("!!!Parse error - TplParser.popIndentStack failed .\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExpressionList, outIndentStack, outActualIndent))
}

pub(crate) fn addAccStringChars(
    mut inExpressionList: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
    mut inAccStringChars: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>> {
    let mut outExpressionList: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>;
    outExpressionList = 'mc: {
        let __mc_input = (inExpressionList, inAccStringChars);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { strList: Deref @ metamodelica::ListNode::Cons { head: Deref @ "", tail: Deref @ metamodelica::ListNode::Cons { head: strNonNl, tail: strLst } }, lastHasNewLine: true } }, _), tail: expLst }, accChars @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => {
                    let mut r#str: ArcStr;
                    let mut strNonNl = (*strNonNl).clone();
                    let mut expLst = (*expLst).clone();
                    if '__try0: {
                        ::match_deref::match_deref! { match &(unwrap_break_err!(stringGetStringChar(strNonNl.clone(), ((strNonNl).len() as i32)), '__try0)) {
                            Deref @ "\n" => (),
                            _ => break '__try0 Err::<_, _>("pattern mismatch"),
                        } };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    strNonNl = { let mut __mm_s = String::new(); __mm_s.push_str(&*strNonNl); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
                    r#str = stringCharListString(accChars.clone().reverse());
                    expLst = metamodelica::cons((metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING_LIST { strList: metamodelica::cons(literal!(""), metamodelica::cons(r#str.clone(), metamodelica::cons(strNonNl.clone(), strLst.clone()))), lastHasNewLine: false }) }), dummySourceInfo.clone()), expLst.clone());
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { strList: Deref @ metamodelica::ListNode::Cons { head: Deref @ "", tail: strLst }, lastHasNewLine: true } }, _), tail: expLst }, accChars @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => {
                    let mut r#str: ArcStr;
                    let mut expLst = (*expLst).clone();
                    r#str = stringCharListString(accChars.clone().reverse());
                    expLst = metamodelica::cons((metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING_LIST { strList: metamodelica::cons(literal!(""), metamodelica::cons(r#str.clone(), metamodelica::cons(literal!("\n"), strLst.clone()))), lastHasNewLine: false }) }), dummySourceInfo.clone()), expLst.clone());
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { strList: Deref @ metamodelica::ListNode::Cons { head: Deref @ "", tail: strLst }, lastHasNewLine: false } }, _), tail: expLst }, accChars @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => {
                    let mut r#str: ArcStr;
                    let mut expLst = (*expLst).clone();
                    r#str = stringCharListString(accChars.clone().reverse());
                    expLst = metamodelica::cons((metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING_LIST { strList: metamodelica::cons(literal!(""), metamodelica::cons(r#str.clone(), strLst.clone())), lastHasNewLine: false }) }), dummySourceInfo.clone()), expLst.clone());
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expLst, accChars @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => {
                    let mut r#str: ArcStr;
                    let mut expLst = (*expLst).clone();
                    r#str = stringCharListString(accChars.clone().reverse());
                    expLst = metamodelica::cons((metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING_LIST { strList: list![literal!(""), r#str.clone()], lastHasNewLine: false }) }), dummySourceInfo.clone()), expLst.clone());
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("!!!Parse error - TplParser.addAccStringChars failed .\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExpressionList)
}

pub(crate) fn finalizeLastStringToken(
    mut inExpressionList: &metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
) -> metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)> {
    let mut outExpressionList: metamodelica::List<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>;
    outExpressionList = 'mc: {
        let __mc_input = &**inExpressionList;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { strList: Deref @ metamodelica::ListNode::Cons { head: Deref @ "", tail: Deref @ metamodelica::ListNode::Cons { head: strNonNl, tail: strLst } }, lastHasNewLine: true } }, _), tail: expLst } => {
                    let mut r#str: ArcStr;
                    let mut expLst = (*expLst).clone();
                    if '__try0: {
                        ::match_deref::match_deref! { match &(unwrap_break_err!(stringGetStringChar(strNonNl.clone(), ((strNonNl).len() as i32)), '__try0)) {
                            Deref @ "\n" => (),
                            _ => break '__try0 Err::<_, _>("pattern mismatch"),
                        } };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*strNonNl); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) };
                    expLst = finalizeLastStringToken(&(metamodelica::cons((metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING_LIST { strList: metamodelica::cons(literal!(""), metamodelica::cons(r#str.clone(), strLst.clone())), lastHasNewLine: false }) }), dummySourceInfo.clone()), expLst.clone())));
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { strList: Deref @ metamodelica::ListNode::Cons { head: Deref @ "", tail: strLst }, lastHasNewLine: true } }, _), tail: expLst } => {
                    let mut expLst = (*expLst).clone();
                    expLst = finalizeLastStringToken(&(metamodelica::cons((metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING_LIST { strList: metamodelica::cons(literal!(""), metamodelica::cons(literal!("\n"), strLst.clone())), lastHasNewLine: false }) }), dummySourceInfo.clone()), expLst.clone())));
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { strList: Deref @ metamodelica::ListNode::Cons { head: Deref @ "", tail: Deref @ metamodelica::ListNode::Nil }, lastHasNewLine: false } }, _), tail: expLst } => {
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { strList: Deref @ metamodelica::ListNode::Cons { head: Deref @ "", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "\n", tail: Deref @ metamodelica::ListNode::Nil } }, lastHasNewLine: false } }, _), tail: expLst } => {
                    let mut expLst = (*expLst).clone();
                    expLst = metamodelica::cons((metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: openmodelica_tpl::Tpl::StringToken::interned_ST_NEW_LINE() }), dummySourceInfo.clone()), expLst.clone());
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { strList: Deref @ metamodelica::ListNode::Cons { head: Deref @ "", tail: Deref @ metamodelica::ListNode::Cons { head: r#str, tail: Deref @ metamodelica::ListNode::Nil } }, lastHasNewLine: false } }, _), tail: expLst } => {
                    let mut expLst = (*expLst).clone();
                    ::match_deref::match_deref! { match &(stringGetStringChar(r#str.clone(), ((r#str).len() as i32))?) {
                        Deref @ "\n" => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    expLst = metamodelica::cons((metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_LINE { line: r#str.clone() }) }), dummySourceInfo.clone()), expLst.clone());
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { strList: Deref @ metamodelica::ListNode::Cons { head: Deref @ "", tail: Deref @ metamodelica::ListNode::Cons { head: r#str, tail: Deref @ metamodelica::ListNode::Nil } }, lastHasNewLine: false } }, _), tail: expLst } => {
                    let mut expLst = (*expLst).clone();
                    if '__try0: {
                        ::match_deref::match_deref! { match &(unwrap_break_err!(stringGetStringChar(r#str.clone(), ((r#str).len() as i32)), '__try0)) {
                            Deref @ "\n" => (),
                            _ => break '__try0 Err::<_, _>("pattern mismatch"),
                        } };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    expLst = metamodelica::cons((metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING { value: r#str.clone() }) }), dummySourceInfo.clone()), expLst.clone());
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: (Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: Deref @ Tpl::StringToken::ST_STRING_LIST { strList: Deref @ metamodelica::ListNode::Cons { head: Deref @ "", tail: strLst @ Deref @ metamodelica::ListNode::Cons { head: r#str, tail: _ } }, lastHasNewLine: false } }, _), tail: expLst } => {
                    let mut hasNL: bool;
                    let mut strLst = (*strLst).clone();
                    let mut expLst = (*expLst).clone();
                    hasNL = metamodelica::stringEq(&(literal!("\n")), &(stringGetStringChar(r#str.clone(), ((r#str).len() as i32))?));
                    strLst = strLst.clone().reverse();
                    expLst = metamodelica::cons((metamodelica::Ref::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: metamodelica::Ref::new(Tpl::StringToken::ST_STRING_LIST { strList: strLst.clone(), lastHasNewLine: hasNL }) }), dummySourceInfo.clone()), expLst.clone());
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExpressionList.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outExpressionList
}

/*
conditionExp(lesc,resc):
  'if' condArgExp(lesc,resc):(isNot, lhsExp, rhsMExpOpt)
  'then' expressionLet(lesc,resc):trueBr
  elseBranch(lesc,resc):elseBrOpt
   => CONDITION(isNot, lhsExp, rhsMExpOpt, trueBr, elseBrOpt)
*/
pub(crate) fn conditionExp(
    mut inChars: &metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outExpression: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = (::match_deref::match_deref! { match inChars {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ "i", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "f", tail: startChars } } => {
            let mut startLInfo = inLineInfo;
            let mut lesc = inLeftEsc;
            let mut resc = inRightEsc;
            let mut chars: metamodelica::List<ArcStr>;
            let mut linfo: LineInfo;
            let mut isNot: bool;
            let mut lhsExp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
            let mut trueBr: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
            let mut rhsMExpOpt: Option<metamodelica::Ref<TplAbsyn::MatchingExp>>;
            let mut elseBrOpt: Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>;
            let mut sinfo: SourceInfo;
            afterKeyword(metamodelica::AsArg::as_arg(&startChars))?;
            (chars, linfo) = interleave(startChars.clone(), startLInfo.clone());
            (chars, linfo, isNot, lhsExp, rhsMExpOpt) = condArgExp(chars, linfo, lesc.clone(), resc.clone())?;
            (chars, linfo) = interleave(chars, linfo);
            (chars, linfo, trueBr) = thenBranch(chars, linfo, lesc.clone(), resc.clone())?;
            (chars, linfo) = interleave(chars, linfo);
            (chars, linfo, elseBrOpt) = elseBranch(chars, linfo, lesc, resc);
            sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), &startLInfo, 2)?, chars.clone(), linfo.clone())?;
            (chars, linfo, (metamodelica::Ref::new(TplAbsyn::ExpressionBase::CONDITION { isNot: isNot, lhsExp: lhsExp, rhsValue: rhsMExpOpt, trueBranch: trueBr, elseBranch: elseBrOpt }), sinfo))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outChars, outLineInfo, outExpression))
}

pub(crate) fn thenBranch(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outTrueBranch: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outTrueBranch) = 'mc: {
        let __mc_input = (inChars, inLineInfo, inLeftEsc, inRightEsc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "h", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: chars } } } }, linfo, lesc, resc) => {
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, exp) = expressionLet(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
                    Ok((chars.clone(), linfo.clone(), exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc) => {
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    ::match_deref::match_deref! { match &(isKeyword(chars.clone(), metamodelica::cons(literal!("t"), metamodelica::cons(literal!("h"), metamodelica::cons(literal!("e"), metamodelica::cons(literal!("n"), metamodelica::nil())))))) {
                        (_, false) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    linfo = parseError(chars.clone(), linfo.clone(), literal!("Expected 'then' keyword at the position."), false)?;
                    (chars, linfo, exp) = expressionLet(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
                    Ok((chars.clone(), linfo.clone(), exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- !!! TplParser.thenBranch failed.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outTrueBranch))
}

/*
elseBranch(lesc,resc):
  'else' expressionLet(lesc,resc):elseBr
    => SOME(elseBr)
  |
  _ => NONE

*/
pub(crate) fn elseBranch(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outElseBranchOpt: Option<(metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo)>;
    (outChars, outLineInfo, outElseBranchOpt) = 'mc: {
        let __mc_input = (&*inChars, inLineInfo.clone(), inLeftEsc, inRightEsc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "l", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "s", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: chars } } } }, linfo, lesc, resc) => {
                    let mut elseBr: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, elseBr) = expressionLet(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
                    Ok((chars.clone(), linfo.clone(), Some(elseBr.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outElseBranchOpt)
}

/*
must not fail
condArgExp:
  'not' expressionPlus(lesc,resc):lhsExp
    => (true, lhsExp,NONE())
  |
  expressionPlus(lesc,resc):lhsExp
  //  condArgRHS:(isNot, rshMExpOpt)
  { isNot = false }
   => (isNot,lhsExp, rhsMExpOpt)
*/
pub(crate) fn condArgExp(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    bool,
    (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
    Option<metamodelica::Ref<TplAbsyn::MatchingExp>>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outIsNot: bool;
    let mut outLHSExpression: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
    let mut outRHSMExpOpt: Option<metamodelica::Ref<TplAbsyn::MatchingExp>>;
    (outChars, outLineInfo, outIsNot, outLHSExpression, outRHSMExpOpt) = 'mc: {
        let __mc_input = (inChars, inLineInfo, inLeftEsc, inRightEsc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "o", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: chars } } }, linfo, lesc, resc) => {
                    let mut lhsExp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, lhsExp) = expressionPlus(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
                    Ok((chars.clone(), linfo.clone(), true, lhsExp.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc) => {
                    let mut lhsExp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, lhsExp) = expressionPlus(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
                    Ok((chars.clone(), linfo.clone(), false, lhsExp.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outIsNot, outLHSExpression, outRHSMExpOpt))
}

/*
condArgRHS:
  'is' 'not' matchBinding:rhsMExp  =>  (true, SOME(rhsMexp))
  |
  'is' matchBinding:rhsMExp  =>  (false, SOME(rhsMexp))
  |
  _ => (false,NONE())
*/
/*
public function condArgRHS
  input list<String> inChars;
  input LineInfo inLineInfo;

  output list<String> outChars;
  output LineInfo outLineInfo;
  output Boolean outIsNot;
  output Option<TplAbsyn.MatchingExp> outRHSMExpOpt;
algorithm
  (outChars, outLineInfo, outIsNot, outRHSMExpOpt) :=
  matchcontinue (inChars, inLineInfo)
    local
      list<String> chars;
      LineInfo linfo;
      String c, lesc, resc;
      Boolean isD, isNot;
      TplAbsyn.Ident id;
      TplAbsyn.PathIdent name;
      TplAbsyn.TypedIdents fields,inargs,outargs;
      TplAbsyn.TypeSignature ts;
      Tpl.StringToken st;
      TplAbsyn.Expression exp, bexp, lhsExp, elseBr;
      TplAbsyn.MatchingExp rhsMExp;
      Option<TplAbsyn.MatchingExp> rhsMExpOpt;
      Option<TplAbsyn.Expression> elseBrOpt;
      list<TplAbsyn.Expression> expLst;
      TplAbsyn.EscOption sopt;
      list<TplAbsyn.EscOption> opts;

    case ("i"::"s":: chars, linfo)
      algorithm
        afterKeyword(chars);
        (chars, linfo) = interleave(chars, linfo);
        ("n"::"o"::"t":: chars) = chars;
        afterKeyword(chars);
        (chars, linfo) = interleave(chars, linfo);
        (chars, linfo, rhsMExp) = matchBinding(chars, linfo);
      then (chars, linfo, true, SOME(rhsMExp));

    case ("i"::"s":: chars, linfo)
      algorithm
        afterKeyword(chars);
        (chars, linfo) = interleave(chars, linfo);
        (chars, linfo, rhsMExp) = matchBinding(chars, linfo);
      then (chars, linfo, false, SOME(rhsMExp));


    else (inChars, inLineInfo, false, NONE());

  end matchcontinue;
end condArgRHS;
*/
/*
optional, can fail
matchExp(lesc,resc):
  'match' expressionIf:exp
    matchCaseList(lesc,resc):mcaseLst  { (_::_) = mcaseLst }//not optional
    matchElseCase(lesc,resc):elseLst
    matchEndMatch
   => MATCH(exp, listAppend(mcaseLst, elseLst))
  //|
  //matchCaseList(lesc,resc):mcaseLst { (_::_) = mcaseLst }
  //=> MATCH(BOUND_VALUE(IDENT("it")), mcaseLst)
*/
pub(crate) fn matchExp(
    mut inChars: &metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outExpression: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
    (outChars, outLineInfo, outExpression) = (::match_deref::match_deref! { match inChars {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ "m", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "c", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "h", tail: startChars } } } } } => {
            let mut startLInfo = inLineInfo;
            let mut lesc = inLeftEsc;
            let mut resc = inRightEsc;
            let mut chars: metamodelica::List<ArcStr>;
            let mut linfo: LineInfo;
            let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
            let mut mcaseLst: metamodelica::List<(metamodelica::Ref<TplAbsyn::MatchingExp>, (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo))>;
            let mut elseLst: metamodelica::List<(metamodelica::Ref<TplAbsyn::MatchingExp>, (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo))>;
            let mut sinfo: SourceInfo;
            afterKeyword(metamodelica::AsArg::as_arg(&startChars))?;
            (chars, linfo) = interleave(startChars.clone(), startLInfo.clone());
            (chars, linfo, exp) = expressionIf(chars, linfo, lesc.clone(), resc.clone())?;
            (chars, linfo, mcaseLst) = matchCaseListNoOpt(chars, linfo, lesc.clone(), resc.clone())?;
            (chars, linfo) = interleave(chars, linfo);
            (chars, linfo, elseLst) = matchElseCase(chars, linfo, lesc, resc);
            mcaseLst = listAppend(mcaseLst, elseLst);
            (chars, linfo) = interleave(chars, linfo);
            (chars, linfo) = matchEndMatch(chars, linfo);
            sinfo = tplSourceInfo(captureStartPosition(startChars.clone(), &startLInfo, 5)?, chars.clone(), linfo.clone())?;
            (chars, linfo, (metamodelica::Ref::new(TplAbsyn::ExpressionBase::MATCH { matchExp: exp, cases: mcaseLst }), sinfo))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outChars, outLineInfo, outExpression))
}

/*
matchCase(lesc,resc):
  'case'  matchBinding:mexp  matchCaseHeads(): mexpHeadLst
  'then'  expression:exp
     => makeMatchCaseLst(mexp::mexpHeadLst,exp)
*/
pub(crate) fn matchCase(
    mut inChars: &metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<(
        metamodelica::Ref<TplAbsyn::MatchingExp>,
        (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
    )>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outMatchCaseLst: metamodelica::List<(
        metamodelica::Ref<TplAbsyn::MatchingExp>,
        (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
    )>;
    (outChars, outLineInfo, outMatchCaseLst) = (::match_deref::match_deref! { match inChars {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ "c", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "s", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: chars } } } } => {
            let mut linfo = inLineInfo;
            let mut lesc = inLeftEsc;
            let mut resc = inRightEsc;
            let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
            let mut mexp: metamodelica::Ref<TplAbsyn::MatchingExp>;
            let mut mexpHeadList: metamodelica::List<metamodelica::Ref<TplAbsyn::MatchingExp>>;
            let mut matchCaseLst: metamodelica::List<(metamodelica::Ref<TplAbsyn::MatchingExp>, (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo))>;
            let mut chars = (*chars).clone();
            afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
            (chars, linfo) = interleave(chars.clone(), linfo);
            (chars, linfo, mexp) = matchBinding(chars.clone(), linfo)?;
            (chars, linfo) = interleave(chars.clone(), linfo);
            (chars, linfo, mexpHeadList) = matchCaseHeads(metamodelica::AsArg::as_arg(&chars), linfo);
            (chars, linfo) = interleave(chars.clone(), linfo);
            (chars, linfo, exp) = thenBranch(chars.clone(), linfo, lesc, resc)?;
            matchCaseLst = makeMatchCaseLst(&(metamodelica::cons(mexp, mexpHeadList)), exp)?;
            (chars.clone(), linfo, matchCaseLst)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outChars, outLineInfo, outMatchCaseLst))
}

/*
matchElseCase(lesc,resc):
  'else' expression:exp
    => {(REST_MATCH(), exp)}
  |
  _ => {}
*/
pub(crate) fn matchElseCase(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<(
        metamodelica::Ref<TplAbsyn::MatchingExp>,
        (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
    )>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outMatchCaseLst: metamodelica::List<(
        metamodelica::Ref<TplAbsyn::MatchingExp>,
        (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
    )>;
    (outChars, outLineInfo, outMatchCaseLst) = 'mc: {
        let __mc_input = (&*inChars, inLineInfo.clone(), inLeftEsc, inRightEsc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "l", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "s", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: chars } } } }, linfo, lesc, resc) => {
                    let mut exp: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo);
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, exp) = expressionLet(chars.clone(), linfo.clone(), lesc.clone(), resc.clone())?;
                    Ok((chars.clone(), linfo.clone(), list![(crate::TplAbsyn::MatchingExp::interned_REST_MATCH(), exp.clone())]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outMatchCaseLst)
}

/*
matchEndMatch:
  'end' 'match'
  |
  _
*/
pub(crate) fn matchEndMatch(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> (metamodelica::List<ArcStr>, LineInfo) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (&*inChars, inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "d", tail: chars } } }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ "m", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "c", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "h", tail: __pa0 } } } } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    chars = metamodelica::Own::own(__pa0);
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo)
}

/*
matchCaseHeads(lesc,resc):
  'case'  matchBinding:mexp  matchCaseHeads(): mexpHeadLst
     => mexp :: mexpHeadLst
  |
  _ => {}
*/
pub(crate) fn matchCaseHeads(
    mut inChars: &metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<metamodelica::Ref<TplAbsyn::MatchingExp>>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outMExpHeadLst: metamodelica::List<metamodelica::Ref<TplAbsyn::MatchingExp>>;
    (outChars, outLineInfo, outMExpHeadLst) = 'mc: {
        let __mc_input = (&**inChars, inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "c", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "s", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "e", tail: chars } } } }, linfo) => {
                    let mut mexp: metamodelica::Ref<TplAbsyn::MatchingExp>;
                    let mut mexpHeadList: metamodelica::List<metamodelica::Ref<TplAbsyn::MatchingExp>>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, mexp) = matchBinding(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, mexpHeadList) = matchCaseHeads(metamodelica::AsArg::as_arg(&chars), linfo.clone());
                    Ok((chars.clone(), linfo.clone(), metamodelica::cons(mexp.clone(), mexpHeadList.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outMExpHeadLst)
}

pub(crate) fn makeMatchCaseLst(
    mut inMExpHeadLst: &metamodelica::List<metamodelica::Ref<TplAbsyn::MatchingExp>>,
    mut inExpression: (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
) -> Result<
    metamodelica::List<(
        metamodelica::Ref<TplAbsyn::MatchingExp>,
        (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
    )>,
> {
    let mut outMatchCaseLst: metamodelica::List<(
        metamodelica::Ref<TplAbsyn::MatchingExp>,
        (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
    )>;
    outMatchCaseLst = (::match_deref::match_deref! { match inMExpHeadLst {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: mexp, tail: mexpHeadList } => {
            let mut exp = inExpression;
            let mut matchCaseLst: metamodelica::List<(metamodelica::Ref<TplAbsyn::MatchingExp>, (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo))>;
            matchCaseLst = makeMatchCaseLst(mexpHeadList, exp.clone())?;
            metamodelica::cons((mexp.clone(), exp), matchCaseLst)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMatchCaseLst)
}

/*
matchCaseList(lesc,resc):
  matchCase(lesc,resc):mcaseLst  matchCaseList(lesc,resc):mcrest
    => listAppend(mcaseLst, mcrest)
  |
  _ => {}
*/
pub(crate) fn matchCaseList(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<(
        metamodelica::Ref<TplAbsyn::MatchingExp>,
        (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
    )>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outMatchCases: metamodelica::List<(
        metamodelica::Ref<TplAbsyn::MatchingExp>,
        (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
    )>;
    (outChars, outLineInfo, outMatchCases) = 'mc: {
        let __mc_input = (inChars.clone(), inLineInfo.clone(), inLeftEsc, inRightEsc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc) => {
                    let mut mcaseLst: metamodelica::List<(metamodelica::Ref<TplAbsyn::MatchingExp>, (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo))>;
                    let mut mcrest: metamodelica::List<(metamodelica::Ref<TplAbsyn::MatchingExp>, (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo))>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, mcaseLst) = matchCase(metamodelica::AsArg::as_arg(&chars), linfo.clone(), lesc.clone(), resc.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, mcrest) = matchCaseList(chars.clone(), linfo.clone(), lesc.clone(), resc.clone());
                    mcaseLst = listAppend(mcaseLst.clone(), mcrest.clone());
                    Ok((chars.clone(), linfo.clone(), mcaseLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outMatchCases)
}

pub(crate) fn matchCaseListNoOpt(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inLeftEsc: ArcStr,
    mut inRightEsc: ArcStr,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<(
        metamodelica::Ref<TplAbsyn::MatchingExp>,
        (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
    )>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outMatchCases: metamodelica::List<(
        metamodelica::Ref<TplAbsyn::MatchingExp>,
        (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo),
    )>;
    (outChars, outLineInfo, outMatchCases) = 'mc: {
        let __mc_input = (inChars, inLineInfo, inLeftEsc, inRightEsc);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, lesc, resc) => {
                    let mut mcaseLst: metamodelica::List<(metamodelica::Ref<TplAbsyn::MatchingExp>, (metamodelica::Ref<TplAbsyn::ExpressionBase>, SourceInfo))>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, mcaseLst) = matchCaseList(chars.clone(), linfo.clone(), lesc.clone(), resc.clone());
                    ::match_deref::match_deref! { match &(mcaseLst.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok((chars.clone(), linfo.clone(), mcaseLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, _, _) => {
                    let mut linfo = (*linfo).clone();
                    ::match_deref::match_deref! { match &(isKeyword(chars.clone(), metamodelica::cons(literal!("c"), metamodelica::cons(literal!("a"), metamodelica::cons(literal!("s"), metamodelica::cons(literal!("e"), metamodelica::nil())))))) {
                        (_, false) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    linfo = parseError(chars.clone(), linfo.clone(), literal!("Expected keyword 'case' at the position."), true)?;
                    Ok((chars.clone(), linfo.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("!!! TplParser.matchCaseListNoOpt failed.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outMatchCases))
}

/*
matchBinding:
  matchBinding_base:headMExp  matchBinding_tail(headMExp):mexp
    => mexp

*/
pub(crate) fn matchBinding(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::Ref<TplAbsyn::MatchingExp>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outMatchingExp: metamodelica::Ref<TplAbsyn::MatchingExp>;
    (outChars, outLineInfo, outMatchingExp) = (::match_deref::match_deref! { match &((inChars, inLineInfo)) {
        (chars, linfo) => {
            let mut headMExp: metamodelica::Ref<TplAbsyn::MatchingExp>;
            let mut mexp: metamodelica::Ref<TplAbsyn::MatchingExp>;
            let mut chars = (*chars).clone();
            let mut linfo = (*linfo).clone();
            (chars, linfo, headMExp) = matchBinding_base(chars.clone(), linfo.clone())?;
            (chars, linfo) = interleave(chars.clone(), linfo.clone());
            (chars, linfo, mexp) = matchBinding_tail(chars.clone(), linfo.clone(), headMExp);
            (chars.clone(), linfo.clone(), mexp)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outChars, outLineInfo, outMatchingExp))
}

/*
matchBinding_tail(headMExp):
  '::' matchBinding:restMExp
    => LIST_CONS_MATCH(headMExp, restMExp)
  |
  _ => headMExp
*/
pub(crate) fn matchBinding_tail(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inHeadMatchingExp: metamodelica::Ref<TplAbsyn::MatchingExp>,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::Ref<TplAbsyn::MatchingExp>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outMatchingExp: metamodelica::Ref<TplAbsyn::MatchingExp>;
    (outChars, outLineInfo, outMatchingExp) = 'mc: {
        let __mc_input = (&*inChars, inLineInfo.clone(), inHeadMatchingExp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ ":", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ ":", tail: chars } }, linfo, headMExp) => {
                    let mut restMExp: metamodelica::Ref<TplAbsyn::MatchingExp>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, restMExp) = matchBinding(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::MatchingExp::LIST_CONS_MATCH { head: headMExp.clone(), rest: restMExp.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), inHeadMatchingExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outMatchingExp)
}

/*
matchBinding_base:
  'SOME' someBinding_rest:mexp
    => SOME_MATCH(mexp)
  |
  'NONE' takeEmptyBraces
    => NONE_MATCH()
  |
  '(' matchBinding:headMExp  tupleOrSingleMatch(headMExp):mexp ')'
    => mexp
  |
  '{' '}'
    => LIST_MATCH({})
  |
  '{' matchBinding:headMExp  listMatch_rest:mrest '}
    => LIST_MATCH(headMExp :: mrest)
  |
  stringConstant:strRevList
    => STRING_MATCH(stringAppendList(listReverse(strRevList))
  |
  literalConstant:(str,litType)
    => LITERAL_MATCH(str,litType)
  |
  '_'
    => REST_MATCH()
  |
  pathIdent:pid  afterIdentBinding(pid):mexp
    => mexp
*/
pub(crate) fn matchBinding_base(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::Ref<TplAbsyn::MatchingExp>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outMatchingExp: metamodelica::Ref<TplAbsyn::MatchingExp>;
    (outChars, outLineInfo, outMatchingExp) = 'mc: {
        let __mc_input = (inChars, inLineInfo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "S", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "O", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "M", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "E", tail: chars } } } }, linfo) => {
                    let mut mexp: metamodelica::Ref<TplAbsyn::MatchingExp>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, mexp) = someBinding_rest(chars.clone(), linfo.clone());
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::MatchingExp::SOME_MATCH { value: mexp.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "N", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "O", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "N", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "E", tail: chars } } } }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo) = takeEmptyBraces(chars.clone(), linfo.clone());
                    Ok((chars.clone(), linfo.clone(), crate::TplAbsyn::MatchingExp::interned_NONE_MATCH()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "(", tail: chars }, linfo) => {
                    let mut headMExp: metamodelica::Ref<TplAbsyn::MatchingExp>;
                    let mut mexp: metamodelica::Ref<TplAbsyn::MatchingExp>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, headMExp) = matchBinding(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, mexp) = tupleOrSingleMatch(chars.clone(), linfo.clone(), headMExp.clone());
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!(")"))?;
                    Ok((chars.clone(), linfo.clone(), mexp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "{", tail: chars }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ "}", tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    chars = metamodelica::Own::own(__pa0);
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::MatchingExp::LIST_MATCH { listElts: metamodelica::nil() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "{", tail: chars }, linfo) => {
                    let mut headMExp: metamodelica::Ref<TplAbsyn::MatchingExp>;
                    let mut mrest: metamodelica::List<metamodelica::Ref<TplAbsyn::MatchingExp>>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, headMExp) = matchBinding(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, mrest) = listMatch_rest(metamodelica::AsArg::as_arg(&chars), linfo.clone());
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!("}"))?;
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::MatchingExp::LIST_MATCH { listElts: metamodelica::cons(headMExp.clone(), mrest.clone()) })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "_", tail: chars }, linfo) => {
                    Ok((chars.clone(), linfo.clone(), crate::TplAbsyn::MatchingExp::interned_REST_MATCH()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut strRevList: metamodelica::List<ArcStr>;
                    let mut r#str: ArcStr;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, strRevList) = stringConstant(chars.clone(), linfo.clone())?;
                    r#str = stringAppendList(strRevList.clone().reverse());
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::MatchingExp::STRING_MATCH { value: r#str.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut r#str: ArcStr;
                    let mut ts: metamodelica::Ref<TplAbsyn::TypeSignature>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, r#str, ts) = literalConstant(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::MatchingExp::LITERAL_MATCH { value: r#str.clone(), litType: ts.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut pid: metamodelica::Ref<TplAbsyn::PathIdent>;
                    let mut mexp: metamodelica::Ref<TplAbsyn::MatchingExp>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, pid) = pathIdent(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, mexp) = afterIdentBinding(chars.clone(), linfo.clone(), pid.clone())?;
                    Ok((chars.clone(), linfo.clone(), mexp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut linfo = (*linfo).clone();
                    linfo = parseError(chars.clone(), linfo.clone(), literal!("Expected a valid match binding expression at the position."), true)?;
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::MatchingExp::LITERAL_MATCH { value: literal!("#Error#"), litType: metamodelica::Ref::new(TplAbsyn::TypeSignature::UNRESOLVED_TYPE { reason: literal!("#Error#") }) })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outMatchingExp))
}

/*
someBinding_rest:
  '(' '__' ')'
    => SOME_MATCH(REST_MATCH())
  |
  '(' matchBinding:mexp ')'
    => SOME_MATCH(mexp)
  |
  _ => SOME_MATCH(REST_MATCH())
*/
pub(crate) fn someBinding_rest(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::Ref<TplAbsyn::MatchingExp>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outMatchingExp: metamodelica::Ref<TplAbsyn::MatchingExp>;
    (outChars, outLineInfo, outMatchingExp) = 'mc: {
        let __mc_input = (&*inChars, inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "(", tail: chars }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ "_", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "_", tail: __pa0 } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    chars = metamodelica::Own::own(__pa0);
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!(")"))?;
                    Ok((chars.clone(), linfo.clone(), crate::TplAbsyn::MatchingExp::interned_REST_MATCH()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "(", tail: chars }, linfo) => {
                    let mut mexp: metamodelica::Ref<TplAbsyn::MatchingExp>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, mexp) = matchBinding(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!(")"))?;
                    Ok((chars.clone(), linfo.clone(), mexp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), crate::TplAbsyn::MatchingExp::interned_REST_MATCH()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outMatchingExp)
}

/*
takeEmptyBraces:
  '(' ')'
  |
  _
*/
pub(crate) fn takeEmptyBraces(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> (metamodelica::List<ArcStr>, LineInfo) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    (outChars, outLineInfo) = 'mc: {
        let __mc_input = (&*inChars, inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "(", tail: chars }, linfo) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!(")"))?;
                    Ok((chars.clone(), linfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo)
}

/*
tupleOrSingleMatch(headMExp):
  ',' matchBinding:secMExp  listMatch_rest:mrest
    => TUPLE_MATCH(headMExp :: secMExp :: mrest)
  |
  _ => headMExp

*/
pub(crate) fn tupleOrSingleMatch(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inHeadMatchingExp: metamodelica::Ref<TplAbsyn::MatchingExp>,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::Ref<TplAbsyn::MatchingExp>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outMatchingExp: metamodelica::Ref<TplAbsyn::MatchingExp>;
    (outChars, outLineInfo, outMatchingExp) = 'mc: {
        let __mc_input = (&*inChars, inLineInfo.clone(), inHeadMatchingExp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ ",", tail: chars }, linfo, headMExp) => {
                    let mut secMExp: metamodelica::Ref<TplAbsyn::MatchingExp>;
                    let mut mrest: metamodelica::List<metamodelica::Ref<TplAbsyn::MatchingExp>>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, secMExp) = matchBinding(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, mrest) = listMatch_rest(metamodelica::AsArg::as_arg(&chars), linfo.clone());
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::MatchingExp::TUPLE_MATCH { tupleArgs: metamodelica::cons(headMExp.clone(), metamodelica::cons(secMExp.clone(), mrest.clone())) })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), inHeadMatchingExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outMatchingExp)
}

/*
listMatch_rest:
  ',' matchBinding:mexp  listMatch_rest:mrest
    => mexp :: mrest
  |
  _ => {}

*/
pub(crate) fn listMatch_rest(
    mut inChars: &metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<metamodelica::Ref<TplAbsyn::MatchingExp>>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outMatchingExpListRest: metamodelica::List<metamodelica::Ref<TplAbsyn::MatchingExp>>;
    (outChars, outLineInfo, outMatchingExpListRest) = 'mc: {
        let __mc_input = (&**inChars, inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ ",", tail: chars }, linfo) => {
                    let mut mexp: metamodelica::Ref<TplAbsyn::MatchingExp>;
                    let mut mrest: metamodelica::List<metamodelica::Ref<TplAbsyn::MatchingExp>>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, mexp) = matchBinding(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, mrest) = listMatch_rest(metamodelica::AsArg::as_arg(&chars), linfo.clone());
                    Ok((chars.clone(), linfo.clone(), metamodelica::cons(mexp.clone(), mrest.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outMatchingExpListRest)
}

/*
afterIdentBinding(pid):
  '(' ')'
    => RECORD_MATCH(pid, {})
  |
  '(' '__' ')'
    => RECORD_MATCH(pid, {}) //TODO: to be RECORD_TYPE_MATCH(pid)
  |
  '(' fieldBinding:fb  fieldBinding_rest:fbs ')'
    => RECORD_MATCH(pid, fb::fbs)
  |
  {pid is PATH_IDENT}
  => error "Expected '(' after the dot path."
  //RECORD_MATCH(pid, {})
  |
  {pid is IDENT(id)}
  'as' matchBinding:mexp
    => BIND_AS_MATCH(id, mexp)
  |
  {pid is IDENT(id)}
  _ => BIND_MATCH(id)
*/
pub(crate) fn afterIdentBinding(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
    mut inPathIdent: metamodelica::Ref<TplAbsyn::PathIdent>,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::Ref<TplAbsyn::MatchingExp>,
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outMatchingExp: metamodelica::Ref<TplAbsyn::MatchingExp>;
    (outChars, outLineInfo, outMatchingExp) = 'mc: {
        let __mc_input = (inChars, inLineInfo, inPathIdent);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "(", tail: chars }, linfo, pid) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ ")", tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    chars = metamodelica::Own::own(__pa0);
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::MatchingExp::RECORD_MATCH { tagName: pid.clone(), fieldMatchings: metamodelica::nil() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "(", tail: chars }, linfo, pid) => {
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ "_", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "_", tail: __pa0 } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    chars = metamodelica::Own::own(__pa0);
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!(")"))?;
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::MatchingExp::RECORD_MATCH { tagName: pid.clone(), fieldMatchings: metamodelica::nil() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "(", tail: chars }, linfo, pid) => {
                    let mut fb: (ArcStr, metamodelica::Ref<TplAbsyn::MatchingExp>);
                    let mut fbs: metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::MatchingExp>)>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, fb) = fieldBinding(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, fbs) = fieldBinding_rest(metamodelica::AsArg::as_arg(&chars), linfo.clone());
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!(")"))?;
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::MatchingExp::RECORD_MATCH { tagName: pid.clone(), fieldMatchings: metamodelica::cons(fb.clone(), fbs.clone()) })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, pid @ Deref @ TplAbsyn::PathIdent::PATH_IDENT { ident: _, path: _ }) => {
                    let mut linfo = (*linfo).clone();
                    linfo = parseError(chars.clone(), linfo.clone(), literal!("Expected '(' after the dot path."), false)?;
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::MatchingExp::RECORD_MATCH { tagName: pid.clone(), fieldMatchings: metamodelica::nil() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "s", tail: chars } }, linfo, Deref @ TplAbsyn::PathIdent::IDENT { ident: id }) => {
                    let mut mexp: metamodelica::Ref<TplAbsyn::MatchingExp>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    afterKeyword(metamodelica::AsArg::as_arg(&chars))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, mexp) = matchBinding(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::MatchingExp::BIND_AS_MATCH { bindIdent: id.clone(), matchingExp: mexp.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo, Deref @ TplAbsyn::PathIdent::IDENT { ident: id }) => {
                    Ok((chars.clone(), linfo.clone(), metamodelica::Ref::new(TplAbsyn::MatchingExp::BIND_MATCH { bindIdent: id.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("!!! TplParser.afterIdentBinding failed.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outMatchingExp))
}

/*
must not fail
fieldBinding:
  identifier:fldId '=' matchBinding:mexp
    => (fldId, mexp)
*/
pub(crate) fn fieldBinding(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(
    metamodelica::List<ArcStr>,
    LineInfo,
    (ArcStr, metamodelica::Ref<TplAbsyn::MatchingExp>),
)> {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outFieldBinding: (ArcStr, metamodelica::Ref<TplAbsyn::MatchingExp>);
    (outChars, outLineInfo, outFieldBinding) = 'mc: {
        let __mc_input = (inChars, inLineInfo);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (chars, linfo) => {
                    let mut id: ArcStr;
                    let mut mexp: metamodelica::Ref<TplAbsyn::MatchingExp>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo, id) = identifierNoOpt(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleaveExpectChar(chars.clone(), linfo.clone(), literal!("="))?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, mexp) = matchBinding(chars.clone(), linfo.clone())?;
                    Ok((chars.clone(), linfo.clone(), (id.clone(), mexp.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- !!! TplParser.fieldBinding failed.\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outChars, outLineInfo, outFieldBinding))
}

/*
fieldBinding_rest:
  ',' fieldBinding:fb  fieldBinding_rest:fbs
    => fb :: fbs
  |
  _ => {}

*/
pub(crate) fn fieldBinding_rest(
    mut inChars: &metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> (
    metamodelica::List<ArcStr>,
    LineInfo,
    metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::MatchingExp>)>,
) {
    let mut outChars: metamodelica::List<ArcStr>;
    let mut outLineInfo: LineInfo;
    let mut outFieldBindingsRest: metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::MatchingExp>)>;
    (outChars, outLineInfo, outFieldBindingsRest) = 'mc: {
        let __mc_input = (&**inChars, inLineInfo.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ ",", tail: chars }, linfo) => {
                    let mut fb: (ArcStr, metamodelica::Ref<TplAbsyn::MatchingExp>);
                    let mut fbs: metamodelica::List<(ArcStr, metamodelica::Ref<TplAbsyn::MatchingExp>)>;
                    let mut chars = (*chars).clone();
                    let mut linfo = (*linfo).clone();
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, fb) = fieldBinding(chars.clone(), linfo.clone())?;
                    (chars, linfo) = interleave(chars.clone(), linfo.clone());
                    (chars, linfo, fbs) = fieldBinding_rest(metamodelica::AsArg::as_arg(&chars), linfo.clone());
                    Ok((chars.clone(), linfo.clone(), metamodelica::cons(fb.clone(), fbs.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inChars.clone(), inLineInfo.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outChars, outLineInfo, outFieldBindingsRest)
}

/*
annotationFooter:
  'annotation(...)' => str
  |
  _ => ""

*/
fn annotationFooter(
    mut inChars: metamodelica::List<ArcStr>,
    mut inLineInfo: LineInfo,
) -> Result<(metamodelica::List<ArcStr>, LineInfo, ArcStr)> {
    let mut chars: metamodelica::List<ArcStr>;
    let mut linfo: LineInfo;
    let mut footer: ArcStr;
    (chars, linfo, footer) = (::match_deref::match_deref! { match &((inChars.clone(), inLineInfo.clone())) {
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "o", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "t", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "i", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "o", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "n", tail: __esc_chars } } } } } } } } } }, __esc_linfo) => {
            chars = (*__esc_chars).clone();
            linfo = (*__esc_linfo).clone();
            let mut footerChars: metamodelica::List<ArcStr>;
            (footerChars, chars) = List::split(inChars.clone(), List::position(literal!(";"), &inChars)? + 1)?;
            footer = stringAppendList(footerChars);
            (chars.clone(), linfo.clone(), footer)
        },
        _ => {
            (inChars, inLineInfo, literal!("annotation(__OpenModelica_generator=\"Susan\");"))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((chars, linfo, footer))
}
