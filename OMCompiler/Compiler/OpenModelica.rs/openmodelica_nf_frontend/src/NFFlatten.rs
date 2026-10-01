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

use crate::NFAlgorithm as Algorithm;
use crate::NFArrayConnections as ArrayConnections;
use crate::NFAttributes as Attributes;
use crate::NFBackendExtension;
use crate::NFBinding as Binding;
use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFCardinalityTable as CardinalityTable;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFConnectEquations as ConnectEquations;
use crate::NFConnection as Connection;
use crate::NFConnectionSets::ConnectionSets;
use crate::NFConnections as Connections;
use crate::NFConnector as Connector;
use crate::NFConnector::Face;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFEvalConstants as EvalConstants;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpandableConnectors as ExpandableConnectors;
use crate::NFExpression as Expression;
use crate::NFExpressionIterator as ExpressionIterator;
use crate::NFFlatModel as FlatModel;
use crate::NFFunction::Function;
use crate::NFInline as Inline;
use crate::NFInstContext;
use crate::NFInstNode;
use crate::NFInstNode::CachedData;
use crate::NFInstNode::InstNode;
use crate::NFInstNode::InstNodeType;
use crate::NFInstUtil as InstUtil;
use crate::NFModifier::Modifier;
use crate::NFOCConnectionGraph;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::ConnectorType;
use crate::NFPrefixes::Direction;
use crate::NFPrefixes::Parallelism;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes::Visibility;
use crate::NFRangeIterator as RangeIterator;
use crate::NFRestriction as Restriction;
use crate::NFSections as Sections;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSimplifyModel as SimplifyModel;
use crate::NFStatement as Statement;
use crate::NFStreamFlowAlias as StreamFlowAlias;
use crate::NFStructural as Structural;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_ast::Absyn::Path;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::MutableWeak;

pub type FunctionTree = metamodelica::Ref<FunctionTreeImpl::Tree>;

pub type DeletedVariables =
    metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;

pub mod FunctionTreeImpl {
    use super::*;
    pub type Key = metamodelica::Ref<Path>;

    pub type Value = metamodelica::Ref<Function::Function>;

    pub(crate) fn keyStr(mut inKey: Key) -> Result<ArcStr> {
        let mut outString: ArcStr;
        outString = AbsynUtil::pathString(inKey, literal!("."), true, false)?;
        Ok(outString)
    }

    pub(crate) fn valueStr(mut inValue: Value) -> ArcStr {
        let mut outString: ArcStr;
        outString = literal!("");
        outString
    }

    pub(crate) fn keyCompare(mut inKey1: Key, mut inKey2: Key) -> Result<i32> {
        let mut outResult: i32;
        outResult = AbsynUtil::pathCompareNoQual(inKey1, inKey2)?;
        Ok(outResult)
    }

    pub use addConflictKeep as addConflictDefault;

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

    pub type ValueNode = metamodelica::Ref<Path>;

    pub(crate) fn add(
        mut inTree: metamodelica::Ref<Tree>,
        mut inKey: &Key,
        mut inValue: &Value,
        mut conflictFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<Function::Function>,
            metamodelica::Ref<Function::Function>,
            metamodelica::Ref<Path>,
        ) -> Result<metamodelica::Ref<Function::Function>>,
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
                        right: crate::NFFlatten::FunctionTreeImpl::Tree::interned_EMPTY(),
                    });
                } else if key_comp == 1 {
                    outTree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: crate::NFFlatten::FunctionTreeImpl::Tree::interned_EMPTY(),
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

    pub(crate) fn addConflictFail(mut newValue: &Value, mut oldValue: &Value, mut key: &Key) -> Result<Value> {
        let mut value: Value;
        return Err("fail");
        Ok(value)
    }

    pub fn addConflictKeep(mut newValue: Value, mut oldValue: Value, mut key: Key) -> Value {
        let mut value: Value = oldValue;
        value
    }

    pub(crate) fn addConflictReplace(mut newValue: Value, mut oldValue: &Value, mut key: &Key) -> Value {
        let mut value: Value = newValue;
        value
    }

    pub(crate) fn addList(
        mut tree: metamodelica::Ref<Tree>,
        mut inValues: &metamodelica::List<(metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>)>,
        mut conflictFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<Function::Function>,
            metamodelica::Ref<Function::Function>,
            metamodelica::Ref<Path>,
        ) -> Result<metamodelica::Ref<Function::Function>>,
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
            Option<metamodelica::Ref<Function::Function>>,
        ) -> Result<metamodelica::Ref<Function::Function>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        pub type UpdateFn = std::sync::Arc<
            dyn ::std::ops::Fn(Option<metamodelica::Ref<Function::Function>>) -> Result<Value> + 'static,
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
                        right: crate::NFFlatten::FunctionTreeImpl::Tree::interned_EMPTY(),
                    });
                } else if key_comp == 1 {
                    new_tree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: crate::NFFlatten::FunctionTreeImpl::Tree::interned_EMPTY(),
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
        mut inFunc: &'__b dyn ::std::ops::Fn(metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>, FT) -> Result<FT>,
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
        mut foldFunc: &dyn ::std::ops::Fn(metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>, FT) -> Result<(FT, bool)>,
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
        mut foldFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<Path>,
            metamodelica::Ref<Function::Function>,
            FT1,
            FT2,
        ) -> Result<(FT1, FT2)>,
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
        mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>) -> Result<()>,
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

    pub fn fromList(
        mut inValues: &metamodelica::List<(metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>)>,
        mut conflictFunc: &dyn ::std::ops::Fn(
            metamodelica::Ref<Function::Function>,
            metamodelica::Ref<Function::Function>,
            metamodelica::Ref<Path>,
        ) -> Result<metamodelica::Ref<Function::Function>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        let mut tree: metamodelica::Ref<Tree> = crate::NFFlatten::FunctionTreeImpl::Tree::interned_EMPTY();
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

    pub(crate) fn getOpt<'__b>(
        mut tree: &'__b metamodelica::Ref<Tree>,
        mut key: Key,
    ) -> Result<Option<metamodelica::Ref<Function::Function>>> {
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
        mut conflictFunc: &'__b dyn ::std::ops::Fn(
            metamodelica::Ref<Function::Function>,
            metamodelica::Ref<Function::Function>,
            metamodelica::Ref<Path>,
        ) -> Result<metamodelica::Ref<Function::Function>>,
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
        mut lst: metamodelica::List<metamodelica::Ref<Path>>,
    ) -> metamodelica::List<metamodelica::Ref<Path>> {
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
        mut lst: metamodelica::List<metamodelica::Ref<Path>>,
    ) -> metamodelica::List<metamodelica::Ref<Path>> {
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
        mut lst: metamodelica::List<metamodelica::Ref<Function::Function>>,
    ) -> metamodelica::List<metamodelica::Ref<Function::Function>> {
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
            metamodelica::Ref<Path>,
            metamodelica::Ref<Function::Function>,
        ) -> Result<metamodelica::Ref<Function::Function>>,
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
            metamodelica::Ref<Path>,
            metamodelica::Ref<Function::Function>,
            FT,
        ) -> Result<(metamodelica::Ref<Function::Function>, FT)>,
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
        let mut outTree: metamodelica::Ref<Tree> = crate::NFFlatten::FunctionTreeImpl::Tree::interned_EMPTY();
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
                __mm_s.push_str(&*keyStr(__inNode_key.clone())?);
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
                node = setTreeLeftRight(outNode, __outNode_left.clone(), crate::NFFlatten::FunctionTreeImpl::Tree::interned_EMPTY())?;
                setTreeLeftRight(child.clone(), node, crate::NFFlatten::FunctionTreeImpl::Tree::interned_EMPTY())?
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
                node = setTreeLeftRight(outNode, crate::NFFlatten::FunctionTreeImpl::Tree::interned_EMPTY(), __outNode_right.clone())?;
                setTreeLeftRight(child.clone(), crate::NFFlatten::FunctionTreeImpl::Tree::interned_EMPTY(), node)?
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
        mut lst: metamodelica::List<(metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>)>,
    ) -> metamodelica::List<(metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>)> {
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
                Function::Function,
            >,
                                                 __a1: metamodelica::Ref<
                Function::Function,
            >,
                                                 __a2: metamodelica::Ref<Path>|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(addConflictReplace(__a0, &__a1, &__a2))
            })?;
        Ok(outTree)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FlattenSettings {
    pub scalarize: bool,
    pub arrayConnect: bool,
    pub nfAPI: bool,
    pub relaxedErrorChecking: bool,
    pub newBackend: bool,
    pub vectorizeBindings: bool,
    pub implicitStartAttribute: bool,
    pub minimalEval: bool,
}

impl metamodelica::gc::MMTrace for FlattenSettings {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.scalarize, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.arrayConnect, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nfAPI, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.relaxedErrorChecking, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.newBackend, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.vectorizeBindings, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.implicitStartAttribute, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.minimalEval, __mmv)?;
        Ok(())
    }
}
impl Default for FlattenSettings {
    fn default() -> Self {
        Self {
            scalarize: Default::default(),
            arrayConnect: Default::default(),
            nfAPI: Default::default(),
            relaxedErrorChecking: Default::default(),
            newBackend: Default::default(),
            vectorizeBindings: Default::default(),
            implicitStartAttribute: Default::default(),
            minimalEval: Default::default(),
        }
    }
}

pub type SETTINGS = FlattenSettings;

pub mod Prefix {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub(crate) enum Prefix {
        PREFIX {
            root: metamodelica::Ref<InstNode::InstNode>,
            prefix: metamodelica::Ref<ComponentRef::NFComponentRef>,
        },
        INDEXED_PREFIX {
            root: metamodelica::Ref<InstNode::InstNode>,
            prefix: metamodelica::Ref<ComponentRef::NFComponentRef>,
            indexedPrefix: metamodelica::Ref<ComponentRef::NFComponentRef>,
        },
    }
    impl metamodelica::gc::MMTrace for Prefix {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                Prefix::PREFIX { root, prefix } => {
                    metamodelica::gc::MMTrace::mm_accept(root, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(prefix, __mmv)?;
                    Ok(())
                }
                Prefix::INDEXED_PREFIX {
                    root,
                    prefix,
                    indexedPrefix,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(root, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(prefix, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(indexedPrefix, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Default for Prefix {
        fn default() -> Self {
            Self::PREFIX {
                root: Default::default(),
                prefix: Default::default(),
            }
        }
    }
    pub(crate) use self::Prefix::{INDEXED_PREFIX, PREFIX};
    pub(crate) fn new(mut root: metamodelica::Ref<InstNode::InstNode>, mut indexed: bool) -> metamodelica::Ref<Prefix> {
        let mut prefix: metamodelica::Ref<Prefix>;
        prefix = if (indexed) {
            metamodelica::Ref::new(Prefix::INDEXED_PREFIX {
                root: root,
                prefix: crate::NFComponentRef::interned_EMPTY(),
                indexedPrefix: crate::NFComponentRef::interned_EMPTY(),
            })
        } else {
            metamodelica::Ref::new(Prefix::PREFIX {
                root: root,
                prefix: crate::NFComponentRef::interned_EMPTY(),
            })
        };
        prefix
    }

    pub(crate) fn isEmpty(mut prefix: &metamodelica::Ref<Prefix>) -> bool {
        let mut empty: bool;
        empty = (match &**prefix {
            PREFIX {
                prefix: __prefix_prefix,
                ..
            } => ComponentRef::isEmpty(metamodelica::AsArg::as_arg(&__prefix_prefix)),
            INDEXED_PREFIX {
                indexedPrefix: __prefix_indexedPrefix,
                ..
            } => ComponentRef::isEmpty(metamodelica::AsArg::as_arg(&__prefix_indexedPrefix)),
        });
        empty
    }

    pub(crate) fn isIndexed(mut prefix: &metamodelica::Ref<Prefix>) -> bool {
        let mut indexed: bool;
        indexed = (match &**prefix {
            INDEXED_PREFIX { .. } => true,
            _ => false,
        });
        indexed
    }

    pub(crate) fn push(
        mut node: metamodelica::Ref<InstNode::InstNode>,
        mut ty: metamodelica::Ref<Type::NFType>,
        mut dims: &metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
        mut prefix: metamodelica::Ref<Prefix>,
    ) -> Result<metamodelica::Ref<Prefix>> {
        let mut prefix: metamodelica::Ref<Prefix> = prefix;
        let () = (match &*prefix {
            PREFIX {
                prefix: __prefix_prefix,
                ..
            } => {
                assign_variant_field!(prefix => Prefix::PREFIX; prefix = ComponentRef::prefixCref(node, ty, metamodelica::nil(), __prefix_prefix.clone())?);
                ()
            }
            INDEXED_PREFIX {
                prefix: __prefix_prefix,
                ..
            } => {
                assign_variant_field!(prefix => Prefix::INDEXED_PREFIX;
                    prefix = ComponentRef::prefixCref(node.clone(), ty.clone(), metamodelica::nil(), __prefix_prefix.clone())?,
                    indexedPrefix = ComponentRef::prefixCref(node, ty, metamodelica::nil(), var_field!((*prefix).indexedPrefix, Prefix::INDEXED_PREFIX).clone())?
                );
                assign_variant_field!(prefix => Prefix::INDEXED_PREFIX; indexedPrefix = ComponentRef::setSubscripts(makeBindingIterators(var_field!((*prefix).indexedPrefix, Prefix::INDEXED_PREFIX), dims)?, var_field!((*prefix).indexedPrefix, Prefix::INDEXED_PREFIX).clone())?);
                ()
            }
        });
        Ok(prefix)
    }

    pub(crate) fn pop(mut prefix: metamodelica::Ref<Prefix>) -> Result<metamodelica::Ref<Prefix>> {
        let mut prefix: metamodelica::Ref<Prefix> = prefix;
        let () = (match &*prefix {
            PREFIX {
                prefix: __prefix_prefix,
                ..
            } => {
                assign_variant_field!(prefix => Prefix::PREFIX; prefix = ComponentRef::rest(metamodelica::AsArg::as_arg(&__prefix_prefix))?);
                ()
            }
            INDEXED_PREFIX {
                prefix: __prefix_prefix,
                ..
            } => {
                assign_variant_field!(prefix => Prefix::INDEXED_PREFIX;
                    prefix = ComponentRef::rest(metamodelica::AsArg::as_arg(&__prefix_prefix))?,
                    indexedPrefix = ComponentRef::rest(var_field!((*prefix).indexedPrefix, Prefix::INDEXED_PREFIX))?
                );
                ()
            }
        });
        Ok(prefix)
    }

    pub(crate) fn prefix(mut prefix: &metamodelica::Ref<Prefix>) -> metamodelica::Ref<ComponentRef::NFComponentRef> {
        let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
        cref = (match &**prefix {
            PREFIX {
                prefix: __prefix_prefix,
                ..
            } => __prefix_prefix.clone(),
            INDEXED_PREFIX {
                prefix: __prefix_prefix,
                ..
            } => __prefix_prefix.clone(),
        });
        cref
    }

    pub(crate) fn indexedPrefix(
        mut prefix: &metamodelica::Ref<Prefix>,
    ) -> metamodelica::Ref<ComponentRef::NFComponentRef> {
        let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
        cref = (match &**prefix {
            PREFIX {
                prefix: __prefix_prefix,
                ..
            } => __prefix_prefix.clone(),
            INDEXED_PREFIX {
                indexedPrefix: __prefix_indexedPrefix,
                ..
            } => __prefix_indexedPrefix.clone(),
        });
        cref
    }

    pub(crate) fn toNonIndexedPrefix(mut prefix: metamodelica::Ref<Prefix>) -> metamodelica::Ref<Prefix> {
        let mut prefix: metamodelica::Ref<Prefix> = prefix;
        prefix = (match &*prefix {
            PREFIX { .. } => prefix,
            INDEXED_PREFIX {
                prefix: __prefix_prefix,
                root: __prefix_root,
                ..
            } => metamodelica::Ref::new(Prefix::PREFIX {
                root: __prefix_root.clone(),
                prefix: __prefix_prefix.clone(),
            }),
        });
        prefix
    }

    pub(crate) fn apply(
        mut prefix: &metamodelica::Ref<Prefix>,
        mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
        let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
        cref = ComponentRef::transferSubscripts(&(indexedPrefix(prefix)), cref)?;
        Ok(cref)
    }

    pub(crate) fn subscript(
        mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
        mut prefix: metamodelica::Ref<Prefix>,
    ) -> Result<metamodelica::Ref<Prefix>> {
        let mut prefix: metamodelica::Ref<Prefix> = prefix;
        let () = (match &*prefix {
            PREFIX {
                prefix: __prefix_prefix,
                ..
            } => {
                assign_variant_field!(prefix => Prefix::PREFIX; prefix = ComponentRef::setSubscripts(subs, __prefix_prefix.clone())?);
                ()
            }
            INDEXED_PREFIX {
                prefix: __prefix_prefix,
                ..
            } => {
                assign_variant_field!(prefix => Prefix::INDEXED_PREFIX; prefix = ComponentRef::setSubscripts(subs, __prefix_prefix.clone())?);
                ()
            }
        });
        Ok(prefix)
    }

    pub(crate) fn toString(mut pre: &metamodelica::Ref<Prefix>) -> Result<ArcStr> {
        let mut r#str: ArcStr = ComponentRef::toString(&(prefix(pre)))?;
        Ok(r#str)
    }

    pub(crate) fn rootNode(mut pre: &metamodelica::Ref<Prefix>) -> metamodelica::Ref<InstNode::InstNode> {
        let mut node: metamodelica::Ref<InstNode::InstNode>;
        node = (match &**pre {
            PREFIX { root: __pre_root, .. } => __pre_root.clone(),
            INDEXED_PREFIX { root: __pre_root, .. } => __pre_root.clone(),
        });
        node
    }

    pub(crate) fn instanceName(mut pre: &metamodelica::Ref<Prefix>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = NFInstNode::InstNode::name(&(rootNode(pre)))?;
        if !(ComponentRef::isEmpty(&(indexedPrefix(pre)))) {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("."));
                __mm_s.push_str(&*toString(pre)?);
                ArcStr::from(__mm_s)
            };
        }
        Ok(r#str)
    }
}

thread_local! { static __EMPTY_PREFIX_TLS: metamodelica::Ref<Prefix::Prefix> = metamodelica::Ref::new(Prefix::Prefix::PREFIX { root: crate::NFInstNode::InstNode::interned_EMPTY_NODE(), prefix: crate::NFComponentRef::interned_EMPTY() }); }
pub(crate) fn EMPTY_PREFIX() -> metamodelica::Ref<Prefix::Prefix> {
    __EMPTY_PREFIX_TLS.with(|__t| __t.clone())
}

thread_local! { static __EMPTY_INDEXED_PREFIX_TLS: metamodelica::Ref<Prefix::Prefix> = metamodelica::Ref::new(Prefix::Prefix::INDEXED_PREFIX { root: crate::NFInstNode::InstNode::interned_EMPTY_NODE(), prefix: crate::NFComponentRef::interned_EMPTY(), indexedPrefix: crate::NFComponentRef::interned_EMPTY() }); }
pub(crate) fn EMPTY_INDEXED_PREFIX() -> metamodelica::Ref<Prefix::Prefix> {
    __EMPTY_INDEXED_PREFIX_TLS.with(|__t| __t.clone())
}

pub fn flatten(
    mut classInst: metamodelica::Ref<InstNode::InstNode>,
    mut classPath: metamodelica::Ref<Path>,
    mut getConnectionResolved: bool,
) -> Result<metamodelica::Ref<FlatModel::NFFlatModel>> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>;
    let mut sections: metamodelica::Ref<Sections::NFSections>;
    let mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    let mut eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut ieql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut alg: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>;
    let mut ialg: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    let mut settings: FlattenSettings;
    let mut deleted_vars: DeletedVariables;
    let mut prefix: metamodelica::Ref<Prefix::Prefix>;
    settings = FlattenSettings {
        scalarize: Flags::isSet(Flags::NF_SCALARIZE.clone())?,
        arrayConnect: Flags::isSet(Flags::ARRAY_CONNECT.clone())?
            || Flags::getConfigBool(Flags::RESIZABLE_ARRAYS.clone())?,
        nfAPI: Flags::isSet(Flags::NF_API.clone())?,
        relaxedErrorChecking: Flags::isSet(Flags::NF_API.clone())? || Flags::getConfigBool(Flags::CHECK_MODEL.clone())?,
        newBackend: Flags::getConfigBool(Flags::NEW_BACKEND.clone())?,
        vectorizeBindings: Flags::isSet(Flags::VECTORIZE_BINDINGS.clone())?,
        implicitStartAttribute: Flags::isConfigFlagSet(
            Flags::ALLOW_NON_STANDARD_MODELICA.clone(),
            literal!("implicitParameterStartAttribute"),
        )?,
        minimalEval: !metamodelica::stringEq(
            &(Flags::getConfigString(Flags::EVALUATE_STRUCTURAL_PARAMETERS.clone())?),
            &(literal!("all")),
        ),
    };
    prefix = Prefix::new(classInst.clone(), settings.vectorizeBindings.clone());
    sections = crate::NFSections::interned_EMPTY();
    src = ElementSource::createElementSource(
        NFInstNode::InstNode::info(&classInst),
        None,
        &(openmodelica_frontend_types::DAE::Prefix::NOPRE),
        (DAE::emptyCref().clone(), DAE::emptyCref().clone()),
    );
    src = ElementSource::addCommentToSource(
        src,
        SCodeUtil::getElementComment(&(NFInstNode::InstNode::definition(classInst.clone())?)),
    );
    deleted_vars = UnorderedSet::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0))
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                  __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                ComponentRef::isEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >),
        13,
    );
    (vars, sections) = flattenClass(
        &(NFInstNode::InstNode::getClass(classInst)?),
        &prefix,
        Visibility::PUBLIC.clone(),
        None,
        metamodelica::nil(),
        sections,
        deleted_vars.clone(),
        settings,
    )?;
    vars = metamodelica::Dangerous::listReverseInPlace(vars);
    flatModel = (match &*sections {
        Sections::SECTIONS {
            algorithms: __sections_algorithms,
            equations: __sections_equations,
            initialAlgorithms: __sections_initialAlgorithms,
            initialEquations: __sections_initialEquations,
        } => {
            eql = metamodelica::Dangerous::listReverseInPlace(__sections_equations.clone());
            ieql = metamodelica::Dangerous::listReverseInPlace(__sections_initialEquations.clone());
            alg = metamodelica::Dangerous::listReverseInPlace(__sections_algorithms.clone());
            ialg = metamodelica::Dangerous::listReverseInPlace(__sections_initialAlgorithms.clone());
            metamodelica::Ref::new(FlatModel::NFFlatModel {
                name: classPath,
                variables: vars,
                equations: eql,
                initialEquations: ieql,
                algorithms: alg,
                initialAlgorithms: ialg,
                source: src,
            })
        }
        _ => metamodelica::Ref::new(FlatModel::NFFlatModel {
            name: classPath,
            variables: vars,
            equations: metamodelica::nil(),
            initialEquations: metamodelica::nil(),
            algorithms: metamodelica::nil(),
            initialAlgorithms: metamodelica::nil(),
            source: src,
        }),
    });
    assign_field!(
        flatModel.algorithms = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>> = metamodelica::nil();
            for mut al in (flatModel.algorithms.clone()).into_iter().cloned() {
                let __x = Algorithm::setInputsOutputs(al.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        flatModel.initialAlgorithms = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>> = metamodelica::nil();
            for mut al in (flatModel.initialAlgorithms.clone()).into_iter().cloned() {
                let __x = Algorithm::setInputsOutputs(al.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    execStat(&(literal!("NFFlatten.flatten")))?;
    InstUtil::dumpFlatModelDebug(literal!("flatten"), flatModel.clone(), &(FunctionTreeImpl::new()))?;
    if getConnectionResolved {
        if settings.newBackend.clone() {
            assign_field!(flatModel.equations = evaluateIfWithConnects(&flatModel.equations)?);
        }
        if settings.arrayConnect.clone() {
            flatModel = resolveArrayConnections(flatModel)?;
        } else {
            flatModel = resolveConnections(flatModel, deleted_vars.clone(), settings)?;
        }
        InstUtil::dumpFlatModelDebug(literal!("connections"), flatModel.clone(), &(FunctionTreeImpl::new()))?;
    }
    assign_field!(
        flatModel.variables = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
            for mut var in (flatModel.variables.clone()).into_iter().cloned() {
                let __x = updateVariability(var.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    if !(Flags::isConfigFlagSet(
        Flags::ALLOW_NON_STANDARD_MODELICA.clone(),
        literal!("illegalConditionalContext"),
    )?) {
        checkDeletedVarRefs(&flatModel, deleted_vars, settings)?;
    }
    Ok(flatModel)
}

pub(crate) fn flattenConnection(
    mut classInst: metamodelica::Ref<InstNode::InstNode>,
    mut classPath: metamodelica::Ref<Path>,
) -> Result<metamodelica::Ref<Connections::NFConnections>> {
    let mut conns: metamodelica::Ref<Connections::NFConnections>;
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>;
    let mut deleted_vars: DeletedVariables;
    flatModel = flatten(classInst, classPath, false)?;
    deleted_vars = UnorderedSet::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0))
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                  __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                ComponentRef::isEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >),
        13,
    );
    (flatModel, conns) = Connections::collectConnections(
        flatModel,
        &({
            let __pe_b1 = deleted_vars;
            move |__pe_a0| isDeletedCref(__pe_a0, __pe_b1.clone())
        }),
    )?;
    (_, conns) = ExpandableConnectors::elaborate(flatModel.clone(), conns)?;
    conns = Connections::collectFlows(&flatModel, conns)?;
    Ok(conns)
}

pub fn collectFunctions(mut flatModel: &metamodelica::Ref<FlatModel::NFFlatModel>) -> Result<FunctionTree> {
    let mut funcs: FunctionTree;
    funcs = FunctionTreeImpl::new();
    funcs = List::fold(
        &flatModel.variables,
        &move |__a0: metamodelica::Ref<Variable::NFVariable>, __a1: metamodelica::Ref<FunctionTreeImpl::Tree>| {
            collectComponentFuncs(&__a0, __a1)
        },
        funcs,
    )?;
    funcs = List::fold(
        &flatModel.equations,
        &move |__a0: metamodelica::Ref<Equation::NFEquation>, __a1: metamodelica::Ref<FunctionTreeImpl::Tree>| {
            collectEquationFuncs(&__a0, __a1)
        },
        funcs,
    )?;
    funcs = List::fold(
        &flatModel.initialEquations,
        &move |__a0: metamodelica::Ref<Equation::NFEquation>, __a1: metamodelica::Ref<FunctionTreeImpl::Tree>| {
            collectEquationFuncs(&__a0, __a1)
        },
        funcs,
    )?;
    funcs = List::fold(
        &flatModel.algorithms,
        &move |__a0: metamodelica::Ref<Algorithm::NFAlgorithm>, __a1: metamodelica::Ref<FunctionTreeImpl::Tree>| {
            collectAlgorithmFuncs(&__a0, __a1)
        },
        funcs,
    )?;
    funcs = List::fold(
        &flatModel.initialAlgorithms,
        &move |__a0: metamodelica::Ref<Algorithm::NFAlgorithm>, __a1: metamodelica::Ref<FunctionTreeImpl::Tree>| {
            collectAlgorithmFuncs(&__a0, __a1)
        },
        funcs,
    )?;
    execStat(&(literal!("NFFlatten.collectFunctions")))?;
    Ok(funcs)
}

pub(crate) fn fillVectorizedVariableBinding(
    mut var: metamodelica::Ref<Variable::NFVariable>,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = var;
    let mut ty_attrs: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)> = metamodelica::nil();
    let mut attr_name: ArcStr;
    let mut attr_binding: metamodelica::Ref<Binding::NFBinding>;
    assign_field!(var.binding = fillVectorizedBinding(var.binding.clone(), var.ty.clone())?);
    for mut ty_attr in &*var.typeAttributes.clone() {
        (attr_name, attr_binding) = ty_attr.clone();
        attr_binding = fillVectorizedBinding(
            attr_binding.clone(),
            Type::copyDims(var.ty.clone(), Binding::getType(&attr_binding)?),
        )?;
        ty_attrs = metamodelica::cons((attr_name, attr_binding), ty_attrs);
    }
    assign_field!(var.typeAttributes = metamodelica::Dangerous::listReverseInPlace(ty_attrs));
    Ok(var)
}

fn flattenClass(
    mut cls: &metamodelica::Ref<Class::NFClass>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut visibility: Visibility,
    mut binding: Option<metamodelica::Ref<Binding::NFBinding>>,
    mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut sections: metamodelica::Ref<Sections::NFSections>,
    mut deletedVars: DeletedVariables,
    mut settings: FlattenSettings,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::Ref<Sections::NFSections>,
)> {
    let mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = vars;
    let mut sections: metamodelica::Ref<Sections::NFSections> = sections;
    let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    let mut bindings: metamodelica::List<metamodelica::Ref<Binding::NFBinding>> = metamodelica::nil();
    let mut b: metamodelica::Ref<Binding::NFBinding>;
    let () = (::match_deref::match_deref! { match cls {
        Deref @ Class::INSTANCED_CLASS { restriction: Deref @ Restriction::TYPE, .. } => (),
        Deref @ Class::INSTANCED_CLASS { elements: Deref @ ClassTree::FLAT_TREE { components: __esc_comps, .. }, sections: __cls_sections, .. } => {
            comps = (*__esc_comps).clone();
            if (binding).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(binding.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                b = metamodelica::Own::own(__pa0);
                if Binding::isBound(&b) {
                    b = flattenBinding(b, &(Prefix::pop(prefix.clone())?), false)?;
                    bindings = getRecordBindings(&b, comps.clone(), prefix)?;
                }
            }
            if (bindings).is_empty() {
                let __range1 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut c in __range1 {
                    (vars, sections) = flattenComponent(c, prefix.clone(), visibility, binding.clone(), vars, sections, deletedVars.clone(), settings)?;
                }
            } else {
                let __range2 = comps.clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut c in __range2 {
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(bindings) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    b = metamodelica::Own::own(__pa3);
                    bindings = metamodelica::Own::own(__pa4);
                    (vars, sections) = flattenComponent(c, prefix.clone(), visibility, Some(b), vars, sections, deletedVars.clone(), settings)?;
                }
            }
            sections = flattenSections(metamodelica::AsArg::as_arg(&__cls_sections), &(Prefix::toNonIndexedPrefix(prefix.clone())), sections, settings)?;
            ()
        },
        Deref @ Class::TYPED_DERIVED { baseClass: __cls_baseClass, .. } => {
            (vars, sections) = flattenClass(&(NFInstNode::InstNode::getClass(__cls_baseClass.clone())?), prefix, visibility, binding, vars, sections, deletedVars, settings)?;
            ()
        },
        Deref @ Class::INSTANCED_BUILTIN { .. } => (),
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFFlatten.flattenClass")); __mm_s.push_str(&*literal!(" got non-instantiated component ")); __mm_s.push_str(&*Prefix::toString(prefix)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFFlatten.mo")))?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((vars, sections))
}

fn flattenComponent(
    mut component: metamodelica::Ref<InstNode::InstNode>,
    mut prefix: metamodelica::Ref<Prefix::Prefix>,
    mut visibility: Visibility,
    mut outerBinding: Option<metamodelica::Ref<Binding::NFBinding>>,
    mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut sections: metamodelica::Ref<Sections::NFSections>,
    mut deletedVars: DeletedVariables,
    mut settings: FlattenSettings,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::Ref<Sections::NFSections>,
)> {
    let mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = vars;
    let mut sections: metamodelica::Ref<Sections::NFSections> = sections;
    let mut comp_node: metamodelica::Ref<InstNode::InstNode>;
    let mut c: metamodelica::Ref<Component::NFComponent>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut condition: metamodelica::Ref<Binding::NFBinding>;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut vis: Visibility;
    let mut children: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    if NFInstNode::InstNode::isEmpty(&component) || NFInstNode::InstNode::isOnlyOuter(&component)? {
        return Ok((vars, sections));
    }
    comp_node = NFInstNode::InstNode::resolveOuter(component.clone());
    c = NFInstNode::InstNode::component(&comp_node)?;
    let () = (match &*c {
        Component::COMPONENT {
            condition: __esc_condition,
            ty: __esc_ty,
            classInst: __c_classInst,
            ..
        } => {
            condition = (*__esc_condition).clone();
            ty = (*__esc_ty).clone();
            if isDeletedComponent(condition.clone(), &prefix)? {
                deleteComponent(component, &prefix, deletedVars)?;
                return Ok((vars, sections));
            }
            cls = NFInstNode::InstNode::getClass(__c_classInst.clone())?;
            vis = if (NFInstNode::InstNode::isProtected(&component)) {
                Visibility::PROTECTED.clone()
            } else {
                visibility
            };
            (vars, sections) = (match getComponentType(metamodelica::AsArg::as_arg(&ty), settings) {
                ComponentType::COMPLEX { .. } => flattenComplexComponent(
                    comp_node,
                    &c,
                    &cls,
                    ty.clone(),
                    vis,
                    outerBinding,
                    prefix,
                    vars,
                    sections,
                    deletedVars,
                    settings,
                )?,
                ComponentType::NORMAL => flattenSimpleComponent(
                    comp_node,
                    &c,
                    vis,
                    outerBinding,
                    Class::getTypeAttributes(cls),
                    prefix,
                    vars,
                    sections,
                    settings,
                    metamodelica::nil(),
                )?,
                ComponentType::RECORD { .. } => {
                    (children, sections) = flattenComplexComponent(
                        comp_node.clone(),
                        &c,
                        &cls,
                        ty.clone(),
                        vis,
                        outerBinding.clone(),
                        prefix.clone(),
                        metamodelica::nil(),
                        sections,
                        deletedVars,
                        settings,
                    )?;
                    flattenSimpleComponent(
                        comp_node,
                        &c,
                        vis,
                        outerBinding,
                        Class::getTypeAttributes(cls),
                        prefix,
                        vars,
                        sections,
                        settings,
                        children.reverse(),
                    )?
                }
                _ => {
                    Error::terminate(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NFFlatten.flattenComponent"));
                            __mm_s.push_str(&*literal!(" got unknown component"));
                            ArcStr::from(__mm_s)
                        },
                        &(metamodelica::sourceInfo!("NFFrontEnd/NFFlatten.mo")),
                    )?;
                    return Err("fail");
                }
            });
            ()
        }
        _ if (Component::isDeleted(&c)?) => {
            deleteComponent(component, &prefix, deletedVars)?;
            ()
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFFlatten.flattenComponent"));
                    __mm_s.push_str(&*literal!(" got unknown component"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFFlatten.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok((vars, sections))
}

fn isDeletedComponent(
    mut condition: metamodelica::Ref<Binding::NFBinding>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
) -> Result<bool> {
    let mut isDeleted: bool;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut cond: metamodelica::Ref<Binding::NFBinding>;
    if Binding::isBound(&condition) {
        cond = flattenBinding(condition, prefix, false)?;
        exp = Binding::getTypedExp(&cond)?;
        exp = Ceval::evalExp(
            exp,
            &(Ceval::EvalTarget::new(Binding::getInfo(&cond), NFInstContext::CONDITION.clone(), None)),
        )?;
        exp = Expression::expandSplitIndices(exp)?;
        if Expression::arrayAllEqual(exp.clone()) {
            exp = Expression::arrayFirstScalar(exp)?;
        }
        isDeleted = (match &*exp {
            Expression::BOOLEAN { value: __exp_value } => !(__exp_value.clone()),
            _ => {
                Error::addSourceMessage(
                    &(Error::CONDITIONAL_EXP_WITHOUT_VALUE.clone()),
                    list![Expression::toString(exp)?],
                    &(Binding::getInfo(&cond)),
                )?;
                return Err("fail");
            }
        });
    } else {
        isDeleted = false;
    }
    Ok(isDeleted)
}

fn deleteComponent(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut deletedVars: DeletedVariables,
) -> Result<()> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    cref = ComponentRef::prefixCref(
        node,
        crate::NFType::interned_UNKNOWN(),
        metamodelica::nil(),
        Prefix::prefix(prefix),
    )?;
    UnorderedSet::add(cref, deletedVars)?;
    Ok(())
}

fn getComponentType<'__b>(
    mut ty: &'__b metamodelica::Ref<Type::NFType>,
    mut settings: FlattenSettings,
) -> ComponentType {
    '__tco: loop {
        ::match_deref::match_deref! { match ty {
            Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { .. }, .. } => return ComponentType::NORMAL.clone(),
            Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { .. }, .. } if (settings.newBackend.clone()) => return ComponentType::RECORD.clone(),
            Deref @ Type::COMPLEX { .. } => return ComponentType::COMPLEX.clone(),
            Deref @ Type::ARRAY { .. } => { (ty, settings) = (var_field!((**ty).elementType, Type::NFType::ARRAY), settings); continue '__tco; },
            _ => return ComponentType::NORMAL.clone(),
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum ComponentType {
    NORMAL = 1,
    COMPLEX = 2,
    RECORD = 3,
}
impl PartialOrd for ComponentType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ComponentType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for ComponentType {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

fn flattenSimpleComponent(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut comp: &metamodelica::Ref<Component::NFComponent>,
    mut visibility: Visibility,
    mut outerBinding: Option<metamodelica::Ref<Binding::NFBinding>>,
    mut typeAttrs: metamodelica::List<metamodelica::Ref<Modifier::Modifier>>,
    mut prefix: metamodelica::Ref<Prefix::Prefix>,
    mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut sections: metamodelica::Ref<Sections::NFSections>,
    mut settings: FlattenSettings,
    mut children: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::Ref<Sections::NFSections>,
)> {
    let mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = vars;
    let mut sections: metamodelica::Ref<Sections::NFSections> = sections;
    let mut comp_node: metamodelica::Ref<InstNode::InstNode> = node.clone();
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut cmt: metamodelica::Ref<SCode::Comment>;
    let mut info: SourceInfo;
    let mut comp_attr: metamodelica::Ref<Attributes::NFAttributes>;
    let mut eq: metamodelica::Ref<Equation::NFEquation>;
    let mut ty_attrs: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>;
    let mut var: Variability;
    let mut unfix: bool;
    let mut pre: metamodelica::Ref<Prefix::Prefix>;
    let mut v: metamodelica::Ref<Variable::NFVariable>;
    let mut fillVectorizedBindingFails: bool = false;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &((*comp)) {
        Deref @ Component::COMPONENT { ty: __pa0, binding: __pa1, attributes: __pa2, comment: __pa3, info: __pa4, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    binding = metamodelica::Own::own(__pa1);
    comp_attr = metamodelica::Own::own(__pa2);
    cmt = metamodelica::Own::own(__pa3);
    info = metamodelica::Own::own(__pa4);
    checkUnspecifiedEnumType(&ty, &node, &info)?;
    var = comp_attr.variability.clone();
    if (outerBinding).is_some() {
        let __pa5 = ::match_deref::match_deref! { match &(outerBinding) {
            Some(__pa5) => __pa5.clone(),
            _ => return Err("pattern mismatch"),
        } };
        binding = metamodelica::Own::own(__pa5);
        unfix = Binding::isUnbound(&binding) && var == Variability::PARAMETER.clone();
    } else {
        binding = flattenBinding(binding, &prefix, false)?;
        unfix = false;
    }
    if !(settings.scalarize.clone())
        && !(settings.vectorizeBindings.clone())
        && Binding::isBound(&binding)
        && !(Prefix::isEmpty(&prefix))
        && Type::isArray(&(ComponentRef::nodeType(&(Prefix::prefix(&prefix)))?))
    {
        fillVectorizedBindingFails = containsPrefix(Binding::getExp(&binding)?, &prefix)?;
    }
    if !(settings.nfAPI.clone()) && settings.scalarize.clone() || fillVectorizedBindingFails {
        if var >= Variability::DISCRETE.clone()
            && Type::isArray(&ty)
            && !(Type::isExternalObject(&(Type::arrayElementType(&ty))))
            && Binding::isBound(&binding)
            || fillVectorizedBindingFails
        {
            name = ComponentRef::prefixCref(
                comp_node.clone(),
                ty.clone(),
                metamodelica::nil(),
                Prefix::prefix(&prefix),
            )?;
            eq = Equation::makeEquality(
                metamodelica::Ref::new(Expression::NFExpression::CREF {
                    ty: ty.clone(),
                    cref: name.clone(),
                }),
                Binding::getTypedExp(&binding)?,
                ty.clone(),
                ElementSource::createElementSource(
                    info.clone(),
                    None,
                    &(openmodelica_frontend_types::DAE::Prefix::NOPRE),
                    (DAE::emptyCref().clone(), DAE::emptyCref().clone()),
                ),
                crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
                Equation::ScalarizeMode::DONT_SCALARIZE.clone(),
            );
            sections = Sections::prependEquation(eq, sections, false)?;
            binding = Binding::EMPTY_BINDING().clone();
            if comp_attr.direction.clone() == Direction::INPUT.clone() && Prefix::isEmpty(&prefix) {
                assign_field!(comp_attr.direction = Direction::NONE.clone());
                Error::addSourceMessage(
                    &(Error::TOP_LEVEL_INPUT_WITH_BINDING.clone()),
                    list![ComponentRef::toString(&name)?],
                    &info,
                )?;
            }
        }
    }
    ty = flattenType(ty, &prefix, &info)?;
    verifyDimensions(&(Type::arrayDims(ty.clone())), &comp_node)?;
    pre = Prefix::push(comp_node, ty.clone(), &(Type::arrayDims(ty.clone())), prefix.clone())?;
    ty_attrs = ({
        let mut __acc: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)> = metamodelica::nil();
        for mut m in (typeAttrs).into_iter().cloned() {
            let __x = flattenTypeAttribute(&(m.clone()), &prefix)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if unfix {
        ty_attrs = Binding::setAttr(
            ty_attrs,
            &(literal!("fixed")),
            &(Binding::makeFlat(
                metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }),
                Variability::CONSTANT.clone(),
                Binding::Source::GENERATED.clone(),
                Binding::NO_CONFIDENCE.clone(),
            )),
        );
    }
    name = Prefix::prefix(&pre);
    v = metamodelica::Ref::new(Variable::NFVariable {
        name: name,
        ty: ty.clone(),
        binding: binding.clone(),
        visibility: visibility,
        attributes: comp_attr,
        typeAttributes: ty_attrs,
        children: children,
        comment: cmt,
        info: info,
        backendinfo: NFBackendExtension::DUMMY_BACKEND_INFO().clone(),
    });
    if !(settings.relaxedErrorChecking.clone())
        && var < Variability::DISCRETE.clone()
        && !(unfix)
        && !(Type::isComplex(&(Type::arrayElementType(&ty))))
    {
        v = verifyBinding(v, var, &binding, settings)?;
    }
    vars = metamodelica::cons(v, vars);
    Ok((vars, sections))
}

fn checkUnspecifiedEnumType(
    mut ty: &metamodelica::Ref<Type::NFType>,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
    mut info: &SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match ty {
        Deref @ Type::ENUMERATION { literals: Deref @ metamodelica::ListNode::Nil, .. } => {
            Error::addSourceMessage(&(Error::UNSPECIFIED_ENUM_COMPONENT.clone()), list![NFInstNode::InstNode::name(node)?], info)?;
            return Err("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn flattenTypeAttribute(
    mut attr: &metamodelica::Ref<Modifier::Modifier>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
) -> Result<(ArcStr, metamodelica::Ref<Binding::NFBinding>)> {
    let mut outAttr: (ArcStr, metamodelica::Ref<Binding::NFBinding>);
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    binding = flattenBinding(Modifier::binding(attr), prefix, true)?;
    outAttr = (Modifier::name(attr)?, binding);
    Ok(outAttr)
}

fn isTypeAttributeNamed(mut name: &ArcStr, mut attr: &(ArcStr, metamodelica::Ref<Binding::NFBinding>)) -> bool {
    let mut isNamed: bool;
    let mut attr_name: ArcStr;
    (attr_name, _) = attr.clone();
    isNamed = metamodelica::stringEq(&name, &attr_name);
    isNamed
}

fn verifyBinding(
    mut var: metamodelica::Ref<Variable::NFVariable>,
    mut variability: Variability,
    mut binding: &metamodelica::Ref<Binding::NFBinding>,
    mut settings: FlattenSettings,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    fn eval_binding(
        mut binding: &metamodelica::Ref<Binding::NFBinding>,
    ) -> Result<Option<metamodelica::Ref<Expression::NFExpression>>> {
        let mut result: Option<metamodelica::Ref<Expression::NFExpression>>;
        if Binding::isBound(binding) {
            result = Some(Ceval::tryEvalExp(
                Binding::getExp(binding)?,
                &(Ceval::noTarget().clone()),
            ));
        } else {
            result = None;
        }
        Ok(result)
    }

    let mut var: metamodelica::Ref<Variable::NFVariable> = var;
    let mut fixed_binding: metamodelica::Ref<Binding::NFBinding>;
    let mut start_binding: metamodelica::Ref<Binding::NFBinding>;
    let mut fixed_exp_opt: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut fixed_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut start_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut fixed: bool;
    let mut min_exp_opt: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut max_exp_opt: Option<metamodelica::Ref<Expression::NFExpression>>;
    if variability > Variability::CONSTANT.clone() && Binding::isBound(binding) {
        return Ok(var);
    }
    fixed_binding = Variable::lookupTypeAttribute(&(literal!("fixed")), &var);
    fixed_exp_opt = eval_binding(&fixed_binding)?;
    if (fixed_exp_opt).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(fixed_exp_opt) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        fixed_exp = metamodelica::Own::own(__pa0);
        if !(Expression::isBoolean(&fixed_exp)) {
            return Ok(var);
        }
        fixed = Expression::isTrue(&fixed_exp);
    } else {
        fixed = true;
    }
    if variability == Variability::CONSTANT.clone() {
        if !(fixed) {
            Error::addSourceMessage(
                &(Error::NON_FIXED_CONSTANT.clone()),
                list![ComponentRef::toString(&var.name)?],
                &(var.info.clone()),
            )?;
            if !(settings.relaxedErrorChecking.clone()) {
                return Err("fail");
            }
        }
        if Binding::isUnbound(binding) {
            Error::addSourceMessage(
                &(Error::NO_CONSTANT_BINDING.clone()),
                list![ComponentRef::toString(&var.name)?],
                &(var.info.clone()),
            )?;
            return Err("fail");
        }
    } else {
        if fixed && Binding::isUnbound(binding) {
            start_binding = Variable::lookupTypeAttribute(&(literal!("start")), &var);
            if Binding::isUnbound(&start_binding) {
                Error::addSourceMessage(
                    &(Error::UNBOUND_PARAMETER_ERROR.clone()),
                    list![ComponentRef::toString(&var.name)?],
                    &(var.info.clone()),
                )?;
                if settings.implicitStartAttribute.clone() {
                    min_exp_opt = eval_binding(&(Variable::lookupTypeAttribute(&(literal!("min")), &var)))?;
                    max_exp_opt = eval_binding(&(Variable::lookupTypeAttribute(&(literal!("max")), &var)))?;
                    start_exp = Expression::makeDefaultValue(&var.ty, min_exp_opt, max_exp_opt)?;
                    assign_field!(
                        var.binding = Binding::makeFlat(
                            start_exp.clone(),
                            Expression::variability(start_exp)?,
                            Binding::Source::GENERATED.clone(),
                            Binding::NO_CONFIDENCE.clone()
                        )
                    );
                } else if !(settings.relaxedErrorChecking.clone()) {
                    return Err("fail");
                }
            } else {
                Error::addSourceMessage(
                    &(Error::UNBOUND_PARAMETER_WITH_START_VALUE_WARNING.clone()),
                    list![
                        ComponentRef::toString(&var.name)?,
                        Binding::toString(&start_binding, &(literal!("")))?
                    ],
                    &(var.info.clone()),
                )?;
            }
        }
    }
    Ok(var)
}

fn getRecordBindings(
    mut binding: &metamodelica::Ref<Binding::NFBinding>,
    mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
) -> Result<metamodelica::List<metamodelica::Ref<Binding::NFBinding>>> {
    let mut recordBindings: metamodelica::List<metamodelica::Ref<Binding::NFBinding>> = metamodelica::nil();
    let mut binding_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut var: Variability;
    let mut bind_src: Binding::Source;
    let mut confidence: i32;
    binding_exp = Binding::getTypedExp(binding)?;
    var = Binding::variability(binding)?;
    bind_src = Binding::Source::GENERATED.clone();
    confidence = Binding::confidence(binding);
    recordBindings = (match &*binding_exp {
        Expression::RECORD {
            elements: __binding_exp_elements,
            ..
        } => {
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Binding::NFBinding>> = metamodelica::nil();
                for mut e in (__binding_exp_elements.clone()).into_iter().cloned() {
                    let __x = if (Expression::isEmpty(&(e.clone()))) {
                        Binding::EMPTY_BINDING().clone()
                    } else {
                        Binding::makeFlat(e.clone(), var, bind_src, confidence)
                    };
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        }
        Expression::ARRAY { .. }
            if (Type::isRecord(&(Type::arrayElementType(&(Expression::typeOf(binding_exp.clone())))))) =>
        {
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Binding::NFBinding>> = metamodelica::nil();
                for mut i in (1..=metamodelica::arrayLength(comps.clone())).into_iter() {
                    let __x = Binding::makeFlat(
                        Expression::nthRecordElement(i.clone(), &binding_exp)?,
                        var,
                        bind_src,
                        confidence,
                    );
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFFlatten.getRecordBindings"));
                    __mm_s.push_str(&*literal!(" got non-record binding "));
                    __mm_s.push_str(&*Expression::toString(binding_exp.clone())?);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFFlatten.mo")),
            )?;
            return Err("fail");
        }
    });
    Error::assertion(
        ((recordBindings).len() as i32) == metamodelica::arrayLength(comps.clone()),
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("NFFlatten.getRecordBindings"));
            __mm_s.push_str(&*literal!(" got record binding with wrong number of elements for "));
            __mm_s.push_str(&*Prefix::toString(prefix)?);
            ArcStr::from(__mm_s)
        },
        &(metamodelica::sourceInfo!("NFFrontEnd/NFFlatten.mo")),
    )?;
    Ok(recordBindings)
}

fn flattenComplexComponent(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut comp: &metamodelica::Ref<Component::NFComponent>,
    mut cls: &metamodelica::Ref<Class::NFClass>,
    mut nodeTy: metamodelica::Ref<Type::NFType>,
    mut visibility: Visibility,
    mut outerBinding: Option<metamodelica::Ref<Binding::NFBinding>>,
    mut prefix: metamodelica::Ref<Prefix::Prefix>,
    mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut sections: metamodelica::Ref<Sections::NFSections>,
    mut deletedVars: DeletedVariables,
    mut settings: FlattenSettings,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::Ref<Sections::NFSections>,
)> {
    let mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = vars;
    let mut sections: metamodelica::Ref<Sections::NFSections> = sections;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut opt_binding: Option<metamodelica::Ref<Binding::NFBinding>>;
    let mut binding_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut binding_exp_eval: metamodelica::Ref<Expression::NFExpression>;
    let mut eq: metamodelica::Ref<Equation::NFEquation>;
    let mut comp_var: Variability;
    let mut binding_var: Variability;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut pre: metamodelica::Ref<Prefix::Prefix>;
    let mut info: SourceInfo;
    info = NFInstNode::InstNode::info(&node);
    ty = flattenType(nodeTy, &prefix, &info)?;
    dims = Type::arrayDims(ty.clone());
    binding = if ((outerBinding).is_some()) {
        Util::getOption(outerBinding)?
    } else {
        Component::getBinding(comp)
    };
    if Binding::isExplicitlyBound(&binding) {
        binding = flattenBinding(binding, &prefix, false)?;
        binding_exp = Binding::getTypedExp(&binding)?;
        binding_var = Binding::variability(&binding)?;
        comp_var = Component::variability(comp)?;
        if comp_var <= Variability::STRUCTURAL_PARAMETER.clone()
            || binding_var <= Variability::STRUCTURAL_PARAMETER.clone()
        {
            binding_exp = Ceval::evalExp(
                binding_exp,
                &(Ceval::EvalTarget::new(info.clone(), NFInstContext::BINDING.clone(), None)),
            )?;
            binding_exp = flattenExp(binding_exp, &prefix, &(Binding::getInfo(&binding)))?;
        } else if binding_var == Variability::PARAMETER.clone() && Component::isFinal(comp)? {
            if '__try0: {
                binding_exp = unwrap_break_err!(Inline::inlineCallExp(binding_exp.clone(), true), '__try0);
                Ok::<(), &'static str>(())
            }
            .is_err()
            {}
            if !(Expression::isRecord(&binding_exp) || Expression::isCref(&binding_exp)) {
                if '__try1: {
                    binding_exp_eval = Ceval::tryEvalExp(binding_exp.clone(), &(Ceval::noTarget().clone()));
                    binding_exp_eval = unwrap_break_err!(flattenExp(binding_exp_eval.clone(), &prefix, &(Binding::getInfo(&binding))), '__try1);
                    let 0 = (Type::dimensionDiff(ty.clone(), Expression::typeOf(binding_exp_eval.clone()))) else { break '__try1 Err::<_, _>("pattern mismatch") };
                    binding_exp = binding_exp_eval.clone();
                    Ok::<(), &'static str>(())
                }.is_err() {
                }
            }
        } else {
            binding_exp = SimplifyExp::simplify(binding_exp, false)?;
        }
        binding_exp = splitRecordCref(binding_exp)?;
        if !(Expression::isRecordOrRecordArray(&binding_exp)?) {
            if !(settings.newBackend.clone()) {
                name =
                    ComponentRef::prefixCref(node.clone(), ty.clone(), metamodelica::nil(), Prefix::prefix(&prefix))?;
                eq = Equation::makeEquality(
                    metamodelica::Ref::new(Expression::NFExpression::CREF {
                        ty: ty.clone(),
                        cref: name,
                    }),
                    binding_exp,
                    ty.clone(),
                    ElementSource::createElementSource(
                        info.clone(),
                        None,
                        &(openmodelica_frontend_types::DAE::Prefix::NOPRE),
                        (DAE::emptyCref().clone(), DAE::emptyCref().clone()),
                    ),
                    crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
                    Equation::ScalarizeMode::NO_PREFERENCE.clone(),
                );
                sections = Sections::prependEquation(eq, sections, comp_var <= Variability::PARAMETER.clone())?;
            }
            opt_binding = Some(Binding::EMPTY_BINDING().clone());
        } else {
            binding = Binding::setTypedExp(binding_exp, binding)?;
            opt_binding = Some(binding);
        }
    } else {
        opt_binding = None;
    }
    pre = Prefix::push(node.clone(), ty.clone(), &dims, prefix)?;
    if (dims).is_empty() {
        (vars, sections) = flattenClass(
            cls,
            &pre,
            visibility,
            opt_binding,
            vars,
            sections,
            deletedVars,
            settings,
        )?;
    } else if settings.scalarize.clone() {
        dims = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
            for mut d in (dims).into_iter().cloned() {
                let __x = flattenDimension(d.clone(), &pre, &info)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        verifyDimensions(&dims, &node)?;
        (vars, sections) = flattenArray(
            cls,
            &dims,
            pre,
            visibility,
            opt_binding,
            vars,
            sections,
            metamodelica::nil(),
            deletedVars,
            &info,
            settings,
        )?;
    } else {
        (vars, sections) = vectorizeArray(
            cls,
            &ty,
            &dims,
            &pre,
            visibility,
            opt_binding,
            vars,
            sections,
            &(metamodelica::nil()),
            deletedVars,
            settings,
        )?;
    }
    Ok((vars, sections))
}

fn splitRecordCref(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut cls: metamodelica::Ref<InstNode::InstNode>;
    let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    let mut cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut field_cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut fields: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut cls_ty: metamodelica::Ref<Type::NFType>;
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    (outExp, _) = ExpandExp::expand(exp.clone(), false, false)?;
    outExp = (::match_deref::match_deref! { match &(outExp.clone()) {
        Deref @ Expression::CREF { ty: __esc_cls_ty @ Deref @ Type::COMPLEX { .. }, cref: __esc_cr } => {
            cls_ty = (*__esc_cls_ty).clone();
            cr = (*__esc_cr).clone();
            cls = Type::complexNode(metamodelica::AsArg::as_arg(&cls_ty))?;
            comps = ClassTree::getComponents(&(Class::classTree(NFInstNode::InstNode::getClass(cls.clone())?)?))?;
            fields = metamodelica::nil();
            for mut i in ({let __s=metamodelica::arrayLength(comps.clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
                ty = NFInstNode::InstNode::getType(({let __elt = (*metamodelica::index_checked(&comps.borrow(), i)?).clone(); __elt}))?;
                field_cr = ComponentRef::prefixCref(({let __elt = (*metamodelica::index_checked(&comps.borrow(), i)?).clone(); __elt}), ty, metamodelica::nil(), cr.clone())?;
                field_cr = flattenCref(field_cr, &(metamodelica::Ref::new(Prefix::Prefix::PREFIX { root: crate::NFInstNode::InstNode::interned_EMPTY_NODE(), prefix: cr.clone() })), &(Absyn::dummyInfo.clone()))?;
                fields = metamodelica::cons(Expression::fromCref(field_cr, false)?, fields);
            }
            Expression::makeRecord(NFInstNode::InstNode::scopePath(cls, NFInstNode::InstNode::ScopeType::RELATIVE.clone(), false)?, var_field!((*outExp).ty, Expression::NFExpression::CREF).clone(), fields)
        },
        Deref @ Expression::ARRAY { .. } => {
            assign_variant_field!(outExp => Expression::NFExpression::ARRAY; elements = Array::map(var_field!((*outExp).elements, Expression::NFExpression::ARRAY).clone(), &splitRecordCref)?);
            outExp
        },
        Deref @ Expression::IF { condition: __outExp_condition, falseBranch: __outExp_falseBranch, trueBranch: __outExp_trueBranch, .. } if (Expression::variability(__outExp_condition.clone())? <= Variability::PARAMETER.clone()) => {
            cond = Ceval::tryEvalExp(__outExp_condition.clone(), &(Ceval::noTarget().clone()));
            if !(referenceEq(&*(&*cond),&*(__outExp_condition.clone()))) {
                Structural::markExp(metamodelica::AsArg::as_arg(&__outExp_condition))?;
            }
            (match &*cond {
        Expression::BOOLEAN { value: __cond_value } => splitRecordCref(if (__cond_value.clone()) {__outExp_trueBranch.clone()} else {__outExp_falseBranch.clone()})?,
        _ => outExp,
    })
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn flattenArray(
    mut cls: &metamodelica::Ref<Class::NFClass>,
    mut dimensions: &metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
    mut prefix: metamodelica::Ref<Prefix::Prefix>,
    mut visibility: Visibility,
    mut binding: Option<metamodelica::Ref<Binding::NFBinding>>,
    mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut sections: metamodelica::Ref<Sections::NFSections>,
    mut subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut deletedVars: DeletedVariables,
    mut info: &SourceInfo,
    mut settings: FlattenSettings,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::Ref<Sections::NFSections>,
)> {
    let mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = vars;
    let mut sections: metamodelica::Ref<Sections::NFSections> = sections;
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    let mut rest_dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut sub_pre: metamodelica::Ref<Prefix::Prefix>;
    let mut range_iter: metamodelica::Ref<RangeIterator::NFRangeIterator>;
    let mut sub_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    if (dimensions).is_empty() {
        subs = subscripts.reverse();
        sub_pre = Prefix::subscript(subs.clone(), prefix)?;
        (vars, sections) = flattenClass(
            cls,
            &sub_pre,
            visibility,
            subscriptBindingOpt(&subs, binding)?,
            vars,
            sections,
            deletedVars,
            settings,
        )?;
    } else {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*dimensions)) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        dim = metamodelica::Own::own(__pa0);
        rest_dims = metamodelica::Own::own(__pa1);
        dim = flattenDimension(dim, &prefix, info)?;
        range_iter = RangeIterator::fromDim(&dim, false)?;
        while RangeIterator::hasNext(&range_iter)? {
            (range_iter, sub_exp) = RangeIterator::next(range_iter)?;
            (vars, sections) = flattenArray(
                cls,
                &rest_dims,
                prefix.clone(),
                visibility,
                binding.clone(),
                vars,
                sections,
                metamodelica::cons(
                    metamodelica::Ref::new(Subscript::NFSubscript::INDEX { index: sub_exp }),
                    subscripts.clone(),
                ),
                deletedVars.clone(),
                info,
                settings,
            )?;
        }
    }
    Ok((vars, sections))
}

fn vectorizeArray(
    mut cls: &metamodelica::Ref<Class::NFClass>,
    mut cls_ty: &metamodelica::Ref<Type::NFType>,
    mut dimensions: &metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut visibility: Visibility,
    mut binding: Option<metamodelica::Ref<Binding::NFBinding>>,
    mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut sections: metamodelica::Ref<Sections::NFSections>,
    mut subscripts: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut deletedVars: DeletedVariables,
    mut settings: FlattenSettings,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::Ref<Sections::NFSections>,
)> {
    let mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = vars;
    let mut sections: metamodelica::Ref<Sections::NFSections> = sections;
    let mut vrs: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    let mut sects: metamodelica::Ref<Sections::NFSections>;
    let mut eq: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut ieq: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut alg: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>;
    let mut ialg: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>;
    if List::any(dimensions, &move |__a0: metamodelica::Ref<Dimension::NFDimension>| {
        Dimension::isZero(&__a0)
    })? {
        return Ok((vars, sections));
    }
    (vrs, sects) = flattenClass(
        cls,
        prefix,
        visibility,
        binding,
        metamodelica::nil(),
        metamodelica::Ref::new(Sections::NFSections::SECTIONS {
            equations: metamodelica::nil(),
            initialEquations: metamodelica::nil(),
            algorithms: metamodelica::nil(),
            initialAlgorithms: metamodelica::nil(),
        }),
        deletedVars,
        settings,
    )?;
    for mut v in &*vrs.reverse() {
        let mut v = v.clone();
        if !(settings.newBackend.clone() && Type::isRecord(&(Type::arrayElementType(cls_ty)))) {
            assign_field!(v.ty = Type::liftArrayLeftList(v.ty.clone(), dimensions));
        }
        vars = metamodelica::cons(v, vars);
    }
    let () = (match &*sects {
        Sections::SECTIONS {
            algorithms: __sects_algorithms,
            equations: __sects_equations,
            initialAlgorithms: __sects_initialAlgorithms,
            initialEquations: __sects_initialEquations,
        } => {
            eq = vectorizeEquations(
                metamodelica::AsArg::as_arg(&__sects_equations),
                dimensions,
                prefix,
                settings,
            )?;
            ieq = vectorizeEquations(
                metamodelica::AsArg::as_arg(&__sects_initialEquations),
                dimensions,
                prefix,
                settings,
            )?;
            alg = vectorizeAlgorithms(metamodelica::AsArg::as_arg(&__sects_algorithms), dimensions, prefix)?;
            ialg = vectorizeAlgorithms(
                metamodelica::AsArg::as_arg(&__sects_initialAlgorithms),
                dimensions,
                prefix,
            )?;
            sections = Sections::prepend(eq, ieq, alg, ialg, sections);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((vars, sections))
}

fn makeBindingIterators(
    mut prefix: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut dimensions: &metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
) -> Result<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>> {
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
    let mut index: i32 = 0;
    let mut iter: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut name: ArcStr;
    name = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("$"));
        __mm_s.push_str(&*ComponentRef::nodeName(prefix)?);
        ArcStr::from(__mm_s)
    };
    for mut d in &**dimensions {
        index = index + 1;
        iter = ComponentRef::makeIterator(
            NFInstNode::InstNode::newIterator(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*name);
                    __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index)));
                    ArcStr::from(__mm_s)
                },
                crate::NFType::interned_INTEGER(),
                Absyn::dummyInfo.clone(),
            ),
            NFInstNode::InstNode::getType(NFInstNode::InstNode::newIterator(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*name);
                    __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index)));
                    ArcStr::from(__mm_s)
                },
                crate::NFType::interned_INTEGER(),
                Absyn::dummyInfo.clone(),
            ))?,
        )?;
        subs = metamodelica::cons(Subscript::makeIndex(Expression::fromCref(iter, false)?)?, subs);
    }
    subs = metamodelica::Dangerous::listReverseInPlace(subs);
    Ok(subs)
}

fn vectorizeBinding(
    mut binding: metamodelica::Ref<Binding::NFBinding>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
) -> Result<metamodelica::Ref<Binding::NFBinding>> {
    let mut binding: metamodelica::Ref<Binding::NFBinding> = binding;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut nodes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut array_call: metamodelica::Ref<Call::NFCall>;
    let mut binding_ty: metamodelica::Ref<Type::NFType>;
    let mut iters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>;
    let mut prefix_cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut confidence: i32;
    if !(Binding::isBound(&binding)) {
        return Ok(binding);
    }
    prefix_cr = Prefix::indexedPrefix(prefix);
    subs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
        for mut s in (ComponentRef::subscriptsAllFlat(&prefix_cr)?).into_iter().cloned() {
            if !(Subscript::isIterator(&(s.clone()))) {
                continue;
            }
            let __x = s.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if (subs).is_empty() {
        return Ok(binding);
    }
    exp = Binding::getExp(&binding)?;
    binding_ty = Binding::getType(&binding)?;
    let () = (match &*exp {
        Expression::SUBSCRIPTED_EXP {
            exp: __exp_exp,
            subscripts: __exp_subscripts,
            ..
        } if (Subscript::isEqualList(metamodelica::AsArg::as_arg(&__exp_subscripts), subs.clone())?) => {
            binding = Binding::makeFlat(
                __exp_exp.clone(),
                Binding::variability(&binding)?,
                Binding::source(&binding),
                Binding::confidence(&binding),
            );
            return Ok(binding);
            ()
        }
        _ => (),
    });
    nodes = ComponentRef::nodes(&prefix_cr, metamodelica::nil())?;
    dims = List::flatten(
        ({
            let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>> =
                metamodelica::nil();
            for mut n in (nodes).into_iter().cloned() {
                let __x = Type::arrayDims(NFInstNode::InstNode::getType(n.clone())?);
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    )?;
    dims = List::lastN(dims, ((subs).len() as i32))?;
    binding_ty = Type::liftArrayLeftList(Expression::typeOf(exp.clone()), &dims);
    if !((dims).is_empty()) {
        if Expression::isLiteral(&exp)?
            || !(Expression::contains(exp.clone(), &move |__a0: metamodelica::Ref<
                Expression::NFExpression,
            >|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(Expression::isIterator(&__a0))
            })?)
        {
            array_call = Call::makeTypedCall(
                NFBuiltinFuncs::FILL_FUNC().clone(),
                metamodelica::cons(
                    exp,
                    ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> =
                            metamodelica::nil();
                        for mut d in (dims).into_iter().cloned() {
                            let __x = Dimension::sizeExp(&(d.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                ),
                Binding::variability(&binding)?,
                Purity::PURE.clone(),
                binding_ty,
            );
        } else {
            iters = ({
                let mut __acc: metamodelica::List<(
                    metamodelica::Ref<InstNode::InstNode>,
                    metamodelica::Ref<Expression::NFExpression>,
                )> = metamodelica::nil();
                let __thr_src0 = subs;
                let mut __thr_it0 = (&__thr_src0).into_iter();
                let __thr_src1 = dims;
                let mut __thr_it1 = (&__thr_src1).into_iter();
                loop {
                    match (__thr_it0.next(), __thr_it1.next()) {
                        (Some(s), Some(d)) => {
                            let __x = (Subscript::toIterator(&(s.clone()))?, Dimension::toRange(&(d.clone()))?);
                            __acc = cons(__x, __acc);
                        }
                        (None, None) => break,
                        _ => return Err("threaded for: ranges of unequal length"),
                    }
                }
                __acc
            });
            array_call = metamodelica::Ref::new(Call::NFCall::TYPED_ARRAY_CONSTRUCTOR {
                ty: binding_ty,
                var: Expression::variability(exp.clone())?,
                purity: Expression::purity(exp.clone())?,
                exp: exp,
                iters: iters,
            });
        }
        exp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: array_call });
    }
    binding = Binding::makeFlat(
        exp,
        Binding::variability(&binding)?,
        Binding::source(&binding),
        Binding::confidence(&binding),
    );
    Ok(binding)
}

fn fillVectorizedBinding(
    mut binding: metamodelica::Ref<Binding::NFBinding>,
    mut varType: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<Binding::NFBinding>> {
    let mut binding: metamodelica::Ref<Binding::NFBinding> = binding;
    let mut bind_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut bind_ty: metamodelica::Ref<Type::NFType>;
    let mut dim_diff: i32;
    let mut dim_expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let () = (match &*binding {
        Binding::TYPED_BINDING {
            bindingExp: __esc_bind_exp,
            ..
        } => {
            bind_exp = (*__esc_bind_exp).clone();
            bind_ty = (match &*bind_exp.clone() {
                Expression::CREF {
                    cref: __bind_exp_cref, ..
                } => ComponentRef::getSubscriptedType(metamodelica::AsArg::as_arg(&__bind_exp_cref), true)?,
                _ => Expression::typeOf(bind_exp.clone()),
            });
            dim_diff = Type::dimensionDiff(varType.clone(), bind_ty);
            if dim_diff > 0 {
                dim_expl = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> =
                        metamodelica::nil();
                    for mut d in (List::firstN(Type::arrayDims(varType.clone()), dim_diff)?)
                        .into_iter()
                        .cloned()
                    {
                        let __x = Dimension::sizeExp(&(d.clone()))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                assign_variant_field!(binding => Binding::NFBinding::TYPED_BINDING; bindingExp = metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::FILL_FUNC().clone(), metamodelica::cons(var_field!((*binding).bindingExp, Binding::NFBinding::TYPED_BINDING).clone(), dim_expl), var_field!((*binding).variability, Binding::NFBinding::TYPED_BINDING).clone(), Purity::PURE.clone(), varType) }));
                assign_variant_field!(binding => Binding::NFBinding::TYPED_BINDING; bindingType = Expression::typeOf(var_field!((*binding).bindingExp, Binding::NFBinding::TYPED_BINDING).clone()));
            }
            ()
        }
        _ => (),
    });
    Ok(binding)
}

fn vectorizeEquations(
    mut eql: &metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut dimensions: &metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut settings: FlattenSettings,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    for mut eq in &**eql {
        equations = vectorizeEquation(eq.clone(), dimensions, prefix, settings, equations)?;
    }
    equations = metamodelica::Dangerous::listReverseInPlace(equations);
    Ok(equations)
}

fn vectorizeEquation(
    mut eqn: metamodelica::Ref<Equation::NFEquation>,
    mut dimensions: &metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut settings: FlattenSettings,
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = equations;
    let mut eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut lhs: metamodelica::Ref<Expression::NFExpression>;
    let mut rhs: metamodelica::Ref<Expression::NFExpression>;
    eql = flattenEquation(eqn, &(EMPTY_PREFIX().clone()), metamodelica::nil(), settings)?;
    for mut eq in &*eql {
        let mut eq = eq.clone();
        equations = (::match_deref::match_deref! { match &(eq.clone()) {
            Deref @ Equation::EQUALITY { lhs: lhs @ Deref @ Expression::CREF { .. }, rhs: rhs @ Deref @ Expression::CREF { .. }, scalarizeMode: __eq_scalarizeMode, scope: __eq_scope, source: __eq_source, ty: __eq_ty } if (!(Flags::getConfigBool(Flags::NEW_BACKEND.clone())?) || List::all(&(ComponentRef::subscriptsAllWithWholeFlat(var_field!((**lhs).cref, Expression::NFExpression::CREF))?), &move |__a0: metamodelica::Ref<Subscript::NFSubscript>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Subscript::isSimple(&__a0)) })? && List::all(&(ComponentRef::subscriptsAllWithWholeFlat(var_field!((**rhs).cref, Expression::NFExpression::CREF))?), &move |__a0: metamodelica::Ref<Subscript::NFSubscript>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Subscript::isSimple(&__a0)) })?) => {
                let mut lhs = (*lhs).clone();
                let mut rhs = (*rhs).clone();
                ty = Type::liftArrayLeftList(__eq_ty.clone(), dimensions);
                lhs = metamodelica::Ref::new(Expression::NFExpression::CREF { ty: ty.clone(), cref: var_field!((*lhs).cref, Expression::NFExpression::CREF).clone() });
                rhs = metamodelica::Ref::new(Expression::NFExpression::CREF { ty: ty.clone(), cref: var_field!((*rhs).cref, Expression::NFExpression::CREF).clone() });
                metamodelica::cons(metamodelica::Ref::new(Equation::NFEquation::EQUALITY { lhs: lhs.clone(), rhs: rhs.clone(), ty: ty, scope: __eq_scope.clone(), source: __eq_source.clone(), scalarizeMode: __eq_scalarizeMode.clone() }), equations)
            },
            Deref @ Equation::NORETCALL { exp: lhs @ Deref @ Expression::CALL { .. }, .. } if (Call::isConnectionsOperator(var_field!((**lhs).call, Expression::NFExpression::CALL))) => metamodelica::cons(eq, equations),
            _ => {
                eq = vectorizeEquationGeneric(eq, dimensions, prefix)?;
                splitForLoop(&eq, &(EMPTY_PREFIX().clone()), equations, settings)?
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(equations)
}

fn vectorizeEquationGeneric(
    mut eqn: metamodelica::Ref<Equation::NFEquation>,
    mut dimensions: &metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
) -> Result<metamodelica::Ref<Equation::NFEquation>> {
    let mut vectorizedEqn: metamodelica::Ref<Equation::NFEquation>;
    let mut iter: metamodelica::Ref<InstNode::InstNode>;
    let mut iters: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    (iters, ranges, subs) = makeIterators(&(Prefix::prefix(prefix)), dimensions)?;
    subs = metamodelica::Dangerous::listReverseInPlace(subs);
    vectorizedEqn = Equation::mapExp(
        eqn.clone(),
        &({
            let __pe_b1 = prefix.clone();
            let __pe_b2 = subs;
            move |__pe_a0| addIterator(__pe_a0, &__pe_b1, &__pe_b2)
        }),
    )?;
    scope = Equation::scopeCell(&eqn);
    src = Equation::source(&eqn);
    while !((iters).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(iters) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        iter = metamodelica::Own::own(__pa0);
        iters = metamodelica::Own::own(__pa1);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ranges) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        range = metamodelica::Own::own(__pa2);
        ranges = metamodelica::Own::own(__pa3);
        vectorizedEqn = metamodelica::Ref::new(Equation::NFEquation::FOR {
            iterator: iter,
            range: Some(range),
            body: list![vectorizedEqn],
            scope: scope.clone(),
            source: src.clone(),
        });
    }
    Ok(vectorizedEqn)
}

fn vectorizeAlgorithms(
    mut algs: &metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
    mut dimensions: &metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
) -> Result<metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>> {
    let mut algorithms: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>> = metamodelica::nil();
    for mut alg in &**algs {
        algorithms = metamodelica::cons(vectorizeAlgorithm(alg.clone(), dimensions, prefix)?, algorithms);
    }
    algorithms = metamodelica::Dangerous::listReverseInPlace(algorithms);
    Ok(algorithms)
}

fn vectorizeAlgorithm(
    mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>,
    mut dimensions: &metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
) -> Result<metamodelica::Ref<Algorithm::NFAlgorithm>> {
    let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm> = alg;
    assign_field!(alg.statements = flattenStatements(alg.statements.clone(), &(EMPTY_PREFIX().clone()))?);
    alg = (::match_deref::match_deref! { match &(alg.clone()) {
        Deref @ Algorithm::ALGORITHM { statements: Deref @ metamodelica::ListNode::Cons { head: Deref @ Statement::ASSIGNMENT { lhs: Deref @ Expression::CREF { .. }, rhs: Deref @ Expression::CREF { .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            alg
        },
        _ => {
            let mut iter: metamodelica::Ref<InstNode::InstNode>;
            let mut iters: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
            let mut range: metamodelica::Ref<Expression::NFExpression>;
            let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
            let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            (iters, ranges, subs) = makeIterators(&(Prefix::prefix(prefix)), dimensions)?;
            subs = metamodelica::Dangerous::listReverseInPlace(subs);
            body = Statement::mapExpList(alg.statements.clone(), &({ let __pe_b1 = prefix.clone(); let __pe_b2 = subs; move |__pe_a0| addIterator(__pe_a0, &__pe_b1, &__pe_b2) }))?;
            while !((iters).is_empty()) {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(iters) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                iter = metamodelica::Own::own(__pa0);
                iters = metamodelica::Own::own(__pa1);
                let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ranges) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                range = metamodelica::Own::own(__pa2);
                ranges = metamodelica::Own::own(__pa3);
                body = list![metamodelica::Ref::new(Statement::NFStatement::FOR { iterator: iter, range: Some(range), body: body, forType: crate::NFStatement::ForType::NORMAL, source: alg.source.clone(), sub_iters: metamodelica::nil() })];
            }
            metamodelica::Ref::new(Algorithm::NFAlgorithm { statements: body, inputs: alg.inputs.clone(), outputs: alg.outputs.clone(), stmtDiffInfo: None, scope: alg.scope.clone(), source: alg.source.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(alg)
}

pub fn makeIterators(
    mut prefix: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut dimensions: &metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
)> {
    let mut iterators: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = metamodelica::nil();
    let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
    let mut prefix_node: metamodelica::Ref<InstNode::InstNode>;
    let mut iter: metamodelica::Ref<InstNode::InstNode>;
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut sub: metamodelica::Ref<Subscript::NFSubscript>;
    prefix_node = ComponentRef::node(prefix)?;
    for mut dim in &**dimensions {
        iter = NFInstNode::InstNode::newUniqueIterator(
            NFInstNode::InstNode::info(&prefix_node),
            crate::NFType::interned_INTEGER(),
        );
        iterators = metamodelica::cons(iter.clone(), iterators);
        range = Expression::makeRange(
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }),
            None,
            Dimension::sizeExp(metamodelica::AsArg::as_arg(&dim))?,
        )?;
        ranges = metamodelica::cons(range, ranges);
        sub = metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
            index: metamodelica::Ref::new(Expression::NFExpression::CREF {
                ty: crate::NFType::interned_INTEGER(),
                cref: ComponentRef::makeIterator(iter, crate::NFType::interned_INTEGER())?,
            }),
        });
        subscripts = metamodelica::cons(sub, subscripts);
    }
    Ok((iterators, ranges, subscripts))
}

fn addIterator(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut subscripts: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = Expression::map(
        exp,
        (std::sync::Arc::new({
            let __pe_b1 = prefix.clone();
            let __pe_b2 = subscripts.clone();
            move |__pe_a0| addIterator_traverse(__pe_a0, &__pe_b1, &__pe_b2)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(exp)
}

fn addIterator_traverse(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut subscripts: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut r#ref: metamodelica::Ref<ComponentRef::NFComponentRef> = Prefix::prefix(prefix);
    let mut restString: ArcStr;
    let mut prefixString: ArcStr = ComponentRef::toString(&r#ref)?;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { restCref, .. }, .. } => {
            restString = ComponentRef::toString(metamodelica::AsArg::as_arg(&restCref))?;
            if StringUtil::startsWith(restString, prefixString) {
                assign_variant_field!(exp => Expression::NFExpression::CREF; cref = mergeIterator(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), &r#ref, subscripts)?);
            }
            exp
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn mergeIterator(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut r#ref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut subscripts: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    cref = (match &*cref {
        ComponentRef::CREF { .. } => {
            if ComponentRef::isEqual(&cref, r#ref)? {
                assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; subscripts = listAppend(var_field!((*cref).subscripts, ComponentRef::NFComponentRef::CREF).clone(), subscripts.clone()));
            } else {
                assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; restCref = mergeIterator(var_field!((*cref).restCref, ComponentRef::NFComponentRef::CREF).clone(), r#ref, subscripts)?);
            }
            cref
        }
        _ => cref,
    });
    Ok(cref)
}

fn containsPrefix(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
) -> Result<bool> {
    let mut contains: bool;
    contains = Expression::fold(
        exp,
        (std::sync::Arc::new({
            let __pe_b2 = prefix.clone();
            move |__pe_a0, __pe_a1| containsPrefix_traverse(&__pe_a0, __pe_a1, &__pe_b2)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, bool) -> Result<bool> + 'static,
            >),
        false,
    )?;
    Ok(contains)
}

fn containsPrefix_traverse(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut contains: bool,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
) -> Result<bool> {
    let mut contains: bool = contains;
    let mut restString: ArcStr;
    let mut prefixString: ArcStr = ComponentRef::toString(&(Prefix::prefix(prefix)))?;
    let () = (::match_deref::match_deref! { match exp {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { restCref, .. }, .. } => {
            restString = ComponentRef::toString(metamodelica::AsArg::as_arg(&restCref))?;
            if StringUtil::startsWith(restString, prefixString) {
                contains = true;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(contains)
}

fn subscriptBindingOpt(
    mut subscripts: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut binding: Option<metamodelica::Ref<Binding::NFBinding>>,
) -> Result<Option<metamodelica::Ref<Binding::NFBinding>>> {
    let mut binding: Option<metamodelica::Ref<Binding::NFBinding>> = binding;
    let mut b: metamodelica::Ref<Binding::NFBinding>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    if (binding).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(binding.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        b = metamodelica::Own::own(__pa0);
        binding = (match &*b.clone() {
            Binding::TYPED_BINDING {
                bindingExp: __esc_exp,
                bindingType: __esc_ty,
                ..
            } => {
                exp = (*__esc_exp).clone();
                ty = (*__esc_ty).clone();
                assign_variant_field!(b => Binding::NFBinding::TYPED_BINDING;
                    bindingExp = Expression::applySubscripts(subscripts, exp.clone(), false)?,
                    bindingType = Type::arrayElementType(metamodelica::AsArg::as_arg(&ty))
                );
                Some(b)
            }
            Binding::FLAT_BINDING {
                bindingExp: __esc_exp, ..
            } => {
                exp = (*__esc_exp).clone();
                assign_variant_field!(b => Binding::NFBinding::FLAT_BINDING; bindingExp = Expression::applySubscripts(subscripts, exp.clone(), false)?);
                Some(b)
            }
            _ => binding,
        });
    }
    Ok(binding)
}

pub(crate) fn flattenBinding(
    mut binding: metamodelica::Ref<Binding::NFBinding>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut isTypeAttribute: bool,
) -> Result<metamodelica::Ref<Binding::NFBinding>> {
    let mut binding: metamodelica::Ref<Binding::NFBinding> = binding;
    let mut info: SourceInfo;
    binding = (match &*binding {
        Binding::UNBOUND => binding,
        Binding::TYPED_BINDING {
            bindingExp: __binding_bindingExp,
            isFlattened: __binding_isFlattened,
            ..
        } => {
            if __binding_isFlattened.clone() {
                return Ok(binding);
            }
            info = Binding::getInfo(&binding);
            assign_variant_field!(binding => Binding::NFBinding::TYPED_BINDING;
                bindingExp = flattenExp(__binding_bindingExp.clone(), prefix, &info)?,
                bindingType = flattenType(var_field!((*binding).bindingType, Binding::NFBinding::TYPED_BINDING).clone(), prefix, &info)?,
                isFlattened = true
            );
            if (Prefix::isIndexed(prefix)) {
                vectorizeBinding(binding, prefix)?
            } else {
                binding
            }
        }
        Binding::CEVAL_BINDING { .. } => Binding::EMPTY_BINDING().clone(),
        Binding::FLAT_BINDING { .. } => binding,
        Binding::INVALID_BINDING {
            errors: __binding_errors,
            ..
        } => {
            Error::addTotalMessages(metamodelica::AsArg::as_arg(&__binding_errors))?;
            return Err("fail");
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFFlatten.flattenBinding"));
                    __mm_s.push_str(&*literal!(" got untyped binding."));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFFlatten.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(binding)
}

pub(crate) fn flattenExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { .. }, .. } => {
            assign_variant_field!(exp => Expression::NFExpression::CREF; cref = ComponentRef::mapExpShallow(var_field!((*exp).cref, Expression::NFExpression::CREF), &({ let __pe_b1 = prefix.clone(); let __pe_b2 = info.clone(); move |__pe_a0| flattenExp(__pe_a0, &__pe_b1, &__pe_b2) }))?);
            assign_variant_field!(exp => Expression::NFExpression::CREF;
                cref = flattenCref(var_field!((*exp).cref, Expression::NFExpression::CREF).clone(), prefix, info)?,
                ty = flattenType(var_field!((*exp).ty, Expression::NFExpression::CREF).clone(), prefix, info)?
            );
            exp
        },
        Deref @ Expression::SUBSCRIPTED_EXP { split: true, exp: __exp_exp, subscripts: __exp_subscripts, .. } => Expression::mapShallow(replaceSplitIndices(__exp_exp.clone(), __exp_subscripts.clone(), prefix, info)?, (std::sync::Arc::new({ let __pe_b1 = prefix.clone(); let __pe_b2 = info.clone(); move |__pe_a0| flattenExp(__pe_a0, &__pe_b1, &__pe_b2) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?,
        Deref @ Expression::IF { ty: Deref @ Type::CONDITIONAL_ARRAY { .. }, .. } => flattenConditionalArrayIfExp(exp, prefix, info)?,
        Deref @ Expression::INSTANCE_NAME { .. } => metamodelica::Ref::new(Expression::NFExpression::STRING { value: Prefix::instanceName(prefix)? }),
        _ => Expression::mapShallow(exp, (std::sync::Arc::new({ let __pe_b1 = prefix.clone(); let __pe_b2 = info.clone(); move |__pe_a0| flattenExp(__pe_a0, &__pe_b1, &__pe_b2) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp = flattenExpType(exp, prefix, info)?;
    Ok(exp)
}

pub(crate) fn replaceSplitIndices(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = subscripts;
    let mut cr_subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut index: i32;
    let mut cr_node: metamodelica::Ref<InstNode::InstNode>;
    for mut cr in &*ComponentRef::toListReverse(&(Prefix::indexedPrefix(prefix)), true, metamodelica::nil()) {
        cr_subs = ComponentRef::getSubscripts(metamodelica::AsArg::as_arg(&cr));
        if !((cr_subs).is_empty()) {
            index = 1;
            cr_node = ComponentRef::node(metamodelica::AsArg::as_arg(&cr))?;
            for mut s in &*cr_subs {
                (subs, _) = List::replaceOnTrue(
                    s.clone(),
                    subs,
                    &({
                        let __pe_b1 = cr_node.clone();
                        let __pe_b2 = index;
                        move |__pe_a0| replaceSplitIndices2(&__pe_a0, &__pe_b1, __pe_b2.clone())
                    }),
                )?;
                index = index + 1;
            }
        }
    }
    subs = Subscript::expandSplitIndices(subs, &(metamodelica::nil()))?;
    exp = Expression::applySubscripts(&subs, exp, false)?;
    exp = flattenExp(exp, prefix, info)?;
    Ok(exp)
}

pub(crate) fn replaceSplitIndices2(
    mut sub: &metamodelica::Ref<Subscript::NFSubscript>,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
    mut index: i32,
) -> Result<bool> {
    let mut replace: bool;
    replace = (match &**sub {
        Subscript::SPLIT_INDEX {
            dimIndex: __sub_dimIndex,
            node: __sub_node,
        } => {
            __sub_dimIndex.clone() == index
                && NFInstNode::InstNode::refEqual(&(NFInstNode::InstNode::borrow(__sub_node.clone())?), node)?
        }
        _ => false,
    });
    Ok(replace)
}

pub(crate) fn flattenCref(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    cref = Prefix::apply(prefix, cref)?;
    if ComponentRef::hasSplitSubscripts(&cref)? {
        cref = flattenCrefSplitSubscripts(cref, prefix)?;
    }
    cref = ComponentRef::mapTypes(
        &cref,
        &({
            let __pe_b1 = prefix.clone();
            let __pe_b2 = info.clone();
            move |__pe_a0| flattenType(__pe_a0, &__pe_b1, &__pe_b2)
        }),
    )?;
    Ok(cref)
}

pub(crate) fn flattenCrefSplitSubscripts(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    pub(crate) type SubscriptList = metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;

    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    let mut sub_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
        >,
    >;
    sub_map = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<InstNode::InstNode>| NFInstNode::InstNode::hash(&__a0))
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<InstNode::InstNode>, __a1: metamodelica::Ref<InstNode::InstNode>| {
                NFInstNode::InstNode::refEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<InstNode::InstNode>,
                        metamodelica::Ref<InstNode::InstNode>,
                    ) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    for mut cr in &*ComponentRef::toListReverse(&(Prefix::indexedPrefix(prefix)), true, metamodelica::nil()) {
        if ComponentRef::hasSubscripts(metamodelica::AsArg::as_arg(&cr))? {
            UnorderedMap::addUnique(
                ComponentRef::node(metamodelica::AsArg::as_arg(&cr))?,
                ComponentRef::getSubscripts(metamodelica::AsArg::as_arg(&cr)),
                sub_map.clone(),
            )?;
        }
    }
    cref = ComponentRef::mapSubscripts(
        cref,
        &({
            let __pe_b1 = sub_map;
            move |__pe_a0| flattenCrefSplitSubscripts2(__pe_a0, __pe_b1.clone())
        }),
        false,
    )?;
    cref = ComponentRef::simplifySubscripts(cref, true)?;
    Ok(cref)
}

pub(crate) fn flattenCrefSplitSubscripts2(
    mut sub: metamodelica::Ref<Subscript::NFSubscript>,
    mut subMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
        >,
    >,
) -> Result<metamodelica::Ref<Subscript::NFSubscript>> {
    let mut sub: metamodelica::Ref<Subscript::NFSubscript> = sub;
    sub = (match &*sub {
        Subscript::SPLIT_INDEX {
            dimIndex: __sub_dimIndex,
            node: __sub_node,
        } => {
            let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
            subs = UnorderedMap::getOrDefault(
                NFInstNode::InstNode::borrow(__sub_node.clone())?,
                subMap,
                metamodelica::nil(),
            )?;
            if (__sub_dimIndex.clone() > ((subs).len() as i32)) {
                crate::NFSubscript::interned_WHOLE()
            } else {
                (subs).get(__sub_dimIndex.clone())?
            }
        }
        _ => sub,
    });
    Ok(sub)
}

pub(crate) fn flattenConditionalArrayIfExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut tb: metamodelica::Ref<Expression::NFExpression>;
    let mut fb: metamodelica::Ref<Expression::NFExpression>;
    let mut cond_var: Variability;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::IF { ty: __pa0, condition: __pa1, trueBranch: __pa2, falseBranch: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    cond = metamodelica::Own::own(__pa1);
    tb = metamodelica::Own::own(__pa2);
    fb = metamodelica::Own::own(__pa3);
    cond = flattenExp(cond, prefix, info)?;
    cond_var = Expression::variability(cond.clone())?;
    if Type::isConditionalArray(&ty) {
        Structural::markExp(&cond)?;
        cond = Ceval::tryEvalExp(cond, &(Ceval::noTarget().clone()));
        exp = (match &*cond {
            Expression::BOOLEAN { value: __cond_value } => {
                if !(Type::isMatchedBranch(__cond_value.clone(), &ty)?) {
                    (tb, fb) = Util::swap(__cond_value.clone(), fb, tb);
                    Error::addSourceMessage(
                        &(Error::ARRAY_DIMENSION_MISMATCH.clone()),
                        list![
                            Expression::toString(tb.clone())?,
                            Type::toString(&(Expression::typeOf(tb.clone())))?,
                            Dimension::toStringList(Type::arrayDims(Expression::typeOf(fb.clone())), false)?
                        ],
                        info,
                    )?;
                    return Err("fail");
                }
                flattenExp(if (__cond_value.clone()) { tb } else { fb }, prefix, info)?
            }
            _ => {
                Error::addSourceMessage(
                    &(Error::TYPE_MISMATCH_IF_EXP.clone()),
                    list![
                        literal!(""),
                        Expression::toString(tb.clone())?,
                        Type::toString(&(Expression::typeOf(tb)))?,
                        Expression::toString(fb.clone())?,
                        Type::toString(&(Expression::typeOf(fb)))?
                    ],
                    info,
                )?;
                return Err("fail");
            }
        });
    } else if Expression::variability(cond.clone())? == Variability::PARAMETER.clone() {
        Structural::markExp(&cond)?;
        tb = flattenExp(tb, prefix, info)?;
        fb = flattenExp(fb, prefix, info)?;
        ty = flattenType(ty, prefix, info)?;
        exp = metamodelica::Ref::new(Expression::NFExpression::IF {
            ty: ty,
            condition: cond,
            trueBranch: tb,
            falseBranch: fb,
        });
    }
    Ok(exp)
}

pub(crate) fn flattenExpType(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut ty: metamodelica::Ref<Type::NFType>;
    ty = Expression::typeOf(exp.clone());
    if Type::isArray(&ty) {
        ty = flattenType(ty, prefix, info)?;
        exp = Expression::setType(ty, exp)?;
    }
    Ok(exp)
}

pub(crate) fn flattenType(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Type::NFType>> {
    let mut ty: metamodelica::Ref<Type::NFType> = ty;
    ty = Type::mapDims(
        ty,
        &({
            let __pe_b1 = prefix.clone();
            let __pe_b2 = info.clone();
            move |__pe_a0| flattenDimension(__pe_a0, &__pe_b1, &__pe_b2)
        }),
    )?;
    Ok(ty)
}

pub(crate) fn flattenDimension(
    mut dim: metamodelica::Ref<Dimension::NFDimension>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<Dimension::NFDimension>> {
    let mut dim: metamodelica::Ref<Dimension::NFDimension> = dim;
    dim = (match &*dim {
        Dimension::EXP {
            exp: __dim_exp,
            var: __dim_var,
        } => Dimension::fromExp(flattenExp(__dim_exp.clone(), prefix, info)?, __dim_var.clone())?,
        _ => dim,
    });
    Ok(dim)
}

pub(crate) fn flattenSections(
    mut sections: &metamodelica::Ref<Sections::NFSections>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut accumSections: metamodelica::Ref<Sections::NFSections>,
    mut settings: FlattenSettings,
) -> Result<metamodelica::Ref<Sections::NFSections>> {
    let mut accumSections: metamodelica::Ref<Sections::NFSections> = accumSections;
    let () = (match &**sections {
        Sections::SECTIONS {
            algorithms: __sections_algorithms,
            equations: __sections_equations,
            initialAlgorithms: __sections_initialAlgorithms,
            initialEquations: __sections_initialEquations,
        } => {
            let mut eq: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
            let mut ieq: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
            let mut alg: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>;
            let mut ialg: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>;
            eq = flattenEquations(metamodelica::AsArg::as_arg(&__sections_equations), prefix, settings)?;
            ieq = flattenEquations(
                metamodelica::AsArg::as_arg(&__sections_initialEquations),
                prefix,
                settings,
            )?;
            alg = flattenAlgorithms(metamodelica::AsArg::as_arg(&__sections_algorithms), prefix)?;
            ialg = flattenAlgorithms(metamodelica::AsArg::as_arg(&__sections_initialAlgorithms), prefix)?;
            accumSections = Sections::prepend(eq, ieq, alg, ialg, accumSections);
            ()
        }
        _ => (),
    });
    Ok(accumSections)
}

pub(crate) fn flattenEquations(
    mut eql: &metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut settings: FlattenSettings,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    for mut eq in &**eql {
        equations = flattenEquation(eq.clone(), prefix, equations, settings)?;
    }
    Ok(equations)
}

pub(crate) fn flattenEquation(
    mut eq: metamodelica::Ref<Equation::NFEquation>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut settings: FlattenSettings,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = equations;
    let mut info: SourceInfo = Equation::info(&eq);
    equations = (match &*eq {
        Equation::EQUALITY {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            scalarizeMode: __eq_scalarizeMode,
            scope: __eq_scope,
            source: __eq_source,
            ty: __eq_ty,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            let mut ty: metamodelica::Ref<Type::NFType>;
            e1 = flattenExp(__eq_lhs.clone(), prefix, &info)?;
            e2 = flattenExp(__eq_rhs.clone(), prefix, &info)?;
            ty = flattenType(__eq_ty.clone(), prefix, &info)?;
            checkEqualityEquation(&e1, &e2, metamodelica::AsArg::as_arg(&__eq_source))?;
            metamodelica::cons(
                metamodelica::Ref::new(Equation::NFEquation::EQUALITY {
                    lhs: e1,
                    rhs: e2,
                    ty: ty,
                    scope: __eq_scope.clone(),
                    source: __eq_source.clone(),
                    scalarizeMode: __eq_scalarizeMode.clone(),
                }),
                equations,
            )
        }
        Equation::FOR { .. } => {
            let mut eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
            if settings.scalarize.clone() {
                eql = unrollForLoop(&eq, prefix, equations, settings)?;
            } else {
                eql = splitForLoop(&eq, prefix, equations, settings)?;
            }
            eql
        }
        Equation::CONNECT {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            scope: __eq_scope,
            source: __eq_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e1 = flattenExp(__eq_lhs.clone(), prefix, &info)?;
            e2 = flattenExp(__eq_rhs.clone(), prefix, &info)?;
            metamodelica::cons(
                metamodelica::Ref::new(Equation::NFEquation::CONNECT {
                    lhs: e1,
                    rhs: e2,
                    scope: __eq_scope.clone(),
                    source: __eq_source.clone(),
                }),
                equations,
            )
        }
        Equation::IF { .. } => flattenIfEquation(eq, prefix, equations, settings)?,
        Equation::WHEN {
            branches: __eq_branches,
            ..
        } => {
            assign_variant_field!(eq => Equation::NFEquation::WHEN; branches = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>> = metamodelica::nil();
                for mut b in (__eq_branches.clone()).into_iter().cloned() {
                    let __x = flattenEqBranch(b.clone(), prefix, &info, settings)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            metamodelica::cons(eq, equations)
        }
        Equation::ASSERT {
            condition: __eq_condition,
            level: __eq_level,
            message: __eq_message,
            scope: __eq_scope,
            source: __eq_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            let mut e3: metamodelica::Ref<Expression::NFExpression>;
            e1 = flattenExp(__eq_condition.clone(), prefix, &info)?;
            e2 = flattenExp(__eq_message.clone(), prefix, &info)?;
            e3 = flattenExp(__eq_level.clone(), prefix, &info)?;
            metamodelica::cons(
                metamodelica::Ref::new(Equation::NFEquation::ASSERT {
                    condition: e1,
                    message: e2,
                    level: e3,
                    scope: __eq_scope.clone(),
                    source: __eq_source.clone(),
                }),
                equations,
            )
        }
        Equation::TERMINATE {
            message: __eq_message,
            scope: __eq_scope,
            source: __eq_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            e1 = flattenExp(__eq_message.clone(), prefix, &info)?;
            metamodelica::cons(
                metamodelica::Ref::new(Equation::NFEquation::TERMINATE {
                    message: e1,
                    scope: __eq_scope.clone(),
                    source: __eq_source.clone(),
                }),
                equations,
            )
        }
        Equation::REINIT {
            cref: __eq_cref,
            reinitExp: __eq_reinitExp,
            scope: __eq_scope,
            source: __eq_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e1 = flattenExp(__eq_cref.clone(), prefix, &info)?;
            e2 = flattenExp(__eq_reinitExp.clone(), prefix, &info)?;
            metamodelica::cons(
                metamodelica::Ref::new(Equation::NFEquation::REINIT {
                    cref: e1,
                    reinitExp: e2,
                    scope: __eq_scope.clone(),
                    source: __eq_source.clone(),
                }),
                equations,
            )
        }
        Equation::NORETCALL {
            exp: __eq_exp,
            scope: __eq_scope,
            source: __eq_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            e1 = flattenExp(__eq_exp.clone(), prefix, &info)?;
            metamodelica::cons(
                metamodelica::Ref::new(Equation::NFEquation::NORETCALL {
                    exp: e1,
                    scope: __eq_scope.clone(),
                    source: __eq_source.clone(),
                }),
                equations,
            )
        }
        _ => metamodelica::cons(eq, equations),
    });
    Ok(equations)
}

pub(crate) fn checkEqualityEquation(
    mut lhs: &metamodelica::Ref<Expression::NFExpression>,
    mut rhs: &metamodelica::Ref<Expression::NFExpression>,
    mut src: &metamodelica::Ref<DAE::ElementSource>,
) -> Result<()> {
    let mut out0: metamodelica::Ref<Expression::NFExpression>;
    let mut out1: metamodelica::Ref<Expression::NFExpression>;
    let mut pos_vel: metamodelica::Ref<Expression::NFExpression>;
    let mut call: metamodelica::Ref<Call::NFCall>;
    let () = (::match_deref::match_deref! { match (lhs, rhs) {
        (Deref @ Expression::TUPLE { elements: Deref @ metamodelica::ListNode::Cons { head: out0, tail: Deref @ metamodelica::ListNode::Cons { head: out1, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, Deref @ Expression::CALL { call }) if ((Expression::isWildCref(metamodelica::AsArg::as_arg(&out0)) || Expression::isWildCref(metamodelica::AsArg::as_arg(&out1))) && Call::isNamed(metamodelica::AsArg::as_arg(&call), &(literal!("spatialDistribution")))?) => {
            if Expression::isWildCref(metamodelica::AsArg::as_arg(&out1)) {
                Error::addSourceMessage(&(Error::SPATIAL_DISTRIBUTION_IGNORED_OUT1.clone()), metamodelica::nil(), &(ElementSource::getInfo(src.clone())))?;
                return Err("fail");
            }
            let __pa0 = ::match_deref::match_deref! { match &(Call::arguments(metamodelica::AsArg::as_arg(&call))?) {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } } } } } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            pos_vel = metamodelica::Own::own(__pa0);
            Structural::markExp(&pos_vel)?;
            pos_vel = Ceval::tryEvalExp(pos_vel, &(Ceval::noTarget().clone()));
            if !(Expression::isTrue(&pos_vel)) {
                Error::addSourceMessage(&(Error::SPATIAL_DISTRIBUTION_IGNORED_OUT0.clone()), metamodelica::nil(), &(ElementSource::getInfo(src.clone())))?;
                return Err("fail");
            }
            ()
        },
        (Deref @ Expression::TUPLE { .. }, Deref @ Expression::CALL { call }) if (Call::isNamed(metamodelica::AsArg::as_arg(&call), &(literal!("noEvent")))?) => {
            checkEqualityEquation(lhs, &(((Call::arguments(metamodelica::AsArg::as_arg(&call))?)).head().cloned()?), src)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn flattenIfEquation(
    mut eq: metamodelica::Ref<Equation::NFEquation>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut settings: FlattenSettings,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = equations;
    let mut branch: metamodelica::Ref<Equation::Branch::Branch>;
    let mut branches: metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>>;
    let mut bl: metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>> = metamodelica::nil();
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut var: Variability;
    let mut has_connect: bool;
    let mut should_eval: bool = false;
    let mut structural: bool = true;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    let mut info: SourceInfo;
    let mut target: metamodelica::Ref<Ceval::EvalTarget::EvalTarget>;
    let mut scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::IF { branches: __pa0, scope: __pa1, source: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    branches = metamodelica::Own::own(__pa0);
    scope = metamodelica::Own::own(__pa1);
    src = metamodelica::Own::own(__pa2);
    has_connect = Equation::contains(
        eq.clone(),
        &move |__a0: metamodelica::Ref<Equation::NFEquation>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(Equation::isConnection(&__a0))
        },
    )?;
    info = Equation::info(&eq);
    target = if (has_connect) {
        Ceval::EvalTarget::new(info.clone(), NFInstContext::NO_CONTEXT.clone(), None)
    } else {
        Ceval::noTarget().clone()
    };
    while !((branches).is_empty()) {
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(branches) {
            Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
            _ => return Err("pattern mismatch"),
        } };
        branch = metamodelica::Own::own(__pa3);
        branches = metamodelica::Own::own(__pa4);
        bl = (::match_deref::match_deref! { match &(branch.clone()) {
            Deref @ Equation::Branch::BRANCH { condition: __esc_cond, conditionVar: __esc_var, body: __esc_eql } => {
                cond = (*__esc_cond).clone();
                var = (*__esc_var).clone();
                eql = (*__esc_eql).clone();
                cond = flattenExp(cond.clone(), prefix, &info)?;
                if var.clone() <= Variability::STRUCTURAL_PARAMETER.clone() {
                    if Expression::isPure(metamodelica::AsArg::as_arg(&cond))? {
                        if has_connect {
                            should_eval = !(settings.newBackend.clone());
                            structural = true;
                        } else if settings.minimalEval.clone() {
                            should_eval = false;
                            structural = false;
                        } else if settings.scalarize.clone() {
                            should_eval = true;
                        } else if settings.newBackend.clone() || Expression::contains(cond.clone(), &move |__a0: metamodelica::Ref<Expression::NFExpression>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Expression::isIterator(&__a0)) })? {
                            should_eval = false;
                            structural = settings.newBackend.clone();
                        } else {
                            should_eval = true;
                        }
                        if structural || should_eval {
                            Structural::markExp(metamodelica::AsArg::as_arg(&cond))?;
                        }
                        if should_eval {
                            cond = Ceval::tryEvalExp(cond.clone(), &target);
                            cond = flattenExp(cond.clone(), prefix, &info)?;
                        }
                    }
                    if !(Expression::isBoolean(metamodelica::AsArg::as_arg(&cond))) && has_connect && !(settings.newBackend.clone()) {
                        Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to evaluate branch condition in if equation containing connect equations: `")); __mm_s.push_str(&*Expression::toString(cond.clone())?); __mm_s.push_str(&*literal!("`")); ArcStr::from(__mm_s) }, info.clone())?;
                        return Err("fail");
                    }
                }
                if Expression::isTrue(metamodelica::AsArg::as_arg(&cond)) {
                    branches = metamodelica::nil();
                    eql = flattenEquations(metamodelica::AsArg::as_arg(&eql), prefix, settings)?;
                    if (bl).is_empty() {
                        equations = listAppend(eql.clone(), equations);
                    } else {
                        bl = metamodelica::cons(Equation::makeBranch(cond.clone(), metamodelica::Dangerous::listReverseInPlace(eql.clone()), var.clone()), bl);
                    }
                } else if !(Expression::isFalse(metamodelica::AsArg::as_arg(&cond))) {
                    eql = flattenEquations(metamodelica::AsArg::as_arg(&eql), prefix, settings)?;
                    bl = metamodelica::cons(Equation::makeBranch(cond.clone(), metamodelica::Dangerous::listReverseInPlace(eql.clone()), var.clone()), bl);
                }
                bl
            },
            Deref @ Equation::Branch::INVALID_BRANCH { branch: Deref @ Equation::Branch::BRANCH { condition: __esc_cond, conditionVar: __esc_var, .. }, .. } if (has_connect) => {
                cond = (*__esc_cond).clone();
                var = (*__esc_var).clone();
                if var.clone() <= Variability::STRUCTURAL_PARAMETER.clone() {
                    Structural::markExp(metamodelica::AsArg::as_arg(&cond))?;
                    cond = Ceval::evalExp(cond.clone(), &target)?;
                    cond = flattenExp(cond.clone(), prefix, &info)?;
                }
                if !(Expression::isFalse(metamodelica::AsArg::as_arg(&cond))) {
                    Equation::Branch::triggerErrors(&branch)?;
                }
                bl
            },
            _ => metamodelica::cons(branch, bl),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    if !((bl).is_empty()) {
        equations = metamodelica::cons(
            metamodelica::Ref::new(Equation::NFEquation::IF {
                branches: metamodelica::Dangerous::listReverseInPlace(bl),
                scope: scope,
                source: src,
            }),
            equations,
        );
    }
    Ok(equations)
}

pub(crate) fn flattenEqBranch(
    mut branch: metamodelica::Ref<Equation::Branch::Branch>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut info: &SourceInfo,
    mut settings: FlattenSettings,
) -> Result<metamodelica::Ref<Equation::Branch::Branch>> {
    let mut branch: metamodelica::Ref<Equation::Branch::Branch> = branch;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut var: Variability;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(branch) {
        Deref @ Equation::Branch::BRANCH { condition: __pa0, conditionVar: __pa1, body: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp = metamodelica::Own::own(__pa0);
    var = metamodelica::Own::own(__pa1);
    eql = metamodelica::Own::own(__pa2);
    exp = flattenExp(exp, prefix, info)?;
    eql = flattenEquations(&eql, prefix, settings)?;
    branch = Equation::makeBranch(exp, metamodelica::Dangerous::listReverseInPlace(eql), var);
    Ok(branch)
}

pub(crate) fn unrollForLoop(
    mut forLoop: &metamodelica::Ref<Equation::NFEquation>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut settings: FlattenSettings,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = equations;
    let mut iter: metamodelica::Ref<InstNode::InstNode>;
    let mut body: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut unrolled_body: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut range_iter: metamodelica::Ref<RangeIterator::NFRangeIterator>;
    let mut val: metamodelica::Ref<Expression::NFExpression>;
    let mut info: SourceInfo;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*forLoop)) {
        Deref @ Equation::FOR { iterator: __pa0, range: Some(__pa1), body: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    iter = metamodelica::Own::own(__pa0);
    range = metamodelica::Own::own(__pa1);
    body = metamodelica::Own::own(__pa2);
    info = Equation::info(forLoop);
    range = flattenExp(range, prefix, &info)?;
    Structural::markExp(&range)?;
    range = Ceval::evalExp(
        range,
        &(Ceval::EvalTarget::new(info, NFInstContext::ITERATION_RANGE.clone(), None)),
    )?;
    range_iter = RangeIterator::fromExp(range)?;
    while RangeIterator::hasNext(&range_iter)? {
        (range_iter, val) = RangeIterator::next(range_iter)?;
        unrolled_body = Equation::replaceIteratorList(body.clone(), &iter, &val)?;
        unrolled_body = flattenEquations(&unrolled_body, prefix, settings)?;
        equations = listAppend(unrolled_body, equations);
    }
    Ok(equations)
}

pub(crate) fn splitForLoop(
    mut forLoop: &metamodelica::Ref<Equation::NFEquation>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut settings: FlattenSettings,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = equations;
    let mut iter: metamodelica::Ref<InstNode::InstNode>;
    let mut opt_range: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut body: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut connects: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut non_connects: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    let mut eq: metamodelica::Ref<Equation::NFEquation>;
    let mut scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &((*forLoop)) {
        Deref @ Equation::FOR { iterator: __pa0, range: __pa1, body: __pa2, scope: __pa3, source: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    iter = metamodelica::Own::own(__pa0);
    opt_range = metamodelica::Own::own(__pa1);
    body = metamodelica::Own::own(__pa2);
    scope = metamodelica::Own::own(__pa3);
    src = metamodelica::Own::own(__pa4);
    body = flattenEquations(&body, &(EMPTY_PREFIX().clone()), settings)?;
    (connects, non_connects) = splitForLoop2(&body, settings)?;
    if !((connects).is_empty()) {
        if (opt_range).is_some() {
            let __pa5 = ::match_deref::match_deref! { match &(opt_range) {
                Some(__pa5) => __pa5.clone(),
                _ => return Err("pattern mismatch"),
            } };
            range = metamodelica::Own::own(__pa5);
            range = Ceval::evalExp(
                range,
                &(Ceval::EvalTarget::new(Equation::info(forLoop), NFInstContext::ITERATION_RANGE.clone(), None)),
            )?;
            Structural::markExp(&range)?;
            opt_range = Some(range);
        }
        eq = metamodelica::Ref::new(Equation::NFEquation::FOR {
            iterator: iter.clone(),
            range: opt_range.clone(),
            body: connects,
            scope: scope.clone(),
            source: src.clone(),
        });
        if settings.arrayConnect.clone() {
            equations = metamodelica::cons(eq, equations);
        } else {
            equations = unrollForLoop(&eq, prefix, equations, settings)?;
        }
    }
    if !((non_connects).is_empty()) {
        equations = metamodelica::cons(
            metamodelica::Ref::new(Equation::NFEquation::FOR {
                iterator: iter,
                range: opt_range,
                body: non_connects,
                scope: scope,
                source: src,
            }),
            equations,
        );
    }
    Ok(equations)
}

pub(crate) fn splitForLoop2(
    mut forBody: &metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut settings: FlattenSettings,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
)> {
    let mut connects: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    let mut nonConnects: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    let mut conns: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut nconns: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    for mut eq in &**forBody {
        let () = (match &*eq.clone() {
            Equation::CONNECT { .. } => {
                connects = metamodelica::cons(eq.clone(), connects);
                ()
            }
            Equation::FOR {
                body: __eq_body,
                iterator: __eq_iterator,
                range: __eq_range,
                scope: __eq_scope,
                source: __eq_source,
            } => {
                (conns, nconns) = splitForLoop2(metamodelica::AsArg::as_arg(&__eq_body), settings)?;
                if !((conns).is_empty()) {
                    connects = metamodelica::cons(
                        metamodelica::Ref::new(Equation::NFEquation::FOR {
                            iterator: __eq_iterator.clone(),
                            range: __eq_range.clone(),
                            body: conns,
                            scope: __eq_scope.clone(),
                            source: __eq_source.clone(),
                        }),
                        connects,
                    );
                }
                if !((nconns).is_empty()) {
                    nonConnects = metamodelica::cons(
                        metamodelica::Ref::new(Equation::NFEquation::FOR {
                            iterator: __eq_iterator.clone(),
                            range: __eq_range.clone(),
                            body: nconns,
                            scope: __eq_scope.clone(),
                            source: __eq_source.clone(),
                        }),
                        nonConnects,
                    );
                }
                ()
            }
            _ => {
                if Equation::contains(
                    eq.clone(),
                    &move |__a0: metamodelica::Ref<Equation::NFEquation>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(Equation::isConnect(&__a0))
                    },
                )? || Equation::containsExp(
                    metamodelica::AsArg::as_arg(&eq),
                    &({
                        let __pe_b1: Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static,
                        > = (std::sync::Arc::new(move |__a0: metamodelica::Ref<Expression::NFExpression>| {
                            Expression::isConnectionCall(&__a0)
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool>
                                    + 'static,
                            >);
                        move |__pe_a0| Expression::contains(__pe_a0, &*__pe_b1)
                    }),
                )? {
                    connects = metamodelica::cons(eq.clone(), connects);
                } else {
                    nonConnects = metamodelica::cons(eq.clone(), nonConnects);
                }
                ()
            }
        });
    }
    Ok((connects, nonConnects))
}

pub(crate) fn unrollForStatementsInAlg(
    mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>,
) -> Result<metamodelica::Ref<Algorithm::NFAlgorithm>> {
    let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm> = alg;
    assign_field!(alg.statements = unrollForStatements(&alg.statements)?);
    Ok(alg)
}

pub(crate) fn unrollForStatements(
    mut stmts: &metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
    let mut outStmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
    for mut s in &**stmts {
        outStmts = unrollForStatement(s.clone(), outStmts)?;
    }
    outStmts = metamodelica::Dangerous::listReverseInPlace(outStmts);
    Ok(outStmts)
}

pub(crate) fn unrollForStatement(
    mut stmt: metamodelica::Ref<Statement::NFStatement>,
    mut statements: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
    let mut statements: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = statements;
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut val: metamodelica::Ref<Expression::NFExpression>;
    let mut info: SourceInfo;
    let mut range_iter: metamodelica::Ref<RangeIterator::NFRangeIterator>;
    let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    let mut has_for: bool;
    statements = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::FOR { range: Some(__esc_range), body: __stmt_body, iterator: __stmt_iterator, .. } => {
            range = (*__esc_range).clone();
            info = Statement::info(&stmt);
            match '__try0: {
                range = unwrap_break_err!(Ceval::evalExp(range.clone(), &(Ceval::EvalTarget::new(info.clone(), NFInstContext::ITERATION_RANGE.clone(), None))), '__try0);
                range_iter = unwrap_break_err!(RangeIterator::fromExp(range.clone()), '__try0);
                Ok::<_, &'static str>((range.clone(), range_iter.clone()))
            } {
                Ok((__try0_o0, __try0_o1)) => {
                    range = __try0_o0;
                    range_iter = __try0_o1;
                }
                Err(__try0_err) => {
                    Error::addSourceMessage(&(Error::UNROLL_FAILURE.clone()), list![Statement::toString(&stmt, literal!(""))?], &info)?;
                    return Err(__try0_err);
                }
            }
            has_for = Statement::containsList(metamodelica::AsArg::as_arg(&__stmt_body), &move |__a0: metamodelica::Ref<Statement::NFStatement>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Statement::isFor(&__a0)) })?;
            while RangeIterator::hasNext(&range_iter)? {
                (range_iter, val) = RangeIterator::next(range_iter)?;
                stmts = Statement::replaceIteratorList(__stmt_body.clone(), metamodelica::AsArg::as_arg(&__stmt_iterator), &val)?;
                if has_for {
                    stmts = unrollForStatements(&stmts)?;
                }
                statements = List::append_reverse(&stmts, statements);
            }
            statements
        },
        _ => metamodelica::cons(stmt, statements),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(statements)
}

pub(crate) fn flattenAlgorithms(
    mut algorithms: &metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
) -> Result<metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>> {
    let mut outAlgorithms: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>> = metamodelica::nil();
    for mut alg in &**algorithms {
        let mut alg = alg.clone();
        assign_field!(alg.statements = flattenStatements(alg.statements.clone(), prefix)?);
        if ComponentRef::hasSubscripts(&(Prefix::prefix(prefix)))? {
            assign_field!(alg.source = addElementSourceArrayPrefix(alg.source.clone(), prefix)?);
        }
        outAlgorithms = metamodelica::cons(alg, outAlgorithms);
    }
    Ok(outAlgorithms)
}

pub(crate) fn flattenStatements(
    mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
    let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = stmts;
    stmts = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
        for mut s in (stmts).into_iter().cloned() {
            let __x = flattenStatement(s.clone(), prefix)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(stmts)
}

pub(crate) fn flattenStatement(
    mut stmt: metamodelica::Ref<Statement::NFStatement>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
) -> Result<metamodelica::Ref<Statement::NFStatement>> {
    let mut stmt: metamodelica::Ref<Statement::NFStatement> = stmt;
    let mut info: SourceInfo = Statement::info(&stmt);
    stmt = (match &*stmt {
        Statement::ASSIGNMENT {
            lhs: __stmt_lhs,
            rhs: __stmt_rhs,
            source: __stmt_source,
            ty: __stmt_ty,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            let mut ty: metamodelica::Ref<Type::NFType>;
            e1 = flattenExp(__stmt_lhs.clone(), prefix, &info)?;
            e2 = flattenExp(__stmt_rhs.clone(), prefix, &info)?;
            ty = flattenType(__stmt_ty.clone(), prefix, &info)?;
            metamodelica::Ref::new(Statement::NFStatement::ASSIGNMENT {
                lhs: e1,
                rhs: e2,
                ty: ty,
                source: __stmt_source.clone(),
            })
        }
        Statement::FOR {
            range: __stmt_range, ..
        } => {
            assign_variant_field!(stmt => Statement::NFStatement::FOR;
                range = Util::applyOption(__stmt_range.clone(), &({ let __pe_b1 = prefix.clone(); let __pe_b2 = info; move |__pe_a0| flattenExp(__pe_a0, &__pe_b1, &__pe_b2) }))?,
                body = flattenStatements(var_field!((*stmt).body, Statement::NFStatement::FOR).clone(), prefix)?
            );
            assign_variant_field!(stmt => Statement::NFStatement::FOR; forType = updateForType(var_field!((*stmt).forType, Statement::NFStatement::FOR).clone(), &(var_field!((*stmt).body, Statement::NFStatement::FOR).clone()))?);
            stmt
        }
        Statement::IF {
            branches: __stmt_branches,
            ..
        } => {
            assign_variant_field!(stmt => Statement::NFStatement::IF; branches = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, metamodelica::List<metamodelica::Ref<Statement::NFStatement>>)> = metamodelica::nil();
                for mut b in (__stmt_branches.clone()).into_iter().cloned() {
                    let __x = flattenStmtBranch(b.clone(), prefix, &info)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            stmt
        }
        Statement::WHEN {
            branches: __stmt_branches,
            ..
        } => {
            assign_variant_field!(stmt => Statement::NFStatement::WHEN; branches = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, metamodelica::List<metamodelica::Ref<Statement::NFStatement>>)> = metamodelica::nil();
                for mut b in (__stmt_branches.clone()).into_iter().cloned() {
                    let __x = flattenStmtBranch(b.clone(), prefix, &info)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            stmt
        }
        Statement::ASSERT {
            condition: __stmt_condition,
            level: __stmt_level,
            message: __stmt_message,
            source: __stmt_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            let mut e3: metamodelica::Ref<Expression::NFExpression>;
            e1 = flattenExp(__stmt_condition.clone(), prefix, &info)?;
            e2 = flattenExp(__stmt_message.clone(), prefix, &info)?;
            e3 = flattenExp(__stmt_level.clone(), prefix, &info)?;
            metamodelica::Ref::new(Statement::NFStatement::ASSERT {
                condition: e1,
                message: e2,
                level: e3,
                source: __stmt_source.clone(),
            })
        }
        Statement::TERMINATE {
            message: __stmt_message,
            source: __stmt_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            e1 = flattenExp(__stmt_message.clone(), prefix, &info)?;
            metamodelica::Ref::new(Statement::NFStatement::TERMINATE {
                message: e1,
                source: __stmt_source.clone(),
            })
        }
        Statement::REINIT {
            cref: __stmt_cref,
            reinitExp: __stmt_reinitExp,
            source: __stmt_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e1 = flattenExp(__stmt_cref.clone(), prefix, &info)?;
            e2 = flattenExp(__stmt_reinitExp.clone(), prefix, &info)?;
            metamodelica::Ref::new(Statement::NFStatement::REINIT {
                cref: e1,
                reinitExp: e2,
                source: __stmt_source.clone(),
            })
        }
        Statement::NORETCALL {
            exp: __stmt_exp,
            source: __stmt_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            e1 = flattenExp(__stmt_exp.clone(), prefix, &info)?;
            metamodelica::Ref::new(Statement::NFStatement::NORETCALL {
                exp: e1,
                source: __stmt_source.clone(),
            })
        }
        Statement::WHILE {
            body: __stmt_body,
            condition: __stmt_condition,
            source: __stmt_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            e1 = flattenExp(__stmt_condition.clone(), prefix, &info)?;
            body = flattenStatements(__stmt_body.clone(), prefix)?;
            metamodelica::Ref::new(Statement::NFStatement::WHILE {
                condition: e1,
                body: body,
                source: __stmt_source.clone(),
            })
        }
        Statement::FAILURE {
            body: __stmt_body,
            source: __stmt_source,
        } => {
            let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            body = flattenStatements(__stmt_body.clone(), prefix)?;
            metamodelica::Ref::new(Statement::NFStatement::FAILURE {
                body: body,
                source: __stmt_source.clone(),
            })
        }
        _ => stmt,
    });
    Ok(stmt)
}

pub(crate) fn flattenStmtBranch(
    mut branch: (
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    ),
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<Expression::NFExpression>,
    metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
)> {
    let mut branch: (
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    ) = branch;
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    (cond, body) = branch;
    cond = flattenExp(cond, prefix, info)?;
    body = flattenStatements(body, prefix)?;
    branch = (cond, body);
    Ok(branch)
}

pub(crate) fn addElementSourceArrayPrefix(
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut prefix: &metamodelica::Ref<Prefix::Prefix>,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut source: metamodelica::Ref<DAE::ElementSource> = source;
    let mut comp_pre: metamodelica::Ref<DAE::ComponentPrefix>;
    comp_pre = metamodelica::Ref::new(DAE::ComponentPrefix::PRE {
        prefix: ComponentRef::firstName(&(Prefix::prefix(prefix)), false)?,
        dimensions: metamodelica::nil(),
        subscripts: list![metamodelica::Ref::new(DAE::Subscript::INDEX {
            exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: -1 })
        })],
        next: openmodelica_frontend_types::DAE::ComponentPrefix::interned_NOCOMPPRE(),
        ci_state: ClassInf::State::UNKNOWN {
            path: metamodelica::Ref::new(Path::IDENT { name: literal!("?") }),
        },
        info: Absyn::dummyInfo.clone(),
    });
    source = ElementSource::addElementSourceInstanceOpt(source, comp_pre)?;
    Ok(source)
}

pub(crate) fn isDeletedCref(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut deletedVars: DeletedVariables,
) -> Result<bool> {
    let mut res: bool;
    let mut cr: metamodelica::Ref<ComponentRef::NFComponentRef> = cref.clone();
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    (cr, _) = ComponentRef::stripSubscripts(cref);
    while ComponentRef::isCref(&cr) {
        node = ComponentRef::node(&cr)?;
        if NFInstNode::InstNode::isComponent(&node)?
            && Component::hasCondition(&(NFInstNode::InstNode::component(&node)?))
        {
            if UnorderedSet::contains(cr.clone(), deletedVars.clone())? {
                res = true;
                return Ok(res);
            }
        }
        (cr, _) = ComponentRef::stripSubscripts(ComponentRef::rest(&cr)?);
    }
    res = false;
    Ok(res)
}

pub(crate) fn resolveConnections(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
    mut deletedVars: DeletedVariables,
    mut settings: FlattenSettings,
) -> Result<metamodelica::Ref<FlatModel::NFFlatModel>> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    let mut conns: metamodelica::Ref<Connections::NFConnections>;
    let mut conn_eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut stream_eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut ec_eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut tlio_eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut tlio_vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    let mut csets: ConnectionSets::Sets;
    let mut csets_array: metamodelica::Array<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>>;
    let mut unhandled_stream_sets: metamodelica::List<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>>;
    let mut ctable: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>>;
    let mut broken: metamodelica::List<Connections::BrokenEdge> = metamodelica::nil();
    let mut vars: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >;
    let mut connectedLocalIOs: metamodelica::Ref<
        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >;
    let mut exposeLocalIOs: i32;
    let mut flow_alias_repl: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >;
    let mut flow_alias_repl_opt: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    > = None;
    vars = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0))
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                  __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                ComponentRef::isEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >),
        ((flatModel.variables).len() as i32),
    );
    for mut v in &*flatModel.variables.clone() {
        UnorderedMap::addNew(v.name.clone(), v.clone(), vars.clone())?;
    }
    (flatModel, conns) = Connections::collectConnections(
        flatModel,
        &({
            let __pe_b1 = deletedVars.clone();
            move |__pe_a0| isDeletedCref(__pe_a0, __pe_b1.clone())
        }),
    )?;
    ctable = CardinalityTable::fromConnections(&conns)?;
    (flatModel, conns) = ExpandableConnectors::elaborate(flatModel, conns)?;
    assign_field!(
        flatModel.variables = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
            for mut v in (flatModel.variables.clone()).into_iter().cloned() {
                if !(Variable::isPresent(&(v.clone()))) {
                    continue;
                }
                let __x = v.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    conns = Connections::collectFlows(&flatModel, conns)?;
    if System::getHasOverconstrainedConnectors() {
        (flatModel, broken) = NFOCConnectionGraph::handleOverconstrainedConnections(
            flatModel,
            &conns,
            &({
                let __pe_b1 = deletedVars;
                move |__pe_a0| isDeletedCref(__pe_a0, __pe_b1.clone())
            }),
        )?;
    }
    conns = Connections::addBroken(broken.clone(), conns);
    conns = Connections::split(conns)?;
    conns = Connections::scalarize(conns, !(settings.scalarize.clone()))?;
    csets = ConnectionSets::fromConnections(&conns)?;
    (csets_array, _) = ConnectionSets::extractSets(&csets)?;
    (conn_eql, connectedLocalIOs, unhandled_stream_sets) =
        ConnectEquations::generateEquations(csets_array.clone(), vars.clone())?;
    if System::getHasOverconstrainedConnectors() {
        ec_eql = List::flatten(
            ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> =
                    metamodelica::nil();
                for mut e in (broken).into_iter().cloned() {
                    let __x = e.brokenEquations.clone();
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        )?;
        assign_field!(flatModel.equations = listAppend(ec_eql, flatModel.equations.clone()));
    }
    assign_field!(flatModel.equations = listAppend(conn_eql, flatModel.equations.clone()));
    if !((unhandled_stream_sets).is_empty()) {
        (flatModel, flow_alias_repl) = StreamFlowAlias::eliminateAliases(flatModel, vars.clone())?;
        conn_eql = ConnectEquations::generateStreamEquationsList(
            &unhandled_stream_sets,
            vars.clone(),
            flow_alias_repl.clone(),
        )?;
        conn_eql = StreamFlowAlias::applyReplacementsInEql(flow_alias_repl.clone(), conn_eql)?;
        assign_field!(flatModel.equations = listAppend(conn_eql, flatModel.equations.clone()));
        flow_alias_repl_opt = Some(flow_alias_repl);
    }
    exposeLocalIOs = Flags::getConfigInt(Flags::EXPOSE_LOCAL_IOS.clone())?;
    if exposeLocalIOs > 0 {
        (tlio_vars, tlio_eql) = generateTopLevelIOs(vars.clone(), connectedLocalIOs, exposeLocalIOs)?;
        assign_field!(
            flatModel.variables = List::append_reverse(&flatModel.variables, tlio_vars),
            flatModel.equations = List::append_reverse(&flatModel.equations, tlio_eql)
        );
    }
    if System::getHasStreamConnectors() || System::getUsesCardinality() {
        flatModel = evaluateConnectionOperators(
            flatModel,
            &csets,
            csets_array.clone(),
            vars,
            ctable,
            flow_alias_repl_opt,
        )?;
    }
    if Flags::getConfigBool(Flags::BUILDING_FMU.clone())?
        && stringEq(
            &(Flags::getConfigString(Flags::FMI_VERSION.clone())?),
            &(literal!("3.0")),
        )
        && !(settings.newBackend.clone())
    {
        flatModel = causalizeAcausalConnectors(flatModel)?;
    }
    execStat(&(literal!("NFFlatten.resolveConnections")))?;
    Ok(flatModel)
}

pub(crate) fn generateTopLevelIOs(
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut connectedLocalIOs: metamodelica::Ref<
        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >,
    mut exposeLocalIOs: i32,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
)> {
    let mut tlio_vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    let mut tlio_eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut attributes: metamodelica::Ref<Attributes::NFAttributes>;
    let mut tlio_var: metamodelica::Ref<Variable::NFVariable>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut name: ArcStr;
    let mut tlio_node: metamodelica::Ref<InstNode::InstNode>;
    let mut level: i32;
    tlio_vars = metamodelica::nil();
    tlio_eql = metamodelica::nil();
    for mut variable in &*UnorderedMap::valueList(variables.clone()) {
        level = ComponentRef::depth(&variable.name) - 1;
        attributes = variable.attributes.clone();
        if 0 < level
            && level <= exposeLocalIOs
            && variable.visibility.clone() == Visibility::PUBLIC.clone()
            && attributes.connectorType.clone() != ConnectorType::NON_CONNECTOR.clone()
            && (attributes.direction.clone() == Direction::INPUT.clone()
                || attributes.direction.clone() == Direction::OUTPUT.clone())
            && !(UnorderedSet::contains(variable.name.clone(), connectedLocalIOs.clone())?)
        {
            tlio_var = Variable::removeNonTopLevelDirection(variable.clone())?;
            attributes = tlio_var.attributes.clone();
            if attributes.direction.clone() == Direction::NONE.clone() {
                tlio_var = variable.clone();
                assign_field!(tlio_var.binding = crate::NFBinding::interned_UNBOUND());
                cref = tlio_var.name.clone();
                name = stringDelimitList(ComponentRef::toString_impl(&cref, metamodelica::nil())?, literal!("."));
                while UnorderedMap::contains(tlio_var.name.clone(), variables.clone())? {
                    tlio_node = metamodelica::Ref::new(InstNode::InstNode::NAME_NODE {
                        name: Util::makeQuotedIdentifier(name.clone())?,
                    });
                    assign_field!(
                        tlio_var.name = (match &*cref {
                            ComponentRef::CREF {
                                subscripts: __cref_subscripts,
                                ty: __cref_ty,
                                ..
                            } => ComponentRef::prefixCref(
                                tlio_node,
                                __cref_ty.clone(),
                                __cref_subscripts.clone(),
                                crate::NFComponentRef::interned_EMPTY()
                            )?,
                            _ => return Err("match: no arm matched"),
                        })
                    );
                    name = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*name);
                        __mm_s.push_str(&*literal!("_"));
                        ArcStr::from(__mm_s)
                    };
                }
                tlio_vars = metamodelica::cons(tlio_var.clone(), tlio_vars);
                tlio_eql = metamodelica::cons(
                    Equation::makeCrefEquality(
                        variable.name.clone(),
                        tlio_var.name.clone(),
                        crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
                        ElementSource::createElementSource(
                            variable.info.clone(),
                            None,
                            &(openmodelica_frontend_types::DAE::Prefix::NOPRE),
                            (DAE::emptyCref().clone(), DAE::emptyCref().clone()),
                        ),
                    )?,
                    tlio_eql,
                );
            }
        }
    }
    Ok((tlio_vars, tlio_eql))
}

pub(crate) fn causalizeAcausalConnectors(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
) -> Result<metamodelica::Ref<FlatModel::NFFlatModel>> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    let mut flowCrefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut unconnectedFlows: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
    let mut boundaryConnectors: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
        metamodelica::nil();
    let mut kept: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    let mut isZeroFlowEq: bool;
    let mut fc: metamodelica::Ref<ComponentRef::NFComponentRef>;
    flowCrefs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        for mut v in (flatModel.variables.clone()).into_iter().cloned() {
            if !(Variable::isFlow(&(v.clone())) && Variable::isPublic(&(v.clone()))) {
                continue;
            }
            let __x = v.name.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if (flowCrefs).is_empty() {
        return Ok(flatModel);
    }
    for mut eq in &*flatModel.equations.clone() {
        isZeroFlowEq = false;
        let () = (::match_deref::match_deref! { match &(eq.clone()) {
            Deref @ Equation::EQUALITY { lhs: Deref @ Expression::CREF { cref: fc, .. }, rhs: Deref @ Expression::REAL { value: rv }, .. } if (rv.clone() == metamodelica::OrderedFloat(0.0_f64) && List::isMemberOnTrue(fc.clone(), &flowCrefs, &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1))? && ComponentRef::isSimple(&(ComponentRef::rest(metamodelica::AsArg::as_arg(&fc))?))) => {
                unconnectedFlows = metamodelica::cons(fc.clone(), unconnectedFlows);
                boundaryConnectors = metamodelica::cons(ComponentRef::rest(metamodelica::AsArg::as_arg(&fc))?, boundaryConnectors);
                isZeroFlowEq = true;
                ()
            },
            _ => {
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        if !(isZeroFlowEq) {
            kept = metamodelica::cons(eq.clone(), kept);
        }
    }
    if (unconnectedFlows).is_empty() {
        return Ok(flatModel);
    }
    assign_field!(
        flatModel.equations = metamodelica::Dangerous::listReverseInPlace(kept),
        flatModel.variables = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
            for mut v in (flatModel.variables.clone()).into_iter().cloned() {
                let __x = causalizeAcausalVar(v.clone(), &unconnectedFlows, &boundaryConnectors)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    Ok(flatModel)
}

pub(crate) fn causalizeAcausalVar(
    mut var: metamodelica::Ref<Variable::NFVariable>,
    mut unconnectedFlows: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut boundaryConnectors: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = var;
    let mut attr: metamodelica::Ref<Attributes::NFAttributes>;
    if Variable::isFlow(&var)
        && List::isMemberOnTrue(var.name.clone(), unconnectedFlows, &move |__a0: metamodelica::Ref<
            ComponentRef::NFComponentRef,
        >,
                                                                           __a1: metamodelica::Ref<
            ComponentRef::NFComponentRef,
        >| {
            ComponentRef::isEqual(&__a0, &__a1)
        })?
    {
        attr = var.attributes.clone();
        assign_field!(attr.direction = Direction::INPUT.clone());
        assign_field!(var.attributes = attr);
    } else if Variable::isPotential(&var)
        && List::isMemberOnTrue(
            ComponentRef::rest(&var.name)?,
            boundaryConnectors,
            &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                   __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                ComponentRef::isEqual(&__a0, &__a1)
            },
        )?
    {
        attr = var.attributes.clone();
        assign_field!(attr.direction = Direction::OUTPUT.clone());
        assign_field!(var.attributes = attr);
    }
    Ok(var)
}

pub(crate) fn evaluateConnectionOperators(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
    mut sets: &ConnectionSets::Sets,
    mut setsArray: metamodelica::Array<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut ctable: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>>,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<metamodelica::Ref<FlatModel::NFFlatModel>> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    assign_field!(
        flatModel.variables = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
            for mut c in (flatModel.variables.clone()).into_iter().cloned() {
                let __x = evaluateBindingConnOp(
                    c.clone(),
                    sets,
                    setsArray.clone(),
                    variables.clone(),
                    ctable.clone(),
                    replacements.clone(),
                )?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        flatModel.equations = evaluateEquationsConnOp(
            flatModel.equations.clone(),
            sets,
            setsArray.clone(),
            variables.clone(),
            ctable.clone(),
            replacements.clone()
        )?,
        flatModel.initialEquations = evaluateEquationsConnOp(
            flatModel.initialEquations.clone(),
            sets,
            setsArray.clone(),
            variables.clone(),
            ctable.clone(),
            replacements.clone()
        )?,
        flatModel.algorithms = evaluateAlgorithmsConnOp(
            flatModel.algorithms.clone(),
            sets,
            setsArray.clone(),
            variables.clone(),
            ctable.clone(),
            replacements.clone()
        )?,
        flatModel.initialAlgorithms = evaluateAlgorithmsConnOp(
            flatModel.initialAlgorithms.clone(),
            sets,
            setsArray.clone(),
            variables,
            ctable,
            replacements
        )?
    );
    Ok(flatModel)
}

pub(crate) fn evaluateAlgorithmsConnOp(
    mut algorithms: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
    mut sets: &ConnectionSets::Sets,
    mut setsArray: metamodelica::Array<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut ctable: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>>,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>> {
    let mut algorithms: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>> = algorithms;
    algorithms = Algorithm::mapExpList(
        algorithms,
        &({
            let __pe_b1 = sets.clone();
            let __pe_b2 = setsArray.clone();
            let __pe_b3 = variables;
            let __pe_b4 = ctable;
            let __pe_b5 = replacements;
            move |__pe_a0| {
                ConnectEquations::evaluateOperators(
                    __pe_a0,
                    &__pe_b1,
                    __pe_b2.clone(),
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                    __pe_b5.clone(),
                )
            }
        }),
    )?;
    Ok(algorithms)
}

pub(crate) fn evaluateBindingConnOp(
    mut var: metamodelica::Ref<Variable::NFVariable>,
    mut sets: &ConnectionSets::Sets,
    mut setsArray: metamodelica::Array<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut ctable: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>>,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = var;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut eval_exp: metamodelica::Ref<Expression::NFExpression>;
    let () = (match &*var {
        Variable::VARIABLE { .. } if (Binding::hasExp(&var.binding)) => {
            exp = Binding::getExp(&var.binding)?;
            eval_exp = ConnectEquations::evaluateOperators(
                exp.clone(),
                sets,
                setsArray.clone(),
                variables,
                ctable,
                replacements,
            )?;
            if !(referenceEq(&*(exp), &*(&*eval_exp))) {
                assign_field!(var.binding = Binding::setExp(eval_exp, var.binding.clone())?);
            }
            ()
        }
        _ => (),
    });
    Ok(var)
}

pub(crate) fn evaluateEquationsConnOp(
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut sets: &ConnectionSets::Sets,
    mut setsArray: metamodelica::Array<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut ctable: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>>,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = equations;
    equations = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
        for mut eq in (equations).into_iter().cloned() {
            let __x = evaluateEquationConnOp(
                eq.clone(),
                sets,
                setsArray.clone(),
                variables.clone(),
                ctable.clone(),
                replacements.clone(),
            )?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(equations)
}

pub(crate) fn evaluateEquationConnOp(
    mut eq: metamodelica::Ref<Equation::NFEquation>,
    mut sets: &ConnectionSets::Sets,
    mut setsArray: metamodelica::Array<metamodelica::List<metamodelica::Ref<Connector::NFConnector>>>,
    mut variables: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Variable::NFVariable>,
        >,
    >,
    mut ctable: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>>,
    mut replacements: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    >,
) -> Result<metamodelica::Ref<Equation::NFEquation>> {
    let mut eq: metamodelica::Ref<Equation::NFEquation> = eq;
    eq = Equation::mapExp(
        eq,
        &({
            let __pe_b1 = sets.clone();
            let __pe_b2 = setsArray.clone();
            let __pe_b3 = variables;
            let __pe_b4 = ctable;
            let __pe_b5 = replacements;
            move |__pe_a0| {
                ConnectEquations::evaluateOperators(
                    __pe_a0,
                    &__pe_b1,
                    __pe_b2.clone(),
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                    __pe_b5.clone(),
                )
            }
        }),
    )?;
    let () = (match &*eq {
        Equation::IF {
            branches: __eq_branches,
            ..
        } => {
            for mut b in &*__eq_branches.clone() {
                let () = (match &*b.clone() {
                    Equation::Branch::BRANCH {
                        condition: __b_condition,
                        conditionVar: __b_conditionVar,
                        ..
                    } => {
                        if __b_conditionVar.clone() == Variability::PARAMETER.clone()
                            && !(Structural::isExpressionNotFixed(
                                metamodelica::AsArg::as_arg(&__b_condition),
                                false,
                                100,
                            )?)
                        {
                            Structural::markExp(metamodelica::AsArg::as_arg(&__b_condition))?;
                        }
                        ()
                    }
                    _ => (),
                });
            }
            ()
        }
        _ => (),
    });
    Ok(eq)
}

pub(crate) fn resolveArrayConnections(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
) -> Result<metamodelica::Ref<FlatModel::NFFlatModel>> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    flatModel = ArrayConnections::resolve(flatModel)?;
    execStat(&(literal!("NFFlatten.resolveArrayConnections")))?;
    Ok(flatModel)
}

pub(crate) fn collectComponentFuncs(
    mut var: &metamodelica::Ref<Variable::NFVariable>,
    mut funcs: FunctionTree,
) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    let () = (match &**var {
        Variable::VARIABLE { .. } => {
            funcs = collectTypeFuncs(&var.ty, funcs)?;
            funcs = collectBindingFuncs(&var.binding, funcs)?;
            for mut attr in &*var.typeAttributes.clone() {
                funcs = collectBindingFuncs(&(Util::tuple22(attr.clone())), funcs)?;
            }
            for mut c in &*var.children.clone() {
                funcs = collectComponentFuncs(metamodelica::AsArg::as_arg(&c), funcs)?;
            }
            ()
        }
    });
    Ok(funcs)
}

pub(crate) fn collectBindingFuncs(
    mut binding: &metamodelica::Ref<Binding::NFBinding>,
    mut funcs: FunctionTree,
) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    if Binding::isExplicitlyBound(binding) {
        funcs = collectExpFuncs(Binding::getTypedExp(binding)?, funcs)?;
    }
    Ok(funcs)
}

pub(crate) fn collectTypeFuncs(
    mut ty: &metamodelica::Ref<Type::NFType>,
    mut funcs: FunctionTree,
) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    let () = (::match_deref::match_deref! { match ty {
        Deref @ Type::ARRAY { dimensions: __ty_dimensions, elementType: __ty_elementType } => {
            funcs = Dimension::foldExpList(metamodelica::AsArg::as_arg(&__ty_dimensions), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Expression::NFExpression>, __a1: metamodelica::Ref<FunctionTreeImpl::Tree>| collectExpFuncs_traverse(&__a0, __a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, metamodelica::Ref<FunctionTreeImpl::Tree>) -> Result<metamodelica::Ref<FunctionTreeImpl::Tree>> + 'static>), funcs)?;
            funcs = collectTypeFuncs(metamodelica::AsArg::as_arg(&__ty_elementType), funcs)?;
            ()
        },
        Deref @ Type::FUNCTION { r#fn, .. } => {
            funcs = flattenFunction(r#fn.clone(), funcs)?;
            ()
        },
        Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { constructor: con, destructor: de }, .. } => {
            funcs = collectStructor(&(NFInstNode::InstNode::borrow(con.clone())?), funcs)?;
            funcs = collectStructor(&(NFInstNode::InstNode::borrow(de.clone())?), funcs)?;
            ()
        },
        Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::RECORD { constructor: rec_con, .. }, .. } => {
            funcs = collectStructor(&(NFInstNode::InstNode::borrow(rec_con.clone())?), funcs)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(funcs)
}

pub(crate) fn collectStructor(
    mut node: &metamodelica::Ref<InstNode::InstNode>,
    mut funcs: FunctionTree,
) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    let mut cache: metamodelica::Ref<CachedData::CachedData>;
    let mut r#fn: metamodelica::List<metamodelica::Ref<Function::Function>> = metamodelica::nil();
    cache = NFInstNode::InstNode::getFuncCache(node)?;
    let () = (match &*cache {
        NFInstNode::CachedData::FUNCTION {
            funcs: __cache_funcs, ..
        } => {
            for mut r#fn in &*__cache_funcs.clone() {
                let mut r#fn = r#fn.clone();
                funcs = flattenFunction(r#fn, funcs)?;
            }
            ()
        }
        _ => (),
    });
    Ok(funcs)
}

pub(crate) fn collectEquationFuncs(
    mut eq: &metamodelica::Ref<Equation::NFEquation>,
    mut funcs: FunctionTree,
) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    let () = (match &**eq {
        Equation::EQUALITY {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ty: __eq_ty,
            ..
        } => {
            funcs = collectExpFuncs(__eq_lhs.clone(), funcs)?;
            funcs = collectExpFuncs(__eq_rhs.clone(), funcs)?;
            funcs = collectTypeFuncs(metamodelica::AsArg::as_arg(&__eq_ty), funcs)?;
            ()
        }
        Equation::FOR { body: __eq_body, .. } => {
            funcs = List::fold(
                metamodelica::AsArg::as_arg(&__eq_body),
                &move |__a0: metamodelica::Ref<Equation::NFEquation>,
                       __a1: metamodelica::Ref<FunctionTreeImpl::Tree>| {
                    collectEquationFuncs(&__a0, __a1)
                },
                funcs,
            )?;
            ()
        }
        Equation::IF {
            branches: __eq_branches,
            ..
        } => {
            funcs = List::fold(
                metamodelica::AsArg::as_arg(&__eq_branches),
                &move |__a0: metamodelica::Ref<Equation::Branch::Branch>,
                       __a1: metamodelica::Ref<FunctionTreeImpl::Tree>| {
                    collectEqBranchFuncs(&__a0, __a1)
                },
                funcs,
            )?;
            ()
        }
        Equation::WHEN {
            branches: __eq_branches,
            ..
        } => {
            funcs = List::fold(
                metamodelica::AsArg::as_arg(&__eq_branches),
                &move |__a0: metamodelica::Ref<Equation::Branch::Branch>,
                       __a1: metamodelica::Ref<FunctionTreeImpl::Tree>| {
                    collectEqBranchFuncs(&__a0, __a1)
                },
                funcs,
            )?;
            ()
        }
        Equation::ASSERT {
            condition: __eq_condition,
            level: __eq_level,
            message: __eq_message,
            ..
        } => {
            funcs = collectExpFuncs(__eq_condition.clone(), funcs)?;
            funcs = collectExpFuncs(__eq_message.clone(), funcs)?;
            funcs = collectExpFuncs(__eq_level.clone(), funcs)?;
            ()
        }
        Equation::TERMINATE {
            message: __eq_message, ..
        } => {
            funcs = collectExpFuncs(__eq_message.clone(), funcs)?;
            ()
        }
        Equation::REINIT {
            reinitExp: __eq_reinitExp,
            ..
        } => {
            funcs = collectExpFuncs(__eq_reinitExp.clone(), funcs)?;
            ()
        }
        Equation::NORETCALL { exp: __eq_exp, .. } => {
            funcs = collectExpFuncs(__eq_exp.clone(), funcs)?;
            ()
        }
        _ => (),
    });
    Ok(funcs)
}

pub(crate) fn collectEqBranchFuncs(
    mut branch: &metamodelica::Ref<Equation::Branch::Branch>,
    mut funcs: FunctionTree,
) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    let () = (match &**branch {
        Equation::Branch::BRANCH {
            body: __branch_body,
            condition: __branch_condition,
            ..
        } => {
            funcs = collectExpFuncs(__branch_condition.clone(), funcs)?;
            funcs = List::fold(
                metamodelica::AsArg::as_arg(&__branch_body),
                &move |__a0: metamodelica::Ref<Equation::NFEquation>,
                       __a1: metamodelica::Ref<FunctionTreeImpl::Tree>| {
                    collectEquationFuncs(&__a0, __a1)
                },
                funcs,
            )?;
            ()
        }
        _ => (),
    });
    Ok(funcs)
}

pub(crate) fn collectAlgorithmFuncs(
    mut alg: &metamodelica::Ref<Algorithm::NFAlgorithm>,
    mut funcs: FunctionTree,
) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    funcs = List::fold(
        &alg.statements,
        &move |__a0: metamodelica::Ref<Statement::NFStatement>, __a1: metamodelica::Ref<FunctionTreeImpl::Tree>| {
            collectStatementFuncs(&__a0, __a1)
        },
        funcs,
    )?;
    Ok(funcs)
}

pub(crate) fn collectStatementFuncs(
    mut stmt: &metamodelica::Ref<Statement::NFStatement>,
    mut funcs: FunctionTree,
) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    let () = (match &**stmt {
        Statement::ASSIGNMENT {
            lhs: __stmt_lhs,
            rhs: __stmt_rhs,
            ty: __stmt_ty,
            ..
        } => {
            funcs = collectExpFuncs(__stmt_lhs.clone(), funcs)?;
            funcs = collectExpFuncs(__stmt_rhs.clone(), funcs)?;
            funcs = collectTypeFuncs(metamodelica::AsArg::as_arg(&__stmt_ty), funcs)?;
            ()
        }
        Statement::FOR {
            body: __stmt_body,
            range: __stmt_range,
            ..
        } => {
            funcs = List::fold(
                metamodelica::AsArg::as_arg(&__stmt_body),
                &move |__a0: metamodelica::Ref<Statement::NFStatement>,
                       __a1: metamodelica::Ref<FunctionTreeImpl::Tree>| {
                    collectStatementFuncs(&__a0, __a1)
                },
                funcs,
            )?;
            funcs = collectExpFuncs(Util::getOption(__stmt_range.clone())?, funcs)?;
            ()
        }
        Statement::IF {
            branches: __stmt_branches,
            ..
        } => {
            funcs = List::fold(
                metamodelica::AsArg::as_arg(&__stmt_branches),
                &collectStmtBranchFuncs,
                funcs,
            )?;
            ()
        }
        Statement::WHEN {
            branches: __stmt_branches,
            ..
        } => {
            funcs = List::fold(
                metamodelica::AsArg::as_arg(&__stmt_branches),
                &collectStmtBranchFuncs,
                funcs,
            )?;
            ()
        }
        Statement::ASSERT {
            condition: __stmt_condition,
            level: __stmt_level,
            message: __stmt_message,
            ..
        } => {
            funcs = collectExpFuncs(__stmt_condition.clone(), funcs)?;
            funcs = collectExpFuncs(__stmt_message.clone(), funcs)?;
            funcs = collectExpFuncs(__stmt_level.clone(), funcs)?;
            ()
        }
        Statement::TERMINATE {
            message: __stmt_message,
            ..
        } => {
            funcs = collectExpFuncs(__stmt_message.clone(), funcs)?;
            ()
        }
        Statement::REINIT {
            cref: __stmt_cref,
            reinitExp: __stmt_reinitExp,
            ..
        } => {
            funcs = collectExpFuncs(__stmt_cref.clone(), funcs)?;
            funcs = collectExpFuncs(__stmt_reinitExp.clone(), funcs)?;
            ()
        }
        Statement::NORETCALL { exp: __stmt_exp, .. } => {
            funcs = collectExpFuncs(__stmt_exp.clone(), funcs)?;
            ()
        }
        Statement::WHILE {
            body: __stmt_body,
            condition: __stmt_condition,
            ..
        } => {
            funcs = collectExpFuncs(__stmt_condition.clone(), funcs)?;
            funcs = List::fold(
                metamodelica::AsArg::as_arg(&__stmt_body),
                &move |__a0: metamodelica::Ref<Statement::NFStatement>,
                       __a1: metamodelica::Ref<FunctionTreeImpl::Tree>| {
                    collectStatementFuncs(&__a0, __a1)
                },
                funcs,
            )?;
            ()
        }
        _ => (),
    });
    Ok(funcs)
}

pub(crate) fn collectStmtBranchFuncs(
    mut branch: (
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    ),
    mut funcs: FunctionTree,
) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    funcs = collectExpFuncs(Util::tuple21(branch.clone()), funcs)?;
    funcs = List::fold(
        &(Util::tuple22(branch)),
        &move |__a0: metamodelica::Ref<Statement::NFStatement>, __a1: metamodelica::Ref<FunctionTreeImpl::Tree>| {
            collectStatementFuncs(&__a0, __a1)
        },
        funcs,
    )?;
    Ok(funcs)
}

pub(crate) fn collectExpFuncs(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut funcs: FunctionTree,
) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    funcs = Expression::fold(
        exp,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<Expression::NFExpression>,
                  __a1: metamodelica::Ref<FunctionTreeImpl::Tree>| collectExpFuncs_traverse(&__a0, __a1),
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                        metamodelica::Ref<FunctionTreeImpl::Tree>,
                    ) -> Result<metamodelica::Ref<FunctionTreeImpl::Tree>>
                    + 'static,
            >),
        funcs,
    )?;
    Ok(funcs)
}

pub(crate) fn collectExpFuncs_traverse(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut funcs: FunctionTree,
) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    let () = (match &**exp {
        Expression::CALL { call: __exp_call } => {
            funcs = flattenFunction(Call::typedFunction(metamodelica::AsArg::as_arg(&__exp_call))?, funcs)?;
            ()
        }
        Expression::CREF { ty: __exp_ty, .. } => {
            funcs = collectTypeFuncs(metamodelica::AsArg::as_arg(&__exp_ty), funcs)?;
            ()
        }
        Expression::RECORD { ty: __exp_ty, .. } => {
            funcs = collectTypeFuncs(metamodelica::AsArg::as_arg(&__exp_ty), funcs)?;
            ()
        }
        Expression::PARTIAL_FUNCTION_APPLICATION { r#fn: __exp_fn, .. } => {
            for mut f in &*Function::getRefCache(metamodelica::AsArg::as_arg(&__exp_fn))? {
                funcs = flattenFunction(f.clone(), funcs)?;
            }
            ()
        }
        _ => (),
    });
    Ok(funcs)
}

pub(crate) fn flattenFunction(
    mut func: metamodelica::Ref<Function::Function>,
    mut funcs: FunctionTree,
) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    let mut r#fn: metamodelica::Ref<Function::Function> = func;
    if !(Function::isCollected(&r#fn)) {
        r#fn = Function::mapExp(
            r#fn,
            (std::sync::Arc::new(Expression::expandSplitIndices)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
            (std::sync::Arc::new(Expression::expandSplitIndices)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
            true,
            true,
        )?;
        r#fn = EvalConstants::evaluateFunction(r#fn)?;
        SimplifyModel::simplifyFunction(r#fn.clone())?;
        Function::collect(&r#fn);
        if !(NFInstNode::InstNode::isPartial(&(NFInstNode::InstNode::fromHandle(&r#fn.node)?))?) {
            funcs = FunctionTreeImpl::add(
                funcs,
                &(Function::name(&r#fn)),
                &(r#fn.clone()),
                &*(std::sync::Arc::new(fnptr!(FunctionTreeImpl::addConflictDefault, _, _, _))
                    as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>),
            )?;
            funcs = collectClassFunctions(NFInstNode::InstNode::fromHandle(&r#fn.node)?, funcs)?;
            for mut fn_der in &*r#fn.derivatives.clone() {
                for mut der_fn in
                    &*Function::getCachedFuncs(NFInstNode::InstNode::borrow(fn_der.derivativeFn.clone())?)?
                {
                    funcs = flattenFunction(der_fn.clone(), funcs)?;
                }
            }
            let __range0 = r#fn.inverses.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut fn_inv in __range0 {
                funcs = collectExpFuncs(fn_inv.inverseCall.clone(), funcs)?;
            }
            if Function::isPartialDerivative(&r#fn) {
                for mut f in
                    &*Function::getCachedFuncs(Class::lastBaseClass(NFInstNode::InstNode::fromHandle(&r#fn.node)?)?)?
                {
                    flattenFunction(f.clone(), funcs.clone())?;
                }
            }
        }
    }
    Ok(funcs)
}

pub(crate) fn collectClassFunctions(
    mut clsNode: metamodelica::Ref<InstNode::InstNode>,
    mut funcs: FunctionTree,
) -> Result<FunctionTree> {
    let mut funcs: FunctionTree = funcs;
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut cls_tree: metamodelica::Ref<ClassTree::ClassTree>;
    let mut sections: metamodelica::Ref<Sections::NFSections>;
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    cls = NFInstNode::InstNode::getClass(clsNode)?;
    let () = (::match_deref::match_deref! { match &(cls) {
        Deref @ Class::INSTANCED_CLASS { elements: __esc_cls_tree @ Deref @ ClassTree::FLAT_TREE { .. }, sections: __esc_sections, .. } => {
            cls_tree = (*__esc_cls_tree).clone();
            sections = (*__esc_sections).clone();
            let __range0 = var_field!((*cls_tree).components, ClassTree::ClassTree::FLAT_TREE).clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut c in __range0 {
                comp = NFInstNode::InstNode::component(&c)?;
                funcs = collectTypeFuncs(&(Component::getType(&comp)?), funcs)?;
                binding = Component::getBinding(&comp);
                if Binding::isExplicitlyBound(&binding) {
                    funcs = collectExpFuncs(Binding::getTypedExp(&binding)?, funcs)?;
                }
            }
            let () = (match &*sections.clone() {
        Sections::SECTIONS { algorithms: __sections_algorithms, .. } => {
            funcs = List::fold(metamodelica::AsArg::as_arg(&__sections_algorithms), &move |__a0: metamodelica::Ref<Algorithm::NFAlgorithm>, __a1: metamodelica::Ref<FunctionTreeImpl::Tree>| collectAlgorithmFuncs(&__a0, __a1), funcs)?;
            ()
        },
        _ => (),
    });
            ()
        },
        Deref @ Class::TYPED_DERIVED { baseClass: __cls_baseClass, .. } => {
            funcs = collectClassFunctions(__cls_baseClass.clone(), funcs)?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(funcs)
}

pub(crate) fn updateForType(
    mut forType: Statement::ForType,
    mut forBody: &metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
) -> Result<Statement::ForType> {
    let mut forType: Statement::ForType = forType;
    let mut vars: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, SourceInfo>,
    >;
    let () = (match forType.clone() {
        Statement::ForType::NORMAL => (),
        Statement::ForType::PARALLEL { .. } => {
            vars = UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
                    >),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                          __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                        ComponentRef::isEqual(&__a0, &__a1)
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                            ) -> Result<bool>
                            + 'static,
                    >),
                1,
            );
            for mut s in &**forBody {
                vars = Statement::fold(
                    s.clone(),
                    &move |__a0: metamodelica::Ref<Statement::NFStatement>,
                           __a1: metamodelica::Ref<
                        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, SourceInfo>,
                    >| collectParallelVariables(&__a0, __a1),
                    vars,
                )?;
            }
            let __owned_variant_vars_0 = UnorderedMap::toList(vars);
            if let Statement::ForType::PARALLEL { vars, .. } = &mut forType {
                *vars = __owned_variant_vars_0;
            } else {
                panic!("owned-variant field-assign: value held a different variant than Statement::ForType::PARALLEL");
            }
            for mut v in &*var_field!(forType.vars, Statement::ForType::PARALLEL).clone() {
                checkParGlobalCref(&(v.clone()))?;
            }
            ()
        }
    });
    Ok(forType)
}

pub(crate) fn collectParallelVariables(
    mut stmt: &metamodelica::Ref<Statement::NFStatement>,
    mut vars: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, SourceInfo>,
    >,
) -> Result<metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, SourceInfo>>>
{
    let mut vars: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, SourceInfo>,
    > = vars;
    let mut info: SourceInfo;
    info = Statement::info(stmt);
    vars = Statement::foldExp(
        stmt,
        &({
            let __pe_b1: Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, _) -> Result<_> + 'static,
            > = (std::sync::Arc::new({
                let __pe_b1 = info;
                move |__pe_a0, __pe_a2| collectParallelVariablesExp(&__pe_a0, __pe_b1.clone(), __pe_a2)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                            metamodelica::Ref<
                                UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, SourceInfo>,
                            >,
                        ) -> Result<
                            metamodelica::Ref<
                                UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, SourceInfo>,
                            >,
                        > + 'static,
                >);
            move |__pe_a0, __pe_a2| Expression::fold(__pe_a0, __pe_b1.clone(), __pe_a2)
        }),
        vars,
    )?;
    Ok(vars)
}

pub(crate) fn collectParallelVariablesExp(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut info: SourceInfo,
    mut vars: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, SourceInfo>,
    >,
) -> Result<metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, SourceInfo>>>
{
    let mut vars: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, SourceInfo>,
    > = vars;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let () = (match &**exp {
        Expression::CREF { cref: __exp_cref, .. }
            if (ComponentRef::isCref(metamodelica::AsArg::as_arg(&__exp_cref))
                && !(ComponentRef::isIterator(metamodelica::AsArg::as_arg(&__exp_cref)))
                && NFInstNode::InstNode::isComponent(
                    &(ComponentRef::node(metamodelica::AsArg::as_arg(&__exp_cref))?),
                )?) =>
        {
            cref = ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&__exp_cref));
            UnorderedMap::tryAdd(cref, info, vars.clone())?;
            ()
        }
        _ => (),
    });
    Ok(vars)
}

pub(crate) fn checkParGlobalCref(
    mut crefInfo: &(metamodelica::Ref<ComponentRef::NFComponentRef>, SourceInfo),
) -> Result<()> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut info: SourceInfo;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut errorString: ArcStr;
    (cref, info) = crefInfo.clone();
    node = ComponentRef::node(&cref)?;
    if Component::parallelism(&(NFInstNode::InstNode::component(&node)?)) != Parallelism::GLOBAL.clone() {
        errorString = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*literal!("- Component '"));
            __mm_s.push_str(&*AbsynUtil::pathString(
                ComponentRef::toPath(&cref)?,
                literal!("."),
                true,
                false,
            )?);
            __mm_s.push_str(&*literal!("' is used in a parallel for loop."));
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*literal!(
                "- Parallel for loops can only contain references to parglobal variables"
            ));
            ArcStr::from(__mm_s)
        };
        Error::addSourceMessage(&(Error::PARMODELICA_ERROR.clone()), list![errorString], &info)?;
        return Err("fail");
    }
    Ok(())
}

pub(crate) fn verifyDimensions(
    mut dimensions: &metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
    mut component: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<()> {
    for mut d in &**dimensions {
        verifyDimension(metamodelica::AsArg::as_arg(&d), component)?;
    }
    Ok(())
}

pub(crate) fn verifyDimension(
    mut dimension: &metamodelica::Ref<Dimension::NFDimension>,
    mut component: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<()> {
    let () = (match &**dimension {
        Dimension::INTEGER {
            size: __dimension_size, ..
        } => {
            if __dimension_size.clone() < 0 {
                Error::addSourceMessage(
                    &(Error::NEGATIVE_DIMENSION_INDEX.clone()),
                    list![
                        ArcStr::from(::std::format!("{}", __dimension_size.clone())),
                        NFInstNode::InstNode::name(component)?
                    ],
                    &(NFInstNode::InstNode::info(component)),
                )?;
                return Err("fail");
            }
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn updateVariability(
    mut var: metamodelica::Ref<Variable::NFVariable>,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = var;
    let mut v: Variability;
    if var.attributes.variability.clone() == Variability::PARAMETER.clone() {
        v = Component::variability(&(NFInstNode::InstNode::component(&(ComponentRef::node(&var.name)?))?))?;
        if v < Variability::PARAMETER.clone() {
            var = Variable::setVariability(var, v);
        }
    }
    Ok(var)
}

pub(crate) fn evaluateIfWithConnects(
    mut eql: &metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut outEql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    for mut eq in &**eql {
        outEql = evaluateIfWithConnects2(eq.clone(), outEql)?;
    }
    outEql = metamodelica::Dangerous::listReverseInPlace(outEql);
    Ok(outEql)
}

pub(crate) fn evaluateIfWithConnects2(
    mut eq: metamodelica::Ref<Equation::NFEquation>,
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = equations;
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut var: Variability;
    let mut eql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut target: metamodelica::Ref<Ceval::EvalTarget::EvalTarget>;
    let mut bl: metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>> = metamodelica::nil();
    equations = (match &*eq {
        Equation::IF {
            branches: __eq_branches,
            scope: __eq_scope,
            source: __eq_source,
        } if (Equation::contains(
            eq.clone(),
            &move |__a0: metamodelica::Ref<Equation::NFEquation>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(Equation::isConnect(&__a0))
            },
        )? || Equation::containsExp(
            &eq,
            &({
                let __pe_b1: Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static,
                > = (std::sync::Arc::new(move |__a0: metamodelica::Ref<Expression::NFExpression>| {
                    Expression::isConnectionCall(&__a0)
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static,
                    >);
                move |__pe_a0| Expression::contains(__pe_a0, &*__pe_b1)
            }),
        )?) =>
        {
            target = Ceval::EvalTarget::new(Equation::info(&eq), NFInstContext::NO_CONTEXT.clone(), None);
            for mut branch in &*__eq_branches.clone() {
                let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(branch.clone()) {
                    Deref @ Equation::Branch::BRANCH { condition: __pa0, conditionVar: __pa1, body: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                cond = metamodelica::Own::own(__pa0);
                var = metamodelica::Own::own(__pa1);
                eql = metamodelica::Own::own(__pa2);
                if var <= Variability::STRUCTURAL_PARAMETER.clone() {
                    if Expression::isPure(&cond)? {
                        Structural::markExp(&cond)?;
                        cond = Ceval::evalExp(cond, &target)?;
                    }
                    if !(Expression::isBoolean(&cond)) {
                        Error::addInternalError(
                            {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!(
                                    "Failed to evaluate branch condition in if equation containing connect equations: `"
                                ));
                                __mm_s.push_str(&*Expression::toString(cond.clone())?);
                                __mm_s.push_str(&*literal!("`"));
                                ArcStr::from(__mm_s)
                            },
                            Equation::info(&eq),
                        )?;
                        return Err("fail");
                    }
                }
                if Expression::isTrue(&cond) {
                    if (bl).is_empty() {
                        eql = evaluateIfWithConnects(&eql)?;
                        equations = listAppend(eql, equations);
                        bl = metamodelica::nil();
                    } else {
                        bl = metamodelica::cons(Equation::makeBranch(cond, eql, var), bl);
                    }
                    break;
                } else if !(Expression::isFalse(&cond)) {
                    bl = metamodelica::cons(Equation::makeBranch(cond, eql, var), bl);
                }
            }
            if !((bl).is_empty()) {
                equations = metamodelica::cons(
                    metamodelica::Ref::new(Equation::NFEquation::IF {
                        branches: metamodelica::Dangerous::listReverseInPlace(bl),
                        scope: __eq_scope.clone(),
                        source: __eq_source.clone(),
                    }),
                    equations,
                );
            }
            equations
        }
        _ => metamodelica::cons(eq.clone(), equations),
    });
    Ok(equations)
}

pub(crate) fn checkDeletedVarRefs(
    mut flatModel: &metamodelica::Ref<FlatModel::NFFlatModel>,
    mut deletedVars: DeletedVariables,
    mut settings: FlattenSettings,
) -> Result<()> {
    for mut var in &*flatModel.variables.clone() {
        checkDeletedVarRefsInVar(metamodelica::AsArg::as_arg(&var), deletedVars.clone(), settings)?;
    }
    for mut eq in &*flatModel.equations.clone() {
        checkDeletedVarRefsInEq(metamodelica::AsArg::as_arg(&eq), deletedVars.clone(), settings)?;
    }
    for mut eq in &*flatModel.initialEquations.clone() {
        checkDeletedVarRefsInEq(metamodelica::AsArg::as_arg(&eq), deletedVars.clone(), settings)?;
    }
    for mut alg in &*flatModel.algorithms.clone() {
        checkDeletedVarRefsInAlg(metamodelica::AsArg::as_arg(&alg), deletedVars.clone(), settings)?;
    }
    for mut alg in &*flatModel.initialAlgorithms.clone() {
        checkDeletedVarRefsInAlg(metamodelica::AsArg::as_arg(&alg), deletedVars.clone(), settings)?;
    }
    Ok(())
}

pub(crate) fn checkDeletedVarRefsInVar(
    mut var: &metamodelica::Ref<Variable::NFVariable>,
    mut deletedVars: DeletedVariables,
    mut settings: FlattenSettings,
) -> Result<()> {
    Variable::applyExpShallow(
        var,
        &({
            let __pe_b1 = deletedVars;
            let __pe_b2 = settings;
            let __pe_b3 = var.info.clone();
            move |__pe_a0| checkDeletedVarRefsInExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), &__pe_b3)
        }),
    )?;
    Ok(())
}

pub(crate) fn checkDeletedVarRefsInExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut deletedVars: DeletedVariables,
    mut settings: FlattenSettings,
    mut info: &SourceInfo,
) -> Result<()> {
    Expression::apply(
        exp,
        &({
            let __pe_b1 = deletedVars;
            let __pe_b2 = settings;
            let __pe_b3 = info.clone();
            move |__pe_a0| checkDeletedVarRefsInExp_traverser(&__pe_a0, __pe_b1.clone(), __pe_b2.clone(), &__pe_b3)
        }),
    )?;
    Ok(())
}

pub(crate) fn checkDeletedVarRefsInExp_traverser(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut deletedVars: DeletedVariables,
    mut settings: FlattenSettings,
    mut info: &SourceInfo,
) -> Result<()> {
    let () = (match &**exp {
        Expression::CREF { cref: __exp_cref, .. } if (isDeletedCref(__exp_cref.clone(), deletedVars.clone())?) => {
            Error::addSourceMessage(
                &(Error::INVALID_DELETED_COMPONENT_CONTEXT.clone()),
                list![ComponentRef::toString(metamodelica::AsArg::as_arg(&__exp_cref))?],
                info,
            )?;
            if !(settings.relaxedErrorChecking.clone()) {
                return Err("fail");
            }
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn checkDeletedVarRefsInEq(
    mut eq: &metamodelica::Ref<Equation::NFEquation>,
    mut deletedVars: DeletedVariables,
    mut settings: FlattenSettings,
) -> Result<()> {
    Equation::applyExp(
        eq,
        &({
            let __pe_b1 = deletedVars;
            let __pe_b2 = settings;
            let __pe_b3 = Equation::info(eq);
            move |__pe_a0| checkDeletedVarRefsInExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), &__pe_b3)
        }),
    )?;
    Ok(())
}

pub(crate) fn checkDeletedVarRefsInAlg(
    mut alg: &metamodelica::Ref<Algorithm::NFAlgorithm>,
    mut deletedVars: DeletedVariables,
    mut settings: FlattenSettings,
) -> Result<()> {
    for mut stmt in &*alg.statements.clone() {
        Statement::applyExp(
            metamodelica::AsArg::as_arg(&stmt),
            &({
                let __pe_b1 = deletedVars.clone();
                let __pe_b2 = settings;
                let __pe_b3 = Statement::info(metamodelica::AsArg::as_arg(&stmt));
                move |__pe_a0| checkDeletedVarRefsInExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), &__pe_b3)
            }),
        )?;
    }
    Ok(())
}
