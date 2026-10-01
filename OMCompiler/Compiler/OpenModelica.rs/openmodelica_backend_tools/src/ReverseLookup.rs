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

use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_program_util::ProgramUtil;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;
use openmodelica_util::ExecStat;
use openmodelica_util::JSON;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct PathEntry {
    pub tree: metamodelica::Ref<PathTree::Tree>,
    pub shadowed: bool,
}

impl metamodelica::gc::MMTrace for PathEntry {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.tree, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.shadowed, __mmv)?;
        Ok(())
    }
}
impl Default for PathEntry {
    fn default() -> Self {
        Self {
            tree: Default::default(),
            shadowed: Default::default(),
        }
    }
}

pub type ENTRY = PathEntry;

pub mod PathTree {
    use super::*;
    pub type Key = ArcStr;

    pub type Value = metamodelica::Ref<PathEntry>;

    pub(crate) fn keyStr(mut inKey: Key) -> ArcStr {
        let mut outString: ArcStr;
        outString = inKey;
        outString
    }

    pub(crate) fn valueStr(mut inValue: Value) -> ArcStr {
        let mut outString: ArcStr;
        outString = literal!("");
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
            metamodelica::Ref<PathEntry>,
            metamodelica::Ref<PathEntry>,
            ArcStr,
        ) -> Result<metamodelica::Ref<PathEntry>>,
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
                    if !(referenceEq(&*(var_field!((*tree).value, Tree::NODE).clone()), &*(&*value))) {
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
                        right: crate::ReverseLookup::PathTree::Tree::interned_EMPTY(),
                    });
                } else if key_comp == 1 {
                    outTree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: crate::ReverseLookup::PathTree::Tree::interned_EMPTY(),
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
                    if !(referenceEq(&*(var_field!((*tree).value, Tree::LEAF).clone()), &*(&*value))) {
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
        mut inValues: &metamodelica::List<(ArcStr, metamodelica::Ref<PathEntry>)>,
        mut conflictFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<PathEntry>,
            metamodelica::Ref<PathEntry>,
            ArcStr,
        ) -> Result<metamodelica::Ref<PathEntry>>,
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
        mut r#fn: &dyn ::std::ops::Fn(Option<metamodelica::Ref<PathEntry>>) -> Result<metamodelica::Ref<PathEntry>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        pub type UpdateFn =
            std::sync::Arc<dyn ::std::ops::Fn(Option<metamodelica::Ref<PathEntry>>) -> Result<Value> + 'static>;

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
                        right: crate::ReverseLookup::PathTree::Tree::interned_EMPTY(),
                    });
                } else if key_comp == 1 {
                    new_tree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: crate::ReverseLookup::PathTree::Tree::interned_EMPTY(),
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
        mut inFunc: &'__b dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<PathEntry>, FT) -> Result<FT>,
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
        mut foldFunc: &dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<PathEntry>, FT) -> Result<(FT, bool)>,
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
        mut foldFunc: &dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<PathEntry>, FT1, FT2) -> Result<(FT1, FT2)>,
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
        mut func: &dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<PathEntry>) -> Result<()>,
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
        mut inValues: &metamodelica::List<(ArcStr, metamodelica::Ref<PathEntry>)>,
        mut conflictFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<PathEntry>,
            metamodelica::Ref<PathEntry>,
            ArcStr,
        ) -> Result<metamodelica::Ref<PathEntry>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        let mut tree: metamodelica::Ref<Tree> = crate::ReverseLookup::PathTree::Tree::interned_EMPTY();
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
    ) -> Option<metamodelica::Ref<PathEntry>> {
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
            metamodelica::Ref<PathEntry>,
            metamodelica::Ref<PathEntry>,
            ArcStr,
        ) -> Result<metamodelica::Ref<PathEntry>>,
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
        mut lst: metamodelica::List<metamodelica::Ref<PathEntry>>,
    ) -> metamodelica::List<metamodelica::Ref<PathEntry>> {
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
        mut inFunc: &dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<PathEntry>) -> Result<metamodelica::Ref<PathEntry>>,
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
                    || !(referenceEq(&*(value.clone()), &*(&*new_value)))
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
                if !(referenceEq(&*(value.clone()), &*(&*new_value))) {
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
        mut inFunc: &dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<PathEntry>, FT) -> Result<(metamodelica::Ref<PathEntry>, FT)>,
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
                    || !(referenceEq(&*(value.clone()), &*(&*new_value)))
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
                if !(referenceEq(&*(value.clone()), &*(&*new_value))) {
                    assign_variant_field!(outTree => Tree::LEAF; value = new_value);
                }
                outTree
            }
            _ => inTree,
        });
        Ok((outTree, outResult))
    }

    pub(crate) fn new() -> metamodelica::Ref<Tree> {
        let mut outTree: metamodelica::Ref<Tree> = crate::ReverseLookup::PathTree::Tree::interned_EMPTY();
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
                node = setTreeLeftRight(outNode, __outNode_left.clone(), crate::ReverseLookup::PathTree::Tree::interned_EMPTY())?;
                setTreeLeftRight(child.clone(), node, crate::ReverseLookup::PathTree::Tree::interned_EMPTY())?
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
                node = setTreeLeftRight(outNode, crate::ReverseLookup::PathTree::Tree::interned_EMPTY(), __outNode_right.clone())?;
                setTreeLeftRight(child.clone(), crate::ReverseLookup::PathTree::Tree::interned_EMPTY(), node)?
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
        mut lst: metamodelica::List<(ArcStr, metamodelica::Ref<PathEntry>)>,
    ) -> metamodelica::List<(ArcStr, metamodelica::Ref<PathEntry>)> {
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
            add(tree.clone(), key, value, &move |__a0: metamodelica::Ref<PathEntry>,
                                                 __a1: metamodelica::Ref<PathEntry>,
                                                 __a2: ArcStr|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(addConflictReplace(__a0, &__a1, &__a2))
            })?;
        Ok(outTree)
    }
}

pub mod Paths {
    use super::*;
    #[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct Paths {
        pub tree: metamodelica::Ref<PathTree::Tree>,
        pub relativePath: metamodelica::List<ArcStr>,
        pub currentPath: metamodelica::List<ArcStr>,
    }

    impl metamodelica::gc::MMTrace for Paths {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.tree, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.relativePath, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.currentPath, __mmv)?;
            Ok(())
        }
    }
    impl Default for Paths {
        fn default() -> Self {
            Self {
                tree: Default::default(),
                relativePath: Default::default(),
                currentPath: Default::default(),
            }
        }
    }

    pub type PATHS = Paths;

    pub(crate) fn currentPathStr(mut paths: &metamodelica::Ref<Paths>) -> ArcStr {
        let mut r#str: ArcStr = stringDelimitList(paths.currentPath.clone().reverse(), literal!("."));
        r#str
    }
}

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Match {
    pub name: metamodelica::Ref<Absyn::ComponentRef>,
    pub scope: ArcStr,
    pub info: SourceInfo,
}

impl metamodelica::gc::MMTrace for Match {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.scope, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.info, __mmv)?;
        Ok(())
    }
}
impl Default for Match {
    fn default() -> Self {
        Self {
            name: Default::default(),
            scope: Default::default(),
            info: Default::default(),
        }
    }
}

pub type MATCH = Match;

pub type Matches = metamodelica::List<Match>;

pub fn lookup(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut scope: metamodelica::Ref<Absyn::Path>,
    mut program: &Absyn::Program,
    mut exactMatch: bool,
    mut prettyPrint: bool,
) -> Result<ArcStr> {
    let mut result: ArcStr;
    let mut tree: metamodelica::Ref<PathTree::Tree>;
    let mut matches: Matches;
    let mut paths: metamodelica::Ref<Paths::Paths>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut opt_path: Option<metamodelica::Ref<Absyn::Path>>;
    let mut relative_path: metamodelica::Ref<Absyn::Path>;
    let mut grouped_matches: metamodelica::List<metamodelica::List<Match>>;
    ExecStat::execStatReset()?;
    if AbsynUtil::pathEqual(
        &scope,
        &(metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("AllLoadedClasses"),
        })),
    ) {
        tree = addPath(&path, PathTree::new())?;
        paths = metamodelica::Ref::new(Paths::Paths {
            tree: tree,
            relativePath: AbsynUtil::pathToStringList(&path),
            currentPath: metamodelica::nil(),
        });
        matches = lookupInProgram(program, paths, exactMatch)?;
    } else {
        opt_path = AbsynUtil::pathStripSamePrefix(path.clone(), scope.clone())?;
        relative_path = opt_path.unwrap_or(path.clone());
        tree = addPath(&relative_path, PathTree::new())?;
        paths = metamodelica::Ref::new(Paths::Paths {
            tree: tree,
            relativePath: AbsynUtil::pathToStringList(&relative_path),
            currentPath: metamodelica::nil(),
        });
        match '__try0: {
            cls =
                unwrap_break_err!(ProgramUtil::getPathedClassInProgram(scope.clone(), program, false, false), '__try0);
            matches = unwrap_break_err!(lookupInClass(&cls, paths.clone(), exactMatch, metamodelica::nil()), '__try0);
            Ok::<_, &'static str>((matches.clone(),))
        } {
            Ok((__try0_o0,)) => {
                matches = __try0_o0;
            }
            Err(_) => {
                matches = metamodelica::nil();
            }
        }
    }
    grouped_matches = groupMatches(&matches)?;
    result = serializeMatches(&grouped_matches, prettyPrint)?;
    ExecStat::execStat(
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("ReverseLookup.lookup("));
            __mm_s.push_str(&*AbsynUtil::pathString(path, literal!("."), true, false)?);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        }),
    )?;
    Ok(result)
}

fn addPath<'__b>(
    mut path: &'__b metamodelica::Ref<Absyn::Path>,
    mut tree: metamodelica::Ref<PathTree::Tree>,
) -> Result<metamodelica::Ref<PathTree::Tree>> {
    '__tco: loop {
        let mut opt_entry: Option<metamodelica::Ref<PathEntry>>;
        let mut entry: metamodelica::Ref<PathEntry>;
        match &**path {
            Absyn::Path::IDENT { .. } => {
                return Ok(PathTree::add(
                    tree,
                    var_field!((**path).name, Absyn::Path::IDENT),
                    &(metamodelica::Ref::new(PathEntry {
                        tree: PathTree::new(),
                        shadowed: false,
                    })),
                    &move |__a0: metamodelica::Ref<PathEntry>,
                           __a1: metamodelica::Ref<PathEntry>,
                           __a2: ArcStr|
                          -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(PathTree::addConflictKeep(&__a0, __a1, &__a2))
                    },
                )?);
            }
            Absyn::Path::QUALIFIED { .. } => {
                opt_entry = PathTree::getOpt(&tree, var_field!((**path).name, Absyn::Path::QUALIFIED).clone());
                if (opt_entry).is_some() {
                    entry = opt_entry.ok_or("pattern mismatch")?;
                    assign_field!(
                        entry.tree = addPath(var_field!((**path).path, Absyn::Path::QUALIFIED), entry.tree.clone())?
                    );
                } else {
                    entry = metamodelica::Ref::new(PathEntry {
                        tree: addPath(var_field!((**path).path, Absyn::Path::QUALIFIED), PathTree::new())?,
                        shadowed: false,
                    });
                }
                return Ok(PathTree::add(
                    tree,
                    var_field!((**path).name, Absyn::Path::QUALIFIED),
                    &entry,
                    &move |__a0: metamodelica::Ref<PathEntry>,
                           __a1: metamodelica::Ref<PathEntry>,
                           __a2: ArcStr|
                          -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(PathTree::addConflictReplace(__a0, &__a1, &__a2))
                    },
                )?);
            }
            Absyn::Path::FULLYQUALIFIED { .. } => {
                (path, tree) = (var_field!((**path).path, Absyn::Path::FULLYQUALIFIED), tree);
                continue '__tco;
            }
        }
    }
}

fn lookupPath(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut paths: &metamodelica::Ref<PathTree::Tree>,
    mut exactMatch: bool,
    mut fullyQualified: bool,
) -> bool {
    let mut found: bool = false;
    let mut entry: metamodelica::Ref<PathEntry> = <metamodelica::Ref<PathEntry> as ::std::default::Default>::default();
    found = 'mc: {
        let __mc_input = &**path;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::IDENT { .. } => {
                    let mut entry: metamodelica::Ref<PathEntry> = entry.clone();
                    entry = PathTree::get(paths, var_field!((**path).name, Absyn::Path::IDENT).clone())?;
                    Ok(((fullyQualified || !(entry.shadowed.clone())) && PathTree::isEmpty(&entry.tree), entry.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            entry = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::QUALIFIED { .. } => {
                    let mut entry: metamodelica::Ref<PathEntry> = entry.clone();
                    let mut found: bool = found.clone();
                    entry = PathTree::get(paths, var_field!((**path).name, Absyn::Path::QUALIFIED).clone())?;
                    if entry.shadowed.clone() && !(fullyQualified) {
                        found = false;
                    } else if PathTree::isEmpty(&entry.tree) && !(exactMatch) {
                        found = true;
                    } else {
                        found = lookupPath(var_field!((**path).path, Absyn::Path::QUALIFIED), &entry.tree, exactMatch, fullyQualified);
                    }
                    Ok((found, entry.clone(), found.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            entry = __wb0;
            found = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::FULLYQUALIFIED { .. } => {
                    Ok(lookupPath(var_field!((**path).path, Absyn::Path::FULLYQUALIFIED), paths, exactMatch, true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    found
}

fn matchPath(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut paths: &metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut info: SourceInfo,
    mut matches: Matches,
) -> Matches {
    let mut matches: Matches = matches;
    if lookupPath(path, &paths.tree, exactMatch, false) {
        matches = metamodelica::cons(
            Match {
                name: AbsynUtil::pathToCref(path),
                scope: Paths::currentPathStr(paths),
                info: info,
            },
            matches,
        );
    }
    matches
}

fn lookupCref(
    mut cref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut paths: &metamodelica::Ref<PathTree::Tree>,
    mut exactMatch: bool,
    mut fullyQualified: bool,
) -> bool {
    let mut found: bool = false;
    let mut entry: metamodelica::Ref<PathEntry> = <metamodelica::Ref<PathEntry> as ::std::default::Default>::default();
    found = 'mc: {
        let __mc_input = &**cref;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_IDENT { .. } => {
                    let mut entry: metamodelica::Ref<PathEntry> = entry.clone();
                    entry = PathTree::get(paths, var_field!((**cref).name, Absyn::ComponentRef::CREF_IDENT).clone())?;
                    Ok(((fullyQualified || !(entry.shadowed.clone())) && PathTree::isEmpty(&entry.tree), entry.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            entry = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_QUAL { .. } => {
                    let mut entry: metamodelica::Ref<PathEntry> = entry.clone();
                    let mut found: bool = found.clone();
                    entry = PathTree::get(paths, var_field!((**cref).name, Absyn::ComponentRef::CREF_QUAL).clone())?;
                    if entry.shadowed.clone() && !(fullyQualified) {
                        found = false;
                    } else if PathTree::isEmpty(&entry.tree) && !(exactMatch) {
                        found = true;
                    } else {
                        found = lookupCref(var_field!((**cref).componentRef, Absyn::ComponentRef::CREF_QUAL), &entry.tree, exactMatch, fullyQualified);
                    }
                    Ok((found, entry.clone(), found.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            entry = __wb0;
            found = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => {
                    Ok(lookupCref(var_field!((**cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED), paths, exactMatch, true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    found
}

fn matchCref(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut paths: &metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut info: SourceInfo,
    mut matches: Matches,
) -> Matches {
    let mut matches: Matches = matches;
    if lookupCref(&cref, &paths.tree, exactMatch, false) {
        matches = metamodelica::cons(
            Match {
                name: cref,
                scope: Paths::currentPathStr(paths),
                info: info,
            },
            matches,
        );
    }
    matches
}

fn shadowLocalNames(
    mut cls: &metamodelica::Ref<Absyn::Class>,
    mut paths: metamodelica::Ref<Paths::Paths>,
) -> Result<metamodelica::Ref<Paths::Paths>> {
    let mut paths: metamodelica::Ref<Paths::Paths> = paths;
    for mut part in &*AbsynUtil::getClassPartsInClass(cls) {
        for mut item in &*AbsynUtil::getElementItemsInClass(cls)? {
            paths = shadowLocalNamesInElementItem(metamodelica::AsArg::as_arg(&item), paths)?;
        }
    }
    Ok(paths)
}

fn shadowLocalNamesInElementItem(
    mut item: &metamodelica::Ref<Absyn::ElementItem>,
    mut paths: metamodelica::Ref<Paths::Paths>,
) -> Result<metamodelica::Ref<Paths::Paths>> {
    let mut paths: metamodelica::Ref<Paths::Paths> = paths;
    let mut spec: metamodelica::Ref<Absyn::ElementSpec>;
    paths = (::match_deref::match_deref! { match item {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: __esc_spec, .. } } => {
            spec = (*__esc_spec).clone();
            shadowLocalNamesInElementSpec(metamodelica::AsArg::as_arg(&spec), paths)?
        },
        _ => paths,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(paths)
}

fn shadowLocalNamesInElementSpec(
    mut spec: &metamodelica::Ref<Absyn::ElementSpec>,
    mut paths: metamodelica::Ref<Paths::Paths>,
) -> Result<metamodelica::Ref<Paths::Paths>> {
    let mut paths: metamodelica::Ref<Paths::Paths> = paths;
    paths = (match &**spec {
        Absyn::ElementSpec::CLASSDEF {
            class_: __spec_class_, ..
        } => shadowLocalName(AbsynUtil::className(metamodelica::AsArg::as_arg(&__spec_class_)), paths)?,
        Absyn::ElementSpec::COMPONENTS {
            components: __spec_components,
            ..
        } => {
            for mut comp in &*__spec_components.clone() {
                paths = shadowLocalName(AbsynUtil::componentName(metamodelica::AsArg::as_arg(&comp))?, paths)?;
            }
            paths
        }
        _ => paths,
    });
    Ok(paths)
}

fn shadowLocalName(
    mut name: ArcStr,
    mut paths: metamodelica::Ref<Paths::Paths>,
) -> Result<metamodelica::Ref<Paths::Paths>> {
    let mut paths: metamodelica::Ref<Paths::Paths> = paths;
    let mut entry: metamodelica::Ref<PathEntry>;
    if PathTree::hasKey(paths.tree.clone(), name.clone())? {
        entry = PathTree::get(&paths.tree, name.clone())?;
        if !(entry.shadowed.clone()) {
            assign_field!(entry.shadowed = true);
            assign_field!(paths.tree = PathTree::update(paths.tree.clone(), &name, &entry)?);
        }
    }
    Ok(paths)
}

fn lookupInProgram(
    mut program: &Absyn::Program,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
) -> Result<Matches> {
    let mut matches: Matches = metamodelica::nil();
    for mut cls in &*program.classes.clone() {
        matches = lookupInClass(metamodelica::AsArg::as_arg(&cls), paths.clone(), exactMatch, matches)?;
    }
    Ok(matches)
}

fn lookupInClass(
    mut cls: &metamodelica::Ref<Absyn::Class>,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    let mut relative_path: metamodelica::List<ArcStr> = paths.relativePath.clone();
    let mut local_paths: metamodelica::Ref<Paths::Paths>;
    local_paths = shadowLocalNames(cls, paths)?;
    if !((relative_path).is_empty()) && metamodelica::stringEq(&cls.name, &((relative_path).head().cloned()?)) {
        relative_path = (relative_path).rest()?;
        assign_field!(local_paths.relativePath = relative_path.clone());
        if !((relative_path).is_empty()) {
            assign_field!(
                local_paths.tree = addPath(&(AbsynUtil::stringListPath(relative_path)?), local_paths.tree.clone())?
            );
        }
    }
    matches = lookupInClassDef(
        &cls.body,
        cls.name.clone(),
        local_paths,
        exactMatch,
        cls.info.clone(),
        matches,
    )?;
    Ok(matches)
}

fn lookupInClassDef(
    mut cdef: &metamodelica::Ref<Absyn::ClassDef>,
    mut name: ArcStr,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut info: SourceInfo,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    let mut local_paths: metamodelica::Ref<Paths::Paths> = paths.clone();
    matches = (match &**cdef {
        Absyn::ClassDef::PARTS {
            ann: __cdef_ann,
            classParts: __cdef_classParts,
            ..
        } => {
            assign_field!(local_paths.currentPath = metamodelica::cons(name, local_paths.currentPath.clone()));
            for mut part in &*__cdef_classParts.clone() {
                matches = lookupInClassPart(
                    metamodelica::AsArg::as_arg(&part),
                    local_paths.clone(),
                    exactMatch,
                    &info,
                    matches,
                )?;
            }
            for mut ann in &*__cdef_ann.clone() {
                matches = lookupInAnnotation(
                    metamodelica::AsArg::as_arg(&ann),
                    local_paths.clone(),
                    exactMatch,
                    matches,
                )?;
            }
            matches
        }
        Absyn::ClassDef::DERIVED {
            arguments: __cdef_arguments,
            comment: __cdef_comment,
            typeSpec: __cdef_typeSpec,
            ..
        } => {
            matches = lookupInTypeSpec(
                metamodelica::AsArg::as_arg(&__cdef_typeSpec),
                &paths,
                exactMatch,
                info,
                matches,
            )?;
            for mut arg in &*__cdef_arguments.clone() {
                matches = lookupInElementArg(metamodelica::AsArg::as_arg(&arg), paths.clone(), exactMatch, matches)?;
            }
            lookupInCommentOpt(__cdef_comment.clone(), paths, exactMatch, matches)?
        }
        Absyn::ClassDef::ENUMERATION {
            comment: __cdef_comment,
            enumLiterals: __cdef_enumLiterals,
        } => {
            matches = lookupInEnumDef(
                metamodelica::AsArg::as_arg(&__cdef_enumLiterals),
                paths.clone(),
                exactMatch,
                matches,
            )?;
            lookupInCommentOpt(__cdef_comment.clone(), paths, exactMatch, matches)?
        }
        Absyn::ClassDef::OVERLOAD {
            comment: __cdef_comment,
            ..
        } => lookupInCommentOpt(__cdef_comment.clone(), paths, exactMatch, matches)?,
        Absyn::ClassDef::CLASS_EXTENDS {
            ann: __cdef_ann,
            modifications: __cdef_modifications,
            parts: __cdef_parts,
            ..
        } => {
            assign_field!(local_paths.currentPath = metamodelica::cons(name, local_paths.currentPath.clone()));
            for mut arg in &*__cdef_modifications.clone() {
                matches = lookupInElementArg(
                    metamodelica::AsArg::as_arg(&arg),
                    local_paths.clone(),
                    exactMatch,
                    matches,
                )?;
            }
            for mut part in &*__cdef_parts.clone() {
                matches = lookupInClassPart(
                    metamodelica::AsArg::as_arg(&part),
                    local_paths.clone(),
                    exactMatch,
                    &info,
                    matches,
                )?;
            }
            for mut ann in &*__cdef_ann.clone() {
                matches = lookupInAnnotation(
                    metamodelica::AsArg::as_arg(&ann),
                    local_paths.clone(),
                    exactMatch,
                    matches,
                )?;
            }
            matches
        }
        Absyn::ClassDef::PDER {
            comment: __cdef_comment,
            functionName: __cdef_functionName,
            ..
        } => {
            matches = matchPath(
                metamodelica::AsArg::as_arg(&__cdef_functionName),
                &paths,
                exactMatch,
                info,
                matches,
            );
            lookupInCommentOpt(__cdef_comment.clone(), paths, exactMatch, matches)?
        }
        _ => matches,
    });
    Ok(matches)
}

fn lookupInClassPart(
    mut part: &metamodelica::Ref<Absyn::ClassPart>,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut info: &SourceInfo,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (match &**part {
        Absyn::ClassPart::PUBLIC {
            contents: __part_contents,
        } => {
            for mut e in &*__part_contents.clone() {
                matches = lookupInElementItem(metamodelica::AsArg::as_arg(&e), paths.clone(), exactMatch, matches)?;
            }
            matches
        }
        Absyn::ClassPart::PROTECTED {
            contents: __part_contents,
        } => {
            for mut e in &*__part_contents.clone() {
                matches = lookupInElementItem(metamodelica::AsArg::as_arg(&e), paths.clone(), exactMatch, matches)?;
            }
            matches
        }
        Absyn::ClassPart::EQUATIONS {
            contents: __part_contents,
        } => {
            for mut e in &*__part_contents.clone() {
                matches = lookupInEquationItem(metamodelica::AsArg::as_arg(&e), paths.clone(), exactMatch, matches)?;
            }
            matches
        }
        Absyn::ClassPart::INITIALEQUATIONS {
            contents: __part_contents,
        } => {
            for mut e in &*__part_contents.clone() {
                matches = lookupInEquationItem(metamodelica::AsArg::as_arg(&e), paths.clone(), exactMatch, matches)?;
            }
            matches
        }
        Absyn::ClassPart::ALGORITHMS {
            contents: __part_contents,
        } => {
            for mut alg in &*__part_contents.clone() {
                matches = lookupInAlgorithmItem(metamodelica::AsArg::as_arg(&alg), paths.clone(), exactMatch, matches)?;
            }
            matches
        }
        Absyn::ClassPart::INITIALALGORITHMS {
            contents: __part_contents,
        } => {
            for mut alg in &*__part_contents.clone() {
                matches = lookupInAlgorithmItem(metamodelica::AsArg::as_arg(&alg), paths.clone(), exactMatch, matches)?;
            }
            matches
        }
        Absyn::ClassPart::EXTERNAL {
            annotation_: __part_annotation_,
            externalDecl: __part_externalDecl,
        } => {
            matches = lookupInExternalDecl(
                metamodelica::AsArg::as_arg(&__part_externalDecl),
                paths.clone(),
                exactMatch,
                info,
                matches,
            )?;
            if (__part_annotation_).is_some() {
                matches = lookupInAnnotation(
                    &(__part_annotation_.clone().ok_or("pattern mismatch")?),
                    paths,
                    exactMatch,
                    matches,
                )?;
            }
            matches
        }
        _ => matches,
    });
    Ok(matches)
}

fn lookupInEnumDef(
    mut enumDef: &metamodelica::Ref<Absyn::EnumDef>,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (match &**enumDef {
        Absyn::EnumDef::ENUMLITERALS {
            enumLiterals: __enumDef_enumLiterals,
        } => {
            for mut lit in &*__enumDef_enumLiterals.clone() {
                matches = lookupInCommentOpt(lit.comment.clone(), paths.clone(), exactMatch, matches)?;
            }
            matches
        }
        _ => matches,
    });
    Ok(matches)
}

fn lookupInCommentOpt(
    mut cmt: Option<metamodelica::Ref<Absyn::Comment>>,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    if (cmt).is_some() {
        matches = lookupInComment(&(cmt.ok_or("pattern mismatch")?), paths, exactMatch, matches)?;
    }
    Ok(matches)
}

fn lookupInComment(
    mut cmt: &metamodelica::Ref<Absyn::Comment>,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    if (cmt.annotation_).is_some() {
        matches = lookupInAnnotation(
            &(cmt.annotation_.clone().ok_or("pattern mismatch")?),
            paths,
            exactMatch,
            matches,
        )?;
    }
    Ok(matches)
}

fn lookupInAnnotation(
    mut ann: &metamodelica::Ref<Absyn::Annotation>,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    for mut arg in &*ann.elementArgs.clone() {
        matches = lookupInElementArg(metamodelica::AsArg::as_arg(&arg), paths.clone(), exactMatch, matches)?;
    }
    Ok(matches)
}

fn lookupInElementArg(
    mut arg: &metamodelica::Ref<Absyn::ElementArg>,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (match &**arg {
        Absyn::ElementArg::MODIFICATION {
            modification: __arg_modification,
            ..
        } => {
            if (__arg_modification).is_some() {
                matches = lookupInModification(
                    &(__arg_modification.clone().ok_or("pattern mismatch")?),
                    paths,
                    exactMatch,
                    matches,
                )?;
            }
            matches
        }
        Absyn::ElementArg::REDECLARATION {
            constrainClass: __arg_constrainClass,
            elementSpec: __arg_elementSpec,
            info: __arg_info,
            ..
        } => {
            matches = lookupInElementSpec(
                metamodelica::AsArg::as_arg(&__arg_elementSpec),
                paths.clone(),
                exactMatch,
                __arg_info.clone(),
                matches,
            )?;
            if (__arg_constrainClass).is_some() {
                matches = lookupInConstrainClass(
                    &(__arg_constrainClass.clone().ok_or("pattern mismatch")?),
                    paths,
                    exactMatch,
                    __arg_info.clone(),
                    matches,
                )?;
            }
            matches
        }
        _ => matches,
    });
    Ok(matches)
}

fn lookupInModification(
    mut r#mod: &metamodelica::Ref<Absyn::Modification>,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    for mut arg in &*r#mod.elementArgLst.clone() {
        matches = lookupInElementArg(metamodelica::AsArg::as_arg(&arg), paths.clone(), exactMatch, matches)?;
    }
    matches = lookupInEqMod(&r#mod.eqMod, &paths, exactMatch, matches)?;
    Ok(matches)
}

fn lookupInEqMod(
    mut eqMod: &metamodelica::Ref<Absyn::EqMod>,
    mut paths: &metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (match &**eqMod {
        Absyn::EqMod::EQMOD {
            exp: __eqMod_exp,
            info: __eqMod_info,
        } => lookupInExp(
            __eqMod_exp.clone(),
            paths,
            exactMatch,
            metamodelica::AsArg::as_arg(&__eqMod_info),
            matches,
        )?,
        _ => matches,
    });
    Ok(matches)
}

fn lookupInExp<'__b>(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut paths: &'__b metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut info: &'__b SourceInfo,
    mut matches: Matches,
) -> Result<Matches> {
    '__tco: loop {
        match &*exp {
            Absyn::Exp::CREF {
                componentRef: __exp_componentRef,
            } => {
                return Ok(matchCref(
                    __exp_componentRef.clone(),
                    paths,
                    exactMatch,
                    info.clone(),
                    matches,
                ));
            }
            Absyn::Exp::BINARY {
                exp1: __exp_exp1,
                exp2: __exp_exp2,
                ..
            } => {
                matches = lookupInExp(__exp_exp1.clone(), paths, exactMatch, info, matches)?;
                {
                    (exp, paths, exactMatch, info, matches) = (__exp_exp2.clone(), paths, exactMatch, info, matches);
                    continue '__tco;
                }
            }
            Absyn::Exp::UNARY { exp: __exp_exp, .. } => {
                (exp, paths, exactMatch, info, matches) = (__exp_exp.clone(), paths, exactMatch, info, matches);
                continue '__tco;
            }
            Absyn::Exp::LBINARY {
                exp1: __exp_exp1,
                exp2: __exp_exp2,
                ..
            } => {
                matches = lookupInExp(__exp_exp1.clone(), paths, exactMatch, info, matches)?;
                {
                    (exp, paths, exactMatch, info, matches) = (__exp_exp2.clone(), paths, exactMatch, info, matches);
                    continue '__tco;
                }
            }
            Absyn::Exp::LUNARY { exp: __exp_exp, .. } => {
                (exp, paths, exactMatch, info, matches) = (__exp_exp.clone(), paths, exactMatch, info, matches);
                continue '__tco;
            }
            Absyn::Exp::IFEXP {
                elseBranch: __exp_elseBranch,
                elseIfBranch: __exp_elseIfBranch,
                ifExp: __exp_ifExp,
                trueBranch: __exp_trueBranch,
            } => {
                matches = lookupInExp(__exp_ifExp.clone(), paths, exactMatch, info, matches)?;
                matches = lookupInExp(__exp_trueBranch.clone(), paths, exactMatch, info, matches)?;
                matches = lookupInExp(__exp_elseBranch.clone(), paths, exactMatch, info, matches)?;
                for mut branch in &*__exp_elseIfBranch.clone() {
                    matches = lookupInExp(Util::tuple21(branch.clone()), paths, exactMatch, info, matches)?;
                    matches = lookupInExp(Util::tuple22(branch.clone()), paths, exactMatch, info, matches)?;
                }
                return Ok(matches);
            }
            Absyn::Exp::CALL {
                functionArgs: __exp_functionArgs,
                function_: __exp_function_,
                ..
            } => {
                matches = matchCref(__exp_function_.clone(), paths, exactMatch, info.clone(), matches);
                return Ok(lookupInFunctionArgs(
                    metamodelica::AsArg::as_arg(&__exp_functionArgs),
                    paths,
                    exactMatch,
                    info,
                    matches,
                )?);
            }
            Absyn::Exp::PARTEVALFUNCTION {
                functionArgs: __exp_functionArgs,
                function_: __exp_function_,
            } => {
                matches = matchCref(__exp_function_.clone(), paths, exactMatch, info.clone(), matches);
                return Ok(lookupInFunctionArgs(
                    metamodelica::AsArg::as_arg(&__exp_functionArgs),
                    paths,
                    exactMatch,
                    info,
                    matches,
                )?);
            }
            Absyn::Exp::ARRAY {
                arrayExp: __exp_arrayExp,
            } => {
                for mut e in &*__exp_arrayExp.clone() {
                    matches = lookupInExp(e.clone(), paths, exactMatch, info, matches)?;
                }
                return Ok(matches);
            }
            Absyn::Exp::MATRIX { matrix: __exp_matrix } => {
                for mut row in &*__exp_matrix.clone() {
                    for mut e in &*row.clone() {
                        matches = lookupInExp(e.clone(), paths, exactMatch, info, matches)?;
                    }
                }
                return Ok(matches);
            }
            Absyn::Exp::RANGE {
                start: __exp_start,
                step: __exp_step,
                stop: __exp_stop,
            } => {
                matches = lookupInExp(__exp_start.clone(), paths, exactMatch, info, matches)?;
                if (__exp_step).is_some() {
                    matches = lookupInExp(
                        __exp_step.clone().ok_or("pattern mismatch")?,
                        paths,
                        exactMatch,
                        info,
                        matches,
                    )?;
                }
                {
                    (exp, paths, exactMatch, info, matches) = (__exp_stop.clone(), paths, exactMatch, info, matches);
                    continue '__tco;
                }
            }
            Absyn::Exp::TUPLE {
                expressions: __exp_expressions,
            } => {
                for mut e in &*__exp_expressions.clone() {
                    matches = lookupInExp(e.clone(), paths, exactMatch, info, matches)?;
                }
                return Ok(matches);
            }
            Absyn::Exp::EXPRESSIONCOMMENT { exp: __exp_exp, .. } => {
                (exp, paths, exactMatch, info, matches) = (__exp_exp.clone(), paths, exactMatch, info, matches);
                continue '__tco;
            }
            Absyn::Exp::SUBSCRIPTED_EXP {
                exp: __exp_exp,
                subscripts: __exp_subscripts,
            } => {
                matches = lookupInExp(__exp_exp.clone(), paths, exactMatch, info, matches)?;
                return Ok(lookupInSubscripts(
                    metamodelica::AsArg::as_arg(&__exp_subscripts),
                    paths,
                    exactMatch,
                    info,
                    matches,
                )?);
            }
            _ => return Ok(matches),
        }
    }
}

fn lookupInCref(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut paths: &metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut info: SourceInfo,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = matchCref(cref.clone(), paths, exactMatch, info.clone(), matches);
    matches = lookupInCrefSubs(&cref, paths, exactMatch, &info, matches)?;
    Ok(matches)
}

fn lookupInCrefSubs<'__b>(
    mut cref: &'__b metamodelica::Ref<Absyn::ComponentRef>,
    mut paths: &'__b metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut info: &'__b SourceInfo,
    mut matches: Matches,
) -> Result<Matches> {
    '__tco: loop {
        match &**cref {
            Absyn::ComponentRef::CREF_IDENT { .. } => {
                return Ok(lookupInSubscripts(
                    var_field!((**cref).subscripts, Absyn::ComponentRef::CREF_IDENT),
                    paths,
                    exactMatch,
                    info,
                    matches,
                )?);
            }
            Absyn::ComponentRef::CREF_QUAL { .. } => {
                matches = lookupInSubscripts(
                    var_field!((**cref).subscripts, Absyn::ComponentRef::CREF_QUAL),
                    paths,
                    exactMatch,
                    info,
                    matches,
                )?;
                {
                    (cref, paths, exactMatch, info, matches) = (
                        var_field!((**cref).componentRef, Absyn::ComponentRef::CREF_QUAL),
                        paths,
                        exactMatch,
                        info,
                        matches,
                    );
                    continue '__tco;
                }
            }
            Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => {
                (cref, paths, exactMatch, info, matches) = (
                    var_field!((**cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED),
                    paths,
                    exactMatch,
                    info,
                    matches,
                );
                continue '__tco;
            }
            _ => return Ok(matches),
        }
    }
}

fn lookupInSubscripts(
    mut subs: &metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut paths: &metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut info: &SourceInfo,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    for mut sub in &**subs {
        matches = lookupInSubscript(metamodelica::AsArg::as_arg(&sub), paths, exactMatch, info, matches)?;
    }
    Ok(matches)
}

fn lookupInSubscript(
    mut sub: &metamodelica::Ref<Absyn::Subscript>,
    mut paths: &metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut info: &SourceInfo,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (match &**sub {
        Absyn::Subscript::SUBSCRIPT {
            subscript: __sub_subscript,
        } => lookupInExp(__sub_subscript.clone(), paths, exactMatch, info, matches)?,
        _ => matches,
    });
    Ok(matches)
}

fn lookupInFunctionArgs(
    mut args: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut paths: &metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut info: &SourceInfo,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (match &**args {
        Absyn::FunctionArgs::FUNCTIONARGS {
            argNames: __args_argNames,
            args: __args_args,
        } => {
            for mut arg in &*__args_args.clone() {
                matches = lookupInExp(arg.clone(), paths, exactMatch, info, matches)?;
            }
            for mut named_arg in &*__args_argNames.clone() {
                matches = lookupInExp(named_arg.argValue.clone(), paths, exactMatch, info, matches)?;
            }
            matches
        }
        Absyn::FunctionArgs::FOR_ITER_FARG {
            exp: __args_exp,
            iterators: __args_iterators,
            ..
        } => {
            matches = lookupInExp(__args_exp.clone(), paths, exactMatch, info, matches)?;
            matches = lookupInForIterators(
                metamodelica::AsArg::as_arg(&__args_iterators),
                paths,
                exactMatch,
                info,
                matches,
            )?;
            matches
        }
    });
    Ok(matches)
}

fn lookupInForIterators(
    mut iterators: &metamodelica::List<metamodelica::Ref<Absyn::ForIterator>>,
    mut paths: &metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut info: &SourceInfo,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    for mut i in &**iterators {
        if (i.range).is_some() {
            matches = lookupInExp(
                i.range.clone().ok_or("pattern mismatch")?,
                paths,
                exactMatch,
                info,
                matches,
            )?;
        }
    }
    Ok(matches)
}

fn lookupInElementItem(
    mut item: &metamodelica::Ref<Absyn::ElementItem>,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (match &**item {
        Absyn::ElementItem::ELEMENTITEM {
            element: __item_element,
        } => lookupInElement(metamodelica::AsArg::as_arg(&__item_element), paths, exactMatch, matches)?,
        _ => matches,
    });
    Ok(matches)
}

fn lookupInElement(
    mut element: &metamodelica::Ref<Absyn::Element>,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (match &**element {
        Absyn::Element::ELEMENT {
            constrainClass: __element_constrainClass,
            info: __element_info,
            specification: __element_specification,
            ..
        } => {
            matches = lookupInElementSpec(
                metamodelica::AsArg::as_arg(&__element_specification),
                paths.clone(),
                exactMatch,
                __element_info.clone(),
                matches,
            )?;
            if (__element_constrainClass).is_some() {
                matches = lookupInConstrainClass(
                    &(__element_constrainClass.clone().ok_or("pattern mismatch")?),
                    paths,
                    exactMatch,
                    __element_info.clone(),
                    matches,
                )?;
            }
            matches
        }
        _ => matches,
    });
    Ok(matches)
}

fn lookupInElementSpec(
    mut spec: &metamodelica::Ref<Absyn::ElementSpec>,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut info: SourceInfo,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (match &**spec {
        Absyn::ElementSpec::CLASSDEF {
            class_: __spec_class_, ..
        } => lookupInClass(metamodelica::AsArg::as_arg(&__spec_class_), paths, exactMatch, matches)?,
        Absyn::ElementSpec::EXTENDS {
            annotationOpt: __spec_annotationOpt,
            elementArg: __spec_elementArg,
            path: __spec_path,
        } => {
            matches = matchPath(
                metamodelica::AsArg::as_arg(&__spec_path),
                &paths,
                exactMatch,
                info,
                matches,
            );
            for mut arg in &*__spec_elementArg.clone() {
                matches = lookupInElementArg(metamodelica::AsArg::as_arg(&arg), paths.clone(), exactMatch, matches)?;
            }
            if (__spec_annotationOpt).is_some() {
                matches = lookupInAnnotation(
                    &(__spec_annotationOpt.clone().ok_or("pattern mismatch")?),
                    paths,
                    exactMatch,
                    matches,
                )?;
            }
            matches
        }
        Absyn::ElementSpec::IMPORT {
            import_: __spec_import_,
            ..
        } => {
            matches = lookupInImport(
                metamodelica::AsArg::as_arg(&__spec_import_),
                &paths,
                exactMatch,
                info,
                matches,
            );
            matches
        }
        Absyn::ElementSpec::COMPONENTS {
            components: __spec_components,
            typeSpec: __spec_typeSpec,
            ..
        } => {
            matches = lookupInTypeSpec(
                metamodelica::AsArg::as_arg(&__spec_typeSpec),
                &paths,
                exactMatch,
                info.clone(),
                matches,
            )?;
            for mut c in &*__spec_components.clone() {
                matches = lookupInComponentItem(
                    metamodelica::AsArg::as_arg(&c),
                    paths.clone(),
                    exactMatch,
                    &info,
                    matches,
                )?;
            }
            matches
        }
    });
    Ok(matches)
}

fn lookupInConstrainClass(
    mut constrainClass: &metamodelica::Ref<Absyn::ConstrainClass>,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut info: SourceInfo,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = lookupInElementSpec(&constrainClass.elementSpec, paths, exactMatch, info, matches)?;
    Ok(matches)
}

fn lookupInImport(
    mut imp: &Absyn::Import,
    mut paths: &metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut info: SourceInfo,
    mut matches: Matches,
) -> Matches {
    let mut matches: Matches = matches;
    matches = (match imp.clone() {
        Absyn::Import::NAMED_IMPORT { .. } => matchPath(
            var_field!(imp.path, Absyn::Import::NAMED_IMPORT),
            paths,
            exactMatch,
            info,
            matches,
        ),
        Absyn::Import::QUAL_IMPORT { .. } => matchPath(
            var_field!(imp.path, Absyn::Import::QUAL_IMPORT),
            paths,
            exactMatch,
            info,
            matches,
        ),
        Absyn::Import::UNQUAL_IMPORT { .. } => matchPath(
            var_field!(imp.path, Absyn::Import::UNQUAL_IMPORT),
            paths,
            exactMatch,
            info,
            matches,
        ),
        Absyn::Import::GROUP_IMPORT { .. } => matchPath(
            var_field!(imp.prefix, Absyn::Import::GROUP_IMPORT),
            paths,
            exactMatch,
            info,
            matches,
        ),
    });
    matches
}

fn lookupInComponentItem(
    mut item: &metamodelica::Ref<Absyn::ComponentItem>,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut info: &SourceInfo,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = lookupInComponent(&item.component, paths.clone(), exactMatch, info, matches)?;
    if (item.condition).is_some() {
        matches = lookupInExp(
            item.condition.clone().ok_or("pattern mismatch")?,
            &paths,
            exactMatch,
            info,
            matches,
        )?;
    }
    Ok(matches)
}

fn lookupInComponent(
    mut component: &Absyn::Component,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut info: &SourceInfo,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = lookupInSubscripts(&component.arrayDim, &paths, exactMatch, info, matches)?;
    if (component.modification).is_some() {
        matches = lookupInModification(
            &(component.modification.clone().ok_or("pattern mismatch")?),
            paths,
            exactMatch,
            matches,
        )?;
    }
    Ok(matches)
}

fn lookupInTypeSpec(
    mut typeSpec: &metamodelica::Ref<Absyn::TypeSpec>,
    mut paths: &metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut info: SourceInfo,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (match &**typeSpec {
        Absyn::TypeSpec::TPATH {
            arrayDim: __typeSpec_arrayDim,
            path: __typeSpec_path,
        } => {
            matches = matchPath(
                metamodelica::AsArg::as_arg(&__typeSpec_path),
                paths,
                exactMatch,
                info.clone(),
                matches,
            );
            if (__typeSpec_arrayDim).is_some() {
                matches = lookupInSubscripts(
                    &(__typeSpec_arrayDim.clone().ok_or("pattern mismatch")?),
                    paths,
                    exactMatch,
                    &info,
                    matches,
                )?;
            }
            matches
        }
        _ => matches,
    });
    Ok(matches)
}

fn lookupInEquationItems(
    mut items: &metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    for mut item in &**items {
        matches = lookupInEquationItem(metamodelica::AsArg::as_arg(&item), paths.clone(), exactMatch, matches)?;
    }
    Ok(matches)
}

fn lookupInEquationItem(
    mut item: &metamodelica::Ref<Absyn::EquationItem>,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (match &**item {
        Absyn::EquationItem::EQUATIONITEM {
            comment: __item_comment,
            equation_: __item_equation_,
            info: __item_info,
        } => {
            matches = lookupInEquation(
                metamodelica::AsArg::as_arg(&__item_equation_),
                paths.clone(),
                exactMatch,
                __item_info.clone(),
                matches,
            )?;
            lookupInCommentOpt(__item_comment.clone(), paths, exactMatch, matches)?
        }
        _ => matches,
    });
    Ok(matches)
}

fn lookupInEquation(
    mut eq: &metamodelica::Ref<Absyn::Equation>,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut info: SourceInfo,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    let () = (match &**eq {
        Absyn::Equation::EQ_IF {
            elseIfBranches: __eq_elseIfBranches,
            equationElseItems: __eq_equationElseItems,
            equationTrueItems: __eq_equationTrueItems,
            ifExp: __eq_ifExp,
        } => {
            matches = lookupInExp(__eq_ifExp.clone(), &paths, exactMatch, &info, matches)?;
            matches = lookupInEquationItems(
                metamodelica::AsArg::as_arg(&__eq_equationTrueItems),
                paths.clone(),
                exactMatch,
                matches,
            )?;
            for mut branch in &*__eq_elseIfBranches.clone() {
                matches = lookupInExp(Util::tuple21(branch.clone()), &paths, exactMatch, &info, matches)?;
                matches = lookupInEquationItems(&(Util::tuple22(branch.clone())), paths.clone(), exactMatch, matches)?;
            }
            matches = lookupInEquationItems(
                metamodelica::AsArg::as_arg(&__eq_equationElseItems),
                paths,
                exactMatch,
                matches,
            )?;
            ()
        }
        Absyn::Equation::EQ_EQUALS {
            leftSide: __eq_leftSide,
            rightSide: __eq_rightSide,
        } => {
            matches = lookupInExp(__eq_leftSide.clone(), &paths, exactMatch, &info, matches)?;
            matches = lookupInExp(__eq_rightSide.clone(), &paths, exactMatch, &info, matches)?;
            ()
        }
        Absyn::Equation::EQ_PDE {
            leftSide: __eq_leftSide,
            rightSide: __eq_rightSide,
            ..
        } => {
            matches = lookupInExp(__eq_leftSide.clone(), &paths, exactMatch, &info, matches)?;
            matches = lookupInExp(__eq_rightSide.clone(), &paths, exactMatch, &info, matches)?;
            ()
        }
        Absyn::Equation::EQ_CONNECT {
            connector1: __eq_connector1,
            connector2: __eq_connector2,
        } => {
            matches = lookupInCref(__eq_connector1.clone(), &paths, exactMatch, info.clone(), matches)?;
            matches = lookupInCref(__eq_connector2.clone(), &paths, exactMatch, info, matches)?;
            ()
        }
        Absyn::Equation::EQ_FOR {
            forEquations: __eq_forEquations,
            iterators: __eq_iterators,
        } => {
            matches = lookupInForIterators(
                metamodelica::AsArg::as_arg(&__eq_iterators),
                &paths,
                exactMatch,
                &info,
                matches,
            )?;
            matches = lookupInEquationItems(
                metamodelica::AsArg::as_arg(&__eq_forEquations),
                paths,
                exactMatch,
                matches,
            )?;
            ()
        }
        Absyn::Equation::EQ_WHEN_E {
            elseWhenEquations: __eq_elseWhenEquations,
            whenEquations: __eq_whenEquations,
            whenExp: __eq_whenExp,
        } => {
            matches = lookupInExp(__eq_whenExp.clone(), &paths, exactMatch, &info, matches)?;
            matches = lookupInEquationItems(
                metamodelica::AsArg::as_arg(&__eq_whenEquations),
                paths.clone(),
                exactMatch,
                matches,
            )?;
            for mut branch in &*__eq_elseWhenEquations.clone() {
                matches = lookupInExp(Util::tuple21(branch.clone()), &paths, exactMatch, &info, matches)?;
                matches = lookupInEquationItems(&(Util::tuple22(branch.clone())), paths.clone(), exactMatch, matches)?;
            }
            ()
        }
        Absyn::Equation::EQ_NORETCALL {
            functionArgs: __eq_functionArgs,
            functionName: __eq_functionName,
        } => {
            matches = lookupInCref(__eq_functionName.clone(), &paths, exactMatch, info.clone(), matches)?;
            matches = lookupInFunctionArgs(
                metamodelica::AsArg::as_arg(&__eq_functionArgs),
                &paths,
                exactMatch,
                &info,
                matches,
            )?;
            ()
        }
        _ => (),
    });
    Ok(matches)
}

fn lookupInAlgorithmItems(
    mut items: &metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    for mut item in &**items {
        matches = lookupInAlgorithmItem(metamodelica::AsArg::as_arg(&item), paths.clone(), exactMatch, matches)?;
    }
    Ok(matches)
}

fn lookupInAlgorithmItem(
    mut item: &metamodelica::Ref<Absyn::AlgorithmItem>,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (match &**item {
        Absyn::AlgorithmItem::ALGORITHMITEM {
            algorithm_: __item_algorithm_,
            comment: __item_comment,
            info: __item_info,
        } => {
            matches = lookupInAlgorithm(
                metamodelica::AsArg::as_arg(&__item_algorithm_),
                paths.clone(),
                exactMatch,
                __item_info.clone(),
                matches,
            )?;
            matches = lookupInCommentOpt(__item_comment.clone(), paths, exactMatch, matches)?;
            matches
        }
        _ => matches,
    });
    Ok(matches)
}

fn lookupInAlgorithm(
    mut alg: &metamodelica::Ref<Absyn::Algorithm>,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut info: SourceInfo,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    let () = (match &**alg {
        Absyn::Algorithm::ALG_ASSIGN {
            assignComponent: __alg_assignComponent,
            value: __alg_value,
        } => {
            matches = lookupInExp(__alg_assignComponent.clone(), &paths, exactMatch, &info, matches)?;
            matches = lookupInExp(__alg_value.clone(), &paths, exactMatch, &info, matches)?;
            ()
        }
        Absyn::Algorithm::ALG_IF {
            elseBranch: __alg_elseBranch,
            elseIfAlgorithmBranch: __alg_elseIfAlgorithmBranch,
            ifExp: __alg_ifExp,
            trueBranch: __alg_trueBranch,
        } => {
            matches = lookupInExp(__alg_ifExp.clone(), &paths, exactMatch, &info, matches)?;
            matches = lookupInAlgorithmItems(
                metamodelica::AsArg::as_arg(&__alg_trueBranch),
                paths.clone(),
                exactMatch,
                matches,
            )?;
            for mut branch in &*__alg_elseIfAlgorithmBranch.clone() {
                matches = lookupInExp(Util::tuple21(branch.clone()), &paths, exactMatch, &info, matches)?;
                matches = lookupInAlgorithmItems(&(Util::tuple22(branch.clone())), paths.clone(), exactMatch, matches)?;
            }
            matches = lookupInAlgorithmItems(
                metamodelica::AsArg::as_arg(&__alg_elseBranch),
                paths,
                exactMatch,
                matches,
            )?;
            ()
        }
        Absyn::Algorithm::ALG_FOR {
            forBody: __alg_forBody,
            iterators: __alg_iterators,
        } => {
            matches = lookupInForIterators(
                metamodelica::AsArg::as_arg(&__alg_iterators),
                &paths,
                exactMatch,
                &info,
                matches,
            )?;
            matches = lookupInAlgorithmItems(metamodelica::AsArg::as_arg(&__alg_forBody), paths, exactMatch, matches)?;
            ()
        }
        Absyn::Algorithm::ALG_PARFOR {
            iterators: __alg_iterators,
            parforBody: __alg_parforBody,
        } => {
            matches = lookupInForIterators(
                metamodelica::AsArg::as_arg(&__alg_iterators),
                &paths,
                exactMatch,
                &info,
                matches,
            )?;
            matches = lookupInAlgorithmItems(
                metamodelica::AsArg::as_arg(&__alg_parforBody),
                paths,
                exactMatch,
                matches,
            )?;
            ()
        }
        Absyn::Algorithm::ALG_WHILE {
            boolExpr: __alg_boolExpr,
            whileBody: __alg_whileBody,
        } => {
            matches = lookupInExp(__alg_boolExpr.clone(), &paths, exactMatch, &info, matches)?;
            matches = lookupInAlgorithmItems(
                metamodelica::AsArg::as_arg(&__alg_whileBody),
                paths,
                exactMatch,
                matches,
            )?;
            ()
        }
        Absyn::Algorithm::ALG_WHEN_A {
            boolExpr: __alg_boolExpr,
            elseWhenAlgorithmBranch: __alg_elseWhenAlgorithmBranch,
            whenBody: __alg_whenBody,
        } => {
            matches = lookupInExp(__alg_boolExpr.clone(), &paths, exactMatch, &info, matches)?;
            matches = lookupInAlgorithmItems(
                metamodelica::AsArg::as_arg(&__alg_whenBody),
                paths.clone(),
                exactMatch,
                matches,
            )?;
            for mut branch in &*__alg_elseWhenAlgorithmBranch.clone() {
                matches = lookupInExp(Util::tuple21(branch.clone()), &paths, exactMatch, &info, matches)?;
                matches = lookupInAlgorithmItems(&(Util::tuple22(branch.clone())), paths.clone(), exactMatch, matches)?;
            }
            ()
        }
        Absyn::Algorithm::ALG_NORETCALL {
            functionArgs: __alg_functionArgs,
            functionCall: __alg_functionCall,
        } => {
            matches = lookupInCref(__alg_functionCall.clone(), &paths, exactMatch, info.clone(), matches)?;
            matches = lookupInFunctionArgs(
                metamodelica::AsArg::as_arg(&__alg_functionArgs),
                &paths,
                exactMatch,
                &info,
                matches,
            )?;
            ()
        }
        _ => (),
    });
    Ok(matches)
}

fn lookupInExternalDecl(
    mut extDecl: &metamodelica::Ref<Absyn::ExternalDecl>,
    mut paths: metamodelica::Ref<Paths::Paths>,
    mut exactMatch: bool,
    mut info: &SourceInfo,
    mut matches: Matches,
) -> Result<Matches> {
    let mut matches: Matches = matches;
    for mut arg in &*extDecl.args.clone() {
        matches = lookupInExp(arg.clone(), &paths, exactMatch, info, matches)?;
    }
    if (extDecl.annotation_).is_some() {
        matches = lookupInAnnotation(
            &(extDecl.annotation_.clone().ok_or("pattern mismatch")?),
            paths,
            exactMatch,
            matches,
        )?;
    }
    Ok(matches)
}

fn serializeMatches(
    mut groupedMatches: &metamodelica::List<metamodelica::List<Match>>,
    mut prettyPrint: bool,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut json_groups: metamodelica::List<metamodelica::Ref<JSON::JSON>> = metamodelica::nil();
    let mut json_elems: metamodelica::List<metamodelica::Ref<JSON::JSON>>;
    let mut json_group: metamodelica::Ref<JSON::JSON>;
    let mut json_elem: metamodelica::Ref<JSON::JSON>;
    let mut first_match: Match;
    for mut group in &**groupedMatches {
        first_match = (group).head().cloned()?;
        json_group = JSON::addPair(
            &(literal!("filename")),
            &(JSON::makeString(first_match.info.fileName.clone())),
            JSON::emptyListObject(),
        )?;
        json_elems = metamodelica::nil();
        for mut m in &*group.clone() {
            json_elem = JSON::dumpJSONSourceInfo(&m.info, false)?;
            json_elem = JSON::addPair(
                &(literal!("name")),
                &(JSON::makeString(Dump::printComponentRefStr(&m.name)?)),
                json_elem,
            )?;
            json_elem = JSON::addPair(&(literal!("class")), &(JSON::makeString(m.scope.clone())), json_elem)?;
            json_elems = metamodelica::cons(json_elem, json_elems);
        }
        json_group = JSON::addPair(&(literal!("matches")), &(JSON::makeArray(json_elems)), json_group)?;
        json_groups = metamodelica::cons(json_group, json_groups);
    }
    r#str = JSON::toString(&(JSON::makeArray(json_groups)), prettyPrint)?;
    Ok(r#str)
}

fn groupMatches(mut matches: &Matches) -> Result<metamodelica::List<metamodelica::List<Match>>> {
    fn add_match(mut oldMatches: Option<metamodelica::List<Match>>, mut newMatch: Match) -> Result<Matches> {
        let mut outMatches: Matches;
        if (oldMatches).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(oldMatches) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            outMatches = metamodelica::Own::own(__pa0);
            outMatches = metamodelica::cons(newMatch, outMatches);
        } else {
            outMatches = list![newMatch];
        }
        Ok(outMatches)
    }

    let mut outMatches: metamodelica::List<metamodelica::List<Match>>;
    let mut grouped_matches: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::List<Match>>>;
    grouped_matches = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    for mut m in &**matches {
        UnorderedMap::addUpdate(
            m.info.fileName.clone(),
            &({
                let __pe_b1 = m.clone();
                move |__pe_a0| add_match(__pe_a0, __pe_b1.clone())
            }),
            grouped_matches.clone(),
        )?;
    }
    outMatches = UnorderedMap::valueList(grouped_matches);
    outMatches = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut l in (outMatches).into_iter().cloned() {
            let __x = metamodelica::Dangerous::listReverseInPlace(l.clone());
            __acc = cons(__x, __acc);
        }
        __acc
    });
    Ok(outMatches)
}
