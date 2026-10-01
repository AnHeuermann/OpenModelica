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

use crate::Curl;
use crate::Unzip;
use openmodelica_util::Autoconf;
use openmodelica_util::AvlSetString;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Global;
use openmodelica_util::JSON;
use openmodelica_util::SemanticVersion;
use openmodelica_util::Settings;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub mod AvailableLibraries {
    use super::*;
    pub type Key = ArcStr;

    pub type Value = metamodelica::Ref<VersionMap::Tree>;

    pub(crate) fn keyStr(mut inKey: Key) -> ArcStr {
        let mut outString: ArcStr;
        outString = inKey;
        outString
    }

    pub(crate) fn valueStr(mut inValue: Value) -> Result<ArcStr> {
        let mut outString: ArcStr;
        outString = VersionMap::printTreeStr(&inValue)?;
        Ok(outString)
    }

    pub(crate) fn keyCompare(mut inKey1: Key, mut inKey2: Key) -> i32 {
        let mut outResult: i32;
        outResult = stringCompare(&inKey1, &inKey2);
        outResult
    }

    pub type ConflictFunc = std::sync::Arc<dyn ::std::ops::Fn(Value, Value, Key) -> Result<Value> + 'static>;

    /// The binary tree data structure.
    #[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum Tree {
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
    pub use self::Tree::{EMPTY, LEAF, NODE};

    pub type ValueNode = ArcStr;

    pub(crate) fn add(
        mut inTree: metamodelica::Ref<Tree>,
        mut inKey: &Key,
        mut inValue: &Value,
        mut conflictFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<VersionMap::Tree>,
            metamodelica::Ref<VersionMap::Tree>,
            ArcStr,
        ) -> Result<metamodelica::Ref<VersionMap::Tree>>,
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
                        right: crate::PackageManagement::AvailableLibraries::Tree::interned_EMPTY(),
                    });
                } else if key_comp == 1 {
                    outTree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: crate::PackageManagement::AvailableLibraries::Tree::interned_EMPTY(),
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
        mut inValues: &metamodelica::List<(ArcStr, metamodelica::Ref<VersionMap::Tree>)>,
        mut conflictFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<VersionMap::Tree>,
            metamodelica::Ref<VersionMap::Tree>,
            ArcStr,
        ) -> Result<metamodelica::Ref<VersionMap::Tree>>,
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
        mut r#fn: &dyn ::std::ops::Fn(Option<metamodelica::Ref<VersionMap::Tree>>) -> Result<metamodelica::Ref<VersionMap::Tree>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        pub type UpdateFn =
            std::sync::Arc<dyn ::std::ops::Fn(Option<metamodelica::Ref<VersionMap::Tree>>) -> Result<Value> + 'static>;

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
                        right: crate::PackageManagement::AvailableLibraries::Tree::interned_EMPTY(),
                    });
                } else if key_comp == 1 {
                    new_tree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: crate::PackageManagement::AvailableLibraries::Tree::interned_EMPTY(),
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
        mut inFunc: &'__b dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<VersionMap::Tree>, FT) -> Result<FT>,
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
        mut foldFunc: &dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<VersionMap::Tree>, FT) -> Result<(FT, bool)>,
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
        mut foldFunc: &dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<VersionMap::Tree>, FT1, FT2) -> Result<(FT1, FT2)>,
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
        mut func: &dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<VersionMap::Tree>) -> Result<()>,
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
        mut inValues: &metamodelica::List<(ArcStr, metamodelica::Ref<VersionMap::Tree>)>,
        mut conflictFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<VersionMap::Tree>,
            metamodelica::Ref<VersionMap::Tree>,
            ArcStr,
        ) -> Result<metamodelica::Ref<VersionMap::Tree>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        let mut tree: metamodelica::Ref<Tree> = crate::PackageManagement::AvailableLibraries::Tree::interned_EMPTY();
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
    ) -> Option<metamodelica::Ref<VersionMap::Tree>> {
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
            metamodelica::Ref<VersionMap::Tree>,
            metamodelica::Ref<VersionMap::Tree>,
            ArcStr,
        ) -> Result<metamodelica::Ref<VersionMap::Tree>>,
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

    pub fn listKeys<'__b>(
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
        mut lst: metamodelica::List<metamodelica::Ref<VersionMap::Tree>>,
    ) -> metamodelica::List<metamodelica::Ref<VersionMap::Tree>> {
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
        mut inFunc: &dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<VersionMap::Tree>) -> Result<metamodelica::Ref<VersionMap::Tree>>,
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
        mut inFunc: &dyn ::std::ops::Fn(
            ArcStr,
            metamodelica::Ref<VersionMap::Tree>,
            FT,
        ) -> Result<(metamodelica::Ref<VersionMap::Tree>, FT)>,
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
        let mut outTree: metamodelica::Ref<Tree> = crate::PackageManagement::AvailableLibraries::Tree::interned_EMPTY();
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
                __mm_s.push_str(&*valueStr(__inNode_value.clone())?);
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
                __mm_s.push_str(&*valueStr(__inNode_value.clone())?);
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
                node = setTreeLeftRight(outNode, __outNode_left.clone(), crate::PackageManagement::AvailableLibraries::Tree::interned_EMPTY())?;
                setTreeLeftRight(child.clone(), node, crate::PackageManagement::AvailableLibraries::Tree::interned_EMPTY())?
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
                node = setTreeLeftRight(outNode, crate::PackageManagement::AvailableLibraries::Tree::interned_EMPTY(), __outNode_right.clone())?;
                setTreeLeftRight(child.clone(), crate::PackageManagement::AvailableLibraries::Tree::interned_EMPTY(), node)?
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
        mut lst: metamodelica::List<(ArcStr, metamodelica::Ref<VersionMap::Tree>)>,
    ) -> metamodelica::List<(ArcStr, metamodelica::Ref<VersionMap::Tree>)> {
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
            add(tree.clone(), key, value, &move |__a0: metamodelica::Ref<
                VersionMap::Tree,
            >,
                                                 __a1: metamodelica::Ref<
                VersionMap::Tree,
            >,
                                                 __a2: ArcStr|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(addConflictReplace(__a0, &__a1, &__a2))
            })?;
        Ok(outTree)
    }
}

pub mod VersionMap {
    use super::*;
    pub type Key = SemanticVersion::Version;

    pub type Value = ArcStr;

    pub(crate) fn keyStr(mut inKey: Key) -> ArcStr {
        let mut outString: ArcStr;
        outString = SemanticVersion::toString(&inKey);
        outString
    }

    pub(crate) fn valueStr(mut inValue: Value) -> ArcStr {
        let mut outString: ArcStr;
        outString = inValue;
        outString
    }

    pub(crate) fn keyCompare(mut inKey1: Key, mut inKey2: Key) -> Result<i32> {
        let mut outResult: i32;
        outResult = SemanticVersion::compare(&inKey1, &inKey2, true, true)?;
        Ok(outResult)
    }

    pub type ConflictFunc = std::sync::Arc<dyn ::std::ops::Fn(Value, Value, Key) -> Result<Value> + 'static>;

    /// The binary tree data structure.
    #[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum Tree {
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
    pub use self::Tree::{EMPTY, LEAF, NODE};

    pub type ValueNode = SemanticVersion::Version;

    pub(crate) fn add(
        mut inTree: metamodelica::Ref<Tree>,
        mut inKey: &Key,
        mut inValue: &Value,
        mut conflictFunc: &dyn ::std::ops::Fn(ArcStr, ArcStr, SemanticVersion::Version) -> Result<ArcStr>,
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
                key_comp = keyCompare(inKey.clone(), key.clone())?;
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
                    if !(referenceEq(&*(var_field!((*tree).value, Tree::NODE).clone()), &*(value.clone()))) {
                        assign_variant_field!(tree => Tree::NODE; value = value);
                    }
                }
                if (key_comp == 0) { tree } else { balance(tree)? }
            }
            Tree::LEAF { key: __tree_key, .. } => {
                let mut value: Value;
                let mut key_comp: i32;
                let mut outTree: metamodelica::Ref<Tree>;
                key_comp = keyCompare(inKey.clone(), __tree_key.clone())?;
                if key_comp == -1 {
                    outTree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: metamodelica::Ref::new(Tree::LEAF {
                            key: inKey.clone(),
                            value: inValue.clone(),
                        }),
                        right: crate::PackageManagement::VersionMap::Tree::interned_EMPTY(),
                    });
                } else if key_comp == 1 {
                    outTree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: crate::PackageManagement::VersionMap::Tree::interned_EMPTY(),
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
                    if !(referenceEq(&*(var_field!((*tree).value, Tree::LEAF).clone()), &*(value.clone()))) {
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
        mut inValues: &metamodelica::List<(SemanticVersion::Version, ArcStr)>,
        mut conflictFunc: &dyn ::std::ops::Fn(ArcStr, ArcStr, SemanticVersion::Version) -> Result<ArcStr>,
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
        mut r#fn: &dyn ::std::ops::Fn(Option<ArcStr>) -> Result<ArcStr>,
    ) -> Result<metamodelica::Ref<Tree>> {
        pub type UpdateFn = std::sync::Arc<dyn ::std::ops::Fn(Option<ArcStr>) -> Result<Value> + 'static>;

        let mut tree: metamodelica::Ref<Tree> = tree;
        let mut key_comp: i32;
        let mut new_tree: metamodelica::Ref<Tree>;
        tree = (match &*tree {
            Tree::EMPTY { .. } => metamodelica::Ref::new(Tree::LEAF {
                key: key.clone(),
                value: r#fn(None)?,
            }),
            Tree::NODE { key: __tree_key, .. } => {
                key_comp = keyCompare(key.clone(), __tree_key.clone())?;
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
                key_comp = keyCompare(key.clone(), __tree_key.clone())?;
                if key_comp == -1 {
                    new_tree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: metamodelica::Ref::new(Tree::LEAF {
                            key: key.clone(),
                            value: r#fn(None)?,
                        }),
                        right: crate::PackageManagement::VersionMap::Tree::interned_EMPTY(),
                    });
                } else if key_comp == 1 {
                    new_tree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: crate::PackageManagement::VersionMap::Tree::interned_EMPTY(),
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
        mut inFunc: &'__b dyn ::std::ops::Fn(SemanticVersion::Version, ArcStr, FT) -> Result<FT>,
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
        mut foldFunc: &dyn ::std::ops::Fn(SemanticVersion::Version, ArcStr, FT) -> Result<(FT, bool)>,
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
        mut foldFunc: &dyn ::std::ops::Fn(SemanticVersion::Version, ArcStr, FT1, FT2) -> Result<(FT1, FT2)>,
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
        mut func: &dyn ::std::ops::Fn(SemanticVersion::Version, ArcStr) -> Result<()>,
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
        mut inValues: &metamodelica::List<(SemanticVersion::Version, ArcStr)>,
        mut conflictFunc: &dyn ::std::ops::Fn(ArcStr, ArcStr, SemanticVersion::Version) -> Result<ArcStr>,
    ) -> Result<metamodelica::Ref<Tree>> {
        let mut tree: metamodelica::Ref<Tree> = crate::PackageManagement::VersionMap::Tree::interned_EMPTY();
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
        value = (::match_deref::match_deref! { match &((keyCompare(key.clone(), k)?, tree.clone())) {
            (0, Deref @ Tree::LEAF { .. }) => var_field!((**tree).value, Tree::LEAF).clone(),
            (0, Deref @ Tree::NODE { .. }) => var_field!((**tree).value, Tree::NODE).clone(),
            (1, Deref @ Tree::NODE { .. }) => get(var_field!((**tree).right, Tree::NODE), key)?,
            ((-1), Deref @ Tree::NODE { .. }) => get(var_field!((**tree).left, Tree::NODE), key)?,
            _ => return Err("match: no arm matched"),
        } });
        Ok(value)
    }

    pub(crate) fn getOpt<'__b>(mut tree: &'__b metamodelica::Ref<Tree>, mut key: Key) -> Result<Option<ArcStr>> {
        '__tco: loop {
            let mut k: Key;
            k = (match &**tree {
                Tree::NODE { .. } => var_field!((**tree).key, Tree::NODE).clone(),
                Tree::LEAF { .. } => var_field!((**tree).key, Tree::LEAF).clone(),
                _ => key.clone(),
            });
            ::match_deref::match_deref! { match &((keyCompare(key.clone(), k)?, tree.clone())) {
                (0, Deref @ Tree::LEAF { .. }) => return Ok(Some(var_field!((**tree).value, Tree::LEAF).clone())),
                (0, Deref @ Tree::NODE { .. }) => return Ok(Some(var_field!((**tree).value, Tree::NODE).clone())),
                (1, Deref @ Tree::NODE { .. }) => { (tree, key) = (var_field!((**tree).right, Tree::NODE), key); continue '__tco; },
                ((-1), Deref @ Tree::NODE { .. }) => { (tree, key) = (var_field!((**tree).left, Tree::NODE), key); continue '__tco; },
                _ => return Ok(None),
                _ => return Err("match: no arm matched"),
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
        key_comp = keyCompare(inKey.clone(), key)?;
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
        mut conflictFunc: &'__b dyn ::std::ops::Fn(ArcStr, ArcStr, SemanticVersion::Version) -> Result<ArcStr>,
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
        mut lst: metamodelica::List<SemanticVersion::Version>,
    ) -> metamodelica::List<SemanticVersion::Version> {
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
        mut lst: metamodelica::List<SemanticVersion::Version>,
    ) -> metamodelica::List<SemanticVersion::Version> {
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
        mut lst: metamodelica::List<ArcStr>,
    ) -> metamodelica::List<ArcStr> {
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
        mut inFunc: &dyn ::std::ops::Fn(SemanticVersion::Version, ArcStr) -> Result<ArcStr>,
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
                    || !(referenceEq(&*(value.clone()), &*(new_value.clone())))
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
                if !(referenceEq(&*(value.clone()), &*(new_value.clone()))) {
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
        mut inFunc: &dyn ::std::ops::Fn(SemanticVersion::Version, ArcStr, FT) -> Result<(ArcStr, FT)>,
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
                    || !(referenceEq(&*(value.clone()), &*(new_value.clone())))
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
                if !(referenceEq(&*(value.clone()), &*(new_value.clone()))) {
                    assign_variant_field!(outTree => Tree::LEAF; value = new_value);
                }
                outTree
            }
            _ => inTree,
        });
        Ok((outTree, outResult))
    }

    pub(crate) fn new() -> metamodelica::Ref<Tree> {
        let mut outTree: metamodelica::Ref<Tree> = crate::PackageManagement::VersionMap::Tree::interned_EMPTY();
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
                node = setTreeLeftRight(outNode, __outNode_left.clone(), crate::PackageManagement::VersionMap::Tree::interned_EMPTY())?;
                setTreeLeftRight(child.clone(), node, crate::PackageManagement::VersionMap::Tree::interned_EMPTY())?
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
                node = setTreeLeftRight(outNode, crate::PackageManagement::VersionMap::Tree::interned_EMPTY(), __outNode_right.clone())?;
                setTreeLeftRight(child.clone(), crate::PackageManagement::VersionMap::Tree::interned_EMPTY(), node)?
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
        mut lst: metamodelica::List<(SemanticVersion::Version, ArcStr)>,
    ) -> metamodelica::List<(SemanticVersion::Version, ArcStr)> {
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
            add(tree.clone(), key, value, &move |__a0: ArcStr,
                                                 __a1: ArcStr,
                                                 __a2: SemanticVersion::Version|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(addConflictReplace(__a0, &__a1, &__a2))
            })?;
        Ok(outTree)
    }
}

pub(crate) const metaDataFileName: &'static str = "openmodelica.metadata.json";

pub fn getInstalledLibraries() -> Result<metamodelica::Ref<AvailableLibraries::Tree>> {
    let mut tree: metamodelica::Ref<AvailableLibraries::Tree>;
    let mut mp: ArcStr;
    let mut gd: ArcStr;
    let mut first: ArcStr;
    let mut ver: ArcStr;
    let mut lib: ArcStr;
    let mut mps: metamodelica::List<ArcStr>;
    let mut files: metamodelica::List<ArcStr>;
    let mut dirs: metamodelica::List<ArcStr>;
    let mut rest: metamodelica::List<ArcStr>;
    let mut versions: metamodelica::Ref<VersionMap::Tree>;
    mp = Settings::getModelicaPath(Testsuite::isRunning()?)?;
    gd = arcstr::literal!(Autoconf::groupDelimiter);
    mps = System::strtok(mp, gd);
    tree = AvailableLibraries::new();
    files = metamodelica::nil();
    dirs = metamodelica::nil();
    for mut mp in &*mps {
        let mut mp = mp.clone();
        files = listAppend(
            ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut file in (System::moFiles(mp.clone())).into_iter().cloned() {
                    let __x = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*mp);
                        __mm_s.push_str(&*literal!("/"));
                        __mm_s.push_str(&*file);
                        ArcStr::from(__mm_s)
                    };
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            files,
        );
        dirs = listAppend(
            ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut dir in (getLibrarySubdirectories(mp.clone())).into_iter().cloned() {
                    let __x = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*mp);
                        __mm_s.push_str(&*literal!("/"));
                        __mm_s.push_str(&*dir);
                        ArcStr::from(__mm_s)
                    };
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            dirs,
        );
    }
    for mut path in &*listAppend(files, dirs) {
        lib = System::basename(path.clone());
        if StringUtil::endsWith(lib.clone(), literal!(".mo")) {
            lib = Util::removeLast3Char(lib)?;
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(System::strtok(lib, literal!(" "))) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        first = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        ver = stringDelimitList(rest, literal!(" "));
        versions = if (AvailableLibraries::hasKey(tree.clone(), first.clone())?) {
            AvailableLibraries::get(&tree, first.clone())?
        } else {
            VersionMap::new()
        };
        versions = VersionMap::add(
            versions,
            &(SemanticVersion::parse(ver, false)?),
            metamodelica::AsArg::as_arg(&path),
            &move |__a0: ArcStr, __a1: ArcStr, __a2: SemanticVersion::Version| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(VersionMap::addConflictReplace(__a0, &__a1, &__a2))
            },
        )?;
        tree = AvailableLibraries::add(tree, &first, &versions, &move |__a0: metamodelica::Ref<
            VersionMap::Tree,
        >,
                                                                       __a1: metamodelica::Ref<
            VersionMap::Tree,
        >,
                                                                       __a2: ArcStr|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(AvailableLibraries::addConflictReplace(__a0, &__a1, &__a2))
        })?;
    }
    Ok(tree)
}

pub fn getInstalledLibraryVersions(mut libraryName: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut libraryVersions: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut tree: metamodelica::Ref<AvailableLibraries::Tree>;
    let mut versionTree: metamodelica::Ref<VersionMap::Tree>;
    let mut versions: metamodelica::List<SemanticVersion::Version> = metamodelica::nil();
    let mut versionStr: ArcStr;
    tree = getInstalledLibraries()?;
    versionTree = AvailableLibraries::get(&tree, libraryName)?;
    versions = VersionMap::listKeys(&versionTree, metamodelica::nil());
    for mut version in &*versions {
        versionStr = VersionMap::keyStr(version.clone());
        if stringCompare(&versionStr, &(literal!(""))) > 0 {
            libraryVersions = metamodelica::cons(versionStr, libraryVersions);
        }
    }
    Ok(libraryVersions)
}

pub(crate) fn getLibrarySubdirectories(mut inPath: ArcStr) -> metamodelica::List<ArcStr> {
    let mut outSubdirectories: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut allSubdirectories: metamodelica::List<ArcStr> = System::subDirectories(inPath.clone());
    let mut pd: ArcStr = arcstr::literal!(Autoconf::pathDelimiter);
    for mut dir in &*allSubdirectories {
        if System::regularFileExists({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*inPath);
            __mm_s.push_str(&*pd);
            __mm_s.push_str(&*dir);
            __mm_s.push_str(&*pd);
            __mm_s.push_str(&*literal!("package.mo"));
            ArcStr::from(__mm_s)
        }) {
            outSubdirectories = metamodelica::cons(dir.clone(), outSubdirectories);
        }
    }
    outSubdirectories
}

pub(crate) fn providesExpectedVersion(
    mut version: ArcStr,
    mut provides: &metamodelica::Ref<JSON::JSON>,
    mut wantedVersion: &SemanticVersion::Version,
) -> Result<bool> {
    let mut matches: bool;
    let mut providedVersions: metamodelica::List<ArcStr>;
    let mut r#str: ArcStr;
    let mut thisVersion: SemanticVersion::Version;
    let () = (::match_deref::match_deref! { match &(wantedVersion) {
        SemanticVersion::Version::NONSEMVER { version: r#str } if (metamodelica::stringEq(&r#str, &(literal!("default"))) || metamodelica::stringEq(&r#str, &(literal!("")))) => {
            matches = true;
            return Ok(matches);
            return Err("fail")
        },
        SemanticVersion::Version::SEMVER { major: 0, minor: 0, patch: 0, prerelease: Deref @ metamodelica::ListNode::Cons { head: Deref @ "default", tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            matches = true;
            return Ok(matches);
            return Err("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    providedVersions = JSON::getStringList(provides)?;
    matches = false;
    for mut v in &*metamodelica::cons(version, providedVersions) {
        thisVersion = SemanticVersion::parse(v.clone(), true)?;
        if SemanticVersion::compare(
            &thisVersion,
            wantedVersion,
            SemanticVersion::isPrerelease(wantedVersion) && SemanticVersion::isPrerelease(wantedVersion),
            false,
        )? == 0
        {
            matches = true;
            return Ok(matches);
        }
    }
    Ok(matches)
}

pub(crate) static supportLevels: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        literal!("fullSupport"),
        literal!("support"),
        literal!("experimental"),
        literal!("obsolete"),
        literal!("unknown"),
        literal!("noSupport")
    ]
});

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum SupportLevel {
    noSupport = 1,
    unknown = 2,
    obsolete = 3,
    experimental = 4,
    support = 5,
    fullSupport = 6,
}
impl PartialOrd for SupportLevel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SupportLevel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for SupportLevel {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub(crate) fn getSupportLevel(mut obj: &metamodelica::Ref<JSON::JSON>) -> Result<SupportLevel> {
    let mut support: SupportLevel;
    support = (::match_deref::match_deref! { match obj {
        Deref @ JSON::STRING { r#str: Deref @ "fullSupport" } => SupportLevel::fullSupport.clone(),
        Deref @ JSON::STRING { r#str: Deref @ "support" } => SupportLevel::support.clone(),
        Deref @ JSON::STRING { r#str: Deref @ "experimental" } => SupportLevel::experimental.clone(),
        Deref @ JSON::STRING { r#str: Deref @ "obsolete" } => SupportLevel::obsolete.clone(),
        Deref @ JSON::STRING { r#str: Deref @ "unknown" } => SupportLevel::unknown.clone(),
        Deref @ JSON::STRING { r#str: Deref @ "noSupport" } => SupportLevel::noSupport.clone(),
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unknown support level ")); __mm_s.push_str(&*JSON::toString(obj, false)?); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("Script/PackageManagement.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(support)
}

pub(crate) fn compareVersionsAndSupportLevel(
    mut x1: &(ArcStr, SemanticVersion::Version, SupportLevel),
    mut x2: &(ArcStr, SemanticVersion::Version, SupportLevel),
) -> Result<bool> {
    let mut c: bool;
    let mut s1: SupportLevel;
    let mut s2: SupportLevel;
    let mut v1: SemanticVersion::Version;
    let mut v2: SemanticVersion::Version;
    (_, v1, s1) = x1.clone();
    (_, v2, s2) = x2.clone();
    if s1 < s2 {
        c = true;
        return Ok(c);
    } else if s1 > s2 {
        c = false;
        return Ok(c);
    }
    if SemanticVersion::isPrerelease(&v1) != SemanticVersion::isPrerelease(&v2) {
        c = SemanticVersion::isPrerelease(&v2);
        return Ok(c);
    }
    c = SemanticVersion::compare(&v1, &v2, true, true)? < 0;
    Ok(c)
}

pub fn updateIndex() -> Result<bool> {
    let mut success: bool;
    let mut userLibraries: ArcStr;
    let mut packageIndex: ArcStr;
    let url: ArcStr = literal!("https://libraries.openmodelica.org/index/v1/index.json");
    userLibraries = getUserLibraryPath()?;
    Util::createDirectoryTree(userLibraries.clone());
    packageIndex = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*userLibraries);
        __mm_s.push_str(&*literal!("index.json"));
        ArcStr::from(__mm_s)
    };
    if !(Curl::multiDownload(list![(list![url.clone()], packageIndex.clone())], Config::noProc()?)?) {
        Error::addMessage(Error::ERROR_PKG_INDEX_FAILED_DOWNLOAD.clone(), list![url, packageIndex])?;
        success = false;
    } else {
        Error::addSourceMessage(
            &(Error::NOTIFY_PKG_INDEX_DOWNLOAD.clone()),
            list![url],
            &(makeSourceInfo(getIndexPath()?)),
        )?;
        success = true;
    }
    {
        let __v = None;
        openmodelica_util::Globals::packageIndexCacheIndex.with(|__root| *__root.borrow_mut() = __v)
    };
    Ok(success)
}

pub fn upgradeInstalledPackages(mut installNewestVersions: bool) -> Result<bool> {
    let mut success: bool;
    let mut installedLibraries: metamodelica::Ref<AvailableLibraries::Tree>;
    let mut versions: metamodelica::Ref<VersionMap::Tree>;
    success = true;
    installedLibraries = getInstalledLibraries()?;
    for mut pkg in &*AvailableLibraries::listKeys(&installedLibraries, metamodelica::nil()) {
        versions = AvailableLibraries::get(&installedLibraries, pkg.clone())?;
        for mut version in &*VersionMap::listKeys(&versions, metamodelica::nil()) {
            success = success
                && installPackage(
                    pkg.clone(),
                    SemanticVersion::toString(metamodelica::AsArg::as_arg(&version)),
                    true,
                    false,
                )?;
        }
        if installNewestVersions {
            success = success && installPackage(pkg.clone(), literal!(""), false, false)?;
        }
    }
    Ok(success)
}

pub(crate) fn getPackageIndex(mut printError: bool) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut obj: metamodelica::Ref<JSON::JSON>;
    let mut userLibraries: ArcStr;
    let mut packageIndex: ArcStr;
    let mut gd: ArcStr;
    let mut mp: ArcStr;
    let mut mps: metamodelica::List<ArcStr>;
    if '__try0: {
        obj = unwrap_break_err!(openmodelica_util::Globals::packageIndexCacheIndex.with(|__root| __root.borrow().clone()).ok_or_else(|| "getGlobalRoot: empty slot packageIndexCacheIndex"), '__try0);
        return Ok(obj);
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    mp = Settings::getModelicaPath(Testsuite::isRunning()?)?;
    gd = arcstr::literal!(Autoconf::groupDelimiter);
    mps = System::strtok(mp.clone(), gd);
    userLibraries = getUserLibraryPath()?;
    packageIndex = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*userLibraries);
        __mm_s.push_str(&*literal!("index.json"));
        ArcStr::from(__mm_s)
    };
    obj = JSON::emptyObject();
    if !(listMember(userLibraries.clone(), mps.clone()))
        && !(listMember(Util::removeLastNChar(userLibraries.clone(), 1)?, mps))
    {
        if printError {
            Error::addMessage(Error::ERROR_PKG_INDEX_NOT_ON_PATH.clone(), list![mp, userLibraries])?;
        }
        return Ok(obj);
    }
    if !(System::regularFileExists(packageIndex.clone())) {
        if !(updateIndex()?) {
            return Ok(obj);
        }
    }
    if '__try1: {
        obj = unwrap_break_err!(JSON::parseFile(packageIndex.clone()), '__try1);
        {
            let __v = Some(obj.clone());
            openmodelica_util::Globals::packageIndexCacheIndex.with(|__root| *__root.borrow_mut() = __v)
        };
        Ok::<(), &'static str>(())
    }
    .is_err()
    {
        Error::addSourceMessage(
            &(Error::ERROR_PKG_INDEX_NOT_PARSED.clone()),
            list![packageIndex.clone()],
            &(makeSourceInfo(getIndexPath()?)),
        )?;
    }
    Ok(obj)
}

pub(crate) fn getAllProvidedVersionsForLibrary(mut lib: ArcStr, mut printError: bool) -> metamodelica::List<ArcStr> {
    let mut result: metamodelica::List<ArcStr>;
    let mut obj: metamodelica::Ref<JSON::JSON>;
    let mut libobject: metamodelica::Ref<JSON::JSON>;
    let mut vers: metamodelica::Ref<JSON::JSON>;
    let mut provides: metamodelica::Ref<JSON::JSON>;
    let mut tree: metamodelica::Ref<AvlSetString::Tree>;
    result = metamodelica::nil();
    tree = AvlSetString::new();
    match '__try0: {
        obj = unwrap_break_err!(getPackageIndex(printError), '__try0);
        libobject = unwrap_break_err!(JSON::get(&(unwrap_break_err!(JSON::get(&obj, literal!("libs")), '__try0)), lib.clone()), '__try0);
        vers = unwrap_break_err!(JSON::get(&libobject, literal!("versions")), '__try0);
        for mut version in &*unwrap_break_err!(JSON::getKeys(&vers), '__try0) {
            tree = unwrap_break_err!(AvlSetString::add(tree.clone(), metamodelica::AsArg::as_arg(&version)), '__try0);
            provides = unwrap_break_err!(JSON::getOrDefault(&(unwrap_break_err!(JSON::get(&vers, version.clone()), '__try0)), literal!("provides"), JSON::emptyArray(0)), '__try0);
            for mut i in 1..=JSON::size(&provides) {
                tree = unwrap_break_err!(AvlSetString::add(tree.clone(), &(unwrap_break_err!(JSON::getString(&(unwrap_break_err!(JSON::at(&provides, i), '__try0))), '__try0))), '__try0);
            }
        }
        result = AvlSetString::listKeys(&tree, metamodelica::nil());
        Ok::<_, &'static str>((libobject.clone(), obj.clone(), result.clone(), vers.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            libobject = __try0_o0;
            obj = __try0_o1;
            result = __try0_o2;
            vers = __try0_o3;
        }
        Err(_) => {
            return result;
        }
    }
    result
}

pub fn versionsThatProvideTheWanted(
    mut id: ArcStr,
    mut version: ArcStr,
    mut printError: bool,
) -> metamodelica::List<ArcStr> {
    let mut result: metamodelica::List<ArcStr>;
    let mut obj: metamodelica::Ref<JSON::JSON>;
    let mut libobject: metamodelica::Ref<JSON::JSON>;
    let mut vers: metamodelica::Ref<JSON::JSON>;
    let mut wantedVersion: SemanticVersion::Version;
    result = metamodelica::nil();
    match '__try0: {
        obj = unwrap_break_err!(getPackageIndex(printError), '__try0);
        libobject = unwrap_break_err!(JSON::get(&(unwrap_break_err!(JSON::get(&obj, literal!("libs")), '__try0)), id.clone()), '__try0);
        vers = unwrap_break_err!(JSON::get(&libobject, literal!("versions")), '__try0);
        wantedVersion = unwrap_break_err!(SemanticVersion::parse(version.clone(), true), '__try0);
        result = unwrap_break_err!(List::map(unwrap_break_err!(List::sort(({
        let mut __acc: metamodelica::List<(ArcStr, SemanticVersion::Version, SupportLevel)> = metamodelica::nil();
        for mut version in (unwrap_break_err!(JSON::getKeys(&vers), '__try0)).into_iter().cloned() {
            if !(unwrap_break_err!(providesExpectedVersion(version.clone(), &(unwrap_break_err!(JSON::getOrDefault(&(unwrap_break_err!(JSON::get(&vers, version.clone()), '__try0)), literal!("provides"), JSON::emptyArray(0)), '__try0)), &wantedVersion), '__try0)) { continue; }
            let __x = (version.clone(), unwrap_break_err!(SemanticVersion::parse(version.clone(), true), '__try0), unwrap_break_err!(getSupportLevel(&(unwrap_break_err!(JSON::get(&(unwrap_break_err!(JSON::get(&vers, version.clone()), '__try0)), literal!("support")), '__try0))), '__try0));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (std::sync::Arc::new(move |__a0: (ArcStr, SemanticVersion::Version, SupportLevel), __a1: (ArcStr, SemanticVersion::Version, SupportLevel)| compareVersionsAndSupportLevel(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, SemanticVersion::Version, SupportLevel), (ArcStr, SemanticVersion::Version, SupportLevel)) -> Result<bool> + 'static>)), '__try0), &fnptr!(Util::tuple31, _)), '__try0);
        Ok::<_, &'static str>((
            libobject.clone(),
            obj.clone(),
            result.clone(),
            vers.clone(),
            wantedVersion.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
            libobject = __try0_o0;
            obj = __try0_o1;
            result = __try0_o2;
            vers = __try0_o3;
            wantedVersion = __try0_o4;
        }
        Err(_) => {
            return result;
        }
    }
    result
}

pub fn versionsThatConvertFromTheWanted(
    mut id: ArcStr,
    mut version: ArcStr,
    mut printError: bool,
) -> metamodelica::List<ArcStr> {
    let mut result: metamodelica::List<ArcStr>;
    let mut obj: metamodelica::Ref<JSON::JSON>;
    let mut libobject: metamodelica::Ref<JSON::JSON>;
    let mut vers: metamodelica::Ref<JSON::JSON>;
    let mut wantedVersion: SemanticVersion::Version;
    let mut convertVersion: SemanticVersion::Version;
    let mut convertFrom: metamodelica::Ref<JSON::JSON>;
    let mut versionStr: ArcStr;
    result = metamodelica::nil();
    match '__try0: {
        obj = unwrap_break_err!(getPackageIndex(printError), '__try0);
        libobject = unwrap_break_err!(JSON::get(&(unwrap_break_err!(JSON::get(&obj, literal!("libs")), '__try0)), id.clone()), '__try0);
        vers = unwrap_break_err!(JSON::get(&libobject, literal!("versions")), '__try0);
        wantedVersion = unwrap_break_err!(SemanticVersion::parse(version.clone(), true), '__try0);
        for mut v in &*unwrap_break_err!(JSON::getKeys(&vers), '__try0) {
            convertFrom = unwrap_break_err!(JSON::getOrDefault(&(unwrap_break_err!(JSON::get(&vers, v.clone()), '__try0)), literal!("convertFromVersion"), JSON::emptyArray(0)), '__try0);
            for mut i in 1..=JSON::size(&convertFrom) {
                let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(JSON::at(&convertFrom, i), '__try0)) {
                    Deref @ JSON::STRING { r#str: __pa1 } => __pa1.clone(),
                    _ => break '__try0 Err::<_, _>("pattern mismatch"),
                } };
                versionStr = metamodelica::Own::own(__pa1);
                convertVersion = unwrap_break_err!(SemanticVersion::parse(versionStr.clone(), true), '__try0);
                if unwrap_break_err!(SemanticVersion::compare(&wantedVersion, &convertVersion, true, false), '__try0)
                    == 0
                {
                    result = metamodelica::cons(v.clone(), result.clone());
                    continue;
                }
            }
        }
        Ok::<_, &'static str>((libobject.clone(), obj.clone(), vers.clone(), wantedVersion.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            libobject = __try0_o0;
            obj = __try0_o1;
            vers = __try0_o2;
            wantedVersion = __try0_o3;
        }
        Err(_) => {
            return result;
        }
    }
    result
}

pub fn versionsThatConvertToTheWanted(
    mut id: ArcStr,
    mut version: ArcStr,
    mut printError: bool,
) -> metamodelica::List<ArcStr> {
    let mut result: metamodelica::List<ArcStr>;
    let mut obj: metamodelica::Ref<JSON::JSON>;
    let mut libobject: metamodelica::Ref<JSON::JSON>;
    let mut vers: metamodelica::Ref<JSON::JSON>;
    let mut wantedVersion: SemanticVersion::Version;
    let mut libVersion: SemanticVersion::Version;
    result = metamodelica::nil();
    match '__try0: {
        obj = unwrap_break_err!(getPackageIndex(printError), '__try0);
        libobject = unwrap_break_err!(JSON::get(&(unwrap_break_err!(JSON::get(&obj, literal!("libs")), '__try0)), id.clone()), '__try0);
        vers = unwrap_break_err!(JSON::get(&libobject, literal!("versions")), '__try0);
        wantedVersion = unwrap_break_err!(SemanticVersion::parse(version.clone(), true), '__try0);
        for mut v in &*unwrap_break_err!(JSON::getKeys(&vers), '__try0) {
            libVersion = unwrap_break_err!(SemanticVersion::parse(v.clone(), true), '__try0);
            if unwrap_break_err!(SemanticVersion::compare(&wantedVersion, &libVersion, true, false), '__try0) == 0 {
                result = unwrap_break_err!(JSON::getStringList(&(unwrap_break_err!(JSON::get(&(unwrap_break_err!(JSON::get(&vers, v.clone()), '__try0)), literal!("convertFromVersion")), '__try0))), '__try0);
                return result;
            }
        }
        Ok::<_, &'static str>((libobject.clone(), obj.clone(), vers.clone(), wantedVersion.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            libobject = __try0_o0;
            obj = __try0_o1;
            vers = __try0_o2;
            wantedVersion = __try0_o3;
        }
        Err(_) => {
            return result;
        }
    }
    result
}

pub fn installPackage(
    mut pkg: ArcStr,
    mut version: ArcStr,
    mut exactMatch: bool,
    mut skipDownload: bool,
) -> Result<bool> {
    let mut success: bool;
    let mut packageList: metamodelica::List<PackageInstallInfo>;
    let mut packagesToInstall: metamodelica::List<PackageInstallInfo>;
    let mut urlPathList: metamodelica::List<(metamodelica::List<ArcStr>, ArcStr)>;
    let mut urlPathListToDownload: metamodelica::List<(metamodelica::List<ArcStr>, ArcStr)>;
    let mut path: ArcStr;
    let mut destPath: ArcStr;
    let mut destPathPkgMo: ArcStr;
    let mut destPathPkgInfo: ArcStr;
    let mut oldSha: ArcStr;
    let mut dirOfPath: ArcStr;
    let mut expectedLocation: ArcStr;
    let mut cachePath: ArcStr = getCachePath()?;
    let mut installCachePath: ArcStr = getInstallationCachePath()?;
    let mut curCachePath: ArcStr;
    let mut mirrors: metamodelica::List<ArcStr>;
    (success, packageList) = installPackageWork(pkg.clone(), version.clone(), exactMatch, false, metamodelica::nil())?;
    for mut p in &*packageList {
        if metamodelica::stringEq(&p.pkg, &pkg) && !(p.needsInstall.clone()) {
            if metamodelica::stringEq(&version, &(SemanticVersion::toString(&p.version))) {
                Error::addSourceMessage(
                    &(Error::NOTIFY_PKG_ALREADY_INSTALLED.clone()),
                    list![pkg.clone(), SemanticVersion::toString(&p.version)],
                    &(makeSourceInfo(p.path.clone())),
                )?;
            } else {
                Error::addSourceMessage(
                    &(Error::NOTIFY_PKG_NO_INSTALL.clone()),
                    list![pkg.clone(), version.clone(), SemanticVersion::toString(&p.version)],
                    &(makeSourceInfo(p.path.clone())),
                )?;
            }
        }
    }
    packagesToInstall = ({
        let mut __acc: metamodelica::List<PackageInstallInfo> = metamodelica::nil();
        for mut p in (packageList).into_iter().cloned() {
            if !(p.needsInstall.clone()) {
                continue;
            }
            let __x = p.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    for mut pack in &*packagesToInstall {
        Util::createDirectoryTree(cachePath.clone());
    }
    if !(skipDownload) {
        mirrors = getMirrors()?;
        urlPathList = List::sort(
            ({
                let mut __acc: metamodelica::List<(metamodelica::List<ArcStr>, ArcStr)> = metamodelica::nil();
                for mut p in (packagesToInstall.clone()).into_iter().cloned() {
                    let __x = (
                        getAllUrls(p.urlToZipFile.clone(), &mirrors)?,
                        if (System::regularFileExists({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*installCachePath);
                            __mm_s.push_str(&*System::basename(p.urlToZipFile.clone()));
                            ArcStr::from(__mm_s)
                        })) {
                            {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*installCachePath);
                                __mm_s.push_str(&*System::basename(p.urlToZipFile.clone()));
                                ArcStr::from(__mm_s)
                            }
                        } else {
                            {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*cachePath);
                                __mm_s.push_str(&*System::basename(p.urlToZipFile.clone()));
                                ArcStr::from(__mm_s)
                            }
                        },
                    );
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            (std::sync::Arc::new(
                move |__a0: (metamodelica::List<ArcStr>, ArcStr), __a1: (metamodelica::List<ArcStr>, ArcStr)| {
                    compareUrlBool(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            (metamodelica::List<ArcStr>, ArcStr),
                            (metamodelica::List<ArcStr>, ArcStr),
                        ) -> Result<bool>
                        + 'static,
                >),
        )?;
        urlPathList = List::unique(&urlPathList);
        urlPathListToDownload = ({
            let mut __acc: metamodelica::List<(metamodelica::List<ArcStr>, ArcStr)> = metamodelica::nil();
            for mut tpl in (urlPathList).into_iter().cloned() {
                if !(!(System::regularFileExists(Util::tuple22(tpl.clone())))) {
                    continue;
                }
                let __x = tpl.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        if !(Curl::multiDownload(urlPathListToDownload, Config::noProc()?)?) {
            return Err("fail");
        }
    }
    for mut pack in &*packagesToInstall {
        destPath = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*getUserLibraryPath()?);
            __mm_s.push_str(&*pack.pkg);
            __mm_s.push_str(&*literal!(" "));
            __mm_s.push_str(&*SemanticVersion::toString(&pack.version));
            ArcStr::from(__mm_s)
        };
        System::removeDirectory(destPath.clone());
        System::createDirectory(destPath.clone());
        destPathPkgMo = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*destPath);
            __mm_s.push_str(&*literal!("/package.mo"));
            ArcStr::from(__mm_s)
        };
        destPathPkgInfo = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*destPath);
            __mm_s.push_str(&*literal!("/"));
            __mm_s.push_str(&*arcstr::literal!(metaDataFileName));
            ArcStr::from(__mm_s)
        };
        oldSha = literal!("");
        if System::regularFileExists(destPathPkgInfo.clone()) {
            if '__try0: {
                oldSha = unwrap_break_err!(getShaOrZipfile(&(unwrap_break_err!(JSON::parseFile(destPathPkgInfo.clone()), '__try0))), '__try0);
                Ok::<(), &'static str>(())
            }.is_err() {
            }
        }
        curCachePath = if (System::regularFileExists({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*installCachePath);
            __mm_s.push_str(&*System::basename(pack.urlToZipFile.clone()));
            ArcStr::from(__mm_s)
        })) {
            installCachePath.clone()
        } else {
            cachePath.clone()
        };
        if StringUtil::endsWith(pack.path.clone(), literal!(".mo")) {
            dirOfPath = System::dirname(pack.path.clone());
            if pack.singleFileStructureCopyAllFiles.clone() {
                Unzip::unzipPath(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*curCachePath);
                        __mm_s.push_str(&*System::basename(pack.urlToZipFile.clone()));
                        ArcStr::from(__mm_s)
                    },
                    if (metamodelica::stringEq(&dirOfPath, &(literal!(".")))) {
                        literal!("")
                    } else {
                        dirOfPath
                    },
                    destPath.clone(),
                );
                expectedLocation = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*destPath);
                    __mm_s.push_str(&*literal!("/"));
                    __mm_s.push_str(&*System::basename(pack.path.clone()));
                    ArcStr::from(__mm_s)
                };
                if !(System::rename(expectedLocation.clone(), destPathPkgMo.clone())) {
                    Error::addMessage(
                        Error::ERROR_PKG_INSTALL_NO_PACKAGE_MO.clone(),
                        list![
                            {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*curCachePath);
                                __mm_s.push_str(&*System::basename(pack.urlToZipFile.clone()));
                                ArcStr::from(__mm_s)
                            },
                            expectedLocation
                        ],
                    )?;
                    return Err("fail");
                }
            } else {
                Unzip::unzipPath(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*curCachePath);
                        __mm_s.push_str(&*System::basename(pack.urlToZipFile.clone()));
                        ArcStr::from(__mm_s)
                    },
                    pack.path.clone(),
                    destPathPkgMo.clone(),
                );
            }
        } else {
            Unzip::unzipPath(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*curCachePath);
                    __mm_s.push_str(&*System::basename(pack.urlToZipFile.clone()));
                    ArcStr::from(__mm_s)
                },
                pack.path.clone(),
                destPath.clone(),
            );
        }
        if System::regularFileExists(destPathPkgMo.clone()) {
            if metamodelica::stringEq(&oldSha, &(literal!(""))) {
                Error::addSourceMessage(
                    &(Error::NOTIFY_PKG_INSTALL_DONE.clone()),
                    list![pack.sha.clone()],
                    &(makeSourceInfo(destPathPkgMo)),
                )?;
            } else {
                Error::addSourceMessage(
                    &(Error::NOTIFY_PKG_UPGRADE_DONE.clone()),
                    list![pack.sha.clone(), oldSha],
                    &(makeSourceInfo(destPathPkgMo)),
                )?;
            }
        } else {
            Error::addMessage(
                Error::ERROR_PKG_INSTALL_NO_PACKAGE_MO.clone(),
                list![
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*curCachePath);
                        __mm_s.push_str(&*System::basename(pack.urlToZipFile.clone()));
                        ArcStr::from(__mm_s)
                    },
                    destPathPkgMo
                ],
            )?;
            System::removeDirectory(destPath);
            return Err("fail");
        }
        System::writeFile(destPathPkgInfo, {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*JSON::toString(&pack.json, false)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        })?;
    }
    Ok(success)
}

pub fn installCachedPackages() -> Result<()> {
    let mut packageIndex: ArcStr;
    let mut homeDir: ArcStr;
    let mut obj: metamodelica::Ref<JSON::JSON>;
    let mut libs_obj: metamodelica::Ref<JSON::JSON>;
    let mut lib_obj: metamodelica::Ref<JSON::JSON>;
    let mut versions_obj: metamodelica::Ref<JSON::JSON>;
    let mut libs: metamodelica::List<ArcStr>;
    homeDir = Settings::getHomeDir(Testsuite::isRunning()?);
    if !((System::subDirectories(getUserLibraryPath()?)).is_empty())
        || metamodelica::stringEq(&homeDir, &(literal!("")))
        || metamodelica::stringEq(&homeDir, &(literal!("/")))
    {
        return Ok(());
    }
    packageIndex = getInstallationIndexPath()?;
    if !(System::regularFileExists(packageIndex.clone())) {
        return Ok(());
    }
    obj = JSON::makeNull();
    if '__try0: {
        obj = unwrap_break_err!(JSON::parseFile(packageIndex.clone()), '__try0);
        Ok::<(), &'static str>(())
    }
    .is_err()
    {
        Error::addSourceMessage(
            &(Error::ERROR_PKG_INDEX_NOT_PARSED.clone()),
            list![packageIndex.clone()],
            &(makeSourceInfo(packageIndex.clone())),
        )?;
    }
    match '__try1: {
        libs_obj = unwrap_break_err!(JSON::get(&obj, literal!("libs")), '__try1);
        libs = unwrap_break_err!(JSON::getKeys(&libs_obj), '__try1);
        Ok::<_, &'static str>((libs.clone(), libs_obj.clone()))
    } {
        Ok((__try1_o0, __try1_o1)) => {
            libs = __try1_o0;
            libs_obj = __try1_o1;
        }
        Err(_) => {
            return Ok(());
        }
    }
    if !((libs).is_empty()) {
        Error::addSourceMessage(
            &(Error::NOTIFY_INITIALIZING_USER_LIBRARIES.clone()),
            list![getUserLibraryPath()?],
            &(makeSourceInfo(packageIndex.clone())),
        )?;
    }
    if !(System::regularFileExists(getIndexPath()?)) {
        Util::createDirectoryTree(getUserLibraryPath()?);
        System::copyFile(packageIndex, getIndexPath()?);
    }
    for mut lib in &*libs {
        lib_obj = JSON::get(&libs_obj, lib.clone())?;
        versions_obj = JSON::getOrDefault(&lib_obj, literal!("versions"), JSON::emptyObject())?;
        for mut version in &*JSON::getKeys(&versions_obj)? {
            installPackage(lib.clone(), version.clone(), true, true)?;
        }
    }
    updateIndex()?;
    Ok(())
}

fn compareUrlBool(
    mut tpl1: &(metamodelica::List<ArcStr>, ArcStr),
    mut tpl2: &(metamodelica::List<ArcStr>, ArcStr),
) -> Result<bool> {
    let mut b: bool;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match tpl1 {
        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    s1 = metamodelica::Own::own(__pa0);
    let __pa1 = ::match_deref::match_deref! { match tpl2 {
        (Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ }, _) => __pa1.clone(),
        _ => return Err("pattern mismatch"),
    } };
    s2 = metamodelica::Own::own(__pa1);
    b = stringCompare(&s1, &s2) > 0;
    Ok(b)
}

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct PackageInstallInfo {
    pub needsInstall: bool,
    pub pkg: ArcStr,
    pub version: SemanticVersion::Version,
    pub urlToZipFile: ArcStr,
    pub path: ArcStr,
    pub sha: ArcStr,
    pub singleFileStructureCopyAllFiles: bool,
    pub json: metamodelica::Ref<JSON::JSON>,
}

impl metamodelica::gc::MMTrace for PackageInstallInfo {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.needsInstall, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.pkg, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.version, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.urlToZipFile, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.path, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.sha, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.singleFileStructureCopyAllFiles, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.json, __mmv)?;
        Ok(())
    }
}
pub type PKG_INSTALL_INFO = PackageInstallInfo;

fn installPackageWork(
    mut pkg: ArcStr,
    mut version: ArcStr,
    mut exactMatch: bool,
    mut fallbackOnNonExactMatch: bool,
    mut packagesToInstall: metamodelica::List<PackageInstallInfo>,
) -> Result<(bool, metamodelica::List<PackageInstallInfo>)> {
    let mut success: bool;
    let mut packagesToInstall: metamodelica::List<PackageInstallInfo> = packagesToInstall;
    let mut installedLibraries: metamodelica::Ref<AvailableLibraries::Tree>;
    let mut installedVersions: metamodelica::Ref<VersionMap::Tree>;
    let mut candidates: metamodelica::List<ArcStr>;
    let mut candidatesSemver: metamodelica::List<SemanticVersion::Version>;
    let mut exactMatches: metamodelica::List<SemanticVersion::Version>;
    let mut versionToInstall: ArcStr;
    let mut usedVersion: ArcStr;
    let mut path: ArcStr;
    let mut sha: ArcStr;
    let mut jsonPath: ArcStr;
    let mut zip: ArcStr;
    let mut semverToInstall: SemanticVersion::Version;
    let mut semver: SemanticVersion::Version;
    let mut index: metamodelica::Ref<JSON::JSON>;
    let mut versionObj: metamodelica::Ref<JSON::JSON>;
    let mut versionsObj: metamodelica::Ref<JSON::JSON>;
    let mut usesObj: metamodelica::Ref<JSON::JSON>;
    let mut indexHasPkg: bool;
    let mut packageToInstall: Option<PackageInstallInfo> = None;
    candidates = versionsThatProvideTheWanted(pkg.clone(), version.clone(), true);
    candidatesSemver = ({
        let mut __acc: metamodelica::List<SemanticVersion::Version> = metamodelica::nil();
        for mut candidate in (candidates.clone()).into_iter().cloned() {
            let __x = SemanticVersion::parse(candidate.clone(), false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    semver = SemanticVersion::parse(version.clone(), false)?;
    exactMatches = ({
        let mut __acc: metamodelica::List<SemanticVersion::Version> = metamodelica::nil();
        for mut candidate in (candidatesSemver.clone()).into_iter().cloned() {
            if !(0
                == SemanticVersion::compare(
                    &(candidate.clone()),
                    &(semver.clone()),
                    true,
                    SemanticVersion::hasMetaInformation(&semver),
                )?)
            {
                continue;
            }
            let __x = candidate.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    success = false;
    for mut pkgInfo in &*packagesToInstall {
        if metamodelica::stringEq(&pkgInfo.pkg, &pkg) {
            if SemanticVersion::compare(&pkgInfo.version, &semver, true, false)? == 0
                || ({
                    let mut __acc: Option<bool> = None;
                    for mut candidate in (candidatesSemver).into_iter().cloned() {
                        let __x = 0 == SemanticVersion::compare(&pkgInfo.version, &(candidate.clone()), true, false)?;
                        __acc = Some(match __acc {
                            None => __x,
                            Some(__cur) => {
                                if __x > __cur {
                                    __x
                                } else {
                                    __cur
                                }
                            }
                        });
                    }
                    __acc.unwrap_or(false)
                })
            {
                success = true;
                return Ok((success, packagesToInstall.clone()));
            }
            Error::addMessage(
                Error::WARNING_PKG_CONFLICTING_VERSIONS.clone(),
                list![pkg, SemanticVersion::toString(&pkgInfo.version), version],
            )?;
            success = true;
            return Ok((success, packagesToInstall.clone()));
        }
    }
    installedLibraries = getInstalledLibraries()?;
    if (candidates).is_empty() {
        versionToInstall = version.clone();
        semverToInstall = semver.clone();
    } else if exactMatch && !((exactMatches).is_empty()) {
        semverToInstall = (exactMatches).head().cloned()?;
        versionToInstall = SemanticVersion::toString(&semverToInstall);
    } else {
        versionToInstall = (candidates).head().cloned()?;
        semverToInstall = (candidatesSemver).head().cloned()?;
    }
    index = getPackageIndex(true)?;
    indexHasPkg = true;
    sha = literal!("");
    if AvailableLibraries::hasKey(installedLibraries.clone(), pkg.clone())? {
        installedVersions = AvailableLibraries::get(&installedLibraries, pkg.clone())?;
        if VersionMap::hasKey(installedVersions.clone(), semverToInstall.clone())?
            || metamodelica::stringEq(&version, &(literal!(""))) && !(indexHasPkg)
        {
            success = true;
            path = if (VersionMap::hasKey(installedVersions.clone(), semverToInstall.clone())?) {
                VersionMap::get(&installedVersions, semverToInstall.clone())?
            } else {
                literal!("#DUMMY#")
            };
            jsonPath = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*path);
                __mm_s.push_str(&*literal!("/"));
                __mm_s.push_str(&*arcstr::literal!(metaDataFileName));
                ArcStr::from(__mm_s)
            };
            if System::regularFileExists(jsonPath.clone()) {
                versionObj = JSON::parseFile(jsonPath)?;
                zip = JSON::getString(&(JSON::get(&versionObj, literal!("zipfile"))?))?;
                if '__try0: {
                    sha = unwrap_break_err!(JSON::getString(&(unwrap_break_err!(JSON::get(&versionObj, literal!("sha")), '__try0))), '__try0);
                    Ok::<(), &'static str>(())
                }.is_err() {
                }
            } else {
                zip = literal!("");
            }
            packageToInstall = Some(PackageInstallInfo {
                needsInstall: false,
                pkg: pkg.clone(),
                version: semverToInstall.clone(),
                urlToZipFile: zip,
                path: path,
                sha: sha.clone(),
                singleFileStructureCopyAllFiles: false,
                json: JSON::emptyObject(),
            });
            indexHasPkg = JSON::hasKey(&(JSON::get(&index, literal!("libs"))?), pkg.clone())?;
        }
    }
    if !(success) {
        if (candidates).is_empty() {
            Error::addSourceMessage(
                &(Error::ERROR_PKG_NOT_FOUND_VERSION.clone()),
                list![
                    pkg.clone(),
                    version,
                    stringDelimitList(getAllProvidedVersionsForLibrary(pkg, true), literal!("\n"))
                ],
                &(makeSourceInfo(getIndexPath()?)),
            )?;
            return Ok((success, packagesToInstall));
        }
        if exactMatch
            && !({
                let mut __acc: Option<bool> = None;
                for mut candidate in (candidatesSemver.clone()).into_iter().cloned() {
                    let __x = 0 == SemanticVersion::compare(&semver, &(candidate.clone()), true, false)?;
                    __acc = Some(match __acc {
                        None => __x,
                        Some(__cur) => {
                            if __x > __cur {
                                __x
                            } else {
                                __cur
                            }
                        }
                    });
                }
                __acc.unwrap_or(false)
            })
        {
            if !(fallbackOnNonExactMatch) {
                Error::addSourceMessage(
                    &(Error::ERROR_PKG_NOT_EXACT_MATCH.clone()),
                    list![pkg, version, stringDelimitList(candidates, literal!(", "))],
                    &(makeSourceInfo(getIndexPath()?)),
                )?;
                return Ok((success, packagesToInstall));
            }
            versionToInstall = (candidates).head().cloned()?;
            semverToInstall = (candidatesSemver).head().cloned()?;
        }
    }
    if !(indexHasPkg) {
        packagesToInstall = metamodelica::cons(packageToInstall.ok_or("pattern mismatch")?, packagesToInstall);
        return Ok((success, packagesToInstall));
    }
    versionsObj = JSON::get(
        &(JSON::get(&(JSON::get(&index, literal!("libs"))?), pkg.clone())?),
        literal!("versions"),
    )?;
    if success && !(JSON::hasKey(&versionsObj, versionToInstall.clone())?) {
        packagesToInstall = metamodelica::cons(packageToInstall.ok_or("pattern mismatch")?, packagesToInstall);
        return Ok((success, packagesToInstall));
    }
    versionObj = JSON::get(&versionsObj, versionToInstall)?;
    if !(success)
        || !metamodelica::stringEq(&sha, &(literal!("")))
            && !metamodelica::stringEq(&sha, &(getShaOrZipfile(&versionObj)?))
    {
        success = true;
        packageToInstall = Some(PackageInstallInfo {
            needsInstall: true,
            pkg: pkg,
            version: semverToInstall,
            urlToZipFile: JSON::getString(&(JSON::get(&versionObj, literal!("zipfile"))?))?,
            path: JSON::getString(&(JSON::get(&versionObj, literal!("path"))?))?,
            sha: getShaOrZipfile(&versionObj)?,
            singleFileStructureCopyAllFiles: JSON::getBoolean(
                &(JSON::getOrDefault(
                    &versionObj,
                    literal!("singleFileStructureCopyAllFiles"),
                    openmodelica_util::JSON::interned_FALSE(),
                )?),
            )?,
            json: versionObj.clone(),
        });
    }
    usesObj = JSON::getOrDefault(&versionObj, literal!("uses"), JSON::emptyObject())?;
    packagesToInstall = metamodelica::cons(packageToInstall.ok_or("pattern mismatch")?, packagesToInstall);
    for mut usesPackage in &*JSON::getKeys(&usesObj)? {
        let __pa1 = ::match_deref::match_deref! { match &(JSON::get(&usesObj, usesPackage.clone())?) {
            Deref @ JSON::STRING { r#str: __pa1 } => __pa1.clone(),
            _ => return Err("pattern mismatch"),
        } };
        usedVersion = metamodelica::Own::own(__pa1);
        (success, packagesToInstall) =
            installPackageWork(usesPackage.clone(), usedVersion, exactMatch, true, packagesToInstall)?;
        if !(success) {
            return Ok((success, packagesToInstall));
        }
    }
    Ok((success, packagesToInstall))
}

fn getShaOrZipfile(mut obj: &metamodelica::Ref<JSON::JSON>) -> Result<ArcStr> {
    let mut res: ArcStr;
    res = if (JSON::hasKey(obj, literal!("sha"))?) {
        JSON::getString(&(JSON::get(obj, literal!("sha"))?))?
    } else {
        System::basename(JSON::getString(&(JSON::get(obj, literal!("zipfile"))?))?)
    };
    Ok(res)
}

fn getAllUrls(mut url: ArcStr, mut mirrors: &metamodelica::List<ArcStr>) -> Result<metamodelica::List<ArcStr>> {
    let mut urls: metamodelica::List<ArcStr>;
    let mut urlWithoutProtocol: ArcStr;
    let mut newUrl: ArcStr;
    urls = list![url.clone()];
    if !(StringUtil::startsWith(url.clone(), literal!("https://"))) {
        return Ok(urls);
    }
    urlWithoutProtocol = substring(url.clone(), 9, ((url).len() as i32))?;
    for mut mirror in &**mirrors {
        newUrl = if (StringUtil::endsWith(mirror.clone(), literal!("/"))) {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*mirror);
                __mm_s.push_str(&*urlWithoutProtocol);
                ArcStr::from(__mm_s)
            }
        } else {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*mirror);
                __mm_s.push_str(&*literal!("/"));
                __mm_s.push_str(&*urlWithoutProtocol);
                ArcStr::from(__mm_s)
            }
        };
        urls = metamodelica::cons(newUrl, urls);
    }
    Ok(urls)
}

fn getMirrors() -> Result<metamodelica::List<ArcStr>> {
    let mut mirrors: metamodelica::List<ArcStr>;
    let mut obj: metamodelica::Ref<JSON::JSON>;
    obj = getPackageIndex(false)?;
    if !(JSON::hasKey(&obj, literal!("mirrors"))?) {
        mirrors = metamodelica::nil();
        return Ok(mirrors);
    }
    obj = JSON::get(&obj, literal!("mirrors"))?;
    mirrors = JSON::getStringList(&obj)?;
    Ok(mirrors)
}

fn getUserLibraryPath() -> Result<ArcStr> {
    let mut path: ArcStr;
    path = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Settings::getHomeDir(Testsuite::isRunning()?));
        __mm_s.push_str(&*literal!("/.openmodelica/libraries/"));
        ArcStr::from(__mm_s)
    };
    Ok(path)
}

fn getIndexPath() -> Result<ArcStr> {
    let mut path: ArcStr;
    path = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Settings::getHomeDir(Testsuite::isRunning()?));
        __mm_s.push_str(&*literal!("/.openmodelica/libraries/index.json"));
        ArcStr::from(__mm_s)
    };
    Ok(path)
}

pub(crate) fn getCachePath() -> Result<ArcStr> {
    let mut path: ArcStr;
    path = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Settings::getHomeDir(Testsuite::isRunning()?));
        __mm_s.push_str(&*literal!("/.openmodelica/cache/"));
        ArcStr::from(__mm_s)
    };
    Ok(path)
}

fn getInstallationIndexPath() -> Result<ArcStr> {
    let mut path: ArcStr;
    path = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?);
        __mm_s.push_str(&*literal!("/share/omlibrary/cache/index.json"));
        ArcStr::from(__mm_s)
    };
    Ok(path)
}

fn getInstallationCachePath() -> Result<ArcStr> {
    let mut path: ArcStr;
    path = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?);
        __mm_s.push_str(&*literal!("/share/omlibrary/cache/"));
        ArcStr::from(__mm_s)
    };
    Ok(path)
}

fn makeSourceInfo(mut fileName: ArcStr) -> SourceInfo {
    let mut info: SourceInfo;
    info = SourceInfo {
        fileName: fileName,
        isReadOnly: true,
        lineNumberStart: 0,
        columnNumberStart: 0,
        lineNumberEnd: 0,
        columnNumberEnd: 0,
        lastModification: metamodelica::OrderedFloat(0.0_f64),
    };
    info
}
