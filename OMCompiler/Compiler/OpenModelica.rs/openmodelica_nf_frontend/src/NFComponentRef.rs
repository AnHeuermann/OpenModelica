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
use crate::NFBackendExtension::Annotations;
use crate::NFBackendExtension::BackendInfo;
use crate::NFBinding as Binding;
use crate::NFCeval;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComponent as Component;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFInstNode::InstNodeType;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes::Visibility;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::JSON;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::MutableWeak;
use openmodelica_util_datatypes_basic::Pointer;
use openmodelica_util_datatypes_basic::PointerWeak;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NFComponentRef {
    CREF {
        /// Weakly: the class tree owns the node.
        node: metamodelica::Ref<NFInstNode::NodeHandle>,
        subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
        ty: metamodelica::Ref<Type::NFType>,
        origin: Origin,
        restCref: metamodelica::Ref<NFComponentRef>,
    },
    EMPTY,
    WILD,
}
impl metamodelica::gc::MMTrace for NFComponentRef {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFComponentRef::CREF {
                node,
                subscripts,
                ty,
                origin,
                restCref,
            } => {
                metamodelica::gc::MMTrace::mm_accept(node, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(subscripts, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(origin, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(restCref, __mmv)?;
                Ok(())
            }
            NFComponentRef::EMPTY => Ok(()),
            NFComponentRef::WILD => Ok(()),
        }
    }
}
impl NFComponentRef {
    pub fn interned_EMPTY() -> metamodelica::Ref<NFComponentRef> {
        thread_local! {
            static INTERNED: metamodelica::Ref<NFComponentRef> = metamodelica::Ref::new(NFComponentRef::EMPTY);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_WILD() -> metamodelica::Ref<NFComponentRef> {
        thread_local! {
            static INTERNED: metamodelica::Ref<NFComponentRef> = metamodelica::Ref::new(NFComponentRef::WILD);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_EMPTY() -> metamodelica::Ref<NFComponentRef> {
    NFComponentRef::interned_EMPTY()
}
pub fn interned_WILD() -> metamodelica::Ref<NFComponentRef> {
    NFComponentRef::interned_WILD()
}
impl Default for NFComponentRef {
    fn default() -> Self {
        Self::EMPTY
    }
}
pub use self::NFComponentRef::{CREF, EMPTY, WILD};
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum Origin {
    /// From an Absyn cref.
    CREF = 1,
    /// From prefixing the cref with its scope.
    SCOPE = 2,
    /// From an iterator.
    ITERATOR = 3,
}
impl PartialOrd for Origin {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Origin {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for Origin {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub fn fromNode(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut origin: Origin,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = metamodelica::Ref::new(NFComponentRef::CREF {
        node: NFInstNode::InstNode::handle(node.clone())?,
        subscripts: subs.clone(),
        ty: ty.clone(),
        origin: origin,
        restCref: crate::NFComponentRef::interned_EMPTY(),
    });
    Ok(cref)
}

pub fn fromOwnedNode(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> metamodelica::Ref<NFComponentRef> {
    let mut cref: metamodelica::Ref<NFComponentRef> = metamodelica::Ref::new(NFComponentRef::CREF {
        node: metamodelica::Ref::new(NFInstNode::NodeHandle::VALUE { node: node.clone() }),
        subscripts: metamodelica::nil(),
        ty: ty.clone(),
        origin: Origin::CREF.clone(),
        restCref: crate::NFComponentRef::interned_EMPTY(),
    });
    cref
}

pub fn storeNode(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut update: bool,
) -> Result<metamodelica::Ref<NFInstNode::NodeHandle>> {
    let mut stored: metamodelica::Ref<NFInstNode::NodeHandle> = if (update) {
        NFInstNode::InstNode::republish(node.clone())?
    } else {
        NFInstNode::InstNode::handle(node.clone())?
    };
    Ok(stored)
}

pub fn prefixCref(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut restCref: metamodelica::Ref<NFComponentRef>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = metamodelica::Ref::new(NFComponentRef::CREF {
        node: NFInstNode::InstNode::handle(node.clone())?,
        subscripts: subs.clone(),
        ty: ty.clone(),
        origin: Origin::CREF.clone(),
        restCref: restCref.clone(),
    });
    Ok(cref)
}

pub(crate) fn prefixScope(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut restCref: metamodelica::Ref<NFComponentRef>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = metamodelica::Ref::new(NFComponentRef::CREF {
        node: NFInstNode::InstNode::handle(node.clone())?,
        subscripts: subs.clone(),
        ty: ty.clone(),
        origin: Origin::SCOPE.clone(),
        restCref: restCref.clone(),
    });
    Ok(cref)
}

pub(crate) fn fromAbsyn(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut subs: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut restCref: metamodelica::Ref<NFComponentRef>,
    mut isIterator: bool,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef>;
    let mut sl: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    sl = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
        for mut s in (subs).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Subscript::NFSubscript::RAW_SUBSCRIPT { subscript: s.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    cref = metamodelica::Ref::new(NFComponentRef::CREF {
        node: NFInstNode::InstNode::handle(node)?,
        subscripts: sl,
        ty: crate::NFType::interned_UNKNOWN(),
        origin: if (isIterator) {
            Origin::ITERATOR.clone()
        } else {
            Origin::CREF.clone()
        },
        restCref: restCref,
    });
    Ok(cref)
}

pub(crate) fn fromAbsynCref<'__b>(
    mut acref: &'__b metamodelica::Ref<Absyn::ComponentRef>,
    mut restCref: metamodelica::Ref<NFComponentRef>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    '__tco: loop {
        match &**acref {
            Absyn::ComponentRef::CREF_IDENT { .. } => {
                return Ok(fromAbsyn(
                    metamodelica::Ref::new(InstNode::InstNode::NAME_NODE {
                        name: var_field!((**acref).name, Absyn::ComponentRef::CREF_IDENT).clone(),
                    }),
                    var_field!((**acref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone(),
                    restCref,
                    false,
                )?);
            }
            Absyn::ComponentRef::CREF_QUAL { .. } => {
                (acref, restCref) = (
                    var_field!((**acref).componentRef, Absyn::ComponentRef::CREF_QUAL),
                    fromAbsyn(
                        metamodelica::Ref::new(InstNode::InstNode::NAME_NODE {
                            name: var_field!((**acref).name, Absyn::ComponentRef::CREF_QUAL).clone(),
                        }),
                        var_field!((**acref).subscripts, Absyn::ComponentRef::CREF_QUAL).clone(),
                        restCref,
                        false,
                    )?,
                );
                continue '__tco;
            }
            Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => {
                (acref, restCref) = (
                    var_field!((**acref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED),
                    crate::NFComponentRef::interned_EMPTY(),
                );
                continue '__tco;
            }
            Absyn::ComponentRef::WILD { .. } => return Ok(crate::NFComponentRef::interned_WILD()),
            Absyn::ComponentRef::ALLWILD { .. } => return Ok(crate::NFComponentRef::interned_WILD()),
        }
    }
}

pub(crate) fn fromBuiltin(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = metamodelica::Ref::new(NFComponentRef::CREF {
        node: NFInstNode::InstNode::handle(node.clone())?,
        subscripts: metamodelica::nil(),
        ty: ty.clone(),
        origin: Origin::SCOPE.clone(),
        restCref: crate::NFComponentRef::interned_EMPTY(),
    });
    Ok(cref)
}

pub fn makeIterator(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = metamodelica::Ref::new(NFComponentRef::CREF {
        node: NFInstNode::InstNode::handle(node.clone())?,
        subscripts: metamodelica::nil(),
        ty: ty.clone(),
        origin: Origin::ITERATOR.clone(),
        restCref: crate::NFComponentRef::interned_EMPTY(),
    });
    Ok(cref)
}

pub fn isWild(mut cref: &metamodelica::Ref<NFComponentRef>) -> bool {
    let mut isWild: bool;
    isWild = (match &**cref {
        WILD { .. } => true,
        _ => false,
    });
    isWild
}

pub fn isEmpty(mut cref: &metamodelica::Ref<NFComponentRef>) -> bool {
    let mut isEmpty: bool;
    isEmpty = (match &**cref {
        EMPTY { .. } => true,
        _ => false,
    });
    isEmpty
}

pub(crate) fn isSimple(mut cref: &metamodelica::Ref<NFComponentRef>) -> bool {
    let mut isSimple: bool;
    isSimple = (::match_deref::match_deref! { match cref {
        Deref @ CREF { restCref: Deref @ EMPTY { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isSimple
}

pub(crate) fn isQualified(mut cref: &metamodelica::Ref<NFComponentRef>) -> bool {
    let mut qualified: bool;
    qualified = (::match_deref::match_deref! { match cref {
        Deref @ CREF { restCref: Deref @ CREF { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    qualified
}

pub fn isTopLevel(mut cref: &metamodelica::Ref<NFComponentRef>) -> bool {
    fn isTopLevelRecord(mut cref: &metamodelica::Ref<NFComponentRef>) -> bool {
        let mut b: bool;
        b = (match &**cref {
            CREF {
                restCref: __cref_restCref,
                ty: __cref_ty,
                ..
            } => {
                Type::isRecord(metamodelica::AsArg::as_arg(&__cref_ty))
                    && isTopLevelRecord(metamodelica::AsArg::as_arg(&__cref_restCref))
            }
            EMPTY { .. } => true,
            _ => false,
        });
        b
    }

    let mut b: bool;
    b = (::match_deref::match_deref! { match cref {
        Deref @ CREF { restCref: Deref @ EMPTY { .. }, .. } => true,
        Deref @ CREF { restCref: __cref_restCref, .. } => isTopLevelRecord(metamodelica::AsArg::as_arg(&__cref_restCref)),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isFlow(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<bool> {
    let mut isFlow: bool;
    let mut comp: metamodelica::Ref<Component::NFComponent>;
    isFlow = (match &**cref {
        CREF { .. } if (NFInstNode::InstNode::isComponent(&(node(cref)?))?) => {
            Component::isFlow(&(NFInstNode::InstNode::component(&(NFInstNode::InstNode::resolveInner(node(cref)?)))?))
        }
        _ => false,
    });
    Ok(isFlow)
}

pub fn isCref(mut cref: &metamodelica::Ref<NFComponentRef>) -> bool {
    let mut isCref: bool;
    isCref = (match &**cref {
        CREF { .. } => true,
        _ => false,
    });
    isCref
}

pub fn isIterator(mut cref: &metamodelica::Ref<NFComponentRef>) -> bool {
    let mut isIterator: bool;
    isIterator = (match &**cref {
        CREF {
            origin: Origin::ITERATOR { .. },
            ..
        } => true,
        _ => false,
    });
    isIterator
}

pub(crate) fn isInput(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<bool> {
    let mut res: bool;
    res = (match &**cref {
        CREF { .. } => NFInstNode::InstNode::isInput(&(node(cref)?)),
        _ => false,
    });
    Ok(res)
}

pub(crate) fn isOutput(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<bool> {
    let mut res: bool;
    res = (match &**cref {
        CREF { .. } => NFInstNode::InstNode::isOutput(&(node(cref)?)),
        _ => false,
    });
    Ok(res)
}

pub fn isNameNode(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<bool> {
    let mut res: bool;
    res = (match &**cref {
        CREF { .. } => NFInstNode::InstNode::isName(&(node(cref)?)),
        _ => false,
    });
    Ok(res)
}

pub fn isEqualRecordChild(
    mut child: &metamodelica::Ref<NFComponentRef>,
    mut recd: &metamodelica::Ref<NFComponentRef>,
) -> Result<bool> {
    let mut b: bool = size(child, true, false)? == size(recd, true, false)?;
    if b {
        b = isRecordChild(child, recd)?;
    }
    Ok(b)
}

pub(crate) fn isRecordChild(
    mut child: &metamodelica::Ref<NFComponentRef>,
    mut recd: &metamodelica::Ref<NFComponentRef>,
) -> Result<bool> {
    let mut b: bool;
    b = (match &**recd {
        CREF {
            restCref: __recd_restCref,
            ..
        } => isEqual(child, recd)? || isRecordChild(child, metamodelica::AsArg::as_arg(&__recd_restCref))?,
        _ => false,
    });
    Ok(b)
}

pub fn node(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut hnd: metamodelica::Ref<NFInstNode::NodeHandle>;
    let __pa0 = ::match_deref::match_deref! { match &((*cref)) {
        Deref @ CREF { node: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    hnd = metamodelica::Own::own(__pa0);
    node = NFInstNode::InstNode::fromHandle(&hnd)?;
    Ok(node)
}

pub(crate) fn nodeName(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<ArcStr> {
    let mut name: ArcStr;
    let mut hnd: metamodelica::Ref<NFInstNode::NodeHandle>;
    let __pa0 = ::match_deref::match_deref! { match &((*cref)) {
        Deref @ CREF { node: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    hnd = metamodelica::Own::own(__pa0);
    name = NFInstNode::InstNode::handleName(&hnd)?;
    Ok(name)
}

pub(crate) fn nodes<'__b>(
    mut cref: &'__b metamodelica::Ref<NFComponentRef>,
    mut accum: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
) -> Result<metamodelica::List<metamodelica::Ref<InstNode::InstNode>>> {
    '__tco: loop {
        match &**cref {
            CREF { .. } => {
                (cref, accum) = (
                    var_field!((**cref).restCref, NFComponentRef::CREF),
                    metamodelica::cons(node(cref)?, accum),
                );
                continue '__tco;
            }
            _ => return Ok(accum),
        }
    }
}

pub(crate) fn nodesIncludingSplitSubs<'__b>(
    mut cref: &'__b metamodelica::Ref<NFComponentRef>,
    mut accum: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
) -> Result<metamodelica::List<metamodelica::Ref<InstNode::InstNode>>> {
    let mut nodes: metamodelica::List<metamodelica::Ref<InstNode::InstNode>> = accum;
    let mut node: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    nodes = (match &**cref {
        CREF { .. } => {
            for mut s in &*var_field!((**cref).subscripts, NFComponentRef::CREF).clone() {
                if Subscript::isSplitIndex(metamodelica::AsArg::as_arg(&s)) {
                    let __pa0 = ::match_deref::match_deref! { match &(s.clone()) {
                        Deref @ Subscript::SPLIT_INDEX { node: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    node = metamodelica::Own::own(__pa0);
                    nodes = metamodelica::cons(NFInstNode::InstNode::borrow(node)?, nodes);
                }
            }
            nodesIncludingSplitSubs(
                var_field!((**cref).restCref, NFComponentRef::CREF),
                metamodelica::cons(self::node(cref)?, nodes),
            )?
        }
        _ => nodes,
    });
    Ok(nodes)
}

pub(crate) fn containsNode(
    mut cref: &metamodelica::Ref<NFComponentRef>,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<bool> {
    let mut res: bool;
    res = (match &**cref {
        CREF {
            restCref: __cref_restCref,
            ..
        } => {
            NFInstNode::InstNode::refEqual(&(self::node(cref)?), node)?
                || containsNode(metamodelica::AsArg::as_arg(&__cref_restCref), node)?
        }
        _ => false,
    });
    Ok(res)
}

pub fn nodeType(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<metamodelica::Ref<Type::NFType>> {
    let mut ty: metamodelica::Ref<Type::NFType>;
    let __pa0 = ::match_deref::match_deref! { match &((*cref)) {
        Deref @ CREF { ty: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    Ok(ty)
}

pub(crate) fn setNodeType(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut cref: metamodelica::Ref<NFComponentRef>,
) -> metamodelica::Ref<NFComponentRef> {
    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    let () = (match &*cref {
        CREF { .. } => {
            assign_variant_field!(cref => NFComponentRef::CREF; ty = ty);
            ()
        }
        _ => (),
    });
    cref
}

pub(crate) fn updateNodeType(mut cref: metamodelica::Ref<NFComponentRef>) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    let () = (match &*cref {
        CREF { .. } if (NFInstNode::InstNode::isComponent(&(node(&cref)?))?) => {
            assign_variant_field!(cref => NFComponentRef::CREF; ty = NFInstNode::InstNode::getType(node(&cref)?)?);
            ()
        }
        _ => (),
    });
    Ok(cref)
}

pub fn scalarType(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<metamodelica::Ref<Type::NFType>> {
    let mut ty: metamodelica::Ref<Type::NFType>;
    let __pa0 = ::match_deref::match_deref! { match &((*cref)) {
        Deref @ CREF { ty: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    ty = Type::arrayElementType(&ty);
    Ok(ty)
}

pub(crate) fn applyToType(
    mut cref: metamodelica::Ref<NFComponentRef>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    pub type typeFunc = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>> + 'static,
    >;

    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    cref = (match &*cref {
        CREF { ty: __cref_ty, .. } => {
            assign_variant_field!(cref => NFComponentRef::CREF;
                ty = func(__cref_ty.clone())?,
                restCref = applyToType(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), func)?
            );
            cref
        }
        _ => cref,
    });
    Ok(cref)
}

pub fn firstName(mut cref: &metamodelica::Ref<NFComponentRef>, mut baseModelica: bool) -> Result<ArcStr> {
    let mut name: ArcStr;
    name = (match &**cref {
        CREF { .. } => nodeName(cref)?,
        WILD { .. } => {
            if (baseModelica) {
                literal!("")
            } else {
                literal!("_")
            }
        }
        _ => literal!(""),
    });
    Ok(name)
}

pub(crate) fn first(mut cref: metamodelica::Ref<NFComponentRef>) -> metamodelica::Ref<NFComponentRef> {
    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    let () = (match &*cref {
        CREF { .. } => {
            assign_variant_field!(cref => NFComponentRef::CREF; restCref = crate::NFComponentRef::interned_EMPTY());
            ()
        }
        _ => (),
    });
    cref
}

pub fn rest(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut restCref: metamodelica::Ref<NFComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match &((*cref)) {
        Deref @ CREF { restCref: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    restCref = metamodelica::Own::own(__pa0);
    Ok(restCref)
}

pub fn last<'__b>(mut cref: &'__b metamodelica::Ref<NFComponentRef>) -> metamodelica::Ref<NFComponentRef> {
    '__tco: loop {
        ::match_deref::match_deref! { match cref {
            Deref @ CREF { restCref: Deref @ CREF { .. }, .. } => { cref = var_field!((**cref).restCref, NFComponentRef::CREF); continue '__tco; },
            _ => return cref.clone(),
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn firstNonScope(mut cref: metamodelica::Ref<NFComponentRef>) -> Result<metamodelica::Ref<NFComponentRef>> {
    '__tco: loop {
        let mut rest_cr: metamodelica::Ref<NFComponentRef> = rest(&cref)?;
        match &*rest_cr {
            CREF {
                origin: Origin::SCOPE, ..
            } => return Ok(cref),
            EMPTY { .. } => return Ok(cref),
            _ => {
                cref = rest_cr;
                continue '__tco;
            }
        }
    }
}

pub fn append(
    mut cref: metamodelica::Ref<NFComponentRef>,
    mut restCref: &metamodelica::Ref<NFComponentRef>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    cref = (match &*cref {
        CREF {
            restCref: __cref_restCref,
            ..
        } => {
            assign_variant_field!(cref => NFComponentRef::CREF; restCref = append(__cref_restCref.clone(), restCref)?);
            cref
        }
        EMPTY { .. } => restCref.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(cref)
}

pub(crate) fn appendScope(
    mut scope: metamodelica::Ref<InstNode::InstNode>,
    mut cref: metamodelica::Ref<NFComponentRef>,
    mut includeRoot: bool,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    let mut prefix: metamodelica::Ref<NFComponentRef>;
    prefix = fromNodeList(&(NFInstNode::InstNode::scopeList(scope, includeRoot, metamodelica::nil())?))?;
    if !(isEmpty(&prefix)) {
        cref = append(cref, &prefix)?;
        cref = removeOuterCrefPrefix(cref)?;
    }
    Ok(cref)
}

pub fn prepend(
    mut restCref: metamodelica::Ref<NFComponentRef>,
    mut cref: metamodelica::Ref<NFComponentRef>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    cref = (match &*cref {
        CREF { .. } => {
            assign_variant_field!(cref => NFComponentRef::CREF; restCref = restCref);
            cref
        }
        EMPTY { .. } => restCref,
        _ => return Err("match: no arm matched"),
    });
    Ok(cref)
}

pub fn getComponentType(mut cref: &metamodelica::Ref<NFComponentRef>) -> metamodelica::Ref<Type::NFType> {
    let mut ty: metamodelica::Ref<Type::NFType>;
    ty = (match &**cref {
        CREF { ty: __cref_ty, .. } => __cref_ty.clone(),
        _ => crate::NFType::interned_UNKNOWN(),
    });
    ty
}

pub fn getSubscriptedType(
    mut cref: &metamodelica::Ref<NFComponentRef>,
    mut includeScope: bool,
) -> Result<metamodelica::Ref<Type::NFType>> {
    let mut ty: metamodelica::Ref<Type::NFType>;
    ty = (match &**cref {
        CREF {
            restCref: __cref_restCref,
            subscripts: __cref_subscripts,
            ty: __cref_ty,
            ..
        } => getSubscriptedType2(
            metamodelica::AsArg::as_arg(&__cref_restCref),
            Type::subscript(__cref_ty.clone(), metamodelica::AsArg::as_arg(&__cref_subscripts), true)?,
            includeScope,
        )?,
        _ => crate::NFType::interned_UNKNOWN(),
    });
    Ok(ty)
}

pub(crate) fn getSubscriptedType2<'__b>(
    mut restCref: &'__b metamodelica::Ref<NFComponentRef>,
    mut accumTy: metamodelica::Ref<Type::NFType>,
    mut includeScope: bool,
) -> Result<metamodelica::Ref<Type::NFType>> {
    let mut ty: metamodelica::Ref<Type::NFType>;
    ty = (match &**restCref {
        CREF { .. }
            if (var_field!((**restCref).origin, NFComponentRef::CREF).clone() == Origin::CREF.clone()
                || includeScope) =>
        {
            ty = Type::liftArrayLeftList(
                accumTy,
                &(Type::arrayDims(Type::subscript(
                    var_field!((**restCref).ty, NFComponentRef::CREF).clone(),
                    var_field!((**restCref).subscripts, NFComponentRef::CREF),
                    true,
                )?)),
            );
            getSubscriptedType2(
                var_field!((**restCref).restCref, NFComponentRef::CREF),
                ty,
                includeScope,
            )?
        }
        _ => accumTy,
    });
    Ok(ty)
}

pub(crate) fn lookupVarAttr(
    mut cref: &metamodelica::Ref<NFComponentRef>,
    mut attr_name: &ArcStr,
) -> Result<Option<metamodelica::Ref<Expression::NFExpression>>> {
    let mut attrValue: Option<metamodelica::Ref<Expression::NFExpression>>;
    attrValue = (match &**cref {
        CREF { .. } => {
            (match &*(node(cref)?) {
                NFInstNode::InstNode::VAR_NODE { varPointer: v, .. } => Binding::typedExp(
                    &(Variable::lookupTypeAttribute(attr_name, &(Pointer::access(PointerWeak::upgrade(v.clone())?)))),
                ),
                _ => None,
            })
        }
        _ => None,
    });
    Ok(attrValue)
}

pub(crate) fn nodeVariability(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<Variability> {
    let mut var: Variability;
    var = (match &**cref {
        CREF { .. } => {
            (::match_deref::match_deref! { match &(node(cref)?) {
                n @ Deref @ NFInstNode::InstNode::COMPONENT_NODE { .. } => {
                    Component::variability(&(NFInstNode::InstNode::component(metamodelica::AsArg::as_arg(&n))?))?
                },
                Deref @ NFInstNode::InstNode::CLASS_NODE { .. } => {
                    Variability::CONSTANT.clone()
                },
                Deref @ NFInstNode::InstNode::VAR_NODE { varPointer: v, .. } => {
                    Variable::variability(&(Pointer::access(PointerWeak::upgrade(v.clone())?)))
                },
                _ => {
                    Variability::CONTINUOUS.clone()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        }
        _ => Variability::CONTINUOUS.clone(),
    });
    Ok(var)
}

pub(crate) fn isResizable(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<bool> {
    let mut b: bool;
    b = (match &**cref {
        CREF { .. } => {
            (::match_deref::match_deref! { match &(node(cref)?) {
                n @ Deref @ NFInstNode::InstNode::COMPONENT_NODE { .. } => {
                    Component::isResizable(&(NFInstNode::InstNode::component(metamodelica::AsArg::as_arg(&n))?))
                },
                Deref @ NFInstNode::InstNode::VAR_NODE { varPointer: v, .. } => {
                    (::match_deref::match_deref! { match &(Pointer::access(PointerWeak::upgrade(v.clone())?)) {
                Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { annotations: Deref @ Annotations::ANNOTATIONS { resizable: __esc_b, .. }, .. }, .. } => {
                    b = (*__esc_b).clone();
                    b.clone()
                },
                _ => false,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
                },
                _ => {
                    false
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        }
        _ => false,
    });
    Ok(b)
}

pub(crate) fn subscriptsVariability(
    mut cref: &metamodelica::Ref<NFComponentRef>,
    mut var: Variability,
) -> Result<Variability> {
    let mut var: Variability = var;
    let () = (match &**cref {
        CREF {
            origin: Origin::CREF { .. },
            subscripts: __cref_subscripts,
            ..
        } => {
            for mut sub in &*__cref_subscripts.clone() {
                var = Prefixes::variabilityMax(var, Subscript::variability(metamodelica::AsArg::as_arg(&sub))?);
            }
            ()
        }
        _ => (),
    });
    Ok(var)
}

pub(crate) fn variability(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<Variability> {
    let mut var: Variability = Prefixes::variabilityMax(
        nodeVariability(cref)?,
        subscriptsVariability(cref, Prefixes::Variability::CONSTANT.clone())?,
    );
    Ok(var)
}

pub(crate) fn purity(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<Purity> {
    fn sub_purity(mut sub: &metamodelica::Ref<Subscript::NFSubscript>, mut pur: Purity) -> Result<Purity> {
        let mut pur: Purity = pur;
        pur = Prefixes::purityMin(pur, Subscript::purity(sub)?);
        Ok(pur)
    }

    let mut pur: Purity;
    pur = (match &**cref {
        CREF {
            origin: Origin::ITERATOR { .. },
            ..
        } => Purity::IMPURE.clone(),
        CREF { .. } => foldSubscripts(
            cref,
            &move |__a0: metamodelica::Ref<Subscript::NFSubscript>, __a1: Purity| sub_purity(&__a0, __a1),
            Purity::PURE.clone(),
            false,
        )?,
        _ => Purity::IMPURE.clone(),
    });
    Ok(pur)
}

pub(crate) fn visibility<'__b>(mut cref: &'__b metamodelica::Ref<NFComponentRef>) -> Result<Visibility> {
    '__tco: loop {
        match &**cref {
            CREF { .. } => {
                if (NFInstNode::InstNode::isProtected(&(node(cref)?))) {
                    return Ok(Visibility::PROTECTED.clone());
                } else {
                    {
                        cref = var_field!((**cref).restCref, NFComponentRef::CREF);
                        continue '__tco;
                    }
                }
            }
            _ => return Ok(Visibility::PUBLIC.clone()),
        }
    }
}

pub fn rename(
    mut name: ArcStr,
    mut cref: metamodelica::Ref<NFComponentRef>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    cref = (match &*cref {
        CREF { .. } => {
            assign_variant_field!(cref => NFComponentRef::CREF; node = NFInstNode::InstNode::handle(NFInstNode::InstNode::rename(name, node(&cref)?)?)?);
            cref
        }
        _ => cref,
    });
    Ok(cref)
}

pub(crate) fn addSubscript(
    mut subscript: metamodelica::Ref<Subscript::NFSubscript>,
    mut cref: metamodelica::Ref<NFComponentRef>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    let () = (match &*cref {
        CREF {
            subscripts: __cref_subscripts,
            ..
        } => {
            assign_variant_field!(cref => NFComponentRef::CREF; subscripts = listAppend(__cref_subscripts.clone(), list![subscript]));
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(cref)
}

pub fn mergeSubscripts(
    mut subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut cref: metamodelica::Ref<NFComponentRef>,
    mut applyToScope: bool,
    mut backend: bool,
    mut reverse: bool,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    let mut old_cref: metamodelica::Ref<NFComponentRef> = cref.clone();
    let mut new_subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    (new_subscripts, cref) = mergeSubscripts2(subscripts.clone(), cref, applyToScope, backend, reverse)?;
    if !((new_subscripts).is_empty()) {
        Error::terminate(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFComponentRef.mergeSubscripts"));
                __mm_s.push_str(&*literal!(" failed because the subscripts "));
                __mm_s.push_str(&*List::toString(
                    subscripts,
                    &move |__a0: metamodelica::Ref<Subscript::NFSubscript>| Subscript::toString(&__a0),
                    List::Style::FLAT_CURLY.clone(),
                )?);
                __mm_s.push_str(&*literal!(" could not be fully merged onto "));
                __mm_s.push_str(&*toString(&old_cref)?);
                __mm_s.push_str(&*literal!(".\nResult: "));
                __mm_s.push_str(&*toString(&cref)?);
                __mm_s.push_str(&*literal!(" with leftover: "));
                __mm_s.push_str(&*List::toString(
                    new_subscripts,
                    &move |__a0: metamodelica::Ref<Subscript::NFSubscript>| Subscript::toString(&__a0),
                    List::Style::FLAT_CURLY.clone(),
                )?);
                __mm_s.push_str(&*literal!("."));
                ArcStr::from(__mm_s)
            },
            &(metamodelica::sourceInfo!("NFFrontEnd/NFComponentRef.mo")),
        )?;
        return Err("fail");
    }
    Ok(cref)
}

pub(crate) fn mergeSubscripts2(
    mut subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut cref: metamodelica::Ref<NFComponentRef>,
    mut applyToScope: bool,
    mut backend: bool,
    mut reverse: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    metamodelica::Ref<NFComponentRef>,
)> {
    let mut subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = subscripts;
    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    (subscripts, cref) = ({
        let mut rest_cref: metamodelica::Ref<NFComponentRef> = crate::NFComponentRef::interned_EMPTY();
        (match &*cref {
            CREF {
                subscripts: cref_subs,
                node: __cref_node,
                origin: __cref_origin,
                restCref: __cref_restCref,
                ty: __cref_ty,
            } if (applyToScope || __cref_origin.clone() == Origin::CREF.clone()) => {
                let mut cref_subs = (*cref_subs).clone();
                if !(reverse) {
                    (subscripts, rest_cref) =
                        mergeSubscripts2(subscripts, __cref_restCref.clone(), applyToScope, backend, reverse)?;
                }
                if !((subscripts).is_empty()) {
                    (cref_subs, subscripts) = Subscript::mergeList(
                        subscripts,
                        cref_subs.clone(),
                        Type::dimensionCount(__cref_ty.clone()),
                        backend,
                    )?;
                }
                if reverse {
                    cref_subs = cref_subs.clone().reverse();
                    (subscripts, rest_cref) =
                        mergeSubscripts2(subscripts, __cref_restCref.clone(), applyToScope, backend, reverse)?;
                }
                (
                    subscripts,
                    metamodelica::Ref::new(NFComponentRef::CREF {
                        node: __cref_node.clone(),
                        subscripts: cref_subs.clone(),
                        ty: __cref_ty.clone(),
                        origin: __cref_origin.clone(),
                        restCref: rest_cref,
                    }),
                )
            }
            _ => (subscripts, cref),
        })
    });
    Ok((subscripts, cref))
}

pub fn mergeSubscriptsMapped(
    mut cref: metamodelica::Ref<NFComponentRef>,
    mut dims_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
            metamodelica::List<metamodelica::Ref<NFComponentRef>>,
        >,
    >,
    mut iter_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<NFComponentRef>, metamodelica::Ref<Subscript::NFSubscript>>,
    >,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    fn checkLocalDimensions(
        mut cref: metamodelica::Ref<NFComponentRef>,
        mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
        mut dims_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
                metamodelica::List<metamodelica::Ref<NFComponentRef>>,
            >,
        >,
        mut iter_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<NFComponentRef>, metamodelica::Ref<Subscript::NFSubscript>>,
        >,
    ) -> Result<metamodelica::Ref<NFComponentRef>> {
        let mut cref: metamodelica::Ref<NFComponentRef> = cref;
        let mut iter_crefs: Option<metamodelica::List<metamodelica::Ref<NFComponentRef>>>;
        let mut new_subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
        iter_crefs = UnorderedMap::get(dims, dims_map)?;
        if (iter_crefs).is_some() {
            new_subs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
                for mut iter_name in (Util::getOption(iter_crefs)?).into_iter().cloned() {
                    let __x = UnorderedMap::getSafe(
                        iter_name.clone(),
                        iter_map.clone(),
                        metamodelica::sourceInfo!("NFFrontEnd/NFComponentRef.mo"),
                    )?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            cref = mergeSubscripts(new_subs, cref, true, true, true)?;
        }
        Ok(cref)
    }

    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    cref = ({
        let mut ty: metamodelica::Ref<Type::NFType> = getSubscriptedType(&cref, false)?;
        (match &*cref {
            CREF { ty: __cref_ty, .. } if (Type::isArray(&ty)) => {
                let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
                let mut new_cref: metamodelica::Ref<NFComponentRef>;
                let mut num_local_dims: i32;
                dims = Type::arrayDims(ty.clone());
                num_local_dims = ((Type::arrayDims(__cref_ty.clone())).len() as i32);
                new_cref = cref;
                while num_local_dims > 0 {
                    new_cref = checkLocalDimensions(new_cref, dims.clone(), dims_map.clone(), iter_map.clone())?;
                    dims = List::stripLast(dims)?;
                    num_local_dims = num_local_dims - 1;
                }
                new_cref = (match &*new_cref {
                    CREF {
                        restCref: __new_cref_restCref,
                        ..
                    } => {
                        assign_variant_field!(new_cref => NFComponentRef::CREF; restCref = mergeSubscriptsMapped(__new_cref_restCref.clone(), dims_map, iter_map)?);
                        new_cref
                    }
                    _ => new_cref,
                });
                new_cref
            }
            CREF {
                restCref: __cref_restCref,
                ..
            } => {
                assign_variant_field!(cref => NFComponentRef::CREF; restCref = mergeSubscriptsMapped(__cref_restCref.clone(), dims_map, iter_map)?);
                cref
            }
            _ => cref,
        })
    });
    Ok(cref)
}

pub fn hasSubscripts(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<bool> {
    let mut hasSubscripts: bool;
    hasSubscripts = (match &**cref {
        CREF {
            restCref: __cref_restCref,
            subscripts: __cref_subscripts,
            ..
        } => !((__cref_subscripts).is_empty()) || self::hasSubscripts(metamodelica::AsArg::as_arg(&__cref_restCref))?,
        _ => false,
    });
    Ok(hasSubscripts)
}

pub(crate) fn hasNonModelSubscripts<'__b>(mut cref: &'__b metamodelica::Ref<NFComponentRef>) -> Result<bool> {
    '__tco: loop {
        match &**cref {
            CREF { .. } if (NFInstNode::InstNode::isModel(node(cref)?)?) => {
                cref = var_field!((**cref).restCref, NFComponentRef::CREF);
                continue '__tco;
            }
            CREF { .. } => {
                return Ok(!((var_field!((**cref).subscripts, NFComponentRef::CREF)).is_empty())
                    || hasNonModelSubscripts(var_field!((**cref).restCref, NFComponentRef::CREF))?);
            }
            _ => return Ok(false),
        }
    }
}

pub(crate) fn hasSplitSubscripts(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<bool> {
    let mut res: bool;
    res = (match &**cref {
        CREF {
            origin: Origin::CREF { .. },
            restCref: __cref_restCref,
            subscripts: __cref_subscripts,
            ..
        } => {
            List::any(
                metamodelica::AsArg::as_arg(&__cref_subscripts),
                &move |__a0: metamodelica::Ref<Subscript::NFSubscript>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(Subscript::isSplitIndex(&__a0))
                },
            )? || hasSplitSubscripts(metamodelica::AsArg::as_arg(&__cref_restCref))?
        }
        _ => false,
    });
    Ok(res)
}

pub(crate) fn expandSplitSubscripts(
    mut cref: metamodelica::Ref<NFComponentRef>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    let () = (match &*cref {
        CREF {
            origin: Origin::CREF { .. },
            subscripts: __cref_subscripts,
            ..
        } => {
            assign_variant_field!(cref => NFComponentRef::CREF;
                subscripts = Subscript::expandSplitIndices(__cref_subscripts.clone(), &(metamodelica::nil()))?,
                restCref = expandSplitSubscripts(var_field!((*cref).restCref, NFComponentRef::CREF).clone())?
            );
            ()
        }
        _ => (),
    });
    Ok(cref)
}

pub fn getSubscripts(
    mut cref: &metamodelica::Ref<NFComponentRef>,
) -> metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> {
    let mut subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    subscripts = (match &**cref {
        CREF {
            subscripts: __cref_subscripts,
            ..
        } => __cref_subscripts.clone(),
        _ => metamodelica::nil(),
    });
    subscripts
}

pub fn outermostIntegerSubscript(mut cref: &metamodelica::Ref<NFComponentRef>) -> i32 {
    let mut value: i32 = 0;
    let () = (match &**cref {
        CREF {
            restCref: __cref_restCref,
            subscripts: __cref_subscripts,
            ..
        } => {
            value = outermostIntegerSubscript(metamodelica::AsArg::as_arg(&__cref_restCref));
            if value == 0 {
                for mut s in &*__cref_subscripts.clone() {
                    let () = (::match_deref::match_deref! { match &(s.clone()) {
                        Deref @ Subscript::INDEX { index: Deref @ Expression::INTEGER { value: v } } => {
                            value = v.clone();
                            ()
                        },
                        _ => {
                            ()
                        },
                        _ => unreachable!("match_deref! exhaustiveness placeholder"),
                    } });
                    if value != 0 {
                        return value;
                    }
                }
            }
            ()
        }
        _ => (),
    });
    value
}

pub fn setSubscripts(
    mut subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut cref: metamodelica::Ref<NFComponentRef>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    let () = (match &*cref {
        CREF { .. } => {
            assign_variant_field!(cref => NFComponentRef::CREF; subscripts = subscripts);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(cref)
}

pub fn setSubscriptsList(
    mut subscripts: &metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>,
    mut cref: metamodelica::Ref<NFComponentRef>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &((subscripts.clone(), cref.clone())) {
        (Deref @ metamodelica::ListNode::Cons { head: subs, tail: rest_subs }, Deref @ CREF { .. }) => {
            let mut rest_cref: metamodelica::Ref<NFComponentRef>;
            rest_cref = setSubscriptsList(metamodelica::AsArg::as_arg(&rest_subs), var_field!((*cref).restCref, NFComponentRef::CREF).clone())?;
            metamodelica::Ref::new(NFComponentRef::CREF { node: var_field!((*cref).node, NFComponentRef::CREF).clone(), subscripts: subs.clone(), ty: var_field!((*cref).ty, NFComponentRef::CREF).clone(), origin: var_field!((*cref).origin, NFComponentRef::CREF).clone(), restCref: rest_cref })
        },
        (Deref @ metamodelica::ListNode::Nil, _) => {
            cref
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(cref)
}

pub fn copySubscripts(
    mut origin: &metamodelica::Ref<NFComponentRef>,
    mut target: metamodelica::Ref<NFComponentRef>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut target: metamodelica::Ref<NFComponentRef> = target;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = subscriptsAllFlat(origin)?;
    if !((subs).is_empty()) {
        target = mergeSubscripts(subs, target, true, true, false)?;
    }
    Ok(target)
}

pub(crate) fn subscriptsAllWithWhole<'__b>(
    mut cref: &'__b metamodelica::Ref<NFComponentRef>,
    mut accumSubs: metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match cref {
            Deref @ CREF { subscripts: Deref @ metamodelica::ListNode::Nil, .. } => {
                let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
                subs = metamodelica::nil();
                for mut dim in &*Type::arrayDims(var_field!((**cref).ty, NFComponentRef::CREF).clone()).reverse() {
                    subs = metamodelica::cons(metamodelica::Ref::new(Subscript::NFSubscript::SLICE { slice: Expression::makeRange(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }), None, Dimension::sizeExp(metamodelica::AsArg::as_arg(&dim))?)? }), subs);
                }
                { (cref, accumSubs) = (var_field!((**cref).restCref, NFComponentRef::CREF), metamodelica::cons(subs, accumSubs)); continue '__tco; }
            },
            Deref @ CREF { .. } => {
                { (cref, accumSubs) = (var_field!((**cref).restCref, NFComponentRef::CREF), metamodelica::cons(var_field!((**cref).subscripts, NFComponentRef::CREF).clone(), accumSubs)); continue '__tco; }
            },
            _ => {
                return Ok(accumSubs)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn subscriptsAllWithWholeFlat(
    mut cref: &metamodelica::Ref<NFComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>> {
    let mut subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> =
        List::flatten(subscriptsAllWithWhole(cref, metamodelica::nil())?)?;
    Ok(subscripts)
}

pub fn subscriptsAll(
    mut cref: &metamodelica::Ref<NFComponentRef>,
) -> metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>> {
    let mut subscripts: metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>> =
        metamodelica::Dangerous::listReverseInPlace(subscriptsAllReverse(cref, metamodelica::nil()));
    subscripts
}

pub(crate) fn subscriptsAllReverse<'__b>(
    mut cref: &'__b metamodelica::Ref<NFComponentRef>,
    mut accumSubs: metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>,
) -> metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>> {
    '__tco: loop {
        match &**cref {
            CREF { .. } => {
                (cref, accumSubs) = (
                    var_field!((**cref).restCref, NFComponentRef::CREF),
                    metamodelica::cons(var_field!((**cref).subscripts, NFComponentRef::CREF).clone(), accumSubs),
                );
                continue '__tco;
            }
            _ => return accumSubs,
        }
    }
}

pub fn subscriptsAllFlat(
    mut cref: &metamodelica::Ref<NFComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>> {
    let mut subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> =
        List::flattenReverse(subscriptsAll(cref))?;
    Ok(subscripts)
}

pub(crate) fn subscriptsExceptModel<'__b>(
    mut cref: &'__b metamodelica::Ref<NFComponentRef>,
    mut accumSubs: metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>> {
    '__tco: loop {
        match &**cref {
            CREF { .. } if (NFInstNode::InstNode::isModel(node(cref)?)?) => {
                (cref, accumSubs) = (
                    var_field!((**cref).restCref, NFComponentRef::CREF),
                    metamodelica::cons(metamodelica::nil(), accumSubs),
                );
                continue '__tco;
            }
            CREF { .. } => {
                (cref, accumSubs) = (
                    var_field!((**cref).restCref, NFComponentRef::CREF),
                    metamodelica::cons(var_field!((**cref).subscripts, NFComponentRef::CREF).clone(), accumSubs),
                );
                continue '__tco;
            }
            _ => return Ok(accumSubs),
        }
    }
}

pub(crate) fn subscriptsN(
    mut cref: metamodelica::Ref<NFComponentRef>,
    mut n: i32,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>> {
    let mut subscripts: metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>> =
        metamodelica::nil();
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut rest: metamodelica::Ref<NFComponentRef> = cref;
    for mut i in 1..=n {
        if isEmpty(&rest) {
            break;
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ CREF { subscripts: __pa0, restCref: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        subs = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        subscripts = metamodelica::cons(subs, subscripts);
    }
    Ok(subscripts)
}

pub(crate) fn transferSubscripts<'__b>(
    mut srcCref: &'__b metamodelica::Ref<NFComponentRef>,
    mut dstCref: metamodelica::Ref<NFComponentRef>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    cref = (::match_deref::match_deref! { match &((srcCref.clone(), dstCref.clone())) {
        (Deref @ EMPTY { .. }, _) => dstCref.clone(),
        (_, Deref @ EMPTY { .. }) => dstCref.clone(),
        (_, Deref @ WILD { .. }) => dstCref.clone(),
        (_, Deref @ CREF { origin: Origin::ITERATOR { .. }, .. }) => dstCref.clone(),
        (Deref @ CREF { .. }, Deref @ CREF { origin: Origin::CREF { .. }, .. }) => {
            assign_variant_field!(dstCref => NFComponentRef::CREF; restCref = transferSubscripts(srcCref, var_field!((*dstCref).restCref, NFComponentRef::CREF).clone())?);
            dstCref.clone()
        },
        (Deref @ CREF { .. }, Deref @ CREF { .. }) if (NFInstNode::InstNode::refEqual(&(node(srcCref)?), &(node(&dstCref)?))?) => {
            cref = transferSubscripts(var_field!((**srcCref).restCref, NFComponentRef::CREF), var_field!((*dstCref).restCref, NFComponentRef::CREF).clone())?;
            subs = if ((var_field!((**srcCref).subscripts, NFComponentRef::CREF)).is_empty()) {var_field!((*dstCref).subscripts, NFComponentRef::CREF).clone()} else {var_field!((**srcCref).subscripts, NFComponentRef::CREF).clone()};
            metamodelica::Ref::new(NFComponentRef::CREF { node: var_field!((*dstCref).node, NFComponentRef::CREF).clone(), subscripts: subs, ty: var_field!((*dstCref).ty, NFComponentRef::CREF).clone(), origin: var_field!((*dstCref).origin, NFComponentRef::CREF).clone(), restCref: cref })
        },
        (Deref @ CREF { .. }, Deref @ CREF { .. }) => transferSubscripts(var_field!((**srcCref).restCref, NFComponentRef::CREF), dstCref.clone())?,
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFComponentRef.transferSubscripts")); __mm_s.push_str(&*literal!(" failed")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFComponentRef.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub(crate) fn applySubscripts(
    mut cref: &metamodelica::Ref<NFComponentRef>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Subscript::NFSubscript>) -> Result<()>,
    mut applyToScope: bool,
) -> Result<()> {
    pub type FuncT =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Subscript::NFSubscript>) -> Result<()> + 'static>;

    let () = (match &**cref {
        CREF {
            origin: __cref_origin,
            restCref: __cref_restCref,
            subscripts: __cref_subscripts,
            ..
        } if (applyToScope || __cref_origin.clone() == Origin::CREF.clone()) => {
            for mut sub in &*__cref_subscripts.clone() {
                func(sub.clone())?;
            }
            applySubscripts(metamodelica::AsArg::as_arg(&__cref_restCref), func, applyToScope)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn foldSubscripts<'__b, ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut cref: &'__b metamodelica::Ref<NFComponentRef>,
    mut func: &'__b dyn ::std::ops::Fn(metamodelica::Ref<Subscript::NFSubscript>, ArgT) -> Result<ArgT>,
    mut arg: ArgT,
    mut applyToScope: bool,
) -> Result<ArgT> {
    pub type FuncT<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Subscript::NFSubscript>, ArgT) -> Result<ArgT> + 'static>;

    '__tco: loop {
        match &**cref {
            CREF { .. }
                if (applyToScope
                    || var_field!((**cref).origin, NFComponentRef::CREF).clone() == Origin::CREF.clone()) =>
            {
                for mut sub in &*var_field!((**cref).subscripts, NFComponentRef::CREF).clone() {
                    arg = func(sub.clone(), arg)?;
                }
                {
                    (cref, func, arg, applyToScope) = (
                        var_field!((**cref).restCref, NFComponentRef::CREF),
                        func,
                        arg,
                        applyToScope,
                    );
                    continue '__tco;
                }
            }
            _ => return Ok(arg),
        }
    }
}

pub fn mapSubscripts(
    mut cref: metamodelica::Ref<NFComponentRef>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Subscript::NFSubscript>) -> Result<metamodelica::Ref<Subscript::NFSubscript>>,
    mut applyToScope: bool,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    pub type FuncT = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Subscript::NFSubscript>,
            ) -> Result<metamodelica::Ref<Subscript::NFSubscript>>
            + 'static,
    >;

    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    cref = (match &*cref {
        CREF {
            origin: __cref_origin, ..
        } if (applyToScope || __cref_origin.clone() == Origin::CREF.clone()) => {
            if !((var_field!((*cref).subscripts, NFComponentRef::CREF)).is_empty()) {
                assign_variant_field!(cref => NFComponentRef::CREF; subscripts = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
                    for mut s in (var_field!((*cref).subscripts, NFComponentRef::CREF).clone()).into_iter().cloned() {
                        let __x = func(s.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }));
            }
            assign_variant_field!(cref => NFComponentRef::CREF; restCref = mapSubscripts(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), func, applyToScope)?);
            cref
        }
        _ => cref,
    });
    Ok(cref)
}

pub fn fillSubscripts(mut cref: metamodelica::Ref<NFComponentRef>) -> metamodelica::Ref<NFComponentRef> {
    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    let () = (match &*cref {
        CREF {
            subscripts: __cref_subscripts,
            ty: __cref_ty,
            ..
        } => {
            let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
            let mut dim_count: i32;
            let mut sub_count: i32;
            dims = Type::arrayDims(__cref_ty.clone());
            dim_count = ((dims).len() as i32);
            sub_count = ((__cref_subscripts).len() as i32);
            if sub_count < dim_count {
                assign_variant_field!(cref => NFComponentRef::CREF; subscripts = listAppend(var_field!((*cref).subscripts, NFComponentRef::CREF).clone(), List::fill(crate::NFSubscript::interned_WHOLE(), dim_count - sub_count)));
            }
            assign_variant_field!(cref => NFComponentRef::CREF; restCref = fillSubscripts(var_field!((*cref).restCref, NFComponentRef::CREF).clone()));
            ()
        }
        _ => (),
    });
    cref
}

pub(crate) fn replaceWholeSubscripts(
    mut cref: metamodelica::Ref<NFComponentRef>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    let () = (match &*cref {
        CREF { .. } => {
            let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
            let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
            if List::any(
                var_field!((*cref).subscripts, NFComponentRef::CREF),
                &move |__a0: metamodelica::Ref<Subscript::NFSubscript>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(Subscript::isWhole(&__a0))
                },
            )? {
                dims = Type::arrayDims(var_field!((*cref).ty, NFComponentRef::CREF).clone());
                subs = metamodelica::nil();
                for mut s in &*var_field!((*cref).subscripts, NFComponentRef::CREF).clone() {
                    let mut s = s.clone();
                    if Subscript::isWhole(&s) {
                        s = Subscript::fromDimension(&((dims).head().cloned()?))?;
                    }
                    subs = metamodelica::cons(s, subs);
                    dims = (dims).rest()?;
                }
                assign_variant_field!(cref => NFComponentRef::CREF; subscripts = metamodelica::Dangerous::listReverseInPlace(subs));
            }
            assign_variant_field!(cref => NFComponentRef::CREF; restCref = replaceWholeSubscripts(var_field!((*cref).restCref, NFComponentRef::CREF).clone())?);
            ()
        }
        _ => (),
    });
    Ok(cref)
}

pub(crate) fn combineSubscripts(
    mut cref: metamodelica::Ref<NFComponentRef>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    cref = fillSubscripts(cref);
    subs = List::flatten(subscriptsAllReverse(&cref, metamodelica::nil()))?;
    if (subs).is_empty() {
        return Ok(cref);
    }
    cref = setSubscripts(subs, stripSubscriptsAll(&cref))?;
    Ok(cref)
}

pub(crate) fn compare<'__b>(
    mut cref1: &'__b metamodelica::Ref<NFComponentRef>,
    mut cref2: &'__b metamodelica::Ref<NFComponentRef>,
) -> Result<i32> {
    let mut comp: i32;
    comp = (::match_deref::match_deref! { match (cref1, cref2) {
        (Deref @ CREF { .. }, Deref @ CREF { .. }) => {
            comp = stringCompare(&(NFInstNode::InstNode::name(&(node(cref1)?))?), &(NFInstNode::InstNode::name(&(node(cref2)?))?));
            if comp != 0 {
                return Ok(comp);
            }
            comp = Subscript::compareList(var_field!((**cref1).subscripts, NFComponentRef::CREF), var_field!((**cref2).subscripts, NFComponentRef::CREF).clone())?;
            if comp != 0 {
                return Ok(comp);
            }
            compare(var_field!((**cref1).restCref, NFComponentRef::CREF), var_field!((**cref2).restCref, NFComponentRef::CREF))?
        },
        (Deref @ EMPTY { .. }, Deref @ EMPTY { .. }) => 0,
        (Deref @ WILD { .. }, Deref @ WILD { .. }) => 0,
        (_, Deref @ EMPTY { .. }) => 1,
        (_, Deref @ WILD { .. }) => 1,
        (Deref @ EMPTY { .. }, _) => -1,
        (Deref @ WILD { .. }, _) => -1,
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFComponentRef.compare")); __mm_s.push_str(&*literal!(" failed")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFComponentRef.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(comp)
}

pub fn isEqual(
    mut cref1: &metamodelica::Ref<NFComponentRef>,
    mut cref2: &metamodelica::Ref<NFComponentRef>,
) -> Result<bool> {
    let mut b: bool;
    if referenceEq(&*(&**cref1), &*(&**cref2)) {
        b = true;
        return Ok(b);
    }
    b = (::match_deref::match_deref! { match (cref1, cref2) {
        (Deref @ CREF { .. }, Deref @ CREF { .. }) => metamodelica::stringEq(&(nodeName(cref1)?), &(nodeName(cref2)?)) && Subscript::isEqualList(var_field!((**cref1).subscripts, NFComponentRef::CREF), var_field!((**cref2).subscripts, NFComponentRef::CREF).clone())? && isEqual(var_field!((**cref1).restCref, NFComponentRef::CREF), var_field!((**cref2).restCref, NFComponentRef::CREF))?,
        (Deref @ EMPTY { .. }, Deref @ EMPTY { .. }) => true,
        (Deref @ WILD { .. }, Deref @ WILD { .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn isEqualStrip(
    mut cref1: &metamodelica::Ref<NFComponentRef>,
    mut cref2: &metamodelica::Ref<NFComponentRef>,
) -> Result<bool> {
    let mut b: bool;
    if referenceEq(&*(&**cref1), &*(&**cref2)) {
        b = true;
        return Ok(b);
    }
    b = (::match_deref::match_deref! { match (cref1, cref2) {
        (Deref @ CREF { .. }, Deref @ CREF { .. }) => metamodelica::stringEq(&(nodeName(cref1)?), &(nodeName(cref2)?)) && isEqualStrip(var_field!((**cref1).restCref, NFComponentRef::CREF), var_field!((**cref2).restCref, NFComponentRef::CREF))?,
        (Deref @ EMPTY { .. }, Deref @ EMPTY { .. }) => true,
        (Deref @ WILD { .. }, Deref @ WILD { .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isLess(
    mut cref1: &metamodelica::Ref<NFComponentRef>,
    mut cref2: &metamodelica::Ref<NFComponentRef>,
) -> Result<bool> {
    let mut isLess: bool = compare(cref1, cref2)? < 0;
    Ok(isLess)
}

pub(crate) fn isGreater(
    mut cref1: &metamodelica::Ref<NFComponentRef>,
    mut cref2: &metamodelica::Ref<NFComponentRef>,
) -> Result<bool> {
    let mut isGreater: bool = compare(cref1, cref2)? > 0;
    Ok(isGreater)
}

pub(crate) fn isPrefix(
    mut cref1: &metamodelica::Ref<NFComponentRef>,
    mut cref2: &metamodelica::Ref<NFComponentRef>,
) -> Result<bool> {
    let mut isPrefix: bool;
    if referenceEq(&*(&**cref1), &*(&**cref2)) {
        isPrefix = true;
        return Ok(isPrefix);
    }
    isPrefix = (::match_deref::match_deref! { match (cref1, cref2) {
        (Deref @ CREF { .. }, Deref @ CREF { .. }) => if (metamodelica::stringEq(&(NFInstNode::InstNode::name(&(node(cref1)?))?), &(NFInstNode::InstNode::name(&(node(cref2)?))?))) {isEqual(var_field!((**cref1).restCref, NFComponentRef::CREF), var_field!((**cref2).restCref, NFComponentRef::CREF))?} else {isEqual(cref1, var_field!((**cref2).restCref, NFComponentRef::CREF))?},
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isPrefix)
}

pub(crate) fn toAbsyn(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut acref: metamodelica::Ref<Absyn::ComponentRef>;
    acref = (match &**cref {
        CREF {
            restCref: __cref_restCref,
            subscripts: __cref_subscripts,
            ..
        } => {
            acref = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                name: nodeName(cref)?,
                subscripts: ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Subscript>> = metamodelica::nil();
                    for mut s in (__cref_subscripts.clone()).into_iter().cloned() {
                        let __x = Subscript::toAbsyn(&(s.clone()))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            });
            toAbsyn_impl(metamodelica::AsArg::as_arg(&__cref_restCref), acref)?
        }
        WILD { .. } => openmodelica_ast::Absyn::ComponentRef::interned_WILD(),
        _ => return Err("match: no arm matched"),
    });
    Ok(acref)
}

pub(crate) fn toAbsyn_impl<'__b>(
    mut cref: &'__b metamodelica::Ref<NFComponentRef>,
    mut accumCref: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut acref: metamodelica::Ref<Absyn::ComponentRef>;
    acref = (match &**cref {
        EMPTY { .. } => accumCref,
        CREF { .. } => {
            acref = metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL {
                name: nodeName(cref)?,
                subscripts: ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Subscript>> = metamodelica::nil();
                    for mut s in (var_field!((**cref).subscripts, NFComponentRef::CREF).clone())
                        .into_iter()
                        .cloned()
                    {
                        let __x = Subscript::toAbsyn(&(s.clone()))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                componentRef: accumCref,
            });
            toAbsyn_impl(var_field!((**cref).restCref, NFComponentRef::CREF), acref)?
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(acref)
}

pub fn toDAE(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut dcref: metamodelica::Ref<DAE::ComponentRef>;
    dcref = (match &**cref {
        CREF {
            restCref: __cref_restCref,
            subscripts: __cref_subscripts,
            ty: __cref_ty,
            ..
        } => {
            dcref = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: nodeName(cref)?,
                identType: Type::toDAE(metamodelica::AsArg::as_arg(&__cref_ty), true)?,
                subscriptLst: ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = metamodelica::nil();
                    for mut s in (__cref_subscripts.clone()).into_iter().cloned() {
                        let __x = Subscript::toDAE(&(s.clone()))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            });
            toDAE_impl(metamodelica::AsArg::as_arg(&__cref_restCref), dcref)?
        }
        WILD { .. } => openmodelica_frontend_types::DAE::ComponentRef::interned_WILD(),
        _ => return Err("match: no arm matched"),
    });
    Ok(dcref)
}

pub(crate) fn toDAE_impl<'__b>(
    mut cref: &'__b metamodelica::Ref<NFComponentRef>,
    mut accumCref: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut dcref: metamodelica::Ref<DAE::ComponentRef>;
    dcref = (match &**cref {
        EMPTY { .. } => accumCref,
        CREF { .. } => {
            let mut ty: metamodelica::Ref<Type::NFType>;
            let mut dty: metamodelica::Ref<DAE::Type>;
            ty = if (Type::isUnknown(var_field!((**cref).ty, NFComponentRef::CREF))) {
                NFInstNode::InstNode::getType(node(cref)?)?
            } else {
                var_field!((**cref).ty, NFComponentRef::CREF).clone()
            };
            dty = Type::toDAE(&ty, false)?;
            dcref = metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
                ident: nodeName(cref)?,
                identType: dty,
                subscriptLst: ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = metamodelica::nil();
                    for mut s in (var_field!((**cref).subscripts, NFComponentRef::CREF).clone())
                        .into_iter()
                        .cloned()
                    {
                        let __x = Subscript::toDAE(&(s.clone()))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                componentRef: accumCref,
            });
            toDAE_impl(var_field!((**cref).restCref, NFComponentRef::CREF), dcref)?
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(dcref)
}

pub fn toString(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(toString_impl(cref, metamodelica::nil())?, literal!("."));
    Ok(r#str)
}

pub(crate) fn toString_impl<'__b>(
    mut cref: &'__b metamodelica::Ref<NFComponentRef>,
    mut strl: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<ArcStr>> {
    '__tco: loop {
        match &**cref {
            CREF { .. } => {
                let mut r#str: ArcStr;
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*nodeName(cref)?);
                    __mm_s.push_str(&*Subscript::toStringList(
                        var_field!((**cref).subscripts, NFComponentRef::CREF).clone(),
                    )?);
                    ArcStr::from(__mm_s)
                };
                {
                    (cref, strl) = (
                        var_field!((**cref).restCref, NFComponentRef::CREF),
                        metamodelica::cons(r#str, strl),
                    );
                    continue '__tco;
                }
            }
            WILD { .. } => return Ok(metamodelica::cons(literal!("_"), strl)),
            _ => return Ok(strl),
        }
    }
}

pub(crate) fn toFlatString(
    mut cref: &metamodelica::Ref<NFComponentRef>,
    mut format: BaseModelica::OutputFormat,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut strl: metamodelica::List<ArcStr>;
    let mut crefs: metamodelica::List<metamodelica::Ref<NFComponentRef>>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut cr: metamodelica::Ref<NFComponentRef>;
    let mut escapeQuotes: bool;
    r#str = firstName(cref, true)?;
    if metamodelica::stringEq(&r#str, &(literal!("time"))) || metamodelica::stringEq(&r#str, &(literal!(""))) {
        return Ok(r#str);
    }
    crefs = toListReverse(cref, true, metamodelica::nil());
    strl = list![literal!("'")];
    subs = metamodelica::nil();
    if format.scalarizeMode.clone() == BaseModelica::ScalarizeMode::NOT_SCALARIZED.clone() {
        while !((crefs).is_empty()) {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(crefs) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa0);
            crefs = metamodelica::Own::own(__pa1);
            strl = metamodelica::cons(Util::escapeQuotes(firstName(&cr, true)?)?, strl);
            subs = listAppend(getSubscripts(&cr), subs);
            if format.recordMode.clone() == BaseModelica::RecordMode::WITH_RECORDS.clone()
                && isCref(&cr)
                && Type::isRecord(&(scalarType(&cr)?))
                && !((crefs).is_empty())
            {
                strl = metamodelica::cons(literal!("'"), strl);
                if !((subs).is_empty()) {
                    strl = metamodelica::cons(Subscript::toFlatStringList(subs, format, false)?, strl);
                    subs = metamodelica::nil();
                }
                if !((crefs).is_empty()) {
                    strl = metamodelica::cons(literal!(".'"), strl);
                }
            } else if !((crefs).is_empty()) {
                strl = metamodelica::cons(literal!("."), strl);
            }
        }
    } else {
        while !((crefs).is_empty()) {
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(crefs) {
                Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa2);
            crefs = metamodelica::Own::own(__pa3);
            strl = metamodelica::cons(Util::escapeQuotes(firstName(&cr, true)?)?, strl);
            subs = getSubscripts(&cr);
            if !((subs).is_empty())
                && !(format.scalarizeMode.clone() == BaseModelica::ScalarizeMode::PARTIALLY_SCALARIZED.clone()
                    && (crefs).is_empty())
            {
                strl = metamodelica::cons(Subscript::toFlatStringList(subs, format, true)?, strl);
            }
            if !((crefs).is_empty()) {
                if format.recordMode.clone() == BaseModelica::RecordMode::WITH_RECORDS.clone()
                    && isCref(&cr)
                    && Type::isRecord(&(scalarType(&cr)?))
                {
                    strl = metamodelica::cons(literal!("'.'"), strl);
                } else {
                    strl = metamodelica::cons(literal!("."), strl);
                }
            }
        }
        if format.scalarizeMode.clone() == BaseModelica::ScalarizeMode::PARTIALLY_SCALARIZED.clone() {
            subs = getSubscripts(cref);
        } else {
            subs = metamodelica::nil();
        }
    }
    strl = metamodelica::cons(literal!("'"), strl);
    if !((subs).is_empty()) {
        strl = metamodelica::cons(
            Subscript::toFlatStringList(
                subs,
                format,
                format.scalarizeMode.clone() == BaseModelica::ScalarizeMode::SCALARIZED.clone(),
            )?,
            strl,
        );
    }
    r#str = stringAppendList(strl.reverse());
    Ok(r#str)
}

pub fn listToString(mut crs: metamodelica::List<metamodelica::Ref<NFComponentRef>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("{"));
        __mm_s.push_str(&*stringDelimitList(
            List::map(crs, &move |__a0: metamodelica::Ref<NFComponentRef>| toString(&__a0))?,
            literal!(","),
        ));
        __mm_s.push_str(&*literal!("}"));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

pub(crate) fn toJSON(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    json = (match &**cref {
        CREF { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair(&(literal!("$kind")), &(JSON::makeString(literal!("cref"))), json)?;
            json = JSON::addPair(
                &(literal!("parts")),
                &(JSON::makeList(toJSON_impl(cref, metamodelica::nil())?)),
                json,
            )?;
            json
        }
        EMPTY { .. } => JSON::makeNull(),
        WILD { .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair(&(literal!("$kind")), &(JSON::makeString(literal!("cref"))), json)?;
            json = JSON::addPair(
                &(literal!("parts")),
                &(JSON::makeList(list![JSON::fromPair(
                    &(literal!("name")),
                    &(JSON::makeString(literal!("_")))
                )?])),
                json,
            )?;
            json
        }
        _ => JSON::makeString(toString(cref)?),
    });
    Ok(json)
}

pub(crate) fn toJSON_impl<'__b>(
    mut cref: &'__b metamodelica::Ref<NFComponentRef>,
    mut accum: metamodelica::List<metamodelica::Ref<JSON::JSON>>,
) -> Result<metamodelica::List<metamodelica::Ref<JSON::JSON>>> {
    '__tco: loop {
        let mut obj: metamodelica::Ref<JSON::JSON>;
        match &**cref {
            CREF { .. } => {
                obj = JSON::emptyListObject();
                obj = JSON::addPair(&(literal!("name")), &(JSON::makeString(nodeName(cref)?)), obj)?;
                if !((var_field!((**cref).subscripts, NFComponentRef::CREF)).is_empty()) {
                    obj = JSON::addPair(
                        &(literal!("subscripts")),
                        &(Subscript::toJSONList(var_field!((**cref).subscripts, NFComponentRef::CREF))?),
                        obj,
                    )?;
                }
                if (isEmpty(var_field!((**cref).restCref, NFComponentRef::CREF))) {
                    return Ok(toJSON_context(node(cref)?, metamodelica::cons(obj, accum))?);
                } else {
                    {
                        (cref, accum) = (
                            var_field!((**cref).restCref, NFComponentRef::CREF),
                            metamodelica::cons(obj, accum),
                        );
                        continue '__tco;
                    }
                }
            }
            _ => return Ok(accum),
        }
    }
}

pub(crate) fn toJSON_context(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut accum: metamodelica::List<metamodelica::Ref<JSON::JSON>>,
) -> Result<metamodelica::List<metamodelica::Ref<JSON::JSON>>> {
    let mut accum: metamodelica::List<metamodelica::Ref<JSON::JSON>> = accum;
    let mut opt_context: Option<metamodelica::Ref<Absyn::Path>>;
    opt_context = NFInstNode::InstNode::rootClassContext(&(NFInstNode::InstNode::instanceParent(node)?));
    if (opt_context).is_some() {
        for mut name in &*AbsynUtil::pathToStringListReverse(&(Util::getOption(opt_context)?), metamodelica::nil()) {
            accum = metamodelica::cons(
                JSON::addPair(
                    &(literal!("name")),
                    &(JSON::makeString(name.clone())),
                    JSON::emptyListObject(),
                )?,
                accum,
            );
        }
    }
    Ok(accum)
}

pub fn hash(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<i32> {
    let mut hash: i32 = hashContinue(cref, false, Util::HASH_SEED.clone())?;
    Ok(hash)
}

pub fn hashStrip(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<i32> {
    let mut hash: i32 = hashContinue(cref, true, Util::HASH_SEED.clone())?;
    Ok(hash)
}

pub(crate) fn hashContinue<'__b>(
    mut cref: &'__b metamodelica::Ref<NFComponentRef>,
    mut strip: bool,
    mut hash: i32,
) -> Result<i32> {
    '__tco: loop {
        match &**cref {
            CREF { .. } => {
                hash = stringHashDjb2Continue(&(nodeName(cref)?), hash);
                if !(strip) {
                    for mut s in &*var_field!((**cref).subscripts, NFComponentRef::CREF).clone() {
                        hash = Subscript::hashStringContinue(metamodelica::AsArg::as_arg(&s), hash)?;
                    }
                }
                {
                    (cref, strip, hash) = (var_field!((**cref).restCref, NFComponentRef::CREF), strip, hash);
                    continue '__tco;
                }
            }
            WILD { .. } => return Ok(stringHashDjb2Continue(&(literal!("_")), hash)),
            _ => return Ok(hash),
        }
    }
}

pub(crate) fn toPath(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut path: metamodelica::Ref<Absyn::Path>;
    path = (match &**cref {
        CREF {
            restCref: __cref_restCref,
            ..
        } => toPath_impl(
            metamodelica::AsArg::as_arg(&__cref_restCref),
            metamodelica::Ref::new(Absyn::Path::IDENT { name: nodeName(cref)? }),
        )?,
        _ => return Err("match: no arm matched"),
    });
    Ok(path)
}

pub(crate) fn toPath_impl<'__b>(
    mut cref: &'__b metamodelica::Ref<NFComponentRef>,
    mut accumPath: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    '__tco: loop {
        match &**cref {
            CREF { .. } => {
                (cref, accumPath) = (
                    var_field!((**cref).restCref, NFComponentRef::CREF),
                    metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                        name: nodeName(cref)?,
                        path: accumPath,
                    }),
                );
                continue '__tco;
            }
            _ => return Ok(accumPath),
        }
    }
}

pub(crate) fn fromNodeList(
    mut nodes: &metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = crate::NFComponentRef::interned_EMPTY();
    for mut n in &**nodes {
        cref = metamodelica::Ref::new(NFComponentRef::CREF {
            node: NFInstNode::InstNode::handle(n.clone())?,
            subscripts: metamodelica::nil(),
            ty: NFInstNode::InstNode::getType(n.clone())?,
            origin: Origin::SCOPE.clone(),
            restCref: cref,
        });
    }
    Ok(cref)
}

pub fn scalarize(
    mut cref: metamodelica::Ref<NFComponentRef>,
    mut resize: bool,
) -> Result<metamodelica::List<metamodelica::Ref<NFComponentRef>>> {
    let mut crefs: metamodelica::List<metamodelica::Ref<NFComponentRef>>;
    crefs = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { ty: Deref @ Type::ARRAY { .. }, subscripts: __cref_subscripts, .. } => {
            let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
            let mut subs: metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>;
            dims = Type::arrayDims(var_field!((*cref).ty, NFComponentRef::CREF).clone());
            subs = Subscript::scalarizeList(metamodelica::AsArg::as_arg(&__cref_subscripts), dims, resize)?;
            subs = List::combination(&subs);
            ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFComponentRef>> = metamodelica::nil();
        for mut s in (subs).into_iter().cloned() {
            let __x = setSubscripts(s.clone(), cref.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        _ => {
            list![cref]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(crefs)
}

pub fn scalarizeAll(
    mut cref: metamodelica::Ref<NFComponentRef>,
    mut resize: bool,
) -> Result<metamodelica::List<metamodelica::Ref<NFComponentRef>>> {
    let mut crefs: metamodelica::List<metamodelica::Ref<NFComponentRef>>;
    let mut next: metamodelica::Ref<NFComponentRef> = cref;
    let mut nested_crefs: metamodelica::List<metamodelica::List<metamodelica::Ref<NFComponentRef>>> =
        metamodelica::nil();
    while !(isEmpty(&next)) {
        nested_crefs = metamodelica::cons(scalarize(next.clone(), resize)?, nested_crefs);
        let __pa0 = ::match_deref::match_deref! { match &(next) {
            Deref @ CREF { restCref: __pa0, .. } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        next = metamodelica::Own::own(__pa0);
    }
    crefs = scalarizeAll_Nesting(
        &nested_crefs,
        &(crate::NFComponentRef::interned_EMPTY()),
        metamodelica::nil(),
    )?;
    Ok(crefs)
}

pub(crate) fn scalarizeAll_Nesting(
    mut nested_crefs: &metamodelica::List<metamodelica::List<metamodelica::Ref<NFComponentRef>>>,
    mut cref: &metamodelica::Ref<NFComponentRef>,
    mut crefs: metamodelica::List<metamodelica::Ref<NFComponentRef>>,
) -> Result<metamodelica::List<metamodelica::Ref<NFComponentRef>>> {
    let mut crefs: metamodelica::List<metamodelica::Ref<NFComponentRef>> = crefs;
    crefs = (::match_deref::match_deref! { match nested_crefs {
        Deref @ metamodelica::ListNode::Cons { head: head, tail: tail } => {
            let mut empty: bool;
            empty = (tail).is_empty();
            for mut head_cref in &*head.clone() {
                let mut head_cref = head_cref.clone();
                crefs = (match &*head_cref {
        CREF { .. } => {
            assign_variant_field!(head_cref => NFComponentRef::CREF; restCref = cref.clone());
            if empty {
                crefs = metamodelica::cons(head_cref, crefs);
            } else {
                crefs = scalarizeAll_Nesting(tail, &head_cref, crefs)?;
            }
            crefs
        },
        _ => return Err("match: no arm matched"),
    });
            }
            crefs
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(crefs)
}

pub fn scalarizeSlice(
    mut cref: metamodelica::Ref<NFComponentRef>,
    mut slice: metamodelica::List<i32>,
    mut resize: bool,
) -> Result<metamodelica::List<metamodelica::Ref<NFComponentRef>>> {
    let mut crefs: metamodelica::List<metamodelica::Ref<NFComponentRef>>;
    let mut next: metamodelica::Ref<NFComponentRef> = cref;
    let mut nested_crefs: metamodelica::List<metamodelica::List<metamodelica::Ref<NFComponentRef>>> =
        metamodelica::nil();
    while !(isEmpty(&next)) {
        nested_crefs = metamodelica::cons(scalarize(next.clone(), resize)?, nested_crefs);
        let __pa0 = ::match_deref::match_deref! { match &(next) {
            Deref @ CREF { restCref: __pa0, .. } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        next = metamodelica::Own::own(__pa0);
    }
    crefs = scalarizeAll_Nesting(
        &nested_crefs,
        &(crate::NFComponentRef::interned_EMPTY()),
        metamodelica::nil(),
    )?;
    if !((slice).is_empty()) {
        crefs = List::getAtIndexLst(crefs, slice, true)?;
    }
    Ok(crefs)
}

pub(crate) fn isPackageConstant(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<bool> {
    let mut isPkgConst: bool;
    isPkgConst = nodeVariability(cref)? <= Variability::PARAMETER.clone() && isPackageConstant2(cref)?;
    Ok(isPkgConst)
}

pub(crate) fn isPackageConstant2<'__b>(mut cref: &'__b metamodelica::Ref<NFComponentRef>) -> Result<bool> {
    '__tco: loop {
        match &**cref {
            CREF { .. } if (NFInstNode::InstNode::isClass(&(node(cref)?))?) => {
                return Ok(NFInstNode::InstNode::isUserdefinedClass(node(cref)?)?);
            }
            CREF {
                origin: Origin::CREF { .. },
                ..
            } => {
                cref = var_field!((**cref).restCref, NFComponentRef::CREF);
                continue '__tco;
            }
            _ => return Ok(false),
        }
    }
}

pub(crate) fn stripSubscripts(
    mut cref: metamodelica::Ref<NFComponentRef>,
) -> (
    metamodelica::Ref<NFComponentRef>,
    metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
) {
    let mut strippedCref: metamodelica::Ref<NFComponentRef>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    (strippedCref, subs) = (match &*cref {
        CREF {
            node: __cref_node,
            origin: __cref_origin,
            restCref: __cref_restCref,
            subscripts: __cref_subscripts,
            ty: __cref_ty,
        } => (
            metamodelica::Ref::new(NFComponentRef::CREF {
                node: __cref_node.clone(),
                subscripts: metamodelica::nil(),
                ty: __cref_ty.clone(),
                origin: __cref_origin.clone(),
                restCref: __cref_restCref.clone(),
            }),
            __cref_subscripts.clone(),
        ),
        _ => (cref, metamodelica::nil()),
    });
    (strippedCref, subs)
}

pub fn stripSubscriptsAll(mut cref: &metamodelica::Ref<NFComponentRef>) -> metamodelica::Ref<NFComponentRef> {
    let mut strippedCref: metamodelica::Ref<NFComponentRef>;
    strippedCref = (match &**cref {
        CREF {
            node: __cref_node,
            origin: __cref_origin,
            restCref: __cref_restCref,
            ty: __cref_ty,
            ..
        } => metamodelica::Ref::new(NFComponentRef::CREF {
            node: __cref_node.clone(),
            subscripts: metamodelica::nil(),
            ty: __cref_ty.clone(),
            origin: __cref_origin.clone(),
            restCref: stripSubscriptsAll(metamodelica::AsArg::as_arg(&__cref_restCref)),
        }),
        _ => cref.clone(),
    });
    strippedCref
}

pub(crate) fn stripSubscriptsExceptModel(
    mut cref: metamodelica::Ref<NFComponentRef>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    cref = (match &*cref {
        CREF {
            restCref,
            node: __cref_node,
            origin: __cref_origin,
            subscripts: __cref_subscripts,
            ty: __cref_ty,
        } if (NFInstNode::InstNode::isModel(self::node(&cref)?)?) => metamodelica::Ref::new(NFComponentRef::CREF {
            node: __cref_node.clone(),
            subscripts: __cref_subscripts.clone(),
            ty: __cref_ty.clone(),
            origin: __cref_origin.clone(),
            restCref: stripSubscriptsExceptModel(restCref.clone())?,
        }),
        CREF {
            restCref,
            node: __cref_node,
            origin: __cref_origin,
            ty: __cref_ty,
            ..
        } => metamodelica::Ref::new(NFComponentRef::CREF {
            node: __cref_node.clone(),
            subscripts: metamodelica::nil(),
            ty: __cref_ty.clone(),
            origin: __cref_origin.clone(),
            restCref: stripSubscriptsExceptModel(restCref.clone())?,
        }),
        _ => cref.clone(),
    });
    Ok(cref)
}

pub fn stripIteratorSubscripts(
    mut cref: metamodelica::Ref<NFComponentRef>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let () = (match &*cref {
        CREF { .. } => {
            if !((var_field!((*cref).subscripts, NFComponentRef::CREF)).is_empty())
                && Subscript::isIterator(&(List::last(var_field!((*cref).subscripts, NFComponentRef::CREF))?))
            {
                subs = var_field!((*cref).subscripts, NFComponentRef::CREF).clone().reverse();
                subs = List::trim(
                    subs,
                    &move |__a0: metamodelica::Ref<Subscript::NFSubscript>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(Subscript::isIterator(&__a0))
                    },
                )?;
                assign_variant_field!(cref => NFComponentRef::CREF; subscripts = metamodelica::Dangerous::listReverseInPlace(subs));
            }
            assign_variant_field!(cref => NFComponentRef::CREF; restCref = stripIteratorSubscripts(var_field!((*cref).restCref, NFComponentRef::CREF).clone())?);
            ()
        }
        _ => (),
    });
    Ok(cref)
}

pub fn simplifySubscripts(
    mut cref: metamodelica::Ref<NFComponentRef>,
    mut trim: bool,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut rest_cref: metamodelica::Ref<NFComponentRef>;
    let mut dirty: bool = false;
    cref = (match &*cref {
        CREF {
            subscripts: __esc_subs,
            node: __cref_node,
            origin: __cref_origin,
            restCref: __cref_restCref,
            ty: __cref_ty,
        } => {
            subs = (*__esc_subs).clone();
            if !((subs).is_empty()) {
                subs = Subscript::simplifyList(
                    var_field!((*cref).subscripts, NFComponentRef::CREF).clone(),
                    Type::arrayDims(__cref_ty.clone()),
                    trim,
                )?;
                dirty = true;
            }
            rest_cref = simplifySubscripts(__cref_restCref.clone(), trim)?;
            dirty = dirty || !(referenceEq(&*(&*rest_cref), &*(__cref_restCref.clone())));
            if (dirty) {
                metamodelica::Ref::new(NFComponentRef::CREF {
                    node: __cref_node.clone(),
                    subscripts: subs.clone(),
                    ty: __cref_ty.clone(),
                    origin: __cref_origin.clone(),
                    restCref: rest_cref,
                })
            } else {
                cref
            }
        }
        _ => cref,
    });
    Ok(cref)
}

pub(crate) fn evaluateSubscripts(
    mut cref: metamodelica::Ref<NFComponentRef>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ CREF { subscripts: Deref @ metamodelica::ListNode::Nil, origin: Origin::CREF { .. }, restCref: __cref_restCref, .. } => {
            assign_variant_field!(cref => NFComponentRef::CREF; restCref = evaluateSubscripts(__cref_restCref.clone())?);
            cref
        },
        Deref @ CREF { origin: Origin::CREF { .. }, node: __cref_node, restCref: __cref_restCref, subscripts: __cref_subscripts, ty: __cref_ty } => {
            let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
            subs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
        for mut s in (__cref_subscripts.clone()).into_iter().cloned() {
            let __x = Subscript::eval(s.clone(), &(NFCeval::noTarget().clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            metamodelica::Ref::new(NFComponentRef::CREF { node: __cref_node.clone(), subscripts: subs, ty: __cref_ty.clone(), origin: var_field!((*cref).origin, NFComponentRef::CREF).clone(), restCref: evaluateSubscripts(__cref_restCref.clone())? })
        },
        _ => {
            cref
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub(crate) fn isDeleted(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<bool> {
    let mut isDeleted: bool;
    isDeleted = (match &**cref {
        CREF {
            origin: Origin::CREF { .. },
            restCref: __cref_restCref,
            ..
        } => {
            let mut node: metamodelica::Ref<InstNode::InstNode>;
            node = self::node(cref)?;
            NFInstNode::InstNode::isComponent(&node)?
                && Component::isDeleted(&(NFInstNode::InstNode::component(&node)?))?
                || self::isDeleted(metamodelica::AsArg::as_arg(&__cref_restCref))?
        }
        _ => false,
    });
    Ok(isDeleted)
}

pub(crate) fn isFromCref(mut cref: &metamodelica::Ref<NFComponentRef>) -> bool {
    let mut fromCref: bool;
    fromCref = (match &**cref {
        CREF {
            origin: Origin::CREF { .. },
            ..
        } => true,
        WILD { .. } => true,
        _ => false,
    });
    fromCref
}

pub(crate) fn toListReverse<'__b>(
    mut cref: &'__b metamodelica::Ref<NFComponentRef>,
    mut includeScope: bool,
    mut accum: metamodelica::List<metamodelica::Ref<NFComponentRef>>,
) -> metamodelica::List<metamodelica::Ref<NFComponentRef>> {
    '__tco: loop {
        match &**cref {
            CREF { .. } if (includeScope) => {
                (cref, includeScope, accum) = (
                    var_field!((**cref).restCref, NFComponentRef::CREF),
                    includeScope,
                    metamodelica::cons(cref.clone(), accum),
                );
                continue '__tco;
            }
            CREF {
                origin: Origin::CREF { .. },
                ..
            } => {
                (cref, includeScope, accum) = (
                    var_field!((**cref).restCref, NFComponentRef::CREF),
                    includeScope,
                    metamodelica::cons(cref.clone(), accum),
                );
                continue '__tco;
            }
            _ => return accum,
        }
    }
}

pub fn depth(mut cref: &metamodelica::Ref<NFComponentRef>) -> i32 {
    let mut d: i32 = 0;
    d = (::match_deref::match_deref! { match cref {
        Deref @ CREF { restCref: Deref @ EMPTY { .. }, .. } => d + 1,
        Deref @ CREF { restCref: __cref_restCref, .. } => {
            d = 1 + depth(metamodelica::AsArg::as_arg(&__cref_restCref));
            d
        },
        Deref @ WILD { .. } => 0,
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    d
}

pub fn size(mut cref: &metamodelica::Ref<NFComponentRef>, mut withComplex: bool, mut resize: bool) -> Result<i32> {
    let mut s: i32 = ({
        let mut __acc: i32 = 1;
        for mut i in (sizes(cref, withComplex, resize, metamodelica::nil())?)
            .into_iter()
            .cloned()
        {
            let __x = i.clone();
            __acc *= __x;
        }
        __acc
    });
    Ok(s)
}

pub fn sizes<'__b>(
    mut cref: &'__b metamodelica::Ref<NFComponentRef>,
    mut withComplex: bool,
    mut resize: bool,
    mut s_lst: metamodelica::List<i32>,
) -> Result<metamodelica::List<i32>> {
    '__tco: loop {
        ({
            let mut local_lst: metamodelica::List<i32> = metamodelica::nil();
            match &**cref {
                EMPTY { .. } => return Ok(s_lst.reverse()),
                CREF { .. } => {
                    local_lst = sizes_local(cref, withComplex, resize)?;
                    s_lst = listAppend(local_lst, s_lst);
                    {
                        (cref, withComplex, resize, s_lst) = (
                            var_field!((**cref).restCref, NFComponentRef::CREF),
                            withComplex,
                            resize,
                            s_lst,
                        );
                        continue '__tco;
                    }
                }
                WILD { .. } => return Ok(list![0]),
            }
        })
    }
}

pub(crate) fn sizes_local(
    mut cref: &metamodelica::Ref<NFComponentRef>,
    mut withComplex: bool,
    mut resize: bool,
) -> Result<metamodelica::List<i32>> {
    let mut s_lst: metamodelica::List<i32> = metamodelica::nil();
    let mut complex_size: Option<i32>;
    s_lst = (match &**cref {
        CREF { ty: __cref_ty, .. } => {
            complex_size = Type::complexSize(metamodelica::AsArg::as_arg(&__cref_ty), false)?;
            s_lst = ({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut dim in (Type::arrayDims(__cref_ty.clone())).into_iter().cloned() {
                    let __x = Dimension::size(&(dim.clone()), resize)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            if withComplex && (complex_size).is_some() {
                s_lst = metamodelica::cons(Util::getOption(complex_size)?, s_lst);
            }
            s_lst = if ((s_lst).is_empty()) { list![1] } else { s_lst };
            s_lst
        }
        _ => metamodelica::nil(),
    });
    Ok(s_lst)
}

pub(crate) fn sizes_local_exp(
    mut cref: &metamodelica::Ref<NFComponentRef>,
    mut withComplex: bool,
) -> Result<metamodelica::List<metamodelica::Ref<Expression::NFExpression>>> {
    let mut s_lst: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut complex_size: Option<i32>;
    s_lst = (match &**cref {
        CREF { ty: __cref_ty, .. } => {
            s_lst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
                for mut dim in (Type::arrayDims(__cref_ty.clone())).into_iter().cloned() {
                    let __x = Dimension::sizeExp(&(dim.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            if withComplex {
                complex_size = Type::complexSize(metamodelica::AsArg::as_arg(&__cref_ty), false)?;
                if (complex_size).is_some() {
                    s_lst = metamodelica::cons(
                        metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                            value: Util::getOption(complex_size)?,
                        }),
                        s_lst,
                    );
                }
            }
            s_lst = if ((s_lst).is_empty()) {
                list![metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 })]
            } else {
                s_lst
            };
            s_lst
        }
        _ => metamodelica::nil(),
    });
    Ok(s_lst)
}

pub fn sizeKnown(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<bool> {
    let mut b: bool;
    b = (match &**cref {
        CREF { ty: __cref_ty, .. } => Type::sizeKnown(__cref_ty.clone())?,
        _ => true,
    });
    Ok(b)
}

pub fn subscriptsToInteger(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<metamodelica::List<i32>> {
    let mut s_lst: metamodelica::List<i32> = metamodelica::nil();
    for mut subs_tmp in &*subscriptsAllReverse(cref, metamodelica::nil()) {
        if (subs_tmp).is_empty() {
            s_lst = metamodelica::cons(1, s_lst);
        } else {
            for mut sub in &*subs_tmp.clone() {
                s_lst = metamodelica::cons(
                    Expression::integerValueOrDefault(&(Subscript::toExp(metamodelica::AsArg::as_arg(&sub))?), 1),
                    s_lst,
                );
            }
        }
    }
    Ok(s_lst)
}

pub fn subscriptsToExpression(
    mut cref: &metamodelica::Ref<NFComponentRef>,
    mut addScalar: bool,
) -> Result<metamodelica::List<metamodelica::Ref<Expression::NFExpression>>> {
    let mut e_lst: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> =
        subscriptsToExpression2(cref, addScalar, metamodelica::nil())?.reverse();
    Ok(e_lst)
}

pub(crate) fn subscriptsToExpression2<'__b>(
    mut cref: &'__b metamodelica::Ref<NFComponentRef>,
    mut addScalar: bool,
    mut accum: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::List<metamodelica::Ref<Expression::NFExpression>>> {
    '__tco: loop {
        let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
        let mut local_lst: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
        let mut exp: metamodelica::Ref<Expression::NFExpression>;
        let mut whole_dim: metamodelica::Ref<Dimension::NFDimension>;
        match &**cref {
            CREF { .. } => {
                if addScalar && (var_field!((**cref).subscripts, NFComponentRef::CREF)).is_empty() {
                    local_lst = list![metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 })];
                } else {
                    dims = Type::arrayDims(var_field!((**cref).ty, NFComponentRef::CREF).clone());
                    local_lst = metamodelica::nil();
                    for mut sub in &*var_field!((**cref).subscripts, NFComponentRef::CREF).clone() {
                        exp = (::match_deref::match_deref! { match &((sub.clone(), dims.clone())) {
                            (Deref @ Subscript::WHOLE, Deref @ metamodelica::ListNode::Cons { head: __esc_whole_dim, tail: _ }) => {
                                whole_dim = (*__esc_whole_dim).clone();
                                Expression::makeRange(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }), None, Dimension::sizeExp(metamodelica::AsArg::as_arg(&whole_dim))?)?
                            },
                            _ => Subscript::toExp(metamodelica::AsArg::as_arg(&sub))?,
                            _ => unreachable!("tail-call lowered match: no arm matched"),
                        } });
                        local_lst = metamodelica::cons(exp, local_lst);
                        dims = if ((dims).is_empty()) { dims } else { (dims).rest()? };
                    }
                    local_lst = local_lst.reverse();
                }
                {
                    (cref, addScalar, accum) = (
                        var_field!((**cref).restCref, NFComponentRef::CREF),
                        addScalar,
                        listAppend(local_lst, accum),
                    );
                    continue '__tco;
                }
            }
            _ => return Ok(accum),
        }
    }
}

pub(crate) fn isEmptyArray(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<bool> {
    let mut isEmpty: bool;
    isEmpty = (match &**cref {
        CREF {
            restCref: __cref_restCref,
            ty: __cref_ty,
            ..
        } => {
            Type::isEmptyArray(metamodelica::AsArg::as_arg(&__cref_ty))?
                || isEmptyArray(metamodelica::AsArg::as_arg(&__cref_restCref))?
        }
        _ => false,
    });
    Ok(isEmpty)
}

pub(crate) fn isComplexArray(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<bool> {
    let mut complexArray: bool;
    complexArray = (match &**cref {
        CREF {
            restCref: __cref_restCref,
            ..
        } => isComplexArray2(metamodelica::AsArg::as_arg(&__cref_restCref))?,
        _ => false,
    });
    Ok(complexArray)
}

pub(crate) fn isComplexArray2<'__b>(mut cref: &'__b metamodelica::Ref<NFComponentRef>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match cref {
            Deref @ CREF { ty: Deref @ Type::ARRAY { .. }, .. } if (Type::isArray(&(Type::subscript(var_field!((**cref).ty, NFComponentRef::CREF).clone(), var_field!((**cref).subscripts, NFComponentRef::CREF), true)?))) => return Ok(true),
            Deref @ CREF { .. } => { cref = var_field!((**cref).restCref, NFComponentRef::CREF); continue '__tco; },
            _ => return Ok(false),
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn containsExp(
    mut cref: &metamodelica::Ref<NFComponentRef>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool>,
) -> Result<bool> {
    pub type ContainsPred =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    res = (match &**cref {
        CREF {
            restCref: __cref_restCref,
            subscripts: __cref_subscripts,
            ..
        } => {
            Subscript::listContainsExp(metamodelica::AsArg::as_arg(&__cref_subscripts), func)?
                || containsExp(metamodelica::AsArg::as_arg(&__cref_restCref), func)?
        }
        _ => false,
    });
    Ok(res)
}

pub(crate) fn containsExpShallow(
    mut cref: &metamodelica::Ref<NFComponentRef>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool>,
) -> Result<bool> {
    pub type ContainsPred =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    res = (match &**cref {
        CREF {
            restCref: __cref_restCref,
            subscripts: __cref_subscripts,
            ..
        } => {
            Subscript::listContainsExpShallow(metamodelica::AsArg::as_arg(&__cref_subscripts), func)?
                || containsExpShallow(metamodelica::AsArg::as_arg(&__cref_restCref), func)?
        }
        _ => false,
    });
    Ok(res)
}

pub(crate) fn applyExp(
    mut cref: &metamodelica::Ref<NFComponentRef>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()> + 'static>;

    let () = (match &**cref {
        CREF {
            restCref: __cref_restCref,
            subscripts: __cref_subscripts,
            ..
        } => {
            for mut s in &*__cref_subscripts.clone() {
                Subscript::applyExp(metamodelica::AsArg::as_arg(&s), func)?;
            }
            applyExp(metamodelica::AsArg::as_arg(&__cref_restCref), func)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn applyExpShallow(
    mut cref: &metamodelica::Ref<NFComponentRef>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()> + 'static>;

    let () = (match &**cref {
        CREF {
            restCref: __cref_restCref,
            subscripts: __cref_subscripts,
            ..
        } => {
            for mut s in &*__cref_subscripts.clone() {
                Subscript::applyExpShallow(metamodelica::AsArg::as_arg(&s), func)?;
            }
            applyExpShallow(metamodelica::AsArg::as_arg(&__cref_restCref), func)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub fn mapExp(
    mut cref: &metamodelica::Ref<NFComponentRef>,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut outCref: metamodelica::Ref<NFComponentRef>;
    outCref = (match &**cref {
        CREF {
            node: __cref_node,
            origin: __cref_origin,
            restCref: __cref_restCref,
            subscripts: __cref_subscripts,
            ty: __cref_ty,
        } => {
            let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
            let mut rest: metamodelica::Ref<NFComponentRef>;
            subs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
                for mut s in (__cref_subscripts.clone()).into_iter().cloned() {
                    let __x = Subscript::mapExp(s.clone(), func.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            rest = mapExp(metamodelica::AsArg::as_arg(&__cref_restCref), func.clone())?;
            metamodelica::Ref::new(NFComponentRef::CREF {
                node: __cref_node.clone(),
                subscripts: subs,
                ty: __cref_ty.clone(),
                origin: __cref_origin.clone(),
                restCref: rest,
            })
        }
        _ => cref.clone(),
    });
    Ok(outCref)
}

pub(crate) fn mapExpShallow(
    mut cref: &metamodelica::Ref<NFComponentRef>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut outCref: metamodelica::Ref<NFComponentRef>;
    outCref = (match &**cref {
        CREF {
            node: __cref_node,
            origin: __cref_origin,
            restCref: __cref_restCref,
            subscripts: __cref_subscripts,
            ty: __cref_ty,
        } => {
            let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
            let mut rest: metamodelica::Ref<NFComponentRef>;
            subs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
                for mut s in (__cref_subscripts.clone()).into_iter().cloned() {
                    let __x = Subscript::mapShallowExp(s.clone(), func)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            rest = mapExpShallow(metamodelica::AsArg::as_arg(&__cref_restCref), func)?;
            metamodelica::Ref::new(NFComponentRef::CREF {
                node: __cref_node.clone(),
                subscripts: subs,
                ty: __cref_ty.clone(),
                origin: __cref_origin.clone(),
                restCref: rest,
            })
        }
        _ => cref.clone(),
    });
    Ok(outCref)
}

pub(crate) fn foldExp<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut cref: &metamodelica::Ref<NFComponentRef>,
    mut func: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>,
    mut arg: ArgT,
) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut arg: ArgT = arg;
    let () = (match &**cref {
        CREF {
            restCref: __cref_restCref,
            subscripts: __cref_subscripts,
            ..
        } => {
            arg = List::fold(
                metamodelica::AsArg::as_arg(&__cref_subscripts),
                &({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, _) -> Result<_> + 'static,
                    > = func.clone();
                    move |__pe_a0, __pe_a2| Subscript::foldExp(&__pe_a0, __pe_b1.clone(), __pe_a2)
                }),
                arg,
            )?;
            arg = foldExp(metamodelica::AsArg::as_arg(&__cref_restCref), func.clone(), arg)?;
            ()
        }
        _ => (),
    });
    Ok(arg)
}

pub(crate) fn mapFoldExp<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut cref: &metamodelica::Ref<NFComponentRef>,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Expression::NFExpression>, ArgT)>
            + 'static,
    >,
    mut arg: ArgT,
) -> Result<(metamodelica::Ref<NFComponentRef>, ArgT)> {
    pub type MapFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Expression::NFExpression>, ArgT)>
            + 'static,
    >;

    let mut outCref: metamodelica::Ref<NFComponentRef>;
    let mut arg: ArgT = arg;
    outCref = (match &**cref {
        CREF {
            node: __cref_node,
            origin: __cref_origin,
            restCref: __cref_restCref,
            subscripts: __cref_subscripts,
            ty: __cref_ty,
        } => {
            let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
            let mut rest: metamodelica::Ref<NFComponentRef>;
            (subs, arg) = List::map1Fold(
                metamodelica::AsArg::as_arg(&__cref_subscripts),
                &Subscript::mapFoldExp,
                func.clone(),
                arg,
            )?;
            (rest, arg) = mapFoldExp(metamodelica::AsArg::as_arg(&__cref_restCref), func.clone(), arg)?;
            metamodelica::Ref::new(NFComponentRef::CREF {
                node: __cref_node.clone(),
                subscripts: subs,
                ty: __cref_ty.clone(),
                origin: __cref_origin.clone(),
                restCref: rest,
            })
        }
        _ => cref.clone(),
    });
    Ok((outCref, arg))
}

pub(crate) fn mapFoldExpShallow<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut cref: &metamodelica::Ref<NFComponentRef>,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Expression::NFExpression>, ArgT)>
            + 'static,
    >,
    mut arg: ArgT,
) -> Result<(metamodelica::Ref<NFComponentRef>, ArgT)> {
    pub type MapFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Expression::NFExpression>, ArgT)>
            + 'static,
    >;

    let mut outCref: metamodelica::Ref<NFComponentRef>;
    let mut arg: ArgT = arg;
    outCref = (match &**cref {
        CREF {
            node: __cref_node,
            origin: __cref_origin,
            restCref: __cref_restCref,
            subscripts: __cref_subscripts,
            ty: __cref_ty,
        } => {
            let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
            let mut rest: metamodelica::Ref<NFComponentRef>;
            (subs, arg) = List::map1Fold(
                metamodelica::AsArg::as_arg(&__cref_subscripts),
                &move |__a0: metamodelica::Ref<Subscript::NFSubscript>, __a1: _, __a2: _| {
                    Subscript::mapFoldExpShallow(__a0, metamodelica::arc_ref(&__a1), __a2)
                },
                func.clone(),
                arg,
            )?;
            (rest, arg) = mapFoldExpShallow(metamodelica::AsArg::as_arg(&__cref_restCref), func.clone(), arg)?;
            metamodelica::Ref::new(NFComponentRef::CREF {
                node: __cref_node.clone(),
                subscripts: subs,
                ty: __cref_ty.clone(),
                origin: __cref_origin.clone(),
                restCref: rest,
            })
        }
        _ => cref.clone(),
    });
    Ok((outCref, arg))
}

pub fn isTime(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<bool> {
    let mut b: bool = metamodelica::stringEq(&(firstName(cref, false)?), &(literal!("time")));
    Ok(b)
}

pub(crate) fn isSubstitute(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<bool> {
    let mut b: bool = metamodelica::stringEq(&(firstName(cref, false)?), &(literal!("$SUBST_CREF")));
    Ok(b)
}

pub(crate) fn isDiscrete(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<bool> {
    let mut result: bool = Type::isDiscrete(nodeType(cref)?)?;
    Ok(result)
}

pub(crate) fn removeOuterCrefPrefix(
    mut cref: metamodelica::Ref<NFComponentRef>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    let () = (match &*cref {
        CREF { .. } => {
            if NFInstNode::InstNode::isGeneratedInner(&(node(&cref)?)) {
                assign_variant_field!(cref => NFComponentRef::CREF; restCref = crate::NFComponentRef::interned_EMPTY());
            } else {
                assign_variant_field!(cref => NFComponentRef::CREF; restCref = removeOuterCrefPrefix(var_field!((*cref).restCref, NFComponentRef::CREF).clone())?);
            }
            ()
        }
        _ => (),
    });
    Ok(cref)
}

pub(crate) fn mapTypes(
    mut cref: &metamodelica::Ref<NFComponentRef>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>> + 'static,
    >;

    let mut outCref: metamodelica::Ref<NFComponentRef>;
    outCref = (match &**cref {
        CREF {
            node: __cref_node,
            origin: __cref_origin,
            restCref: __cref_restCref,
            subscripts: __cref_subscripts,
            ty: __cref_ty,
        } => {
            let mut rest: metamodelica::Ref<NFComponentRef>;
            let mut ty: metamodelica::Ref<Type::NFType>;
            ty = func(__cref_ty.clone())?;
            rest = mapTypes(metamodelica::AsArg::as_arg(&__cref_restCref), func)?;
            metamodelica::Ref::new(NFComponentRef::CREF {
                node: __cref_node.clone(),
                subscripts: __cref_subscripts.clone(),
                ty: ty,
                origin: __cref_origin.clone(),
                restCref: rest,
            })
        }
        _ => cref.clone(),
    });
    Ok(outCref)
}

pub(crate) fn mapNodes(
    mut cref: &metamodelica::Ref<NFComponentRef>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<metamodelica::Ref<InstNode::InstNode>>,
) -> Result<metamodelica::Ref<NFComponentRef>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<InstNode::InstNode>) -> Result<metamodelica::Ref<InstNode::InstNode>>
            + 'static,
    >;

    let mut outCref: metamodelica::Ref<NFComponentRef>;
    outCref = (match &**cref {
        CREF {
            origin: __cref_origin,
            restCref: __cref_restCref,
            subscripts: __cref_subscripts,
            ty: __cref_ty,
            ..
        } => {
            let mut rest: metamodelica::Ref<NFComponentRef>;
            let mut node: metamodelica::Ref<InstNode::InstNode>;
            node = func(self::node(cref)?)?;
            rest = mapNodes(metamodelica::AsArg::as_arg(&__cref_restCref), func)?;
            metamodelica::Ref::new(NFComponentRef::CREF {
                node: NFInstNode::InstNode::handle(node)?,
                subscripts: __cref_subscripts.clone(),
                ty: __cref_ty.clone(),
                origin: __cref_origin.clone(),
                restCref: rest,
            })
        }
        _ => cref.clone(),
    });
    Ok(outCref)
}

pub fn getArrayCrefOpt(
    mut scal: &metamodelica::Ref<NFComponentRef>,
) -> Result<Option<metamodelica::Ref<NFComponentRef>>> {
    let mut arr: Option<metamodelica::Ref<NFComponentRef>>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    if Flags::getConfigBool(Flags::SIM_CODE_SCALARIZE.clone())? {
        subs = subscriptsAllFlat(scal)?;
        if (subs).is_empty() {
            arr = None;
        } else if List::all(
            &subs,
            &move |__a0: metamodelica::Ref<Subscript::NFSubscript>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(Subscript::isFirst(&__a0))
            },
        )? {
            arr = Some(stripSubscriptsAll(scal));
        } else {
            arr = None;
        }
    } else {
        arr = if (Type::isArray(&(getSubscriptedType(scal, false)?))) {
            Some(stripSubscriptsAll(scal))
        } else {
            None
        };
    }
    Ok(arr)
}

pub(crate) fn isSliced(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<bool> {
    fn is_sliced_impl(mut cref: &metamodelica::Ref<NFComponentRef>) -> Result<bool> {
        let mut sliced: bool;
        sliced = (match &**cref {
            CREF {
                origin: Origin::CREF { .. },
                restCref: __cref_restCref,
                subscripts: __cref_subscripts,
                ty: __cref_ty,
                ..
            } => {
                sliced = Type::dimensionCount(__cref_ty.clone()) > ((__cref_subscripts).len() as i32)
                    || List::any(
                        metamodelica::AsArg::as_arg(&__cref_subscripts),
                        &move |__a0: metamodelica::Ref<Subscript::NFSubscript>| -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(Subscript::isSliced(&__a0))
                        },
                    )?;
                sliced || is_sliced_impl(metamodelica::AsArg::as_arg(&__cref_restCref))?
            }
            _ => false,
        });
        Ok(sliced)
    }

    let mut sliced: bool;
    sliced = (match &**cref {
        CREF {
            restCref: __cref_restCref,
            ..
        } => is_sliced_impl(metamodelica::AsArg::as_arg(&__cref_restCref))?,
        _ => false,
    });
    Ok(sliced)
}

pub(crate) fn hasImplicitTrailingIndex(mut cref: &metamodelica::Ref<NFComponentRef>) -> bool {
    let mut res: bool;
    res = (match &**cref {
        CREF {
            origin: Origin::CREF { .. },
            subscripts: __cref_subscripts,
            ty: __cref_ty,
            ..
        } => {
            !((__cref_subscripts).is_empty())
                && ((__cref_subscripts).len() as i32) < Type::dimensionCount(__cref_ty.clone())
        }
        _ => false,
    });
    res
}

pub(crate) fn iterate(
    mut cref: metamodelica::Ref<NFComponentRef>,
) -> Result<(
    metamodelica::Ref<NFComponentRef>,
    metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>,
)> {
    fn iterate_impl(
        mut cref: metamodelica::Ref<NFComponentRef>,
        mut iterators: metamodelica::List<(
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::Ref<Expression::NFExpression>,
        )>,
    ) -> Result<(
        metamodelica::Ref<NFComponentRef>,
        metamodelica::List<(
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::Ref<Expression::NFExpression>,
        )>,
    )> {
        let mut cref: metamodelica::Ref<NFComponentRef> = cref;
        let mut iterators: metamodelica::List<(
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::Ref<Expression::NFExpression>,
        )> = iterators;
        let mut rest_cref: metamodelica::Ref<NFComponentRef>;
        let mut dim: metamodelica::Ref<Dimension::NFDimension>;
        let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
        let mut dim_count: i32;
        let mut sub_count: i32;
        let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
        let mut isubs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
        let mut dim_index: i32;
        let mut iterator: metamodelica::Ref<InstNode::InstNode>;
        let mut range: metamodelica::Ref<Expression::NFExpression>;
        let () = (match &*cref {
            CREF {
                origin: Origin::CREF { .. },
                subscripts: __cref_subscripts,
                ty: __cref_ty,
                ..
            } => {
                dims = Type::arrayDims(__cref_ty.clone()).reverse();
                dim_count = ((dims).len() as i32);
                sub_count = ((__cref_subscripts).len() as i32);
                subs = List::consN(
                    dim_count - sub_count,
                    crate::NFSubscript::interned_WHOLE(),
                    __cref_subscripts.clone(),
                );
                isubs = metamodelica::nil();
                dim_index = dim_count;
                for mut s in &*subs.reverse() {
                    let mut s = s.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dims) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    dim = metamodelica::Own::own(__pa0);
                    dims = metamodelica::Own::own(__pa1);
                    if !(Subscript::isIndex(&s)) {
                        range = (match &*s {
                            Subscript::SLICE { slice: __s_slice } => __s_slice.clone(),
                            Subscript::WHOLE => Expression::makeRange(
                                Dimension::lowerBoundExp(&dim)?,
                                None,
                                Dimension::endExp(
                                    &dim,
                                    &(metamodelica::Ref::new(Expression::NFExpression::CREF {
                                        ty: __cref_ty.clone(),
                                        cref: cref.clone(),
                                    })),
                                    dim_index,
                                )?,
                            )?,
                            _ => return Err("match: no arm matched"),
                        });
                        iterator = NFInstNode::InstNode::newUniqueIterator(
                            Absyn::dummyInfo.clone(),
                            crate::NFType::interned_INTEGER(),
                        );
                        iterators = metamodelica::cons((iterator.clone(), range), iterators);
                        dim_index = dim_index - 1;
                        s = metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                            index: Expression::fromCref(
                                makeIterator(iterator, crate::NFType::interned_INTEGER())?,
                                false,
                            )?,
                        });
                    }
                    isubs = metamodelica::cons(s, isubs);
                }
                assign_variant_field!(cref => NFComponentRef::CREF; subscripts = isubs);
                (rest_cref, iterators) =
                    iterate_impl(var_field!((*cref).restCref, NFComponentRef::CREF).clone(), iterators)?;
                assign_variant_field!(cref => NFComponentRef::CREF; restCref = rest_cref);
                ()
            }
            _ => (),
        });
        Ok((cref, iterators))
    }

    let mut cref: metamodelica::Ref<NFComponentRef> = cref;
    let mut iterators: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )>;
    let mut rest_cref: metamodelica::Ref<NFComponentRef>;
    iterators = (match &*cref {
        CREF {
            restCref: __cref_restCref,
            ..
        } => {
            (rest_cref, iterators) = iterate_impl(__cref_restCref.clone(), metamodelica::nil())?;
            if !((iterators).is_empty()) {
                assign_variant_field!(cref => NFComponentRef::CREF; restCref = rest_cref);
                iterators = metamodelica::Dangerous::listReverseInPlace(iterators);
            }
            iterators
        }
        _ => metamodelica::nil(),
    });
    Ok((cref, iterators))
}

pub fn getRecordChildren(
    mut cref: metamodelica::Ref<NFComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<NFComponentRef>>> {
    let mut children: metamodelica::List<metamodelica::Ref<NFComponentRef>> = metamodelica::nil();
    let mut ty: metamodelica::Ref<Type::NFType> = Type::arrayElementType(&(getComponentType(&cref)));
    let mut children_nodes: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>> =
        metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect());
    if Type::isComplex(&ty) {
        children_nodes = (match &*cref {
            CREF { .. } => ClassTree::getComponents(
                &(Class::classTree(NFInstNode::InstNode::getClass(Component::classInstance(
                    &(NFInstNode::InstNode::component(&(node(&cref)?))?),
                )?)?)?),
            )?,
            _ => metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
        });
    }
    if !(children_nodes.clone().borrow().is_empty()) {
        children = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<NFComponentRef>> = metamodelica::nil();
            for mut node in (children_nodes.clone()).borrow().iter() {
                let __x = prefixCref(
                    node.clone(),
                    NFInstNode::InstNode::getType(node.clone())?,
                    metamodelica::nil(),
                    cref.clone(),
                )?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
    }
    Ok(children)
}
