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
use crate::NFCeval as Ceval;
use crate::NFCeval::EvalTarget;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRangeIterator as RangeIterator;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::JSON;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::MutableWeak;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NFSubscript {
    RAW_SUBSCRIPT {
        subscript: metamodelica::Ref<Absyn::Subscript>,
    },
    UNTYPED {
        exp: metamodelica::Ref<Expression::NFExpression>,
    },
    INDEX {
        index: metamodelica::Ref<Expression::NFExpression>,
    },
    SLICE {
        slice: metamodelica::Ref<Expression::NFExpression>,
    },
    EXPANDED_SLICE {
        indices: metamodelica::List<metamodelica::Ref<NFSubscript>>,
    },
    WHOLE,
    SPLIT_PROXY {
        /// Weakly: the class tree owns both of these.
        origin: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
        parent: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
    },
    SPLIT_INDEX {
        /// Weakly: the class tree owns it.
        node: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
        dimIndex: i32,
    },
}
impl metamodelica::gc::MMTrace for NFSubscript {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFSubscript::RAW_SUBSCRIPT { subscript } => {
                metamodelica::gc::MMTrace::mm_accept(subscript, __mmv)?;
                Ok(())
            }
            NFSubscript::UNTYPED { exp } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            NFSubscript::INDEX { index } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                Ok(())
            }
            NFSubscript::SLICE { slice } => {
                metamodelica::gc::MMTrace::mm_accept(slice, __mmv)?;
                Ok(())
            }
            NFSubscript::EXPANDED_SLICE { indices } => {
                metamodelica::gc::MMTrace::mm_accept(indices, __mmv)?;
                Ok(())
            }
            NFSubscript::WHOLE => Ok(()),
            NFSubscript::SPLIT_PROXY { origin, parent } => {
                metamodelica::gc::MMTrace::mm_accept(origin, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(parent, __mmv)?;
                Ok(())
            }
            NFSubscript::SPLIT_INDEX { node, dimIndex } => {
                metamodelica::gc::MMTrace::mm_accept(node, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dimIndex, __mmv)?;
                Ok(())
            }
        }
    }
}
impl NFSubscript {
    pub fn interned_WHOLE() -> metamodelica::Ref<NFSubscript> {
        thread_local! {
            static INTERNED: metamodelica::Ref<NFSubscript> = metamodelica::Ref::new(NFSubscript::WHOLE);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_WHOLE() -> metamodelica::Ref<NFSubscript> {
    NFSubscript::interned_WHOLE()
}
impl Default for NFSubscript {
    fn default() -> Self {
        Self::WHOLE
    }
}
pub use self::NFSubscript::{EXPANDED_SLICE, INDEX, RAW_SUBSCRIPT, SLICE, SPLIT_INDEX, SPLIT_PROXY, UNTYPED, WHOLE};
pub fn fromExp(mut exp: metamodelica::Ref<Expression::NFExpression>) -> metamodelica::Ref<NFSubscript> {
    let mut subscript: metamodelica::Ref<NFSubscript>;
    subscript = (match &*exp {
        Expression::INTEGER { .. } => metamodelica::Ref::new(NFSubscript::INDEX { index: exp }),
        Expression::BOOLEAN { .. } => metamodelica::Ref::new(NFSubscript::INDEX { index: exp }),
        Expression::ENUM_LITERAL { .. } => metamodelica::Ref::new(NFSubscript::INDEX { index: exp }),
        _ => metamodelica::Ref::new(NFSubscript::UNTYPED { exp: exp }),
    });
    subscript
}

pub(crate) fn fromTypedExp(mut exp: metamodelica::Ref<Expression::NFExpression>) -> metamodelica::Ref<NFSubscript> {
    let mut subscript: metamodelica::Ref<NFSubscript>;
    subscript = if (Type::isArray(&(Expression::typeOf(exp.clone())))) {
        metamodelica::Ref::new(NFSubscript::SLICE { slice: exp })
    } else {
        metamodelica::Ref::new(NFSubscript::INDEX { index: exp })
    };
    subscript
}

pub fn toExp(mut subscript: &metamodelica::Ref<NFSubscript>) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (match &**subscript {
        UNTYPED { exp: __subscript_exp } => __subscript_exp.clone(),
        INDEX {
            index: __subscript_index,
        } => __subscript_index.clone(),
        SLICE {
            slice: __subscript_slice,
        } => __subscript_slice.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(exp)
}

pub fn toInteger(mut subscript: &metamodelica::Ref<NFSubscript>) -> Result<i32> {
    let mut int: i32;
    int = (match &**subscript {
        INDEX {
            index: __subscript_index,
        } => Expression::toInteger(metamodelica::AsArg::as_arg(&__subscript_index))?,
        _ => return Err("match: no arm matched"),
    });
    Ok(int)
}

pub(crate) fn toIntegerOpt(mut subscript: &metamodelica::Ref<NFSubscript>) -> Result<Option<i32>> {
    let mut int: Option<i32>;
    int = (match &**subscript {
        INDEX {
            index: __subscript_index,
        } => Some(Expression::toInteger(metamodelica::AsArg::as_arg(&__subscript_index))?),
        _ => None,
    });
    Ok(int)
}

pub fn toIndexList(mut subscript: &metamodelica::Ref<NFSubscript>, mut length: i32) -> Result<metamodelica::List<i32>> {
    let mut indices: metamodelica::List<i32>;
    indices = (::match_deref::match_deref! { match subscript {
        Deref @ INDEX { .. } => {
            list![toInteger(subscript)?]
        },
        Deref @ WHOLE { .. } => {
            List::intRange2(1, length)
        },
        Deref @ SLICE { slice: Deref @ Expression::ARRAY { elements: elems, .. } } => {
            ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut e in (elems.clone()).borrow().iter() {
            let __x = Expression::toInteger(&(e.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        Deref @ SLICE { slice: Deref @ Expression::RANGE { start: Deref @ Expression::INTEGER { value: start }, step: Some(Deref @ Expression::INTEGER { value: step }), stop: Deref @ Expression::INTEGER { value: stop }, .. } } => {
            List::intRange3(start.clone(), step.clone(), stop.clone())?
        },
        Deref @ SLICE { slice: Deref @ Expression::RANGE { start: Deref @ Expression::INTEGER { value: start }, step: None, stop: Deref @ Expression::INTEGER { value: stop }, .. } } => {
            List::intRange2(start.clone(), stop.clone())
        },
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFSubscript.toIndexList")); __mm_s.push_str(&*literal!(" got an incorrect subscript type ")); __mm_s.push_str(&*toString(subscript)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFSubscript.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(indices)
}

fn isValidIndexType(mut ty: &metamodelica::Ref<Type::NFType>) -> Result<bool> {
    let mut b: bool = Type::isInteger(ty)? || Type::isBoolean(ty) || Type::isEnumeration(ty);
    Ok(b)
}

pub(crate) fn makeIndex(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<NFSubscript>> {
    let mut subscript: metamodelica::Ref<NFSubscript>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    ty = Expression::typeOf(exp.clone());
    if isValidIndexType(&ty)? {
        subscript = metamodelica::Ref::new(NFSubscript::INDEX { index: exp });
    } else {
        Error::terminate(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFSubscript.makeIndex"));
                __mm_s.push_str(&*literal!(" got a non integer type exp to make an index sub"));
                ArcStr::from(__mm_s)
            },
            &(metamodelica::sourceInfo!("NFFrontEnd/NFSubscript.mo")),
        )?;
        return Err("fail");
    }
    Ok(subscript)
}

pub(crate) fn makeSplitIndex(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut dimIndex: i32,
) -> Result<metamodelica::Ref<NFSubscript>> {
    let mut subscript: metamodelica::Ref<NFSubscript> = metamodelica::Ref::new(NFSubscript::SPLIT_INDEX {
        node: NFInstNode::InstNode::scopeRef(node.clone()),
        dimIndex: dimIndex,
    });
    if dimIndex < 1 {
        Error::terminate(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFSubscript.makeSplitIndex"));
                __mm_s.push_str(&*literal!(" got invalid index "));
                __mm_s.push_str(&*ArcStr::from(::std::format!("{}", dimIndex)));
                ArcStr::from(__mm_s)
            },
            &(metamodelica::sourceInfo!("NFFrontEnd/NFSubscript.mo")),
        )?;
    }
    Ok(subscript)
}

pub fn isIndex(mut sub: &metamodelica::Ref<NFSubscript>) -> bool {
    let mut isIndex: bool;
    isIndex = (match &**sub {
        INDEX { .. } => true,
        _ => false,
    });
    isIndex
}

pub fn isWhole(mut sub: &metamodelica::Ref<NFSubscript>) -> bool {
    let mut isWhole: bool;
    isWhole = (match &**sub {
        WHOLE { .. } => true,
        _ => false,
    });
    isWhole
}

pub(crate) fn isSimple(mut sub: &metamodelica::Ref<NFSubscript>) -> bool {
    let mut isSimple: bool = isIndex(sub) || isWhole(sub);
    isSimple
}

pub fn isSliced(mut sub: &metamodelica::Ref<NFSubscript>) -> bool {
    let mut sliced: bool;
    sliced = (match &**sub {
        SLICE { .. } => true,
        WHOLE { .. } => true,
        _ => false,
    });
    sliced
}

pub fn isScalar(mut sub: &metamodelica::Ref<NFSubscript>) -> Result<bool> {
    let mut isScalar: bool;
    isScalar = (match &**sub {
        INDEX { index: __sub_index } => {
            let mut ty: metamodelica::Ref<Type::NFType>;
            ty = Expression::typeOf(__sub_index.clone());
            isValidIndexType(&ty)?
        }
        SPLIT_INDEX { .. } => true,
        _ => false,
    });
    Ok(isScalar)
}

pub(crate) fn isScalarLiteral(mut sub: &metamodelica::Ref<NFSubscript>) -> bool {
    let mut isScalarLiteral: bool;
    isScalarLiteral = (match &**sub {
        INDEX { index: __sub_index } => Expression::isScalarLiteral(metamodelica::AsArg::as_arg(&__sub_index)),
        _ => false,
    });
    isScalarLiteral
}

pub(crate) fn equalsIterator(
    mut sub: &metamodelica::Ref<NFSubscript>,
    mut iterator: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<bool> {
    let mut res: bool;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    res = (::match_deref::match_deref! { match sub {
        Deref @ UNTYPED { exp: Deref @ Expression::CREF { cref: __esc_cref, .. } } => {
            cref = (*__esc_cref).clone();
            NFInstNode::InstNode::refEqual(iterator, &(ComponentRef::node(metamodelica::AsArg::as_arg(&cref))?))?
        },
        Deref @ INDEX { index: Deref @ Expression::CREF { cref: __esc_cref, .. } } => {
            cref = (*__esc_cref).clone();
            NFInstNode::InstNode::refEqual(iterator, &(ComponentRef::node(metamodelica::AsArg::as_arg(&cref))?))?
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn isIterator(mut sub: &metamodelica::Ref<NFSubscript>) -> bool {
    let mut res: bool;
    res = (match &**sub {
        UNTYPED { exp: __sub_exp } => Expression::isIterator(metamodelica::AsArg::as_arg(&__sub_exp)),
        INDEX { index: __sub_index } => Expression::isIterator(metamodelica::AsArg::as_arg(&__sub_index)),
        _ => false,
    });
    res
}

pub(crate) fn toIterator(mut sub: &metamodelica::Ref<NFSubscript>) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut iterator: metamodelica::Ref<InstNode::InstNode>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    iterator = (::match_deref::match_deref! { match sub {
        Deref @ UNTYPED { exp: Deref @ Expression::CREF { cref, .. } } if (ComponentRef::isIterator(metamodelica::AsArg::as_arg(&cref))) => ComponentRef::node(metamodelica::AsArg::as_arg(&cref))?,
        Deref @ INDEX { index: Deref @ Expression::CREF { cref, .. } } if (ComponentRef::isIterator(metamodelica::AsArg::as_arg(&cref))) => ComponentRef::node(metamodelica::AsArg::as_arg(&cref))?,
        _ => crate::NFInstNode::InstNode::interned_EMPTY_NODE(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(iterator)
}

pub(crate) fn isBackendIterator(mut sub: &metamodelica::Ref<NFSubscript>) -> bool {
    let mut res: bool;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    res = (::match_deref::match_deref! { match sub {
        Deref @ INDEX { index: Deref @ Expression::CREF { cref: __esc_cref, .. } } => {
            cref = (*__esc_cref).clone();
            ComponentRef::isIterator(metamodelica::AsArg::as_arg(&cref))
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isEqual(
    mut subscript1: &metamodelica::Ref<NFSubscript>,
    mut subscript2: &metamodelica::Ref<NFSubscript>,
) -> Result<bool> {
    let mut isEqual: bool;
    isEqual = (::match_deref::match_deref! { match (subscript1, subscript2) {
        (Deref @ RAW_SUBSCRIPT { .. }, Deref @ RAW_SUBSCRIPT { .. }) => AbsynUtil::subscriptEqual(var_field!((**subscript1).subscript, NFSubscript::RAW_SUBSCRIPT), var_field!((**subscript2).subscript, NFSubscript::RAW_SUBSCRIPT))?,
        (Deref @ UNTYPED { .. }, Deref @ UNTYPED { .. }) => Expression::isEqual(var_field!((**subscript1).exp, NFSubscript::UNTYPED).clone(), var_field!((**subscript2).exp, NFSubscript::UNTYPED).clone())?,
        (Deref @ INDEX { .. }, Deref @ INDEX { .. }) => Expression::isEqual(var_field!((**subscript1).index, NFSubscript::INDEX).clone(), var_field!((**subscript2).index, NFSubscript::INDEX).clone())?,
        (Deref @ SLICE { .. }, Deref @ SLICE { .. }) => Expression::isEqual(var_field!((**subscript1).slice, NFSubscript::SLICE).clone(), var_field!((**subscript2).slice, NFSubscript::SLICE).clone())?,
        (Deref @ WHOLE { .. }, Deref @ WHOLE { .. }) => true,
        (Deref @ SPLIT_INDEX { .. }, Deref @ SPLIT_INDEX { .. }) => var_field!((**subscript1).dimIndex, NFSubscript::SPLIT_INDEX).clone() == var_field!((**subscript2).dimIndex, NFSubscript::SPLIT_INDEX).clone() && NFInstNode::InstNode::refEqual(&(NFInstNode::InstNode::borrow(var_field!((**subscript1).node, NFSubscript::SPLIT_INDEX).clone())?), &(NFInstNode::InstNode::borrow(var_field!((**subscript2).node, NFSubscript::SPLIT_INDEX).clone())?))?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isEqual)
}

pub(crate) fn isEqualList(
    mut subscripts1: &metamodelica::List<metamodelica::Ref<NFSubscript>>,
    mut subscripts2: metamodelica::List<metamodelica::Ref<NFSubscript>>,
) -> Result<bool> {
    let mut isEqual: bool;
    let mut s2: metamodelica::Ref<NFSubscript>;
    let mut rest: metamodelica::List<metamodelica::Ref<NFSubscript>> = subscripts2;
    for mut s1 in &**subscripts1 {
        if (rest).is_empty() {
            isEqual = false;
            return Ok(isEqual);
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        s2 = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        if !(self::isEqual(metamodelica::AsArg::as_arg(&s1), &s2)?) {
            isEqual = false;
            return Ok(isEqual);
        }
    }
    isEqual = (rest).is_empty();
    Ok(isEqual)
}

pub(crate) fn compare(
    mut subscript1: &metamodelica::Ref<NFSubscript>,
    mut subscript2: &metamodelica::Ref<NFSubscript>,
) -> Result<i32> {
    let mut comp: i32;
    if referenceEq(&*(&**subscript1), &*(&**subscript2)) {
        comp = 0;
        return Ok(comp);
    }
    comp = Util::intCompare(
        metamodelica::valueConstructor((&*&**subscript1))?,
        metamodelica::valueConstructor((&*&**subscript2))?,
    );
    if comp != 0 {
        return Ok(comp);
    }
    comp = (match &**subscript1 {
        UNTYPED { exp: __subscript1_exp } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let __pa0 = ::match_deref::match_deref! { match &((*subscript2)) {
                Deref @ UNTYPED { exp: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            Expression::compare(__subscript1_exp.clone(), e)?
        }
        INDEX {
            index: __subscript1_index,
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let __pa0 = ::match_deref::match_deref! { match &((*subscript2)) {
                Deref @ INDEX { index: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            Expression::compare(__subscript1_index.clone(), e)?
        }
        SLICE {
            slice: __subscript1_slice,
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            let __pa0 = ::match_deref::match_deref! { match &((*subscript2)) {
                Deref @ SLICE { slice: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            Expression::compare(__subscript1_slice.clone(), e)?
        }
        WHOLE { .. } => 0,
        SPLIT_INDEX {
            dimIndex: __subscript1_dimIndex,
            node: __subscript1_node,
        } => {
            let mut node: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
            let mut index: i32;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*subscript2)) {
                Deref @ SPLIT_INDEX { node: __pa0, dimIndex: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            node = metamodelica::Own::own(__pa0);
            index = metamodelica::Own::own(__pa1);
            comp = NFInstNode::InstNode::refCompare(
                &(NFInstNode::InstNode::borrow(__subscript1_node.clone())?),
                &(NFInstNode::InstNode::borrow(node)?),
            )?;
            if (comp == 0) {
                Util::intCompare(__subscript1_dimIndex.clone(), index)
            } else {
                comp
            }
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(comp)
}

pub(crate) fn compareList(
    mut subscripts1: &metamodelica::List<metamodelica::Ref<NFSubscript>>,
    mut subscripts2: metamodelica::List<metamodelica::Ref<NFSubscript>>,
) -> Result<i32> {
    let mut comp: i32;
    let mut s2: metamodelica::Ref<NFSubscript>;
    let mut rest_s2: metamodelica::List<metamodelica::Ref<NFSubscript>> = subscripts2.clone();
    comp = Util::intCompare(((subscripts1).len() as i32), ((subscripts2).len() as i32));
    if comp != 0 {
        return Ok(comp);
    }
    for mut s1 in &**subscripts1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_s2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        s2 = metamodelica::Own::own(__pa0);
        rest_s2 = metamodelica::Own::own(__pa1);
        comp = compare(metamodelica::AsArg::as_arg(&s1), &s2)?;
        if comp != 0 {
            return Ok(comp);
        }
    }
    comp = 0;
    Ok(comp)
}

pub(crate) fn containsExp(
    mut subscript: &metamodelica::Ref<NFSubscript>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool>,
) -> Result<bool> {
    pub type ContainsPred =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    res = (match &**subscript {
        UNTYPED { exp: __subscript_exp } => Expression::contains(__subscript_exp.clone(), func)?,
        INDEX {
            index: __subscript_index,
        } => Expression::contains(__subscript_index.clone(), func)?,
        SLICE {
            slice: __subscript_slice,
        } => Expression::contains(__subscript_slice.clone(), func)?,
        _ => false,
    });
    Ok(res)
}

pub(crate) fn listContainsExp(
    mut subscripts: &metamodelica::List<metamodelica::Ref<NFSubscript>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool>,
) -> Result<bool> {
    pub type ContainsPred =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    for mut s in &**subscripts {
        if containsExp(metamodelica::AsArg::as_arg(&s), func)? {
            res = true;
            return Ok(res);
        }
    }
    res = false;
    Ok(res)
}

pub(crate) fn containsExpShallow(
    mut subscript: &metamodelica::Ref<NFSubscript>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool>,
) -> Result<bool> {
    pub type ContainsPred =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    res = (match &**subscript {
        UNTYPED { exp: __subscript_exp } => func(__subscript_exp.clone())?,
        INDEX {
            index: __subscript_index,
        } => func(__subscript_index.clone())?,
        SLICE {
            slice: __subscript_slice,
        } => func(__subscript_slice.clone())?,
        _ => false,
    });
    Ok(res)
}

pub(crate) fn listContainsExpShallow(
    mut subscripts: &metamodelica::List<metamodelica::Ref<NFSubscript>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool>,
) -> Result<bool> {
    pub type ContainsPred =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    for mut s in &**subscripts {
        if containsExpShallow(metamodelica::AsArg::as_arg(&s), func)? {
            res = true;
            return Ok(res);
        }
    }
    res = false;
    Ok(res)
}

pub(crate) fn applyExp(
    mut subscript: &metamodelica::Ref<NFSubscript>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()> + 'static>;

    let () = (match &**subscript {
        UNTYPED { exp: __subscript_exp } => {
            Expression::apply(__subscript_exp.clone(), func)?;
            ()
        }
        INDEX {
            index: __subscript_index,
        } => {
            Expression::apply(__subscript_index.clone(), func)?;
            ()
        }
        SLICE {
            slice: __subscript_slice,
        } => {
            Expression::apply(__subscript_slice.clone(), func)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn applyExpShallow(
    mut subscript: &metamodelica::Ref<NFSubscript>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()> + 'static>;

    let () = (match &**subscript {
        UNTYPED { exp: __subscript_exp } => {
            func(__subscript_exp.clone())?;
            ()
        }
        INDEX {
            index: __subscript_index,
        } => {
            func(__subscript_index.clone())?;
            ()
        }
        SLICE {
            slice: __subscript_slice,
        } => {
            func(__subscript_slice.clone())?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub fn mapExp(
    mut subscript: metamodelica::Ref<NFSubscript>,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >,
) -> Result<metamodelica::Ref<NFSubscript>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut outSubscript: metamodelica::Ref<NFSubscript>;
    outSubscript = (match &*subscript {
        UNTYPED { exp: e1 } => {
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e2 = Expression::map(e1.clone(), func.clone())?;
            if (referenceEq(&*(e1.clone()), &*(&*e2))) {
                subscript
            } else {
                metamodelica::Ref::new(NFSubscript::UNTYPED { exp: e2 })
            }
        }
        INDEX { index: e1 } => {
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e2 = Expression::map(e1.clone(), func.clone())?;
            if (referenceEq(&*(e1.clone()), &*(&*e2))) {
                subscript
            } else {
                fromTypedExp(e2)
            }
        }
        SLICE { slice: e1 } => {
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e2 = Expression::map(e1.clone(), func.clone())?;
            if (referenceEq(&*(e1.clone()), &*(&*e2))) {
                subscript
            } else {
                fromTypedExp(e2)
            }
        }
        _ => subscript,
    });
    Ok(outSubscript)
}

pub(crate) fn mapShallowExp(
    mut subscript: metamodelica::Ref<NFSubscript>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<NFSubscript>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut outSubscript: metamodelica::Ref<NFSubscript>;
    outSubscript = (match &*subscript {
        UNTYPED { exp: e1 } => {
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e2 = func(e1.clone())?;
            if (referenceEq(&*(e1.clone()), &*(&*e2))) {
                subscript
            } else {
                metamodelica::Ref::new(NFSubscript::UNTYPED { exp: e2 })
            }
        }
        INDEX { index: e1 } => {
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e2 = func(e1.clone())?;
            if (referenceEq(&*(e1.clone()), &*(&*e2))) {
                subscript
            } else {
                fromTypedExp(e2)
            }
        }
        SLICE { slice: e1 } => {
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e2 = func(e1.clone())?;
            if (referenceEq(&*(e1.clone()), &*(&*e2))) {
                subscript
            } else {
                fromTypedExp(e2)
            }
        }
        _ => subscript,
    });
    Ok(outSubscript)
}

pub(crate) fn foldExp<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut subscript: &metamodelica::Ref<NFSubscript>,
    mut func: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>,
    mut arg: ArgT,
) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut result: ArgT;
    result = (match &**subscript {
        UNTYPED { exp: __subscript_exp } => Expression::fold(__subscript_exp.clone(), func.clone(), arg)?,
        INDEX {
            index: __subscript_index,
        } => Expression::fold(__subscript_index.clone(), func.clone(), arg)?,
        SLICE {
            slice: __subscript_slice,
        } => Expression::fold(__subscript_slice.clone(), func.clone(), arg)?,
        _ => arg,
    });
    Ok(result)
}

pub(crate) fn mapFoldExp<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut subscript: metamodelica::Ref<NFSubscript>,
    mut func: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Expression::NFExpression>, ArgT)>
            + 'static,
    >,
    mut arg: ArgT,
) -> Result<(metamodelica::Ref<NFSubscript>, ArgT)> {
    pub type MapFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Expression::NFExpression>, ArgT)>
            + 'static,
    >;

    let mut outSubscript: metamodelica::Ref<NFSubscript>;
    let mut arg: ArgT = arg;
    outSubscript = (match &*subscript {
        UNTYPED { exp: __subscript_exp } => {
            let mut exp: metamodelica::Ref<Expression::NFExpression>;
            (exp, arg) = Expression::mapFold(__subscript_exp.clone(), func.clone(), arg)?;
            if (referenceEq(&*(__subscript_exp.clone()), &*(&*exp))) {
                subscript
            } else {
                metamodelica::Ref::new(NFSubscript::UNTYPED { exp: exp })
            }
        }
        INDEX {
            index: __subscript_index,
        } => {
            let mut exp: metamodelica::Ref<Expression::NFExpression>;
            (exp, arg) = Expression::mapFold(__subscript_index.clone(), func.clone(), arg)?;
            if (referenceEq(&*(__subscript_index.clone()), &*(&*exp))) {
                subscript
            } else {
                fromTypedExp(exp)
            }
        }
        SLICE {
            slice: __subscript_slice,
        } => {
            let mut exp: metamodelica::Ref<Expression::NFExpression>;
            (exp, arg) = Expression::mapFold(__subscript_slice.clone(), func.clone(), arg)?;
            if (referenceEq(&*(__subscript_slice.clone()), &*(&*exp))) {
                subscript
            } else {
                fromTypedExp(exp)
            }
        }
        _ => subscript,
    });
    Ok((outSubscript, arg))
}

pub(crate) fn mapFoldExpShallow<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut subscript: metamodelica::Ref<NFSubscript>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
        ArgT,
    ) -> Result<(metamodelica::Ref<Expression::NFExpression>, ArgT)>,
    mut arg: ArgT,
) -> Result<(metamodelica::Ref<NFSubscript>, ArgT)> {
    pub type MapFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                ArgT,
            ) -> Result<(metamodelica::Ref<Expression::NFExpression>, ArgT)>
            + 'static,
    >;

    let mut outSubscript: metamodelica::Ref<NFSubscript>;
    let mut arg: ArgT = arg;
    outSubscript = (match &*subscript {
        UNTYPED { exp: __subscript_exp } => {
            let mut exp: metamodelica::Ref<Expression::NFExpression>;
            (exp, arg) = func(__subscript_exp.clone(), arg)?;
            if (referenceEq(&*(__subscript_exp.clone()), &*(&*exp))) {
                subscript
            } else {
                metamodelica::Ref::new(NFSubscript::UNTYPED { exp: exp })
            }
        }
        INDEX {
            index: __subscript_index,
        } => {
            let mut exp: metamodelica::Ref<Expression::NFExpression>;
            (exp, arg) = func(__subscript_index.clone(), arg)?;
            if (referenceEq(&*(__subscript_index.clone()), &*(&*exp))) {
                subscript
            } else {
                fromTypedExp(exp)
            }
        }
        SLICE {
            slice: __subscript_slice,
        } => {
            let mut exp: metamodelica::Ref<Expression::NFExpression>;
            (exp, arg) = func(__subscript_slice.clone(), arg)?;
            if (referenceEq(&*(__subscript_slice.clone()), &*(&*exp))) {
                subscript
            } else {
                fromTypedExp(exp)
            }
        }
        _ => subscript,
    });
    Ok((outSubscript, arg))
}

pub(crate) fn toAbsyn(mut subscript: &metamodelica::Ref<NFSubscript>) -> Result<metamodelica::Ref<Absyn::Subscript>> {
    let mut asubscript: metamodelica::Ref<Absyn::Subscript>;
    asubscript = (match &**subscript {
        RAW_SUBSCRIPT {
            subscript: __subscript_subscript,
        } => __subscript_subscript.clone(),
        UNTYPED { exp: __subscript_exp } => metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT {
            subscript: Expression::toAbsyn(__subscript_exp.clone())?,
        }),
        INDEX {
            index: __subscript_index,
        } => metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT {
            subscript: Expression::toAbsyn(__subscript_index.clone())?,
        }),
        SLICE {
            slice: __subscript_slice,
        } => metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT {
            subscript: Expression::toAbsyn(__subscript_slice.clone())?,
        }),
        WHOLE { .. } => openmodelica_ast::Absyn::Subscript::interned_NOSUB(),
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFSubscript.toAbsyn"));
                    __mm_s.push_str(&*literal!(" failed on unknown subscript"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFSubscript.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(asubscript)
}

pub(crate) fn toDAE(mut subscript: &metamodelica::Ref<NFSubscript>) -> Result<metamodelica::Ref<DAE::Subscript>> {
    let mut daeSubscript: metamodelica::Ref<DAE::Subscript>;
    daeSubscript = (match &**subscript {
        INDEX {
            index: __subscript_index,
        } => metamodelica::Ref::new(DAE::Subscript::INDEX {
            exp: Expression::toDAE(__subscript_index.clone(), false)?,
        }),
        SLICE {
            slice: __subscript_slice,
        } => metamodelica::Ref::new(DAE::Subscript::SLICE {
            exp: Expression::toDAE(__subscript_slice.clone(), false)?,
        }),
        WHOLE { .. } => openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(),
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFSubscript.toDAE"));
                    __mm_s.push_str(&*literal!(" failed on unknown subscript "));
                    __mm_s.push_str(&*toString(subscript)?);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFSubscript.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(daeSubscript)
}

pub fn toString(mut subscript: &metamodelica::Ref<NFSubscript>) -> Result<ArcStr> {
    let mut string: ArcStr;
    string = (match &**subscript {
        RAW_SUBSCRIPT {
            subscript: __subscript_subscript,
        } => Dump::printSubscriptStr(metamodelica::AsArg::as_arg(&__subscript_subscript))?,
        UNTYPED { exp: __subscript_exp } => Expression::toString(__subscript_exp.clone())?,
        INDEX {
            index: __subscript_index,
        } => Expression::toString(__subscript_index.clone())?,
        SLICE {
            slice: __subscript_slice,
        } => Expression::toString(__subscript_slice.clone())?,
        EXPANDED_SLICE {
            indices: __subscript_indices,
        } => List::toString(
            __subscript_indices.clone(),
            &move |__a0: metamodelica::Ref<NFSubscript>| toString(&__a0),
            List::Style::FLAT_CURLY.clone(),
        )?,
        WHOLE { .. } => literal!(":"),
        SPLIT_PROXY {
            origin: __subscript_origin,
            parent: __subscript_parent,
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("<"));
            __mm_s.push_str(&*NFInstNode::InstNode::name(
                &(NFInstNode::InstNode::borrow(__subscript_origin.clone())?),
            )?);
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*NFInstNode::InstNode::name(
                &(NFInstNode::InstNode::borrow(__subscript_parent.clone())?),
            )?);
            __mm_s.push_str(&*literal!(">"));
            ArcStr::from(__mm_s)
        }
        SPLIT_INDEX {
            dimIndex: __subscript_dimIndex,
            node: __subscript_node,
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("<"));
            __mm_s.push_str(&*NFInstNode::InstNode::name(
                &(NFInstNode::InstNode::borrow(__subscript_node.clone())?),
            )?);
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", __subscript_dimIndex.clone())));
            __mm_s.push_str(&*literal!(">"));
            ArcStr::from(__mm_s)
        }
    });
    Ok(string)
}

pub fn toStringList(mut subscripts: metamodelica::List<metamodelica::Ref<NFSubscript>>) -> Result<ArcStr> {
    let mut string: ArcStr;
    string = List::toStringCustom(
        subscripts,
        &move |__a0: metamodelica::Ref<NFSubscript>| toString(&__a0),
        literal!(""),
        literal!("["),
        literal!(", "),
        literal!("]"),
        false,
        0,
    )?;
    Ok(string)
}

pub(crate) fn toFlatString(
    mut subscript: &metamodelica::Ref<NFSubscript>,
    mut format: BaseModelica::OutputFormat,
) -> Result<ArcStr> {
    let mut string: ArcStr;
    string = (match &**subscript {
        RAW_SUBSCRIPT {
            subscript: __subscript_subscript,
        } => Dump::printSubscriptStr(metamodelica::AsArg::as_arg(&__subscript_subscript))?,
        UNTYPED { exp: __subscript_exp } => Expression::toFlatString(__subscript_exp.clone(), format)?,
        INDEX {
            index: __subscript_index,
        } => Expression::toFlatString(__subscript_index.clone(), format)?,
        SLICE {
            slice: __subscript_slice,
        } => Expression::toFlatString(__subscript_slice.clone(), format)?,
        EXPANDED_SLICE {
            indices: __subscript_indices,
        } => List::toStringCustom(
            __subscript_indices.clone(),
            &move |__a0: metamodelica::Ref<NFSubscript>| toString(&__a0),
            literal!(""),
            literal!("{"),
            literal!(", "),
            literal!("}"),
            false,
            0,
        )?,
        WHOLE { .. } => literal!(":"),
        SPLIT_INDEX {
            dimIndex: __subscript_dimIndex,
            node: __subscript_node,
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("<"));
            __mm_s.push_str(&*NFInstNode::InstNode::name(
                &(NFInstNode::InstNode::borrow(__subscript_node.clone())?),
            )?);
            __mm_s.push_str(&*literal!(", "));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", __subscript_dimIndex.clone())));
            __mm_s.push_str(&*literal!(">"));
            ArcStr::from(__mm_s)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(string)
}

pub(crate) fn toFlatStringList(
    mut subscripts: metamodelica::List<metamodelica::Ref<NFSubscript>>,
    mut format: BaseModelica::OutputFormat,
    mut escapeQuotes: bool,
) -> Result<ArcStr> {
    let mut string: ArcStr;
    string = List::toStringCustom(
        subscripts,
        &({
            let __pe_b1 = format;
            move |__pe_a0| toFlatString(&__pe_a0, __pe_b1.clone())
        }),
        literal!(""),
        literal!("["),
        literal!(","),
        literal!("]"),
        false,
        0,
    )?;
    if escapeQuotes {
        string = Util::escapeQuotes(string)?;
    }
    Ok(string)
}

pub(crate) fn toJSON(mut subscript: &metamodelica::Ref<NFSubscript>) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    json = (match &**subscript {
        UNTYPED { exp: __subscript_exp } => Expression::toJSON(__subscript_exp.clone())?,
        INDEX {
            index: __subscript_index,
        } => Expression::toJSON(__subscript_index.clone())?,
        SLICE {
            slice: __subscript_slice,
        } => Expression::toJSON(__subscript_slice.clone())?,
        _ => JSON::makeString(toString(subscript)?),
    });
    Ok(json)
}

pub(crate) fn toJSONList(
    mut subscripts: &metamodelica::List<metamodelica::Ref<NFSubscript>>,
) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON> = JSON::makeNull();
    for mut s in &**subscripts {
        json = JSON::addElement(&(toJSON(metamodelica::AsArg::as_arg(&s))?), json)?;
    }
    Ok(json)
}

pub(crate) fn eval(
    mut subscript: metamodelica::Ref<NFSubscript>,
    mut target: &metamodelica::Ref<EvalTarget::EvalTarget>,
) -> Result<metamodelica::Ref<NFSubscript>> {
    let mut outSubscript: metamodelica::Ref<NFSubscript>;
    outSubscript = (match &*subscript {
        INDEX {
            index: __subscript_index,
        } => metamodelica::Ref::new(NFSubscript::INDEX {
            index: Ceval::evalExp(__subscript_index.clone(), target)?,
        }),
        SLICE {
            slice: __subscript_slice,
        } => metamodelica::Ref::new(NFSubscript::SLICE {
            slice: Ceval::evalExp(__subscript_slice.clone(), target)?,
        }),
        _ => subscript,
    });
    Ok(outSubscript)
}

pub(crate) fn simplify(
    mut subscript: metamodelica::Ref<NFSubscript>,
    mut dimension: &metamodelica::Ref<Dimension::NFDimension>,
) -> Result<metamodelica::Ref<NFSubscript>> {
    let mut outSubscript: metamodelica::Ref<NFSubscript>;
    outSubscript = (match &*subscript {
        INDEX {
            index: __subscript_index,
        } => metamodelica::Ref::new(NFSubscript::INDEX {
            index: SimplifyExp::simplify(__subscript_index.clone(), false)?,
        }),
        SLICE {
            slice: __subscript_slice,
        } => simplifySlice(__subscript_slice.clone(), dimension)?,
        _ => subscript,
    });
    Ok(outSubscript)
}

pub(crate) fn simplifySlice(
    mut slice: metamodelica::Ref<Expression::NFExpression>,
    mut dimension: &metamodelica::Ref<Dimension::NFDimension>,
) -> Result<metamodelica::Ref<NFSubscript>> {
    let mut outSubscript: metamodelica::Ref<NFSubscript>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = SimplifyExp::simplify(slice, false)?;
    outSubscript = (match &*exp {
        Expression::RANGE {
            start: __exp_start,
            step: __exp_step,
            stop: __exp_stop,
            ..
        } if (((__exp_step).is_none() || Expression::isOne(&(Util::getOption(__exp_step.clone())?))?)
            && Dimension::expIsLowerBound(metamodelica::AsArg::as_arg(&__exp_start))
            && Dimension::expIsUpperBound(metamodelica::AsArg::as_arg(&__exp_stop), dimension)) =>
        {
            crate::NFSubscript::interned_WHOLE()
        }
        _ => metamodelica::Ref::new(NFSubscript::SLICE { slice: exp }),
    });
    Ok(outSubscript)
}

pub(crate) fn simplifyList(
    mut subscripts: metamodelica::List<metamodelica::Ref<NFSubscript>>,
    mut dimensions: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
    mut trim: bool,
) -> Result<metamodelica::List<metamodelica::Ref<NFSubscript>>> {
    let mut outSubscripts: metamodelica::List<metamodelica::Ref<NFSubscript>> = metamodelica::nil();
    let mut d: metamodelica::Ref<Dimension::NFDimension>;
    let mut rest_d: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = dimensions.clone();
    if (dimensions).is_empty() {
        outSubscripts = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<NFSubscript>> = metamodelica::nil();
            for mut s in (subscripts).into_iter().cloned() {
                let __x = simplify(s.clone(), &(crate::NFDimension::interned_UNKNOWN()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
    } else {
        for mut s in &*subscripts {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_d) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            d = metamodelica::Own::own(__pa0);
            rest_d = metamodelica::Own::own(__pa1);
            outSubscripts = metamodelica::cons(simplify(s.clone(), &d)?, outSubscripts);
        }
        if trim {
            outSubscripts = metamodelica::Dangerous::listReverseInPlace(List::trim(
                outSubscripts,
                &move |__a0: metamodelica::Ref<NFSubscript>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(isWhole(&__a0))
                },
            )?);
        } else {
            outSubscripts = metamodelica::Dangerous::listReverseInPlace(outSubscripts);
        }
    }
    Ok(outSubscripts)
}

pub(crate) fn toDimension(
    mut subscript: &metamodelica::Ref<NFSubscript>,
) -> Result<metamodelica::Ref<Dimension::NFDimension>> {
    let mut dimension: metamodelica::Ref<Dimension::NFDimension>;
    dimension = (match &**subscript {
        INDEX { .. } => Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone()),
        SLICE {
            slice: __subscript_slice,
        } => (Type::arrayDims(Expression::typeOf(__subscript_slice.clone())))
            .head()
            .cloned()?,
        WHOLE { .. } => crate::NFDimension::interned_UNKNOWN(),
        SPLIT_INDEX { .. } => Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone()),
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFSubscript.toDimension"));
                    __mm_s.push_str(&*literal!(" got wrong subscript "));
                    __mm_s.push_str(&*toString(subscript)?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFSubscript.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(dimension)
}

pub(crate) fn fromDimension(
    mut dimension: &metamodelica::Ref<Dimension::NFDimension>,
) -> Result<metamodelica::Ref<NFSubscript>> {
    let mut subscript: metamodelica::Ref<NFSubscript>;
    subscript = (match &**dimension {
        Dimension::INTEGER {
            size: __dimension_size, ..
        } => metamodelica::Ref::new(NFSubscript::SLICE {
            slice: Expression::makeIntegerRange(1, 1, __dimension_size.clone())?,
        }),
        Dimension::BOOLEAN => metamodelica::Ref::new(NFSubscript::SLICE {
            slice: Expression::makeRange(
                metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }),
                None,
                metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true }),
            )?,
        }),
        Dimension::ENUM {
            enumType: __dimension_enumType,
        } => metamodelica::Ref::new(NFSubscript::SLICE {
            slice: Expression::makeRange(
                Expression::makeEnumLiteral(__dimension_enumType.clone(), 1)?,
                None,
                Expression::makeEnumLiteral(
                    __dimension_enumType.clone(),
                    Type::enumSize(metamodelica::AsArg::as_arg(&__dimension_enumType))?,
                )?,
            )?,
        }),
        Dimension::EXP {
            exp: __dimension_exp, ..
        } => metamodelica::Ref::new(NFSubscript::SLICE {
            slice: Expression::makeRange(
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }),
                None,
                __dimension_exp.clone(),
            )?,
        }),
        Dimension::RESIZABLE {
            exp: __dimension_exp, ..
        } => metamodelica::Ref::new(NFSubscript::SLICE {
            slice: Expression::makeRange(
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }),
                None,
                __dimension_exp.clone(),
            )?,
        }),
        _ => return Err("match: no arm matched"),
    });
    Ok(subscript)
}

pub fn scalarize(
    mut subscript: metamodelica::Ref<NFSubscript>,
    mut dimension: &metamodelica::Ref<Dimension::NFDimension>,
    mut resize: bool,
) -> Result<metamodelica::List<metamodelica::Ref<NFSubscript>>> {
    let mut subscripts: metamodelica::List<metamodelica::Ref<NFSubscript>>;
    subscripts = (match &*subscript {
        INDEX { .. } => list![subscript],
        SLICE {
            slice: __subscript_slice,
        } => {
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<NFSubscript>> = metamodelica::nil();
                for mut e in
                    (Expression::arrayElements(&((ExpandExp::expand(__subscript_slice.clone(), resize, false)?).0))?)
                        .borrow()
                        .iter()
                {
                    let __x = metamodelica::Ref::new(NFSubscript::INDEX { index: e.clone() });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        }
        WHOLE { .. } => RangeIterator::map(RangeIterator::fromDim(dimension, resize)?, &makeIndex)?,
        _ => list![subscript],
    });
    Ok(subscripts)
}

pub fn scalarizeList(
    mut subscripts: &metamodelica::List<metamodelica::Ref<NFSubscript>>,
    mut dimensions: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
    mut resize: bool,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<NFSubscript>>>> {
    let mut outSubscripts: metamodelica::List<metamodelica::List<metamodelica::Ref<NFSubscript>>> = metamodelica::nil();
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    let mut rest_dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = dimensions;
    let mut subs: metamodelica::List<metamodelica::Ref<NFSubscript>>;
    for mut s in &**subscripts {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_dims) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        dim = metamodelica::Own::own(__pa0);
        rest_dims = metamodelica::Own::own(__pa1);
        subs = scalarize(s.clone(), &dim, resize)?;
        if (subs).is_empty() {
            outSubscripts = metamodelica::nil();
            return Ok(outSubscripts);
        } else {
            outSubscripts = metamodelica::cons(subs, outSubscripts);
        }
    }
    for mut d in &*rest_dims {
        subs = RangeIterator::map(
            RangeIterator::fromDim(metamodelica::AsArg::as_arg(&d), resize)?,
            &makeIndex,
        )?;
        if (subs).is_empty() {
            outSubscripts = metamodelica::nil();
            return Ok(outSubscripts);
        } else {
            outSubscripts = metamodelica::cons(subs, outSubscripts);
        }
    }
    outSubscripts = outSubscripts.reverse();
    Ok(outSubscripts)
}

pub(crate) fn expand(
    mut subscript: metamodelica::Ref<NFSubscript>,
    mut dimension: &metamodelica::Ref<Dimension::NFDimension>,
    mut resize: bool,
) -> Result<(metamodelica::Ref<NFSubscript>, bool)> {
    let mut outSubscript: metamodelica::Ref<NFSubscript>;
    let mut expanded: bool;
    (outSubscript, expanded) = (match &*subscript {
        SLICE { .. } => expandSlice(subscript, resize)?,
        WHOLE { .. } => {
            let mut iter: metamodelica::Ref<RangeIterator::NFRangeIterator>;
            iter = RangeIterator::fromDim(dimension, resize)?;
            if RangeIterator::isValid(&iter) {
                outSubscript = metamodelica::Ref::new(NFSubscript::EXPANDED_SLICE {
                    indices: RangeIterator::map(iter, &makeIndex)?,
                });
                expanded = true;
            } else {
                outSubscript = subscript;
                expanded = false;
            }
            (outSubscript, expanded)
        }
        _ => (subscript, true),
    });
    Ok((outSubscript, expanded))
}

pub(crate) fn expandSlice(
    mut subscript: metamodelica::Ref<NFSubscript>,
    mut resize: bool,
) -> Result<(metamodelica::Ref<NFSubscript>, bool)> {
    let mut outSubscript: metamodelica::Ref<NFSubscript>;
    let mut expanded: bool;
    (outSubscript, expanded) = (match &*subscript {
        SLICE {
            slice: __subscript_slice,
        } => {
            let mut exp: metamodelica::Ref<Expression::NFExpression>;
            (exp, _) = ExpandExp::expand(SimplifyExp::simplify(__subscript_slice.clone(), false)?, resize, false)?;
            if Expression::isArray(&exp) {
                outSubscript = metamodelica::Ref::new(NFSubscript::EXPANDED_SLICE {
                    indices: ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<NFSubscript>> = metamodelica::nil();
                        for mut e in (Expression::arrayElements(&exp)?).borrow().iter() {
                            let __x = metamodelica::Ref::new(NFSubscript::INDEX { index: e.clone() });
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                });
                expanded = true;
            } else {
                outSubscript = subscript;
                expanded = false;
            }
            (outSubscript, expanded)
        }
        _ => (subscript, false),
    });
    Ok((outSubscript, expanded))
}

pub(crate) fn expandList(
    mut subscripts: &metamodelica::List<metamodelica::Ref<NFSubscript>>,
    mut dimensions: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
    mut resize: bool,
) -> Result<metamodelica::List<metamodelica::Ref<NFSubscript>>> {
    let mut outSubscripts: metamodelica::List<metamodelica::Ref<NFSubscript>> = metamodelica::nil();
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    let mut rest_dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = dimensions;
    let mut sub: metamodelica::Ref<NFSubscript>;
    for mut s in &**subscripts {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_dims) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        dim = metamodelica::Own::own(__pa0);
        rest_dims = metamodelica::Own::own(__pa1);
        (sub, _) = expand(s.clone(), &dim, resize)?;
        outSubscripts = metamodelica::cons(sub, outSubscripts);
    }
    for mut d in &*rest_dims {
        sub = metamodelica::Ref::new(NFSubscript::EXPANDED_SLICE {
            indices: RangeIterator::map(
                RangeIterator::fromDim(metamodelica::AsArg::as_arg(&d), resize)?,
                &makeIndex,
            )?,
        });
        outSubscripts = metamodelica::cons(sub, outSubscripts);
    }
    outSubscripts = outSubscripts.reverse();
    Ok(outSubscripts)
}

pub(crate) fn variability(mut subscript: &metamodelica::Ref<NFSubscript>) -> Result<Variability> {
    let mut var: Variability;
    var = (match &**subscript {
        UNTYPED { exp: __subscript_exp } => Expression::variability(__subscript_exp.clone())?,
        INDEX {
            index: __subscript_index,
        } => Expression::variability(__subscript_index.clone())?,
        SLICE {
            slice: __subscript_slice,
        } => Expression::variability(__subscript_slice.clone())?,
        _ => Variability::CONSTANT.clone(),
    });
    Ok(var)
}

pub(crate) fn variabilityList(
    mut subscripts: &metamodelica::List<metamodelica::Ref<NFSubscript>>,
) -> Result<Variability> {
    let mut var: Variability = Variability::CONSTANT.clone();
    for mut s in &**subscripts {
        var = Prefixes::variabilityMax(var, variability(metamodelica::AsArg::as_arg(&s))?);
    }
    Ok(var)
}

pub(crate) fn purity(mut subscript: &metamodelica::Ref<NFSubscript>) -> Result<Purity> {
    let mut purity: Purity;
    purity = (match &**subscript {
        UNTYPED { exp: __subscript_exp } => Expression::purity(__subscript_exp.clone())?,
        INDEX {
            index: __subscript_index,
        } => Expression::purity(__subscript_index.clone())?,
        SLICE {
            slice: __subscript_slice,
        } => Expression::purity(__subscript_slice.clone())?,
        _ => Purity::IMPURE.clone(),
    });
    Ok(purity)
}

pub(crate) fn purityList(mut subscripts: &metamodelica::List<metamodelica::Ref<NFSubscript>>) -> Result<Purity> {
    let mut pur: Purity = Purity::PURE.clone();
    for mut s in &**subscripts {
        pur = Prefixes::purityMin(pur, purity(metamodelica::AsArg::as_arg(&s))?);
    }
    Ok(pur)
}

pub(crate) fn mergeList(
    mut newSubs: metamodelica::List<metamodelica::Ref<NFSubscript>>,
    mut oldSubs: metamodelica::List<metamodelica::Ref<NFSubscript>>,
    mut dimensions: i32,
    mut backend: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<NFSubscript>>,
    metamodelica::List<metamodelica::Ref<NFSubscript>>,
)> {
    let mut outSubs: metamodelica::List<metamodelica::Ref<NFSubscript>>;
    let mut remainingSubs: metamodelica::List<metamodelica::Ref<NFSubscript>>;
    let mut subs_count: i32;
    let mut new_sub: metamodelica::Ref<NFSubscript>;
    let mut old_sub: metamodelica::Ref<NFSubscript>;
    let mut rest_old_subs: metamodelica::List<metamodelica::Ref<NFSubscript>>;
    let mut merged: bool = true;
    if backend
        && ((oldSubs).len() as i32) >= dimensions
        && List::all(
            &(List::firstN(oldSubs.clone(), dimensions)?),
            &move |__a0: metamodelica::Ref<NFSubscript>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isBackendIterator(&__a0))
            },
        )?
    {
        (_, remainingSubs) = List::split(newSubs, dimensions)?;
        (outSubs, _) = List::split(oldSubs, dimensions)?;
        return Ok((outSubs, remainingSubs));
    }
    if (oldSubs).is_empty() {
        if ((newSubs).len() as i32) <= dimensions {
            outSubs = newSubs;
            remainingSubs = metamodelica::nil();
        } else {
            (outSubs, remainingSubs) = List::split(newSubs, dimensions)?;
        }
        return Ok((outSubs, remainingSubs));
    }
    subs_count = ((oldSubs).len() as i32);
    remainingSubs = newSubs;
    rest_old_subs = oldSubs;
    outSubs = metamodelica::nil();
    while merged && !((remainingSubs).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(remainingSubs) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        new_sub = metamodelica::Own::own(__pa0);
        remainingSubs = metamodelica::Own::own(__pa1);
        merged = false;
        while !(merged) {
            if (rest_old_subs).is_empty() {
                remainingSubs = metamodelica::cons(new_sub, remainingSubs);
                break;
            } else {
                let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_old_subs) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                old_sub = metamodelica::Own::own(__pa2);
                rest_old_subs = metamodelica::Own::own(__pa3);
                (merged, outSubs) = (match &*old_sub.clone() {
                    SLICE { slice: __old_sub_slice } => {
                        if !(isWhole(&new_sub)) {
                            outSubs = metamodelica::cons(
                                metamodelica::Ref::new(NFSubscript::INDEX {
                                    index: Expression::applySubscript(
                                        &new_sub,
                                        metamodelica::AsArg::as_arg(&__old_sub_slice),
                                        &(metamodelica::nil()),
                                        false,
                                    )?,
                                }),
                                outSubs,
                            );
                        } else {
                            outSubs = metamodelica::cons(old_sub, outSubs);
                        }
                        (true, outSubs)
                    }
                    WHOLE { .. } => (true, metamodelica::cons(new_sub.clone(), outSubs)),
                    _ => (false, metamodelica::cons(old_sub, outSubs)),
                });
            }
        }
    }
    for mut s in &*rest_old_subs {
        outSubs = metamodelica::cons(s.clone(), outSubs);
    }
    while !((remainingSubs).is_empty()) && subs_count < dimensions {
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(remainingSubs) {
            Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => return Err("pattern mismatch"),
        } };
        new_sub = metamodelica::Own::own(__pa4);
        remainingSubs = metamodelica::Own::own(__pa5);
        outSubs = metamodelica::cons(new_sub, outSubs);
        subs_count = subs_count + 1;
    }
    outSubs = metamodelica::Dangerous::listReverseInPlace(outSubs);
    Ok((outSubs, remainingSubs))
}

pub fn nth(mut dim: &metamodelica::Ref<Dimension::NFDimension>, mut i: i32) -> Result<metamodelica::Ref<NFSubscript>> {
    let mut sub: metamodelica::Ref<NFSubscript>;
    sub = (match &**dim {
        Dimension::INTEGER { .. } => metamodelica::Ref::new(NFSubscript::INDEX {
            index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i }),
        }),
        Dimension::BOOLEAN if (i == 1) => metamodelica::Ref::new(NFSubscript::INDEX {
            index: metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }),
        }),
        Dimension::BOOLEAN if (i == 2) => metamodelica::Ref::new(NFSubscript::INDEX {
            index: metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true }),
        }),
        Dimension::ENUM {
            enumType: __dim_enumType,
        } => metamodelica::Ref::new(NFSubscript::INDEX {
            index: Expression::nthEnumLiteral(__dim_enumType.clone(), i)?,
        }),
        Dimension::RESIZABLE { .. } => metamodelica::Ref::new(NFSubscript::INDEX {
            index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i }),
        }),
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFSubscript.nth"));
                    __mm_s.push_str(&*literal!(" got an incorrect dimension type "));
                    __mm_s.push_str(&*Dimension::toString(dim)?);
                    __mm_s.push_str(&*literal!("."));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFSubscript.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(sub)
}

pub(crate) fn first(mut dim: &metamodelica::Ref<Dimension::NFDimension>) -> Result<metamodelica::Ref<NFSubscript>> {
    let mut sub: metamodelica::Ref<NFSubscript>;
    sub = (match &**dim {
        Dimension::INTEGER { .. } => metamodelica::Ref::new(NFSubscript::INDEX {
            index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }),
        }),
        Dimension::BOOLEAN => metamodelica::Ref::new(NFSubscript::INDEX {
            index: metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }),
        }),
        Dimension::ENUM {
            enumType: __dim_enumType,
        } => metamodelica::Ref::new(NFSubscript::INDEX {
            index: Expression::nthEnumLiteral(__dim_enumType.clone(), 1)?,
        }),
        Dimension::RESIZABLE { .. } => metamodelica::Ref::new(NFSubscript::INDEX {
            index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }),
        }),
        _ => return Err("match: no arm matched"),
    });
    Ok(sub)
}

pub(crate) fn isFirst(mut sub: &metamodelica::Ref<NFSubscript>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match sub {
        Deref @ INDEX { index: Deref @ Expression::INTEGER { value: 1 } } => true,
        Deref @ INDEX { index: Deref @ Expression::BOOLEAN { value: false } } => true,
        Deref @ INDEX { index: Deref @ Expression::ENUM_LITERAL { index: 1, .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isSplit(mut sub: &metamodelica::Ref<NFSubscript>) -> bool {
    let mut res: bool;
    res = (match &**sub {
        SPLIT_PROXY { .. } => true,
        SPLIT_INDEX { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn isSplitIndex(mut sub: &metamodelica::Ref<NFSubscript>) -> bool {
    let mut res: bool;
    res = (match &**sub {
        SPLIT_INDEX { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn isSplitClassProxy(mut sub: &metamodelica::Ref<NFSubscript>) -> Result<bool> {
    let mut res: bool;
    res = (match &**sub {
        SPLIT_PROXY {
            origin: __sub_origin, ..
        } => NFInstNode::InstNode::isClass(&(NFInstNode::InstNode::borrow(__sub_origin.clone())?))?,
        _ => false,
    });
    Ok(res)
}

pub(crate) fn isSplitFromOrigin(
    mut sub: &metamodelica::Ref<NFSubscript>,
    mut origin: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<bool> {
    let mut res: bool;
    res = (match &**sub {
        SPLIT_PROXY {
            origin: __sub_origin, ..
        } => NFInstNode::InstNode::refEqual(origin, &(NFInstNode::InstNode::borrow(__sub_origin.clone())?))?,
        _ => false,
    });
    Ok(res)
}

pub(crate) fn expandSplitIndices(
    mut subs: metamodelica::List<metamodelica::Ref<NFSubscript>>,
    mut indicesToKeep: &metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
) -> Result<metamodelica::List<metamodelica::Ref<NFSubscript>>> {
    let mut outSubs: metamodelica::List<metamodelica::Ref<NFSubscript>> = metamodelica::nil();
    let mut changed: bool = false;
    for mut s in &*subs {
        let () = (match &*s.clone() {
            SPLIT_INDEX { node: __s_node, .. } => {
                if List::isMemberOnTrue(
                    NFInstNode::InstNode::borrow(__s_node.clone())?,
                    indicesToKeep,
                    &move |__a0: metamodelica::Ref<InstNode::InstNode>, __a1: metamodelica::Ref<InstNode::InstNode>| {
                        NFInstNode::InstNode::refEqual(&__a0, &__a1)
                    },
                )? {
                    outSubs = metamodelica::cons(s.clone(), outSubs);
                } else {
                    outSubs = metamodelica::cons(crate::NFSubscript::interned_WHOLE(), outSubs);
                    changed = true;
                }
                ()
            }
            _ => {
                outSubs = metamodelica::cons(s.clone(), outSubs);
                ()
            }
        });
    }
    if changed {
        outSubs = List::trim(
            outSubs,
            &move |__a0: metamodelica::Ref<NFSubscript>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isWhole(&__a0))
            },
        )?;
        outSubs = metamodelica::Dangerous::listReverseInPlace(outSubs);
    } else {
        outSubs = subs;
    }
    Ok(outSubs)
}

pub(crate) fn hash(mut sub: &metamodelica::Ref<NFSubscript>) -> Result<i32> {
    let mut hash: i32 = hashContinue(sub, Util::HASH_SEED.clone())?;
    Ok(hash)
}

pub(crate) fn hashStringContinue(mut sub: &metamodelica::Ref<NFSubscript>, mut hash: i32) -> Result<i32> {
    let mut hash: i32 = hash;
    hash = (::match_deref::match_deref! { match sub {
        Deref @ INDEX { index: Deref @ Expression::INTEGER { value: i } } => {
            intHashDjb2Continue(i.clone(), hash)
        },
        _ => {
            stringHashDjb2Continue(&(toString(sub)?), hash)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(hash)
}

pub(crate) fn hashContinue(mut sub: &metamodelica::Ref<NFSubscript>, mut hash: i32) -> Result<i32> {
    let mut hash: i32 = hash;
    hash = (match &**sub {
        RAW_SUBSCRIPT {
            subscript: __sub_subscript,
        } => stringHashDjb2Continue(
            &(Dump::printSubscriptStr(metamodelica::AsArg::as_arg(&__sub_subscript))?),
            hash,
        ),
        UNTYPED { exp: __sub_exp } => Expression::hashContinue(__sub_exp.clone(), hash)?,
        INDEX { index: __sub_index } => Expression::hashContinue(__sub_index.clone(), hash)?,
        SLICE { slice: __sub_slice } => Expression::hashContinue(__sub_slice.clone(), hash)?,
        EXPANDED_SLICE { indices: __sub_indices } => {
            hash = stringHashDjb2Continue(&(literal!("{")), hash);
            for mut s in &*__sub_indices.clone() {
                hash = hashContinue(metamodelica::AsArg::as_arg(&s), hash)?;
                hash = stringHashDjb2Continue(&(literal!(", ")), hash);
            }
            hash = stringHashDjb2Continue(&(literal!("}")), hash);
            hash
        }
        WHOLE { .. } => stringHashDjb2Continue(&(literal!(":")), hash),
        SPLIT_PROXY {
            origin: __sub_origin,
            parent: __sub_parent,
        } => {
            hash = NFInstNode::InstNode::hashContinue(&(NFInstNode::InstNode::borrow(__sub_origin.clone())?), hash)?;
            hash = NFInstNode::InstNode::hashContinue(&(NFInstNode::InstNode::borrow(__sub_parent.clone())?), hash)?;
            hash
        }
        SPLIT_INDEX {
            dimIndex: __sub_dimIndex,
            node: __sub_node,
        } => {
            hash = NFInstNode::InstNode::hashContinue(&(NFInstNode::InstNode::borrow(__sub_node.clone())?), hash)?;
            hash = stringHashDjb2Continue(&(intString(__sub_dimIndex.clone())), hash);
            hash
        }
        _ => hash,
    });
    Ok(hash)
}

pub(crate) fn splitIndexDimExp(
    mut sub: &metamodelica::Ref<NFSubscript>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut node: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let mut index: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*sub)) {
        Deref @ SPLIT_INDEX { node: __pa0, dimIndex: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    node = metamodelica::Own::own(__pa0);
    index = metamodelica::Own::own(__pa1);
    exp = Dimension::sizeExp(
        &(Type::nthDimension(
            NFInstNode::InstNode::getType(NFInstNode::InstNode::borrow(node)?)?,
            index,
        )?),
    )?;
    Ok(exp)
}

pub fn isLiteral(mut sub: &metamodelica::Ref<NFSubscript>) -> Result<bool> {
    let mut literal: bool;
    literal = (match &**sub {
        UNTYPED { exp: __sub_exp } => Expression::isLiteral(metamodelica::AsArg::as_arg(&__sub_exp))?,
        INDEX { index: __sub_index } => Expression::isLiteral(metamodelica::AsArg::as_arg(&__sub_index))?,
        SLICE { slice: __sub_slice } => Expression::isLiteral(metamodelica::AsArg::as_arg(&__sub_slice))?,
        WHOLE { .. } => true,
        _ => false,
    });
    Ok(literal)
}

pub fn fillWithWholeLeft(
    mut subs: metamodelica::List<metamodelica::Ref<NFSubscript>>,
    mut targetLength: i32,
) -> metamodelica::List<metamodelica::Ref<NFSubscript>> {
    let mut subs: metamodelica::List<metamodelica::Ref<NFSubscript>> = subs;
    subs = listAppend(
        List::fill(
            crate::NFSubscript::interned_WHOLE(),
            targetLength - ((subs).len() as i32),
        ),
        subs,
    );
    subs
}
