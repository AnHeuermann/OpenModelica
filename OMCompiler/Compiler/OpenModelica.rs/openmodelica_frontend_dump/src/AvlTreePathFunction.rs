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

use crate::AbsynUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;

pub type Key = metamodelica::Ref<Absyn::Path>;

pub type Value = Option<DAE::Function>;

pub(crate) fn keyStr(mut inKey: Key) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = AbsynUtil::pathString(inKey, literal!("."), true, false)?;
    Ok(outString)
}

pub(crate) fn valueStr(mut inValue: Value) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inValue {
        Some(DAE::Function::FUNCTION { path: mut path, .. }) => {
            AbsynUtil::pathString(path.clone(), literal!("."), true, false)?
        }
        Some(DAE::Function::RECORD_CONSTRUCTOR { path: mut path, .. }) => {
            AbsynUtil::pathString(path.clone(), literal!("."), true, false)?
        }
        Some(DAE::Function::RECORD_CONSTRUCTOR { .. }) => {
            literal!("<SOME_FUNCTION>")
        }
        _ => {
            literal!("<NO_FUNCTION>")
        }
    });
    Ok(outString)
}

pub(crate) fn keyCompare(mut inKey1: Key, mut inKey2: Key) -> Result<i32> {
    let mut outResult: i32;
    outResult = AbsynUtil::pathCompareNoQual(inKey1, inKey2)?;
    Ok(outResult)
}

pub use addConflictReplace as addConflictDefault;

pub fn addDaeFunction(
    mut functions: &metamodelica::List<DAE::Function>,
    mut functionTree: metamodelica::Ref<Tree>,
) -> Result<metamodelica::Ref<Tree>> {
    let mut functionTree: metamodelica::Ref<Tree> = functionTree;
    for mut f in &**functions {
        functionTree = add(
            functionTree,
            &(functionName(metamodelica::AsArg::as_arg(&f))),
            Some(f.clone()),
            &*(std::sync::Arc::new(fnptr!(addConflictDefault, _, _, _))
                as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
        )?;
    }
    Ok(functionTree)
}

pub(crate) fn addDaeExtFunction(
    mut functions: &metamodelica::List<DAE::Function>,
    mut functionTree: metamodelica::Ref<Tree>,
) -> Result<metamodelica::Ref<Tree>> {
    let mut functionTree: metamodelica::Ref<Tree> = functionTree;
    for mut f in &**functions {
        if isExtFunction(metamodelica::AsArg::as_arg(&f)) {
            functionTree = add(
                functionTree,
                &(functionName(metamodelica::AsArg::as_arg(&f))),
                Some(f.clone()),
                &*(std::sync::Arc::new(fnptr!(addConflictDefault, _, _, _))
                    as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
            )?;
        }
    }
    Ok(functionTree)
}

fn functionName(mut elt: &DAE::Function) -> metamodelica::Ref<Absyn::Path> {
    let mut name: metamodelica::Ref<Absyn::Path>;
    name = (match elt.clone() {
        DAE::Function::FUNCTION {
            path: ref __esc_name, ..
        } => {
            name = __esc_name.clone();
            name.clone()
        }
        DAE::Function::RECORD_CONSTRUCTOR {
            path: ref __esc_name, ..
        } => {
            name = __esc_name.clone();
            name.clone()
        }
    });
    name
}

fn isExtFunction(mut elt: &DAE::Function) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(elt) {
        DAE::Function::FUNCTION { functions: Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { .. }, tail: _ }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub type ConflictFunc = std::sync::Arc<dyn ::std::ops::Fn(Value, Value, Key) -> Result<Value> + 'static>;

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

pub type ValueNode = metamodelica::Ref<Absyn::Path>;

pub fn add(
    mut inTree: metamodelica::Ref<Tree>,
    mut inKey: &Key,
    mut inValue: Value,
    mut conflictFunc: &dyn ::std::ops::Fn(
        Option<DAE::Function>,
        Option<DAE::Function>,
        metamodelica::Ref<Absyn::Path>,
    ) -> Result<Option<DAE::Function>>,
) -> Result<metamodelica::Ref<Tree>> {
    let mut tree: metamodelica::Ref<Tree> = inTree;
    tree = (match &*tree.clone() {
        Tree::EMPTY { .. } => metamodelica::Ref::new(Tree::LEAF {
            key: inKey.clone(),
            value: inValue,
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
                value = conflictFunc(inValue, var_field!((*tree).value, Tree::NODE).clone(), key.clone())?;
                if !(match (&(var_field!((*tree).value, Tree::NODE)), &(value)) {
                    (None, None) => true,
                    (Some(__refeq_l), Some(__refeq_r)) => {
                        (match (&(*__refeq_l), &(*__refeq_r)) {
                            (
                                DAE::Function::FUNCTION {
                                    path: __refeq_v0l,
                                    functions: __refeq_v1l,
                                    type_: __refeq_v2l,
                                    visibility: __refeq_v3l,
                                    partialPrefix: __refeq_v4l,
                                    isImpure: __refeq_v5l,
                                    inlineType: __refeq_v6l,
                                    unusedInputs: __refeq_v7l,
                                    source: __refeq_v8l,
                                    comment: __refeq_v9l,
                                },
                                DAE::Function::FUNCTION {
                                    path: __refeq_v0r,
                                    functions: __refeq_v1r,
                                    type_: __refeq_v2r,
                                    visibility: __refeq_v3r,
                                    partialPrefix: __refeq_v4r,
                                    isImpure: __refeq_v5r,
                                    inlineType: __refeq_v6r,
                                    unusedInputs: __refeq_v7r,
                                    source: __refeq_v8r,
                                    comment: __refeq_v9r,
                                },
                            ) => {
                                referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                                    && metamodelica::ReferenceEq::reference_eq(&(*__refeq_v1l), &(*__refeq_v1r))
                                    && referenceEq(&*(*__refeq_v2l), &*(*__refeq_v2r))
                                    && (match (&(*__refeq_v3l), &(*__refeq_v3r)) {
                                        (SCode::Visibility::PROTECTED, SCode::Visibility::PROTECTED) => true,
                                        (SCode::Visibility::PUBLIC, SCode::Visibility::PUBLIC) => true,
                                        _ => false,
                                    })
                                    && ((*__refeq_v4l) == (*__refeq_v4r))
                                    && ((*__refeq_v5l) == (*__refeq_v5r))
                                    && (match (&(*__refeq_v6l), &(*__refeq_v6r)) {
                                        (
                                            DAE::InlineType::AFTER_INDEX_RED_INLINE,
                                            DAE::InlineType::AFTER_INDEX_RED_INLINE,
                                        ) => true,
                                        (
                                            DAE::InlineType::BUILTIN_EARLY_INLINE,
                                            DAE::InlineType::BUILTIN_EARLY_INLINE,
                                        ) => true,
                                        (DAE::InlineType::DEFAULT_INLINE, DAE::InlineType::DEFAULT_INLINE) => true,
                                        (DAE::InlineType::EARLY_INLINE, DAE::InlineType::EARLY_INLINE) => true,
                                        (DAE::InlineType::NORM_INLINE, DAE::InlineType::NORM_INLINE) => true,
                                        (DAE::InlineType::NO_INLINE, DAE::InlineType::NO_INLINE) => true,
                                        _ => false,
                                    })
                                    && metamodelica::ReferenceEq::reference_eq(&(*__refeq_v7l), &(*__refeq_v7r))
                                    && referenceEq(&*(*__refeq_v8l), &*(*__refeq_v8r))
                                    && (match (&(*__refeq_v9l), &(*__refeq_v9r)) {
                                        (None, None) => true,
                                        (Some(__refeq_l), Some(__refeq_r)) => {
                                            referenceEq(&*(*__refeq_l), &*(*__refeq_r))
                                        }
                                        _ => false,
                                    })
                            }
                            (
                                DAE::Function::RECORD_CONSTRUCTOR {
                                    path: __refeq_v0l,
                                    type_: __refeq_v1l,
                                    source: __refeq_v2l,
                                },
                                DAE::Function::RECORD_CONSTRUCTOR {
                                    path: __refeq_v0r,
                                    type_: __refeq_v1r,
                                    source: __refeq_v2r,
                                },
                            ) => {
                                referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                                    && referenceEq(&*(*__refeq_v1l), &*(*__refeq_v1r))
                                    && referenceEq(&*(*__refeq_v2l), &*(*__refeq_v2r))
                            }
                            _ => false,
                        })
                    }
                    _ => false,
                }) {
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
                        value: inValue,
                    }),
                    right: crate::AvlTreePathFunction::Tree::interned_EMPTY(),
                });
            } else if key_comp == 1 {
                outTree = metamodelica::Ref::new(Tree::NODE {
                    key: var_field!((*tree).key, Tree::LEAF).clone(),
                    value: var_field!((*tree).value, Tree::LEAF).clone(),
                    height: 2,
                    left: crate::AvlTreePathFunction::Tree::interned_EMPTY(),
                    right: metamodelica::Ref::new(Tree::LEAF {
                        key: inKey.clone(),
                        value: inValue,
                    }),
                });
            } else {
                value = conflictFunc(
                    inValue,
                    var_field!((*tree).value, Tree::LEAF).clone(),
                    var_field!((*tree).key, Tree::LEAF).clone(),
                )?;
                if !(match (&(var_field!((*tree).value, Tree::LEAF)), &(value)) {
                    (None, None) => true,
                    (Some(__refeq_l), Some(__refeq_r)) => {
                        (match (&(*__refeq_l), &(*__refeq_r)) {
                            (
                                DAE::Function::FUNCTION {
                                    path: __refeq_v0l,
                                    functions: __refeq_v1l,
                                    type_: __refeq_v2l,
                                    visibility: __refeq_v3l,
                                    partialPrefix: __refeq_v4l,
                                    isImpure: __refeq_v5l,
                                    inlineType: __refeq_v6l,
                                    unusedInputs: __refeq_v7l,
                                    source: __refeq_v8l,
                                    comment: __refeq_v9l,
                                },
                                DAE::Function::FUNCTION {
                                    path: __refeq_v0r,
                                    functions: __refeq_v1r,
                                    type_: __refeq_v2r,
                                    visibility: __refeq_v3r,
                                    partialPrefix: __refeq_v4r,
                                    isImpure: __refeq_v5r,
                                    inlineType: __refeq_v6r,
                                    unusedInputs: __refeq_v7r,
                                    source: __refeq_v8r,
                                    comment: __refeq_v9r,
                                },
                            ) => {
                                referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                                    && metamodelica::ReferenceEq::reference_eq(&(*__refeq_v1l), &(*__refeq_v1r))
                                    && referenceEq(&*(*__refeq_v2l), &*(*__refeq_v2r))
                                    && (match (&(*__refeq_v3l), &(*__refeq_v3r)) {
                                        (SCode::Visibility::PROTECTED, SCode::Visibility::PROTECTED) => true,
                                        (SCode::Visibility::PUBLIC, SCode::Visibility::PUBLIC) => true,
                                        _ => false,
                                    })
                                    && ((*__refeq_v4l) == (*__refeq_v4r))
                                    && ((*__refeq_v5l) == (*__refeq_v5r))
                                    && (match (&(*__refeq_v6l), &(*__refeq_v6r)) {
                                        (
                                            DAE::InlineType::AFTER_INDEX_RED_INLINE,
                                            DAE::InlineType::AFTER_INDEX_RED_INLINE,
                                        ) => true,
                                        (
                                            DAE::InlineType::BUILTIN_EARLY_INLINE,
                                            DAE::InlineType::BUILTIN_EARLY_INLINE,
                                        ) => true,
                                        (DAE::InlineType::DEFAULT_INLINE, DAE::InlineType::DEFAULT_INLINE) => true,
                                        (DAE::InlineType::EARLY_INLINE, DAE::InlineType::EARLY_INLINE) => true,
                                        (DAE::InlineType::NORM_INLINE, DAE::InlineType::NORM_INLINE) => true,
                                        (DAE::InlineType::NO_INLINE, DAE::InlineType::NO_INLINE) => true,
                                        _ => false,
                                    })
                                    && metamodelica::ReferenceEq::reference_eq(&(*__refeq_v7l), &(*__refeq_v7r))
                                    && referenceEq(&*(*__refeq_v8l), &*(*__refeq_v8r))
                                    && (match (&(*__refeq_v9l), &(*__refeq_v9r)) {
                                        (None, None) => true,
                                        (Some(__refeq_l), Some(__refeq_r)) => {
                                            referenceEq(&*(*__refeq_l), &*(*__refeq_r))
                                        }
                                        _ => false,
                                    })
                            }
                            (
                                DAE::Function::RECORD_CONSTRUCTOR {
                                    path: __refeq_v0l,
                                    type_: __refeq_v1l,
                                    source: __refeq_v2l,
                                },
                                DAE::Function::RECORD_CONSTRUCTOR {
                                    path: __refeq_v0r,
                                    type_: __refeq_v1r,
                                    source: __refeq_v2r,
                                },
                            ) => {
                                referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                                    && referenceEq(&*(*__refeq_v1l), &*(*__refeq_v1r))
                                    && referenceEq(&*(*__refeq_v2l), &*(*__refeq_v2r))
                            }
                            _ => false,
                        })
                    }
                    _ => false,
                }) {
                    assign_variant_field!(tree => Tree::LEAF; value = value);
                }
                outTree = tree;
            }
            if (key_comp == 0) { outTree } else { balance(outTree)? }
        }
    });
    Ok(tree)
}

pub(crate) fn addConflictFail(mut newValue: Value, mut oldValue: Value, mut key: &Key) -> Result<Value> {
    let mut value: Value;
    return Err("fail");
    Ok(value)
}

pub(crate) fn addConflictKeep(mut newValue: Value, mut oldValue: Value, mut key: &Key) -> Value {
    let mut value: Value = oldValue;
    value
}

pub fn addConflictReplace(mut newValue: Value, mut oldValue: Value, mut key: Key) -> Value {
    let mut value: Value = newValue;
    value
}

pub fn addList(
    mut tree: metamodelica::Ref<Tree>,
    mut inValues: &metamodelica::List<(metamodelica::Ref<Absyn::Path>, Option<DAE::Function>)>,
    mut conflictFunc: &dyn ::std::ops::Fn(
        Option<DAE::Function>,
        Option<DAE::Function>,
        metamodelica::Ref<Absyn::Path>,
    ) -> Result<Option<DAE::Function>>,
) -> Result<metamodelica::Ref<Tree>> {
    let mut tree: metamodelica::Ref<Tree> = tree;
    let mut key: Key;
    let mut value: Value;
    for mut t in &**inValues {
        (key, value) = t.clone();
        tree = add(tree, &key, value, conflictFunc)?;
    }
    Ok(tree)
}

pub(crate) fn addUpdate(
    mut tree: metamodelica::Ref<Tree>,
    mut key: &Key,
    mut r#fn: &dyn ::std::ops::Fn(Option<Option<DAE::Function>>) -> Result<Option<DAE::Function>>,
) -> Result<metamodelica::Ref<Tree>> {
    pub type UpdateFn = std::sync::Arc<dyn ::std::ops::Fn(Option<Option<DAE::Function>>) -> Result<Value> + 'static>;

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
                    right: crate::AvlTreePathFunction::Tree::interned_EMPTY(),
                });
            } else if key_comp == 1 {
                new_tree = metamodelica::Ref::new(Tree::NODE {
                    key: var_field!((*tree).key, Tree::LEAF).clone(),
                    value: var_field!((*tree).value, Tree::LEAF).clone(),
                    height: 2,
                    left: crate::AvlTreePathFunction::Tree::interned_EMPTY(),
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

pub(crate) fn fold<'__b, FT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inTree: &'__b metamodelica::Ref<Tree>,
    mut inFunc: &'__b dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>, Option<DAE::Function>, FT) -> Result<FT>,
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
    mut foldFunc: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>, Option<DAE::Function>, FT) -> Result<(FT, bool)>,
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
    mut foldFunc: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>, Option<DAE::Function>, FT1, FT2) -> Result<(FT1, FT2)>,
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
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>, Option<DAE::Function>) -> Result<()>,
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
    mut inValues: &metamodelica::List<(metamodelica::Ref<Absyn::Path>, Option<DAE::Function>)>,
    mut conflictFunc: &dyn ::std::ops::Fn(
        Option<DAE::Function>,
        Option<DAE::Function>,
        metamodelica::Ref<Absyn::Path>,
    ) -> Result<Option<DAE::Function>>,
) -> Result<metamodelica::Ref<Tree>> {
    let mut tree: metamodelica::Ref<Tree> = crate::AvlTreePathFunction::Tree::interned_EMPTY();
    let mut key: Key;
    let mut value: Value;
    for mut t in &**inValues {
        (key, value) = t.clone();
        tree = add(tree, &key, value, conflictFunc)?;
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
    value = (::match_deref::match_deref! { match &((keyCompare(key.clone(), k)?, tree.clone())) {
        (0, Deref @ Tree::LEAF { .. }) => var_field!((**tree).value, Tree::LEAF).clone(),
        (0, Deref @ Tree::NODE { .. }) => var_field!((**tree).value, Tree::NODE).clone(),
        (1, Deref @ Tree::NODE { .. }) => get(var_field!((**tree).right, Tree::NODE), key)?,
        ((-1), Deref @ Tree::NODE { .. }) => get(var_field!((**tree).left, Tree::NODE), key)?,
        _ => return Err("match: no arm matched"),
    } });
    Ok(value)
}

pub fn getOpt<'__b>(mut tree: &'__b metamodelica::Ref<Tree>, mut key: Key) -> Result<Option<Option<DAE::Function>>> {
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

pub fn join<'__b>(
    mut tree: metamodelica::Ref<Tree>,
    mut treeToJoin: &'__b metamodelica::Ref<Tree>,
    mut conflictFunc: &'__b dyn ::std::ops::Fn(
        Option<DAE::Function>,
        Option<DAE::Function>,
        metamodelica::Ref<Absyn::Path>,
    ) -> Result<Option<DAE::Function>>,
) -> Result<metamodelica::Ref<Tree>> {
    '__tco: loop {
        match &**treeToJoin {
            Tree::EMPTY { .. } => return Ok(tree),
            Tree::NODE { .. } => {
                tree = add(
                    tree,
                    var_field!((**treeToJoin).key, Tree::NODE),
                    var_field!((**treeToJoin).value, Tree::NODE).clone(),
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
                    var_field!((**treeToJoin).value, Tree::LEAF).clone(),
                    conflictFunc,
                )?);
            }
        }
    }
}

pub(crate) fn listKeys<'__b>(
    mut tree: &'__b metamodelica::Ref<Tree>,
    mut lst: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Path>> {
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
    mut lst: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Path>> {
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

pub fn listValues<'__b>(
    mut tree: &'__b metamodelica::Ref<Tree>,
    mut lst: metamodelica::List<Option<DAE::Function>>,
) -> metamodelica::List<Option<DAE::Function>> {
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
    mut inFunc: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>, Option<DAE::Function>) -> Result<Option<DAE::Function>>,
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
                || !(match (&(value), &(new_value)) {
                    (None, None) => true,
                    (Some(__refeq_l), Some(__refeq_r)) => {
                        (match (&(*__refeq_l), &(*__refeq_r)) {
                            (
                                DAE::Function::FUNCTION {
                                    path: __refeq_v0l,
                                    functions: __refeq_v1l,
                                    type_: __refeq_v2l,
                                    visibility: __refeq_v3l,
                                    partialPrefix: __refeq_v4l,
                                    isImpure: __refeq_v5l,
                                    inlineType: __refeq_v6l,
                                    unusedInputs: __refeq_v7l,
                                    source: __refeq_v8l,
                                    comment: __refeq_v9l,
                                },
                                DAE::Function::FUNCTION {
                                    path: __refeq_v0r,
                                    functions: __refeq_v1r,
                                    type_: __refeq_v2r,
                                    visibility: __refeq_v3r,
                                    partialPrefix: __refeq_v4r,
                                    isImpure: __refeq_v5r,
                                    inlineType: __refeq_v6r,
                                    unusedInputs: __refeq_v7r,
                                    source: __refeq_v8r,
                                    comment: __refeq_v9r,
                                },
                            ) => {
                                referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                                    && metamodelica::ReferenceEq::reference_eq(&(*__refeq_v1l), &(*__refeq_v1r))
                                    && referenceEq(&*(*__refeq_v2l), &*(*__refeq_v2r))
                                    && (match (&(*__refeq_v3l), &(*__refeq_v3r)) {
                                        (SCode::Visibility::PROTECTED, SCode::Visibility::PROTECTED) => true,
                                        (SCode::Visibility::PUBLIC, SCode::Visibility::PUBLIC) => true,
                                        _ => false,
                                    })
                                    && ((*__refeq_v4l) == (*__refeq_v4r))
                                    && ((*__refeq_v5l) == (*__refeq_v5r))
                                    && (match (&(*__refeq_v6l), &(*__refeq_v6r)) {
                                        (
                                            DAE::InlineType::AFTER_INDEX_RED_INLINE,
                                            DAE::InlineType::AFTER_INDEX_RED_INLINE,
                                        ) => true,
                                        (
                                            DAE::InlineType::BUILTIN_EARLY_INLINE,
                                            DAE::InlineType::BUILTIN_EARLY_INLINE,
                                        ) => true,
                                        (DAE::InlineType::DEFAULT_INLINE, DAE::InlineType::DEFAULT_INLINE) => true,
                                        (DAE::InlineType::EARLY_INLINE, DAE::InlineType::EARLY_INLINE) => true,
                                        (DAE::InlineType::NORM_INLINE, DAE::InlineType::NORM_INLINE) => true,
                                        (DAE::InlineType::NO_INLINE, DAE::InlineType::NO_INLINE) => true,
                                        _ => false,
                                    })
                                    && metamodelica::ReferenceEq::reference_eq(&(*__refeq_v7l), &(*__refeq_v7r))
                                    && referenceEq(&*(*__refeq_v8l), &*(*__refeq_v8r))
                                    && (match (&(*__refeq_v9l), &(*__refeq_v9r)) {
                                        (None, None) => true,
                                        (Some(__refeq_l), Some(__refeq_r)) => {
                                            referenceEq(&*(*__refeq_l), &*(*__refeq_r))
                                        }
                                        _ => false,
                                    })
                            }
                            (
                                DAE::Function::RECORD_CONSTRUCTOR {
                                    path: __refeq_v0l,
                                    type_: __refeq_v1l,
                                    source: __refeq_v2l,
                                },
                                DAE::Function::RECORD_CONSTRUCTOR {
                                    path: __refeq_v0r,
                                    type_: __refeq_v1r,
                                    source: __refeq_v2r,
                                },
                            ) => {
                                referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                                    && referenceEq(&*(*__refeq_v1l), &*(*__refeq_v1r))
                                    && referenceEq(&*(*__refeq_v2l), &*(*__refeq_v2r))
                            }
                            _ => false,
                        })
                    }
                    _ => false,
                })
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
            if !(match (&(value), &(new_value)) {
                (None, None) => true,
                (Some(__refeq_l), Some(__refeq_r)) => {
                    (match (&(*__refeq_l), &(*__refeq_r)) {
                        (
                            DAE::Function::FUNCTION {
                                path: __refeq_v0l,
                                functions: __refeq_v1l,
                                type_: __refeq_v2l,
                                visibility: __refeq_v3l,
                                partialPrefix: __refeq_v4l,
                                isImpure: __refeq_v5l,
                                inlineType: __refeq_v6l,
                                unusedInputs: __refeq_v7l,
                                source: __refeq_v8l,
                                comment: __refeq_v9l,
                            },
                            DAE::Function::FUNCTION {
                                path: __refeq_v0r,
                                functions: __refeq_v1r,
                                type_: __refeq_v2r,
                                visibility: __refeq_v3r,
                                partialPrefix: __refeq_v4r,
                                isImpure: __refeq_v5r,
                                inlineType: __refeq_v6r,
                                unusedInputs: __refeq_v7r,
                                source: __refeq_v8r,
                                comment: __refeq_v9r,
                            },
                        ) => {
                            referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                                && metamodelica::ReferenceEq::reference_eq(&(*__refeq_v1l), &(*__refeq_v1r))
                                && referenceEq(&*(*__refeq_v2l), &*(*__refeq_v2r))
                                && (match (&(*__refeq_v3l), &(*__refeq_v3r)) {
                                    (SCode::Visibility::PROTECTED, SCode::Visibility::PROTECTED) => true,
                                    (SCode::Visibility::PUBLIC, SCode::Visibility::PUBLIC) => true,
                                    _ => false,
                                })
                                && ((*__refeq_v4l) == (*__refeq_v4r))
                                && ((*__refeq_v5l) == (*__refeq_v5r))
                                && (match (&(*__refeq_v6l), &(*__refeq_v6r)) {
                                    (
                                        DAE::InlineType::AFTER_INDEX_RED_INLINE,
                                        DAE::InlineType::AFTER_INDEX_RED_INLINE,
                                    ) => true,
                                    (DAE::InlineType::BUILTIN_EARLY_INLINE, DAE::InlineType::BUILTIN_EARLY_INLINE) => {
                                        true
                                    }
                                    (DAE::InlineType::DEFAULT_INLINE, DAE::InlineType::DEFAULT_INLINE) => true,
                                    (DAE::InlineType::EARLY_INLINE, DAE::InlineType::EARLY_INLINE) => true,
                                    (DAE::InlineType::NORM_INLINE, DAE::InlineType::NORM_INLINE) => true,
                                    (DAE::InlineType::NO_INLINE, DAE::InlineType::NO_INLINE) => true,
                                    _ => false,
                                })
                                && metamodelica::ReferenceEq::reference_eq(&(*__refeq_v7l), &(*__refeq_v7r))
                                && referenceEq(&*(*__refeq_v8l), &*(*__refeq_v8r))
                                && (match (&(*__refeq_v9l), &(*__refeq_v9r)) {
                                    (None, None) => true,
                                    (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l), &*(*__refeq_r)),
                                    _ => false,
                                })
                        }
                        (
                            DAE::Function::RECORD_CONSTRUCTOR {
                                path: __refeq_v0l,
                                type_: __refeq_v1l,
                                source: __refeq_v2l,
                            },
                            DAE::Function::RECORD_CONSTRUCTOR {
                                path: __refeq_v0r,
                                type_: __refeq_v1r,
                                source: __refeq_v2r,
                            },
                        ) => {
                            referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                                && referenceEq(&*(*__refeq_v1l), &*(*__refeq_v1r))
                                && referenceEq(&*(*__refeq_v2l), &*(*__refeq_v2r))
                        }
                        _ => false,
                    })
                }
                _ => false,
            }) {
                assign_variant_field!(outTree => Tree::LEAF; value = new_value);
            }
            outTree
        }
        _ => inTree,
    });
    Ok(outTree)
}

pub fn mapFold<FT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inTree: metamodelica::Ref<Tree>,
    mut inFunc: &dyn ::std::ops::Fn(
        metamodelica::Ref<Absyn::Path>,
        Option<DAE::Function>,
        FT,
    ) -> Result<(Option<DAE::Function>, FT)>,
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
                || !(match (&(value), &(new_value)) {
                    (None, None) => true,
                    (Some(__refeq_l), Some(__refeq_r)) => {
                        (match (&(*__refeq_l), &(*__refeq_r)) {
                            (
                                DAE::Function::FUNCTION {
                                    path: __refeq_v0l,
                                    functions: __refeq_v1l,
                                    type_: __refeq_v2l,
                                    visibility: __refeq_v3l,
                                    partialPrefix: __refeq_v4l,
                                    isImpure: __refeq_v5l,
                                    inlineType: __refeq_v6l,
                                    unusedInputs: __refeq_v7l,
                                    source: __refeq_v8l,
                                    comment: __refeq_v9l,
                                },
                                DAE::Function::FUNCTION {
                                    path: __refeq_v0r,
                                    functions: __refeq_v1r,
                                    type_: __refeq_v2r,
                                    visibility: __refeq_v3r,
                                    partialPrefix: __refeq_v4r,
                                    isImpure: __refeq_v5r,
                                    inlineType: __refeq_v6r,
                                    unusedInputs: __refeq_v7r,
                                    source: __refeq_v8r,
                                    comment: __refeq_v9r,
                                },
                            ) => {
                                referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                                    && metamodelica::ReferenceEq::reference_eq(&(*__refeq_v1l), &(*__refeq_v1r))
                                    && referenceEq(&*(*__refeq_v2l), &*(*__refeq_v2r))
                                    && (match (&(*__refeq_v3l), &(*__refeq_v3r)) {
                                        (SCode::Visibility::PROTECTED, SCode::Visibility::PROTECTED) => true,
                                        (SCode::Visibility::PUBLIC, SCode::Visibility::PUBLIC) => true,
                                        _ => false,
                                    })
                                    && ((*__refeq_v4l) == (*__refeq_v4r))
                                    && ((*__refeq_v5l) == (*__refeq_v5r))
                                    && (match (&(*__refeq_v6l), &(*__refeq_v6r)) {
                                        (
                                            DAE::InlineType::AFTER_INDEX_RED_INLINE,
                                            DAE::InlineType::AFTER_INDEX_RED_INLINE,
                                        ) => true,
                                        (
                                            DAE::InlineType::BUILTIN_EARLY_INLINE,
                                            DAE::InlineType::BUILTIN_EARLY_INLINE,
                                        ) => true,
                                        (DAE::InlineType::DEFAULT_INLINE, DAE::InlineType::DEFAULT_INLINE) => true,
                                        (DAE::InlineType::EARLY_INLINE, DAE::InlineType::EARLY_INLINE) => true,
                                        (DAE::InlineType::NORM_INLINE, DAE::InlineType::NORM_INLINE) => true,
                                        (DAE::InlineType::NO_INLINE, DAE::InlineType::NO_INLINE) => true,
                                        _ => false,
                                    })
                                    && metamodelica::ReferenceEq::reference_eq(&(*__refeq_v7l), &(*__refeq_v7r))
                                    && referenceEq(&*(*__refeq_v8l), &*(*__refeq_v8r))
                                    && (match (&(*__refeq_v9l), &(*__refeq_v9r)) {
                                        (None, None) => true,
                                        (Some(__refeq_l), Some(__refeq_r)) => {
                                            referenceEq(&*(*__refeq_l), &*(*__refeq_r))
                                        }
                                        _ => false,
                                    })
                            }
                            (
                                DAE::Function::RECORD_CONSTRUCTOR {
                                    path: __refeq_v0l,
                                    type_: __refeq_v1l,
                                    source: __refeq_v2l,
                                },
                                DAE::Function::RECORD_CONSTRUCTOR {
                                    path: __refeq_v0r,
                                    type_: __refeq_v1r,
                                    source: __refeq_v2r,
                                },
                            ) => {
                                referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                                    && referenceEq(&*(*__refeq_v1l), &*(*__refeq_v1r))
                                    && referenceEq(&*(*__refeq_v2l), &*(*__refeq_v2r))
                            }
                            _ => false,
                        })
                    }
                    _ => false,
                })
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
            if !(match (&(value), &(new_value)) {
                (None, None) => true,
                (Some(__refeq_l), Some(__refeq_r)) => {
                    (match (&(*__refeq_l), &(*__refeq_r)) {
                        (
                            DAE::Function::FUNCTION {
                                path: __refeq_v0l,
                                functions: __refeq_v1l,
                                type_: __refeq_v2l,
                                visibility: __refeq_v3l,
                                partialPrefix: __refeq_v4l,
                                isImpure: __refeq_v5l,
                                inlineType: __refeq_v6l,
                                unusedInputs: __refeq_v7l,
                                source: __refeq_v8l,
                                comment: __refeq_v9l,
                            },
                            DAE::Function::FUNCTION {
                                path: __refeq_v0r,
                                functions: __refeq_v1r,
                                type_: __refeq_v2r,
                                visibility: __refeq_v3r,
                                partialPrefix: __refeq_v4r,
                                isImpure: __refeq_v5r,
                                inlineType: __refeq_v6r,
                                unusedInputs: __refeq_v7r,
                                source: __refeq_v8r,
                                comment: __refeq_v9r,
                            },
                        ) => {
                            referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                                && metamodelica::ReferenceEq::reference_eq(&(*__refeq_v1l), &(*__refeq_v1r))
                                && referenceEq(&*(*__refeq_v2l), &*(*__refeq_v2r))
                                && (match (&(*__refeq_v3l), &(*__refeq_v3r)) {
                                    (SCode::Visibility::PROTECTED, SCode::Visibility::PROTECTED) => true,
                                    (SCode::Visibility::PUBLIC, SCode::Visibility::PUBLIC) => true,
                                    _ => false,
                                })
                                && ((*__refeq_v4l) == (*__refeq_v4r))
                                && ((*__refeq_v5l) == (*__refeq_v5r))
                                && (match (&(*__refeq_v6l), &(*__refeq_v6r)) {
                                    (
                                        DAE::InlineType::AFTER_INDEX_RED_INLINE,
                                        DAE::InlineType::AFTER_INDEX_RED_INLINE,
                                    ) => true,
                                    (DAE::InlineType::BUILTIN_EARLY_INLINE, DAE::InlineType::BUILTIN_EARLY_INLINE) => {
                                        true
                                    }
                                    (DAE::InlineType::DEFAULT_INLINE, DAE::InlineType::DEFAULT_INLINE) => true,
                                    (DAE::InlineType::EARLY_INLINE, DAE::InlineType::EARLY_INLINE) => true,
                                    (DAE::InlineType::NORM_INLINE, DAE::InlineType::NORM_INLINE) => true,
                                    (DAE::InlineType::NO_INLINE, DAE::InlineType::NO_INLINE) => true,
                                    _ => false,
                                })
                                && metamodelica::ReferenceEq::reference_eq(&(*__refeq_v7l), &(*__refeq_v7r))
                                && referenceEq(&*(*__refeq_v8l), &*(*__refeq_v8r))
                                && (match (&(*__refeq_v9l), &(*__refeq_v9r)) {
                                    (None, None) => true,
                                    (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l), &*(*__refeq_r)),
                                    _ => false,
                                })
                        }
                        (
                            DAE::Function::RECORD_CONSTRUCTOR {
                                path: __refeq_v0l,
                                type_: __refeq_v1l,
                                source: __refeq_v2l,
                            },
                            DAE::Function::RECORD_CONSTRUCTOR {
                                path: __refeq_v0r,
                                type_: __refeq_v1r,
                                source: __refeq_v2r,
                            },
                        ) => {
                            referenceEq(&*(*__refeq_v0l), &*(*__refeq_v0r))
                                && referenceEq(&*(*__refeq_v1l), &*(*__refeq_v1r))
                                && referenceEq(&*(*__refeq_v2l), &*(*__refeq_v2r))
                        }
                        _ => false,
                    })
                }
                _ => false,
            }) {
                assign_variant_field!(outTree => Tree::LEAF; value = new_value);
            }
            outTree
        }
        _ => inTree,
    });
    Ok((outTree, outResult))
}

pub fn new() -> metamodelica::Ref<Tree> {
    let mut outTree: metamodelica::Ref<Tree> = crate::AvlTreePathFunction::Tree::interned_EMPTY();
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
            __mm_s.push_str(&*keyStr(__inNode_key.clone())?);
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
            __mm_s.push_str(&*keyStr(__inNode_key.clone())?);
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
            node = setTreeLeftRight(outNode, __outNode_left.clone(), crate::AvlTreePathFunction::Tree::interned_EMPTY())?;
            setTreeLeftRight(child.clone(), node, crate::AvlTreePathFunction::Tree::interned_EMPTY())?
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
            node = setTreeLeftRight(outNode, crate::AvlTreePathFunction::Tree::interned_EMPTY(), __outNode_right.clone())?;
            setTreeLeftRight(child.clone(), crate::AvlTreePathFunction::Tree::interned_EMPTY(), node)?
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

pub fn toList<'__b>(
    mut inTree: &'__b metamodelica::Ref<Tree>,
    mut lst: metamodelica::List<(metamodelica::Ref<Absyn::Path>, Option<DAE::Function>)>,
) -> metamodelica::List<(metamodelica::Ref<Absyn::Path>, Option<DAE::Function>)> {
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
    mut value: Value,
) -> Result<metamodelica::Ref<Tree>> {
    let mut outTree: metamodelica::Ref<Tree> = add(
        tree.clone(),
        key,
        value.clone(),
        &fnptr!(
            addConflictReplace,
            Option<DAE::Function>,
            Option<DAE::Function>,
            metamodelica::Ref<Absyn::Path>
        ),
    )?;
    Ok(outTree)
}
