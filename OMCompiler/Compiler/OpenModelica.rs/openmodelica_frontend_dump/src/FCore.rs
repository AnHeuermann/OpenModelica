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
use crate::AvlSetCR;
use crate::AvlTreePathFunction;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;
use openmodelica_util::Config;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::MutableWeak;

// ************************ FNode structures ***************************
// ************************ FNode structures ***************************
// ************************ FNode structures ***************************
// ************************ FNode structures ***************************
/// an identifier is just a string
pub type Name = ArcStr;

/// list of names
pub type Names = metamodelica::List<ArcStr>;

pub type Import = Absyn::Import;

pub type Id = i32;

pub type Seq = i32;

pub type Next = i32;

pub static emptyImportTable: std::sync::LazyLock<ImportTable> = std::sync::LazyLock::new(|| ImportTable {
    hidden: false,
    qualifiedImports: metamodelica::nil(),
    unqualifiedImports: metamodelica::nil(),
});

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ImportTable {
    /// If true means that the imports are hidden.
    pub hidden: bool,
    pub qualifiedImports: metamodelica::List<Absyn::Import>,
    pub unqualifiedImports: metamodelica::List<Absyn::Import>,
}

impl metamodelica::gc::MMTrace for ImportTable {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.hidden, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.qualifiedImports, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.unqualifiedImports, __mmv)?;
        Ok(())
    }
}
impl Default for ImportTable {
    fn default() -> Self {
        Self {
            hidden: Default::default(),
            qualifiedImports: Default::default(),
            unqualifiedImports: Default::default(),
        }
    }
}

pub type IMPORT_TABLE = ImportTable;

/// one mutable slot; a node's identity is its cell
pub type Ref = Mutable::Mutable<metamodelica::Ref<Node>>;

/// a parent, held without owning it
pub type WeakRef = MutableWeak::MutableWeak<metamodelica::Ref<Node>>;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Node {
    /// node name, class/component/extends name, etc. see also *NodeName in above
    pub name: Name,
    /// Unique node id
    pub id: Id,
    /// A node can have several parents depending on the context.
    ///                         Held weakly: a parent owns its children, so owning the
    ///                         parent back would make the graph unreclaimable by
    ///                         reference counting. The graph is rooted at the top node,
    ///                         which keeps every parent alive while it is reachable.
    pub parents: WeakParents,
    /// List of uniquely named classes and variables
    pub children: Children,
    /// More data for this node, Class, Var, etc
    pub data: metamodelica::Ref<Data>,
}

impl metamodelica::gc::MMTrace for Node {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.id, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.parents, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.children, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.data, __mmv)?;
        Ok(())
    }
}
impl Default for Node {
    fn default() -> Self {
        Self {
            name: Default::default(),
            id: Default::default(),
            parents: Default::default(),
            children: Default::default(),
            data: Default::default(),
        }
    }
}

pub type N = Node;

/// Used to know where a modifier came from, for error reporting.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum ModScope {
    MS_COMPONENT { name: ArcStr },
    MS_EXTENDS { path: metamodelica::Ref<Absyn::Path> },
    MS_DERIVED { path: metamodelica::Ref<Absyn::Path> },
    MS_CLASS_EXTENDS { name: ArcStr },
    MS_CONSTRAINEDBY { path: metamodelica::Ref<Absyn::Path> },
}
impl metamodelica::gc::MMTrace for ModScope {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ModScope::MS_COMPONENT { name } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                Ok(())
            }
            ModScope::MS_EXTENDS { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            ModScope::MS_DERIVED { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
            ModScope::MS_CLASS_EXTENDS { name } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                Ok(())
            }
            ModScope::MS_CONSTRAINEDBY { path } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::ModScope::{MS_CLASS_EXTENDS, MS_COMPONENT, MS_CONSTRAINEDBY, MS_DERIVED, MS_EXTENDS};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Data {
    /// top
    TOP,
    IT {
        /// instantiated component
        i: metamodelica::Ref<DAE::Var>,
    },
    /// import
    IM {
        /// imports
        i: ImportTable,
    },
    /// class
    CL {
        e: metamodelica::Ref<SCode::Element>,
        pre: DAE::Prefix,
        /// modification
        r#mod: metamodelica::Ref<DAE::Mod>,
        /// usedefined, builtin, basic type
        kind: Kind,
        /// if it is untyped, typed or fully instantiated (dae)
        status: Status,
    },
    /// component
    CO {
        e: metamodelica::Ref<SCode::Element>,
        /// modification
        r#mod: metamodelica::Ref<DAE::Mod>,
        /// usedefined, builtin, basic type
        kind: Kind,
        /// if it is untyped, typed or fully instantiated (dae)
        status: Status,
    },
    /// extends
    EX {
        e: metamodelica::Ref<SCode::Element>,
        /// modification
        r#mod: metamodelica::Ref<DAE::Mod>,
    },
    /// units
    DU {
        els: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    },
    /// function type nodes
    FT {
        /// list since several types with the same name can exist in the same scope (overloading)
        tys: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    },
    /// algorithm section
    AL {
        /// al or ial (initial)
        name: Name,
        a: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>,
    },
    /// equation section
    EQ {
        /// eq or ieq (initial)
        name: Name,
        e: metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    },
    /// optimization
    OT {
        constrainLst: metamodelica::List<SCode::ConstraintSection>,
        clsAttrs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    },
    /// external declaration
    ED { ed: metamodelica::Ref<SCode::ExternalDecl> },
    /// for iterators scope
    FS {
        fis: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>>,
    },
    /// for iterator
    FI { fi: metamodelica::Ref<Absyn::ForIterator> },
    /// match scope
    MS { e: metamodelica::Ref<Absyn::Exp> },
    /// mod
    MO { m: metamodelica::Ref<SCode::Mod> },
    /// binding, condition, array dim, etc
    EXP {
        /// what is the expression for
        name: ArcStr,
        e: metamodelica::Ref<Absyn::Exp>,
    },
    /// component reference
    CR { r: metamodelica::Ref<Absyn::ComponentRef> },
    /// dimensions
    DIMS {
        /// what are the dimensions for, type or component
        name: ArcStr,
        dims: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    },
    /// constrainedby class
    CC {
        cc: metamodelica::Ref<SCode::ConstrainClass>,
    },
    /// reference node
    REF { target: Scope },
    /// no data
    ND { scopeType: Option<ScopeType> },
    /// version node, contains the node that decided the generation of the clone
    VR {
        source: Scope,
        p: DAE::Prefix,
        m: metamodelica::Ref<DAE::Mod>,
        scopeType: Option<ScopeType>,
    },
    /// an assertion node, to be used in places
    ///    where we want to assert things in the graph.
    ///    for example if we looked up A.B from A.B.C.D
    ///    but could not find C then we add an assertion
    ///    node. we have just a message here but might
    ///    add new info later on.
    ASSERT { message: ArcStr },
    /// status node
    STATUS { isInstantiating: bool },
}
impl metamodelica::gc::MMTrace for Data {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Data::TOP => Ok(()),
            Data::IT { i } => {
                metamodelica::gc::MMTrace::mm_accept(i, __mmv)?;
                Ok(())
            }
            Data::IM { i } => {
                metamodelica::gc::MMTrace::mm_accept(i, __mmv)?;
                Ok(())
            }
            Data::CL {
                e,
                pre,
                r#mod,
                kind,
                status,
            } => {
                metamodelica::gc::MMTrace::mm_accept(e, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(pre, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(r#mod, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(kind, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(status, __mmv)?;
                Ok(())
            }
            Data::CO { e, r#mod, kind, status } => {
                metamodelica::gc::MMTrace::mm_accept(e, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(r#mod, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(kind, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(status, __mmv)?;
                Ok(())
            }
            Data::EX { e, r#mod } => {
                metamodelica::gc::MMTrace::mm_accept(e, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(r#mod, __mmv)?;
                Ok(())
            }
            Data::DU { els } => {
                metamodelica::gc::MMTrace::mm_accept(els, __mmv)?;
                Ok(())
            }
            Data::FT { tys } => {
                metamodelica::gc::MMTrace::mm_accept(tys, __mmv)?;
                Ok(())
            }
            Data::AL { name, a } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(a, __mmv)?;
                Ok(())
            }
            Data::EQ { name, e } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(e, __mmv)?;
                Ok(())
            }
            Data::OT { constrainLst, clsAttrs } => {
                metamodelica::gc::MMTrace::mm_accept(constrainLst, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(clsAttrs, __mmv)?;
                Ok(())
            }
            Data::ED { ed } => {
                metamodelica::gc::MMTrace::mm_accept(ed, __mmv)?;
                Ok(())
            }
            Data::FS { fis } => {
                metamodelica::gc::MMTrace::mm_accept(fis, __mmv)?;
                Ok(())
            }
            Data::FI { fi } => {
                metamodelica::gc::MMTrace::mm_accept(fi, __mmv)?;
                Ok(())
            }
            Data::MS { e } => {
                metamodelica::gc::MMTrace::mm_accept(e, __mmv)?;
                Ok(())
            }
            Data::MO { m } => {
                metamodelica::gc::MMTrace::mm_accept(m, __mmv)?;
                Ok(())
            }
            Data::EXP { name, e } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(e, __mmv)?;
                Ok(())
            }
            Data::CR { r } => {
                metamodelica::gc::MMTrace::mm_accept(r, __mmv)?;
                Ok(())
            }
            Data::DIMS { name, dims } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dims, __mmv)?;
                Ok(())
            }
            Data::CC { cc } => {
                metamodelica::gc::MMTrace::mm_accept(cc, __mmv)?;
                Ok(())
            }
            Data::REF { target } => {
                metamodelica::gc::MMTrace::mm_accept(target, __mmv)?;
                Ok(())
            }
            Data::ND { scopeType } => {
                metamodelica::gc::MMTrace::mm_accept(scopeType, __mmv)?;
                Ok(())
            }
            Data::VR {
                source,
                p,
                m,
                scopeType,
            } => {
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(p, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(m, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scopeType, __mmv)?;
                Ok(())
            }
            Data::ASSERT { message } => {
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                Ok(())
            }
            Data::STATUS { isInstantiating } => {
                metamodelica::gc::MMTrace::mm_accept(isInstantiating, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Data {
    pub fn interned_TOP() -> metamodelica::Ref<Data> {
        thread_local! {
            static INTERNED: metamodelica::Ref<Data> = metamodelica::Ref::new(Data::TOP);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_TOP() -> metamodelica::Ref<Data> {
    Data::interned_TOP()
}
impl Default for Data {
    fn default() -> Self {
        Self::TOP
    }
}
pub use self::Data::{
    AL, ASSERT, CC, CL, CO, CR, DIMS, DU, ED, EQ, EX, EXP, FI, FS, FT, IM, IT, MO, MS, ND, OT, REF, STATUS, TOP, VR,
};

pub type Refs = metamodelica::List<Mutable::Mutable<metamodelica::Ref<Node>>>;

/// as handed out by FNode.parents, already upgraded
pub type Parents = metamodelica::List<Mutable::Mutable<metamodelica::Ref<Node>>>;

/// as stored in a node; see Node.N.parents
pub type WeakParents = metamodelica::List<MutableWeak::MutableWeak<metamodelica::Ref<Node>>>;

pub type Scope = metamodelica::List<Mutable::Mutable<metamodelica::Ref<Node>>>;

pub type Children = metamodelica::Ref<RefTree::Tree>;

thread_local! { static __emptyScope_TLS: metamodelica::List<Mutable::Mutable<metamodelica::Ref<Node>>> = metamodelica::nil(); }
pub(crate) fn emptyScope() -> metamodelica::List<Mutable::Mutable<metamodelica::Ref<Node>>> {
    __emptyScope_TLS.with(|__t| __t.clone())
}

pub mod RefTree {
    use super::*;
    pub type Key = ArcStr;

    pub type Value = Mutable::Mutable<metamodelica::Ref<Node>>;

    pub(crate) fn keyStr(mut inKey: Key) -> ArcStr {
        let mut outString: ArcStr;
        outString = inKey;
        outString
    }

    pub(crate) fn valueStr(mut inValue: Value) -> ArcStr {
        let mut outString: ArcStr;
        let __arc1 = Mutable::access(inValue);
        let Node { name: __pa0, .. } = &*__arc1;
        outString = metamodelica::Own::own(__pa0);
        outString
    }

    pub(crate) fn keyCompare(mut inKey1: Key, mut inKey2: Key) -> i32 {
        let mut outResult: i32;
        outResult = stringCompare(&inKey1, &inKey2);
        outResult
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

    pub type ValueNode = ArcStr;

    pub fn add(
        mut inTree: metamodelica::Ref<Tree>,
        mut inKey: &Key,
        mut inValue: Value,
        mut conflictFunc: &dyn ::std::ops::Fn(
            Mutable::Mutable<metamodelica::Ref<Node>>,
            Mutable::Mutable<metamodelica::Ref<Node>>,
            ArcStr,
        ) -> Result<Mutable::Mutable<metamodelica::Ref<Node>>>,
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
                key_comp = keyCompare(inKey.clone(), key.clone());
                if key_comp == -1 {
                    assign_variant_field!(tree => Tree::NODE; left = add(var_field!((*tree).left, Tree::NODE).clone(), inKey, inValue, conflictFunc)?);
                } else if key_comp == 1 {
                    assign_variant_field!(tree => Tree::NODE; right = add(var_field!((*tree).right, Tree::NODE).clone(), inKey, inValue, conflictFunc)?);
                } else {
                    value = conflictFunc(inValue, var_field!((*tree).value, Tree::NODE).clone(), key.clone())?;
                    if !(Mutable::referenceEq(&(var_field!((*tree).value, Tree::NODE).clone()), &(value.clone()))) {
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
                            value: inValue,
                        }),
                        right: crate::FCore::RefTree::Tree::interned_EMPTY(),
                    });
                } else if key_comp == 1 {
                    outTree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: crate::FCore::RefTree::Tree::interned_EMPTY(),
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
                    if !(Mutable::referenceEq(&(var_field!((*tree).value, Tree::LEAF).clone()), &(value.clone()))) {
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

    pub(crate) fn addConflictKeep(mut newValue: Value, mut oldValue: Value, mut key: &Key) -> Value {
        let mut value: Value = oldValue;
        value
    }

    pub fn addConflictReplace(mut newValue: Value, mut oldValue: Value, mut key: &Key) -> Value {
        let mut value: Value = newValue;
        value
    }

    pub(crate) fn addList(
        mut tree: metamodelica::Ref<Tree>,
        mut inValues: &metamodelica::List<(ArcStr, Mutable::Mutable<metamodelica::Ref<Node>>)>,
        mut conflictFunc: &dyn ::std::ops::Fn(
            Mutable::Mutable<metamodelica::Ref<Node>>,
            Mutable::Mutable<metamodelica::Ref<Node>>,
            ArcStr,
        ) -> Result<Mutable::Mutable<metamodelica::Ref<Node>>>,
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
        mut r#fn: &dyn ::std::ops::Fn(
            Option<Mutable::Mutable<metamodelica::Ref<Node>>>,
        ) -> Result<Mutable::Mutable<metamodelica::Ref<Node>>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        pub type UpdateFn = std::sync::Arc<
            dyn ::std::ops::Fn(Option<Mutable::Mutable<metamodelica::Ref<Node>>>) -> Result<Value> + 'static,
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
                        right: crate::FCore::RefTree::Tree::interned_EMPTY(),
                    });
                } else if key_comp == 1 {
                    new_tree = metamodelica::Ref::new(Tree::NODE {
                        key: var_field!((*tree).key, Tree::LEAF).clone(),
                        value: var_field!((*tree).value, Tree::LEAF).clone(),
                        height: 2,
                        left: crate::FCore::RefTree::Tree::interned_EMPTY(),
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

    pub fn fold<'__b, FT: Clone + 'static + metamodelica::gc::MMTrace>(
        mut inTree: &'__b metamodelica::Ref<Tree>,
        mut inFunc: &'__b dyn ::std::ops::Fn(ArcStr, Mutable::Mutable<metamodelica::Ref<Node>>, FT) -> Result<FT>,
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
        mut foldFunc: &dyn ::std::ops::Fn(ArcStr, Mutable::Mutable<metamodelica::Ref<Node>>, FT) -> Result<(FT, bool)>,
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
        mut foldFunc: &dyn ::std::ops::Fn(ArcStr, Mutable::Mutable<metamodelica::Ref<Node>>, FT1, FT2) -> Result<(FT1, FT2)>,
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
        mut func: &dyn ::std::ops::Fn(ArcStr, Mutable::Mutable<metamodelica::Ref<Node>>) -> Result<()>,
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
        mut inValues: &metamodelica::List<(ArcStr, Mutable::Mutable<metamodelica::Ref<Node>>)>,
        mut conflictFunc: &dyn ::std::ops::Fn(
            Mutable::Mutable<metamodelica::Ref<Node>>,
            Mutable::Mutable<metamodelica::Ref<Node>>,
            ArcStr,
        ) -> Result<Mutable::Mutable<metamodelica::Ref<Node>>>,
    ) -> Result<metamodelica::Ref<Tree>> {
        let mut tree: metamodelica::Ref<Tree> = crate::FCore::RefTree::Tree::interned_EMPTY();
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
    ) -> Option<Mutable::Mutable<metamodelica::Ref<Node>>> {
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

    pub fn isEmpty(mut tree: &metamodelica::Ref<Tree>) -> bool {
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
            Mutable::Mutable<metamodelica::Ref<Node>>,
            Mutable::Mutable<metamodelica::Ref<Node>>,
            ArcStr,
        ) -> Result<Mutable::Mutable<metamodelica::Ref<Node>>>,
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

    pub fn listValues<'__b>(
        mut tree: &'__b metamodelica::Ref<Tree>,
        mut lst: metamodelica::List<Mutable::Mutable<metamodelica::Ref<Node>>>,
    ) -> metamodelica::List<Mutable::Mutable<metamodelica::Ref<Node>>> {
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
        mut inFunc: &dyn ::std::ops::Fn(
            ArcStr,
            Mutable::Mutable<metamodelica::Ref<Node>>,
        ) -> Result<Mutable::Mutable<metamodelica::Ref<Node>>>,
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
                    || !(Mutable::referenceEq(&(value.clone()), &(new_value.clone())))
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
                if !(Mutable::referenceEq(&(value.clone()), &(new_value.clone()))) {
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
            ArcStr,
            Mutable::Mutable<metamodelica::Ref<Node>>,
            FT,
        ) -> Result<(Mutable::Mutable<metamodelica::Ref<Node>>, FT)>,
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
                    || !(Mutable::referenceEq(&(value.clone()), &(new_value.clone())))
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
                if !(Mutable::referenceEq(&(value.clone()), &(new_value.clone()))) {
                    assign_variant_field!(outTree => Tree::LEAF; value = new_value);
                }
                outTree
            }
            _ => inTree,
        });
        Ok((outTree, outResult))
    }

    pub fn new() -> metamodelica::Ref<Tree> {
        let mut outTree: metamodelica::Ref<Tree> = crate::FCore::RefTree::Tree::interned_EMPTY();
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
                node = setTreeLeftRight(outNode, __outNode_left.clone(), crate::FCore::RefTree::Tree::interned_EMPTY())?;
                setTreeLeftRight(child.clone(), node, crate::FCore::RefTree::Tree::interned_EMPTY())?
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
                node = setTreeLeftRight(outNode, crate::FCore::RefTree::Tree::interned_EMPTY(), __outNode_right.clone())?;
                setTreeLeftRight(child.clone(), crate::FCore::RefTree::Tree::interned_EMPTY(), node)?
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
        mut lst: metamodelica::List<(ArcStr, Mutable::Mutable<metamodelica::Ref<Node>>)>,
    ) -> metamodelica::List<(ArcStr, Mutable::Mutable<metamodelica::Ref<Node>>)> {
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
            &move |__a0: Mutable::Mutable<metamodelica::Ref<Node>>,
                   __a1: Mutable::Mutable<metamodelica::Ref<Node>>,
                   __a2: ArcStr|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(addConflictReplace(__a0, __a1, &__a2))
            },
        )?;
        Ok(outTree)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Kind {
    USERDEFINED,
    BUILTIN,
    BASIC_TYPE,
}
impl metamodelica::gc::MMTrace for Kind {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Kind::USERDEFINED => Ok(()),
            Kind::BUILTIN => Ok(()),
            Kind::BASIC_TYPE => Ok(()),
        }
    }
}
pub use self::Kind::{BASIC_TYPE, BUILTIN, USERDEFINED};

/// Used to distinguish between different phases of the instantiation of a component
/// A component is first added to environment untyped. It can thereafter be instantiated to get its type
/// and finally instantiated to produce the DAE. These three states are indicated by this datatype.
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Status {
    /// Untyped variables, initially added to env
    VAR_UNTYPED,
    /// Typed variables, when instantiation to get type has been performed
    VAR_TYPED,
    /// Typed variables that also have been instantiated to generate dae. Required to distinguish
    ///                  between typed variables without DAE to know when to skip multiply declared dae elements
    VAR_DAE,
    /// A conditional variable that was deleted.
    VAR_DELETED,
    /// just added to the env
    CLS_UNTYPED,
    /// partially instantiated
    CLS_PARTIAL,
    /// fully instantiated
    CLS_FULL,
    /// a class that was generated for a component
    CLS_INSTANCE { instanceOf: ArcStr },
}
impl metamodelica::gc::MMTrace for Status {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Status::VAR_UNTYPED => Ok(()),
            Status::VAR_TYPED => Ok(()),
            Status::VAR_DAE => Ok(()),
            Status::VAR_DELETED => Ok(()),
            Status::CLS_UNTYPED => Ok(()),
            Status::CLS_PARTIAL => Ok(()),
            Status::CLS_FULL => Ok(()),
            Status::CLS_INSTANCE { instanceOf } => {
                metamodelica::gc::MMTrace::mm_accept(instanceOf, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Status {
    fn default() -> Self {
        Self::VAR_UNTYPED
    }
}
pub use self::Status::{
    CLS_FULL, CLS_INSTANCE, CLS_PARTIAL, CLS_UNTYPED, VAR_DAE, VAR_DELETED, VAR_TYPED, VAR_UNTYPED,
};

// ************************ FVisit structures ***************************
// ************************ FVisit structures ***************************
// ************************ FVisit structures ***************************
// ************************ FVisit structures ***************************
/// Visit Node Info
/// Visit Node Info
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Visit {
    /// which node it is
    pub r#ref: Ref,
    /// order in which was visited
    pub seq: Seq,
}

impl metamodelica::gc::MMTrace for Visit {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.r#ref, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.seq, __mmv)?;
        Ok(())
    }
}
impl Default for Visit {
    fn default() -> Self {
        Self {
            r#ref: Default::default(),
            seq: Default::default(),
        }
    }
}

pub type VN = Visit;

/// Visited structure is an AvlTree Id <-> Visit
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Visited {
    pub tree: metamodelica::Ref<VAvlTree>,
    /// the next visit node id
    pub next: Next,
}

impl metamodelica::gc::MMTrace for Visited {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.tree, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.next, __mmv)?;
        Ok(())
    }
}
impl Default for Visited {
    fn default() -> Self {
        Self {
            tree: Default::default(),
            next: Default::default(),
        }
    }
}

pub type V = Visited;

pub type VAvlKey = i32;

pub type VAvlValue = Visit;

/// The binary tree data structure for visited
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct VAvlTree {
    /// Value
    pub value: Option<VAvlTreeValue>,
    /// heigth of tree, used for balancing
    pub height: i32,
    /// left subtree
    pub left: Option<metamodelica::Ref<VAvlTree>>,
    /// right subtree
    pub right: Option<metamodelica::Ref<VAvlTree>>,
}

impl metamodelica::gc::MMTrace for VAvlTree {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.value, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.height, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.left, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.right, __mmv)?;
        Ok(())
    }
}
impl Default for VAvlTree {
    fn default() -> Self {
        Self {
            value: Default::default(),
            height: Default::default(),
            left: Default::default(),
            right: Default::default(),
        }
    }
}

pub type VAVLTREENODE = VAvlTree;

/// Each node in the binary tree can have a value associated with it.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct VAvlTreeValue {
    /// Key
    pub key: VAvlKey,
    /// Value
    pub value: VAvlValue,
}

impl metamodelica::gc::MMTrace for VAvlTreeValue {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.key, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.value, __mmv)?;
        Ok(())
    }
}
impl Default for VAvlTreeValue {
    fn default() -> Self {
        Self {
            key: Default::default(),
            value: Default::default(),
        }
    }
}

pub type VAVLTREEVALUE = VAvlTreeValue;

thread_local! { static __emptyVAvlTree_TLS: metamodelica::Ref<VAvlTree> = metamodelica::Ref::new(VAvlTree { value: None, height: 0, left: None, right: None }); }
pub fn emptyVAvlTree() -> metamodelica::Ref<VAvlTree> {
    __emptyVAvlTree_TLS.with(|__t| __t.clone())
}

// ************************ FGraph structures ***************************
// ************************ FGraph structures ***************************
// ************************ FGraph structures ***************************
// ************************ FGraph structures ***************************
pub static dummyTopModel: std::sync::LazyLock<metamodelica::Ref<Absyn::Path>> = std::sync::LazyLock::new(|| {
    metamodelica::Ref::new(Absyn::Path::IDENT {
        name: literal!("$EMPTY"),
    })
});

pub(crate) static dummyExtra: std::sync::LazyLock<Extra> = std::sync::LazyLock::new(|| Extra {
    topModel: dummyTopModel.clone(),
});

pub(crate) const recordConstructorSuffix: &'static str = "$recordconstructor";

pub const forScopeName: &'static str = "$for loop scope$";

pub const forIterScopeName: &'static str = "$foriter loop scope$";

pub const parForScopeName: &'static str = "$pafor loop scope$";

pub const parForIterScopeName: &'static str = "$parforiter loop scope$";

pub const matchScopeName: &'static str = "$match scope$";

pub const caseScopeName: &'static str = "$case scope$";

pub const patternTypeScope: &'static str = "$pattern type scope$";

pub static implicitScopeNames: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        arcstr::literal!(forScopeName),
        arcstr::literal!(forIterScopeName),
        arcstr::literal!(parForScopeName),
        arcstr::literal!(parForIterScopeName),
        arcstr::literal!(matchScopeName),
        arcstr::literal!(caseScopeName),
        arcstr::literal!(patternTypeScope)
    ]
});

/// propagate more info into env if needed
/// propagate more info into env if needed
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Extra {
    pub topModel: metamodelica::Ref<Absyn::Path>,
}

impl metamodelica::gc::MMTrace for Extra {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.topModel, __mmv)?;
        Ok(())
    }
}
impl Default for Extra {
    fn default() -> Self {
        Self {
            topModel: Default::default(),
        }
    }
}

pub type EXTRA = Extra;

/// graph
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Graph {
    /// graph
    G {
        /// the top node
        top: Top,
        /// current scope
        scope: Scope,
    },
    /// empty graph
    EG { name: Name },
}
impl metamodelica::gc::MMTrace for Graph {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Graph::G { top, scope } => {
                metamodelica::gc::MMTrace::mm_accept(top, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                Ok(())
            }
            Graph::EG { name } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Graph {
    fn default() -> Self {
        Self::EG {
            name: Default::default(),
        }
    }
}
pub use self::Graph::{EG, G};

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Top {
    /// name of the graph
    pub name: Name,
    /// the top node
    pub node: Ref,
    /// extra information
    pub extra: Extra,
}

impl metamodelica::gc::MMTrace for Top {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.node, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.extra, __mmv)?;
        Ok(())
    }
}
impl Default for Top {
    fn default() -> Self {
        Self {
            name: Default::default(),
            node: Default::default(),
            extra: Default::default(),
        }
    }
}

pub type GTOP = Top;

pub const firstId: i32 = 0;

// ************************ Cache structures ***************************
// ************************ Cache structures ***************************
// ************************ Cache structures ***************************
// ************************ Cache structures ***************************
pub type StructuralParameters = (
    metamodelica::Ref<AvlSetCR::Tree>,
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
);

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Cache {
    CACHE {
        /// and the initial environment
        initialGraph: Option<Graph>,
        /// set of Option<DAE.Function>; NONE() means instantiation started; SOME() means it's finished
        functions: Mutable::Mutable<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        /// ht of prefixed crefs and a stack of evaluated but not yet prefix crefs
        evaluatedParams: StructuralParameters,
        /// name of the model being instantiated
        modelName: metamodelica::Ref<Absyn::Path>,
    },
    /// no cache
    NO_CACHE,
}
impl metamodelica::gc::MMTrace for Cache {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Cache::CACHE {
                initialGraph,
                functions,
                evaluatedParams,
                modelName,
            } => {
                metamodelica::gc::MMTrace::mm_accept(initialGraph, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(functions, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(evaluatedParams, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(modelName, __mmv)?;
                Ok(())
            }
            Cache::NO_CACHE => Ok(()),
        }
    }
}
impl Default for Cache {
    fn default() -> Self {
        Self::NO_CACHE
    }
}
pub use self::Cache::{CACHE, NO_CACHE};

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum ScopeType {
    FUNCTION_SCOPE,
    CLASS_SCOPE,
    PARALLEL_SCOPE,
}
impl metamodelica::gc::MMTrace for ScopeType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ScopeType::FUNCTION_SCOPE => Ok(()),
            ScopeType::CLASS_SCOPE => Ok(()),
            ScopeType::PARALLEL_SCOPE => Ok(()),
        }
    }
}
pub use self::ScopeType::{CLASS_SCOPE, FUNCTION_SCOPE, PARALLEL_SCOPE};

// ************************ functions ***************************
pub fn next(mut inext: Next) -> Next {
    let mut onext: Next;
    onext = inext + 1;
    onext
}

pub fn emptyCache() -> Cache {
    let mut cache: Cache;
    let mut instFuncs: Mutable::Mutable<metamodelica::Ref<AvlTreePathFunction::Tree>>;
    let mut ht: StructuralParameters;
    instFuncs = Mutable::create(crate::AvlTreePathFunction::Tree::interned_EMPTY());
    ht = (crate::AvlSetCR::Tree::interned_EMPTY(), metamodelica::nil());
    cache = Cache::CACHE {
        initialGraph: None,
        functions: instFuncs,
        evaluatedParams: ht,
        modelName: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("##UNDEFINED##"),
        }),
    };
    cache
}

pub fn noCache() -> Cache {
    let mut cache: Cache;
    cache = crate::FCore::Cache::NO_CACHE;
    cache
}

pub fn addEvaluatedCref(
    mut cache: Cache,
    mut var: SCode::Variability,
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
) -> Cache {
    let mut ocache: Cache;
    ocache = (::match_deref::match_deref! { match &((cache.clone(), var)) {
        (Cache::CACHE { initialGraph, functions, evaluatedParams: (ht, Deref @ metamodelica::ListNode::Cons { head: crs, tail: st }), modelName: p }, SCode::Variability::PARAM { .. }) => {
            Cache::CACHE { initialGraph: initialGraph.clone(), functions: functions.clone(), evaluatedParams: (ht.clone(), metamodelica::cons(metamodelica::cons(cr, crs.clone()), st.clone())), modelName: p.clone() }
        },
        (Cache::CACHE { initialGraph, functions, evaluatedParams: (ht, Deref @ metamodelica::ListNode::Nil), modelName: p }, SCode::Variability::PARAM { .. }) => {
            Cache::CACHE { initialGraph: initialGraph.clone(), functions: functions.clone(), evaluatedParams: (ht.clone(), metamodelica::cons(list![cr], metamodelica::nil())), modelName: p.clone() }
        },
        _ => {
            cache
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ocache
}

pub fn getEvaluatedParams(mut cache: Cache) -> Result<metamodelica::Ref<AvlSetCR::Tree>> {
    let mut ht: metamodelica::Ref<AvlSetCR::Tree>;
    let Cache::CACHE {
        evaluatedParams: (__pa0, _),
        ..
    } = (cache)
    else {
        return Err("pattern mismatch");
    };
    ht = metamodelica::Own::own(__pa0);
    Ok(ht)
}

pub(crate) fn printNumStructuralParameters(mut cache: Cache) -> Result<()> {
    let mut crs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let __pa0 = ::match_deref::match_deref! { match &(cache) {
        Cache::CACHE { evaluatedParams: (_, Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ }), .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    crs = metamodelica::Own::own(__pa0);
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("printNumStructuralParameters: "));
        __mm_s.push_str(&*intString(((crs).len() as i32)));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub fn setCacheClassName(mut inCache: Cache, mut p: metamodelica::Ref<Absyn::Path>) -> Cache {
    let mut outCache: Cache;
    outCache = (match inCache.clone() {
        Cache::CACHE {
            initialGraph: mut igraph,
            functions: mut ef,
            evaluatedParams: mut ht,
            modelName: _,
        } => Cache::CACHE {
            initialGraph: igraph.clone(),
            functions: ef.clone(),
            evaluatedParams: ht.clone(),
            modelName: p,
        },
        _ => inCache,
    });
    outCache
}

pub fn isImplicitScope(mut inName: Name) -> bool {
    let mut isImplicit: bool;
    isImplicit = 'mc: {
        let __mc_input = inName;
        if let Ok(__v) = (|| -> Result<_> {
            let mut id = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(stringGet(&id, 1)? == 36)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(false)
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    isImplicit
}

pub fn getCachedInstFunc(mut inCache: &Cache, mut path: metamodelica::Ref<Absyn::Path>) -> Result<DAE::Function> {
    let mut func: DAE::Function;
    func = (match inCache.clone() {
        Cache::CACHE { functions: mut ef, .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(&(Mutable::access(ef.clone())), path)?) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            func = metamodelica::Own::own(__pa0);
            func
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(func)
}

pub fn checkCachedInstFuncGuard(mut inCache: &Cache, mut path: metamodelica::Ref<Absyn::Path>) -> Result<()> {
    let () = (match inCache.clone() {
        Cache::CACHE { functions: mut ef, .. } => {
            AvlTreePathFunction::get(&(Mutable::access(ef.clone())), path)?;
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

pub fn getFunctionTree(mut cache: &Cache) -> metamodelica::Ref<AvlTreePathFunction::Tree> {
    let mut ft: metamodelica::Ref<AvlTreePathFunction::Tree>;
    ft = (match cache.clone() {
        Cache::CACHE { functions: mut ef, .. } => Mutable::access(ef.clone()),
        _ => crate::AvlTreePathFunction::Tree::interned_EMPTY(),
    });
    ft
}

pub fn addCachedInstFuncGuard(mut cache: Cache, mut func: metamodelica::Ref<Absyn::Path>) -> Result<Cache> {
    let mut outCache: Cache;
    outCache = 'mc: {
        let __mc_input = (&cache, &*func);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    checkCachedInstFuncGuard(&cache, func.clone())?;
                    Ok(cache.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Cache::CACHE { functions: ef, .. }, Deref @ Absyn::Path::FULLYQUALIFIED { path: _ }) => {
                    Mutable::update(ef.clone(), AvlTreePathFunction::add(Mutable::access(ef.clone()), &func, None, &*((std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>)))?);
                    Ok(cache.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    Ok(cache.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCache)
}

pub fn addDaeFunction(mut inCache: Cache, mut funcs: &metamodelica::List<DAE::Function>) -> Result<Cache> {
    let mut outCache: Cache;
    outCache = (match inCache.clone() {
        Cache::CACHE {
            initialGraph: _,
            functions: mut ef,
            evaluatedParams: _,
            modelName: _,
        } => {
            Mutable::update(
                ef.clone(),
                AvlTreePathFunction::addDaeFunction(funcs, Mutable::access(ef.clone()))?,
            );
            inCache
        }
        _ => inCache,
    });
    Ok(outCache)
}

pub fn addDaeExtFunction(mut inCache: Cache, mut funcs: &metamodelica::List<DAE::Function>) -> Result<Cache> {
    let mut outCache: Cache;
    outCache = (match inCache.clone() {
        Cache::CACHE {
            initialGraph: _,
            functions: mut ef,
            evaluatedParams: _,
            modelName: _,
        } => {
            Mutable::update(
                ef.clone(),
                AvlTreePathFunction::addDaeExtFunction(funcs, Mutable::access(ef.clone()))?,
            );
            inCache
        }
        _ => inCache,
    });
    Ok(outCache)
}

pub fn setCachedFunctionTree(mut inCache: &Cache, mut inFunctions: metamodelica::Ref<AvlTreePathFunction::Tree>) -> () {
    let () = (match inCache.clone() {
        Cache::CACHE { .. } => {
            Mutable::update(var_field!(inCache.functions, Cache::CACHE).clone(), inFunctions);
            ()
        }
        _ => (),
    });
    ()
}

pub fn isTyped(mut is: &Status) -> bool {
    let mut b: bool;
    b = (match is.clone() {
        Status::VAR_UNTYPED { .. } => false,
        _ => true,
    });
    b
}

pub fn isDeletedComp(mut status: &Status) -> bool {
    let mut isDeleted: bool;
    isDeleted = (match status.clone() {
        Status::VAR_DELETED { .. } => true,
        _ => false,
    });
    isDeleted
}

pub fn getCachedInitialGraph(mut cache: &Cache) -> Result<Graph> {
    let mut g: Graph;
    g = (match cache.clone() {
        Cache::CACHE {
            initialGraph: Some(mut __esc_g),
            ..
        } => {
            g = __esc_g.clone();
            g
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(g)
}

pub fn setCachedInitialGraph(mut cache: Cache, mut g: Graph) -> Cache {
    let mut cache: Cache = cache;
    cache = (match cache.clone() {
        Cache::CACHE { .. } => {
            let __owned_variant_initialGraph_0 = Some(g);
            if let Cache::CACHE { initialGraph, .. } = &mut cache {
                *initialGraph = __owned_variant_initialGraph_0;
            } else {
                panic!("owned-variant field-assign: value held a different variant than Cache::CACHE");
            }
            cache
        }
        _ => cache,
    });
    cache
}

pub(crate) fn getRecordConstructorName(mut inName: Name) -> Result<Name> {
    let mut outName: Name;
    outName = if (Config::acceptMetaModelicaGrammar()?) {
        inName
    } else {
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*inName);
            __mm_s.push_str(&*arcstr::literal!(recordConstructorSuffix));
            ArcStr::from(__mm_s)
        }
    };
    Ok(outName)
}

pub(crate) fn getRecordConstructorPath(
    mut inPath: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    let mut lastId: Name;
    if Config::acceptMetaModelicaGrammar()? {
        outPath = inPath;
    } else {
        lastId = AbsynUtil::pathLastIdent(&inPath);
        lastId = getRecordConstructorName(lastId)?;
        outPath = AbsynUtil::pathSetLastIdent(&inPath, &lastId);
    }
    Ok(outPath)
}
