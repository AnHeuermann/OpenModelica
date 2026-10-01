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

use crate::BaseModelica;
use crate::NFBinding as Binding;
use crate::NFInst as Inst;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFSubscript as Subscript;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;
use openmodelica_util::Error;
use openmodelica_util::IOStream;
use openmodelica_util_datatypes_basic::List;

thread_local! { static __EMPTY_MOD_TLS: metamodelica::Ref<Modifier::Modifier> = crate::NFModifier::Modifier::interned_NOMOD(); }
pub(crate) fn EMPTY_MOD() -> metamodelica::Ref<Modifier::Modifier> {
    __EMPTY_MOD_TLS.with(|__t| __t.clone())
}

pub mod ModTable {
    use super::*;
    pub(crate) fn keyStr(mut inKey: Key) -> ArcStr {
        let mut outString: ArcStr;
        outString = inKey;
        outString
    }

    pub(crate) fn valueStr(mut inValue: Value) -> Result<ArcStr> {
        let mut outString: ArcStr;
        outString = Modifier::toString(&inValue, true)?;
        Ok(outString)
    }

    pub(crate) fn keyCompare(mut inKey1: Key, mut inKey2: Key) -> i32 {
        let mut outResult: i32;
        outResult = stringCompare(&inKey1, &inKey2);
        outResult
    }

    pub type ConflictFunc = std::sync::Arc<dyn ::std::ops::Fn(Value, Value, Key) -> Result<Value> + 'static>;

    pub type Key = ArcStr;

    /// The binary tree data structure.
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
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
            thread_local! {
                static INTERNED: metamodelica::Ref<Tree> = metamodelica::Ref::new(Tree::EMPTY);
            }
            INTERNED.with(|i| i.clone())
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

    pub type Value = metamodelica::Ref<Modifier::Modifier>;

    pub type ValueNode = ArcStr;

    pub(crate) fn add(
        mut inTree: metamodelica::Ref<Tree>,
        mut inKey: &Key,
        mut inValue: &Value,
        mut conflictFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<Modifier::Modifier>,
            metamodelica::Ref<Modifier::Modifier>,
            ArcStr,
        ) -> Result<metamodelica::Ref<Modifier::Modifier>>,
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
                        right: crate::NFModifier::ModTable::Tree::interned_EMPTY(),
                    });
                } else if key_comp == 1 {
                    outTree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: crate::NFModifier::ModTable::Tree::interned_EMPTY(),
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
        mut inValues: &metamodelica::List<(ArcStr, metamodelica::Ref<Modifier::Modifier>)>,
        mut conflictFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<Modifier::Modifier>,
            metamodelica::Ref<Modifier::Modifier>,
            ArcStr,
        ) -> Result<metamodelica::Ref<Modifier::Modifier>>,
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
            Option<metamodelica::Ref<Modifier::Modifier>>,
        ) -> Result<metamodelica::Ref<Modifier::Modifier>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        pub type UpdateFn = std::sync::Arc<
            dyn ::std::ops::Fn(Option<metamodelica::Ref<Modifier::Modifier>>) -> Result<Value> + 'static,
        >;

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
                        right: crate::NFModifier::ModTable::Tree::interned_EMPTY(),
                    });
                } else if key_comp == 1 {
                    new_tree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: crate::NFModifier::ModTable::Tree::interned_EMPTY(),
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
        mut inFunc: &'__b dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<Modifier::Modifier>, FT) -> Result<FT>,
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
        mut foldFunc: &dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<Modifier::Modifier>, FT) -> Result<(FT, bool)>,
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
        mut foldFunc: &dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<Modifier::Modifier>, FT1, FT2) -> Result<(FT1, FT2)>,
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
        mut func: &dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<Modifier::Modifier>) -> Result<()>,
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
        mut inValues: &metamodelica::List<(ArcStr, metamodelica::Ref<Modifier::Modifier>)>,
        mut conflictFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<Modifier::Modifier>,
            metamodelica::Ref<Modifier::Modifier>,
            ArcStr,
        ) -> Result<metamodelica::Ref<Modifier::Modifier>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        let mut tree: metamodelica::Ref<Tree> = crate::NFModifier::ModTable::Tree::interned_EMPTY();
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
    ) -> Option<metamodelica::Ref<Modifier::Modifier>> {
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
            metamodelica::Ref<Modifier::Modifier>,
            metamodelica::Ref<Modifier::Modifier>,
            ArcStr,
        ) -> Result<metamodelica::Ref<Modifier::Modifier>>,
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
        mut lst: metamodelica::List<metamodelica::Ref<Modifier::Modifier>>,
    ) -> metamodelica::List<metamodelica::Ref<Modifier::Modifier>> {
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
            metamodelica::Ref<Modifier::Modifier>,
        ) -> Result<metamodelica::Ref<Modifier::Modifier>>,
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
            metamodelica::Ref<Modifier::Modifier>,
            FT,
        ) -> Result<(metamodelica::Ref<Modifier::Modifier>, FT)>,
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
        let mut outTree: metamodelica::Ref<Tree> = crate::NFModifier::ModTable::Tree::interned_EMPTY();
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
                node = setTreeLeftRight(outNode, __outNode_left.clone(), crate::NFModifier::ModTable::Tree::interned_EMPTY())?;
                setTreeLeftRight(child.clone(), node, crate::NFModifier::ModTable::Tree::interned_EMPTY())?
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
                node = setTreeLeftRight(outNode, crate::NFModifier::ModTable::Tree::interned_EMPTY(), __outNode_right.clone())?;
                setTreeLeftRight(child.clone(), crate::NFModifier::ModTable::Tree::interned_EMPTY(), node)?
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
        mut lst: metamodelica::List<(ArcStr, metamodelica::Ref<Modifier::Modifier>)>,
    ) -> metamodelica::List<(ArcStr, metamodelica::Ref<Modifier::Modifier>)> {
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
                Modifier::Modifier,
            >,
                                                 __a1: metamodelica::Ref<
                Modifier::Modifier,
            >,
                                                 __a2: ArcStr|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(addConflictReplace(__a0, &__a1, &__a2))
            })?;
        Ok(outTree)
    }
}

pub mod ModifierScope {
    use super::*;
    /// Structure that represents where a modifier comes from.
    #[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum ModifierScope {
        COMPONENT { name: ArcStr },
        CLASS { name: ArcStr },
        EXTENDS { path: metamodelica::Ref<Absyn::Path> },
    }
    impl metamodelica::gc::MMTrace for ModifierScope {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                ModifierScope::COMPONENT { name } => {
                    metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                    Ok(())
                }
                ModifierScope::CLASS { name } => {
                    metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                    Ok(())
                }
                ModifierScope::EXTENDS { path } => {
                    metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Default for ModifierScope {
        fn default() -> Self {
            Self::COMPONENT {
                name: Default::default(),
            }
        }
    }
    pub use self::ModifierScope::{CLASS, COMPONENT, EXTENDS};
    pub(crate) fn fromElement(
        mut element: &metamodelica::Ref<SCode::Element>,
    ) -> Result<metamodelica::Ref<ModifierScope>> {
        let mut scope: metamodelica::Ref<ModifierScope>;
        scope = (match &**element {
            SCode::Element::COMPONENT {
                name: __element_name, ..
            } => metamodelica::Ref::new(ModifierScope::COMPONENT {
                name: __element_name.clone(),
            }),
            SCode::Element::CLASS {
                name: __element_name, ..
            } => metamodelica::Ref::new(ModifierScope::CLASS {
                name: __element_name.clone(),
            }),
            SCode::Element::EXTENDS {
                baseClassPath: __element_baseClassPath,
                ..
            } => metamodelica::Ref::new(ModifierScope::EXTENDS {
                path: __element_baseClassPath.clone(),
            }),
            _ => return Err("match: no arm matched"),
        });
        Ok(scope)
    }

    pub(crate) fn name(mut scope: &metamodelica::Ref<ModifierScope>) -> Result<ArcStr> {
        let mut name: ArcStr;
        name = (match &**scope {
            COMPONENT { name: __scope_name } => __scope_name.clone(),
            CLASS { name: __scope_name } => __scope_name.clone(),
            EXTENDS { path: __scope_path } => AbsynUtil::pathString(__scope_path.clone(), literal!("."), true, false)?,
        });
        Ok(name)
    }

    pub(crate) fn isClass(mut scope: &metamodelica::Ref<ModifierScope>) -> bool {
        let mut res: bool;
        res = (match &**scope {
            CLASS { .. } => true,
            _ => false,
        });
        res
    }

    pub(crate) fn toString(mut scope: &metamodelica::Ref<ModifierScope>) -> Result<ArcStr> {
        let mut string: ArcStr;
        string = (match &**scope {
            COMPONENT { name: __scope_name } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("component "));
                __mm_s.push_str(&*__scope_name);
                ArcStr::from(__mm_s)
            }
            CLASS { name: __scope_name } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("class "));
                __mm_s.push_str(&*__scope_name);
                ArcStr::from(__mm_s)
            }
            EXTENDS { path: __scope_path } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("extends "));
                __mm_s.push_str(&*AbsynUtil::pathString(
                    __scope_path.clone(),
                    literal!("."),
                    true,
                    false,
                )?);
                ArcStr::from(__mm_s)
            }
        });
        Ok(string)
    }
}

pub mod Modifier {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum Modifier {
        MODIFIER {
            name: ArcStr,
            finalPrefix: SCode::Final,
            eachPrefix: SCode::Each,
            binding: metamodelica::Ref<Binding::NFBinding>,
            subModifiers: metamodelica::Ref<ModTable::Tree>,
            info: SourceInfo,
        },
        REDECLARE {
            finalPrefix: SCode::Final,
            eachPrefix: SCode::Each,
            element: metamodelica::Ref<InstNode::InstNode>,
            innerMod: metamodelica::Ref<Modifier>,
            outerMod: metamodelica::Ref<Modifier>,
            constrainingMod: metamodelica::Ref<Modifier>,
            propagatedSubs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
        },
        NOMOD,
    }
    impl metamodelica::gc::MMTrace for Modifier {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                Modifier::MODIFIER {
                    name,
                    finalPrefix,
                    eachPrefix,
                    binding,
                    subModifiers,
                    info,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(eachPrefix, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(binding, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(subModifiers, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                    Ok(())
                }
                Modifier::REDECLARE {
                    finalPrefix,
                    eachPrefix,
                    element,
                    innerMod,
                    outerMod,
                    constrainingMod,
                    propagatedSubs,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(eachPrefix, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(element, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(innerMod, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(outerMod, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(constrainingMod, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(propagatedSubs, __mmv)?;
                    Ok(())
                }
                Modifier::NOMOD => Ok(()),
            }
        }
    }
    impl Modifier {
        pub fn interned_NOMOD() -> metamodelica::Ref<Modifier> {
            thread_local! {
                static INTERNED: metamodelica::Ref<Modifier> = metamodelica::Ref::new(Modifier::NOMOD);
            }
            INTERNED.with(|i| i.clone())
        }
    }
    pub fn interned_NOMOD() -> metamodelica::Ref<Modifier> {
        Modifier::interned_NOMOD()
    }
    impl Default for Modifier {
        fn default() -> Self {
            Self::NOMOD
        }
    }
    pub use self::Modifier::{MODIFIER, NOMOD, REDECLARE};
    pub fn create(
        mut r#mod: &metamodelica::Ref<SCode::Mod>,
        mut name: ArcStr,
        mut modScope: &metamodelica::Ref<ModifierScope::ModifierScope>,
        mut scope: metamodelica::Ref<InstNode::InstNode>,
        mut confidence: i32,
    ) -> Result<metamodelica::Ref<Modifier>> {
        let mut newMod: metamodelica::Ref<Modifier>;
        newMod = (match &**r#mod {
            SCode::Mod::NOMOD { .. } => crate::NFModifier::Modifier::interned_NOMOD(),
            SCode::Mod::MOD {
                binding: __mod_binding,
                eachPrefix: __mod_eachPrefix,
                finalPrefix: __mod_finalPrefix,
                info: __mod_info,
                subModLst: __mod_subModLst,
                ..
            } => {
                let mut submod_lst: metamodelica::List<(ArcStr, metamodelica::Ref<Modifier>)>;
                let mut submod_table: metamodelica::Ref<ModTable::Tree>;
                let mut binding: metamodelica::Ref<Binding::NFBinding>;
                let mut is_each: bool;
                is_each = SCodeUtil::eachBool(__mod_eachPrefix.clone());
                binding = Binding::fromAbsyn(
                    __mod_binding.clone(),
                    is_each,
                    scope.clone(),
                    confidence,
                    __mod_info.clone(),
                );
                submod_lst = ({
                    let mut __acc: metamodelica::List<(ArcStr, metamodelica::Ref<Modifier>)> = metamodelica::nil();
                    for mut m in (__mod_subModLst.clone()).into_iter().cloned() {
                        if !(!(SCodeUtil::isBreakSubMod(&(m.clone())))) {
                            continue;
                        }
                        let __x = (
                            m.ident.clone(),
                            createSubMod(&(m.clone()), modScope, scope.clone(), confidence)?,
                        );
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                submod_table = ModTable::fromList(
                    &submod_lst,
                    &({
                        let __pe_b3 = modScope.clone();
                        let __pe_b4 = metamodelica::nil();
                        move |__pe_a0, __pe_a1, __pe_a2| {
                            mergeLocal(__pe_a0, __pe_a1, &__pe_a2, &__pe_b3, __pe_b4.clone())
                        }
                    }),
                )?;
                metamodelica::Ref::new(Modifier::MODIFIER {
                    name: name,
                    finalPrefix: __mod_finalPrefix.clone(),
                    eachPrefix: __mod_eachPrefix.clone(),
                    binding: binding,
                    subModifiers: submod_table,
                    info: __mod_info.clone(),
                })
            }
            SCode::Mod::REDECL {
                element: elem,
                eachPrefix: __mod_eachPrefix,
                finalPrefix: __mod_finalPrefix,
            } => {
                let mut node: metamodelica::Ref<InstNode::InstNode>;
                let mut cc_mod: metamodelica::Ref<Modifier>;
                node = NFInstNode::InstNode::new(elem.clone(), scope.clone())?;
                if NFInstNode::InstNode::isClass(&node)? {
                    Inst::partialInstClass(node.clone())?;
                }
                cc_mod = createConstrainingMod(elem, scope, confidence)?;
                metamodelica::Ref::new(Modifier::REDECLARE {
                    finalPrefix: __mod_finalPrefix.clone(),
                    eachPrefix: __mod_eachPrefix.clone(),
                    element: node,
                    innerMod: crate::NFModifier::Modifier::interned_NOMOD(),
                    outerMod: crate::NFModifier::Modifier::interned_NOMOD(),
                    constrainingMod: cc_mod,
                    propagatedSubs: metamodelica::nil(),
                })
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(newMod)
    }

    pub(crate) fn createConstrainingMod(
        mut element: &metamodelica::Ref<SCode::Element>,
        mut scope: metamodelica::Ref<InstNode::InstNode>,
        mut confidence: i32,
    ) -> Result<metamodelica::Ref<Modifier>> {
        let mut r#mod: metamodelica::Ref<Modifier>;
        let mut smod: metamodelica::Ref<SCode::Mod>;
        r#mod = (::match_deref::match_deref! { match element {
            Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { modifier: __esc_smod, .. }) }, .. }, name: __element_name, .. } => {
                smod = (*__esc_smod).clone();
                create(metamodelica::AsArg::as_arg(&smod), __element_name.clone(), &(metamodelica::Ref::new(ModifierScope::ModifierScope::CLASS { name: __element_name.clone() })), scope, confidence)?
            },
            Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { modifier: __esc_smod, .. }) }, .. }, name: __element_name, .. } => {
                smod = (*__esc_smod).clone();
                create(metamodelica::AsArg::as_arg(&smod), __element_name.clone(), &(metamodelica::Ref::new(ModifierScope::ModifierScope::COMPONENT { name: __element_name.clone() })), scope, confidence)?
            },
            _ => crate::NFModifier::Modifier::interned_NOMOD(),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(r#mod)
    }

    pub(crate) fn stripSCodeMod(
        mut elem: metamodelica::Ref<SCode::Element>,
    ) -> (metamodelica::Ref<SCode::Element>, metamodelica::Ref<SCode::Mod>) {
        let mut elem: metamodelica::Ref<SCode::Element> = elem;
        let mut r#mod: metamodelica::Ref<SCode::Mod>;
        r#mod = (::match_deref::match_deref! { match &(elem.clone()) {
            Deref @ SCode::Element::CLASS { classDef: cdef @ Deref @ SCode::ClassDef::DERIVED { modifications: __esc_mod, .. }, .. } => {
                r#mod = (*__esc_mod).clone();
                let mut cdef = (*cdef).clone();
                if !(SCodeUtil::isEmptyMod(metamodelica::AsArg::as_arg(&r#mod))) {
                    assign_variant_field!(cdef => SCode::ClassDef::DERIVED; modifications = openmodelica_frontend_types::SCode::Mod::interned_NOMOD());
                    assign_variant_field!(elem => SCode::Element::CLASS; classDef = cdef.clone());
                }
                r#mod.clone()
            },
            Deref @ SCode::Element::COMPONENT { modifications: __esc_mod, .. } => {
                r#mod = (*__esc_mod).clone();
                if !(SCodeUtil::isEmptyMod(metamodelica::AsArg::as_arg(&r#mod))) {
                    assign_variant_field!(elem => SCode::Element::COMPONENT; modifications = openmodelica_frontend_types::SCode::Mod::interned_NOMOD());
                }
                r#mod.clone()
            },
            _ => {
                openmodelica_frontend_types::SCode::Mod::interned_NOMOD()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        (elem, r#mod)
    }

    pub(crate) fn fromElement(
        mut element: &metamodelica::Ref<SCode::Element>,
        mut scope: metamodelica::Ref<InstNode::InstNode>,
        mut confidence: i32,
    ) -> Result<metamodelica::Ref<Modifier>> {
        let mut r#mod: metamodelica::Ref<Modifier>;
        r#mod = (::match_deref::match_deref! { match element {
            Deref @ SCode::Element::EXTENDS { baseClassPath: __element_baseClassPath, modifications: __element_modifications, .. } => {
                create(metamodelica::AsArg::as_arg(&__element_modifications), literal!(""), &(metamodelica::Ref::new(ModifierScope::ModifierScope::EXTENDS { path: __element_baseClassPath.clone() })), scope, confidence)?
            },
            Deref @ SCode::Element::COMPONENT { info: __element_info, modifications: __element_modifications, name: __element_name, prefixes: __element_prefixes, .. } => {
                let mut smod: metamodelica::Ref<SCode::Mod>;
                smod = patchElementModFinal(metamodelica::AsArg::as_arg(&__element_prefixes), __element_info.clone(), __element_modifications.clone());
                create(&smod, __element_name.clone(), &(metamodelica::Ref::new(ModifierScope::ModifierScope::COMPONENT { name: __element_name.clone() })), scope, confidence)?
            },
            Deref @ SCode::Element::CLASS { classDef: def @ Deref @ SCode::ClassDef::DERIVED { .. }, name: __element_name, .. } => {
                create(var_field!((**def).modifications, SCode::ClassDef::DERIVED), __element_name.clone(), &(metamodelica::Ref::new(ModifierScope::ModifierScope::CLASS { name: __element_name.clone() })), scope, confidence)?
            },
            Deref @ SCode::Element::CLASS { classDef: def @ Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, name: __element_name, .. } => {
                create(var_field!((**def).modifications, SCode::ClassDef::CLASS_EXTENDS), __element_name.clone(), &(metamodelica::Ref::new(ModifierScope::ModifierScope::CLASS { name: __element_name.clone() })), scope, confidence)?
            },
            _ => {
                crate::NFModifier::Modifier::interned_NOMOD()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(r#mod)
    }

    pub(crate) fn patchElementModFinal(
        mut prefixes: &metamodelica::Ref<SCode::Prefixes>,
        mut info: SourceInfo,
        mut r#mod: metamodelica::Ref<SCode::Mod>,
    ) -> metamodelica::Ref<SCode::Mod> {
        let mut r#mod: metamodelica::Ref<SCode::Mod> = r#mod;
        if SCodeUtil::finalBool(SCodeUtil::prefixesFinal(prefixes)) {
            r#mod = (match &*r#mod {
                SCode::Mod::MOD { .. } => {
                    assign_variant_field!(r#mod => SCode::Mod::MOD; finalPrefix = openmodelica_frontend_types::SCode::Final::FINAL);
                    r#mod
                }
                SCode::Mod::REDECL { .. } => {
                    assign_variant_field!(r#mod => SCode::Mod::REDECL; finalPrefix = openmodelica_frontend_types::SCode::Final::FINAL);
                    r#mod
                }
                _ => metamodelica::Ref::new(SCode::Mod::MOD {
                    finalPrefix: openmodelica_frontend_types::SCode::Final::FINAL,
                    eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
                    subModLst: metamodelica::nil(),
                    binding: None,
                    comment: None,
                    info: info,
                }),
            });
        }
        r#mod
    }

    pub(crate) fn lookupModifier(
        mut modName: ArcStr,
        mut modifier: &metamodelica::Ref<Modifier>,
    ) -> metamodelica::Ref<Modifier> {
        let mut subMod: metamodelica::Ref<Modifier>;
        subMod = 'mc: {
            let __mc_input = &**modifier;
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ MODIFIER { .. } => {
                        Ok(ModTable::get(var_field!((**modifier).subModifiers, Modifier::MODIFIER), modName.clone())?)
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        Ok(EMPTY_MOD().clone())
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            panic!("matchcontinue: no arm matched")
        };
        subMod
    }

    pub fn name(mut modifier: &metamodelica::Ref<Modifier>) -> Result<ArcStr> {
        let mut name: ArcStr;
        name = (match &**modifier {
            MODIFIER {
                name: __modifier_name, ..
            } => __modifier_name.clone(),
            REDECLARE {
                element: __modifier_element,
                ..
            } => NFInstNode::InstNode::name(metamodelica::AsArg::as_arg(&__modifier_element))?,
            _ => return Err("match: no arm matched"),
        });
        Ok(name)
    }

    pub(crate) fn info(mut modifier: &metamodelica::Ref<Modifier>) -> SourceInfo {
        let mut info: SourceInfo;
        info = (match &**modifier {
            MODIFIER {
                info: __modifier_info, ..
            } => __modifier_info.clone(),
            REDECLARE {
                element: __modifier_element,
                ..
            } => NFInstNode::InstNode::info(metamodelica::AsArg::as_arg(&__modifier_element)),
            _ => Absyn::dummyInfo.clone(),
        });
        info
    }

    pub(crate) fn hasBinding(mut modifier: &metamodelica::Ref<Modifier>) -> bool {
        let mut hasBinding: bool;
        hasBinding = (match &**modifier {
            MODIFIER {
                binding: __modifier_binding,
                ..
            } => Binding::isBound(metamodelica::AsArg::as_arg(&__modifier_binding)),
            _ => false,
        });
        hasBinding
    }

    pub fn binding(mut modifier: &metamodelica::Ref<Modifier>) -> metamodelica::Ref<Binding::NFBinding> {
        let mut binding: metamodelica::Ref<Binding::NFBinding>;
        binding = (match &**modifier {
            MODIFIER {
                binding: __modifier_binding,
                ..
            } => __modifier_binding.clone(),
            _ => Binding::EMPTY_BINDING().clone(),
        });
        binding
    }

    pub(crate) fn setBinding(
        mut binding: metamodelica::Ref<Binding::NFBinding>,
        mut modifier: metamodelica::Ref<Modifier>,
    ) -> Result<metamodelica::Ref<Modifier>> {
        let mut modifier: metamodelica::Ref<Modifier> = modifier;
        let () = (match &*modifier {
            MODIFIER { .. } => {
                assign_variant_field!(modifier => Modifier::MODIFIER; binding = binding);
                ()
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(modifier)
    }

    pub(crate) fn merge(
        mut outerMod: metamodelica::Ref<Modifier>,
        mut innerMod: metamodelica::Ref<Modifier>,
        mut name: &ArcStr,
    ) -> Result<metamodelica::Ref<Modifier>> {
        let mut mergedMod: metamodelica::Ref<Modifier>;
        mergedMod = (::match_deref::match_deref! { match &((outerMod.clone(), innerMod.clone())) {
            (Deref @ NOMOD { .. }, _) => {
                innerMod
            },
            (_, Deref @ NOMOD { .. }) => {
                outerMod
            },
            (Deref @ MODIFIER { .. }, Deref @ MODIFIER { .. }) => {
                let mut submods: metamodelica::Ref<ModTable::Tree>;
                let mut binding: metamodelica::Ref<Binding::NFBinding>;
                checkFinalOverride(var_field!((*innerMod).finalPrefix, Modifier::MODIFIER).clone(), &outerMod, var_field!((*innerMod).info, Modifier::MODIFIER).clone())?;
                binding = if (Binding::isBound(var_field!((*outerMod).binding, Modifier::MODIFIER))) {var_field!((*outerMod).binding, Modifier::MODIFIER).clone()} else {var_field!((*innerMod).binding, Modifier::MODIFIER).clone()};
                submods = ModTable::join(var_field!((*innerMod).subModifiers, Modifier::MODIFIER).clone(), var_field!((*outerMod).subModifiers, Modifier::MODIFIER), &move |__a0: metamodelica::Ref<Modifier>, __a1: metamodelica::Ref<Modifier>, __a2: ArcStr| merge(__a0, __a1, &__a2))?;
                metamodelica::Ref::new(Modifier::MODIFIER { name: var_field!((*outerMod).name, Modifier::MODIFIER).clone(), finalPrefix: var_field!((*outerMod).finalPrefix, Modifier::MODIFIER).clone(), eachPrefix: var_field!((*outerMod).eachPrefix, Modifier::MODIFIER).clone(), binding: binding, subModifiers: submods, info: var_field!((*outerMod).info, Modifier::MODIFIER).clone() })
            },
            (Deref @ REDECLARE { .. }, Deref @ MODIFIER { .. }) => {
                assign_variant_field!(outerMod => Modifier::REDECLARE; innerMod = merge(var_field!((*outerMod).innerMod, Modifier::REDECLARE).clone(), innerMod, &(literal!("")))?);
                outerMod
            },
            (Deref @ MODIFIER { .. }, Deref @ REDECLARE { .. }) => {
                assign_variant_field!(innerMod => Modifier::REDECLARE; outerMod = merge(outerMod, var_field!((*innerMod).outerMod, Modifier::REDECLARE).clone(), &(literal!("")))?);
                innerMod
            },
            (Deref @ REDECLARE { constrainingMod: Deref @ NOMOD { .. }, .. }, Deref @ REDECLARE { constrainingMod: Deref @ MODIFIER { .. }, .. }) => {
                metamodelica::Ref::new(Modifier::REDECLARE { finalPrefix: var_field!((*outerMod).finalPrefix, Modifier::REDECLARE).clone(), eachPrefix: var_field!((*outerMod).eachPrefix, Modifier::REDECLARE).clone(), element: var_field!((*outerMod).element, Modifier::REDECLARE).clone(), innerMod: var_field!((*outerMod).innerMod, Modifier::REDECLARE).clone(), outerMod: var_field!((*outerMod).outerMod, Modifier::REDECLARE).clone(), constrainingMod: var_field!((*innerMod).constrainingMod, Modifier::REDECLARE).clone(), propagatedSubs: var_field!((*outerMod).propagatedSubs, Modifier::REDECLARE).clone() })
            },
            (Deref @ REDECLARE { .. }, _) => {
                outerMod
            },
            (_, Deref @ REDECLARE { .. }) => {
                innerMod
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("Mod.mergeMod failed on unknown mod.")])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(mergedMod)
    }

    pub(crate) fn propagate(
        mut r#mod: metamodelica::Ref<Modifier>,
        mut origin: metamodelica::Ref<InstNode::InstNode>,
        mut parent: metamodelica::Ref<InstNode::InstNode>,
    ) -> Result<metamodelica::Ref<Modifier>> {
        let mut outMod: metamodelica::Ref<Modifier> = propagateSubs(
            r#mod.clone(),
            &(list![metamodelica::Ref::new(Subscript::NFSubscript::SPLIT_PROXY {
                origin: NFInstNode::InstNode::scopeRef(origin.clone()),
                parent: NFInstNode::InstNode::scopeRef(parent.clone())
            })]),
        )?;
        Ok(outMod)
    }

    pub(crate) fn propagateSubs(
        mut r#mod: metamodelica::Ref<Modifier>,
        mut subs: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    ) -> Result<metamodelica::Ref<Modifier>> {
        let mut r#mod: metamodelica::Ref<Modifier> = r#mod;
        let () = (match &*r#mod {
            MODIFIER {
                subModifiers: __mod_subModifiers,
                ..
            } => {
                assign_variant_field!(r#mod => Modifier::MODIFIER; subModifiers = ModTable::map(__mod_subModifiers.clone(), &({ let __pe_b2 = subs.clone(); move |__pe_a0, __pe_a1| propagateSubMod(&__pe_a0, __pe_a1, &__pe_b2) }))?);
                ()
            }
            _ => (),
        });
        Ok(r#mod)
    }

    pub(crate) fn propagateBinding(
        mut r#mod: metamodelica::Ref<Modifier>,
        mut origin: metamodelica::Ref<InstNode::InstNode>,
        mut parent: metamodelica::Ref<InstNode::InstNode>,
    ) -> metamodelica::Ref<Modifier> {
        let mut r#mod: metamodelica::Ref<Modifier> = r#mod;
        let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
        let () = (match &*r#mod {
            MODIFIER {
                binding: __mod_binding, ..
            } => {
                subs = list![metamodelica::Ref::new(Subscript::NFSubscript::SPLIT_PROXY {
                    origin: NFInstNode::InstNode::scopeRef(origin),
                    parent: NFInstNode::InstNode::scopeRef(parent)
                })];
                assign_variant_field!(r#mod => Modifier::MODIFIER; binding = Binding::propagate(__mod_binding.clone(), subs));
                ()
            }
            _ => (),
        });
        r#mod
    }

    pub(crate) fn propagateSubMod(
        mut name: &ArcStr,
        mut submod: metamodelica::Ref<Modifier>,
        mut subs: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    ) -> Result<metamodelica::Ref<Modifier>> {
        let mut submod: metamodelica::Ref<Modifier> = submod;
        let () = (match &*submod {
            MODIFIER {
                eachPrefix: SCode::Each::NOT_EACH { .. },
                binding: __submod_binding,
                ..
            } => {
                assign_variant_field!(submod => Modifier::MODIFIER;
                    binding = Binding::propagate(__submod_binding.clone(), subs.clone()),
                    subModifiers = ModTable::map(var_field!((*submod).subModifiers, Modifier::MODIFIER).clone(), &({ let __pe_b2 = subs.clone(); move |__pe_a0, __pe_a1| propagateSubMod(&__pe_a0, __pe_a1, &__pe_b2) }))?
                );
                ()
            }
            REDECLARE {
                eachPrefix: SCode::Each::NOT_EACH { .. },
                innerMod: __submod_innerMod,
                ..
            } => {
                assign_variant_field!(submod => Modifier::REDECLARE;
                    innerMod = propagateSubMod(name, __submod_innerMod.clone(), subs)?,
                    propagatedSubs = listAppend(subs.clone(), var_field!((*submod).propagatedSubs, Modifier::REDECLARE).clone())
                );
                ()
            }
            _ => (),
        });
        Ok(submod)
    }

    pub(crate) fn setSource(
        mut r#mod: metamodelica::Ref<Modifier>,
        mut source: Binding::Source,
        mut confidence: i32,
    ) -> Result<metamodelica::Ref<Modifier>> {
        let mut r#mod: metamodelica::Ref<Modifier> = r#mod;
        let () = (match &*r#mod {
            MODIFIER {
                binding: __mod_binding, ..
            } => {
                assign_variant_field!(r#mod => Modifier::MODIFIER;
                    binding = Binding::setConfidence(confidence, Binding::setSource(source, __mod_binding.clone())),
                    subModifiers = ModTable::map(var_field!((*r#mod).subModifiers, Modifier::MODIFIER).clone(), &({ let __pe_b2 = source; let __pe_b3 = confidence; move |__pe_a0, __pe_a1| setSourceSubMod(&__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }))?
                );
                ()
            }
            _ => (),
        });
        Ok(r#mod)
    }

    pub(crate) fn setSourceSubMod(
        mut name: &ArcStr,
        mut submod: metamodelica::Ref<Modifier>,
        mut source: Binding::Source,
        mut confidence: i32,
    ) -> Result<metamodelica::Ref<Modifier>> {
        let mut submod: metamodelica::Ref<Modifier> = submod;
        submod = setSource(submod, source, confidence)?;
        Ok(submod)
    }

    pub(crate) fn isEmpty(mut r#mod: &metamodelica::Ref<Modifier>) -> bool {
        let mut isEmpty: bool;
        isEmpty = (match &**r#mod {
            NOMOD { .. } => true,
            _ => false,
        });
        isEmpty
    }

    pub(crate) fn isRedeclare(mut r#mod: &metamodelica::Ref<Modifier>) -> bool {
        let mut isRedeclare: bool;
        isRedeclare = (match &**r#mod {
            REDECLARE { .. } => true,
            _ => false,
        });
        isRedeclare
    }

    pub(crate) fn toList(mut r#mod: &metamodelica::Ref<Modifier>) -> metamodelica::List<metamodelica::Ref<Modifier>> {
        let mut modList: metamodelica::List<metamodelica::Ref<Modifier>>;
        modList = (match &**r#mod {
            MODIFIER {
                subModifiers: __mod_subModifiers,
                ..
            } => ModTable::listValues(metamodelica::AsArg::as_arg(&__mod_subModifiers), metamodelica::nil()),
            _ => metamodelica::nil(),
        });
        modList
    }

    pub(crate) fn isEach(mut r#mod: &metamodelica::Ref<Modifier>) -> bool {
        let mut isEach: bool;
        isEach = (match &**r#mod {
            MODIFIER {
                eachPrefix: SCode::Each::EACH { .. },
                ..
            } => true,
            _ => false,
        });
        isEach
    }

    pub(crate) fn isFinal(mut r#mod: &metamodelica::Ref<Modifier>) -> bool {
        let mut isFinal: bool;
        isFinal = (match &**r#mod {
            MODIFIER {
                finalPrefix: SCode::Final::FINAL { .. },
                ..
            } => true,
            _ => false,
        });
        isFinal
    }

    pub(crate) fn map(
        mut r#mod: metamodelica::Ref<Modifier>,
        mut func: &dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<Modifier>) -> Result<metamodelica::Ref<Modifier>>,
    ) -> Result<metamodelica::Ref<Modifier>> {
        pub type FuncT = std::sync::Arc<
            dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<Modifier>) -> Result<metamodelica::Ref<Modifier>> + 'static,
        >;

        let mut r#mod: metamodelica::Ref<Modifier> = r#mod;
        let () = (match &*r#mod {
            MODIFIER {
                subModifiers: __mod_subModifiers,
                ..
            } => {
                assign_variant_field!(r#mod => Modifier::MODIFIER; subModifiers = ModTable::map(__mod_subModifiers.clone(), func)?);
                ()
            }
            _ => (),
        });
        Ok(r#mod)
    }

    pub(crate) fn toString(mut r#mod: &metamodelica::Ref<Modifier>, mut printName: bool) -> Result<ArcStr> {
        let mut string: ArcStr;
        string = (match &**r#mod {
            MODIFIER {
                binding: __mod_binding,
                name: __mod_name,
                subModifiers: __mod_subModifiers,
                ..
            } => {
                let mut submods: metamodelica::List<metamodelica::Ref<Modifier>>;
                let mut subs_str: ArcStr;
                let mut binding_str: ArcStr;
                let mut binding_sep: ArcStr;
                submods = ModTable::listValues(metamodelica::AsArg::as_arg(&__mod_subModifiers), metamodelica::nil());
                if !((submods).is_empty()) {
                    subs_str = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("("));
                        __mm_s.push_str(&*stringDelimitList(
                            ({
                                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                                for mut s in (submods).into_iter().cloned() {
                                    let __x = toString(&(s.clone()), true)?;
                                    __acc = cons(__x, __acc);
                                }
                                __acc.reverse()
                            }),
                            literal!(", "),
                        ));
                        __mm_s.push_str(&*literal!(")"));
                        ArcStr::from(__mm_s)
                    };
                    binding_sep = literal!(" = ");
                } else {
                    subs_str = literal!("");
                    binding_sep = if (printName) { literal!(" = ") } else { literal!("= ") };
                }
                binding_str = Binding::toString(metamodelica::AsArg::as_arg(&__mod_binding), &binding_sep)?;
                if (printName) {
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*__mod_name);
                        __mm_s.push_str(&*subs_str);
                        __mm_s.push_str(&*binding_str);
                        ArcStr::from(__mm_s)
                    }
                } else {
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*subs_str);
                        __mm_s.push_str(&*binding_str);
                        ArcStr::from(__mm_s)
                    }
                }
            }
            REDECLARE {
                element: __mod_element, ..
            } => NFInstNode::InstNode::toString(__mod_element.clone())?,
            _ => {
                literal!("")
            }
        });
        Ok(string)
    }

    pub(crate) fn toFlatStreamList(
        mut modifiers: metamodelica::List<metamodelica::Ref<Modifier>>,
        mut format: BaseModelica::OutputFormat,
        mut s: IOStream::IOStream,
        mut delimiter: ArcStr,
    ) -> Result<IOStream::IOStream> {
        let mut s: IOStream::IOStream = s;
        let mut mods: metamodelica::List<metamodelica::Ref<Modifier>> = modifiers;
        if (mods).is_empty() {
            return Ok(s);
        }
        loop {
            s = toFlatStream(&((mods).head().cloned()?), format, s, true)?;
            mods = (mods).rest()?;
            if (mods).is_empty() {
                break;
            } else {
                s = IOStream::append(s, delimiter.clone())?;
            }
        }
        Ok(s)
    }

    pub(crate) fn toFlatStream(
        mut r#mod: &metamodelica::Ref<Modifier>,
        mut format: BaseModelica::OutputFormat,
        mut s: IOStream::IOStream,
        mut printName: bool,
    ) -> Result<IOStream::IOStream> {
        let mut s: IOStream::IOStream = s;
        let mut submods: metamodelica::List<metamodelica::Ref<Modifier>>;
        let mut binding_sep: ArcStr;
        let () = (match &**r#mod {
            MODIFIER {
                binding: __mod_binding,
                name: __mod_name,
                subModifiers: __mod_subModifiers,
                ..
            } => {
                if printName {
                    s = IOStream::append(s, __mod_name.clone())?;
                }
                submods = ModTable::listValues(metamodelica::AsArg::as_arg(&__mod_subModifiers), metamodelica::nil());
                if !((submods).is_empty()) {
                    s = IOStream::append(s, literal!("("))?;
                    s = toFlatStreamList(submods, format, s, literal!(", "))?;
                    s = IOStream::append(s, literal!(")"))?;
                    binding_sep = literal!(" = ");
                } else {
                    binding_sep = if (printName) { literal!(" = ") } else { literal!("= ") };
                }
                s = IOStream::append(s, Binding::toFlatString(__mod_binding.clone(), format, &binding_sep)?)?;
                ()
            }
            _ => (),
        });
        Ok(s)
    }

    pub(crate) fn toFlatString(
        mut r#mod: &metamodelica::Ref<Modifier>,
        mut format: BaseModelica::OutputFormat,
        mut printName: bool,
    ) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        let mut s: IOStream::IOStream;
        s = IOStream::create(
            literal!("NFModifier.Modifier.toFlatString"),
            openmodelica_util::IOStream::IOStreamType::LIST,
        )?;
        s = toFlatStream(r#mod, format, s, printName)?;
        r#str = IOStream::string(&s)?;
        IOStream::delete(&s)?;
        Ok(r#str)
    }

    fn createSubMod(
        mut subMod: &metamodelica::Ref<SCode::SubMod>,
        mut modScope: &metamodelica::Ref<ModifierScope::ModifierScope>,
        mut scope: metamodelica::Ref<InstNode::InstNode>,
        mut confidence: i32,
    ) -> Result<metamodelica::Ref<Modifier>> {
        let mut r#mod: metamodelica::Ref<Modifier> =
            create(&subMod.r#mod, subMod.ident.clone(), modScope, scope.clone(), confidence)?;
        Ok(r#mod)
    }

    fn checkFinalOverride(
        mut innerFinal: SCode::Final,
        mut outerMod: &metamodelica::Ref<Modifier>,
        mut innerInfo: SourceInfo,
    ) -> Result<()> {
        let () = (match innerFinal {
            SCode::Final::FINAL { .. } => {
                Error::addMultiSourceMessage(
                    &(Error::FINAL_COMPONENT_OVERRIDE.clone()),
                    &(list![name(outerMod)?, toString(outerMod, false)?]),
                    &(list![info(outerMod), innerInfo]),
                )?;
                return Err("fail");
            }
            _ => (),
        });
        Ok(())
    }

    fn mergeLocal(
        mut mod1: metamodelica::Ref<Modifier>,
        mut mod2: metamodelica::Ref<Modifier>,
        mut name: &ArcStr,
        mut scope: &metamodelica::Ref<ModifierScope::ModifierScope>,
        mut prefix: metamodelica::List<ArcStr>,
    ) -> Result<metamodelica::Ref<Modifier>> {
        let mut r#mod: metamodelica::Ref<Modifier>;
        let mut comp_name: ArcStr;
        r#mod = (::match_deref::match_deref! { match &((mod1.clone(), mod2.clone())) {
            (Deref @ MODIFIER { .. }, Deref @ MODIFIER { binding: Deref @ Binding::UNBOUND, .. }) => {
                assign_variant_field!(mod1 => Modifier::MODIFIER; subModifiers = ModTable::join(var_field!((*mod1).subModifiers, Modifier::MODIFIER).clone(), var_field!((*mod2).subModifiers, Modifier::MODIFIER), &({ let __pe_b3 = scope.clone(); let __pe_b4 = metamodelica::cons(var_field!((*mod1).name, Modifier::MODIFIER).clone(), prefix); move |__pe_a0, __pe_a1, __pe_a2| mergeLocal(__pe_a0, __pe_a1, &__pe_a2, &__pe_b3, __pe_b4.clone()) }))?);
                mod1
            },
            (Deref @ MODIFIER { binding: Deref @ Binding::UNBOUND, .. }, Deref @ MODIFIER { .. }) => {
                assign_variant_field!(mod2 => Modifier::MODIFIER; subModifiers = ModTable::join(var_field!((*mod2).subModifiers, Modifier::MODIFIER).clone(), &(var_field!((*mod1).subModifiers, Modifier::MODIFIER).clone()), &({ let __pe_b3 = scope.clone(); let __pe_b4 = metamodelica::cons(var_field!((*mod1).name, Modifier::MODIFIER).clone(), prefix); move |__pe_a0, __pe_a1, __pe_a2| mergeLocal(__pe_a0, __pe_a1, &__pe_a2, &__pe_b3, __pe_b4.clone()) }))?);
                mod2
            },
            _ => {
                comp_name = stringDelimitList(metamodelica::cons(self::name(&mod1)?, prefix).reverse(), literal!("."));
                Error::addMultiSourceMessage(&(Error::DUPLICATE_MODIFICATIONS.clone()), &(list![comp_name, ModifierScope::toString(scope)?]), &(list![info(&mod1), info(&mod2)]))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(r#mod)
    }
}
