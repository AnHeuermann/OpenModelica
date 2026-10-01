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
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFConvertDAE as ConvertDAE;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFModifier::Modifier;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::AccessLevel;
use crate::NFPrefixes::Visibility;
use crate::NFRestriction as Restriction;
use crate::NFSections as Sections;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::Global;
use openmodelica_util::IOStream;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::MutableWeak;
use openmodelica_util_datatypes_basic::Pointer;
use openmodelica_util_datatypes_basic::PointerWeak;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum InstNodeType {
    /// An element with no specific characteristics.
    NORMAL_CLASS,
    /// A base class extended by another class.
    BASE_CLASS {
        /// The extending class, weakly; see
        ///      InstNode.CLASS_NODE.parentScope.
        parent: ScopeRef,
        /// The extends clause definition.
        definition: metamodelica::Ref<SCode::Element>,
        /// The original node type before the class was extended.
        ty: metamodelica::Ref<InstNodeType>,
    },
    /// A short class definition.
    DERIVED_CLASS {
        /// The base node type not considering that it's a derived class.
        ty: metamodelica::Ref<InstNodeType>,
    },
    /// A builtin element.
    BUILTIN_CLASS,
    /// The unnamed class containing all the top-level classes.
    TOP_SCOPE {
        annotationScope: metamodelica::Ref<InstNode::InstNode>,
        generatedInners: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::Ref<InstNode::InstNode>>>,
        /// The identity cells of every node under this scope.
        ///      A node refers to its own scope weakly, and a cref outlives the node value
        ///      it was made from, so the tree owns the cells rather than the values do.
        roots: MutableWeak::Roots,
    },
    /// The root of the instance tree, i.e. the class that the instantiation starts from.
    ROOT_CLASS {
        /// The parent of the class, e.g. when instantiating a function
        ///                     in a component where the component is the parent. Weakly:
        ///                     the class tree owns it.
        parent: ScopeRef,
        /// Used by getModelInstance to add context to instances.
        context: Option<metamodelica::Ref<Absyn::Path>>,
    },
    NORMAL_COMP,
    REDECLARED_COMP {
        /// The parent of the replaced component, weakly.
        parent: ScopeRef,
    },
    REDECLARED_CLASS {
        /// Weakly: the class tree owns it.
        parent: ScopeRef,
        originalType: metamodelica::Ref<InstNodeType>,
        originalNode: Option<metamodelica::Ref<InstNode::InstNode>>,
        /// instance level of the redeclare, see Inst.classConfidence
        confidence: i32,
    },
    /// A generated inner element due to a missing outer.
    GENERATED_INNER,
    /// An implicit scope that's ignored when e.g. constructing a scope path. Not
    ///     used by implicit scope nodes since those have no node type (they're
    ///     implicitly implicit), but by e.g. the annotation scope.
    IMPLICIT_SCOPE,
}
impl metamodelica::gc::MMTrace for InstNodeType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            InstNodeType::NORMAL_CLASS => Ok(()),
            InstNodeType::BASE_CLASS { parent, definition, ty } => {
                metamodelica::gc::MMTrace::mm_accept(parent, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(definition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            InstNodeType::DERIVED_CLASS { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            InstNodeType::BUILTIN_CLASS => Ok(()),
            InstNodeType::TOP_SCOPE {
                annotationScope,
                generatedInners,
                roots,
            } => {
                metamodelica::gc::MMTrace::mm_accept(annotationScope, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(generatedInners, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(roots, __mmv)?;
                Ok(())
            }
            InstNodeType::ROOT_CLASS { parent, context } => {
                metamodelica::gc::MMTrace::mm_accept(parent, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(context, __mmv)?;
                Ok(())
            }
            InstNodeType::NORMAL_COMP => Ok(()),
            InstNodeType::REDECLARED_COMP { parent } => {
                metamodelica::gc::MMTrace::mm_accept(parent, __mmv)?;
                Ok(())
            }
            InstNodeType::REDECLARED_CLASS {
                parent,
                originalType,
                originalNode,
                confidence,
            } => {
                metamodelica::gc::MMTrace::mm_accept(parent, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(originalType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(originalNode, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(confidence, __mmv)?;
                Ok(())
            }
            InstNodeType::GENERATED_INNER => Ok(()),
            InstNodeType::IMPLICIT_SCOPE => Ok(()),
        }
    }
}
impl InstNodeType {
    pub fn interned_NORMAL_CLASS() -> metamodelica::Ref<InstNodeType> {
        thread_local! {
            static INTERNED: metamodelica::Ref<InstNodeType> = metamodelica::Ref::new(InstNodeType::NORMAL_CLASS);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_BUILTIN_CLASS() -> metamodelica::Ref<InstNodeType> {
        thread_local! {
            static INTERNED: metamodelica::Ref<InstNodeType> = metamodelica::Ref::new(InstNodeType::BUILTIN_CLASS);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_NORMAL_COMP() -> metamodelica::Ref<InstNodeType> {
        thread_local! {
            static INTERNED: metamodelica::Ref<InstNodeType> = metamodelica::Ref::new(InstNodeType::NORMAL_COMP);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_GENERATED_INNER() -> metamodelica::Ref<InstNodeType> {
        thread_local! {
            static INTERNED: metamodelica::Ref<InstNodeType> = metamodelica::Ref::new(InstNodeType::GENERATED_INNER);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_IMPLICIT_SCOPE() -> metamodelica::Ref<InstNodeType> {
        thread_local! {
            static INTERNED: metamodelica::Ref<InstNodeType> = metamodelica::Ref::new(InstNodeType::IMPLICIT_SCOPE);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_NORMAL_CLASS() -> metamodelica::Ref<InstNodeType> {
    InstNodeType::interned_NORMAL_CLASS()
}
pub fn interned_BUILTIN_CLASS() -> metamodelica::Ref<InstNodeType> {
    InstNodeType::interned_BUILTIN_CLASS()
}
pub fn interned_NORMAL_COMP() -> metamodelica::Ref<InstNodeType> {
    InstNodeType::interned_NORMAL_COMP()
}
pub fn interned_GENERATED_INNER() -> metamodelica::Ref<InstNodeType> {
    InstNodeType::interned_GENERATED_INNER()
}
pub fn interned_IMPLICIT_SCOPE() -> metamodelica::Ref<InstNodeType> {
    InstNodeType::interned_IMPLICIT_SCOPE()
}
impl Default for InstNodeType {
    fn default() -> Self {
        Self::NORMAL_CLASS
    }
}
pub use self::InstNodeType::{
    BASE_CLASS, BUILTIN_CLASS, DERIVED_CLASS, GENERATED_INNER, IMPLICIT_SCOPE, NORMAL_CLASS, NORMAL_COMP,
    REDECLARED_CLASS, REDECLARED_COMP, ROOT_CLASS, TOP_SCOPE,
};

/// A stored reference to a node. `CELL` is the weak edge; `VALUE` is for a node
///   that has no identity cell -- the builtin constants, which are immutable
///   globals and so cannot be on a cycle, and which a constant's serialized form
///   in a .interface.mo can express while a weak reference cannot.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NodeHandle {
    CELL {
        cell: MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>,
        /// The node's name, fixed for the cell's lifetime.
        name: ArcStr,
    },
    VALUE {
        node: metamodelica::Ref<InstNode::InstNode>,
    },
}
impl metamodelica::gc::MMTrace for NodeHandle {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NodeHandle::CELL { cell, name } => {
                metamodelica::gc::MMTrace::mm_accept(cell, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                Ok(())
            }
            NodeHandle::VALUE { node } => {
                metamodelica::gc::MMTrace::mm_accept(node, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for NodeHandle {
    fn default() -> Self {
        Self::VALUE {
            node: Default::default(),
        }
    }
}
pub use self::NodeHandle::{CELL, VALUE};

pub type ScopeRef = Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;

pub const fn NO_SCOPE() -> Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>> {
    None
}

pub(crate) const NUMBER_OF_CACHES: i32 = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum PackageCacheState {
    NOT_INITIALIZED = 1,
    PROCESSING = 2,
    EXPANDED = 3,
    PARTIALLY_INSTANTIATED = 4,
    INSTANTIATED = 5,
}
impl PartialOrd for PackageCacheState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for PackageCacheState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for PackageCacheState {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub mod CachedData {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum CachedData {
        NO_CACHE,
        PACKAGE {
            /// Weakly: this is a cache of the node it hangs off, so
            ///      the node's own scope owns it.
            instance: metamodelica::Ref<NodeHandle>,
            state: PackageCacheState,
        },
        FUNCTION {
            funcs: metamodelica::List<metamodelica::Ref<Function::Function>>,
            typed: bool,
            specialBuiltin: bool,
        },
        PARTIAL_DAE_TYPE {
            ty: metamodelica::Ref<DAE::Type>,
        },
    }
    impl metamodelica::gc::MMTrace for CachedData {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                CachedData::NO_CACHE => Ok(()),
                CachedData::PACKAGE { instance, state } => {
                    metamodelica::gc::MMTrace::mm_accept(instance, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(state, __mmv)?;
                    Ok(())
                }
                CachedData::FUNCTION {
                    funcs,
                    typed,
                    specialBuiltin,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(funcs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(typed, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(specialBuiltin, __mmv)?;
                    Ok(())
                }
                CachedData::PARTIAL_DAE_TYPE { ty } => {
                    metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl CachedData {
        pub fn interned_NO_CACHE() -> metamodelica::Ref<CachedData> {
            thread_local! {
                static INTERNED: metamodelica::Ref<CachedData> = metamodelica::Ref::new(CachedData::NO_CACHE);
            }
            INTERNED.with(|i| i.clone())
        }
    }
    pub fn interned_NO_CACHE() -> metamodelica::Ref<CachedData> {
        CachedData::interned_NO_CACHE()
    }
    impl Default for CachedData {
        fn default() -> Self {
            Self::NO_CACHE
        }
    }
    pub use self::CachedData::{FUNCTION, NO_CACHE, PACKAGE, PARTIAL_DAE_TYPE};
    pub(crate) fn empty() -> metamodelica::Array<metamodelica::Ref<CachedData>> {
        let mut cache: metamodelica::Array<metamodelica::Ref<CachedData>> = arrayCreate(
            NUMBER_OF_CACHES.clone(),
            crate::NFInstNode::CachedData::interned_NO_CACHE(),
        );
        cache
    }

    pub(crate) fn initFunc(mut caches: metamodelica::Array<metamodelica::Ref<CachedData>>) -> Result<()> {
        let mut func_cache: metamodelica::Ref<CachedData>;
        func_cache = getFuncCache(caches.clone())?;
        func_cache = (match &*func_cache {
            NO_CACHE { .. } => metamodelica::Ref::new(CachedData::FUNCTION {
                funcs: metamodelica::nil(),
                typed: false,
                specialBuiltin: false,
            }),
            FUNCTION { .. } => func_cache,
            _ => return Err("match: no arm matched"),
        });
        setFuncCache(caches.clone(), func_cache)?;
        Ok(())
    }

    pub(crate) fn addFunc(
        mut r#fn: metamodelica::Ref<Function::Function>,
        mut specialBuiltin: bool,
        mut caches: metamodelica::Array<metamodelica::Ref<CachedData>>,
    ) -> Result<()> {
        let mut func_cache: metamodelica::Ref<CachedData>;
        func_cache = getFuncCache(caches.clone())?;
        func_cache = (match &*func_cache {
            NO_CACHE { .. } => metamodelica::Ref::new(CachedData::FUNCTION {
                funcs: list![r#fn],
                typed: false,
                specialBuiltin: specialBuiltin,
            }),
            FUNCTION {
                funcs: __func_cache_funcs,
                specialBuiltin: __func_cache_specialBuiltin,
                ..
            } => metamodelica::Ref::new(CachedData::FUNCTION {
                funcs: listAppend(__func_cache_funcs.clone(), list![r#fn]),
                typed: false,
                specialBuiltin: __func_cache_specialBuiltin.clone() || specialBuiltin,
            }),
            _ => {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFInstNode.CachedData.addFunc"));
                        __mm_s.push_str(&*literal!(": Invalid cache for function"));
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFInstNode.mo")),
                )?;
                return Err("fail");
            }
        });
        setFuncCache(caches.clone(), func_cache)?;
        Ok(())
    }

    pub(crate) fn getFuncCache(
        mut in_caches: metamodelica::Array<metamodelica::Ref<CachedData>>,
    ) -> Result<metamodelica::Ref<CachedData>> {
        let mut out_cache: metamodelica::Ref<CachedData> = metamodelica::arrayGet(in_caches.clone(), 1)?;
        Ok(out_cache)
    }

    pub(crate) fn setFuncCache(
        mut in_caches: metamodelica::Array<metamodelica::Ref<CachedData>>,
        mut in_cache: metamodelica::Ref<CachedData>,
    ) -> Result<()> {
        metamodelica::arrayUpdate(in_caches.clone(), 1, in_cache)?;
        Ok(())
    }

    pub(crate) fn getPackageCache(
        mut in_caches: metamodelica::Array<metamodelica::Ref<CachedData>>,
    ) -> Result<metamodelica::Ref<CachedData>> {
        let mut out_cache: metamodelica::Ref<CachedData> = metamodelica::arrayGet(in_caches.clone(), 2)?;
        Ok(out_cache)
    }

    pub(crate) fn getTypeCache(
        mut in_caches: metamodelica::Array<metamodelica::Ref<CachedData>>,
    ) -> Result<metamodelica::Ref<CachedData>> {
        let mut out_cache: metamodelica::Ref<CachedData> = if (metamodelica::arrayLength(in_caches.clone()) >= 3) {
            metamodelica::arrayGet(in_caches.clone(), 3)?
        } else {
            crate::NFInstNode::CachedData::interned_NO_CACHE()
        };
        Ok(out_cache)
    }

    pub(crate) fn setTypeCache(
        mut in_caches: metamodelica::Array<metamodelica::Ref<CachedData>>,
        mut in_cache: metamodelica::Ref<CachedData>,
    ) -> Result<()> {
        if metamodelica::arrayLength(in_caches.clone()) >= 3 {
            metamodelica::arrayUpdate(in_caches.clone(), 3, in_cache)?;
        }
        Ok(())
    }

    pub(crate) fn clearTypeCache(mut in_caches: metamodelica::Array<metamodelica::Ref<CachedData>>) -> Result<()> {
        let () = (match &*(getTypeCache(in_caches.clone())?) {
            PARTIAL_DAE_TYPE { .. } => {
                metamodelica::arrayUpdate(in_caches.clone(), 3, crate::NFInstNode::CachedData::interned_NO_CACHE())?;
                ()
            }
            _ => (),
        });
        Ok(())
    }

    pub(crate) fn setPackageCache(
        mut in_caches: metamodelica::Array<metamodelica::Ref<CachedData>>,
        mut in_cache: metamodelica::Ref<CachedData>,
    ) -> Result<metamodelica::Array<metamodelica::Ref<CachedData>>> {
        let mut out_caches: metamodelica::Array<metamodelica::Ref<CachedData>> =
            metamodelica::arrayUpdate(in_caches.clone(), 2, in_cache.clone())?;
        Ok(out_caches)
    }

    pub(crate) fn clearPackageCache(
        mut in_caches: metamodelica::Array<metamodelica::Ref<CachedData>>,
    ) -> Result<metamodelica::Array<metamodelica::Ref<CachedData>>> {
        let mut out_caches: metamodelica::Array<metamodelica::Ref<CachedData>> =
            metamodelica::arrayUpdate(in_caches.clone(), 2, crate::NFInstNode::CachedData::interned_NO_CACHE())?;
        Ok(out_caches)
    }
}

pub mod InstNode {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum InstNode {
        CLASS_NODE {
            name: ArcStr,
            definition: metamodelica::Ref<SCode::Element>,
            visibility: Visibility,
            cls: Pointer::Pointer<metamodelica::Ref<Class::NFClass>>,
            caches: metamodelica::Array<metamodelica::Ref<CachedData::CachedData>>,
            /// The cell this node publishes itself into
            ///      for its children to read back. NONE() in the published copy, which must
            ///      not own the cell it lives in.
            owner: Option<Mutable::Mutable<metamodelica::Ref<InstNode>>>,
            /// The same cell, weakly.
            identity: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>,
            /// The enclosing scope's identity.
            ///      Weak: a scope owns the nodes in it.
            parentScope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>,
            nodeType: metamodelica::Ref<InstNodeType>,
        },
        COMPONENT_NODE {
            name: ArcStr,
            definition: Option<metamodelica::Ref<SCode::Element>>,
            visibility: Visibility,
            component: Pointer::Pointer<metamodelica::Ref<Component::NFComponent>>,
            /// See CLASS_NODE.owner.
            owner: Option<Mutable::Mutable<metamodelica::Ref<InstNode>>>,
            /// See CLASS_NODE.identity.
            identity: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>,
            /// The instance that this component is
            ///      part of; see CLASS_NODE.parentScope.
            parent: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>,
            nodeType: metamodelica::Ref<InstNodeType>,
        },
        /// A node representing an outer element, with a reference to the corresponding inner.
        INNER_OUTER_NODE {
            innerNode: metamodelica::Ref<InstNode>,
            outerNode: metamodelica::Ref<InstNode>,
        },
        REF_NODE {
            index: i32,
        },
        NAME_NODE {
            name: ArcStr,
        },
        IMPLICIT_SCOPE {
            parentScope: metamodelica::Ref<InstNode>,
            locals: metamodelica::List<metamodelica::Ref<InstNode>>,
        },
        ITERATOR_NODE {
            exp: metamodelica::Ref<Expression::NFExpression>,
        },
        /// This is an extension for better use in the backend. Not used in the Frontend.
        ///    NOTE: Map and traversal functions are not allowed to follow the variable
        ///    pointer, it would create cyclic behaviour! Var->cref->pointer->Var
        VAR_NODE {
            name: ArcStr,
            /// Weak, which is what stops the
            ///      `Var -> cref -> pointer -> Var` loop the note above warns about. The
            ///      backend's `VariablePointers` owns every variable.
            varPointer: PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>,
        },
        EMPTY_NODE,
    }
    impl metamodelica::gc::MMTrace for InstNode {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                InstNode::CLASS_NODE {
                    name,
                    definition,
                    visibility,
                    cls,
                    caches,
                    owner,
                    identity,
                    parentScope,
                    nodeType,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(definition, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(visibility, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(cls, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(caches, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(owner, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(identity, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(parentScope, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(nodeType, __mmv)?;
                    Ok(())
                }
                InstNode::COMPONENT_NODE {
                    name,
                    definition,
                    visibility,
                    component,
                    owner,
                    identity,
                    parent,
                    nodeType,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(definition, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(visibility, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(component, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(owner, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(identity, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(parent, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(nodeType, __mmv)?;
                    Ok(())
                }
                InstNode::INNER_OUTER_NODE { innerNode, outerNode } => {
                    metamodelica::gc::MMTrace::mm_accept(innerNode, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(outerNode, __mmv)?;
                    Ok(())
                }
                InstNode::REF_NODE { index } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    Ok(())
                }
                InstNode::NAME_NODE { name } => {
                    metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                    Ok(())
                }
                InstNode::IMPLICIT_SCOPE { parentScope, locals } => {
                    metamodelica::gc::MMTrace::mm_accept(parentScope, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(locals, __mmv)?;
                    Ok(())
                }
                InstNode::ITERATOR_NODE { exp } => {
                    metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                    Ok(())
                }
                InstNode::VAR_NODE { name, varPointer } => {
                    metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(varPointer, __mmv)?;
                    Ok(())
                }
                InstNode::EMPTY_NODE => Ok(()),
            }
        }
    }
    impl InstNode {
        pub fn interned_EMPTY_NODE() -> metamodelica::Ref<InstNode> {
            thread_local! {
                static INTERNED: metamodelica::Ref<InstNode> = metamodelica::Ref::new(InstNode::EMPTY_NODE);
            }
            INTERNED.with(|i| i.clone())
        }
    }
    pub fn interned_EMPTY_NODE() -> metamodelica::Ref<InstNode> {
        InstNode::interned_EMPTY_NODE()
    }
    impl Default for InstNode {
        fn default() -> Self {
            Self::EMPTY_NODE
        }
    }
    pub use self::InstNode::{
        CLASS_NODE, COMPONENT_NODE, EMPTY_NODE, IMPLICIT_SCOPE, INNER_OUTER_NODE, ITERATOR_NODE, NAME_NODE, REF_NODE,
        VAR_NODE,
    };
    pub(crate) fn new(
        mut definition: metamodelica::Ref<SCode::Element>,
        mut parent: metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode>;
        node = (match &*definition {
            SCode::Element::CLASS { .. } => newClass(
                definition,
                parent,
                crate::NFInstNode::InstNodeType::interned_NORMAL_CLASS(),
            )?,
            SCode::Element::COMPONENT { .. } => newComponent(definition, parent)?,
            _ => return Err("match: no arm matched"),
        });
        Ok(node)
    }

    pub(crate) fn newClass(
        mut definition: metamodelica::Ref<SCode::Element>,
        mut parent: metamodelica::Ref<InstNode>,
        mut nodeType: metamodelica::Ref<InstNodeType>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode>;
        let mut name: ArcStr;
        let mut vis: SCode::Visibility;
        let mut owner: Option<Mutable::Mutable<metamodelica::Ref<InstNode>>>;
        let mut identity: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>;
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(definition.clone()) {
            Deref @ SCode::Element::CLASS { name: __pa0, prefixes: Deref @ SCode::Prefixes { visibility: __pa1, .. }, .. } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        name = metamodelica::Own::own(__pa0);
        vis = metamodelica::Own::own(__pa1);
        (owner, identity) = newIdentity();
        node = metamodelica::Ref::new(InstNode::CLASS_NODE {
            name: name,
            definition: definition,
            visibility: Prefixes::visibilityFromSCode(vis),
            cls: Pointer::create(crate::NFClass::interned_NOT_INSTANTIATED()),
            caches: CachedData::empty(),
            owner: owner,
            identity: identity,
            parentScope: identityCell(parent),
            nodeType: nodeType,
        });
        Ok(node)
    }

    pub(crate) fn newComponent(
        mut definition: metamodelica::Ref<SCode::Element>,
        mut parent: metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode>;
        let mut name: ArcStr;
        let mut vis: SCode::Visibility;
        let mut owner: Option<Mutable::Mutable<metamodelica::Ref<InstNode>>>;
        let mut identity: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>;
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(definition.clone()) {
            Deref @ SCode::Element::COMPONENT { name: __pa0, prefixes: Deref @ SCode::Prefixes { visibility: __pa1, .. }, .. } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        name = metamodelica::Own::own(__pa0);
        vis = metamodelica::Own::own(__pa1);
        (owner, identity) = newIdentity();
        node = metamodelica::Ref::new(InstNode::COMPONENT_NODE {
            name: name,
            definition: Some(definition.clone()),
            visibility: Prefixes::visibilityFromSCode(vis),
            component: Pointer::create(Component::new(definition)),
            owner: owner,
            identity: identity,
            parent: identityCell(parent),
            nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP(),
        });
        Ok(node)
    }

    pub(crate) fn newExtends(
        mut definition: metamodelica::Ref<SCode::Element>,
        mut parent: metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode>;
        let mut base_path: metamodelica::Ref<Absyn::Path>;
        let mut name: ArcStr;
        let mut vis: SCode::Visibility;
        let mut owner: Option<Mutable::Mutable<metamodelica::Ref<InstNode>>>;
        let mut identity: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>;
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(definition.clone()) {
            Deref @ SCode::Element::EXTENDS { baseClassPath: __pa0, visibility: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        base_path = metamodelica::Own::own(__pa0);
        vis = metamodelica::Own::own(__pa1);
        name = AbsynUtil::pathLastIdent(&base_path);
        (owner, identity) = newIdentity();
        node = metamodelica::Ref::new(InstNode::CLASS_NODE {
            name: name,
            definition: definition.clone(),
            visibility: Prefixes::visibilityFromSCode(vis),
            cls: Pointer::create(crate::NFClass::interned_NOT_INSTANTIATED()),
            caches: CachedData::empty(),
            owner: owner,
            identity: identity,
            parentScope: identityCell(parent.clone()),
            nodeType: metamodelica::Ref::new(InstNodeType::BASE_CLASS {
                parent: identityCell(parent.clone()),
                definition: definition,
                ty: nodeType(&parent)?,
            }),
        });
        Ok(node)
    }

    pub fn newIterator(
        mut name: ArcStr,
        mut ty: metamodelica::Ref<Type::NFType>,
        mut info: SourceInfo,
    ) -> metamodelica::Ref<InstNode> {
        let mut iterator: metamodelica::Ref<InstNode>;
        iterator = fromComponent(
            name,
            Component::newIterator(ty, info),
            crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
        );
        iterator
    }

    pub fn newUniqueIterator(
        mut info: SourceInfo,
        mut ty: metamodelica::Ref<Type::NFType>,
    ) -> metamodelica::Ref<InstNode> {
        let mut iterator: metamodelica::Ref<InstNode>;
        iterator = newIterator(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("$i"));
                __mm_s.push_str(&*ArcStr::from(::std::format!(
                    "{}",
                    System::tmpTickIndex(Global::iteratorIndex.clone())
                )));
                ArcStr::from(__mm_s)
            },
            ty,
            info,
        );
        iterator
    }

    pub(crate) fn newIndexedIterator(
        mut index: i32,
        mut name: &ArcStr,
        mut info: SourceInfo,
        mut ty: metamodelica::Ref<Type::NFType>,
    ) -> metamodelica::Ref<InstNode> {
        let mut iterator: metamodelica::Ref<InstNode>;
        iterator = newIterator(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("$"));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index)));
                ArcStr::from(__mm_s)
            },
            ty,
            info,
        );
        iterator
    }

    pub(crate) fn fromComponent(
        mut name: ArcStr,
        mut component: metamodelica::Ref<Component::NFComponent>,
        mut parent: metamodelica::Ref<InstNode>,
    ) -> metamodelica::Ref<InstNode> {
        let mut node: metamodelica::Ref<InstNode>;
        let mut owner: Option<Mutable::Mutable<metamodelica::Ref<InstNode>>>;
        let mut identity: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>;
        (owner, identity) = newIdentity();
        node = metamodelica::Ref::new(InstNode::COMPONENT_NODE {
            name: name,
            definition: None,
            visibility: Visibility::PUBLIC.clone(),
            component: Pointer::create(component),
            owner: owner,
            identity: identity,
            parent: identityCell(parent),
            nodeType: crate::NFInstNode::InstNodeType::interned_NORMAL_COMP(),
        });
        node
    }

    pub fn isClass<'__b>(mut node: &'__b metamodelica::Ref<InstNode>) -> Result<bool> {
        '__tco: loop {
            match &**node {
                CLASS_NODE { .. } => return Ok(true),
                INNER_OUTER_NODE { .. } => {
                    node = var_field!((**node).innerNode, InstNode::INNER_OUTER_NODE);
                    continue '__tco;
                }
                _ => return Ok(false),
            }
        }
    }

    pub(crate) fn isBaseClass(mut node: &metamodelica::Ref<InstNode>) -> bool {
        let mut isBaseClass: bool;
        isBaseClass = (::match_deref::match_deref! { match node {
            Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::BASE_CLASS { .. }, .. } => true,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        isBaseClass
    }

    pub(crate) fn isUserdefinedClass(mut node: metamodelica::Ref<InstNode>) -> Result<bool> {
        '__tco: loop {
            let mut ty: metamodelica::Ref<InstNodeType>;
            match &*node {
                CLASS_NODE { nodeType: __esc_ty, .. } => {
                    ty = (*__esc_ty).clone();
                    match &*ty.clone() {
                        InstNodeType::NORMAL_CLASS => return Ok(true),
                        InstNodeType::BASE_CLASS { .. } => return Ok(true),
                        InstNodeType::DERIVED_CLASS { .. } => return Ok(true),
                        InstNodeType::REDECLARED_CLASS {
                            parent: __ty_parent, ..
                        } => {
                            node = borrow(__ty_parent.clone())?;
                            continue '__tco;
                        }
                        _ => return Ok(false),
                    }
                }
                _ => return Ok(false),
            }
        }
    }

    pub fn isDerivedClass(mut node: &metamodelica::Ref<InstNode>) -> bool {
        let mut isDerived: bool;
        isDerived = (::match_deref::match_deref! { match node {
            Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::DERIVED_CLASS { .. }, .. } => true,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        isDerived
    }

    pub fn makeRootClass(
        mut node: metamodelica::Ref<InstNode>,
        mut parent: metamodelica::Ref<InstNode>,
        mut context: Option<metamodelica::Ref<Absyn::Path>>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode> = node;
        node = setNodeType(
            metamodelica::Ref::new(InstNodeType::ROOT_CLASS {
                parent: scopeRef(parent),
                context: context,
            }),
            node,
        )?;
        Ok(node)
    }

    pub fn isRootClass(mut node: &metamodelica::Ref<InstNode>) -> bool {
        let mut res: bool;
        res = (::match_deref::match_deref! { match node {
            Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::ROOT_CLASS { .. }, .. } => true,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        res
    }

    pub(crate) fn rootClassContext(mut node: &metamodelica::Ref<InstNode>) -> Option<metamodelica::Ref<Absyn::Path>> {
        let mut context: Option<metamodelica::Ref<Absyn::Path>>;
        context = (::match_deref::match_deref! { match node {
            Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::ROOT_CLASS { context: __esc_context, .. }, .. } => {
                context = (*__esc_context).clone();
                context.clone()
            },
            _ => None,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        context
    }

    pub fn isFunction(mut node: metamodelica::Ref<InstNode>) -> Result<bool> {
        let mut isFunc: bool;
        isFunc = (match &*node {
            CLASS_NODE { cls: __node_cls, .. } => Class::isFunction(&(Pointer::access(__node_cls.clone()))),
            COMPONENT_NODE { .. } => Class::isFunction(&(getClass(node)?)),
            _ => false,
        });
        Ok(isFunc)
    }

    pub fn isComponent<'__b>(mut node: &'__b metamodelica::Ref<InstNode>) -> Result<bool> {
        '__tco: loop {
            match &**node {
                COMPONENT_NODE { .. } => return Ok(true),
                INNER_OUTER_NODE { .. } => {
                    node = var_field!((**node).innerNode, InstNode::INNER_OUTER_NODE);
                    continue '__tco;
                }
                _ => return Ok(false),
            }
        }
    }

    pub(crate) fn isIterator(mut node: &metamodelica::Ref<InstNode>) -> bool {
        let mut iterator: bool;
        iterator = (match &**node {
            COMPONENT_NODE {
                component: __node_component,
                ..
            } => Component::isIterator(&(Pointer::access(__node_component.clone()))),
            _ => false,
        });
        iterator
    }

    pub(crate) fn isRef(mut node: &metamodelica::Ref<InstNode>) -> bool {
        let mut isRef: bool;
        isRef = (match &**node {
            REF_NODE { .. } => true,
            _ => false,
        });
        isRef
    }

    pub fn isVar(mut node: &metamodelica::Ref<InstNode>) -> bool {
        let mut isVar: bool;
        isVar = (match &**node {
            VAR_NODE { .. } => true,
            _ => false,
        });
        isVar
    }

    pub fn varPointer(
        mut node: &metamodelica::Ref<InstNode>,
    ) -> Result<PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>> {
        let mut varPointer: PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>;
        let __pa0 = ::match_deref::match_deref! { match &((*node)) {
            Deref @ VAR_NODE { varPointer: __pa0, .. } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        varPointer = metamodelica::Own::own(__pa0);
        Ok(varPointer)
    }

    pub fn isEmpty(mut node: &metamodelica::Ref<InstNode>) -> bool {
        let mut isEmpty: bool;
        isEmpty = (match &**node {
            EMPTY_NODE { .. } => true,
            _ => false,
        });
        isEmpty
    }

    pub(crate) fn isImplicit(mut node: &metamodelica::Ref<InstNode>) -> bool {
        let mut isImplicit: bool;
        isImplicit = (match &**node {
            IMPLICIT_SCOPE { .. } => true,
            _ => false,
        });
        isImplicit
    }

    pub fn isName(mut node: &metamodelica::Ref<InstNode>) -> bool {
        let mut isName: bool;
        isName = (match &**node {
            NAME_NODE { .. } => true,
            _ => false,
        });
        isName
    }

    pub(crate) fn isConnector(mut node: &metamodelica::Ref<InstNode>) -> Result<bool> {
        let mut isConnector: bool;
        isConnector = (match &**node {
            COMPONENT_NODE { .. } => Component::isConnector(&(component(node)?)),
            NAME_NODE { .. } => true,
            _ => false,
        });
        Ok(isConnector)
    }

    pub(crate) fn isExpandableConnector(mut node: &metamodelica::Ref<InstNode>) -> Result<bool> {
        let mut isConnector: bool;
        isConnector = (match &**node {
            COMPONENT_NODE { .. } => Component::isExpandableConnector(&(component(node)?)),
            _ => false,
        });
        Ok(isConnector)
    }

    pub(crate) fn hasParentExpandableConnector(mut node: metamodelica::Ref<InstNode>) -> Result<bool> {
        let mut b: bool = isExpandableConnector(&node)?;
        let mut p: metamodelica::Ref<InstNode>;
        p = node;
        while !(isEmpty(&p)) {
            p = parent(&p)?;
            b = boolOr(b, isExpandableConnector(&p)?);
            if b {
                break;
            }
        }
        Ok(b)
    }

    pub(crate) fn isOperator<'__b>(mut node: &'__b metamodelica::Ref<InstNode>) -> bool {
        '__tco: loop {
            match &**node {
                CLASS_NODE { .. } => {
                    return SCodeUtil::isOperator(var_field!((**node).definition, InstNode::CLASS_NODE));
                }
                INNER_OUTER_NODE { .. } => {
                    node = var_field!((**node).innerNode, InstNode::INNER_OUTER_NODE);
                    continue '__tco;
                }
                _ => return false,
            }
        }
    }

    pub fn name<'__b>(mut node: &'__b metamodelica::Ref<InstNode>) -> Result<ArcStr> {
        '__tco: loop {
            match &**node {
                CLASS_NODE { .. } => return Ok(var_field!((**node).name, InstNode::CLASS_NODE).clone()),
                COMPONENT_NODE { .. } => return Ok(var_field!((**node).name, InstNode::COMPONENT_NODE).clone()),
                INNER_OUTER_NODE { .. } => {
                    node = var_field!((**node).innerNode, InstNode::INNER_OUTER_NODE);
                    continue '__tco;
                }
                VAR_NODE { .. } => return Ok(var_field!((**node).name, InstNode::VAR_NODE).clone()),
                REF_NODE { .. } => {
                    return Ok({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("$REF["));
                        __mm_s.push_str(&*ArcStr::from(::std::format!(
                            "{}",
                            var_field!((**node).index, InstNode::REF_NODE).clone()
                        )));
                        __mm_s.push_str(&*literal!("]"));
                        ArcStr::from(__mm_s)
                    });
                }
                NAME_NODE { .. } => return Ok(var_field!((**node).name, InstNode::NAME_NODE).clone()),
                IMPLICIT_SCOPE { .. } => return Ok(literal!("$IMPLICIT")),
                ITERATOR_NODE { .. } => {
                    return Ok({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("$ITERATOR("));
                        __mm_s.push_str(&*Expression::toString(
                            var_field!((**node).exp, InstNode::ITERATOR_NODE).clone(),
                        )?);
                        __mm_s.push_str(&*literal!(")"));
                        ArcStr::from(__mm_s)
                    });
                }
                EMPTY_NODE { .. } => return Ok(literal!("$EMPTY")),
            }
        }
    }

    pub(crate) fn isNamed<'__b>(mut node: &'__b metamodelica::Ref<InstNode>, mut name: &'__b ArcStr) -> bool {
        '__tco: loop {
            match &**node {
                CLASS_NODE { .. } => {
                    return metamodelica::stringEq(&var_field!((**node).name, InstNode::CLASS_NODE), &name);
                }
                COMPONENT_NODE { .. } => {
                    return metamodelica::stringEq(&var_field!((**node).name, InstNode::COMPONENT_NODE), &name);
                }
                INNER_OUTER_NODE { .. } => {
                    (node, name) = (var_field!((**node).innerNode, InstNode::INNER_OUTER_NODE), name);
                    continue '__tco;
                }
                VAR_NODE { .. } => {
                    return metamodelica::stringEq(&var_field!((**node).name, InstNode::VAR_NODE), &name);
                }
                NAME_NODE { .. } => {
                    return metamodelica::stringEq(&var_field!((**node).name, InstNode::NAME_NODE), &name);
                }
                _ => return false,
            }
        }
    }

    pub(crate) fn className(mut node: &metamodelica::Ref<InstNode>) -> Result<ArcStr> {
        let mut name: ArcStr;
        let __pa0 = ::match_deref::match_deref! { match &((*node)) {
            Deref @ CLASS_NODE { name: __pa0, .. } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        name = metamodelica::Own::own(__pa0);
        Ok(name)
    }

    pub(crate) fn scopeName(mut node: &metamodelica::Ref<InstNode>) -> Result<ArcStr> {
        let mut outName: ArcStr = name(&(classScope(explicitScope(node))?))?;
        Ok(outName)
    }

    pub(crate) fn typeName<'__b>(mut node: &'__b metamodelica::Ref<InstNode>) -> Result<ArcStr> {
        '__tco: loop {
            match &**node {
                CLASS_NODE { .. } => return Ok(literal!("class")),
                COMPONENT_NODE { .. } => return Ok(literal!("component")),
                INNER_OUTER_NODE { .. } => {
                    node = var_field!((**node).innerNode, InstNode::INNER_OUTER_NODE);
                    continue '__tco;
                }
                REF_NODE { .. } => return Ok(literal!("ref node")),
                NAME_NODE { .. } => return Ok(literal!("name node")),
                IMPLICIT_SCOPE { .. } => return Ok(literal!("implicit scope")),
                EMPTY_NODE { .. } => return Ok(literal!("empty node")),
                VAR_NODE { .. } => return Ok(literal!("var node")),
                _ => return Err("match: no arm matched"),
            }
        }
    }

    pub fn rename(mut name: ArcStr, mut node: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode> = node;
        node = reidentify(node);
        let () = (match &*node {
            CLASS_NODE { .. } => {
                assign_variant_field!(node => InstNode::CLASS_NODE; name = name);
                ()
            }
            COMPONENT_NODE { .. } => {
                assign_variant_field!(node => InstNode::COMPONENT_NODE; name = name);
                ()
            }
            NAME_NODE { .. } => {
                assign_variant_field!(node => InstNode::NAME_NODE; name = name);
                ()
            }
            VAR_NODE { .. } => {
                assign_variant_field!(node => InstNode::VAR_NODE; name = name);
                ()
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(node)
    }

    pub fn reidentify(mut node: metamodelica::Ref<InstNode>) -> metamodelica::Ref<InstNode> {
        let mut node: metamodelica::Ref<InstNode> = node;
        let mut owner: Option<Mutable::Mutable<metamodelica::Ref<InstNode>>>;
        let mut identity: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>;
        let () = (match &*node {
            CLASS_NODE { .. } => {
                (owner, identity) = newIdentity();
                assign_variant_field!(node => InstNode::CLASS_NODE;
                    owner = owner,
                    identity = identity
                );
                ()
            }
            COMPONENT_NODE { .. } => {
                (owner, identity) = newIdentity();
                assign_variant_field!(node => InstNode::COMPONENT_NODE;
                    owner = owner,
                    identity = identity
                );
                ()
            }
            _ => (),
        });
        node
    }

    pub fn identityCell(
        mut node: metamodelica::Ref<InstNode>,
    ) -> Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>> {
        let mut identity: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>;
        identity = (match &*node.clone() {
            CLASS_NODE {
                owner: Some(cell),
                identity: __node_identity,
                ..
            } => {
                Mutable::update(cell.clone(), disown(node));
                __node_identity.clone()
            }
            COMPONENT_NODE {
                owner: Some(cell),
                identity: __node_identity,
                ..
            } => {
                Mutable::update(cell.clone(), disown(node));
                __node_identity.clone()
            }
            CLASS_NODE {
                identity: __node_identity,
                ..
            } => __node_identity.clone(),
            COMPONENT_NODE {
                identity: __node_identity,
                ..
            } => __node_identity.clone(),
            _ => None,
        });
        identity
    }

    pub(crate) fn scopeRef(
        mut node: metamodelica::Ref<InstNode>,
    ) -> Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>> {
        let mut scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>;
        scope = (match &*node.clone() {
            CLASS_NODE {
                owner: Some(cell),
                identity: __node_identity,
                ..
            } => {
                if isEmpty(&(Mutable::access(cell.clone()))) {
                    Mutable::update(cell.clone(), disown(node));
                }
                __node_identity.clone()
            }
            COMPONENT_NODE {
                owner: Some(cell),
                identity: __node_identity,
                ..
            } => {
                if isEmpty(&(Mutable::access(cell.clone()))) {
                    Mutable::update(cell.clone(), disown(node));
                }
                __node_identity.clone()
            }
            CLASS_NODE {
                identity: __node_identity,
                ..
            } => __node_identity.clone(),
            COMPONENT_NODE {
                identity: __node_identity,
                ..
            } => __node_identity.clone(),
            _ => None,
        });
        scope
    }

    pub(crate) fn handle(mut node: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<NodeHandle>> {
        let mut hnd: metamodelica::Ref<NodeHandle>;
        hnd = (match &*node.clone() {
            CLASS_NODE {
                owner: Some(cell),
                identity: __node_identity,
                ..
            } => {
                if isEmpty(&(Mutable::access(cell.clone()))) {
                    Mutable::update(cell.clone(), disown(node.clone()));
                }
                fromIdentity(__node_identity.clone(), node)?
            }
            COMPONENT_NODE {
                owner: Some(cell),
                identity: __node_identity,
                ..
            } => {
                if isEmpty(&(Mutable::access(cell.clone()))) {
                    Mutable::update(cell.clone(), disown(node.clone()));
                }
                fromIdentity(__node_identity.clone(), node)?
            }
            CLASS_NODE {
                identity: __node_identity,
                ..
            } => fromIdentity(__node_identity.clone(), node)?,
            COMPONENT_NODE {
                identity: __node_identity,
                ..
            } => fromIdentity(__node_identity.clone(), node)?,
            _ => metamodelica::Ref::new(NodeHandle::VALUE { node: node }),
        });
        Ok(hnd)
    }

    pub(crate) fn republish(mut node: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<NodeHandle>> {
        let mut hnd: metamodelica::Ref<NodeHandle>;
        hnd = (match &*node.clone() {
            CLASS_NODE {
                owner: Some(cell),
                identity: __node_identity,
                ..
            } => {
                Mutable::update(cell.clone(), disown(node.clone()));
                fromIdentity(__node_identity.clone(), node)?
            }
            COMPONENT_NODE {
                owner: Some(cell),
                identity: __node_identity,
                ..
            } => {
                Mutable::update(cell.clone(), disown(node.clone()));
                fromIdentity(__node_identity.clone(), node)?
            }
            _ => handle(node)?,
        });
        Ok(hnd)
    }

    pub(crate) fn handleName(mut hnd: &metamodelica::Ref<NodeHandle>) -> Result<ArcStr> {
        let mut name: ArcStr;
        name = (match &**hnd {
            NodeHandle::CELL { name: __hnd_name, .. } => __hnd_name.clone(),
            NodeHandle::VALUE { node: __hnd_node } => self::name(metamodelica::AsArg::as_arg(&__hnd_node))?,
        });
        Ok(name)
    }

    pub fn fromHandle(mut hnd: &metamodelica::Ref<NodeHandle>) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode>;
        node = (match &**hnd {
            NodeHandle::CELL {
                cell: __hnd_cell,
                name: __hnd_name,
            } => Mutable::access(upgradeCell(
                __hnd_cell.clone(),
                false,
                metamodelica::AsArg::as_arg(&__hnd_name),
            )?),
            NodeHandle::VALUE { node: __hnd_node } => __hnd_node.clone(),
        });
        Ok(node)
    }

    pub fn borrow(
        mut cell: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode>;
        node = (match cell {
            Some(mut w) => Mutable::access(upgradeCell(w, false, &(literal!("")))?),
            _ => crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
        });
        Ok(node)
    }

    pub(crate) fn fromCell(
        mut cell: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode>;
        node = (match cell {
            Some(mut w) => {
                let mut c: Mutable::Mutable<metamodelica::Ref<InstNode>>;
                c = upgradeCell(w, true, &(literal!("")))?;
                reown(Mutable::access(c.clone()), c)
            }
            _ => crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
        });
        Ok(node)
    }

    pub fn parent(mut node: &metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<InstNode>> {
        let mut parent: metamodelica::Ref<InstNode>;
        parent = (match &**node {
            CLASS_NODE {
                parentScope: __node_parentScope,
                ..
            } => fromCell(__node_parentScope.clone())?,
            COMPONENT_NODE {
                parent: __node_parent, ..
            } => fromCell(__node_parent.clone())?,
            IMPLICIT_SCOPE {
                parentScope: __node_parentScope,
                ..
            } => __node_parentScope.clone(),
            _ => crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
        });
        Ok(parent)
    }

    pub(crate) fn borrowParent(mut node: &metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<InstNode>> {
        let mut parent: metamodelica::Ref<InstNode>;
        parent = (match &**node {
            CLASS_NODE {
                parentScope: __node_parentScope,
                ..
            } => borrow(__node_parentScope.clone())?,
            COMPONENT_NODE {
                parent: __node_parent, ..
            } => borrow(__node_parent.clone())?,
            IMPLICIT_SCOPE {
                parentScope: __node_parentScope,
                ..
            } => __node_parentScope.clone(),
            _ => crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
        });
        Ok(parent)
    }

    pub(crate) fn explicitParent(mut node: &metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<InstNode>> {
        let mut parentNode: metamodelica::Ref<InstNode> = explicitScope(&(parent(node)?));
        Ok(parentNode)
    }

    pub fn classParent(mut node: &metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<InstNode>> {
        let mut parent: metamodelica::Ref<InstNode>;
        let mut p: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>;
        let __pa0 = ::match_deref::match_deref! { match &((*node)) {
            Deref @ CLASS_NODE { parentScope: __pa0, .. } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        p = metamodelica::Own::own(__pa0);
        parent = fromCell(p)?;
        Ok(parent)
    }

    pub(crate) fn instanceParent(mut node: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<InstNode>> {
        let mut parent: metamodelica::Ref<InstNode>;
        let mut rdcl_scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>;
        parent = (::match_deref::match_deref! { match &(node.clone()) {
            Deref @ CLASS_NODE { .. } => getDerivedNode(self::parent(&(getDerivedNode(node, true)?))?, true)?,
            Deref @ COMPONENT_NODE { nodeType: Deref @ InstNodeType::REDECLARED_COMP { parent: __esc_rdcl_scope }, .. } => {
                rdcl_scope = (*__esc_rdcl_scope).clone();
                getDerivedNode(fromCell(rdcl_scope.clone())?, true)?
            },
            Deref @ COMPONENT_NODE { .. } => getDerivedNode(self::parent(&(getDerivedNode(node, true)?))?, true)?,
            Deref @ IMPLICIT_SCOPE { .. } => getDerivedNode(self::parent(&(getDerivedNode(node, true)?))?, true)?,
            _ => crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(parent)
    }

    pub(crate) fn borrowInstanceParent(mut node: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<InstNode>> {
        let mut parent: metamodelica::Ref<InstNode>;
        let mut rdcl_scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>;
        parent = (::match_deref::match_deref! { match &(node.clone()) {
            Deref @ COMPONENT_NODE { nodeType: Deref @ InstNodeType::REDECLARED_COMP { parent: __esc_rdcl_scope }, .. } => {
                rdcl_scope = (*__esc_rdcl_scope).clone();
                getDerivedNode(borrow(rdcl_scope.clone())?, true)?
            },
            Deref @ COMPONENT_NODE { parent: __node_parent, .. } => getDerivedNode(borrow(__node_parent.clone())?, true)?,
            _ => instanceParent(node)?,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(parent)
    }

    pub(crate) fn rootParent(mut node: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<InstNode>> {
        let mut parent: metamodelica::Ref<InstNode>;
        parent = (match &*node.clone() {
            CLASS_NODE {
                nodeType: __node_nodeType,
                ..
            } => rootTypeParent(metamodelica::AsArg::as_arg(&__node_nodeType), &node)?,
            _ => self::parent(&node)?,
        });
        Ok(parent)
    }

    pub(crate) fn rootTypeParent<'__b>(
        mut nodeType: &'__b metamodelica::Ref<InstNodeType>,
        mut node: &'__b metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        '__tco: loop {
            match &**nodeType {
                InstNodeType::ROOT_CLASS { .. }
                    if (!(isEmpty(&(borrow(var_field!((**nodeType).parent, InstNodeType::ROOT_CLASS).clone())?)))) =>
                {
                    return Ok(borrow(
                        var_field!((**nodeType).parent, InstNodeType::ROOT_CLASS).clone(),
                    )?);
                }
                InstNodeType::DERIVED_CLASS { .. } => {
                    (nodeType, node) = (var_field!((**nodeType).ty, InstNodeType::DERIVED_CLASS), node);
                    continue '__tco;
                }
                _ => return Ok(self::parent(node)?),
            }
        }
    }

    pub(crate) fn parentScope(
        mut node: metamodelica::Ref<InstNode>,
        mut ignoreRedeclare: bool,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut scope: metamodelica::Ref<InstNode>;
        let mut orig_node: metamodelica::Ref<InstNode>;
        let mut rdcl_scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>;
        scope = (::match_deref::match_deref! { match &(node.clone()) {
            Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::DERIVED_CLASS { .. }, parentScope: __node_parentScope, .. } => {
                scope = Class::lastBaseClass(node.clone())?;
                if (isBuiltin(&scope)) {topScope(fromCell(__node_parentScope.clone())?)?} else if (referenceEq(&*(node),&*(&*scope))) {fromCell(__node_parentScope.clone())?} else {parentScope(scope, false)?}
            },
            Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::REDECLARED_CLASS { originalNode: Some(__esc_orig_node), .. }, .. } if (ignoreRedeclare) => {
                orig_node = (*__esc_orig_node).clone();
                parentScope(orig_node.clone(), false)?
            },
            Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::REDECLARED_CLASS { parent: __esc_rdcl_scope, .. }, .. } if (ignoreRedeclare) => {
                rdcl_scope = (*__esc_rdcl_scope).clone();
                fromCell(rdcl_scope.clone())?
            },
            Deref @ CLASS_NODE { parentScope: __node_parentScope, .. } => fromCell(__node_parentScope.clone())?,
            Deref @ COMPONENT_NODE { component: __node_component, .. } => parentScope(Component::classInstance(&(Pointer::access(__node_component.clone())))?, false)?,
            Deref @ IMPLICIT_SCOPE { parentScope: __node_parentScope, .. } => __node_parentScope.clone(),
            _ => return Err("match: no arm matched"),
        } });
        Ok(scope)
    }

    pub fn enclosingScopePath(
        mut node: metamodelica::Ref<InstNode>,
        mut ignoreRedeclare: bool,
        mut ignoreBaseClass: bool,
    ) -> Result<metamodelica::Ref<Absyn::Path>> {
        let mut path: metamodelica::Ref<Absyn::Path>;
        path = AbsynUtil::stringListPath(
            ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut n in (enclosingScopeList(node, ignoreRedeclare, ignoreBaseClass)?)
                    .into_iter()
                    .cloned()
                {
                    let __x = name(&(n.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        )?;
        Ok(path)
    }

    pub(crate) fn enclosingScopeList(
        mut node: metamodelica::Ref<InstNode>,
        mut ignoreRedeclare: bool,
        mut ignoreBaseClass: bool,
    ) -> Result<metamodelica::List<metamodelica::Ref<InstNode>>> {
        let mut res: metamodelica::List<metamodelica::Ref<InstNode>> = metamodelica::nil();
        let mut scope: metamodelica::Ref<InstNode> = node;
        while !(isTopScope(&scope)) {
            res = metamodelica::cons(scope.clone(), res);
            scope = enclosingScope(scope, ignoreRedeclare, ignoreBaseClass)?;
            if isEmpty(&scope) {
                break;
            }
            scope = classScope(scope)?;
        }
        Ok(res)
    }

    pub(crate) fn enclosingScope(
        mut node: metamodelica::Ref<InstNode>,
        mut ignoreRedeclare: bool,
        mut ignoreBaseClass: bool,
    ) -> Result<metamodelica::Ref<InstNode>> {
        '__tco: loop {
            let mut orig_node: metamodelica::Ref<InstNode>;
            let mut rdcl_scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>;
            ::match_deref::match_deref! { match &(node.clone()) {
                Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::REDECLARED_CLASS { originalNode: Some(__esc_orig_node), .. }, .. } if (ignoreRedeclare) => {
                    orig_node = (*__esc_orig_node).clone();
                    { (node, ignoreRedeclare, ignoreBaseClass) = (orig_node.clone(), ignoreRedeclare, ignoreBaseClass); continue '__tco; }
                },
                Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::REDECLARED_CLASS { parent: __esc_rdcl_scope, .. }, .. } if (ignoreRedeclare) => {
                    rdcl_scope = (*__esc_rdcl_scope).clone();
                    return Ok(fromCell(rdcl_scope.clone())?)
                },
                Deref @ CLASS_NODE { parentScope: __node_parentScope, .. } => if (ignoreBaseClass) {return Ok(getDerivedNode(fromCell(__node_parentScope.clone())?, true)?)} else {return Ok(fromCell(__node_parentScope.clone())?)},
                Deref @ COMPONENT_NODE { .. } => { (node, ignoreRedeclare, ignoreBaseClass) = (classScope(node)?, ignoreRedeclare, ignoreBaseClass); continue '__tco; },
                Deref @ IMPLICIT_SCOPE { parentScope: __node_parentScope, .. } => return Ok(__node_parentScope.clone()),
                _ => return Err("match: no arm matched"),
            } }
        }
    }

    pub fn classScope(mut node: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<InstNode>> {
        let mut scope: metamodelica::Ref<InstNode>;
        scope = (match &*node {
            COMPONENT_NODE {
                component: __node_component,
                ..
            } => Component::classInstance(&(Pointer::access(__node_component.clone())))?,
            _ => node,
        });
        Ok(scope)
    }

    pub(crate) fn libraryScope(mut node: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<InstNode>> {
        '__tco: loop {
            match &*node {
                CLASS_NODE {
                    parentScope: __node_parentScope,
                    ..
                } if (isTopScope(&(fromCell(__node_parentScope.clone())?))) => return Ok(node),
                _ => {
                    node = parentScope(node, false)?;
                    continue '__tco;
                }
            }
        }
    }

    pub fn topScope(mut node: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<InstNode>> {
        '__tco: loop {
            ::match_deref::match_deref! { match &(node.clone()) {
                Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::TOP_SCOPE { .. }, .. } => return Ok(node),
                Deref @ EMPTY_NODE { .. } => {
                    Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInstNode.InstNode.topScope")); __mm_s.push_str(&*literal!(" walked past the top scope: a parent cell lost its node")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("NFFrontEnd/NFInstNode.mo"))?;
                    return Ok(return Err("fail"))
                },
                _ => { node = parent(&node)?; continue '__tco; },
                _ => return Err("match: no arm matched"),
            } }
        }
    }

    pub(crate) fn annotationScope(mut node: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<InstNode>> {
        let mut annScope: metamodelica::Ref<InstNode>;
        annScope = (::match_deref::match_deref! { match &(node.clone()) {
            Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::TOP_SCOPE { annotationScope: __esc_annScope, .. }, .. } => {
                annScope = (*__esc_annScope).clone();
                annScope.clone()
            },
            _ => annotationScope(parentScope(node, false)?)?,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(annScope)
    }

    pub(crate) fn isTopScope(mut node: &metamodelica::Ref<InstNode>) -> bool {
        let mut res: bool;
        res = (::match_deref::match_deref! { match node {
            Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::TOP_SCOPE { .. }, .. } => true,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        res
    }

    pub(crate) fn topComponent(mut node: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<InstNode>> {
        '__tco: loop {
            match &*node {
                COMPONENT_NODE {
                    parent: __node_parent, ..
                } if (isEmpty(&(fromCell(__node_parent.clone())?))) => return Ok(node),
                COMPONENT_NODE {
                    parent: __node_parent, ..
                } => {
                    node = fromCell(__node_parent.clone())?;
                    continue '__tco;
                }
                _ => return Err("match: no arm matched"),
            }
        }
    }

    pub(crate) fn setParent(
        mut parent: metamodelica::Ref<InstNode>,
        mut node: metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode> = node;
        let () = (match &*node {
            CLASS_NODE { .. } => {
                assign_variant_field!(node => InstNode::CLASS_NODE; parentScope = identityCell(parent));
                ()
            }
            COMPONENT_NODE { .. } => {
                assign_variant_field!(node => InstNode::COMPONENT_NODE; parent = identityCell(parent));
                ()
            }
            IMPLICIT_SCOPE { .. } => {
                assign_variant_field!(node => InstNode::IMPLICIT_SCOPE; parentScope = parent);
                ()
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(node)
    }

    pub(crate) fn setOrphanParent(
        mut parent: metamodelica::Ref<InstNode>,
        mut node: metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode> = node;
        let () = (match &*node {
            CLASS_NODE {
                parentScope: __node_parentScope,
                ..
            } if (isEmpty(&(fromCell(__node_parentScope.clone())?))) => {
                assign_variant_field!(node => InstNode::CLASS_NODE; parentScope = identityCell(parent));
                ()
            }
            COMPONENT_NODE {
                parent: __node_parent, ..
            } if (isEmpty(&(fromCell(__node_parent.clone())?))) => {
                assign_variant_field!(node => InstNode::COMPONENT_NODE; parent = identityCell(parent));
                ()
            }
            _ => (),
        });
        Ok(node)
    }

    pub fn getClass(mut node: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<Class::NFClass>> {
        '__tco: loop {
            match &*node {
                CLASS_NODE { cls: __node_cls, .. } => return Ok(Pointer::access(__node_cls.clone())),
                COMPONENT_NODE {
                    component: __node_component,
                    ..
                } => {
                    node = Component::classInstance(&(Pointer::access(__node_component.clone())))?;
                    continue '__tco;
                }
                _ => return Err("match: no arm matched"),
            }
        }
    }

    pub(crate) fn getDerivedClass(mut node: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<Class::NFClass>> {
        let mut cls: metamodelica::Ref<Class::NFClass>;
        cls = (match &*node {
            CLASS_NODE { .. } => getClass(getDerivedNode(node, true)?)?,
            COMPONENT_NODE {
                component: __node_component,
                ..
            } => getClass(getDerivedNode(
                Component::classInstance(&(Pointer::access(__node_component.clone())))?,
                true,
            )?)?,
            _ => return Err("match: no arm matched"),
        });
        Ok(cls)
    }

    pub fn getDerivedNode(
        mut node: metamodelica::Ref<InstNode>,
        mut recursive: bool,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut derived: metamodelica::Ref<InstNode>;
        derived = (match &*node.clone() {
            CLASS_NODE {
                nodeType: __node_nodeType,
                ..
            } => getDerivedNode2(&node, metamodelica::AsArg::as_arg(&__node_nodeType), recursive)?,
            _ => node,
        });
        Ok(derived)
    }

    pub(crate) fn getDerivedNode2<'__b>(
        mut node: &'__b metamodelica::Ref<InstNode>,
        mut ty: &'__b metamodelica::Ref<InstNodeType>,
        mut recursive: bool,
    ) -> Result<metamodelica::Ref<InstNode>> {
        '__tco: loop {
            match &**ty {
                InstNodeType::BASE_CLASS { .. } => {
                    if (recursive) {
                        return Ok(getDerivedNode(
                            fromCell(var_field!((**ty).parent, InstNodeType::BASE_CLASS).clone())?,
                            true,
                        )?);
                    } else {
                        return Ok(fromCell(var_field!((**ty).parent, InstNodeType::BASE_CLASS).clone())?);
                    }
                }
                InstNodeType::DERIVED_CLASS { .. } => {
                    (node, ty, recursive) = (node, var_field!((**ty).ty, InstNodeType::DERIVED_CLASS), recursive);
                    continue '__tco;
                }
                _ => return Ok(node.clone()),
            }
        }
    }

    pub(crate) fn updateClass(
        mut cls: metamodelica::Ref<Class::NFClass>,
        mut node: metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode> = node;
        node = (match &*node {
            CLASS_NODE { cls: __node_cls, .. } => {
                Pointer::update(__node_cls.clone(), cls);
                CachedData::clearTypeCache(var_field!((*node).caches, InstNode::CLASS_NODE).clone())?;
                node
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(node)
    }

    pub fn component(mut node: &metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<Component::NFComponent>> {
        let mut component: metamodelica::Ref<Component::NFComponent>;
        component = (match &**node {
            COMPONENT_NODE {
                component: __node_component,
                ..
            } => Pointer::access(__node_component.clone()),
            VAR_NODE { .. } => crate::NFComponent::interned_WILD(),
            NAME_NODE { .. } => crate::NFComponent::interned_WILD(),
            _ => return Err("match: no arm matched"),
        });
        Ok(component)
    }

    pub(crate) fn updateComponent(
        mut component: metamodelica::Ref<Component::NFComponent>,
        mut node: metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode> = node;
        node = (match &*node {
            COMPONENT_NODE {
                component: __node_component,
                ..
            } => {
                Pointer::update(__node_component.clone(), component);
                node
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(node)
    }

    pub fn replaceComponent(
        mut component: metamodelica::Ref<Component::NFComponent>,
        mut node: metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode> = node;
        let () = (match &*node {
            COMPONENT_NODE { .. } => {
                assign_variant_field!(node => InstNode::COMPONENT_NODE; component = Pointer::create(component));
                ()
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(node)
    }

    pub fn replaceClass(
        mut cls: metamodelica::Ref<Class::NFClass>,
        mut node: metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode> = node;
        let () = (match &*node {
            CLASS_NODE { .. } => {
                assign_variant_field!(node => InstNode::CLASS_NODE; cls = Pointer::create(cls));
                ()
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(node)
    }

    pub(crate) fn nodeType(mut node: &metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<InstNodeType>> {
        let mut nodeType: metamodelica::Ref<InstNodeType>;
        nodeType = (match &**node {
            CLASS_NODE {
                nodeType: __node_nodeType,
                ..
            } => __node_nodeType.clone(),
            COMPONENT_NODE {
                nodeType: __node_nodeType,
                ..
            } => __node_nodeType.clone(),
            _ => return Err("match: no arm matched"),
        });
        Ok(nodeType)
    }

    pub(crate) fn derivedNodeType(mut node: &metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<InstNodeType>> {
        let mut ty: metamodelica::Ref<InstNodeType>;
        ty = (::match_deref::match_deref! { match node {
            Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::DERIVED_CLASS { ty: __esc_ty }, .. } => {
                ty = (*__esc_ty).clone();
                ty.clone()
            },
            _ => nodeType(node)?,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(ty)
    }

    pub fn setNodeType(
        mut nodeType: metamodelica::Ref<InstNodeType>,
        mut node: metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode> = node;
        let () = (match &*node {
            CLASS_NODE { .. } => {
                assign_variant_field!(node => InstNode::CLASS_NODE; nodeType = nodeType);
                CachedData::clearTypeCache(var_field!((*node).caches, InstNode::CLASS_NODE).clone())?;
                ()
            }
            COMPONENT_NODE { .. } => {
                assign_variant_field!(node => InstNode::COMPONENT_NODE; nodeType = nodeType);
                ()
            }
            _ => (),
        });
        Ok(node)
    }

    pub fn definition(mut node: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<SCode::Element>> {
        let mut definition: metamodelica::Ref<SCode::Element>;
        definition = (::match_deref::match_deref! { match &(node.clone()) {
            Deref @ CLASS_NODE { definition: __node_definition, .. } => __node_definition.clone(),
            Deref @ COMPONENT_NODE { definition: Some(__esc_definition), .. } => {
                definition = (*__esc_definition).clone();
                definition.clone()
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInstNode.InstNode.definition")); __mm_s.push_str(&*literal!(" failed for non class/component node: ")); __mm_s.push_str(&*toString(node)?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(definition)
    }

    pub(crate) fn classDefinition(mut node: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<SCode::Element>> {
        '__tco: loop {
            match &*node {
                CLASS_NODE {
                    definition: __node_definition,
                    ..
                } => return Ok(__node_definition.clone()),
                COMPONENT_NODE {
                    component: __node_component,
                    ..
                } => {
                    node = Component::classInstance(&(Pointer::access(__node_component.clone())))?;
                    continue '__tco;
                }
                _ => {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NFInstNode.InstNode.classDefinition"));
                            __mm_s.push_str(&*literal!(" failed for non class/component node: "));
                            __mm_s.push_str(&*toString(node)?);
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                    return Ok(return Err("fail"));
                }
            }
        }
    }

    pub fn extendsDefinition(
        mut node: &metamodelica::Ref<InstNode>,
    ) -> Result<Option<metamodelica::Ref<SCode::Element>>> {
        let mut definition: Option<metamodelica::Ref<SCode::Element>>;
        let mut ty: metamodelica::Ref<InstNodeType>;
        ty = derivedNodeType(node)?;
        definition = (match &*ty {
            InstNodeType::BASE_CLASS {
                definition: __ty_definition,
                ..
            } => Some(__ty_definition.clone()),
            _ => None,
        });
        Ok(definition)
    }

    pub fn setDefinition(
        mut definition: metamodelica::Ref<SCode::Element>,
        mut node: metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode> = node;
        let () = (match &*node {
            CLASS_NODE { .. } => {
                assign_variant_field!(node => InstNode::CLASS_NODE; definition = definition);
                ()
            }
            COMPONENT_NODE { .. } => {
                assign_variant_field!(node => InstNode::COMPONENT_NODE; definition = Some(definition));
                ()
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(node)
    }

    pub fn setComponentDirection(
        mut direction: Prefixes::Direction,
        mut node: metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode> = node;
        node = (match &*node {
            COMPONENT_NODE {
                component: __node_component,
                ..
            } => {
                assign_variant_field!(node => InstNode::COMPONENT_NODE; component = Pointer::create(Component::setDirection(direction, Pointer::access(__node_component.clone()))));
                node
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFInstNode.InstNode.setComponentDirection"));
                        __mm_s.push_str(&*literal!(" failed for non component node: "));
                        __mm_s.push_str(&*toString(node)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(node)
    }

    pub fn info(mut node: &metamodelica::Ref<InstNode>) -> SourceInfo {
        let mut info: SourceInfo;
        info = 'mc: {
            let __mc_input = &**node;
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ CLASS_NODE { nodeType: ty @ Deref @ InstNodeType::BASE_CLASS { .. }, .. } => {
                        Ok(SCodeUtil::elementInfo(var_field!((**ty).definition, InstNodeType::BASE_CLASS)))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ CLASS_NODE { .. } => {
                        Ok(SCodeUtil::elementInfo(var_field!((**node).definition, InstNode::CLASS_NODE)))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ COMPONENT_NODE { .. } => {
                        Ok(Component::info(&(Pointer::access(var_field!((**node).component, InstNode::COMPONENT_NODE).clone())))?)
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ COMPONENT_NODE { .. } => {
                        Ok(self::info(&(fromCell(var_field!((**node).parent, InstNode::COMPONENT_NODE).clone())?)))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        Ok(Absyn::dummyInfo.clone())
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            panic!("matchcontinue: no arm matched")
        };
        info
    }

    pub fn getType(mut node: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<Type::NFType>> {
        let mut ty: metamodelica::Ref<Type::NFType>;
        let mut var: metamodelica::Ref<Variable::NFVariable>;
        ty = (match &*node.clone() {
            CLASS_NODE { cls: __node_cls, .. } => Class::getType(Pointer::access(__node_cls.clone()), node)?,
            COMPONENT_NODE {
                component: __node_component,
                ..
            } => Component::getType(&(Pointer::access(__node_component.clone())))?,
            VAR_NODE {
                varPointer: __node_varPointer,
                ..
            } => {
                var = Pointer::access(PointerWeak::upgrade(__node_varPointer.clone())?);
                var.ty.clone()
            }
            NAME_NODE { .. } => crate::NFType::interned_UNKNOWN(),
            _ => return Err("match: no arm matched"),
        });
        Ok(ty)
    }

    pub(crate) fn classApply<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
        mut node: metamodelica::Ref<InstNode>,
        mut func: &dyn ::std::ops::Fn(ArgT, metamodelica::Ref<Class::NFClass>) -> Result<metamodelica::Ref<Class::NFClass>>,
        mut arg: ArgT,
    ) -> Result<metamodelica::Ref<InstNode>> {
        pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
            dyn ::std::ops::Fn(ArgT, metamodelica::Ref<Class::NFClass>) -> Result<metamodelica::Ref<Class::NFClass>>
                + 'static,
        >;

        let mut node: metamodelica::Ref<InstNode> = node;
        let () = (match &*node {
            CLASS_NODE { cls: __node_cls, .. } => {
                Pointer::update(__node_cls.clone(), func(arg, Pointer::access(__node_cls.clone()))?);
                ()
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(node)
    }

    pub(crate) fn componentApply<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
        mut node: metamodelica::Ref<InstNode>,
        mut func: &dyn ::std::ops::Fn(
            ArgT,
            metamodelica::Ref<Component::NFComponent>,
        ) -> Result<metamodelica::Ref<Component::NFComponent>>,
        mut arg: ArgT,
    ) -> Result<metamodelica::Ref<InstNode>> {
        pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
            dyn ::std::ops::Fn(
                    ArgT,
                    metamodelica::Ref<Component::NFComponent>,
                ) -> Result<metamodelica::Ref<Component::NFComponent>>
                + 'static,
        >;

        let mut node: metamodelica::Ref<InstNode> = node;
        let () = (match &*node {
            COMPONENT_NODE {
                component: __node_component,
                ..
            } => {
                Pointer::update(
                    __node_component.clone(),
                    func(arg, Pointer::access(__node_component.clone()))?,
                );
                ()
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(node)
    }

    pub(crate) fn scopeList(
        mut node: metamodelica::Ref<InstNode>,
        mut includeRoot: bool,
        mut accumScopes: metamodelica::List<metamodelica::Ref<InstNode>>,
    ) -> Result<metamodelica::List<metamodelica::Ref<InstNode>>> {
        '__tco: loop {
            ::match_deref::match_deref! { match &(node.clone()) {
                Deref @ CLASS_NODE { nodeType: __node_nodeType, .. } => {
                    return Ok(scopeListClass(&node, metamodelica::AsArg::as_arg(&__node_nodeType), includeRoot, &accumScopes)?)
                },
                Deref @ COMPONENT_NODE { parent: __node_parent, .. } if (isEmpty(&(borrow(__node_parent.clone())?))) => {
                    return Ok(accumScopes)
                },
                Deref @ COMPONENT_NODE { nodeType: Deref @ InstNodeType::REDECLARED_COMP { parent: rdcl_scope }, .. } => {
                    { (node, includeRoot, accumScopes) = (borrow(rdcl_scope.clone())?, includeRoot, metamodelica::cons(node, accumScopes)); continue '__tco; }
                },
                Deref @ COMPONENT_NODE { parent: __node_parent, .. } => {
                    { (node, includeRoot, accumScopes) = (borrow(__node_parent.clone())?, includeRoot, metamodelica::cons(node, accumScopes)); continue '__tco; }
                },
                Deref @ IMPLICIT_SCOPE { parentScope: __node_parentScope, .. } => {
                    { (node, includeRoot, accumScopes) = (__node_parentScope.clone(), includeRoot, accumScopes); continue '__tco; }
                },
                _ => {
                    return Ok(accumScopes)
                },
                _ => return Err("match: no arm matched"),
            } }
        }
    }

    pub(crate) fn scopeListClass<'__b>(
        mut clsNode: &'__b metamodelica::Ref<InstNode>,
        mut ty: &'__b metamodelica::Ref<InstNodeType>,
        mut includeRoot: bool,
        mut accumScopes: &'__b metamodelica::List<metamodelica::Ref<InstNode>>,
    ) -> Result<metamodelica::List<metamodelica::Ref<InstNode>>> {
        '__tco: loop {
            match &**ty {
                InstNodeType::NORMAL_CLASS => {
                    return Ok(scopeList(
                        borrowParent(clsNode)?,
                        includeRoot,
                        metamodelica::cons(clsNode.clone(), accumScopes.clone()),
                    )?);
                }
                InstNodeType::BASE_CLASS { .. } => {
                    return Ok(scopeList(
                        borrow(var_field!((**ty).parent, InstNodeType::BASE_CLASS).clone())?,
                        includeRoot,
                        accumScopes.clone(),
                    )?);
                }
                InstNodeType::DERIVED_CLASS { .. } => {
                    (clsNode, ty, includeRoot, accumScopes) = (
                        clsNode,
                        var_field!((**ty).ty, InstNodeType::DERIVED_CLASS),
                        includeRoot,
                        accumScopes,
                    );
                    continue '__tco;
                }
                InstNodeType::BUILTIN_CLASS => return Ok(metamodelica::cons(clsNode.clone(), accumScopes.clone())),
                InstNodeType::TOP_SCOPE { .. } => return Ok(accumScopes.clone()),
                InstNodeType::ROOT_CLASS { .. } => {
                    if (includeRoot) {
                        return Ok(scopeList(
                            borrowParent(clsNode)?,
                            includeRoot,
                            metamodelica::cons(clsNode.clone(), accumScopes.clone()),
                        )?);
                    } else {
                        return Ok(accumScopes.clone());
                    }
                }
                InstNodeType::REDECLARED_CLASS { .. } => {
                    return Ok(scopeList(
                        borrow(var_field!((**ty).parent, InstNodeType::REDECLARED_CLASS).clone())?,
                        includeRoot,
                        metamodelica::cons(getDerivedNode(clsNode.clone(), true)?, accumScopes.clone()),
                    )?);
                }
                InstNodeType::IMPLICIT_SCOPE => {
                    return Ok(scopeList(borrowParent(clsNode)?, includeRoot, accumScopes.clone())?);
                }
                _ => {
                    Error::terminate(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NFInstNode.InstNode.scopeListClass"));
                            __mm_s.push_str(&*literal!(" got unknown node type"));
                            ArcStr::from(__mm_s)
                        },
                        &(metamodelica::sourceInfo!("NFFrontEnd/NFInstNode.mo")),
                    )?;
                    return Ok(return Err("fail"));
                }
            }
        }
    }

    pub(crate) fn getAnnotation(
        mut name: &ArcStr,
        mut node: metamodelica::Ref<InstNode>,
    ) -> Result<(metamodelica::Ref<SCode::Mod>, metamodelica::Ref<InstNode>)> {
        let mut r#mod: metamodelica::Ref<SCode::Mod>;
        let mut scope: metamodelica::Ref<InstNode> = node;
        let mut ann: Option<metamodelica::Ref<SCode::Annotation>>;
        while isComponent(&scope)? {
            ann = SCodeUtil::commentAnnotation(&(Component::comment(&(component(&scope)?))?));
            if (ann).is_some() {
                r#mod = SCodeUtil::lookupAnnotation(&(Util::getOption(ann)?), name);
                if !(SCodeUtil::isEmptyMod(&r#mod)) {
                    scope = instanceParent(scope)?;
                    return Ok((r#mod, scope));
                }
            }
            scope = borrowInstanceParent(scope)?;
        }
        r#mod = openmodelica_frontend_types::SCode::Mod::interned_NOMOD();
        Ok((r#mod, scope))
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
    #[repr(i32)]
    pub enum ScopeType {
        /// Stops at a root class and doesn't include the root
        RELATIVE = 1,
        /// Stops at a root class and includes the root
        INCLUDING_ROOT = 2,
        /// Stops at the top scope
        FULL = 3,
    }
    impl PartialOrd for ScopeType {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for ScopeType {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            (*self as i32).cmp(&(*other as i32))
        }
    }
    impl metamodelica::gc::MMTrace for ScopeType {
        fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            Ok(())
        }
    }

    pub(crate) fn rootPath(
        mut node: metamodelica::Ref<InstNode>,
        mut ignoreBaseClass: bool,
    ) -> Result<metamodelica::Ref<Absyn::Path>> {
        let mut path: metamodelica::Ref<Absyn::Path> =
            scopePath(node.clone(), ScopeType::INCLUDING_ROOT.clone(), ignoreBaseClass)?;
        Ok(path)
    }

    pub fn fullPath(
        mut node: metamodelica::Ref<InstNode>,
        mut ignoreBaseClass: bool,
    ) -> Result<metamodelica::Ref<Absyn::Path>> {
        let mut path: metamodelica::Ref<Absyn::Path> =
            scopePath(node.clone(), ScopeType::FULL.clone(), ignoreBaseClass)?;
        Ok(path)
    }

    pub(crate) fn scopePath(
        mut node: metamodelica::Ref<InstNode>,
        mut scopeType: ScopeType,
        mut ignoreBaseClass: bool,
    ) -> Result<metamodelica::Ref<Absyn::Path>> {
        '__tco: loop {
            match &*node {
                CLASS_NODE {
                    nodeType: it,
                    name: __node_name,
                    parentScope: __node_parentScope,
                    ..
                } => match &*it.clone() {
                    InstNodeType::BASE_CLASS {
                        parent: __it_parent, ..
                    } if (!(ignoreBaseClass)) => {
                        (node, scopeType, ignoreBaseClass) = (fromCell(__it_parent.clone())?, scopeType, false);
                        continue '__tco;
                    }
                    _ => {
                        return Ok(scopePath2(
                            fromCell(__node_parentScope.clone())?,
                            scopeType,
                            metamodelica::Ref::new(Absyn::Path::IDENT {
                                name: __node_name.clone(),
                            }),
                        )?);
                    }
                },
                COMPONENT_NODE {
                    name: __node_name,
                    parent: __node_parent,
                    ..
                } => {
                    return Ok(scopePath2(
                        fromCell(__node_parent.clone())?,
                        scopeType,
                        metamodelica::Ref::new(Absyn::Path::IDENT {
                            name: __node_name.clone(),
                        }),
                    )?);
                }
                IMPLICIT_SCOPE {
                    parentScope: __node_parentScope,
                    ..
                } => {
                    (node, scopeType, ignoreBaseClass) = (__node_parentScope.clone(), scopeType, false);
                    continue '__tco;
                }
                _ => return Ok(metamodelica::Ref::new(Absyn::Path::IDENT { name: name(&node)? })),
            }
        }
    }

    pub(crate) fn scopePath2(
        mut node: metamodelica::Ref<InstNode>,
        mut scopeType: ScopeType,
        mut accumPath: metamodelica::Ref<Absyn::Path>,
    ) -> Result<metamodelica::Ref<Absyn::Path>> {
        '__tco: loop {
            match &*node.clone() {
                CLASS_NODE {
                    nodeType: __node_nodeType,
                    ..
                } => {
                    return Ok(scopePathClass(
                        &node,
                        metamodelica::AsArg::as_arg(&__node_nodeType),
                        scopeType,
                        &accumPath,
                    )?);
                }
                COMPONENT_NODE {
                    name: __node_name,
                    parent: __node_parent,
                    ..
                } => {
                    (node, scopeType, accumPath) = (
                        fromCell(__node_parent.clone())?,
                        scopeType,
                        metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                            name: __node_name.clone(),
                            path: accumPath,
                        }),
                    );
                    continue '__tco;
                }
                _ => return Ok(accumPath),
            }
        }
    }

    pub(crate) fn scopePathClass<'__b>(
        mut node: &'__b metamodelica::Ref<InstNode>,
        mut ty: &'__b metamodelica::Ref<InstNodeType>,
        mut scopeType: ScopeType,
        mut accumPath: &'__b metamodelica::Ref<Absyn::Path>,
    ) -> Result<metamodelica::Ref<Absyn::Path>> {
        '__tco: loop {
            match &**ty {
                InstNodeType::NORMAL_CLASS => {
                    return Ok(scopePath2(
                        classParent(node)?,
                        scopeType,
                        metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                            name: className(node)?,
                            path: accumPath.clone(),
                        }),
                    )?);
                }
                InstNodeType::BASE_CLASS { .. } => {
                    return Ok(scopePath2(
                        fromCell(var_field!((**ty).parent, InstNodeType::BASE_CLASS).clone())?,
                        scopeType,
                        accumPath.clone(),
                    )?);
                }
                InstNodeType::DERIVED_CLASS { .. } => {
                    (node, ty, scopeType, accumPath) = (
                        node,
                        var_field!((**ty).ty, InstNodeType::DERIVED_CLASS),
                        scopeType,
                        accumPath,
                    );
                    continue '__tco;
                }
                InstNodeType::BUILTIN_CLASS => {
                    return Ok(metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                        name: className(node)?,
                        path: accumPath.clone(),
                    }));
                }
                InstNodeType::TOP_SCOPE { .. } => return Ok(accumPath.clone()),
                InstNodeType::ROOT_CLASS { .. } => {
                    if (scopeType == ScopeType::FULL.clone()) {
                        return Ok(scopePath2(
                            classParent(node)?,
                            scopeType,
                            metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                                name: className(node)?,
                                path: accumPath.clone(),
                            }),
                        )?);
                    } else if (scopeType == ScopeType::INCLUDING_ROOT.clone()) {
                        return Ok(metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                            name: className(node)?,
                            path: accumPath.clone(),
                        }));
                    } else {
                        return Ok(accumPath.clone());
                    }
                }
                InstNodeType::REDECLARED_CLASS { .. } => {
                    return Ok(scopePath2(
                        fromCell(var_field!((**ty).parent, InstNodeType::REDECLARED_CLASS).clone())?,
                        scopeType,
                        metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                            name: className(node)?,
                            path: accumPath.clone(),
                        }),
                    )?);
                }
                InstNodeType::IMPLICIT_SCOPE => {
                    return Ok(scopePath2(classParent(node)?, scopeType, accumPath.clone())?);
                }
                _ => {
                    Error::terminate(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NFInstNode.InstNode.scopePathClass"));
                            __mm_s.push_str(&*literal!(" got unknown node type"));
                            ArcStr::from(__mm_s)
                        },
                        &(metamodelica::sourceInfo!("NFFrontEnd/NFInstNode.mo")),
                    )?;
                    return Ok(return Err("fail"));
                }
            }
        }
    }

    pub(crate) fn isInput(mut node: &metamodelica::Ref<InstNode>) -> bool {
        let mut isInput: bool;
        isInput = (match &**node {
            COMPONENT_NODE {
                component: __node_component,
                ..
            } => Component::isInput(&(Pointer::access(__node_component.clone()))),
            _ => false,
        });
        isInput
    }

    pub(crate) fn isOutput(mut node: &metamodelica::Ref<InstNode>) -> bool {
        let mut isOutput: bool;
        isOutput = (match &**node {
            COMPONENT_NODE {
                component: __node_component,
                ..
            } => Component::isOutput(&(Pointer::access(__node_component.clone()))),
            _ => false,
        });
        isOutput
    }

    pub(crate) fn isInner<'__b>(mut node: &'__b metamodelica::Ref<InstNode>) -> Result<bool> {
        '__tco: loop {
            match &**node {
                COMPONENT_NODE { .. } => {
                    return Ok(Component::isInner(
                        &(Pointer::access(var_field!((**node).component, InstNode::COMPONENT_NODE).clone())),
                    )?);
                }
                CLASS_NODE { .. } => {
                    return Ok(AbsynUtil::isInner(SCodeUtil::prefixesInnerOuter(
                        &(SCodeUtil::elementPrefixes(var_field!((**node).definition, InstNode::CLASS_NODE))?),
                    )));
                }
                INNER_OUTER_NODE { .. } => {
                    node = var_field!((**node).outerNode, InstNode::INNER_OUTER_NODE);
                    continue '__tco;
                }
                _ => return Ok(false),
            }
        }
    }

    pub(crate) fn isOuter<'__b>(mut node: &'__b metamodelica::Ref<InstNode>) -> Result<bool> {
        '__tco: loop {
            match &**node {
                COMPONENT_NODE { .. } => {
                    return Ok(Component::isOuter(
                        &(Pointer::access(var_field!((**node).component, InstNode::COMPONENT_NODE).clone())),
                    )?);
                }
                CLASS_NODE { .. } => {
                    return Ok(AbsynUtil::isOuter(SCodeUtil::prefixesInnerOuter(
                        &(SCodeUtil::elementPrefixes(var_field!((**node).definition, InstNode::CLASS_NODE))?),
                    )));
                }
                INNER_OUTER_NODE { .. } => {
                    node = var_field!((**node).outerNode, InstNode::INNER_OUTER_NODE);
                    continue '__tco;
                }
                _ => return Ok(false),
            }
        }
    }

    pub(crate) fn isOnlyOuter<'__b>(mut node: &'__b metamodelica::Ref<InstNode>) -> Result<bool> {
        '__tco: loop {
            match &**node {
                COMPONENT_NODE { .. } => {
                    return Ok(Component::isOnlyOuter(
                        &(Pointer::access(var_field!((**node).component, InstNode::COMPONENT_NODE).clone())),
                    )?);
                }
                CLASS_NODE { .. } => {
                    return Ok(AbsynUtil::isOnlyOuter(SCodeUtil::prefixesInnerOuter(
                        &(SCodeUtil::elementPrefixes(var_field!((**node).definition, InstNode::CLASS_NODE))?),
                    )));
                }
                INNER_OUTER_NODE { .. } => {
                    node = var_field!((**node).outerNode, InstNode::INNER_OUTER_NODE);
                    continue '__tco;
                }
                _ => return Ok(false),
            }
        }
    }

    pub(crate) fn isInnerOuterNode(mut node: &metamodelica::Ref<InstNode>) -> bool {
        let mut isIO: bool;
        isIO = (match &**node {
            INNER_OUTER_NODE { .. } => true,
            _ => false,
        });
        isIO
    }

    pub fn isGeneratedInner(mut node: &metamodelica::Ref<InstNode>) -> bool {
        let mut isInner: bool;
        isInner = (::match_deref::match_deref! { match node {
            Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::GENERATED_INNER, .. } => true,
            Deref @ COMPONENT_NODE { nodeType: Deref @ InstNodeType::GENERATED_INNER, .. } => true,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        isInner
    }

    pub fn resolveInner(mut node: metamodelica::Ref<InstNode>) -> metamodelica::Ref<InstNode> {
        let mut innerNode: metamodelica::Ref<InstNode>;
        innerNode = (match &*node {
            INNER_OUTER_NODE {
                innerNode: __node_innerNode,
                ..
            } => __node_innerNode.clone(),
            _ => node,
        });
        innerNode
    }

    pub fn resolveOuter(mut node: metamodelica::Ref<InstNode>) -> metamodelica::Ref<InstNode> {
        let mut outerNode: metamodelica::Ref<InstNode>;
        outerNode = (match &*node {
            INNER_OUTER_NODE {
                outerNode: __node_outerNode,
                ..
            } => __node_outerNode.clone(),
            _ => node,
        });
        outerNode
    }

    pub(crate) fn cacheInitFunc(mut node: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode> = node;
        let () = (match &*node {
            CLASS_NODE { .. } => {
                CachedData::initFunc(var_field!((*node).caches, InstNode::CLASS_NODE).clone())?;
                ()
            }
            _ => {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFInstNode.InstNode.cacheInitFunc"));
                        __mm_s.push_str(&*literal!(" got node without cache"));
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFInstNode.mo")),
                )?;
                return Err("fail");
            }
        });
        Ok(node)
    }

    pub(crate) fn cacheAddFunc(
        mut node: metamodelica::Ref<InstNode>,
        mut r#fn: metamodelica::Ref<Function::Function>,
        mut specialBuiltin: bool,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode> = node;
        let () = (match &*node {
            CLASS_NODE { .. } => {
                CachedData::addFunc(
                    r#fn,
                    specialBuiltin,
                    var_field!((*node).caches, InstNode::CLASS_NODE).clone(),
                )?;
                ()
            }
            _ => {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFInstNode.InstNode.cacheAddFunc"));
                        __mm_s.push_str(&*literal!(" got node without cache"));
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFInstNode.mo")),
                )?;
                return Err("fail");
            }
        });
        Ok(node)
    }

    pub fn newFuncCache(
        mut node: metamodelica::Ref<InstNode>,
        mut in_func_cache: metamodelica::Ref<CachedData::CachedData>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode> = node;
        let () = (match &*node {
            CLASS_NODE { .. } => {
                assign_variant_field!(node => InstNode::CLASS_NODE; caches = arrayCreate(1, in_func_cache));
                ()
            }
            _ => {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFInstNode.InstNode.newFuncCache"));
                        __mm_s.push_str(&*literal!(" got node without cache"));
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFInstNode.mo")),
                )?;
                return Err("fail");
            }
        });
        Ok(node)
    }

    pub(crate) fn getFuncCache(
        mut inNode: &metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<CachedData::CachedData>> {
        let mut func_cache: metamodelica::Ref<CachedData::CachedData>;
        func_cache = (match &**inNode {
            CLASS_NODE { .. } => CachedData::getFuncCache(var_field!((**inNode).caches, InstNode::CLASS_NODE).clone())?,
            _ => {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFInstNode.InstNode.getFuncCache"));
                        __mm_s.push_str(&*literal!(" got node without cache"));
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFInstNode.mo")),
                )?;
                return Err("fail");
            }
        });
        Ok(func_cache)
    }

    pub(crate) fn setFuncCache(
        mut node: metamodelica::Ref<InstNode>,
        mut in_func_cache: metamodelica::Ref<CachedData::CachedData>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode> = node;
        let () = (match &*node {
            CLASS_NODE { .. } => {
                CachedData::setFuncCache(var_field!((*node).caches, InstNode::CLASS_NODE).clone(), in_func_cache)?;
                ()
            }
            _ => {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFInstNode.InstNode.setFuncCache"));
                        __mm_s.push_str(&*literal!(" got node without cache"));
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFInstNode.mo")),
                )?;
                return Err("fail");
            }
        });
        Ok(node)
    }

    pub(crate) fn getPackageCache(
        mut inNode: &metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<CachedData::CachedData>> {
        let mut pack_cache: metamodelica::Ref<CachedData::CachedData>;
        pack_cache = (match &**inNode {
            CLASS_NODE { .. } => {
                CachedData::getPackageCache(var_field!((**inNode).caches, InstNode::CLASS_NODE).clone())?
            }
            _ => {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFInstNode.InstNode.getPackageCache"));
                        __mm_s.push_str(&*literal!(" got node without cache"));
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFInstNode.mo")),
                )?;
                return Err("fail");
            }
        });
        Ok(pack_cache)
    }

    pub(crate) fn setPackageCache(
        mut node: metamodelica::Ref<InstNode>,
        mut packageNode: metamodelica::Ref<InstNode>,
        mut state: PackageCacheState,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode> = node;
        let () = (match &*node {
            CLASS_NODE { .. } => {
                CachedData::setPackageCache(
                    var_field!((*node).caches, InstNode::CLASS_NODE).clone(),
                    metamodelica::Ref::new(CachedData::CachedData::PACKAGE {
                        instance: handle(packageNode)?,
                        state: state,
                    }),
                )?;
                ()
            }
            _ => {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFInstNode.InstNode.setPackageCache"));
                        __mm_s.push_str(&*literal!(" got node without cache"));
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFInstNode.mo")),
                )?;
                return Err("fail");
            }
        });
        Ok(node)
    }

    pub(crate) fn clearPackageCache(mut node: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode> = node;
        let () = (match &*node {
            CLASS_NODE { .. } => {
                CachedData::clearPackageCache(var_field!((*node).caches, InstNode::CLASS_NODE).clone())?;
                ()
            }
            _ => {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFInstNode.InstNode.clearPackageCache"));
                        __mm_s.push_str(&*literal!(" got node without cache"));
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFInstNode.mo")),
                )?;
                return Err("fail");
            }
        });
        Ok(node)
    }

    pub(crate) fn openImplicitScope(mut scope: metamodelica::Ref<InstNode>) -> metamodelica::Ref<InstNode> {
        let mut scope: metamodelica::Ref<InstNode> = scope;
        scope = (match &*scope {
            IMPLICIT_SCOPE { .. } => scope,
            _ => metamodelica::Ref::new(InstNode::IMPLICIT_SCOPE {
                parentScope: scope,
                locals: metamodelica::nil(),
            }),
        });
        scope
    }

    pub(crate) fn explicitScope<'__b>(mut node: &'__b metamodelica::Ref<InstNode>) -> metamodelica::Ref<InstNode> {
        '__tco: loop {
            match &**node {
                IMPLICIT_SCOPE { .. } => {
                    node = var_field!((**node).parentScope, InstNode::IMPLICIT_SCOPE);
                    continue '__tco;
                }
                _ => return node.clone(),
            }
        }
    }

    pub(crate) fn addIterator(
        mut iterator: metamodelica::Ref<InstNode>,
        mut scope: metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut scope: metamodelica::Ref<InstNode> = scope;
        scope = (match &*scope.clone() {
            IMPLICIT_SCOPE {
                locals: __scope_locals, ..
            } => metamodelica::Ref::new(InstNode::IMPLICIT_SCOPE {
                parentScope: scope,
                locals: metamodelica::cons(iterator, __scope_locals.clone()),
            }),
            _ => return Err("match: no arm matched"),
        });
        Ok(scope)
    }

    pub(crate) fn refEqual(
        mut node1: &metamodelica::Ref<InstNode>,
        mut node2: &metamodelica::Ref<InstNode>,
    ) -> Result<bool> {
        let mut refEqual: bool;
        refEqual = (::match_deref::match_deref! { match (node1, node2) {
            (Deref @ CLASS_NODE { .. }, Deref @ CLASS_NODE { .. }) => referenceEq(&*(Pointer::access(var_field!((**node1).cls, InstNode::CLASS_NODE).clone())),&*(Pointer::access(var_field!((**node2).cls, InstNode::CLASS_NODE).clone()))),
            (Deref @ COMPONENT_NODE { .. }, Deref @ COMPONENT_NODE { .. }) => referenceEq(&*(Pointer::access(var_field!((**node1).component, InstNode::COMPONENT_NODE).clone())),&*(Pointer::access(var_field!((**node2).component, InstNode::COMPONENT_NODE).clone()))),
            (Deref @ VAR_NODE { .. }, Deref @ VAR_NODE { .. }) => referenceEq(&*(Pointer::access(PointerWeak::upgrade(var_field!((**node1).varPointer, InstNode::VAR_NODE).clone())?)),&*(Pointer::access(PointerWeak::upgrade(var_field!((**node2).varPointer, InstNode::VAR_NODE).clone())?))),
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(refEqual)
    }

    pub(crate) fn refCompare(
        mut node1: &metamodelica::Ref<InstNode>,
        mut node2: &metamodelica::Ref<InstNode>,
    ) -> Result<i32> {
        let mut res: i32;
        res = (::match_deref::match_deref! { match (node1, node2) {
            (Deref @ CLASS_NODE { .. }, Deref @ CLASS_NODE { .. }) => Util::referenceCompare(Pointer::access(var_field!((**node1).cls, InstNode::CLASS_NODE).clone()), Pointer::access(var_field!((**node2).cls, InstNode::CLASS_NODE).clone())),
            (Deref @ COMPONENT_NODE { .. }, Deref @ COMPONENT_NODE { .. }) => Util::referenceCompare(Pointer::access(var_field!((**node1).component, InstNode::COMPONENT_NODE).clone()), Pointer::access(var_field!((**node2).component, InstNode::COMPONENT_NODE).clone())),
            (Deref @ CLASS_NODE { .. }, Deref @ COMPONENT_NODE { .. }) => Util::referenceCompare(Pointer::access(var_field!((**node1).cls, InstNode::CLASS_NODE).clone()), Pointer::access(var_field!((**node2).component, InstNode::COMPONENT_NODE).clone())),
            (Deref @ COMPONENT_NODE { .. }, Deref @ CLASS_NODE { .. }) => Util::referenceCompare(Pointer::access(var_field!((**node1).component, InstNode::COMPONENT_NODE).clone()), Pointer::access(var_field!((**node2).cls, InstNode::CLASS_NODE).clone())),
            _ => return Err("match: no arm matched"),
        } });
        Ok(res)
    }

    pub fn nameEqual(mut node1: &metamodelica::Ref<InstNode>, mut node2: &metamodelica::Ref<InstNode>) -> Result<bool> {
        let mut equal: bool = metamodelica::stringEq(&(name(node1)?), &(name(node2)?));
        Ok(equal)
    }

    pub(crate) fn isSame(mut node1: metamodelica::Ref<InstNode>, mut node2: metamodelica::Ref<InstNode>) -> bool {
        let mut same: bool = false;
        let mut n1: metamodelica::Ref<InstNode> = resolveOuter(node1.clone());
        let mut n2: metamodelica::Ref<InstNode> = resolveOuter(node2.clone());
        if referenceEq(&*(n1), &*(n2)) {
            same = true;
            return same;
        }
        match '__try0: {
            same = referenceEq(
                &*(unwrap_break_err!(definition(node1.clone()), '__try0)),
                &*(unwrap_break_err!(definition(node2.clone()), '__try0)),
            );
            Ok::<_, &'static str>((same.clone(),))
        } {
            Ok((__try0_o0,)) => {
                same = __try0_o0;
            }
            Err(_) => {
                same = false;
            }
        }
        same
    }

    pub(crate) fn checkIdentical(
        mut node1: metamodelica::Ref<InstNode>,
        mut node2: metamodelica::Ref<InstNode>,
    ) -> Result<()> {
        let mut n1: metamodelica::Ref<InstNode> = resolveOuter(node1.clone());
        let mut n2: metamodelica::Ref<InstNode> = resolveOuter(node2.clone());
        if referenceEq(&*(&*n1), &*(&*n2)) {
            return Ok(());
        }
        let () = 'mc: {
            let __mc_input = (&*n1, &*n2);
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ CLASS_NODE { .. }, Deref @ CLASS_NODE { .. }) => {
                        if !((Class::isIdentical(&(getClass(n1.clone())?), &(getClass(n2.clone())?))?)) { return Err("guard") }
                        Ok(())
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (Deref @ COMPONENT_NODE { .. }, Deref @ COMPONENT_NODE { .. }) => {
                        if !((Component::isIdentical(&(component(&n1)?), &(component(&n2)?))?)) { return Err("guard") }
                        Ok(())
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        Error::addMultiSourceMessage(&(Error::DUPLICATE_ELEMENTS_NOT_IDENTICAL.clone()), &(list![toString(n1.clone())?, toString(n2.clone())?]), &(list![info(&n1), info(&n2)]))?;
                        Ok(return Err("fail"))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            return Err("matchcontinue: no arm matched");
        };
        Ok(())
    }

    pub fn toString(mut node: metamodelica::Ref<InstNode>) -> Result<ArcStr> {
        let mut name: ArcStr;
        name = (match &*node {
            COMPONENT_NODE {
                component: __node_component,
                name: __node_name,
                ..
            } => Component::toString(
                metamodelica::AsArg::as_arg(&__node_name),
                &(Pointer::access(__node_component.clone())),
            )?,
            CLASS_NODE {
                definition: __node_definition,
                ..
            } => SCodeDump::unparseElementStr(__node_definition.clone(), SCodeDump::defaultOptions.clone())?,
            _ => self::name(&node)?,
        });
        Ok(name)
    }

    pub(crate) fn toFlatString(
        mut node: metamodelica::Ref<InstNode>,
        mut format: BaseModelica::OutputFormat,
        mut indent: ArcStr,
    ) -> Result<ArcStr> {
        let mut name: ArcStr;
        name = (match &*node.clone() {
            COMPONENT_NODE {
                component: __node_component,
                name: __node_name,
                ..
            } => Component::toFlatString(
                __node_name.clone(),
                &(Pointer::access(__node_component.clone())),
                format,
                indent,
            )?,
            CLASS_NODE { cls: __node_cls, .. } => {
                Class::toFlatString(&(Pointer::access(__node_cls.clone())), node, format, indent)?
            }
            _ => self::name(&node)?,
        });
        Ok(name)
    }

    pub(crate) fn toFlatStream(
        mut node: metamodelica::Ref<InstNode>,
        mut format: BaseModelica::OutputFormat,
        mut indent: ArcStr,
        mut s: IOStream::IOStream,
    ) -> Result<IOStream::IOStream> {
        let mut s: IOStream::IOStream = s;
        s = (match &*node.clone() {
            COMPONENT_NODE {
                component: __node_component,
                name: __node_name,
                ..
            } => Component::toFlatStream(
                __node_name.clone(),
                &(Pointer::access(__node_component.clone())),
                format,
                indent,
                s,
            )?,
            CLASS_NODE { cls: __node_cls, .. } => {
                Class::toFlatStream(&(Pointer::access(__node_cls.clone())), node, format, indent, s)?
            }
            _ => IOStream::append(s, toFlatString(node, format, indent)?)?,
        });
        Ok(s)
    }

    pub(crate) fn isRedeclare(mut node: metamodelica::Ref<InstNode>) -> Result<bool> {
        let mut isRedeclare: bool;
        isRedeclare = (match &*node {
            CLASS_NODE { .. } => SCodeUtil::isElementRedeclare(&(definition(node)?))?,
            COMPONENT_NODE {
                component: __node_component,
                ..
            } => Component::isRedeclare(&(Pointer::access(__node_component.clone())))?,
            _ => false,
        });
        Ok(isRedeclare)
    }

    pub(crate) fn isRedeclared(mut node: &metamodelica::Ref<InstNode>) -> Result<bool> {
        let mut redeclared: bool;
        redeclared = (match &*(nodeType(node)?) {
            InstNodeType::REDECLARED_COMP { .. } => true,
            InstNodeType::REDECLARED_CLASS { .. } => true,
            _ => false,
        });
        Ok(redeclared)
    }

    pub fn getRedeclaredNode(mut node: metamodelica::Ref<InstNode>) -> metamodelica::Ref<InstNode> {
        let mut outNode: metamodelica::Ref<InstNode>;
        outNode = (::match_deref::match_deref! { match &(node.clone()) {
            Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::REDECLARED_CLASS { originalNode: Some(__esc_outNode), .. }, .. } => {
                outNode = (*__esc_outNode).clone();
                outNode.clone()
            },
            _ => node,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        outNode
    }

    pub fn isReplaceable(mut node: &metamodelica::Ref<InstNode>) -> Result<bool> {
        let mut repl: bool;
        let mut elem: metamodelica::Ref<SCode::Element>;
        repl = (::match_deref::match_deref! { match node {
            Deref @ CLASS_NODE { definition: __node_definition, .. } => SCodeUtil::isElementReplaceable(metamodelica::AsArg::as_arg(&__node_definition))?,
            Deref @ COMPONENT_NODE { definition: Some(__esc_elem), .. } => {
                elem = (*__esc_elem).clone();
                SCodeUtil::isElementReplaceable(metamodelica::AsArg::as_arg(&elem))?
            },
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(repl)
    }

    pub(crate) fn isProtectedBaseClass(mut node: &metamodelica::Ref<InstNode>) -> bool {
        let mut isProtected: bool;
        isProtected = (::match_deref::match_deref! { match node {
            Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::BASE_CLASS { definition: Deref @ SCode::Element::EXTENDS { visibility: SCode::Visibility::PROTECTED { .. }, .. }, .. }, .. } => true,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        isProtected
    }

    pub fn visibility(mut node: &metamodelica::Ref<InstNode>) -> Visibility {
        let mut vis: Visibility;
        vis = (match &**node {
            CLASS_NODE {
                visibility: __node_visibility,
                ..
            } => __node_visibility.clone(),
            COMPONENT_NODE {
                visibility: __node_visibility,
                ..
            } => __node_visibility.clone(),
            _ => Visibility::PUBLIC.clone(),
        });
        vis
    }

    pub fn isProtected(mut node: &metamodelica::Ref<InstNode>) -> bool {
        let mut isProtected: bool;
        isProtected = (match &**node {
            CLASS_NODE {
                visibility: Prefixes::Visibility::PROTECTED { .. },
                ..
            } => true,
            COMPONENT_NODE {
                visibility: Prefixes::Visibility::PROTECTED { .. },
                ..
            } => true,
            _ => false,
        });
        isProtected
    }

    pub(crate) fn isInheritedProtected(mut node: &metamodelica::Ref<InstNode>) -> Result<bool> {
        let mut isProtected: bool;
        isProtected = (match &**node {
            CLASS_NODE {
                visibility: __node_visibility,
                ..
            } => {
                __node_visibility.clone() == Visibility::PROTECTED.clone()
                    || isInheritedProtected(&(instanceParent(node.clone())?))?
            }
            COMPONENT_NODE {
                visibility: __node_visibility,
                ..
            } => {
                __node_visibility.clone() == Visibility::PROTECTED.clone()
                    || isInheritedProtected(&(instanceParent(node.clone())?))?
            }
            _ => false,
        });
        Ok(isProtected)
    }

    pub(crate) fn isPublic(mut node: &metamodelica::Ref<InstNode>) -> bool {
        let mut isPublic: bool = !(isProtected(node));
        isPublic
    }

    pub(crate) fn protectClass(mut cls: metamodelica::Ref<InstNode>) -> metamodelica::Ref<InstNode> {
        let mut cls: metamodelica::Ref<InstNode> = cls;
        let () = (match &*cls {
            CLASS_NODE {
                visibility: Prefixes::Visibility::PUBLIC { .. },
                ..
            } => {
                assign_variant_field!(cls => InstNode::CLASS_NODE; visibility = Visibility::PROTECTED.clone());
                ()
            }
            _ => (),
        });
        cls
    }

    pub(crate) fn protectComponent(mut comp: metamodelica::Ref<InstNode>) -> metamodelica::Ref<InstNode> {
        let mut comp: metamodelica::Ref<InstNode> = comp;
        let () = (match &*comp {
            COMPONENT_NODE {
                visibility: Prefixes::Visibility::PUBLIC { .. },
                ..
            } => {
                assign_variant_field!(comp => InstNode::COMPONENT_NODE; visibility = Visibility::PROTECTED.clone());
                ()
            }
            _ => (),
        });
        comp
    }

    pub fn protect(mut node: metamodelica::Ref<InstNode>) -> metamodelica::Ref<InstNode> {
        let mut node: metamodelica::Ref<InstNode> = node;
        let () = (match &*node {
            COMPONENT_NODE {
                visibility: Prefixes::Visibility::PUBLIC { .. },
                ..
            } => {
                assign_variant_field!(node => InstNode::COMPONENT_NODE; visibility = Visibility::PROTECTED.clone());
                ()
            }
            CLASS_NODE {
                visibility: Prefixes::Visibility::PUBLIC { .. },
                ..
            } => {
                assign_variant_field!(node => InstNode::CLASS_NODE; visibility = Visibility::PROTECTED.clone());
                ()
            }
            _ => (),
        });
        node
    }

    pub(crate) fn isEncapsulated(mut node: metamodelica::Ref<InstNode>) -> Result<bool> {
        let mut enc: bool;
        enc = (match &*node {
            CLASS_NODE { cls: __node_cls, .. } => Class::isEncapsulated(Pointer::access(__node_cls.clone()))?,
            COMPONENT_NODE { .. } => Class::isEncapsulated(getClass(node)?)?,
            _ => false,
        });
        Ok(enc)
    }

    pub(crate) fn getModifier(mut node: &metamodelica::Ref<InstNode>) -> metamodelica::Ref<Modifier::Modifier> {
        let mut r#mod: metamodelica::Ref<Modifier::Modifier>;
        r#mod = (match &**node {
            CLASS_NODE { cls: __node_cls, .. } => Class::getModifier(&(Pointer::access(__node_cls.clone()))),
            COMPONENT_NODE {
                component: __node_component,
                ..
            } => Component::getModifier(&(Pointer::access(__node_component.clone()))),
            _ => crate::NFModifier::Modifier::interned_NOMOD(),
        });
        r#mod
    }

    pub(crate) fn mergeModifier(
        mut r#mod: metamodelica::Ref<Modifier::Modifier>,
        mut node: metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode> = node;
        let () = (match &*node {
            CLASS_NODE { cls: __node_cls, .. } => {
                Pointer::update(
                    __node_cls.clone(),
                    Class::mergeModifier(r#mod, Pointer::access(__node_cls.clone()))?,
                );
                ()
            }
            COMPONENT_NODE {
                component: __node_component,
                ..
            } => {
                Pointer::update(
                    __node_component.clone(),
                    Component::mergeModifier(r#mod, Pointer::access(__node_component.clone()))?,
                );
                ()
            }
            _ => (),
        });
        Ok(node)
    }

    pub(crate) fn setModifier(
        mut r#mod: metamodelica::Ref<Modifier::Modifier>,
        mut node: metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode> = node;
        let () = (match &*node {
            CLASS_NODE { cls: __node_cls, .. } => {
                Pointer::update(
                    __node_cls.clone(),
                    Class::setModifier(r#mod, Pointer::access(__node_cls.clone()))?,
                );
                ()
            }
            COMPONENT_NODE {
                component: __node_component,
                ..
            } => {
                Pointer::update(
                    __node_component.clone(),
                    Component::mergeModifier(r#mod, Pointer::access(__node_component.clone()))?,
                );
                ()
            }
            _ => (),
        });
        Ok(node)
    }

    pub(crate) fn toPartialDAEType(mut clsNode: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<DAE::Type>> {
        let mut outType: metamodelica::Ref<DAE::Type>;
        outType = (match &*clsNode {
            CLASS_NODE { cls: __clsNode_cls, .. } => {
                let mut cls: metamodelica::Ref<Class::NFClass>;
                let mut state: ClassInf::State;
                let mut res: metamodelica::Ref<Restriction::NFRestriction>;
                cls = Pointer::access(__clsNode_cls.clone());
                (match &*cls {
                    Class::DAE_TYPE { ty: __cls_ty } => stripDAETypeVars(__cls_ty.clone()),
                    _ => {
                        (match &*(CachedData::getTypeCache(
                            var_field!((*clsNode).caches, InstNode::CLASS_NODE).clone(),
                        )?) {
                            CachedData::PARTIAL_DAE_TYPE { ty: cached } => cached.clone(),
                            _ => {
                                res = Class::restriction(&cls);
                                state = Restriction::toDAE(&res, fullPath(clsNode.clone(), false)?);
                                outType = metamodelica::Ref::new(DAE::Type::T_COMPLEX {
                                    complexClassType: state,
                                    varLst: metamodelica::nil(),
                                    equalityConstraint: None,
                                    usedExternally: Restriction::isExternalRecord(&res),
                                });
                                CachedData::setTypeCache(
                                    var_field!((*clsNode).caches, InstNode::CLASS_NODE).clone(),
                                    metamodelica::Ref::new(CachedData::CachedData::PARTIAL_DAE_TYPE {
                                        ty: outType.clone(),
                                    }),
                                )?;
                                outType
                            }
                        })
                    }
                })
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(outType)
    }

    pub(crate) fn stripDAETypeVars(mut ty: metamodelica::Ref<DAE::Type>) -> metamodelica::Ref<DAE::Type> {
        let mut ty: metamodelica::Ref<DAE::Type> = ty;
        let () = (match &*ty {
            DAE::Type::T_COMPLEX { .. } => {
                assign_variant_field!(ty => DAE::Type::T_COMPLEX; varLst = metamodelica::nil());
                ()
            }
            _ => (),
        });
        ty
    }

    pub(crate) fn toFullDAEType(mut clsNode: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<DAE::Type>> {
        let mut outType: metamodelica::Ref<DAE::Type>;
        outType = (match &*clsNode.clone() {
            CLASS_NODE { cls: __clsNode_cls, .. } => {
                let mut cls: metamodelica::Ref<Class::NFClass>;
                let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                let mut state: ClassInf::State;
                let mut res: metamodelica::Ref<Restriction::NFRestriction>;
                cls = Pointer::access(__clsNode_cls.clone());
                (match &*cls {
                    Class::DAE_TYPE { ty: __cls_ty } => __cls_ty.clone(),
                    _ => {
                        res = Class::restriction(&cls);
                        state = Restriction::toDAE(&res, fullPath(clsNode.clone(), false)?);
                        vars = ConvertDAE::makeTypeVars(clsNode)?;
                        outType = metamodelica::Ref::new(DAE::Type::T_COMPLEX {
                            complexClassType: state,
                            varLst: vars,
                            equalityConstraint: None,
                            usedExternally: Restriction::isExternalRecord(&res),
                        });
                        Pointer::update(
                            __clsNode_cls.clone(),
                            metamodelica::Ref::new(Class::NFClass::DAE_TYPE { ty: outType.clone() }),
                        );
                        outType
                    }
                })
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(outType)
    }

    pub(crate) fn isBuiltin(mut node: &metamodelica::Ref<InstNode>) -> bool {
        let mut isBuiltin: bool;
        isBuiltin = (match &**node {
            CLASS_NODE {
                nodeType: __node_nodeType,
                ..
            } => isBuiltinNodeType(metamodelica::AsArg::as_arg(&__node_nodeType)),
            _ => false,
        });
        isBuiltin
    }

    pub(crate) fn isBuiltinNodeType<'__b>(mut nodeType: &'__b metamodelica::Ref<InstNodeType>) -> bool {
        '__tco: loop {
            match &**nodeType {
                InstNodeType::BUILTIN_CLASS => return true,
                InstNodeType::BASE_CLASS { .. } => {
                    nodeType = var_field!((**nodeType).ty, InstNodeType::BASE_CLASS);
                    continue '__tco;
                }
                _ => return false,
            }
        }
    }

    pub fn isPartial(mut node: &metamodelica::Ref<InstNode>) -> Result<bool> {
        let mut isPartial: bool;
        isPartial = (match &**node {
            CLASS_NODE { cls: __node_cls, .. } => Class::isPartial(Pointer::access(__node_cls.clone()))?,
            _ => false,
        });
        Ok(isPartial)
    }

    pub(crate) fn clone(mut node: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<InstNode>> {
        let mut node: metamodelica::Ref<InstNode> = node;
        let () = (match &*node {
            CLASS_NODE { cls: __node_cls, .. } => {
                let mut cls: metamodelica::Ref<Class::NFClass>;
                cls = Pointer::access(__node_cls.clone());
                cls = Class::classTreeApply(cls, &ClassTree::clone)?;
                assign_variant_field!(node => InstNode::CLASS_NODE;
                    cls = Pointer::create(cls),
                    caches = CachedData::empty()
                );
                ()
            }
            COMPONENT_NODE {
                component: __node_component,
                ..
            } => {
                let mut comp: metamodelica::Ref<Component::NFComponent>;
                comp = Pointer::access(__node_component.clone());
                comp = Component::setClassInstance(clone(Component::classInstance(&comp)?)?, comp)?;
                assign_variant_field!(node => InstNode::COMPONENT_NODE; component = Pointer::create(comp));
                ()
            }
            _ => (),
        });
        node = reidentify(node);
        Ok(node)
    }

    pub(crate) fn cloneComponent(
        mut component: &metamodelica::Ref<InstNode>,
        mut newParent: metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut outComponent: metamodelica::Ref<InstNode> =
            cloneComponentInScope(component, identityCell(newParent.clone()))?;
        Ok(outComponent)
    }

    pub(crate) fn cloneComponentInScope(
        mut component: &metamodelica::Ref<InstNode>,
        mut parent: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut outComponent: metamodelica::Ref<InstNode>;
        outComponent = (match &**component {
            COMPONENT_NODE {
                component: __component_component,
                definition: __component_definition,
                name: __component_name,
                nodeType: __component_nodeType,
                visibility: __component_visibility,
                ..
            } => {
                let mut owner: Option<Mutable::Mutable<metamodelica::Ref<InstNode>>>;
                let mut identity: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>;
                (owner, identity) = newIdentity();
                metamodelica::Ref::new(InstNode::COMPONENT_NODE {
                    name: __component_name.clone(),
                    definition: __component_definition.clone(),
                    visibility: __component_visibility.clone(),
                    component: Pointer::create(Pointer::access(__component_component.clone())),
                    owner: owner,
                    identity: identity,
                    parent: parent,
                    nodeType: __component_nodeType.clone(),
                })
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(outComponent)
    }

    pub(crate) fn getComments(
        mut node: &metamodelica::Ref<InstNode>,
        mut accumCmts: metamodelica::List<metamodelica::Ref<SCode::Comment>>,
    ) -> Result<metamodelica::List<metamodelica::Ref<SCode::Comment>>> {
        let mut cmts: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
        cmts = (::match_deref::match_deref! { match node {
            Deref @ CLASS_NODE { definition: Deref @ SCode::Element::CLASS { cmt, .. }, cls: __node_cls, .. } => {
                metamodelica::cons(cmt.clone(), Class::getDerivedComments(Pointer::access(__node_cls.clone()), accumCmts)?)
            },
            _ => {
                accumCmts
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(cmts)
    }

    pub(crate) fn copyInstancePtr(
        mut srcNode: &metamodelica::Ref<InstNode>,
        mut dstNode: metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<InstNode>> {
        let mut dstNode: metamodelica::Ref<InstNode> = dstNode;
        let () = (::match_deref::match_deref! { match &((srcNode.clone(), dstNode.clone())) {
            (Deref @ COMPONENT_NODE { .. }, Deref @ COMPONENT_NODE { .. }) => {
                assign_variant_field!(dstNode => InstNode::COMPONENT_NODE; component = var_field!((**srcNode).component, InstNode::COMPONENT_NODE).clone());
                ()
            },
            (Deref @ CLASS_NODE { .. }, Deref @ CLASS_NODE { .. }) => {
                assign_variant_field!(dstNode => InstNode::CLASS_NODE; cls = var_field!((**srcNode).cls, InstNode::CLASS_NODE).clone());
                ()
            },
            _ => return Err("match: no arm matched"),
        } });
        Ok(dstNode)
    }

    pub(crate) fn isRecord(mut node: metamodelica::Ref<InstNode>) -> Result<bool> {
        '__tco: loop {
            match &*node {
                CLASS_NODE { cls: __node_cls, .. } => {
                    return Ok(Restriction::isRecord(
                        &(Class::restriction(&(Pointer::access(__node_cls.clone())))),
                    ));
                }
                COMPONENT_NODE {
                    component: __node_component,
                    ..
                } => {
                    node = Component::classInstance(&(Pointer::access(__node_component.clone())))?;
                    continue '__tco;
                }
                _ => return Ok(false),
            }
        }
    }

    pub(crate) fn isModel(mut node: metamodelica::Ref<InstNode>) -> Result<bool> {
        '__tco: loop {
            match &*node {
                CLASS_NODE { cls: __node_cls, .. } => {
                    return Ok(Restriction::isModel(
                        &(Class::restriction(&(Pointer::access(__node_cls.clone())))),
                    ));
                }
                COMPONENT_NODE {
                    component: __node_component,
                    ..
                } => {
                    node = Component::classInstance(&(Pointer::access(__node_component.clone())))?;
                    continue '__tco;
                }
                _ => return Ok(false),
            }
        }
    }

    pub fn isEnumerationType(mut node: metamodelica::Ref<InstNode>) -> Result<bool> {
        let mut isEnum: bool = isClass(&node)? && Class::isEnumeration(getClass(resolveInner(node.clone()))?)?;
        Ok(isEnum)
    }

    pub(crate) fn hasBinding(mut node: metamodelica::Ref<InstNode>) -> Result<bool> {
        let mut hasBinding: bool;
        hasBinding = (match &*node.clone() {
            COMPONENT_NODE {
                component: __node_component,
                ..
            } => {
                Component::hasBinding(
                    &(Pointer::access(__node_component.clone())),
                    &(crate::NFInstNode::InstNode::interned_EMPTY_NODE()),
                )? || self::hasBinding(instanceParent(node)?)?
            }
            _ => false,
        });
        Ok(hasBinding)
    }

    pub fn getBindingExpOpt(
        mut node: &metamodelica::Ref<InstNode>,
    ) -> Result<Option<metamodelica::Ref<Expression::NFExpression>>> {
        let mut binding_exp: Option<metamodelica::Ref<Expression::NFExpression>>;
        binding_exp = (match &**node {
            COMPONENT_NODE {
                component: __node_component,
                ..
            } => {
                let mut scope: metamodelica::Ref<InstNode>;
                scope = instanceParent(node.clone())?;
                match '__try0: {
                    binding_exp = Binding::getExpOpt(
                        &(unwrap_break_err!(Component::getImplicitBinding(&(Pointer::access(__node_component.clone())), scope.clone()), '__try0)),
                    );
                    Ok::<_, &'static str>((binding_exp.clone(),))
                } {
                    Ok((__try0_o0,)) => {
                        binding_exp = __try0_o0;
                    }
                    Err(_) => {
                        binding_exp = getBindingExpOpt(&scope)?;
                    }
                }
                binding_exp
            }
            VAR_NODE {
                varPointer: __node_varPointer,
                ..
            } => {
                let mut var: metamodelica::Ref<Variable::NFVariable>;
                var = Pointer::access(PointerWeak::upgrade(__node_varPointer.clone())?);
                Binding::getExpOpt(&var.binding)
            }
            _ => None,
        });
        Ok(binding_exp)
    }

    pub(crate) fn getSections(
        mut node: metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<Sections::NFSections>> {
        '__tco: loop {
            let mut cls: metamodelica::Ref<Class::NFClass> = getClass(node.clone())?;
            match &*cls {
                Class::INSTANCED_CLASS {
                    sections: __cls_sections,
                    ..
                } => return Ok(__cls_sections.clone()),
                Class::TYPED_DERIVED {
                    baseClass: __cls_baseClass,
                    ..
                } => {
                    node = __cls_baseClass.clone();
                    continue '__tco;
                }
                _ => {
                    Error::terminate(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NFInstNode.InstNode.getSections"));
                            __mm_s.push_str(&*literal!(" did not get an instanced class"));
                            ArcStr::from(__mm_s)
                        },
                        &(metamodelica::sourceInfo!("NFFrontEnd/NFInstNode.mo")),
                    )?;
                    return Ok(return Err("fail"));
                }
            }
        }
    }

    pub fn hash(mut node: &metamodelica::Ref<InstNode>) -> Result<i32> {
        let mut hash: i32 = stringHashDjb2(&(name(node)?));
        Ok(hash)
    }

    pub(crate) fn hashContinue(mut node: &metamodelica::Ref<InstNode>, mut hash: i32) -> Result<i32> {
        let mut hash: i32 = hash;
        hash = stringHashDjb2Continue(&(name(node)?), hash);
        Ok(hash)
    }

    pub(crate) fn dimensionCount(mut node: &metamodelica::Ref<InstNode>) -> i32 {
        let mut count: i32;
        count = (match &**node {
            COMPONENT_NODE {
                component: __node_component,
                ..
            } => Component::dimensionCount(&(Pointer::access(__node_component.clone()))),
            CLASS_NODE { cls: __node_cls, .. } => Class::dimensionCount(&(Pointer::access(__node_cls.clone()))),
            _ => 0,
        });
        count
    }

    pub(crate) fn isClockType(mut node: &metamodelica::Ref<InstNode>) -> bool {
        let mut clock: bool;
        clock = (::match_deref::match_deref! { match node {
            Deref @ CLASS_NODE { name: Deref @ "Clock", nodeType: Deref @ InstNodeType::BUILTIN_CLASS, .. } => true,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        clock
    }

    pub fn restriction(mut node: metamodelica::Ref<InstNode>) -> Result<metamodelica::Ref<Restriction::NFRestriction>> {
        '__tco: loop {
            match &*node {
                CLASS_NODE { cls: __node_cls, .. } => {
                    return Ok(Class::restriction(&(Pointer::access(__node_cls.clone()))));
                }
                COMPONENT_NODE {
                    component: __node_component,
                    ..
                } => {
                    node = Component::classInstance(&(Pointer::access(__node_component.clone())))?;
                    continue '__tco;
                }
                INNER_OUTER_NODE {
                    innerNode: __node_innerNode,
                    ..
                } => {
                    node = __node_innerNode.clone();
                    continue '__tco;
                }
                _ => return Ok(crate::NFRestriction::interned_UNKNOWN()),
            }
        }
    }

    pub(crate) fn isExtends(mut node: &metamodelica::Ref<InstNode>) -> bool {
        let mut res: bool;
        res = (::match_deref::match_deref! { match node {
            Deref @ CLASS_NODE { definition: Deref @ SCode::Element::EXTENDS { .. }, .. } => true,
            Deref @ CLASS_NODE { nodeType: Deref @ InstNodeType::BASE_CLASS { definition: Deref @ SCode::Element::EXTENDS { .. }, .. }, .. } => true,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        res
    }

    pub(crate) fn isDiscreteClass(mut clsNode: metamodelica::Ref<InstNode>) -> Result<bool> {
        '__tco: loop {
            let mut base_node: metamodelica::Ref<InstNode>;
            let mut cls: metamodelica::Ref<Class::NFClass>;
            let mut exts: metamodelica::Array<metamodelica::Ref<InstNode>>;
            base_node = Class::lastBaseClass(clsNode)?;
            cls = getClass(base_node.clone())?;
            ::match_deref::match_deref! { match &(cls.clone()) {
                Deref @ Class::EXPANDED_CLASS { restriction: Deref @ Restriction::TYPE, elements: __cls_elements, .. } => {
                    exts = ClassTree::getExtends(metamodelica::AsArg::as_arg(&__cls_elements));
                    if (metamodelica::arrayLength(exts.clone()) == 1) {{ clsNode = ({let __elt = (*metamodelica::index_checked(&exts.borrow(), 1)?).clone(); __elt}); continue '__tco; }} else {return Ok(false)}
                },
                _ => return Ok(Type::isDiscrete(Class::getType(cls, base_node)?)?),
                _ => return Err("match: no arm matched"),
            } }
        }
    }

    pub fn clearGeneratedInners(mut node: metamodelica::Ref<InstNode>) -> Result<()> {
        let mut inners: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::Ref<InstNode>>>;
        let __pa0 = ::match_deref::match_deref! { match &(nodeType(&(topScope(node)?))?) {
            Deref @ InstNodeType::TOP_SCOPE { generatedInners: __pa0, .. } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        inners = metamodelica::Own::own(__pa0);
        UnorderedMap::clear(inners);
        Ok(())
    }

    pub fn scopeRoots(mut node: metamodelica::Ref<InstNode>) -> Result<MutableWeak::Roots> {
        let mut roots: MutableWeak::Roots;
        let __pa0 = ::match_deref::match_deref! { match &(nodeType(&(topScope(node)?))?) {
            Deref @ InstNodeType::TOP_SCOPE { roots: __pa0, .. } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        roots = metamodelica::Own::own(__pa0);
        Ok(roots)
    }

    pub(crate) fn getAccessLevel(mut node: metamodelica::Ref<InstNode>) -> Result<Option<AccessLevel>> {
        let mut access: Option<AccessLevel> = None;
        let mut scope: metamodelica::Ref<InstNode>;
        let mut access_mod: metamodelica::Ref<SCode::Mod>;
        let mut access_exp: Option<metamodelica::Ref<Absyn::Exp>>;
        scope = classScope(parent(&(resolveInner(node)))?)?;
        while isClass(&scope)? {
            access_mod = SCodeUtil::lookupElementAnnotation(&(definition(scope.clone())?), &(literal!("Protection")))?;
            access_mod = SCodeUtil::lookupModInMod(&(literal!("access")), &access_mod);
            access_exp = SCodeUtil::getModifierBinding(&access_mod);
            if (access_exp).is_some() {
                access = Prefixes::accessLevelFromAbsyn(&(Util::getOption(access_exp)?));
                if (access).is_some() {
                    return Ok(access);
                }
            }
            scope = parent(&scope)?;
        }
        Ok(access)
    }

    pub(crate) fn upgradeCell(
        mut weak: MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>,
        mut owning: bool,
        mut name: &ArcStr,
    ) -> Result<Mutable::Mutable<metamodelica::Ref<InstNode>>> {
        let mut cell: Mutable::Mutable<metamodelica::Ref<InstNode>>;
        match '__try0: {
            if owning {
                cell = unwrap_break_err!(MutableWeak::upgradeOwning(weak.clone()), '__try0);
            } else {
                cell = unwrap_break_err!(MutableWeak::upgrade(weak.clone()), '__try0);
            }
            Ok::<_, &'static str>((cell.clone(),))
        } {
            Ok((__try0_o0,)) => {
                cell = __try0_o0;
            }
            Err(__try0_err) => {
                Error::addInternalError(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("weakly held InstNode "));
                        __mm_s.push_str(&*if (stringEmpty(&name)) {
                            literal!("scope")
                        } else {
                            {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("'"));
                                __mm_s.push_str(&*name);
                                __mm_s.push_str(&*literal!("'"));
                                ArcStr::from(__mm_s)
                            }
                        });
                        __mm_s.push_str(&*literal!(" was collected before the reference to it"));
                        ArcStr::from(__mm_s)
                    },
                    metamodelica::sourceInfo!("NFFrontEnd/NFInstNode.mo"),
                )?;
                return Err(__try0_err);
            }
        }
        Ok(cell)
    }

    pub(crate) fn newIdentity() -> (
        Option<Mutable::Mutable<metamodelica::Ref<InstNode>>>,
        Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>,
    ) {
        let mut owner: Option<Mutable::Mutable<metamodelica::Ref<InstNode>>>;
        let mut identity: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>;
        let mut cell: Mutable::Mutable<metamodelica::Ref<InstNode>>;
        cell = Mutable::create(crate::NFInstNode::InstNode::interned_EMPTY_NODE());
        MutableWeak::root(cell.clone());
        owner = Some(cell.clone());
        identity = Some(MutableWeak::downgrade(cell));
        (owner, identity)
    }

    pub(crate) fn setOwner(
        mut node: metamodelica::Ref<InstNode>,
        mut owner: Option<Mutable::Mutable<metamodelica::Ref<InstNode>>>,
    ) -> metamodelica::Ref<InstNode> {
        let mut node: metamodelica::Ref<InstNode> = node;
        let () = (match &*node {
            CLASS_NODE { .. } => {
                assign_variant_field!(node => InstNode::CLASS_NODE; owner = owner);
                ()
            }
            COMPONENT_NODE { .. } => {
                assign_variant_field!(node => InstNode::COMPONENT_NODE; owner = owner);
                ()
            }
            _ => (),
        });
        node
    }

    pub(crate) fn disown(mut node: metamodelica::Ref<InstNode>) -> metamodelica::Ref<InstNode> {
        let mut node: metamodelica::Ref<InstNode> = node;
        node = setOwner(node, None);
        node
    }

    pub(crate) fn reown(
        mut node: metamodelica::Ref<InstNode>,
        mut cell: Mutable::Mutable<metamodelica::Ref<InstNode>>,
    ) -> metamodelica::Ref<InstNode> {
        let mut outNode: metamodelica::Ref<InstNode>;
        outNode = setOwner(node, Some(cell));
        outNode
    }

    pub(crate) fn fromIdentity(
        mut identity: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode>>>,
        mut node: metamodelica::Ref<InstNode>,
    ) -> Result<metamodelica::Ref<NodeHandle>> {
        let mut hnd: metamodelica::Ref<NodeHandle>;
        hnd = (match identity {
            Some(mut w) => metamodelica::Ref::new(NodeHandle::CELL {
                cell: w,
                name: name(&node)?,
            }),
            _ => metamodelica::Ref::new(NodeHandle::VALUE { node: node }),
        });
        Ok(hnd)
    }
}
