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
use openmodelica_ast::Absyn::Path;
use openmodelica_ast::GlobalScript;
use openmodelica_backend::GlobalScriptDump;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_loader::Parser;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum ConversionRule {
    /// convertClass
    CLASS {
        oldPath: metamodelica::Array<ArcStr>,
        newPath: metamodelica::Ref<Path>,
    },
    /// convertClassIf (not yet implemented)
    CLASS_IF,
    /// convertElement
    ELEMENT {
        oldPath: metamodelica::Array<ArcStr>,
        oldName: ArcStr,
        newName: ArcStr,
    },
    /// convertModifiers
    MODIFIERS {
        oldMods: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
        newMods: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
        info: SourceInfo,
    },
    /// convertMessage
    MESSAGE { message: ArcStr },
}
impl metamodelica::gc::MMTrace for ConversionRule {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ConversionRule::CLASS { oldPath, newPath } => {
                metamodelica::gc::MMTrace::mm_accept(oldPath, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(newPath, __mmv)?;
                Ok(())
            }
            ConversionRule::CLASS_IF => Ok(()),
            ConversionRule::ELEMENT {
                oldPath,
                oldName,
                newName,
            } => {
                metamodelica::gc::MMTrace::mm_accept(oldPath, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(oldName, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(newName, __mmv)?;
                Ok(())
            }
            ConversionRule::MODIFIERS { oldMods, newMods, info } => {
                metamodelica::gc::MMTrace::mm_accept(oldMods, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(newMods, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(info, __mmv)?;
                Ok(())
            }
            ConversionRule::MESSAGE { message } => {
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for ConversionRule {
    fn default() -> Self {
        Self::CLASS_IF
    }
}
pub use self::ConversionRule::{CLASS, CLASS_IF, ELEMENT, MESSAGE, MODIFIERS};

pub mod ConversionRules {
    use super::*;
    /// Structure used to store conversion rules. Each node corresponds to one
    ///     element, and each node has a map of child nodes and a list of rules. So
    ///     e.g. convertClass('A.B', 'A.C') becomes
    ///     A(nodes = {B(nodes = {}, rules = {convertClass(A.C)})}, rules = {})
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct ConversionRules {
        pub nodes: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::Ref<ConversionRules>>>,
        pub rules: metamodelica::List<ConversionRule>,
    }

    impl metamodelica::gc::MMTrace for ConversionRules {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.nodes, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.rules, __mmv)?;
            Ok(())
        }
    }
    impl Default for ConversionRules {
        fn default() -> Self {
            Self {
                nodes: Default::default(),
                rules: Default::default(),
            }
        }
    }

    pub type CONVERSION_RULES = ConversionRules;

    pub(crate) fn newNode() -> metamodelica::Ref<ConversionRules> {
        let mut node: metamodelica::Ref<ConversionRules>;
        node = metamodelica::Ref::new(ConversionRules {
            nodes: UnorderedMap::new(
                (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                    as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
                (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                    as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
                1,
            ),
            rules: metamodelica::nil(),
        });
        node
    }
}

pub type RuleList = metamodelica::List<ConversionRule>;

pub type RuleTable = metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::List<ConversionRule>>>;

pub type TypeTable = metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::Ref<Path>>>;

// Used to specify which arguments to the conversion functions can be vectorized.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum ArgType {
    SCALAR = 1,
    ARRAY = 2,
}
impl PartialOrd for ArgType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ArgType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for ArgType {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub(crate) static CONVERT_CLASS_TYPE: std::sync::LazyLock<metamodelica::List<ArgType>> =
    std::sync::LazyLock::new(|| list![ArgType::SCALAR.clone(), ArgType::SCALAR.clone()]);

pub(crate) static CONVERT_CLASS_IF_TYPE: std::sync::LazyLock<metamodelica::List<ArgType>> =
    std::sync::LazyLock::new(|| {
        list![
            ArgType::SCALAR.clone(),
            ArgType::SCALAR.clone(),
            ArgType::SCALAR.clone(),
            ArgType::SCALAR.clone()
        ]
    });

pub(crate) static CONVERT_ELEMENT_TYPE: std::sync::LazyLock<metamodelica::List<ArgType>> =
    std::sync::LazyLock::new(|| {
        list![
            ArgType::SCALAR.clone(),
            ArgType::SCALAR.clone(),
            ArgType::SCALAR.clone()
        ]
    });

pub(crate) static CONVERT_MODIFIER_TYPE: std::sync::LazyLock<metamodelica::List<ArgType>> =
    std::sync::LazyLock::new(|| {
        list![
            ArgType::SCALAR.clone(),
            ArgType::ARRAY.clone(),
            ArgType::ARRAY.clone(),
            ArgType::SCALAR.clone()
        ]
    });

pub(crate) static CONVERT_MESSAGE_TYPE: std::sync::LazyLock<metamodelica::List<ArgType>> =
    std::sync::LazyLock::new(|| {
        list![
            ArgType::SCALAR.clone(),
            ArgType::SCALAR.clone(),
            ArgType::SCALAR.clone()
        ]
    });

/// Struct for storing import data.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ImportData {
    /// The import before conversion
    pub originalPath: metamodelica::Ref<Path>,
    /// The import after conversion
    pub convertedPath: metamodelica::Ref<Path>,
    /// The import name after conversion (same as before for
    ///                         named imports, possibly different for qualified imports)
    pub importName: ArcStr,
    /// Shadowed by another element or not
    pub shadowed: bool,
}

impl metamodelica::gc::MMTrace for ImportData {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.originalPath, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.convertedPath, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.importName, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.shadowed, __mmv)?;
        Ok(())
    }
}
impl Default for ImportData {
    fn default() -> Self {
        Self {
            originalPath: Default::default(),
            convertedPath: Default::default(),
            importName: Default::default(),
            shadowed: Default::default(),
        }
    }
}

pub type IMPORT_DATA = ImportData;

pub type ImportTree = metamodelica::Ref<ImportTreeImpl::Tree>;

pub mod ImportTreeImpl {
    use super::*;
    pub type Key = ArcStr;

    pub type Value = ImportData;

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

    pub use addConflictReplace as addConflictDefault;

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
        mut conflictFunc: &dyn ::std::ops::Fn(ImportData, ImportData, ArcStr) -> Result<ImportData>,
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
                    if !({
                        let __refeq_sl = &(var_field!((*tree).value, Tree::NODE).clone());
                        let __refeq_sr = &(value.clone());
                        referenceEq(&*(__refeq_sl.originalPath), &*(__refeq_sr.originalPath))
                            && referenceEq(&*(__refeq_sl.convertedPath), &*(__refeq_sr.convertedPath))
                            && referenceEq(&*(__refeq_sl.importName), &*(__refeq_sr.importName))
                            && ((__refeq_sl.shadowed) == (__refeq_sr.shadowed))
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
                        right: crate::Conversion::ImportTreeImpl::Tree::interned_EMPTY(),
                    });
                } else if key_comp == 1 {
                    outTree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: crate::Conversion::ImportTreeImpl::Tree::interned_EMPTY(),
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
                    if !({
                        let __refeq_sl = &(var_field!((*tree).value, Tree::LEAF).clone());
                        let __refeq_sr = &(value.clone());
                        referenceEq(&*(__refeq_sl.originalPath), &*(__refeq_sr.originalPath))
                            && referenceEq(&*(__refeq_sl.convertedPath), &*(__refeq_sr.convertedPath))
                            && referenceEq(&*(__refeq_sl.importName), &*(__refeq_sr.importName))
                            && ((__refeq_sl.shadowed) == (__refeq_sr.shadowed))
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

    pub(crate) fn addConflictFail(mut newValue: &Value, mut oldValue: &Value, mut key: &Key) -> Result<Value> {
        let mut value: Value;
        return Err("fail");
        Ok(value)
    }

    pub(crate) fn addConflictKeep(mut newValue: &Value, mut oldValue: Value, mut key: &Key) -> Value {
        let mut value: Value = oldValue;
        value
    }

    pub fn addConflictReplace(mut newValue: Value, mut oldValue: Value, mut key: Key) -> Value {
        let mut value: Value = newValue;
        value
    }

    pub(crate) fn addList(
        mut tree: metamodelica::Ref<Tree>,
        mut inValues: &metamodelica::List<(ArcStr, ImportData)>,
        mut conflictFunc: &dyn ::std::ops::Fn(ImportData, ImportData, ArcStr) -> Result<ImportData>,
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
        mut r#fn: &dyn ::std::ops::Fn(Option<ImportData>) -> Result<ImportData>,
    ) -> Result<metamodelica::Ref<Tree>> {
        pub type UpdateFn = std::sync::Arc<dyn ::std::ops::Fn(Option<ImportData>) -> Result<Value> + 'static>;

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
                        right: crate::Conversion::ImportTreeImpl::Tree::interned_EMPTY(),
                    });
                } else if key_comp == 1 {
                    new_tree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: crate::Conversion::ImportTreeImpl::Tree::interned_EMPTY(),
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
        mut inFunc: &'__b dyn ::std::ops::Fn(ArcStr, ImportData, FT) -> Result<FT>,
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
        mut foldFunc: &dyn ::std::ops::Fn(ArcStr, ImportData, FT) -> Result<(FT, bool)>,
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
        mut foldFunc: &dyn ::std::ops::Fn(ArcStr, ImportData, FT1, FT2) -> Result<(FT1, FT2)>,
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
        mut func: &dyn ::std::ops::Fn(ArcStr, ImportData) -> Result<()>,
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
        mut inValues: &metamodelica::List<(ArcStr, ImportData)>,
        mut conflictFunc: &dyn ::std::ops::Fn(ImportData, ImportData, ArcStr) -> Result<ImportData>,
    ) -> Result<metamodelica::Ref<Tree>> {
        let mut tree: metamodelica::Ref<Tree> = crate::Conversion::ImportTreeImpl::Tree::interned_EMPTY();
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

    pub(crate) fn getOpt<'__b>(mut tree: &'__b metamodelica::Ref<Tree>, mut key: Key) -> Option<ImportData> {
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
        mut conflictFunc: &'__b dyn ::std::ops::Fn(ImportData, ImportData, ArcStr) -> Result<ImportData>,
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
        mut lst: metamodelica::List<ImportData>,
    ) -> metamodelica::List<ImportData> {
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
        mut inFunc: &dyn ::std::ops::Fn(ArcStr, ImportData) -> Result<ImportData>,
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
                    || !({
                        let __refeq_sl = &(value.clone());
                        let __refeq_sr = &(new_value.clone());
                        referenceEq(&*(__refeq_sl.originalPath), &*(__refeq_sr.originalPath))
                            && referenceEq(&*(__refeq_sl.convertedPath), &*(__refeq_sr.convertedPath))
                            && referenceEq(&*(__refeq_sl.importName), &*(__refeq_sr.importName))
                            && ((__refeq_sl.shadowed) == (__refeq_sr.shadowed))
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
                if !({
                    let __refeq_sl = &(value.clone());
                    let __refeq_sr = &(new_value.clone());
                    referenceEq(&*(__refeq_sl.originalPath), &*(__refeq_sr.originalPath))
                        && referenceEq(&*(__refeq_sl.convertedPath), &*(__refeq_sr.convertedPath))
                        && referenceEq(&*(__refeq_sl.importName), &*(__refeq_sr.importName))
                        && ((__refeq_sl.shadowed) == (__refeq_sr.shadowed))
                }) {
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
        mut inFunc: &dyn ::std::ops::Fn(ArcStr, ImportData, FT) -> Result<(ImportData, FT)>,
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
                    || !({
                        let __refeq_sl = &(value.clone());
                        let __refeq_sr = &(new_value.clone());
                        referenceEq(&*(__refeq_sl.originalPath), &*(__refeq_sr.originalPath))
                            && referenceEq(&*(__refeq_sl.convertedPath), &*(__refeq_sr.convertedPath))
                            && referenceEq(&*(__refeq_sl.importName), &*(__refeq_sr.importName))
                            && ((__refeq_sl.shadowed) == (__refeq_sr.shadowed))
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
                if !({
                    let __refeq_sl = &(value.clone());
                    let __refeq_sr = &(new_value.clone());
                    referenceEq(&*(__refeq_sl.originalPath), &*(__refeq_sr.originalPath))
                        && referenceEq(&*(__refeq_sl.convertedPath), &*(__refeq_sr.convertedPath))
                        && referenceEq(&*(__refeq_sl.importName), &*(__refeq_sr.importName))
                        && ((__refeq_sl.shadowed) == (__refeq_sr.shadowed))
                }) {
                    assign_variant_field!(outTree => Tree::LEAF; value = new_value);
                }
                outTree
            }
            _ => inTree,
        });
        Ok((outTree, outResult))
    }

    pub(crate) fn new() -> metamodelica::Ref<Tree> {
        let mut outTree: metamodelica::Ref<Tree> = crate::Conversion::ImportTreeImpl::Tree::interned_EMPTY();
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
                node = setTreeLeftRight(outNode, __outNode_left.clone(), crate::Conversion::ImportTreeImpl::Tree::interned_EMPTY())?;
                setTreeLeftRight(child.clone(), node, crate::Conversion::ImportTreeImpl::Tree::interned_EMPTY())?
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
                node = setTreeLeftRight(outNode, crate::Conversion::ImportTreeImpl::Tree::interned_EMPTY(), __outNode_right.clone())?;
                setTreeLeftRight(child.clone(), crate::Conversion::ImportTreeImpl::Tree::interned_EMPTY(), node)?
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
        mut lst: metamodelica::List<(ArcStr, ImportData)>,
    ) -> metamodelica::List<(ArcStr, ImportData)> {
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
        let mut outTree: metamodelica::Ref<Tree> = add(
            tree.clone(),
            key,
            value,
            &fnptr!(addConflictReplace, ImportData, ImportData, ArcStr),
        )?;
        Ok(outTree)
    }
}

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Env {
    pub components: TypeTable,
    pub imports: ImportTree,
}

impl metamodelica::gc::MMTrace for Env {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.components, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.imports, __mmv)?;
        Ok(())
    }
}
impl Default for Env {
    fn default() -> Self {
        Self {
            components: Default::default(),
            imports: Default::default(),
        }
    }
}

pub type ENV = Env;

pub fn convertPackage(
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut scriptFile: ArcStr,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    let mut rules: metamodelica::Ref<ConversionRules::ConversionRules>;
    let mut stmts: metamodelica::List<GlobalScript::Statement>;
    stmts = loadScript(scriptFile)?;
    rules = ConversionRules::newNode();
    rules = parseRules(&stmts, rules)?;
    if Flags::isSet(Flags::DUMP_CONVERSION_RULES.clone())? {
        dumpRules(&rules, &(literal!("")))?;
    }
    cls = convertClass(cls, rules, newEnv(), &(metamodelica::nil()))?;
    Ok(cls)
}

fn loadScript(mut scriptFile: ArcStr) -> Result<metamodelica::List<GlobalScript::Statement>> {
    let mut stmts: metamodelica::List<GlobalScript::Statement>;
    let mut script: ArcStr;
    script = System::readFile(scriptFile.clone())?;
    script = System::stringReplace(script, literal!(")\n"), literal!(");\n"))?;
    let GlobalScript::Statements {
        interactiveStmtLst: __pa0,
        ..
    } = Parser::parsestringexp(script, scriptFile)?;
    stmts = metamodelica::Own::own(__pa0);
    Ok(stmts)
}

fn parseRules(
    mut stmts: &metamodelica::List<GlobalScript::Statement>,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
) -> Result<metamodelica::Ref<ConversionRules::ConversionRules>> {
    let mut rules: metamodelica::Ref<ConversionRules::ConversionRules> = rules;
    for mut stmt in &**stmts {
        rules = parseRule(metamodelica::AsArg::as_arg(&stmt), rules)?;
    }
    Ok(rules)
}

fn parseRule(
    mut stmt: &GlobalScript::Statement,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
) -> Result<metamodelica::Ref<ConversionRules::ConversionRules>> {
    type ParseFn = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
                SourceInfo,
                metamodelica::Ref<ConversionRules::ConversionRules>,
            ) -> Result<metamodelica::Ref<ConversionRules::ConversionRules>>
            + 'static,
    >;

    let mut rules: metamodelica::Ref<ConversionRules::ConversionRules> = rules;
    let mut fn_name: ArcStr;
    let mut args: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    let mut parse_fn: Arc<
        dyn ::std::ops::Fn(
                metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
                SourceInfo,
                metamodelica::Ref<ConversionRules::ConversionRules>,
            ) -> Result<metamodelica::Ref<ConversionRules::ConversionRules>>
            + 'static,
    >;
    let mut fn_type: metamodelica::List<ArgType>;
    let () = (::match_deref::match_deref! { match &(stmt) {
        GlobalScript::Statement::IEXP { exp: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: __esc_fn_name, .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: __esc_args, argNames: Deref @ metamodelica::ListNode::Nil }, .. }, .. } => {
            fn_name = (*__esc_fn_name).clone();
            args = (*__esc_args).clone();
            (parse_fn, fn_type) = (::match_deref::match_deref! { match &(fn_name.clone()) {
        Deref @ "convertClass" => ((std::sync::Arc::new(move |__a0: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a1: SourceInfo, __a2: metamodelica::Ref<ConversionRules::ConversionRules>| parseConvertClass(__a0, &__a1, __a2)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<Absyn::Exp>>, SourceInfo, metamodelica::Ref<ConversionRules::ConversionRules>) -> Result<metamodelica::Ref<ConversionRules::ConversionRules>> + 'static>), CONVERT_CLASS_TYPE.clone()),
        Deref @ "convertClassIf" => ((std::sync::Arc::new(move |__a0: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a1: SourceInfo, __a2: metamodelica::Ref<ConversionRules::ConversionRules>| parseConvertClassIf(&__a0, &__a1, __a2)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<Absyn::Exp>>, SourceInfo, metamodelica::Ref<ConversionRules::ConversionRules>) -> Result<metamodelica::Ref<ConversionRules::ConversionRules>> + 'static>), CONVERT_CLASS_IF_TYPE.clone()),
        Deref @ "convertElement" => ((std::sync::Arc::new(move |__a0: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a1: SourceInfo, __a2: metamodelica::Ref<ConversionRules::ConversionRules>| parseConvertElement(__a0, &__a1, __a2)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<Absyn::Exp>>, SourceInfo, metamodelica::Ref<ConversionRules::ConversionRules>) -> Result<metamodelica::Ref<ConversionRules::ConversionRules>> + 'static>), CONVERT_ELEMENT_TYPE.clone()),
        Deref @ "convertModifiers" => ((std::sync::Arc::new(parseConvertModifiers) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<Absyn::Exp>>, SourceInfo, metamodelica::Ref<ConversionRules::ConversionRules>) -> Result<metamodelica::Ref<ConversionRules::ConversionRules>> + 'static>), CONVERT_MODIFIER_TYPE.clone()),
        Deref @ "convertMessage" => ((std::sync::Arc::new(move |__a0: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a1: SourceInfo, __a2: metamodelica::Ref<ConversionRules::ConversionRules>| parseConvertMessage(__a0, &__a1, __a2)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<Absyn::Exp>>, SourceInfo, metamodelica::Ref<ConversionRules::ConversionRules>) -> Result<metamodelica::Ref<ConversionRules::ConversionRules>> + 'static>), CONVERT_MESSAGE_TYPE.clone()),
        _ => {
            printConversionRuleError(stmt)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            args = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
        for mut a in (args.clone()).into_iter().cloned() {
            let __x = expandArg(a.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            for mut a in &*vectorizeArgs(args.clone(), fn_type, stmt)? {
                rules = parse_fn(a.clone(), var_field!(stmt.info, GlobalScript::Statement::IEXP).clone(), rules)?;
            }
            ()
        },
        _ => {
            printConversionRuleError(stmt)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(rules)
}

fn expandArg(mut exp: metamodelica::Ref<Absyn::Exp>) -> metamodelica::Ref<Absyn::Exp> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "fill", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::INTEGER { value: 0 }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, .. } => metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: metamodelica::nil() }),
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

fn vectorizeArgs(
    mut args: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut fnType: metamodelica::List<ArgType>,
    mut stmt: &GlobalScript::Statement,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>> {
    let mut vargs: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>;
    let mut vdim: i32 = -1;
    let mut dim: i32;
    let mut fn_ty: metamodelica::List<ArgType> = fnType.clone();
    let mut arg_ty: ArgType;
    let mut is_varg: metamodelica::List<bool> = metamodelica::nil();
    let mut expl: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    if ((args).len() as i32) > ((fnType).len() as i32) {
        printConversionRuleError(stmt)?;
    }
    for mut arg in &*args {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(fn_ty) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        arg_ty = metamodelica::Own::own(__pa0);
        fn_ty = metamodelica::Own::own(__pa1);
        (vdim, is_varg) = (::match_deref::match_deref! { match &((arg.clone(), arg_ty)) {
            (Deref @ Absyn::Exp::ARRAY { .. }, ArgType::SCALAR) => {
                dim = ((var_field!((**arg).arrayExp, Absyn::Exp::ARRAY)).len() as i32);
                if vdim >= 0 && dim != vdim {
                    printConversionRuleError(stmt)?;
                }
                (dim, metamodelica::cons(true, is_varg))
            },
            (Deref @ Absyn::Exp::ARRAY { .. }, ArgType::ARRAY { .. }) => (vdim, metamodelica::cons(false, is_varg)),
            (_, ArgType::ARRAY { .. }) => {
                printConversionRuleError(stmt)?;
                return Err("fail")
            },
            _ => (vdim, metamodelica::cons(false, is_varg)),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    if vdim == 0 {
        vargs = metamodelica::nil();
    } else if vdim == -1 {
        vargs = list![args];
    } else {
        vargs = metamodelica::nil();
        for mut arg in &*args.reverse() {
            if (is_varg).head().cloned()? {
                let __pa2 = ::match_deref::match_deref! { match &(arg.clone()) {
                    Deref @ Absyn::Exp::ARRAY { arrayExp: __pa2 } => __pa2.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                expl = metamodelica::Own::own(__pa2);
                vargs = metamodelica::cons(expl, vargs);
            } else {
                vargs = metamodelica::cons(List::fill(arg.clone(), vdim), vargs);
            }
            is_varg = (is_varg).rest()?;
        }
        vargs = List::transposeList(vargs)?;
    }
    Ok(vargs)
}

fn statementInfo(mut stmt: &GlobalScript::Statement) -> SourceInfo {
    let mut info: SourceInfo;
    info = (match stmt.clone() {
        GlobalScript::Statement::IEXP { .. } => var_field!(stmt.info, GlobalScript::Statement::IEXP).clone(),
        _ => Absyn::dummyInfo.clone(),
    });
    info
}

fn printConversionRuleError(mut stmt: &GlobalScript::Statement) -> Result<()> {
    Error::addSourceMessage(
        &(Error::INVALID_CONVERSION_RULE.clone()),
        list![GlobalScriptDump::printIstmtStr(stmt)?],
        &(statementInfo(stmt)),
    )?;
    return Err("fail");
    Ok(())
}

fn parseConvertClass(
    mut args: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut info: &SourceInfo,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
) -> Result<metamodelica::Ref<ConversionRules::ConversionRules>> {
    let mut rules: metamodelica::Ref<ConversionRules::ConversionRules> = rules;
    let () = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::STRING { value: old_cls }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::STRING { value: new_cls }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            parseConvertClassStr(old_cls.clone(), new_cls.clone(), rules.clone())?;
            ()
        },
        _ => {
            Error::addSourceMessage(&(Error::INVALID_CONVERSION_RULE.clone()), list![List::toStringCustom(args, &Dump::printExpStr, literal!("convertClass"), literal!("("), literal!(", "), literal!(")"), true, 0)?], info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(rules)
}

fn parseConvertClassStr(
    mut oldName: ArcStr,
    mut newName: ArcStr,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
) -> Result<metamodelica::Ref<ConversionRules::ConversionRules>> {
    let mut rules: metamodelica::Ref<ConversionRules::ConversionRules> = rules;
    let mut old_path: metamodelica::List<ArcStr>;
    let mut rule: ConversionRule;
    old_path = parsePathList(oldName)?;
    rule = ConversionRule::CLASS {
        oldPath: metamodelica::arrayFromVec(old_path.clone().into_iter().cloned().collect()),
        newPath: parsePath(newName)?,
    };
    rules = addRule(&old_path, rule, rules)?;
    Ok(rules)
}

fn parseConvertClassIf(
    mut args: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut info: &SourceInfo,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
) -> Result<metamodelica::Ref<ConversionRules::ConversionRules>> {
    let mut rules: metamodelica::Ref<ConversionRules::ConversionRules> = rules;
    Error::terminate(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Conversion.parseConvertClassIf"));
            __mm_s.push_str(&*literal!(": not implemented"));
            ArcStr::from(__mm_s)
        },
        info,
    )?;
    Ok(rules)
}

fn parseConvertElement(
    mut args: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut info: &SourceInfo,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
) -> Result<metamodelica::Ref<ConversionRules::ConversionRules>> {
    let mut rules: metamodelica::Ref<ConversionRules::ConversionRules> = rules;
    let () = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::STRING { value: cls_name }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::STRING { value: old_name }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::STRING { value: new_name }, tail: Deref @ metamodelica::ListNode::Nil } } } => {
            let mut old_path: metamodelica::List<ArcStr>;
            let mut rule: ConversionRule;
            old_path = parsePathList(cls_name.clone())?;
            rule = ConversionRule::ELEMENT { oldPath: metamodelica::arrayFromVec(old_path.clone().into_iter().cloned().collect()), oldName: old_name.clone(), newName: new_name.clone() };
            rules = addRule(&old_path, rule, rules)?;
            ()
        },
        _ => {
            Error::addSourceMessage(&(Error::INVALID_CONVERSION_RULE.clone()), list![List::toStringCustom(args, &Dump::printExpStr, literal!("convertElement"), literal!("("), literal!(", "), literal!(")"), true, 0)?], info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(rules)
}

fn parseConvertModifiers(
    mut args: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut info: SourceInfo,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
) -> Result<metamodelica::Ref<ConversionRules::ConversionRules>> {
    let mut rules: metamodelica::Ref<ConversionRules::ConversionRules> = rules;
    rules = 'mc: {
        let __mc_input = &*args;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::STRING { value: cls_name }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::ARRAY { arrayExp: old_mods }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::ARRAY { arrayExp: new_mods }, tail: Deref @ metamodelica::ListNode::Nil } } } => {
                    Ok(parseConvertModifiers2(cls_name.clone(), old_mods.clone(), new_mods.clone(), false, info.clone(), rules.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::STRING { value: cls_name }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::ARRAY { arrayExp: old_mods }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::ARRAY { arrayExp: new_mods }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::BOOL { value: simplify }, tail: Deref @ metamodelica::ListNode::Nil } } } } => {
                    Ok(parseConvertModifiers2(cls_name.clone(), old_mods.clone(), new_mods.clone(), simplify.clone(), info.clone(), rules.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(&(Error::INVALID_CONVERSION_RULE.clone()), list![List::toStringCustom(args.clone(), &Dump::printExpStr, literal!("convertModifiers"), literal!("("), literal!(", "), literal!(")"), true, 0)?], &info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(rules)
}

fn parseConvertModifiers2(
    mut className: ArcStr,
    mut oldMods: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut newMods: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut simplify: bool,
    mut info: SourceInfo,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
) -> Result<metamodelica::Ref<ConversionRules::ConversionRules>> {
    let mut rules: metamodelica::Ref<ConversionRules::ConversionRules> = rules;
    let mut cls_path: metamodelica::List<ArcStr>;
    let mut old_mods: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut new_mods: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    cls_path = parsePathList(className)?;
    old_mods = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
        for mut m in (oldMods).into_iter().cloned() {
            let __x = parseModifier(&(m.clone()), &info)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    new_mods = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
        for mut m in (newMods).into_iter().cloned() {
            let __x = parseModifier(&(m.clone()), &info)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    rules = addRule(
        &cls_path,
        ConversionRule::MODIFIERS {
            oldMods: old_mods,
            newMods: new_mods,
            info: info,
        },
        rules,
    )?;
    Ok(rules)
}

fn parseModifier(
    mut r#mod: &metamodelica::Ref<Absyn::Exp>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
    let mut outMod: metamodelica::Ref<Absyn::ElementArg>;
    let mut r#str: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &((*r#mod)) {
        Deref @ Absyn::Exp::STRING { value: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#str = metamodelica::Own::own(__pa0);
    outMod = Parser::stringMod(quotePlaceholders(r#str, info)?, literal!("<internal>"))?;
    Ok(outMod)
}

fn quotePlaceholders(mut r#str: ArcStr, mut info: &SourceInfo) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    let mut strl: metamodelica::List<ArcStr>;
    let mut res: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut in_ident: bool = false;
    strl = System::strtokIncludingDelimiters(r#str.clone(), literal!("%"));
    if ((strl).len() as i32) <= 1 {
        return Ok(r#str);
    }
    for mut s in &*strl {
        let mut s = s.clone();
        if metamodelica::stringEq(&s, &(literal!("%"))) {
            s = if (in_ident) { literal!("%'") } else { literal!("'%") };
            in_ident = !(in_ident);
        }
        res = metamodelica::cons(s, res);
    }
    if in_ident {
        Error::addSourceMessage(&(Error::CONVERSION_MISMATCHED_PLACEHOLDER.clone()), list![r#str], info)?;
        return Err("fail");
    }
    r#str = stringAppendList(metamodelica::Dangerous::listReverseInPlace(res));
    Ok(r#str)
}

fn parseConvertMessage(
    mut args: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut info: &SourceInfo,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
) -> Result<metamodelica::Ref<ConversionRules::ConversionRules>> {
    let mut rules: metamodelica::Ref<ConversionRules::ConversionRules> = rules;
    let () = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::STRING { value: cls_name }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::STRING { value: msg }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            let mut rule: ConversionRule;
            rule = ConversionRule::MESSAGE { message: msg.clone() };
            rules = addRule(&(parsePathList(cls_name.clone())?), rule, rules)?;
            ()
        },
        _ => {
            Error::addSourceMessage(&(Error::INVALID_CONVERSION_RULE.clone()), list![List::toStringCustom(args, &Dump::printExpStr, literal!("convertMessage"), literal!("("), literal!(", "), literal!(")"), true, 0)?], info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(rules)
}

fn parsePath(mut r#str: ArcStr) -> Result<metamodelica::Ref<Path>> {
    let mut path: metamodelica::Ref<Path> = AbsynUtil::stringPath(r#str.clone())?;
    Ok(path)
}

fn parsePathList(mut r#str: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut path: metamodelica::List<ArcStr> = Util::stringSplitAtChar(r#str.clone(), literal!("."))?;
    Ok(path)
}

fn addRule(
    mut path: &metamodelica::List<ArcStr>,
    mut rule: ConversionRule,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
) -> Result<metamodelica::Ref<ConversionRules::ConversionRules>> {
    let mut rules: metamodelica::Ref<ConversionRules::ConversionRules> = rules;
    updateNode(Some(rules.clone()), path, rule)?;
    Ok(rules)
}

fn updateNode(
    mut onode: Option<metamodelica::Ref<ConversionRules::ConversionRules>>,
    mut path: &metamodelica::List<ArcStr>,
    mut rule: ConversionRule,
) -> Result<metamodelica::Ref<ConversionRules::ConversionRules>> {
    let mut node: metamodelica::Ref<ConversionRules::ConversionRules>;
    if (onode).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(onode) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        node = metamodelica::Own::own(__pa0);
    } else {
        node = ConversionRules::newNode();
    }
    if (path).is_empty() {
        assign_field!(node.rules = metamodelica::cons(rule, node.rules.clone()));
    } else {
        UnorderedMap::addUpdate(
            (path).head().cloned()?,
            &({
                let __pe_b1 = (path).rest()?;
                let __pe_b2 = rule;
                move |__pe_a0| updateNode(__pe_a0, &__pe_b1, __pe_b2.clone())
            }),
            node.nodes.clone(),
        )?;
    }
    Ok(node)
}

fn lookupRuleNode(
    mut path: &metamodelica::Ref<Path>,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
) -> Result<Option<metamodelica::Ref<ConversionRules::ConversionRules>>> {
    let mut outNode: Option<metamodelica::Ref<ConversionRules::ConversionRules>> = None;
    let mut node: metamodelica::Ref<ConversionRules::ConversionRules> = rules;
    for mut name in &*AbsynUtil::pathToStringList(path) {
        outNode = UnorderedMap::get(name.clone(), node.nodes.clone())?;
        if (outNode).is_none() {
            return Ok(outNode);
        }
        let __pa0 = ::match_deref::match_deref! { match &(outNode.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        node = metamodelica::Own::own(__pa0);
    }
    Ok(outNode)
}

fn lookupRules(
    mut path: &metamodelica::Ref<Path>,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
) -> Result<metamodelica::List<metamodelica::List<ConversionRule>>> {
    let mut outRules: metamodelica::List<metamodelica::List<ConversionRule>> = metamodelica::nil();
    let mut onode: Option<metamodelica::Ref<ConversionRules::ConversionRules>>;
    let mut node: metamodelica::Ref<ConversionRules::ConversionRules> = rules;
    for mut name in &*AbsynUtil::pathToStringList(path) {
        onode = UnorderedMap::get(name.clone(), node.nodes.clone())?;
        if (onode).is_none() {
            outRules = metamodelica::cons(metamodelica::nil(), outRules);
            return Ok(outRules);
        }
        let __pa0 = ::match_deref::match_deref! { match &(onode) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        node = metamodelica::Own::own(__pa0);
        if !((node.rules).is_empty()) {
            outRules = metamodelica::cons(node.rules.clone(), outRules);
        }
    }
    Ok(outRules)
}

fn lookupTypeRules(
    mut typePath: &metamodelica::Ref<Path>,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: &Env,
) -> Result<(Option<ConversionRule>, RuleTable, metamodelica::List<ConversionRule>)> {
    let mut typeRule: Option<ConversionRule> = None;
    let mut localRules: RuleTable = newRuleTable();
    let mut modifierRules: metamodelica::List<ConversionRule> = metamodelica::nil();
    let mut found_rules: metamodelica::List<metamodelica::List<ConversionRule>>;
    found_rules = lookupRules(typePath, rules)?;
    if (found_rules).is_empty() {
        return Ok((typeRule, localRules, modifierRules));
    }
    modifierRules = sortLocalRules(&((found_rules).head().cloned()?), localRules.clone())?;
    for mut rl in &*found_rules {
        for mut r in &*rl.clone() {
            let () = (match r.clone() {
                ConversionRule::CLASS { .. } => {
                    if (typeRule).is_none() {
                        typeRule = Some(r.clone());
                    }
                    ()
                }
                _ => (),
            });
        }
    }
    Ok((typeRule, localRules, modifierRules))
}

fn newRuleTable() -> RuleTable {
    let mut table: RuleTable;
    table = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    table
}

fn newTypeTable() -> TypeTable {
    let mut table: TypeTable;
    table = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    table
}

fn newEnv() -> Env {
    let mut env: Env = Env {
        components: newTypeTable(),
        imports: ImportTreeImpl::new(),
    };
    env
}

fn sortLocalRules(
    mut rules: &metamodelica::List<ConversionRule>,
    mut localRules: RuleTable,
) -> Result<metamodelica::List<ConversionRule>> {
    let mut modifierRules: metamodelica::List<ConversionRule> = metamodelica::nil();
    for mut rule in &**rules {
        let () = (match rule.clone() {
            ConversionRule::ELEMENT { .. } => {
                UnorderedMap::addUpdate(
                    var_field!(rule.oldName, ConversionRule::ELEMENT).clone(),
                    &({
                        let __pe_b1 = rule.clone();
                        move |__pe_a0| mergeRuleList(__pe_a0, __pe_b1.clone())
                    }),
                    localRules.clone(),
                )?;
                ()
            }
            ConversionRule::MODIFIERS { .. } => {
                modifierRules = metamodelica::cons(rule.clone(), modifierRules);
                ()
            }
            _ => (),
        });
    }
    Ok(modifierRules)
}

fn mergeRuleList(
    mut oldRules: Option<metamodelica::List<ConversionRule>>,
    mut newRule: ConversionRule,
) -> Result<metamodelica::List<ConversionRule>> {
    let mut outRules: metamodelica::List<ConversionRule>;
    if (oldRules).is_none() {
        outRules = list![newRule];
    } else {
        let __pa0 = ::match_deref::match_deref! { match &(oldRules) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        outRules = metamodelica::Own::own(__pa0);
        outRules = metamodelica::cons(newRule, outRules);
    }
    Ok(outRules)
}

fn lookupClassExtendsRules(
    mut name: ArcStr,
    mut extendsRules: &metamodelica::List<metamodelica::Ref<ConversionRules::ConversionRules>>,
) -> Result<(RuleTable, metamodelica::List<ConversionRule>)> {
    let mut localRules: RuleTable = newRuleTable();
    let mut modificationRules: metamodelica::List<ConversionRule> = metamodelica::nil();
    let mut onode: Option<metamodelica::Ref<ConversionRules::ConversionRules>>;
    let mut node: metamodelica::Ref<ConversionRules::ConversionRules>;
    for mut ext in &**extendsRules {
        onode = UnorderedMap::get(name.clone(), ext.nodes.clone())?;
        if (onode).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(onode) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            node = metamodelica::Own::own(__pa0);
            modificationRules = sortLocalRules(&node.rules, localRules.clone())?;
            return Ok((localRules, modificationRules));
        }
    }
    Ok((localRules, modificationRules))
}

fn dumpRules(mut rules: &metamodelica::Ref<ConversionRules::ConversionRules>, mut indent: &ArcStr) -> Result<()> {
    let mut keys: metamodelica::Array<ArcStr>;
    let mut values: metamodelica::Array<metamodelica::Ref<ConversionRules::ConversionRules>>;
    let mut rule: ConversionRule;
    let mut rest_rules: metamodelica::List<ConversionRule> = rules.rules.clone();
    keys = UnorderedMap::keyArray(rules.nodes.clone());
    values = UnorderedMap::valueArray(rules.nodes.clone());
    while !((rest_rules).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_rules) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        rule = metamodelica::Own::own(__pa0);
        rest_rules = metamodelica::Own::own(__pa1);
        if (rest_rules).is_empty() && keys.clone().borrow().is_empty() {
            dumpRule(&rule, {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*indent);
                __mm_s.push_str(&*literal!("└─"));
                ArcStr::from(__mm_s)
            })?;
        } else {
            dumpRule(&rule, {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*indent);
                __mm_s.push_str(&*literal!("├─"));
                ArcStr::from(__mm_s)
            })?;
        }
    }
    for mut i in 1..=metamodelica::arrayLength(keys.clone()) {
        if i == metamodelica::arrayLength(keys.clone()) {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*indent);
                __mm_s.push_str(&*literal!("└─"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print(
                ({
                    let __elt = (*metamodelica::index_checked(&keys.borrow(), i)?).clone();
                    __elt
                }),
            );
            metamodelica::print(literal!("\n"));
            dumpRules(
                &({
                    let __elt = (*metamodelica::index_checked(&values.borrow(), i)?).clone();
                    __elt
                }),
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*indent);
                    __mm_s.push_str(&*literal!("  "));
                    ArcStr::from(__mm_s)
                }),
            )?;
        } else {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*indent);
                __mm_s.push_str(&*literal!("├─"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print(
                ({
                    let __elt = (*metamodelica::index_checked(&keys.borrow(), i)?).clone();
                    __elt
                }),
            );
            metamodelica::print(literal!("\n"));
            dumpRules(
                &({
                    let __elt = (*metamodelica::index_checked(&values.borrow(), i)?).clone();
                    __elt
                }),
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*indent);
                    __mm_s.push_str(&*literal!("│ "));
                    ArcStr::from(__mm_s)
                }),
            )?;
        }
    }
    Ok(())
}

fn dumpRule(mut rule: &ConversionRule, mut indent: ArcStr) -> Result<()> {
    metamodelica::print(indent);
    let () = (match rule.clone() {
        ConversionRule::CLASS { .. } => {
            metamodelica::print(literal!("convertClass: "));
            metamodelica::print(AbsynUtil::pathString(
                var_field!(rule.newPath, ConversionRule::CLASS).clone(),
                literal!("."),
                true,
                false,
            )?);
            ()
        }
        ConversionRule::CLASS_IF => {
            metamodelica::print(literal!("convertClassIf: "));
            ()
        }
        ConversionRule::ELEMENT { .. } => {
            metamodelica::print(literal!("convertElement: "));
            metamodelica::print(var_field!(rule.oldName, ConversionRule::ELEMENT).clone());
            metamodelica::print(literal!(" => "));
            metamodelica::print(var_field!(rule.newName, ConversionRule::ELEMENT).clone());
            ()
        }
        ConversionRule::MODIFIERS { .. } => {
            metamodelica::print(literal!("convertModifiers: "));
            metamodelica::print(List::toString(
                var_field!(rule.oldMods, ConversionRule::MODIFIERS).clone(),
                &Dump::unparseElementArgStr,
                List::Style::FLAT_CURLY.clone(),
            )?);
            metamodelica::print(literal!(" => "));
            metamodelica::print(List::toString(
                var_field!(rule.newMods, ConversionRule::MODIFIERS).clone(),
                &Dump::unparseElementArgStr,
                List::Style::FLAT_CURLY.clone(),
            )?);
            ()
        }
        ConversionRule::MESSAGE { .. } => {
            metamodelica::print(literal!("convertMessage: \""));
            metamodelica::print(var_field!(rule.message, ConversionRule::MESSAGE).clone());
            metamodelica::print(literal!("\""));
            ()
        }
    });
    metamodelica::print(literal!("\n"));
    Ok(())
}

fn convertProgram(
    mut program: Absyn::Program,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
) -> Result<Absyn::Program> {
    let mut program: Absyn::Program = program;
    program.classes = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Class>> = metamodelica::nil();
        for mut c in (program.classes.clone()).into_iter().cloned() {
            let __x = convertClass(c.clone(), rules.clone(), env.clone(), &(metamodelica::nil()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(program)
}

fn convertClass(
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
    mut extendsRules: &metamodelica::List<metamodelica::Ref<ConversionRules::ConversionRules>>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    assign_field!(cls.body = convertClassDef(cls.body.clone(), rules, env, extendsRules, &(cls.info.clone()))?);
    Ok(cls)
}

fn convertClassDef(
    mut cdef: metamodelica::Ref<Absyn::ClassDef>,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
    mut extendsRules: &metamodelica::List<metamodelica::Ref<ConversionRules::ConversionRules>>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Absyn::ClassDef>> {
    let mut cdef: metamodelica::Ref<Absyn::ClassDef> = cdef;
    let () = (match &*cdef {
        Absyn::ClassDef::PARTS {
            classParts: __cdef_classParts,
            ..
        } => {
            assign_variant_field!(cdef => Absyn::ClassDef::PARTS; classParts = convertClassParts(__cdef_classParts.clone(), newRuleTable(), rules, env, info)?);
            ()
        }
        Absyn::ClassDef::DERIVED {
            typeSpec: __cdef_typeSpec,
            ..
        } => {
            let mut local_rules: RuleTable;
            let mut mod_rules: metamodelica::List<ConversionRule>;
            let mut ty: metamodelica::Ref<Absyn::TypeSpec>;
            (ty, local_rules, mod_rules) = convertTypeSpec(__cdef_typeSpec.clone(), rules.clone(), &env, info)?;
            assign_variant_field!(cdef => Absyn::ClassDef::DERIVED;
                typeSpec = ty,
                arguments = convertModification2(&mod_rules, var_field!((*cdef).arguments, Absyn::ClassDef::DERIVED).clone())?
            );
            assign_variant_field!(cdef => Absyn::ClassDef::DERIVED; arguments = convertElementArgs(var_field!((*cdef).arguments, Absyn::ClassDef::DERIVED).clone(), local_rules, rules, env)?);
            ()
        }
        Absyn::ClassDef::CLASS_EXTENDS {
            baseClassName: __cdef_baseClassName,
            modifications: __cdef_modifications,
            ..
        } => {
            let mut local_rules: RuleTable;
            let mut mod_rules: metamodelica::List<ConversionRule>;
            (local_rules, mod_rules) = lookupClassExtendsRules(__cdef_baseClassName.clone(), extendsRules)?;
            assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; modifications = convertModification2(&mod_rules, __cdef_modifications.clone())?);
            assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS;
                modifications = convertElementArgs(var_field!((*cdef).modifications, Absyn::ClassDef::CLASS_EXTENDS).clone(), local_rules.clone(), rules.clone(), env.clone())?,
                parts = convertClassParts(var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(), local_rules, rules, env, info)?
            );
            ()
        }
        _ => (),
    });
    Ok(cdef)
}

fn convertClassParts(
    mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut localRules: RuleTable,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
    mut info: &SourceInfo,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = parts;
    let mut extends_rules: metamodelica::List<metamodelica::Ref<ConversionRules::ConversionRules>>;
    let mut cls_env: Env;
    cls_env = addImportNamesToEnv(&(getImportsInParts(&parts)), rules.clone(), env.clone())?;
    addComponentTypesToEnv(&parts, env.components.clone())?;
    cls_env.imports = shadowImportsInParts(&parts, cls_env.imports.clone())?;
    extends_rules = getExtendsRules(&parts, rules.clone(), &cls_env)?;
    parts = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
        for mut p in (parts).into_iter().cloned() {
            let __x = convertClassPart(
                p.clone(),
                localRules.clone(),
                rules.clone(),
                cls_env.clone(),
                &extends_rules,
                info,
            )?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(parts)
}

fn convertClassPart(
    mut part: metamodelica::Ref<Absyn::ClassPart>,
    mut localRules: RuleTable,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
    mut extendsRules: &metamodelica::List<metamodelica::Ref<ConversionRules::ConversionRules>>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Absyn::ClassPart>> {
    let mut part: metamodelica::Ref<Absyn::ClassPart> = part;
    let () = (match &*part {
        Absyn::ClassPart::PUBLIC {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::PUBLIC; contents = convertElementItems(__part_contents.clone(), rules, env, extendsRules)?);
            ()
        }
        Absyn::ClassPart::PROTECTED {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::PROTECTED; contents = convertElementItems(__part_contents.clone(), rules, env, extendsRules)?);
            ()
        }
        Absyn::ClassPart::EQUATIONS {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::EQUATIONS; contents = convertEquationItems(__part_contents.clone(), localRules, rules, env)?);
            ()
        }
        Absyn::ClassPart::INITIALEQUATIONS {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::INITIALEQUATIONS; contents = convertEquationItems(__part_contents.clone(), localRules, rules, env)?);
            ()
        }
        Absyn::ClassPart::ALGORITHMS {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::ALGORITHMS; contents = convertAlgorithmItems(__part_contents.clone(), localRules, rules, env)?);
            ()
        }
        Absyn::ClassPart::INITIALALGORITHMS {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::INITIALALGORITHMS; contents = convertAlgorithmItems(__part_contents.clone(), localRules, rules, env)?);
            ()
        }
        Absyn::ClassPart::EXTERNAL {
            externalDecl: __part_externalDecl,
            ..
        } => {
            assign_variant_field!(part => Absyn::ClassPart::EXTERNAL; externalDecl = convertExternalDecl(__part_externalDecl.clone(), localRules, &rules, &env, info)?);
            ()
        }
        _ => (),
    });
    Ok(part)
}

fn convertElementArgs(
    mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut localRules: RuleTable,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>> {
    let mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = args;
    args = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
        for mut a in (args).into_iter().cloned() {
            let __x = convertElementArg(a.clone(), localRules.clone(), rules.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(args)
}

fn convertElementArg(
    mut arg: metamodelica::Ref<Absyn::ElementArg>,
    mut localRules: RuleTable,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
    let mut arg: metamodelica::Ref<Absyn::ElementArg> = arg;
    let () = (match &*arg {
        Absyn::ElementArg::MODIFICATION { path: __arg_path, .. } => {
            let mut mod_rules: metamodelica::List<ConversionRule>;
            mod_rules = UnorderedMap::getOrDefault(
                AbsynUtil::pathString(__arg_path.clone(), literal!("."), true, false)?,
                localRules.clone(),
                metamodelica::nil(),
            )?;
            for mut rule in &*mod_rules {
                let () = (match rule.clone() {
                    ConversionRule::ELEMENT { .. } => {
                        assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; path = metamodelica::Ref::new(Path::IDENT { name: var_field!(rule.newName, ConversionRule::ELEMENT).clone() }));
                        ()
                    }
                    _ => (),
                });
            }
            assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; modification = convertModificationExps(var_field!((*arg).modification, Absyn::ElementArg::MODIFICATION).clone(), localRules, rules, env, var_field!((*arg).info, Absyn::ElementArg::MODIFICATION).clone())?);
            ()
        }
        Absyn::ElementArg::REDECLARATION {
            elementSpec: __arg_elementSpec,
            info: __arg_info,
            ..
        } => {
            assign_variant_field!(arg => Absyn::ElementArg::REDECLARATION;
                elementSpec = convertElementSpec(__arg_elementSpec.clone(), rules.clone(), env.clone(), &(metamodelica::nil()), __arg_info.clone())?,
                constrainClass = convertOption(var_field!((*arg).constrainClass, Absyn::ElementArg::REDECLARATION).clone(), &convertConstrainClass, rules, env, var_field!((*arg).info, Absyn::ElementArg::REDECLARATION).clone())?
            );
            ()
        }
        Absyn::ElementArg::ELEMENTARGCOMMENT { .. } => (),
        _ => return Err("match: no arm matched"),
    });
    Ok(arg)
}

fn convertModificationExps(
    mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>,
    mut localRules: RuleTable,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
    mut info: SourceInfo,
) -> Result<Option<metamodelica::Ref<Absyn::Modification>>> {
    let mut r#mod: Option<metamodelica::Ref<Absyn::Modification>> = r#mod;
    r#mod = convertOption(
        r#mod,
        &({
            let __pe_b1 = localRules;
            move |__pe_a0, __pe_a2, __pe_a3, __pe_a4| {
                convertModificationExps2(__pe_a0, __pe_b1.clone(), __pe_a2, __pe_a3, &__pe_a4)
            }
        }),
        rules,
        env,
        info,
    )?;
    Ok(r#mod)
}

fn convertModificationExps2(
    mut r#mod: metamodelica::Ref<Absyn::Modification>,
    mut localRules: RuleTable,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Absyn::Modification>> {
    let mut r#mod: metamodelica::Ref<Absyn::Modification> = r#mod;
    assign_field!(
        r#mod.elementArgLst = convertElementArgs(
            r#mod.elementArgLst.clone(),
            localRules.clone(),
            rules.clone(),
            env.clone()
        )?,
        r#mod.eqMod = convertEqMod(r#mod.eqMod.clone(), localRules, &rules, &env)?
    );
    Ok(r#mod)
}

fn convertEqMod(
    mut r#mod: metamodelica::Ref<Absyn::EqMod>,
    mut localRules: RuleTable,
    mut rules: &metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: &Env,
) -> Result<metamodelica::Ref<Absyn::EqMod>> {
    let mut r#mod: metamodelica::Ref<Absyn::EqMod> = r#mod;
    let () = (match &*r#mod {
        Absyn::EqMod::EQMOD {
            exp: __mod_exp,
            info: __mod_info,
        } => {
            assign_variant_field!(r#mod => Absyn::EqMod::EQMOD; exp = convertExp(__mod_exp.clone(), localRules, rules, env, metamodelica::AsArg::as_arg(&__mod_info))?);
            ()
        }
        _ => (),
    });
    Ok(r#mod)
}

fn convertModification(
    mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>,
    mut modifierRules: &metamodelica::List<ConversionRule>,
) -> Result<Option<metamodelica::Ref<Absyn::Modification>>> {
    let mut r#mod: Option<metamodelica::Ref<Absyn::Modification>> = r#mod;
    let mut elem_args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut eq_mod: metamodelica::Ref<Absyn::EqMod>;
    if (r#mod).is_some() {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(r#mod) {
            Some(Deref @ Absyn::Modification { elementArgLst: __pa0, eqMod: __pa1 }) => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        elem_args = metamodelica::Own::own(__pa0);
        eq_mod = metamodelica::Own::own(__pa1);
    } else {
        elem_args = metamodelica::nil();
        eq_mod = openmodelica_ast::Absyn::EqMod::interned_NOMOD();
    }
    elem_args = convertModification2(modifierRules, elem_args)?;
    r#mod = (::match_deref::match_deref! { match &((elem_args.clone(), eq_mod.clone())) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ Absyn::EqMod::NOMOD { .. }) => None,
        _ => Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: elem_args, eqMod: eq_mod })),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(r#mod)
}

fn convertModification2(
    mut modifierRules: &metamodelica::List<ConversionRule>,
    mut elemArgs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>> {
    let mut elemArgs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = elemArgs;
    for mut rule in &**modifierRules {
        elemArgs = convertModifier(rule.clone(), elemArgs)?;
    }
    Ok(elemArgs)
}

fn convertModifier(
    mut rule: ConversionRule,
    mut elemArgs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>> {
    let mut elemArgs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = elemArgs;
    let mut old_mods: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut new_mods: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut matching_mods: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut rest_mods: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut placeholders: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Option<metamodelica::Ref<Absyn::Exp>>>>;
    let mut info: SourceInfo;
    let ConversionRule::MODIFIERS {
        oldMods: __pa0,
        newMods: __pa1,
        info: __pa2,
    } = (rule)
    else {
        return Err("pattern mismatch");
    };
    old_mods = metamodelica::Own::own(__pa0);
    new_mods = metamodelica::Own::own(__pa1);
    info = metamodelica::Own::own(__pa2);
    if (old_mods).is_empty() {
        elemArgs = mergeModifiers(elemArgs, new_mods)?;
    } else {
        (matching_mods, rest_mods) = List::splitOnTrue(
            &elemArgs,
            &({
                let __pe_b1 = old_mods.clone();
                move |__pe_a0| isModifierInList(&__pe_a0, &__pe_b1)
            }),
        )?;
        if !((matching_mods).is_empty()) {
            placeholders = makePlaceholderTable(&(listAppend(old_mods, matching_mods)))?;
            new_mods = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
                for mut m in (new_mods).into_iter().cloned() {
                    let __x = replacePlaceholders(m.clone(), placeholders.clone(), &info)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            elemArgs = mergeModifiers(rest_mods, new_mods)?;
        }
    }
    Ok(elemArgs)
}

fn isModifierInList(
    mut r#mod: &metamodelica::Ref<Absyn::ElementArg>,
    mut mods: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> Result<bool> {
    let mut res: bool = List::any(
        mods,
        &({
            let __pe_b1 = r#mod.clone();
            move |__pe_a0| Ok(isEqualNameMod(&__pe_a0, &__pe_b1))
        }),
    )?;
    Ok(res)
}

fn isEqualNameMod(
    mut mod1: &metamodelica::Ref<Absyn::ElementArg>,
    mut mod2: &metamodelica::Ref<Absyn::ElementArg>,
) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match (mod1, mod2) {
        (Deref @ Absyn::ElementArg::MODIFICATION { .. }, Deref @ Absyn::ElementArg::MODIFICATION { .. }) => AbsynUtil::pathEqual(var_field!((**mod1).path, Absyn::ElementArg::MODIFICATION), var_field!((**mod2).path, Absyn::ElementArg::MODIFICATION)),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

fn makePlaceholderTable(
    mut args: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> Result<metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Option<metamodelica::Ref<Absyn::Exp>>>>> {
    pub(crate) type OptExp = Option<metamodelica::Ref<Absyn::Exp>>;

    let mut placeholders: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Option<metamodelica::Ref<Absyn::Exp>>>>;
    placeholders = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    for mut arg in &**args {
        UnorderedMap::add(
            AbsynUtil::pathString(
                AbsynUtil::elementArgName(metamodelica::AsArg::as_arg(&arg))?,
                literal!("."),
                true,
                false,
            )?,
            getElementArgBinding(metamodelica::AsArg::as_arg(&arg)),
            placeholders.clone(),
        )?;
    }
    Ok(placeholders)
}

fn getElementArgBinding(mut arg: &metamodelica::Ref<Absyn::ElementArg>) -> Option<metamodelica::Ref<Absyn::Exp>> {
    let mut exp: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut e: metamodelica::Ref<Absyn::Exp>;
    exp = (::match_deref::match_deref! { match arg {
        Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: __esc_e, .. }, .. }), .. } => {
            e = (*__esc_e).clone();
            Some(e.clone())
        },
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

fn replacePlaceholders(
    mut arg: metamodelica::Ref<Absyn::ElementArg>,
    mut placeholders: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Option<metamodelica::Ref<Absyn::Exp>>>>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
    let mut arg: metamodelica::Ref<Absyn::ElementArg> = arg;
    let mut r#mod: metamodelica::Ref<Absyn::Modification>;
    let mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut eq_mod: metamodelica::Ref<Absyn::EqMod>;
    let () = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(__esc_mod), info: __arg_info, .. } => {
            r#mod = (*__esc_mod).clone();
            let __arc2 = r#mod.clone();
            let Absyn::Modification { elementArgLst: __pa0, eqMod: __pa1 } = &*__arc2;
            args = metamodelica::Own::own(__pa0);
            eq_mod = metamodelica::Own::own(__pa1);
            args = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
        for mut a in (args).into_iter().cloned() {
            let __x = replacePlaceholders(a.clone(), placeholders.clone(), info)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            eq_mod = replacePlaceholdersEqMod(eq_mod, placeholders, &(list![__arg_info.clone(), info.clone()]))?;
            assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; modification = Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: args, eqMod: eq_mod })));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(arg)
}

fn replacePlaceholdersEqMod(
    mut eqMod: metamodelica::Ref<Absyn::EqMod>,
    mut placeholders: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Option<metamodelica::Ref<Absyn::Exp>>>>,
    mut info: &metamodelica::List<SourceInfo>,
) -> Result<metamodelica::Ref<Absyn::EqMod>> {
    let mut eqMod: metamodelica::Ref<Absyn::EqMod> = eqMod;
    let () = (match &*eqMod {
        Absyn::EqMod::EQMOD { exp: __eqMod_exp, .. } => {
            assign_variant_field!(eqMod => Absyn::EqMod::EQMOD; exp = AbsynUtil::traverseExp(__eqMod_exp.clone(), (std::sync::Arc::new({ let __pe_b2 = info.clone(); move |__pe_a0, __pe_a1| replacePlaceholdersExp(__pe_a0, __pe_a1, &__pe_b2) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Option<metamodelica::Ref<Absyn::Exp>>>>) -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Option<metamodelica::Ref<Absyn::Exp>>>>)> + 'static>), placeholders)?.0);
            ()
        }
        _ => (),
    });
    Ok(eqMod)
}

fn replacePlaceholdersExp(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut placeholders: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Option<metamodelica::Ref<Absyn::Exp>>>>,
    mut info: &metamodelica::List<SourceInfo>,
) -> Result<(
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, Option<metamodelica::Ref<Absyn::Exp>>>>,
)> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut outPlaceholders: metamodelica::Ref<
        UnorderedMap::UnorderedMap<ArcStr, Option<metamodelica::Ref<Absyn::Exp>>>,
    > = placeholders.clone();
    let mut name: ArcStr;
    let mut len: i32;
    let mut new_exp: Option<metamodelica::Ref<Absyn::Exp>>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: __esc_name, subscripts: Deref @ metamodelica::ListNode::Nil } } => {
            name = (*__esc_name).clone();
            len = ((name).len() as i32);
            if len > 4 && stringGet(&name,1)? == 39 && stringGet(&name,2)? == 37 && stringGet(&name,len - 1)? == 37 && stringGet(&name,len)? == 39 {
                name = substring(name.clone(), 3, len - 2)?;
                new_exp = UnorderedMap::getOrDefault(name.clone(), placeholders, None)?;
                if (new_exp).is_none() {
                    Error::addMultiSourceMessage(&(Error::CONVERSION_MISSING_PLACEHOLDER_VALUE.clone()), &(list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("%")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!("%")); ArcStr::from(__mm_s) }]), info)?;
                }
                let __pa0 = ::match_deref::match_deref! { match &(new_exp) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                outExp = metamodelica::Own::own(__pa0);
            } else {
                outExp = exp;
            }
            outExp
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outPlaceholders))
}

fn mergeModifiers(
    mut outerMods: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut innerMods: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>> {
    let mut mods: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = outerMods.clone();
    for mut m in &*innerMods.reverse() {
        if !(isModifierInList(metamodelica::AsArg::as_arg(&m), &outerMods)?) {
            mods = metamodelica::cons(m.clone(), mods);
        }
    }
    Ok(mods)
}

fn convertTypeSpec(
    mut ty: metamodelica::Ref<Absyn::TypeSpec>,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: &Env,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Absyn::TypeSpec>,
    RuleTable,
    metamodelica::List<ConversionRule>,
)> {
    let mut ty: metamodelica::Ref<Absyn::TypeSpec> = ty;
    let mut localRules: RuleTable;
    let mut modifierRules: metamodelica::List<ConversionRule>;
    let mut ty_rule: Option<ConversionRule>;
    let mut ty_path: metamodelica::Ref<Path>;
    let mut import_path: Option<(metamodelica::Ref<Path>, ArcStr)>;
    (ty_path, import_path) = applyImportsToPath(AbsynUtil::typeSpecPath(&ty), &env.imports)?;
    (ty_rule, localRules, modifierRules) = lookupTypeRules(&ty_path, rules.clone(), env)?;
    let () = (match &*ty {
        Absyn::TypeSpec::TPATH { .. } => {
            if (ty_rule).is_some() {
                assign_variant_field!(ty => Absyn::TypeSpec::TPATH; path = convertTypePath(ty_path, &(Util::getOption(ty_rule)?), import_path, info)?);
            }
            assign_variant_field!(ty => Absyn::TypeSpec::TPATH; arrayDim = convertOption(var_field!((*ty).arrayDim, Absyn::TypeSpec::TPATH).clone(), &({ let __pe_b1 = localRules.clone(); move |__pe_a0, __pe_a2, __pe_a3, __pe_a4| convertSubscripts(__pe_a0, __pe_b1.clone(), &__pe_a2, &__pe_a3, &__pe_a4) }), rules, env.clone(), info.clone())?);
            ()
        }
        Absyn::TypeSpec::TCOMPLEX { .. } => {
            if (ty_rule).is_some() {
                assign_variant_field!(ty => Absyn::TypeSpec::TCOMPLEX; path = convertTypePath(ty_path, &(Util::getOption(ty_rule)?), import_path, info)?);
            }
            assign_variant_field!(ty => Absyn::TypeSpec::TCOMPLEX;
                        typeSpecs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::TypeSpec>> = metamodelica::nil();
                for mut t in (var_field!((*ty).typeSpecs, Absyn::TypeSpec::TCOMPLEX).clone()).into_iter().cloned() {
                    let __x = (convertTypeSpec(t.clone(), rules.clone(), env, info)?).0;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        arrayDim = convertOption(var_field!((*ty).arrayDim, Absyn::TypeSpec::TCOMPLEX).clone(), &({ let __pe_b1 = localRules.clone(); move |__pe_a0, __pe_a2, __pe_a3, __pe_a4| convertSubscripts(__pe_a0, __pe_b1.clone(), &__pe_a2, &__pe_a3, &__pe_a4) }), rules, env.clone(), info.clone())?
                    );
            ()
        }
        _ => (),
    });
    Ok((ty, localRules, modifierRules))
}

fn convertTypePath(
    mut path: metamodelica::Ref<Path>,
    mut rule: &ConversionRule,
    mut importPath: Option<(metamodelica::Ref<Path>, ArcStr)>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Path>> {
    let mut path: metamodelica::Ref<Path> = path;
    let () = (match rule.clone() {
        ConversionRule::CLASS { .. } => {
            if AbsynUtil::pathPartCount(&path, 0)
                == metamodelica::arrayLength(var_field!(rule.oldPath, ConversionRule::CLASS).clone())
            {
                path = var_field!(rule.newPath, ConversionRule::CLASS).clone();
            } else {
                path = Util::foldcallN(
                    metamodelica::arrayLength(var_field!(rule.oldPath, ConversionRule::CLASS).clone()),
                    &AbsynUtil::pathRest,
                    path,
                )?;
                path = AbsynUtil::joinPaths(var_field!(rule.newPath, ConversionRule::CLASS).clone(), path)?;
            }
            ()
        }
        ConversionRule::MESSAGE { .. } => {
            Error::addSourceMessage(
                &(Error::CONVERSION_MESSAGE.clone()),
                list![var_field!(rule.message, ConversionRule::MESSAGE).clone()],
                info,
            )?;
            ()
        }
        _ => (),
    });
    path = stripImportPath(path, importPath)?;
    Ok(path)
}

fn convertElementItems(
    mut elements: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
    mut extendsRules: &metamodelica::List<metamodelica::Ref<ConversionRules::ConversionRules>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>> {
    let mut elements: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = elements;
    elements = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = metamodelica::nil();
        for mut e in (elements).into_iter().cloned() {
            let __x = convertElementItem(e.clone(), rules.clone(), env.clone(), extendsRules)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    elements = filterDuplicateImports(elements)?;
    Ok(elements)
}

fn convertElementItem(
    mut element: metamodelica::Ref<Absyn::ElementItem>,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
    mut extendsRules: &metamodelica::List<metamodelica::Ref<ConversionRules::ConversionRules>>,
) -> Result<metamodelica::Ref<Absyn::ElementItem>> {
    let mut element: metamodelica::Ref<Absyn::ElementItem> = element;
    let () = (match &*element {
        Absyn::ElementItem::ELEMENTITEM {
            element: __element_element,
        } => {
            assign_variant_field!(element => Absyn::ElementItem::ELEMENTITEM; element = convertElement(__element_element.clone(), rules, env, extendsRules)?);
            ()
        }
        _ => (),
    });
    Ok(element)
}

fn convertElement(
    mut element: metamodelica::Ref<Absyn::Element>,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
    mut extendsRules: &metamodelica::List<metamodelica::Ref<ConversionRules::ConversionRules>>,
) -> Result<metamodelica::Ref<Absyn::Element>> {
    let mut element: metamodelica::Ref<Absyn::Element> = element;
    let () = (match &*element {
        Absyn::Element::ELEMENT {
            info: __element_info,
            specification: __element_specification,
            ..
        } => {
            assign_variant_field!(element => Absyn::Element::ELEMENT;
                specification = convertElementSpec(__element_specification.clone(), rules.clone(), env.clone(), extendsRules, __element_info.clone())?,
                constrainClass = convertOption(var_field!((*element).constrainClass, Absyn::Element::ELEMENT).clone(), &convertConstrainClass, rules, env, var_field!((*element).info, Absyn::Element::ELEMENT).clone())?
            );
            ()
        }
        Absyn::Element::DEFINEUNIT {
            args: __element_args,
            info: __element_info,
            ..
        } => {
            let mut local_rules: RuleTable;
            local_rules = newRuleTable();
            assign_variant_field!(element => Absyn::Element::DEFINEUNIT; args = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>> = metamodelica::nil();
                for mut a in (__element_args.clone()).into_iter().cloned() {
                    let __x = convertNamedArg(a.clone(), local_rules.clone(), &rules, &env, metamodelica::AsArg::as_arg(&__element_info))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        _ => (),
    });
    Ok(element)
}

fn convertConstrainClass(
    mut cc: metamodelica::Ref<Absyn::ConstrainClass>,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<Absyn::ConstrainClass>> {
    let mut cc: metamodelica::Ref<Absyn::ConstrainClass> = cc;
    assign_field!(
        cc.elementSpec = convertElementSpec(cc.elementSpec.clone(), rules, env, &(metamodelica::nil()), info)?
    );
    Ok(cc)
}

fn convertElementSpec(
    mut spec: metamodelica::Ref<Absyn::ElementSpec>,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
    mut extendsRules: &metamodelica::List<metamodelica::Ref<ConversionRules::ConversionRules>>,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<Absyn::ElementSpec>> {
    let mut spec: metamodelica::Ref<Absyn::ElementSpec> = spec;
    let () = (match &*spec {
        Absyn::ElementSpec::CLASSDEF {
            class_: __spec_class_, ..
        } => {
            assign_variant_field!(spec => Absyn::ElementSpec::CLASSDEF; class_ = convertClass(__spec_class_.clone(), rules, env, extendsRules)?);
            ()
        }
        Absyn::ElementSpec::EXTENDS { path: __spec_path, .. } => {
            let mut local_rules: RuleTable;
            let mut mod_rules: metamodelica::List<ConversionRule>;
            let mut ty_path: metamodelica::Ref<Path>;
            let mut import_path: Option<(metamodelica::Ref<Path>, ArcStr)>;
            (ty_path, import_path) = applyImportsToPath(__spec_path.clone(), &env.imports)?;
            (_, local_rules, mod_rules) = lookupTypeRules(&ty_path, rules.clone(), &env)?;
            ty_path = convertPath(ty_path, rules.clone(), &env.imports, &info)?;
            assign_variant_field!(spec => Absyn::ElementSpec::EXTENDS;
                path = stripImportPath(ty_path, import_path)?,
                elementArg = convertModification2(&mod_rules, var_field!((*spec).elementArg, Absyn::ElementSpec::EXTENDS).clone())?
            );
            assign_variant_field!(spec => Absyn::ElementSpec::EXTENDS; elementArg = convertElementArgs(var_field!((*spec).elementArg, Absyn::ElementSpec::EXTENDS).clone(), local_rules, rules, env)?);
            ()
        }
        Absyn::ElementSpec::IMPORT {
            import_: __spec_import_,
            ..
        } => {
            assign_variant_field!(spec => Absyn::ElementSpec::IMPORT; import_ = convertImport(__spec_import_.clone(), rules, &info)?);
            ()
        }
        Absyn::ElementSpec::COMPONENTS {
            typeSpec: __spec_typeSpec,
            ..
        } => {
            let mut local_rules: RuleTable;
            let mut mod_rules: metamodelica::List<ConversionRule>;
            let mut ty: metamodelica::Ref<Absyn::TypeSpec>;
            (ty, local_rules, mod_rules) = convertTypeSpec(__spec_typeSpec.clone(), rules.clone(), &env, &info)?;
            assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS;
                        typeSpec = ty,
                        components = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>> = metamodelica::nil();
                for mut c in (var_field!((*spec).components, Absyn::ElementSpec::COMPONENTS).clone()).into_iter().cloned() {
                    let __x = convertComponentItem(c.clone(), local_rules.clone(), &mod_rules, rules.clone(), env.clone(), info.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            ()
        }
        _ => (),
    });
    Ok(spec)
}

fn convertImport(
    mut imp: Absyn::Import,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut info: &SourceInfo,
) -> Result<Absyn::Import> {
    let mut imp: Absyn::Import = imp;
    let () = (match imp.clone() {
        Absyn::Import::NAMED_IMPORT { .. } => {
            let __owned_variant_path_0 = convertPath(
                var_field!(imp.path, Absyn::Import::NAMED_IMPORT).clone(),
                rules,
                &(ImportTreeImpl::new()),
                info,
            )?;
            if let Absyn::Import::NAMED_IMPORT { path, .. } = &mut imp {
                *path = __owned_variant_path_0;
            } else {
                panic!("owned-variant field-assign: value held a different variant than Absyn::Import::NAMED_IMPORT");
            }
            ()
        }
        Absyn::Import::QUAL_IMPORT { .. } => {
            let __owned_variant_path_0 = convertPath(
                var_field!(imp.path, Absyn::Import::QUAL_IMPORT).clone(),
                rules,
                &(ImportTreeImpl::new()),
                info,
            )?;
            if let Absyn::Import::QUAL_IMPORT { path, .. } = &mut imp {
                *path = __owned_variant_path_0;
            } else {
                panic!("owned-variant field-assign: value held a different variant than Absyn::Import::QUAL_IMPORT");
            }
            ()
        }
        Absyn::Import::UNQUAL_IMPORT { .. } => {
            let __owned_variant_path_0 = convertPath(
                var_field!(imp.path, Absyn::Import::UNQUAL_IMPORT).clone(),
                rules,
                &(ImportTreeImpl::new()),
                info,
            )?;
            if let Absyn::Import::UNQUAL_IMPORT { path, .. } = &mut imp {
                *path = __owned_variant_path_0;
            } else {
                panic!("owned-variant field-assign: value held a different variant than Absyn::Import::UNQUAL_IMPORT");
            }
            ()
        }
        Absyn::Import::GROUP_IMPORT { .. } => {
            let __owned_variant_prefix_0 = convertPath(
                var_field!(imp.prefix, Absyn::Import::GROUP_IMPORT).clone(),
                rules,
                &(ImportTreeImpl::new()),
                info,
            )?;
            if let Absyn::Import::GROUP_IMPORT { prefix, .. } = &mut imp {
                *prefix = __owned_variant_prefix_0;
            } else {
                panic!("owned-variant field-assign: value held a different variant than Absyn::Import::GROUP_IMPORT");
            }
            ()
        }
        _ => (),
    });
    imp = simplifyImport(imp);
    Ok(imp)
}

fn simplifyImport(mut imp: Absyn::Import) -> Absyn::Import {
    let mut imp: Absyn::Import = imp;
    imp = (match imp.clone() {
        Absyn::Import::NAMED_IMPORT { .. }
            if (metamodelica::stringEq(
                &(var_field!(imp.name, Absyn::Import::NAMED_IMPORT).clone()),
                &(AbsynUtil::pathLastIdent(var_field!(imp.path, Absyn::Import::NAMED_IMPORT))),
            )) =>
        {
            Absyn::Import::QUAL_IMPORT {
                path: var_field!(imp.path, Absyn::Import::NAMED_IMPORT).clone(),
            }
        }
        _ => imp.clone(),
    });
    imp
}

fn filterDuplicateImports(
    mut elements: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>> {
    let mut outElements: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut imports: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Path>>>;
    imports = UnorderedSet::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<Path>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(AbsynUtil::pathHash(&__a0))
        }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Path>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<Path>, __a1: metamodelica::Ref<Path>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1))
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Path>, metamodelica::Ref<Path>) -> Result<bool> + 'static,
            >),
        1,
    );
    outElements = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = metamodelica::nil();
        for mut e in (elements).into_iter().cloned() {
            if !(!(importExists(&(e.clone()), imports.clone())?)) {
                continue;
            }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outElements)
}

fn importExists(
    mut element: &metamodelica::Ref<Absyn::ElementItem>,
    mut imports: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<Path>>>,
) -> Result<bool> {
    let mut exists: bool;
    let mut path: metamodelica::Ref<Path>;
    exists = (::match_deref::match_deref! { match element {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::IMPORT { import_: Absyn::Import::QUAL_IMPORT { path: __esc_path }, .. }, .. } } => {
            path = (*__esc_path).clone();
            exists = UnorderedSet::contains(path.clone(), imports.clone())?;
            if !(exists) {
                UnorderedSet::add(path.clone(), imports)?;
            }
            exists
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exists)
}

fn convertComponentItem(
    mut comp: metamodelica::Ref<Absyn::ComponentItem>,
    mut localRules: RuleTable,
    mut modifierRules: &metamodelica::List<ConversionRule>,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<Absyn::ComponentItem>> {
    let mut comp: metamodelica::Ref<Absyn::ComponentItem> = comp;
    assign_field!(
        comp.component = convertComponent(
            comp.component.clone(),
            localRules.clone(),
            modifierRules,
            rules.clone(),
            env.clone(),
            info.clone()
        )?,
        comp.condition = convertOptExp(comp.condition.clone(), localRules, &rules, &env, &info)?
    );
    Ok(comp)
}

fn convertComponent(
    mut comp: Absyn::Component,
    mut localRules: RuleTable,
    mut modifierRules: &metamodelica::List<ConversionRule>,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
    mut info: SourceInfo,
) -> Result<Absyn::Component> {
    let mut comp: Absyn::Component = comp;
    comp.arrayDim = convertSubscripts(comp.arrayDim.clone(), localRules.clone(), &rules, &env, &info)?;
    if !((modifierRules).is_empty()) {
        comp.modification = convertModification(comp.modification.clone(), modifierRules)?;
    }
    comp.modification = convertModificationExps(comp.modification.clone(), localRules, rules, env, info)?;
    Ok(comp)
}

fn convertEquationItems(
    mut eqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut localRules: RuleTable,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>> {
    let mut eqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>> = eqs;
    eqs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>> = metamodelica::nil();
        for mut eq in (eqs).into_iter().cloned() {
            let __x = convertEquationItem(eq.clone(), localRules.clone(), rules.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(eqs)
}

fn convertEquationItem(
    mut eq: metamodelica::Ref<Absyn::EquationItem>,
    mut localRules: RuleTable,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
) -> Result<metamodelica::Ref<Absyn::EquationItem>> {
    let mut eq: metamodelica::Ref<Absyn::EquationItem> = eq;
    let () = (match &*eq {
        Absyn::EquationItem::EQUATIONITEM {
            equation_: __eq_equation_,
            info: __eq_info,
            ..
        } => {
            assign_variant_field!(eq => Absyn::EquationItem::EQUATIONITEM; equation_ = convertEquation(__eq_equation_.clone(), localRules, rules, env, metamodelica::AsArg::as_arg(&__eq_info))?);
            ()
        }
        _ => (),
    });
    Ok(eq)
}

fn convertEquation(
    mut eq: metamodelica::Ref<Absyn::Equation>,
    mut localRules: RuleTable,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Absyn::Equation>> {
    let mut eq: metamodelica::Ref<Absyn::Equation> = eq;
    let () = (match &*eq {
        Absyn::Equation::EQ_IF { ifExp: __eq_ifExp, .. } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_IF;
                ifExp = convertExp(__eq_ifExp.clone(), localRules.clone(), &rules, &env, info)?,
                equationTrueItems = convertEquationItems(var_field!((*eq).equationTrueItems, Absyn::Equation::EQ_IF).clone(), localRules.clone(), rules.clone(), env.clone())?,
                elseIfBranches = convertBranches(var_field!((*eq).elseIfBranches, Absyn::Equation::EQ_IF).clone(), &({ let __pe_b4 = info.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3| convertExp(__pe_a0, __pe_a1, &__pe_a2, &__pe_a3, &__pe_b4) }), &convertEquationItems, localRules.clone(), rules.clone(), env.clone())?,
                equationElseItems = convertEquationItems(var_field!((*eq).equationElseItems, Absyn::Equation::EQ_IF).clone(), localRules, rules, env)?
            );
            ()
        }
        Absyn::Equation::EQ_EQUALS {
            leftSide: __eq_leftSide,
            ..
        } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_EQUALS;
                leftSide = convertExp(__eq_leftSide.clone(), localRules.clone(), &rules, &env, info)?,
                rightSide = convertExp(var_field!((*eq).rightSide, Absyn::Equation::EQ_EQUALS).clone(), localRules, &rules, &env, info)?
            );
            ()
        }
        Absyn::Equation::EQ_PDE {
            leftSide: __eq_leftSide,
            ..
        } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_PDE;
                leftSide = convertExp(__eq_leftSide.clone(), localRules.clone(), &rules, &env, info)?,
                rightSide = convertExp(var_field!((*eq).rightSide, Absyn::Equation::EQ_PDE).clone(), localRules, &rules, &env, info)?
            );
            ()
        }
        Absyn::Equation::EQ_CONNECT {
            connector1: __eq_connector1,
            ..
        } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_CONNECT;
                connector1 = convertCref(__eq_connector1.clone(), localRules.clone(), rules.clone(), &env, info)?,
                connector2 = convertCref(var_field!((*eq).connector2, Absyn::Equation::EQ_CONNECT).clone(), localRules, rules, &env, info)?
            );
            ()
        }
        Absyn::Equation::EQ_FOR {
            iterators: __eq_iterators,
            ..
        } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_FOR;
                iterators = convertForIterators(__eq_iterators.clone(), localRules.clone(), &rules, &env, info)?,
                forEquations = convertEquationItems(var_field!((*eq).forEquations, Absyn::Equation::EQ_FOR).clone(), localRules, rules, env)?
            );
            ()
        }
        Absyn::Equation::EQ_WHEN_E {
            whenExp: __eq_whenExp, ..
        } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_WHEN_E;
                whenExp = convertExp(__eq_whenExp.clone(), localRules.clone(), &rules, &env, info)?,
                whenEquations = convertEquationItems(var_field!((*eq).whenEquations, Absyn::Equation::EQ_WHEN_E).clone(), localRules.clone(), rules.clone(), env.clone())?,
                elseWhenEquations = convertBranches(var_field!((*eq).elseWhenEquations, Absyn::Equation::EQ_WHEN_E).clone(), &({ let __pe_b4 = info.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3| convertExp(__pe_a0, __pe_a1, &__pe_a2, &__pe_a3, &__pe_b4) }), &convertEquationItems, localRules, rules, env)?
            );
            ()
        }
        Absyn::Equation::EQ_NORETCALL {
            functionName: __eq_functionName,
            ..
        } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_NORETCALL;
                functionName = convertCref(__eq_functionName.clone(), localRules.clone(), rules.clone(), &env, info)?,
                functionArgs = convertFunctionArgs(var_field!((*eq).functionArgs, Absyn::Equation::EQ_NORETCALL).clone(), localRules, &rules, &env, info)?
            );
            ()
        }
        Absyn::Equation::EQ_FAILURE { equ: __eq_equ } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_FAILURE; equ = convertEquationItem(__eq_equ.clone(), localRules, rules, env)?);
            ()
        }
        _ => (),
    });
    Ok(eq)
}

fn convertAlgorithmItems(
    mut algs: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    mut localRules: RuleTable,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>> {
    let mut algs: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>> = algs;
    algs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>> = metamodelica::nil();
        for mut alg in (algs).into_iter().cloned() {
            let __x = convertAlgorithmItem(alg.clone(), localRules.clone(), rules.clone(), env.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(algs)
}

fn convertAlgorithmItem(
    mut alg: metamodelica::Ref<Absyn::AlgorithmItem>,
    mut localRules: RuleTable,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
) -> Result<metamodelica::Ref<Absyn::AlgorithmItem>> {
    let mut alg: metamodelica::Ref<Absyn::AlgorithmItem> = alg;
    let () = (match &*alg {
        Absyn::AlgorithmItem::ALGORITHMITEM {
            algorithm_: __alg_algorithm_,
            info: __alg_info,
            ..
        } => {
            assign_variant_field!(alg => Absyn::AlgorithmItem::ALGORITHMITEM; algorithm_ = convertAlgorithm(__alg_algorithm_.clone(), localRules, rules, env, metamodelica::AsArg::as_arg(&__alg_info))?);
            ()
        }
        _ => (),
    });
    Ok(alg)
}

fn convertAlgorithm(
    mut alg: metamodelica::Ref<Absyn::Algorithm>,
    mut localRules: RuleTable,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Absyn::Algorithm>> {
    let mut alg: metamodelica::Ref<Absyn::Algorithm> = alg;
    let () = (match &*alg {
        Absyn::Algorithm::ALG_ASSIGN {
            assignComponent: __alg_assignComponent,
            ..
        } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_ASSIGN;
                assignComponent = convertExp(__alg_assignComponent.clone(), localRules.clone(), &rules, &env, info)?,
                value = convertExp(var_field!((*alg).value, Absyn::Algorithm::ALG_ASSIGN).clone(), localRules, &rules, &env, info)?
            );
            ()
        }
        Absyn::Algorithm::ALG_IF { ifExp: __alg_ifExp, .. } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_IF;
                ifExp = convertExp(__alg_ifExp.clone(), localRules.clone(), &rules, &env, info)?,
                trueBranch = convertAlgorithmItems(var_field!((*alg).trueBranch, Absyn::Algorithm::ALG_IF).clone(), localRules.clone(), rules.clone(), env.clone())?,
                elseIfAlgorithmBranch = convertBranches(var_field!((*alg).elseIfAlgorithmBranch, Absyn::Algorithm::ALG_IF).clone(), &({ let __pe_b4 = info.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3| convertExp(__pe_a0, __pe_a1, &__pe_a2, &__pe_a3, &__pe_b4) }), &convertAlgorithmItems, localRules.clone(), rules.clone(), env.clone())?,
                elseBranch = convertAlgorithmItems(var_field!((*alg).elseBranch, Absyn::Algorithm::ALG_IF).clone(), localRules, rules, env)?
            );
            ()
        }
        Absyn::Algorithm::ALG_FOR {
            iterators: __alg_iterators,
            ..
        } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_FOR;
                iterators = convertForIterators(__alg_iterators.clone(), localRules.clone(), &rules, &env, info)?,
                forBody = convertAlgorithmItems(var_field!((*alg).forBody, Absyn::Algorithm::ALG_FOR).clone(), localRules, rules, env)?
            );
            ()
        }
        Absyn::Algorithm::ALG_PARFOR {
            iterators: __alg_iterators,
            ..
        } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_PARFOR;
                iterators = convertForIterators(__alg_iterators.clone(), localRules.clone(), &rules, &env, info)?,
                parforBody = convertAlgorithmItems(var_field!((*alg).parforBody, Absyn::Algorithm::ALG_PARFOR).clone(), localRules, rules, env)?
            );
            ()
        }
        Absyn::Algorithm::ALG_WHILE {
            boolExpr: __alg_boolExpr,
            ..
        } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_WHILE;
                boolExpr = convertExp(__alg_boolExpr.clone(), localRules.clone(), &rules, &env, info)?,
                whileBody = convertAlgorithmItems(var_field!((*alg).whileBody, Absyn::Algorithm::ALG_WHILE).clone(), localRules, rules, env)?
            );
            ()
        }
        Absyn::Algorithm::ALG_WHEN_A {
            boolExpr: __alg_boolExpr,
            ..
        } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_WHEN_A;
                boolExpr = convertExp(__alg_boolExpr.clone(), localRules.clone(), &rules, &env, info)?,
                whenBody = convertAlgorithmItems(var_field!((*alg).whenBody, Absyn::Algorithm::ALG_WHEN_A).clone(), localRules.clone(), rules.clone(), env.clone())?,
                elseWhenAlgorithmBranch = convertBranches(var_field!((*alg).elseWhenAlgorithmBranch, Absyn::Algorithm::ALG_WHEN_A).clone(), &({ let __pe_b4 = info.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3| convertExp(__pe_a0, __pe_a1, &__pe_a2, &__pe_a3, &__pe_b4) }), &convertAlgorithmItems, localRules, rules, env)?
            );
            ()
        }
        Absyn::Algorithm::ALG_NORETCALL {
            functionCall: __alg_functionCall,
            ..
        } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_NORETCALL;
                functionCall = convertCref(__alg_functionCall.clone(), localRules.clone(), rules.clone(), &env, info)?,
                functionArgs = convertFunctionArgs(var_field!((*alg).functionArgs, Absyn::Algorithm::ALG_NORETCALL).clone(), localRules, &rules, &env, info)?
            );
            ()
        }
        Absyn::Algorithm::ALG_FAILURE { equ: __alg_equ } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_FAILURE; equ = convertAlgorithmItems(__alg_equ.clone(), localRules, rules, env)?);
            ()
        }
        Absyn::Algorithm::ALG_TRY { body: __alg_body, .. } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_TRY;
                body = convertAlgorithmItems(__alg_body.clone(), localRules.clone(), rules.clone(), env.clone())?,
                elseBody = convertAlgorithmItems(var_field!((*alg).elseBody, Absyn::Algorithm::ALG_TRY).clone(), localRules, rules, env)?
            );
            ()
        }
        _ => (),
    });
    Ok(alg)
}

fn convertBranches<
    CondT: Clone + 'static + metamodelica::gc::MMTrace,
    BodyT: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut branches: metamodelica::List<(CondT, BodyT)>,
    mut condFunc: &dyn ::std::ops::Fn(
        CondT,
        metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::List<ConversionRule>>>,
        metamodelica::Ref<ConversionRules::ConversionRules>,
        Env,
    ) -> Result<CondT>,
    mut bodyFunc: &dyn ::std::ops::Fn(
        BodyT,
        metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::List<ConversionRule>>>,
        metamodelica::Ref<ConversionRules::ConversionRules>,
        Env,
    ) -> Result<BodyT>,
    mut localRules: RuleTable,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
) -> Result<metamodelica::List<(CondT, BodyT)>> {
    pub type CondFunc<CondT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(CondT, RuleTable, metamodelica::Ref<ConversionRules::ConversionRules>, Env) -> Result<CondT>
            + 'static,
    >;

    pub type BodyFunc<BodyT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(BodyT, RuleTable, metamodelica::Ref<ConversionRules::ConversionRules>, Env) -> Result<BodyT>
            + 'static,
    >;

    let mut branches: metamodelica::List<(CondT, BodyT)> = branches;
    branches = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut b in (branches).into_iter().cloned() {
            let __x = (
                condFunc(Util::tuple21(b.clone()), localRules.clone(), rules.clone(), env.clone())?,
                bodyFunc(Util::tuple22(b.clone()), localRules.clone(), rules.clone(), env.clone())?,
            );
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(branches)
}

fn convertForIterators(
    mut iters: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>>,
    mut localRules: RuleTable,
    mut rules: &metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: &Env,
    mut info: &SourceInfo,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ForIterator>>> {
    let mut iters: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>> = iters;
    iters = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>> = metamodelica::nil();
        for mut i in (iters).into_iter().cloned() {
            let __x = convertForIterator(i.clone(), localRules.clone(), rules, env, info)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(iters)
}

fn convertForIterator(
    mut iter: metamodelica::Ref<Absyn::ForIterator>,
    mut localRules: RuleTable,
    mut rules: &metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: &Env,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Absyn::ForIterator>> {
    let mut iter: metamodelica::Ref<Absyn::ForIterator> = iter;
    assign_field!(
        iter.guardExp = convertOptExp(iter.guardExp.clone(), localRules.clone(), rules, env, info)?,
        iter.range = convertOptExp(iter.range.clone(), localRules, rules, env, info)?
    );
    Ok(iter)
}

fn convertExternalDecl(
    mut extDecl: metamodelica::Ref<Absyn::ExternalDecl>,
    mut localRules: RuleTable,
    mut rules: &metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: &Env,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Absyn::ExternalDecl>> {
    let mut extDecl: metamodelica::Ref<Absyn::ExternalDecl> = extDecl;
    assign_field!(extDecl.args = convertExps(extDecl.args.clone(), localRules, rules, env, info)?);
    Ok(extDecl)
}

fn convertExps(
    mut exps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut localRules: RuleTable,
    mut rules: &metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: &Env,
    mut info: &SourceInfo,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Exp>>> {
    let mut exps: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = exps;
    exps = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
        for mut e in (exps).into_iter().cloned() {
            let __x = convertExp(e.clone(), localRules.clone(), rules, env, info)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(exps)
}

fn convertOptExp(
    mut exp: Option<metamodelica::Ref<Absyn::Exp>>,
    mut localRules: RuleTable,
    mut rules: &metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: &Env,
    mut info: &SourceInfo,
) -> Result<Option<metamodelica::Ref<Absyn::Exp>>> {
    let mut exp: Option<metamodelica::Ref<Absyn::Exp>> = exp;
    exp = (::match_deref::match_deref! { match &(exp) {
        Some(e) => {
            Some(convertExp(e.clone(), localRules, rules, env, info)?)
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn convertExp(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut localRules: RuleTable,
    mut rules: &metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: &Env,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    let () = (match &*exp {
        Absyn::Exp::CREF {
            componentRef: __exp_componentRef,
        } => {
            assign_variant_field!(exp => Absyn::Exp::CREF; componentRef = convertCref(__exp_componentRef.clone(), localRules, rules.clone(), env, info)?);
            ()
        }
        Absyn::Exp::BINARY { exp1: __exp_exp1, .. } => {
            assign_variant_field!(exp => Absyn::Exp::BINARY;
                exp1 = convertExp(__exp_exp1.clone(), localRules.clone(), rules, env, info)?,
                exp2 = convertExp(var_field!((*exp).exp2, Absyn::Exp::BINARY).clone(), localRules, rules, env, info)?
            );
            ()
        }
        Absyn::Exp::UNARY { exp: __exp_exp, .. } => {
            assign_variant_field!(exp => Absyn::Exp::UNARY; exp = convertExp(__exp_exp.clone(), localRules, rules, env, info)?);
            ()
        }
        Absyn::Exp::LBINARY { exp1: __exp_exp1, .. } => {
            assign_variant_field!(exp => Absyn::Exp::LBINARY;
                exp1 = convertExp(__exp_exp1.clone(), localRules.clone(), rules, env, info)?,
                exp2 = convertExp(var_field!((*exp).exp2, Absyn::Exp::LBINARY).clone(), localRules, rules, env, info)?
            );
            ()
        }
        Absyn::Exp::LUNARY { exp: __exp_exp, .. } => {
            assign_variant_field!(exp => Absyn::Exp::LUNARY; exp = convertExp(__exp_exp.clone(), localRules, rules, env, info)?);
            ()
        }
        Absyn::Exp::RELATION { exp1: __exp_exp1, .. } => {
            assign_variant_field!(exp => Absyn::Exp::RELATION;
                exp1 = convertExp(__exp_exp1.clone(), localRules.clone(), rules, env, info)?,
                exp2 = convertExp(var_field!((*exp).exp2, Absyn::Exp::RELATION).clone(), localRules, rules, env, info)?
            );
            ()
        }
        Absyn::Exp::IFEXP { ifExp: __exp_ifExp, .. } => {
            assign_variant_field!(exp => Absyn::Exp::IFEXP;
                ifExp = convertExp(__exp_ifExp.clone(), localRules.clone(), rules, env, info)?,
                trueBranch = convertExp(var_field!((*exp).trueBranch, Absyn::Exp::IFEXP).clone(), localRules.clone(), rules, env, info)?,
                elseBranch = convertExp(var_field!((*exp).elseBranch, Absyn::Exp::IFEXP).clone(), localRules.clone(), rules, env, info)?,
                elseIfBranch = convertBranches(var_field!((*exp).elseIfBranch, Absyn::Exp::IFEXP).clone(), &({ let __pe_b4 = info.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3| convertExp(__pe_a0, __pe_a1, &__pe_a2, &__pe_a3, &__pe_b4) }), &({ let __pe_b4 = info.clone(); move |__pe_a0, __pe_a1, __pe_a2, __pe_a3| convertExp(__pe_a0, __pe_a1, &__pe_a2, &__pe_a3, &__pe_b4) }), localRules, rules.clone(), env.clone())?
            );
            ()
        }
        Absyn::Exp::CALL {
            function_: __exp_function_,
            ..
        } => {
            assign_variant_field!(exp => Absyn::Exp::CALL;
                function_ = convertCref(__exp_function_.clone(), localRules.clone(), rules.clone(), env, info)?,
                functionArgs = convertFunctionArgs(var_field!((*exp).functionArgs, Absyn::Exp::CALL).clone(), localRules, rules, env, info)?
            );
            ()
        }
        Absyn::Exp::PARTEVALFUNCTION {
            function_: __exp_function_,
            ..
        } => {
            assign_variant_field!(exp => Absyn::Exp::PARTEVALFUNCTION;
                function_ = convertCref(__exp_function_.clone(), localRules.clone(), rules.clone(), env, info)?,
                functionArgs = convertFunctionArgs(var_field!((*exp).functionArgs, Absyn::Exp::PARTEVALFUNCTION).clone(), localRules, rules, env, info)?
            );
            ()
        }
        Absyn::Exp::ARRAY {
            arrayExp: __exp_arrayExp,
        } => {
            assign_variant_field!(exp => Absyn::Exp::ARRAY; arrayExp = convertExps(__exp_arrayExp.clone(), localRules, rules, env, info)?);
            ()
        }
        Absyn::Exp::MATRIX { matrix: __exp_matrix } => {
            assign_variant_field!(exp => Absyn::Exp::MATRIX; matrix = ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>> = metamodelica::nil();
                for mut e in (__exp_matrix.clone()).into_iter().cloned() {
                    let __x = convertExps(e.clone(), localRules.clone(), rules, env, info)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        Absyn::Exp::RANGE { start: __exp_start, .. } => {
            assign_variant_field!(exp => Absyn::Exp::RANGE;
                start = convertExp(__exp_start.clone(), localRules.clone(), rules, env, info)?,
                step = convertOptExp(var_field!((*exp).step, Absyn::Exp::RANGE).clone(), localRules.clone(), rules, env, info)?,
                stop = convertExp(var_field!((*exp).stop, Absyn::Exp::RANGE).clone(), localRules, rules, env, info)?
            );
            ()
        }
        Absyn::Exp::TUPLE {
            expressions: __exp_expressions,
        } => {
            assign_variant_field!(exp => Absyn::Exp::TUPLE; expressions = convertExps(__exp_expressions.clone(), localRules, rules, env, info)?);
            ()
        }
        Absyn::Exp::EXPRESSIONCOMMENT { exp: __exp_exp, .. } => {
            assign_variant_field!(exp => Absyn::Exp::EXPRESSIONCOMMENT; exp = convertExp(__exp_exp.clone(), localRules, rules, env, info)?);
            ()
        }
        Absyn::Exp::SUBSCRIPTED_EXP { exp: __exp_exp, .. } => {
            assign_variant_field!(exp => Absyn::Exp::SUBSCRIPTED_EXP;
                exp = convertExp(__exp_exp.clone(), localRules.clone(), rules, env, info)?,
                subscripts = convertSubscripts(var_field!((*exp).subscripts, Absyn::Exp::SUBSCRIPTED_EXP).clone(), localRules, rules, env, info)?
            );
            ()
        }
        _ => (),
    });
    Ok(exp)
}

fn convertCref(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut localRules: RuleTable,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: &Env,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut cref: metamodelica::Ref<Absyn::ComponentRef> = cref;
    cref = (match &*cref {
        Absyn::ComponentRef::WILD { .. } => cref,
        Absyn::ComponentRef::ALLWILD { .. } => cref,
        Absyn::ComponentRef::CREF_FULLYQUALIFIED {
            componentRef: __cref_componentRef,
        } => metamodelica::Ref::new(Absyn::ComponentRef::CREF_FULLYQUALIFIED {
            componentRef: convertCref2(__cref_componentRef.clone(), localRules, rules, env, info)?,
        }),
        _ => convertCref2(cref, localRules, rules, env, info)?,
    });
    Ok(cref)
}

fn convertCref2(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut localRules: RuleTable,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: &Env,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut cref: metamodelica::Ref<Absyn::ComponentRef> = cref;
    let mut path: metamodelica::Ref<Path>;
    let mut cref_rules: metamodelica::List<ConversionRule>;
    let mut rule: ConversionRule;
    let mut has_subs: bool;
    let mut converted: bool;
    has_subs = AbsynUtil::crefHasSubscripts(&cref);
    if has_subs {
        cref = convertCrefSubscripts(cref, localRules.clone(), &rules, env, info)?;
    }
    cref_rules = UnorderedMap::getOrDefault(AbsynUtil::crefFirstIdent(&cref)?, localRules, metamodelica::nil())?;
    if !((cref_rules).is_empty()) {
        rule = (cref_rules).head().cloned()?;
        cref = (match rule.clone() {
            ConversionRule::ELEMENT { .. } => {
                AbsynUtil::crefSetFirstIdent(cref, var_field!(rule.newName, ConversionRule::ELEMENT))
            }
            _ => cref,
        });
        converted = true;
    } else {
        (cref, converted) = convertCrefFromType(cref, rules.clone(), env)?;
    }
    if !(converted) && !(has_subs) {
        path = AbsynUtil::crefToPath(&cref)?;
        path = convertPath(path, rules, &env.imports, info)?;
        cref = AbsynUtil::pathToCref(&path);
    }
    Ok(cref)
}

fn convertCrefFromType(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: &Env,
) -> Result<(metamodelica::Ref<Absyn::ComponentRef>, bool)> {
    let mut cref: metamodelica::Ref<Absyn::ComponentRef> = cref;
    let mut converted: bool = false;
    let mut id: ArcStr;
    let mut first_cref: metamodelica::Ref<Absyn::ComponentRef>;
    let mut rest_cref: metamodelica::Ref<Absyn::ComponentRef>;
    let mut opt_ty: Option<metamodelica::Ref<Path>>;
    let mut cref_rules: metamodelica::List<ConversionRule>;
    if !(AbsynUtil::crefIsQual(&cref)) {
        return Ok((cref, converted));
    }
    id = AbsynUtil::crefFirstIdent(&cref)?;
    opt_ty = UnorderedMap::get(id, env.components.clone())?;
    if (opt_ty).is_some() {
        cref_rules = (lookupRules(&(Util::getOption(opt_ty)?), rules)?).head().cloned()?;
    } else {
        cref_rules = metamodelica::nil();
    }
    if (cref_rules).is_empty() {
        return Ok((cref, converted));
    }
    first_cref = AbsynUtil::crefFirstCref(&cref);
    rest_cref = AbsynUtil::crefStripFirst(&cref)?;
    id = AbsynUtil::crefFirstIdent(&rest_cref)?;
    for mut rule in &*cref_rules {
        let () = (match rule.clone() {
            ConversionRule::ELEMENT { .. }
                if (metamodelica::stringEq(&var_field!(rule.oldName, ConversionRule::ELEMENT), &id)) =>
            {
                rest_cref = AbsynUtil::crefSetFirstIdent(rest_cref, var_field!(rule.newName, ConversionRule::ELEMENT));
                cref = AbsynUtil::joinCrefs(&first_cref, rest_cref)?;
                converted = true;
                return Ok((cref, converted));
                ()
            }
            _ => (),
        });
    }
    Ok((cref, converted))
}

fn convertCrefSubscripts(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut localRules: RuleTable,
    mut rules: &metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: &Env,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut cref: metamodelica::Ref<Absyn::ComponentRef> = cref;
    let () = (match &*cref {
        Absyn::ComponentRef::CREF_QUAL {
            subscripts: __cref_subscripts,
            ..
        } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL;
                subscripts = convertSubscripts(__cref_subscripts.clone(), localRules.clone(), rules, env, info)?,
                componentRef = convertCrefSubscripts(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone(), localRules, rules, env, info)?
            );
            ()
        }
        Absyn::ComponentRef::CREF_IDENT {
            subscripts: __cref_subscripts,
            ..
        } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; subscripts = convertSubscripts(__cref_subscripts.clone(), localRules, rules, env, info)?);
            ()
        }
        _ => (),
    });
    Ok(cref)
}

fn convertSubscripts(
    mut subs: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut localRules: RuleTable,
    mut rules: &metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: &Env,
    mut info: &SourceInfo,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>> {
    let mut subs: metamodelica::List<metamodelica::Ref<Absyn::Subscript>> = subs;
    subs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Subscript>> = metamodelica::nil();
        for mut s in (subs).into_iter().cloned() {
            let __x = convertSubscript(s.clone(), localRules.clone(), rules, env, info)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(subs)
}

fn convertSubscript(
    mut sub: metamodelica::Ref<Absyn::Subscript>,
    mut localRules: RuleTable,
    mut rules: &metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: &Env,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Absyn::Subscript>> {
    let mut sub: metamodelica::Ref<Absyn::Subscript> = sub;
    let () = (match &*sub {
        Absyn::Subscript::SUBSCRIPT {
            subscript: __sub_subscript,
        } => {
            assign_variant_field!(sub => Absyn::Subscript::SUBSCRIPT; subscript = convertExp(__sub_subscript.clone(), localRules, rules, env, info)?);
            ()
        }
        _ => (),
    });
    Ok(sub)
}

fn convertPath(
    mut path: metamodelica::Ref<Path>,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut imports: &ImportTree,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Path>> {
    let mut path: metamodelica::Ref<Path> = path;
    let mut import_path: Option<(metamodelica::Ref<Path>, ArcStr)>;
    (path, import_path) = applyImportsToPath(path, imports)?;
    path = applyRulesPath(path.clone(), &(lookupRules(&path, rules)?), info)?;
    path = stripImportPath(path, import_path)?;
    Ok(path)
}

fn applyRulesPath(
    mut path: metamodelica::Ref<Path>,
    mut rules: &metamodelica::List<metamodelica::List<ConversionRule>>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Path>> {
    let mut path: metamodelica::Ref<Path> = path;
    let mut path_len: i32 = AbsynUtil::pathPartCount(&path, 0);
    let mut found: bool;
    for mut rl in &**rules {
        for mut rule in &*rl.clone() {
            found = (match rule.clone() {
                ConversionRule::CLASS { .. } => {
                    if path_len == metamodelica::arrayLength(var_field!(rule.oldPath, ConversionRule::CLASS).clone()) {
                        path = var_field!(rule.newPath, ConversionRule::CLASS).clone();
                    } else {
                        path = Util::foldcallN(
                            metamodelica::arrayLength(var_field!(rule.oldPath, ConversionRule::CLASS).clone()),
                            &AbsynUtil::pathRest,
                            path,
                        )?;
                        path = AbsynUtil::joinPaths(var_field!(rule.newPath, ConversionRule::CLASS).clone(), path)?;
                    }
                    true
                }
                ConversionRule::ELEMENT { .. }
                    if (path_len
                        > metamodelica::arrayLength(var_field!(rule.oldPath, ConversionRule::ELEMENT).clone())
                        && metamodelica::stringEq(
                            &(AbsynUtil::pathNthIdent(
                                path.clone(),
                                metamodelica::arrayLength(var_field!(rule.oldPath, ConversionRule::ELEMENT).clone())
                                    + 1,
                            )?),
                            &(var_field!(rule.oldName, ConversionRule::ELEMENT).clone()),
                        )) =>
                {
                    if path_len
                        == metamodelica::arrayLength(var_field!(rule.oldPath, ConversionRule::ELEMENT).clone()) - 1
                    {
                        path = AbsynUtil::pathSetLastIdent(&path, var_field!(rule.newName, ConversionRule::ELEMENT));
                    } else {
                        path = AbsynUtil::pathSetNthIdent(
                            &path,
                            var_field!(rule.newName, ConversionRule::ELEMENT),
                            metamodelica::arrayLength(var_field!(rule.oldPath, ConversionRule::ELEMENT).clone()) + 1,
                        )?;
                    }
                    true
                }
                ConversionRule::MESSAGE { .. } => {
                    Error::addSourceMessage(
                        &(Error::CONVERSION_MESSAGE.clone()),
                        list![var_field!(rule.message, ConversionRule::MESSAGE).clone()],
                        info,
                    )?;
                    true
                }
                _ => false,
            });
            if found {
                return Ok(path);
            }
        }
    }
    Ok(path)
}

fn convertFunctionArgs(
    mut args: metamodelica::Ref<Absyn::FunctionArgs>,
    mut localRules: RuleTable,
    mut rules: &metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: &Env,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Absyn::FunctionArgs>> {
    let mut args: metamodelica::Ref<Absyn::FunctionArgs> = args;
    let () = (match &*args {
        Absyn::FunctionArgs::FUNCTIONARGS { args: __args_args, .. } => {
            assign_variant_field!(args => Absyn::FunctionArgs::FUNCTIONARGS;
                        args = convertExps(__args_args.clone(), localRules.clone(), rules, env, info)?,
                        argNames = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>> = metamodelica::nil();
                for mut a in (var_field!((*args).argNames, Absyn::FunctionArgs::FUNCTIONARGS).clone()).into_iter().cloned() {
                    let __x = convertNamedArg(a.clone(), localRules.clone(), rules, env, info)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            ()
        }
        Absyn::FunctionArgs::FOR_ITER_FARG { exp: __args_exp, .. } => {
            assign_variant_field!(args => Absyn::FunctionArgs::FOR_ITER_FARG;
                exp = convertExp(__args_exp.clone(), localRules.clone(), rules, env, info)?,
                iterators = convertForIterators(var_field!((*args).iterators, Absyn::FunctionArgs::FOR_ITER_FARG).clone(), localRules, rules, env, info)?
            );
            ()
        }
    });
    Ok(args)
}

fn convertNamedArg(
    mut arg: metamodelica::Ref<Absyn::NamedArg>,
    mut localRules: RuleTable,
    mut rules: &metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: &Env,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Absyn::NamedArg>> {
    let mut arg: metamodelica::Ref<Absyn::NamedArg> = arg;
    assign_field!(arg.argValue = convertExp(arg.argValue.clone(), localRules, rules, env, info)?);
    Ok(arg)
}

fn convertOption<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut opt: Option<T>,
    mut optFunc: &dyn ::std::ops::Fn(T, metamodelica::Ref<ConversionRules::ConversionRules>, Env, SourceInfo) -> Result<T>,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
    mut info: SourceInfo,
) -> Result<Option<T>> {
    pub type OptFunc<T: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(T, metamodelica::Ref<ConversionRules::ConversionRules>, Env, SourceInfo) -> Result<T>
            + 'static,
    >;

    let mut opt: Option<T> = opt;
    let mut e: T;
    opt = (match opt.clone() {
        Some(mut __esc_e) => {
            e = __esc_e.clone();
            Some(optFunc(e, rules, env, info)?)
        }
        _ => opt,
    });
    Ok(opt)
}

fn getExtendsRules(
    mut parts: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: &Env,
) -> Result<metamodelica::List<metamodelica::Ref<ConversionRules::ConversionRules>>> {
    let mut extendsRules: metamodelica::List<metamodelica::Ref<ConversionRules::ConversionRules>> = metamodelica::nil();
    let mut onode: Option<metamodelica::Ref<ConversionRules::ConversionRules>>;
    for mut ext in &*getExtendsPathsInParts(parts) {
        onode = lookupRuleNode(metamodelica::AsArg::as_arg(&ext), rules.clone())?;
        if (onode).is_some() {
            extendsRules = metamodelica::cons(Util::getOption(onode)?, extendsRules);
        }
    }
    Ok(extendsRules)
}

fn getExtendsPathsInParts(
    mut parts: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> metamodelica::List<metamodelica::Ref<Path>> {
    let mut extendsPaths: metamodelica::List<metamodelica::Ref<Path>> = metamodelica::nil();
    for mut part in &**parts {
        let () = (match &*part.clone() {
            Absyn::ClassPart::PUBLIC {
                contents: __part_contents,
            } => {
                for mut e in &*__part_contents.clone() {
                    extendsPaths = getExtendsPathsInElementItem(metamodelica::AsArg::as_arg(&e), extendsPaths);
                }
                ()
            }
            Absyn::ClassPart::PROTECTED {
                contents: __part_contents,
            } => {
                for mut e in &*__part_contents.clone() {
                    extendsPaths = getExtendsPathsInElementItem(metamodelica::AsArg::as_arg(&e), extendsPaths);
                }
                ()
            }
            _ => (),
        });
    }
    extendsPaths
}

fn getExtendsPathsInElementItem(
    mut element: &metamodelica::Ref<Absyn::ElementItem>,
    mut extendsPaths: metamodelica::List<metamodelica::Ref<Path>>,
) -> metamodelica::List<metamodelica::Ref<Path>> {
    let mut extendsPaths: metamodelica::List<metamodelica::Ref<Path>> = extendsPaths;
    let () = (::match_deref::match_deref! { match element {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::EXTENDS { path: ext_path, .. }, .. } } => {
            extendsPaths = metamodelica::cons(ext_path.clone(), extendsPaths);
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    extendsPaths
}

fn getImportsInParts(
    mut parts: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>> {
    let mut imports: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>> = metamodelica::nil();
    for mut part in &**parts {
        let () = (match &*part.clone() {
            Absyn::ClassPart::PUBLIC {
                contents: __part_contents,
            } => {
                for mut e in &*__part_contents.clone() {
                    imports = getImportsInElementItem(metamodelica::AsArg::as_arg(&e), imports);
                }
                ()
            }
            Absyn::ClassPart::PROTECTED {
                contents: __part_contents,
            } => {
                for mut e in &*__part_contents.clone() {
                    imports = getImportsInElementItem(metamodelica::AsArg::as_arg(&e), imports);
                }
                ()
            }
            _ => (),
        });
    }
    imports
}

fn getImportsInElementItem(
    mut element: &metamodelica::Ref<Absyn::ElementItem>,
    mut imports: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>> {
    let mut imports: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>> = imports;
    let () = (::match_deref::match_deref! { match element {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: imp @ Deref @ Absyn::ElementSpec::IMPORT { .. }, .. } } => {
            imports = metamodelica::cons(imp.clone(), imports);
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    imports
}

fn addImportNamesToEnv(
    mut elements: &metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>>,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut env: Env,
) -> Result<Env> {
    let mut env: Env = env;
    let mut imp: Absyn::Import;
    let mut info: SourceInfo;
    let mut imps: ImportTree;
    if (elements).is_empty() {
        return Ok(env);
    }
    imps = env.imports.clone();
    for mut e in &**elements {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(e.clone()) {
            Deref @ Absyn::ElementSpec::IMPORT { import_: __pa0, info: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        imp = metamodelica::Own::own(__pa0);
        info = metamodelica::Own::own(__pa1);
        imps = addImportName(&imp, rules.clone(), &info, imps)?;
    }
    env.imports = imps;
    Ok(env)
}

fn addImportName(
    mut imp: &Absyn::Import,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut info: &SourceInfo,
    mut imports: ImportTree,
) -> Result<ImportTree> {
    let mut imports: ImportTree = imports;
    let mut name: ArcStr;
    let mut imp_name: ArcStr;
    let mut old_path: metamodelica::Ref<Path>;
    let mut new_path: metamodelica::Ref<Path>;
    let () = (match imp.clone() {
        Absyn::Import::NAMED_IMPORT {
            name: mut __esc_name,
            path: ref __esc_old_path,
        } => {
            name = __esc_name.clone();
            old_path = __esc_old_path.clone();
            new_path = convertPath(old_path.clone(), rules, &(ImportTreeImpl::new()), info)?;
            imports = ImportTreeImpl::add(
                imports,
                &(name.clone()),
                &(ImportData {
                    originalPath: old_path.clone(),
                    convertedPath: new_path,
                    importName: name,
                    shadowed: false,
                }),
                &*(std::sync::Arc::new(fnptr!(ImportTreeImpl::addConflictDefault, _, _, _))
                    as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
            )?;
            ()
        }
        Absyn::Import::QUAL_IMPORT {
            path: ref __esc_old_path,
        } => {
            old_path = __esc_old_path.clone();
            new_path = convertPath(old_path.clone(), rules, &(ImportTreeImpl::new()), info)?;
            name = AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&old_path));
            imp_name = AbsynUtil::pathLastIdent(&new_path);
            imports = ImportTreeImpl::add(
                imports,
                &name,
                &(ImportData {
                    originalPath: old_path.clone(),
                    convertedPath: new_path,
                    importName: imp_name,
                    shadowed: false,
                }),
                &*(std::sync::Arc::new(fnptr!(ImportTreeImpl::addConflictDefault, _, _, _))
                    as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
            )?;
            ()
        }
        Absyn::Import::GROUP_IMPORT {
            prefix: ref __esc_old_path,
            ..
        } => {
            old_path = __esc_old_path.clone();
            for mut group in &*var_field!(imp.groups, Absyn::Import::GROUP_IMPORT).clone() {
                imports = addGroupImportName(
                    metamodelica::AsArg::as_arg(&old_path),
                    metamodelica::AsArg::as_arg(&group),
                    rules.clone(),
                    info,
                    imports,
                )?;
            }
            ()
        }
        _ => (),
    });
    Ok(imports)
}

fn addGroupImportName(
    mut prefix: &metamodelica::Ref<Path>,
    mut imp: &Absyn::GroupImport,
    mut rules: metamodelica::Ref<ConversionRules::ConversionRules>,
    mut info: &SourceInfo,
    mut imports: ImportTree,
) -> Result<ImportTree> {
    let mut imports: ImportTree = imports;
    let mut rename: ArcStr;
    let mut name: ArcStr;
    let mut imp_name: ArcStr;
    let mut old_path: metamodelica::Ref<Path>;
    let mut new_path: metamodelica::Ref<Path>;
    (rename, name) = (match imp.clone() {
        Absyn::GroupImport::GROUP_IMPORT_NAME { name: mut __esc_name } => {
            name = __esc_name.clone();
            (name.clone(), name)
        }
        Absyn::GroupImport::GROUP_IMPORT_RENAME {
            rename: mut __esc_rename,
            name: mut __esc_name,
        } => {
            rename = __esc_rename.clone();
            name = __esc_name.clone();
            (rename, name)
        }
    });
    old_path = AbsynUtil::suffixPath(prefix, &name);
    new_path = convertPath(old_path.clone(), rules, &(ImportTreeImpl::new()), info)?;
    imp_name = (match imp.clone() {
        Absyn::GroupImport::GROUP_IMPORT_NAME { .. } => AbsynUtil::pathLastIdent(&new_path),
        Absyn::GroupImport::GROUP_IMPORT_RENAME { .. } => rename.clone(),
    });
    imports = ImportTreeImpl::add(
        imports,
        &rename,
        &(ImportData {
            originalPath: old_path,
            convertedPath: new_path,
            importName: imp_name,
            shadowed: false,
        }),
        &*(std::sync::Arc::new(fnptr!(ImportTreeImpl::addConflictDefault, _, _, _))
            as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
    )?;
    Ok(imports)
}

fn shadowImportsInParts(
    mut parts: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut imports: ImportTree,
) -> Result<ImportTree> {
    let mut imports: ImportTree = imports;
    for mut part in &**parts {
        let () = (match &*part.clone() {
            Absyn::ClassPart::PUBLIC {
                contents: __part_contents,
            } => {
                for mut e in &*__part_contents.clone() {
                    imports = shadowImportsInElementItem(metamodelica::AsArg::as_arg(&e), imports)?;
                }
                ()
            }
            Absyn::ClassPart::PROTECTED {
                contents: __part_contents,
            } => {
                for mut e in &*__part_contents.clone() {
                    imports = shadowImportsInElementItem(metamodelica::AsArg::as_arg(&e), imports)?;
                }
                ()
            }
            _ => (),
        });
    }
    Ok(imports)
}

fn shadowImportsInElementItem(
    mut element: &metamodelica::Ref<Absyn::ElementItem>,
    mut imports: ImportTree,
) -> Result<ImportTree> {
    let mut imports: ImportTree = imports;
    let () = (::match_deref::match_deref! { match element {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: spec, .. } } => {
            imports = shadowImportsInElementSpec(metamodelica::AsArg::as_arg(&spec), imports)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(imports)
}

fn shadowImportsInElementSpec(
    mut spec: &metamodelica::Ref<Absyn::ElementSpec>,
    mut imports: ImportTree,
) -> Result<ImportTree> {
    let mut imports: ImportTree = imports;
    let () = (::match_deref::match_deref! { match spec {
        Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { name, .. }, .. } => {
            imports = shadowImport(name.clone(), imports)?;
            ()
        },
        Deref @ Absyn::ElementSpec::COMPONENTS { components: __spec_components, .. } => {
            for mut c in &*__spec_components.clone() {
                imports = shadowImport(AbsynUtil::componentName(metamodelica::AsArg::as_arg(&c))?, imports)?;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(imports)
}

fn shadowImport(mut name: ArcStr, mut imports: ImportTree) -> Result<ImportTree> {
    let mut imports: ImportTree = imports;
    let mut imp_data: ImportData;
    if !(ImportTreeImpl::hasKey(imports.clone(), name.clone())?) {
        return Ok(imports);
    }
    imp_data = ImportTreeImpl::get(&imports, name.clone())?;
    imp_data.shadowed = true;
    imports = ImportTreeImpl::update(imports, &name, &imp_data)?;
    Ok(imports)
}

fn applyImportsToPath(
    mut path: metamodelica::Ref<Path>,
    mut imports: &ImportTree,
) -> Result<(metamodelica::Ref<Path>, Option<(metamodelica::Ref<Path>, ArcStr)>)> {
    let mut path: metamodelica::Ref<Path> = path;
    let mut importPath: Option<(metamodelica::Ref<Path>, ArcStr)>;
    let mut imp_data_opt: Option<ImportData>;
    let mut imp_data: ImportData;
    imp_data_opt = (match &*path {
        Absyn::Path::QUALIFIED { name: __path_name, .. } => ImportTreeImpl::getOpt(imports, __path_name.clone()),
        Absyn::Path::IDENT { name: __path_name } => ImportTreeImpl::getOpt(imports, __path_name.clone()),
        _ => None,
    });
    if (imp_data_opt).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(imp_data_opt) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        imp_data = metamodelica::Own::own(__pa0);
        if !(imp_data.shadowed.clone()) {
            importPath = Some((imp_data.convertedPath.clone(), imp_data.importName.clone()));
            path = AbsynUtil::pathReplaceFirst(&path, &imp_data.originalPath)?;
        } else {
            importPath = None;
        }
    } else {
        importPath = None;
    }
    Ok((path, importPath))
}

fn stripImportPath(
    mut path: metamodelica::Ref<Path>,
    mut importPath: Option<(metamodelica::Ref<Path>, ArcStr)>,
) -> Result<metamodelica::Ref<Path>> {
    let mut path: metamodelica::Ref<Path> = path;
    let mut import_path: metamodelica::Ref<Path>;
    let mut import_name: ArcStr;
    let mut imp_len: i32;
    let mut path_len: i32;
    if (importPath).is_none() {
        return Ok(path);
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(importPath) {
        Some((__pa0, __pa1)) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    import_path = metamodelica::Own::own(__pa0);
    import_name = metamodelica::Own::own(__pa1);
    if AbsynUtil::pathPrefixOf(import_path.clone(), path.clone()) {
        imp_len = AbsynUtil::pathPartCount(&import_path, 0);
        path_len = AbsynUtil::pathPartCount(&path, 0);
        if imp_len == path_len {
            path = metamodelica::Ref::new(Path::IDENT { name: import_name });
        } else {
            path = Util::foldcallN(AbsynUtil::pathPartCount(&import_path, 0), &AbsynUtil::pathRest, path)?;
            path = AbsynUtil::prefixPath(import_name, path);
        }
    }
    Ok(path)
}

fn addComponentTypesToEnv(
    mut parts: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut components: TypeTable,
) -> Result<()> {
    UnorderedMap::clear(components.clone());
    for mut part in &**parts {
        let () = (match &*part.clone() {
            Absyn::ClassPart::PUBLIC {
                contents: __part_contents,
            } => {
                for mut e in &*__part_contents.clone() {
                    addComponentTypesToEnv2(metamodelica::AsArg::as_arg(&e), components.clone())?;
                }
                ()
            }
            Absyn::ClassPart::PROTECTED {
                contents: __part_contents,
            } => {
                for mut e in &*__part_contents.clone() {
                    addComponentTypesToEnv2(metamodelica::AsArg::as_arg(&e), components.clone())?;
                }
                ()
            }
            _ => (),
        });
    }
    Ok(())
}

fn addComponentTypesToEnv2(
    mut element: &metamodelica::Ref<Absyn::ElementItem>,
    mut components: TypeTable,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match element {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: comps @ Deref @ Absyn::ElementSpec::COMPONENTS { .. }, .. } } => {
            let mut ty_path: metamodelica::Ref<Path>;
            ty_path = AbsynUtil::typeSpecPath(var_field!((**comps).typeSpec, Absyn::ElementSpec::COMPONENTS));
            for mut c in &*var_field!((**comps).components, Absyn::ElementSpec::COMPONENTS).clone() {
                UnorderedMap::add(AbsynUtil::componentName(metamodelica::AsArg::as_arg(&c))?, ty_path.clone(), components.clone())?;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}
