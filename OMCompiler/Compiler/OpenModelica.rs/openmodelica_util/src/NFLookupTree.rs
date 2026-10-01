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
use crate::BaseAvlTree;

pub mod Entry {
    use super::*;
    #[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum Entry {
        CLASS { index: i32 },
        COMPONENT { index: i32 },
        IMPORT { index: i32 },
    }
    impl metamodelica::gc::MMTrace for Entry {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                Entry::CLASS { index } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    Ok(())
                }
                Entry::COMPONENT { index } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    Ok(())
                }
                Entry::IMPORT { index } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Default for Entry {
        fn default() -> Self {
            Self::CLASS {
                index: Default::default(),
            }
        }
    }
    pub use self::Entry::{CLASS, COMPONENT, IMPORT};
    pub fn index(mut entry: &metamodelica::Ref<Entry>) -> i32 {
        let mut index: i32;
        index = (match &**entry {
            CLASS { index: __entry_index } => __entry_index.clone(),
            COMPONENT { index: __entry_index } => __entry_index.clone(),
            IMPORT { index: __entry_index } => __entry_index.clone(),
        });
        index
    }

    pub fn isEqual(mut entry1: &metamodelica::Ref<Entry>, mut entry2: &metamodelica::Ref<Entry>) -> bool {
        let mut isEqual: bool = index(entry1) == index(entry2);
        isEqual
    }

    pub fn isImport(mut entry: &metamodelica::Ref<Entry>) -> bool {
        let mut isImport: bool;
        isImport = (match &**entry {
            IMPORT { .. } => true,
            _ => false,
        });
        isImport
    }
}

pub(crate) fn keyStr(mut inKey: Key) -> ArcStr {
    let mut outString: ArcStr;
    outString = inKey;
    outString
}

pub(crate) fn valueStr(mut inValue: Value) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match &*inValue {
        Entry::CLASS { index: __inValue_index } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("class "));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", __inValue_index.clone())));
            ArcStr::from(__mm_s)
        }
        Entry::COMPONENT { index: __inValue_index } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("comp "));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", __inValue_index.clone())));
            ArcStr::from(__mm_s)
        }
        _ => return Err("match: no arm matched"),
    });
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

pub type Value = metamodelica::Ref<Entry::Entry>;

pub type ValueNode = ArcStr;

pub fn add(
    mut inTree: metamodelica::Ref<Tree>,
    mut inKey: &Key,
    mut inValue: &Value,
    mut conflictFunc: &dyn ::std::ops::Fn(
        metamodelica::Ref<Entry::Entry>,
        metamodelica::Ref<Entry::Entry>,
        ArcStr,
    ) -> Result<metamodelica::Ref<Entry::Entry>>,
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
                    right: crate::NFLookupTree::Tree::interned_EMPTY(),
                });
            } else if key_comp == 1 {
                outTree = metamodelica::Ref::new(Tree::NODE {
                    key: var_field!((*tree).key, Tree::LEAF).clone(),
                    value: var_field!((*tree).value, Tree::LEAF).clone(),
                    height: 2,
                    left: crate::NFLookupTree::Tree::interned_EMPTY(),
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
    mut inValues: &metamodelica::List<(ArcStr, metamodelica::Ref<Entry::Entry>)>,
    mut conflictFunc: &dyn ::std::ops::Fn(
        metamodelica::Ref<Entry::Entry>,
        metamodelica::Ref<Entry::Entry>,
        ArcStr,
    ) -> Result<metamodelica::Ref<Entry::Entry>>,
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
    mut r#fn: &dyn ::std::ops::Fn(Option<metamodelica::Ref<Entry::Entry>>) -> Result<metamodelica::Ref<Entry::Entry>>,
) -> Result<metamodelica::Ref<Tree>> {
    pub type UpdateFn =
        std::sync::Arc<dyn ::std::ops::Fn(Option<metamodelica::Ref<Entry::Entry>>) -> Result<Value> + 'static>;

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
                    right: crate::NFLookupTree::Tree::interned_EMPTY(),
                });
            } else if key_comp == 1 {
                new_tree = metamodelica::Ref::new(Tree::NODE {
                    key: var_field!((*tree).key, Tree::LEAF).clone(),
                    value: var_field!((*tree).value, Tree::LEAF).clone(),
                    height: 2,
                    left: crate::NFLookupTree::Tree::interned_EMPTY(),
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
        } => height(metamodelica::AsArg::as_arg(&__inNode_left)) - height(metamodelica::AsArg::as_arg(&__inNode_right)),
        Tree::LEAF { .. } => 0,
        _ => 0,
    });
    outBalance
}

pub fn fold<'__b, FT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inTree: &'__b metamodelica::Ref<Tree>,
    mut inFunc: &'__b dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<Entry::Entry>, FT) -> Result<FT>,
    mut inStartValue: FT,
) -> Result<FT> {
    pub type FoldFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<FT> + 'static>;

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
    mut foldFunc: &dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<Entry::Entry>, FT) -> Result<(FT, bool)>,
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
    mut foldFunc: &dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<Entry::Entry>, FT1, FT2) -> Result<(FT1, FT2)>,
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
            (foldArg1, foldArg2) = fold_2(metamodelica::AsArg::as_arg(&__tree_right), foldFunc, foldArg1, foldArg2)?;
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
    mut func: &dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<Entry::Entry>) -> Result<()>,
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
    mut inValues: &metamodelica::List<(ArcStr, metamodelica::Ref<Entry::Entry>)>,
    mut conflictFunc: &dyn ::std::ops::Fn(
        metamodelica::Ref<Entry::Entry>,
        metamodelica::Ref<Entry::Entry>,
        ArcStr,
    ) -> Result<metamodelica::Ref<Entry::Entry>>,
) -> Result<metamodelica::Ref<Tree>> {
    let mut tree: metamodelica::Ref<Tree> = crate::NFLookupTree::Tree::interned_EMPTY();
    let mut key: Key;
    let mut value: Value;
    for mut t in &**inValues {
        (key, value) = t.clone();
        tree = add(tree, &key, &value, conflictFunc)?;
    }
    Ok(tree)
}

pub fn get<'__b>(mut tree: &'__b metamodelica::Ref<Tree>, mut key: Key) -> Result<Value> {
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

pub fn getOpt<'__b>(mut tree: &'__b metamodelica::Ref<Tree>, mut key: Key) -> Option<metamodelica::Ref<Entry::Entry>> {
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
        metamodelica::Ref<Entry::Entry>,
        metamodelica::Ref<Entry::Entry>,
        ArcStr,
    ) -> Result<metamodelica::Ref<Entry::Entry>>,
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
    mut lst: metamodelica::List<metamodelica::Ref<Entry::Entry>>,
) -> metamodelica::List<metamodelica::Ref<Entry::Entry>> {
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

pub fn map(
    mut inTree: metamodelica::Ref<Tree>,
    mut inFunc: &dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<Entry::Entry>) -> Result<metamodelica::Ref<Entry::Entry>>,
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
    mut inFunc: &dyn ::std::ops::Fn(ArcStr, metamodelica::Ref<Entry::Entry>, FT) -> Result<(metamodelica::Ref<Entry::Entry>, FT)>,
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

pub fn new() -> metamodelica::Ref<Tree> {
    let mut outTree: metamodelica::Ref<Tree> = crate::NFLookupTree::Tree::interned_EMPTY();
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
            node = setTreeLeftRight(outNode, __outNode_left.clone(), crate::NFLookupTree::Tree::interned_EMPTY())?;
            setTreeLeftRight(child.clone(), node, crate::NFLookupTree::Tree::interned_EMPTY())?
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
            node = setTreeLeftRight(outNode, crate::NFLookupTree::Tree::interned_EMPTY(), __outNode_right.clone())?;
            setTreeLeftRight(child.clone(), crate::NFLookupTree::Tree::interned_EMPTY(), node)?
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
    mut lst: metamodelica::List<(ArcStr, metamodelica::Ref<Entry::Entry>)>,
) -> metamodelica::List<(ArcStr, metamodelica::Ref<Entry::Entry>)> {
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
    let mut outTree: metamodelica::Ref<Tree> = add(tree.clone(), key, value, &move |__a0: metamodelica::Ref<
        Entry::Entry,
    >,
                                                                                    __a1: metamodelica::Ref<
        Entry::Entry,
    >,
                                                                                    __a2: ArcStr|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(addConflictReplace(__a0, &__a1, &__a2))
    })?;
    Ok(outTree)
}
