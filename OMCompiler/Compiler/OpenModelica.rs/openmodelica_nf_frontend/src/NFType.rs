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
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFComplexType as ComplexType;
use crate::NFDimension as Dimension;
use crate::NFFunction::Function;
use crate::NFInstContext;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes;
use crate::NFRecord as Record;
use crate::NFSubscript as Subscript;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::IOStream;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::MutableWeak;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NFType {
    INTEGER,
    REAL,
    STRING,
    BOOLEAN,
    CLOCK,
    ENUMERATION {
        typePath: metamodelica::Ref<Absyn::Path>,
        literals: metamodelica::List<ArcStr>,
    },
    __ENUMERATION_ANY_NOT_USED__,
    ARRAY {
        elementType: metamodelica::Ref<NFType>,
        dimensions: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
    },
    TUPLE {
        types: metamodelica::List<metamodelica::Ref<NFType>>,
        names: Option<metamodelica::List<ArcStr>>,
    },
    NORETCALL,
    UNKNOWN,
    COMPLEX {
        /// The class this type names, weakly:
        ///      a class's own type names it back, and that is a cycle. Owned by whatever
        ///      the class hangs off -- a component's `classInst`, a `Function`, a cref.
        cls: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
        complexTy: metamodelica::Ref<ComplexType::NFComplexType>,
    },
    FUNCTION {
        r#fn: metamodelica::Ref<Function::Function>,
        fnType: FunctionType,
    },
    /// Used for MetaModelica generic types
    METABOXED {
        ty: metamodelica::Ref<NFType>,
    },
    POLYMORPHIC {
        name: ArcStr,
    },
    ANY,
    /// A type that might be one of two types depending on a condition.
    ///     The two types are assumed to be array types with equal number of dimensions.
    CONDITIONAL_ARRAY {
        trueType: metamodelica::Ref<NFType>,
        falseType: metamodelica::Ref<NFType>,
        matchedBranch: Branch,
    },
    /// Used by untyped components to store type information needed during typing.
    UNTYPED {
        typeNode: metamodelica::Ref<InstNode::InstNode>,
        dimensions: metamodelica::Array<metamodelica::Ref<Dimension::NFDimension>>,
    },
}
impl metamodelica::gc::MMTrace for NFType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFType::INTEGER => Ok(()),
            NFType::REAL => Ok(()),
            NFType::STRING => Ok(()),
            NFType::BOOLEAN => Ok(()),
            NFType::CLOCK => Ok(()),
            NFType::ENUMERATION { typePath, literals } => {
                metamodelica::gc::MMTrace::mm_accept(typePath, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(literals, __mmv)?;
                Ok(())
            }
            NFType::__ENUMERATION_ANY_NOT_USED__ => Ok(()),
            NFType::ARRAY {
                elementType,
                dimensions,
            } => {
                metamodelica::gc::MMTrace::mm_accept(elementType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dimensions, __mmv)?;
                Ok(())
            }
            NFType::TUPLE { types, names } => {
                metamodelica::gc::MMTrace::mm_accept(types, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(names, __mmv)?;
                Ok(())
            }
            NFType::NORETCALL => Ok(()),
            NFType::UNKNOWN => Ok(()),
            NFType::COMPLEX { cls, complexTy } => {
                metamodelica::gc::MMTrace::mm_accept(cls, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(complexTy, __mmv)?;
                Ok(())
            }
            NFType::FUNCTION { r#fn, fnType } => {
                metamodelica::gc::MMTrace::mm_accept(r#fn, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fnType, __mmv)?;
                Ok(())
            }
            NFType::METABOXED { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            NFType::POLYMORPHIC { name } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                Ok(())
            }
            NFType::ANY => Ok(()),
            NFType::CONDITIONAL_ARRAY {
                trueType,
                falseType,
                matchedBranch,
            } => {
                metamodelica::gc::MMTrace::mm_accept(trueType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(falseType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(matchedBranch, __mmv)?;
                Ok(())
            }
            NFType::UNTYPED { typeNode, dimensions } => {
                metamodelica::gc::MMTrace::mm_accept(typeNode, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dimensions, __mmv)?;
                Ok(())
            }
        }
    }
}
impl NFType {
    pub fn interned_INTEGER() -> metamodelica::Ref<NFType> {
        thread_local! {
            static INTERNED: metamodelica::Ref<NFType> = metamodelica::Ref::new(NFType::INTEGER);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_REAL() -> metamodelica::Ref<NFType> {
        thread_local! {
            static INTERNED: metamodelica::Ref<NFType> = metamodelica::Ref::new(NFType::REAL);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_STRING() -> metamodelica::Ref<NFType> {
        thread_local! {
            static INTERNED: metamodelica::Ref<NFType> = metamodelica::Ref::new(NFType::STRING);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_BOOLEAN() -> metamodelica::Ref<NFType> {
        thread_local! {
            static INTERNED: metamodelica::Ref<NFType> = metamodelica::Ref::new(NFType::BOOLEAN);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_CLOCK() -> metamodelica::Ref<NFType> {
        thread_local! {
            static INTERNED: metamodelica::Ref<NFType> = metamodelica::Ref::new(NFType::CLOCK);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned___ENUMERATION_ANY_NOT_USED__() -> metamodelica::Ref<NFType> {
        thread_local! {
            static INTERNED: metamodelica::Ref<NFType> = metamodelica::Ref::new(NFType::__ENUMERATION_ANY_NOT_USED__);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_NORETCALL() -> metamodelica::Ref<NFType> {
        thread_local! {
            static INTERNED: metamodelica::Ref<NFType> = metamodelica::Ref::new(NFType::NORETCALL);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_UNKNOWN() -> metamodelica::Ref<NFType> {
        thread_local! {
            static INTERNED: metamodelica::Ref<NFType> = metamodelica::Ref::new(NFType::UNKNOWN);
        }
        INTERNED.with(|i| i.clone())
    }
    pub fn interned_ANY() -> metamodelica::Ref<NFType> {
        thread_local! {
            static INTERNED: metamodelica::Ref<NFType> = metamodelica::Ref::new(NFType::ANY);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_INTEGER() -> metamodelica::Ref<NFType> {
    NFType::interned_INTEGER()
}
pub fn interned_REAL() -> metamodelica::Ref<NFType> {
    NFType::interned_REAL()
}
pub fn interned_STRING() -> metamodelica::Ref<NFType> {
    NFType::interned_STRING()
}
pub fn interned_BOOLEAN() -> metamodelica::Ref<NFType> {
    NFType::interned_BOOLEAN()
}
pub fn interned_CLOCK() -> metamodelica::Ref<NFType> {
    NFType::interned_CLOCK()
}
pub fn interned___ENUMERATION_ANY_NOT_USED__() -> metamodelica::Ref<NFType> {
    NFType::interned___ENUMERATION_ANY_NOT_USED__()
}
pub fn interned_NORETCALL() -> metamodelica::Ref<NFType> {
    NFType::interned_NORETCALL()
}
pub fn interned_UNKNOWN() -> metamodelica::Ref<NFType> {
    NFType::interned_UNKNOWN()
}
pub fn interned_ANY() -> metamodelica::Ref<NFType> {
    NFType::interned_ANY()
}
impl Default for NFType {
    fn default() -> Self {
        Self::INTEGER
    }
}
pub use self::NFType::{
    __ENUMERATION_ANY_NOT_USED__, ANY, ARRAY, BOOLEAN, CLOCK, COMPLEX, CONDITIONAL_ARRAY, ENUMERATION, FUNCTION,
    INTEGER, METABOXED, NORETCALL, POLYMORPHIC, REAL, STRING, TUPLE, UNKNOWN, UNTYPED,
};
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum FunctionType {
    /// Function parameter of function type.
    FUNCTIONAL_PARAMETER = 1,
    /// Function name used to reference a function.
    FUNCTION_REFERENCE = 2,
    /// A variable that contains a function reference.
    FUNCTIONAL_VARIABLE = 3,
}
impl PartialOrd for FunctionType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for FunctionType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for FunctionType {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum Branch {
    NONE = 1,
    TRUE = 2,
    FALSE = 3,
}
impl PartialOrd for Branch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Branch {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for Branch {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub fn liftArrayLeft(
    mut ty: metamodelica::Ref<NFType>,
    mut dim: &metamodelica::Ref<Dimension::NFDimension>,
) -> metamodelica::Ref<NFType> {
    let mut ty: metamodelica::Ref<NFType> = ty;
    ty = (match &*ty {
        ARRAY {
            dimensions: __ty_dimensions,
            elementType: __ty_elementType,
        } => metamodelica::Ref::new(NFType::ARRAY {
            elementType: __ty_elementType.clone(),
            dimensions: metamodelica::cons(dim.clone(), __ty_dimensions.clone()),
        }),
        CONDITIONAL_ARRAY {
            falseType: __ty_falseType,
            matchedBranch: __ty_matchedBranch,
            trueType: __ty_trueType,
        } => metamodelica::Ref::new(NFType::CONDITIONAL_ARRAY {
            trueType: liftArrayLeft(__ty_trueType.clone(), dim),
            falseType: liftArrayLeft(__ty_falseType.clone(), dim),
            matchedBranch: __ty_matchedBranch.clone(),
        }),
        _ => metamodelica::Ref::new(NFType::ARRAY {
            elementType: ty,
            dimensions: list![dim.clone()],
        }),
    });
    ty
}

pub fn liftArrayLeftList(
    mut ty: metamodelica::Ref<NFType>,
    mut dims: &metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
) -> metamodelica::Ref<NFType> {
    let mut ty: metamodelica::Ref<NFType> = ty;
    if (dims).is_empty() {
        return ty;
    }
    ty = (match &*ty {
        ARRAY {
            dimensions: __ty_dimensions,
            elementType: __ty_elementType,
        } => metamodelica::Ref::new(NFType::ARRAY {
            elementType: __ty_elementType.clone(),
            dimensions: listAppend(dims.clone(), __ty_dimensions.clone()),
        }),
        CONDITIONAL_ARRAY {
            falseType: __ty_falseType,
            matchedBranch: __ty_matchedBranch,
            trueType: __ty_trueType,
        } => metamodelica::Ref::new(NFType::CONDITIONAL_ARRAY {
            trueType: liftArrayLeftList(__ty_trueType.clone(), dims),
            falseType: liftArrayLeftList(__ty_falseType.clone(), dims),
            matchedBranch: __ty_matchedBranch.clone(),
        }),
        _ => metamodelica::Ref::new(NFType::ARRAY {
            elementType: ty,
            dimensions: dims.clone(),
        }),
    });
    ty
}

pub fn liftArrayRightList(
    mut ty: metamodelica::Ref<NFType>,
    mut dims: &metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
) -> metamodelica::Ref<NFType> {
    let mut ty: metamodelica::Ref<NFType> = ty;
    if (dims).is_empty() {
        return ty;
    }
    ty = (match &*ty {
        ARRAY {
            dimensions: __ty_dimensions,
            elementType: __ty_elementType,
        } => metamodelica::Ref::new(NFType::ARRAY {
            elementType: __ty_elementType.clone(),
            dimensions: listAppend(__ty_dimensions.clone(), dims.clone()),
        }),
        CONDITIONAL_ARRAY {
            falseType: __ty_falseType,
            matchedBranch: __ty_matchedBranch,
            trueType: __ty_trueType,
        } => metamodelica::Ref::new(NFType::CONDITIONAL_ARRAY {
            trueType: liftArrayRightList(__ty_trueType.clone(), dims),
            falseType: liftArrayRightList(__ty_falseType.clone(), dims),
            matchedBranch: __ty_matchedBranch.clone(),
        }),
        _ => metamodelica::Ref::new(NFType::ARRAY {
            elementType: ty,
            dimensions: dims.clone(),
        }),
    });
    ty
}

pub(crate) fn unliftArray(mut ty: metamodelica::Ref<NFType>) -> Result<metamodelica::Ref<NFType>> {
    let mut ty: metamodelica::Ref<NFType> = ty;
    ty = (::match_deref::match_deref! { match &(ty) {
        Deref @ ARRAY { dimensions: Deref @ metamodelica::ListNode::Cons { head: _, tail: dims }, elementType: __ty_elementType } => {
            if ((dims).is_empty()) {__ty_elementType.clone()} else {metamodelica::Ref::new(NFType::ARRAY { elementType: __ty_elementType.clone(), dimensions: dims.clone() })}
        },
        Deref @ CONDITIONAL_ARRAY { falseType: __ty_falseType, matchedBranch: __ty_matchedBranch, trueType: __ty_trueType } => {
            let mut tty: metamodelica::Ref<NFType>;
            let mut fty: metamodelica::Ref<NFType>;
            tty = unliftArray(__ty_trueType.clone())?;
            fty = unliftArray(__ty_falseType.clone())?;
            if (isEqual(&tty, &fty)?) {tty} else {metamodelica::Ref::new(NFType::CONDITIONAL_ARRAY { trueType: tty, falseType: fty, matchedBranch: __ty_matchedBranch.clone() })}
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(ty)
}

pub(crate) fn unliftArrayN(mut N: i32, mut ty: metamodelica::Ref<NFType>) -> Result<metamodelica::Ref<NFType>> {
    let mut ty: metamodelica::Ref<NFType> = ty;
    if N == 0 {
        return Ok(ty);
    }
    ty = (match &*ty {
        ARRAY {
            dimensions: dims,
            elementType: __ty_elementType,
        } => {
            let mut dims = (*dims).clone();
            for mut i in 1..=N {
                dims = (dims).rest()?;
            }
            if ((dims).is_empty()) {
                __ty_elementType.clone()
            } else {
                metamodelica::Ref::new(NFType::ARRAY {
                    elementType: __ty_elementType.clone(),
                    dimensions: dims.clone(),
                })
            }
        }
        CONDITIONAL_ARRAY {
            falseType: __ty_falseType,
            matchedBranch: __ty_matchedBranch,
            trueType: __ty_trueType,
        } => {
            let mut tty: metamodelica::Ref<NFType>;
            let mut fty: metamodelica::Ref<NFType>;
            tty = unliftArrayN(N, __ty_trueType.clone())?;
            fty = unliftArrayN(N, __ty_falseType.clone())?;
            if (isEqual(&tty, &fty)?) {
                tty
            } else {
                metamodelica::Ref::new(NFType::CONDITIONAL_ARRAY {
                    trueType: tty,
                    falseType: fty,
                    matchedBranch: __ty_matchedBranch.clone(),
                })
            }
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(ty)
}

pub fn isInteger<'__b>(mut ty: &'__b metamodelica::Ref<NFType>) -> Result<bool> {
    '__tco: loop {
        match &**ty {
            INTEGER { .. } => return Ok(true),
            METABOXED { .. } => {
                ty = var_field!((**ty).ty, NFType::METABOXED);
                continue '__tco;
            }
            _ => return Ok(false),
        }
    }
}

pub fn isReal<'__b>(mut ty: &'__b metamodelica::Ref<NFType>) -> Result<bool> {
    '__tco: loop {
        match &**ty {
            REAL { .. } => return Ok(true),
            METABOXED { .. } => {
                ty = var_field!((**ty).ty, NFType::METABOXED);
                continue '__tco;
            }
            _ => return Ok(false),
        }
    }
}

pub(crate) fn isBoolean<'__b>(mut ty: &'__b metamodelica::Ref<NFType>) -> bool {
    '__tco: loop {
        match &**ty {
            BOOLEAN { .. } => return true,
            METABOXED { .. } => {
                ty = var_field!((**ty).ty, NFType::METABOXED);
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub fn isString<'__b>(mut ty: &'__b metamodelica::Ref<NFType>) -> Result<bool> {
    '__tco: loop {
        match &**ty {
            STRING { .. } => return Ok(true),
            METABOXED { .. } => {
                ty = var_field!((**ty).ty, NFType::METABOXED);
                continue '__tco;
            }
            _ => return Ok(false),
        }
    }
}

pub fn isClock<'__b>(mut ty: &'__b metamodelica::Ref<NFType>) -> Result<bool> {
    '__tco: loop {
        match &**ty {
            CLOCK { .. } => return Ok(true),
            METABOXED { .. } => {
                ty = var_field!((**ty).ty, NFType::METABOXED);
                continue '__tco;
            }
            _ => return Ok(false),
        }
    }
}

pub fn isContinuous(mut ty: metamodelica::Ref<NFType>) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ COMPLEX { complexTy: ct @ Deref @ ComplexType::RECORD { .. }, .. } => {
            List::all(&(({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFType>> = metamodelica::nil();
        for mut field in (var_field!((**ct).fields, ComplexType::NFComplexType::RECORD).clone()).borrow().iter() {
            let __x = lookupRecordFieldType(&(Record::Field::name(&(field.clone()))), &ty)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), &isContinuous)?
        },
        _ => {
            isReal(&(elementType(ty)))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isScalar(mut ty: &metamodelica::Ref<NFType>) -> bool {
    let mut isScalar: bool;
    isScalar = (match &**ty {
        ARRAY { .. } => false,
        CONDITIONAL_ARRAY { .. } => false,
        _ => true,
    });
    isScalar
}

pub fn isArray(mut ty: &metamodelica::Ref<NFType>) -> bool {
    let mut isArray: bool;
    isArray = (match &**ty {
        ARRAY { .. } => true,
        CONDITIONAL_ARRAY { .. } => true,
        _ => false,
    });
    isArray
}

pub(crate) fn isConditionalArray(mut ty: &metamodelica::Ref<NFType>) -> bool {
    let mut isConditionalArray: bool;
    isConditionalArray = (match &**ty {
        CONDITIONAL_ARRAY { .. } => true,
        _ => false,
    });
    isConditionalArray
}

pub fn isResizable(mut ty: metamodelica::Ref<NFType>) -> Result<bool> {
    let mut b: bool = List::any(&(arrayDims(ty.clone())), &move |__a0: metamodelica::Ref<
        Dimension::NFDimension,
    >|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(Dimension::isResizable(&__a0))
    })?;
    Ok(b)
}

pub(crate) fn sizeKnown(mut ty: metamodelica::Ref<NFType>) -> Result<bool> {
    let mut b: bool = !(List::any(&(arrayDims(ty.clone())), &move |__a0: metamodelica::Ref<
        Dimension::NFDimension,
    >|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(Dimension::isUnknown(&__a0))
    })?);
    Ok(b)
}

pub fn isAny(mut ty: &metamodelica::Ref<NFType>) -> bool {
    let mut b: bool;
    b = (match &**ty {
        ANY => true,
        _ => false,
    });
    b
}

pub(crate) fn setConditionalArrayTypes(
    mut condType: &metamodelica::Ref<NFType>,
    mut trueType: metamodelica::Ref<NFType>,
    mut falseType: metamodelica::Ref<NFType>,
) -> Result<metamodelica::Ref<NFType>> {
    let mut outType: metamodelica::Ref<NFType>;
    let mut matched_branch: Branch;
    let __pa0 = ::match_deref::match_deref! { match &((*condType)) {
        Deref @ CONDITIONAL_ARRAY { matchedBranch: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    matched_branch = metamodelica::Own::own(__pa0);
    outType = metamodelica::Ref::new(NFType::CONDITIONAL_ARRAY {
        trueType: trueType,
        falseType: falseType,
        matchedBranch: matched_branch,
    });
    Ok(outType)
}

pub fn removeSizeOneArraysAndRecords(mut ty: metamodelica::Ref<NFType>) -> Result<metamodelica::Ref<NFType>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(ty.clone()) {
            Deref @ ARRAY { dimensions: __ty_dimensions, .. } => {
                assign_variant_field!(ty => NFType::ARRAY; dimensions = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
            for mut dim in (__ty_dimensions.clone()).into_iter().cloned() {
                if !(!(Dimension::isOne(&(dim.clone()))?)) { continue; }
                let __x = dim.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }));
                if ((var_field!((*ty).dimensions, NFType::ARRAY)).is_empty()) {{ ty = var_field!((*ty).elementType, NFType::ARRAY).clone(); continue '__tco; }} else {return Ok(ty)}
            },
            Deref @ COMPLEX { complexTy: Deref @ ComplexType::RECORD { fields, .. }, .. } if (metamodelica::arrayLength(fields.clone()) == 1) => {
                { ty = lookupRecordFieldType(&(Record::Field::name(&({let __elt = (*metamodelica::index_checked(&fields.borrow(), 1)?).clone(); __elt}))), &ty)?; continue '__tco; }
            },
            _ => {
                return Ok(ty)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn isMatchedBranch(mut condition: bool, mut condType: &metamodelica::Ref<NFType>) -> Result<bool> {
    let mut isMatched: bool = true;
    let mut matched_branch: Branch;
    let __pa0 = ::match_deref::match_deref! { match &((*condType)) {
        Deref @ CONDITIONAL_ARRAY { matchedBranch: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    matched_branch = metamodelica::Own::own(__pa0);
    if condition && matched_branch == Branch::FALSE.clone() || !(condition) && matched_branch == Branch::TRUE.clone() {
        isMatched = false;
    }
    Ok(isMatched)
}

pub(crate) fn matchedConditionalArrayType(mut ty: &metamodelica::Ref<NFType>) -> Result<metamodelica::Ref<NFType>> {
    let mut outType: metamodelica::Ref<NFType>;
    outType = (match &**ty {
        CONDITIONAL_ARRAY {
            falseType: __ty_falseType,
            matchedBranch: __ty_matchedBranch,
            trueType: __ty_trueType,
        } => {
            (match __ty_matchedBranch.clone() {
                Branch::TRUE => __ty_trueType.clone(),
                Branch::FALSE => __ty_falseType.clone(),
                _ => return Err("match: no arm matched"),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outType)
}

pub(crate) fn simplifyConditionalArray(mut ty: metamodelica::Ref<NFType>) -> metamodelica::Ref<NFType> {
    let mut outType: metamodelica::Ref<NFType>;
    outType = (match &*ty.clone() {
        CONDITIONAL_ARRAY {
            falseType: __ty_falseType,
            matchedBranch: __ty_matchedBranch,
            trueType: __ty_trueType,
        } => {
            (match __ty_matchedBranch.clone() {
                Branch::TRUE => __ty_trueType.clone(),
                Branch::FALSE => __ty_falseType.clone(),
                _ => ty,
            })
        }
        _ => ty,
    });
    outType
}

pub(crate) fn unifyArrays(
    mut ty1: metamodelica::Ref<NFType>,
    mut ty2: metamodelica::Ref<NFType>,
) -> Result<metamodelica::Ref<NFType>> {
    fn unify_dims(
        mut dim1: metamodelica::Ref<Dimension::NFDimension>,
        mut dim2: &metamodelica::Ref<Dimension::NFDimension>,
    ) -> Result<metamodelica::Ref<Dimension::NFDimension>> {
        let mut dim: metamodelica::Ref<Dimension::NFDimension>;
        if Dimension::isSame(&dim1, dim2)? {
            dim = dim1;
        } else {
            dim = crate::NFDimension::interned_UNKNOWN();
        }
        Ok(dim)
    }

    let mut outType: metamodelica::Ref<NFType>;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    dims = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
        let __thr_src0 = arrayDims(ty1.clone());
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = arrayDims(ty2);
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(d1), Some(d2)) => {
                    let __x = unify_dims(d1.clone(), &(d2.clone()))?;
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    outType = metamodelica::Ref::new(NFType::ARRAY {
        elementType: elementType(ty1),
        dimensions: dims,
    });
    Ok(outType)
}

pub(crate) fn isVector<'__b>(mut ty: &'__b metamodelica::Ref<NFType>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match ty {
            Deref @ ARRAY { dimensions: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. } => return Ok(true),
            Deref @ CONDITIONAL_ARRAY { .. } => { ty = var_field!((**ty).trueType, NFType::CONDITIONAL_ARRAY); continue '__tco; },
            _ => return Ok(false),
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn isMatrix<'__b>(mut ty: &'__b metamodelica::Ref<NFType>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match ty {
            Deref @ ARRAY { dimensions: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => return Ok(true),
            Deref @ CONDITIONAL_ARRAY { .. } => { ty = var_field!((**ty).trueType, NFType::CONDITIONAL_ARRAY); continue '__tco; },
            _ => return Ok(false),
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn isSquareMatrix<'__b>(mut ty: &'__b metamodelica::Ref<NFType>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match ty {
            Deref @ ARRAY { dimensions: Deref @ metamodelica::ListNode::Cons { head: d1, tail: Deref @ metamodelica::ListNode::Cons { head: d2, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
                return Ok(Dimension::isEqualKnown(metamodelica::AsArg::as_arg(&d1), metamodelica::AsArg::as_arg(&d2))?)
            },
            Deref @ CONDITIONAL_ARRAY { .. } => {
                { ty = var_field!((**ty).trueType, NFType::CONDITIONAL_ARRAY); continue '__tco; }
            },
            _ => {
                return Ok(false)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn isEmptyArray<'__b>(mut ty: &'__b metamodelica::Ref<NFType>) -> Result<bool> {
    '__tco: loop {
        match &**ty {
            ARRAY { .. } => {
                return Ok(List::any(
                    var_field!((**ty).dimensions, NFType::ARRAY),
                    &move |__a0: metamodelica::Ref<Dimension::NFDimension>| Dimension::isZero(&__a0),
                )?);
            }
            CONDITIONAL_ARRAY { .. } => {
                ty = var_field!((**ty).trueType, NFType::CONDITIONAL_ARRAY);
                continue '__tco;
            }
            _ => return Ok(false),
        }
    }
}

pub(crate) fn isSingleElementArray(mut ty: &metamodelica::Ref<NFType>) -> Result<bool> {
    let mut isSingleElement: bool;
    isSingleElement = (::match_deref::match_deref! { match ty {
        Deref @ ARRAY { dimensions: Deref @ metamodelica::ListNode::Cons { head: d, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            Dimension::isKnown(metamodelica::AsArg::as_arg(&d), false) && Dimension::size(metamodelica::AsArg::as_arg(&d), false)? == 1
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isSingleElement)
}

pub(crate) fn isEnumeration(mut ty: &metamodelica::Ref<NFType>) -> bool {
    let mut isEnum: bool;
    isEnum = (match &**ty {
        ENUMERATION { .. } => true,
        _ => false,
    });
    isEnum
}

pub(crate) fn isBuiltinEnumeration(mut ty: &metamodelica::Ref<NFType>) -> bool {
    let mut isBuiltin: bool;
    let mut name: ArcStr;
    isBuiltin = (::match_deref::match_deref! { match ty {
        Deref @ ENUMERATION { typePath: Deref @ Absyn::Path::IDENT { name: __esc_name }, .. } => {
            name = (*__esc_name).clone();
            (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "StateSelect" => true,
        Deref @ "AssertionLevel" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBuiltin
}

pub(crate) fn isUnspecifiedEnumeration(mut ty: &metamodelica::Ref<NFType>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match ty {
        Deref @ ENUMERATION { literals: Deref @ metamodelica::ListNode::Nil, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isComplex(mut ty: &metamodelica::Ref<NFType>) -> bool {
    let mut isComplex: bool;
    isComplex = (match &**ty {
        COMPLEX { .. } => true,
        _ => false,
    });
    isComplex
}

pub(crate) fn isComplexArray(mut ty: &metamodelica::Ref<NFType>) -> Result<bool> {
    let mut isComplex: bool;
    isComplex = (match &**ty {
        ARRAY {
            elementType: __ty_elementType,
            ..
        } => self::isComplex(metamodelica::AsArg::as_arg(&__ty_elementType)),
        _ => false,
    });
    Ok(isComplex)
}

pub fn complexNode(mut ty: &metamodelica::Ref<NFType>) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut cell: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let __pa0 = ::match_deref::match_deref! { match &((*ty)) {
        Deref @ COMPLEX { cls: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cell = metamodelica::Own::own(__pa0);
    node = NFInstNode::InstNode::borrow(cell)?;
    Ok(node)
}

pub(crate) fn complexComponents(
    mut ty: &metamodelica::Ref<NFType>,
) -> Result<metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>> {
    let mut comps: metamodelica::Array<metamodelica::Ref<InstNode::InstNode>>;
    comps = ClassTree::getComponents(&(Class::classTree(NFInstNode::InstNode::getClass(complexNode(ty)?)?)?))?;
    Ok(comps)
}

pub(crate) fn isConnector(mut ty: &metamodelica::Ref<NFType>) -> bool {
    let mut isConnector: bool;
    isConnector = (::match_deref::match_deref! { match ty {
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::CONNECTOR { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isConnector
}

pub(crate) fn isStreamConnector(mut ty: &metamodelica::Ref<NFType>) -> bool {
    let mut isStreamConnector: bool;
    isStreamConnector = (::match_deref::match_deref! { match ty {
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::CONNECTOR { streams: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isStreamConnector
}

pub(crate) fn isExpandableConnector(mut ty: &metamodelica::Ref<NFType>) -> bool {
    let mut isExpandable: bool;
    isExpandable = (::match_deref::match_deref! { match ty {
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::EXPANDABLE_CONNECTOR { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isExpandable
}

pub fn isExternalObject(mut ty: &metamodelica::Ref<NFType>) -> bool {
    let mut isEO: bool;
    isEO = (::match_deref::match_deref! { match ty {
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEO
}

pub fn isRecord(mut ty: &metamodelica::Ref<NFType>) -> bool {
    let mut isRecord: bool;
    isRecord = (::match_deref::match_deref! { match ty {
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::RECORD { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isRecord
}

pub(crate) fn isBasic(mut ty: metamodelica::Ref<NFType>) -> bool {
    '__tco: loop {
        match &*ty {
            REAL { .. } => return true,
            INTEGER { .. } => return true,
            BOOLEAN { .. } => return true,
            STRING { .. } => return true,
            ENUMERATION { .. } => return true,
            CLOCK { .. } => return true,
            FUNCTION { r#fn: __ty_fn, .. } => {
                ty = Function::returnType(metamodelica::AsArg::as_arg(&__ty_fn));
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub(crate) fn isBasicNumeric(mut ty: &metamodelica::Ref<NFType>) -> bool {
    let mut isNumeric: bool;
    isNumeric = (match &**ty {
        REAL { .. } => true,
        INTEGER { .. } => true,
        _ => false,
    });
    isNumeric
}

pub(crate) fn isNumeric<'__b>(mut ty: &'__b metamodelica::Ref<NFType>) -> Result<bool> {
    '__tco: loop {
        match &**ty {
            ARRAY { .. } => return Ok(isBasicNumeric(var_field!((**ty).elementType, NFType::ARRAY))),
            CONDITIONAL_ARRAY { .. } => {
                ty = var_field!((**ty).trueType, NFType::CONDITIONAL_ARRAY);
                continue '__tco;
            }
            _ => return Ok(isBasicNumeric(ty)),
        }
    }
}

pub(crate) fn isScalarBuiltin(mut ty: metamodelica::Ref<NFType>) -> Result<bool> {
    '__tco: loop {
        match &*ty {
            INTEGER { .. } => return Ok(true),
            REAL { .. } => return Ok(true),
            STRING { .. } => return Ok(true),
            BOOLEAN { .. } => return Ok(true),
            CLOCK { .. } => return Ok(true),
            ENUMERATION { .. } => return Ok(true),
            FUNCTION { r#fn: __ty_fn, .. } => {
                ty = Function::returnType(metamodelica::AsArg::as_arg(&__ty_fn));
                continue '__tco;
            }
            _ => return Ok(false),
        }
    }
}

pub fn isTuple(mut ty: &metamodelica::Ref<NFType>) -> bool {
    let mut isTuple: bool;
    isTuple = (match &**ty {
        TUPLE { .. } => true,
        _ => false,
    });
    isTuple
}

pub(crate) fn isUnknown(mut ty: &metamodelica::Ref<NFType>) -> bool {
    let mut isUnknown: bool;
    isUnknown = (match &**ty {
        UNKNOWN { .. } => true,
        _ => false,
    });
    isUnknown
}

pub(crate) fn isKnown(mut ty: &metamodelica::Ref<NFType>) -> bool {
    let mut isKnown: bool;
    isKnown = (match &**ty {
        UNKNOWN { .. } => false,
        UNTYPED { .. } => false,
        _ => true,
    });
    isKnown
}

pub(crate) fn isPolymorphic(mut ty: &metamodelica::Ref<NFType>) -> bool {
    let mut isPolymorphic: bool;
    isPolymorphic = (match &**ty {
        POLYMORPHIC { .. } => true,
        _ => false,
    });
    isPolymorphic
}

pub(crate) fn isPolymorphicNamed(mut ty: &metamodelica::Ref<NFType>, mut name: &ArcStr) -> bool {
    let mut res: bool;
    res = (match &**ty {
        POLYMORPHIC { name: __ty_name } => metamodelica::stringEq(&name, &__ty_name),
        _ => false,
    });
    res
}

pub(crate) fn firstTupleType(mut ty: &metamodelica::Ref<NFType>) -> Result<metamodelica::Ref<NFType>> {
    let mut outTy: metamodelica::Ref<NFType>;
    outTy = (match &**ty {
        TUPLE { types: __ty_types, .. } => (__ty_types).head().cloned()?,
        ARRAY {
            dimensions: __ty_dimensions,
            elementType: __ty_elementType,
        } => metamodelica::Ref::new(NFType::ARRAY {
            elementType: firstTupleType(metamodelica::AsArg::as_arg(&__ty_elementType))?,
            dimensions: __ty_dimensions.clone(),
        }),
        _ => ty.clone(),
    });
    Ok(outTy)
}

pub(crate) fn nthTupleType(mut ty: &metamodelica::Ref<NFType>, mut n: i32) -> Result<metamodelica::Ref<NFType>> {
    let mut outTy: metamodelica::Ref<NFType>;
    outTy = (match &**ty {
        TUPLE { types: __ty_types, .. } => (__ty_types).get(n)?,
        ARRAY {
            dimensions: __ty_dimensions,
            elementType: __ty_elementType,
        } => metamodelica::Ref::new(NFType::ARRAY {
            elementType: nthTupleType(metamodelica::AsArg::as_arg(&__ty_elementType), n)?,
            dimensions: __ty_dimensions.clone(),
        }),
        _ => ty.clone(),
    });
    Ok(outTy)
}

pub fn arrayElementType<'__b>(mut ty: &'__b metamodelica::Ref<NFType>) -> metamodelica::Ref<NFType> {
    '__tco: loop {
        match &**ty {
            ARRAY { .. } => return var_field!((**ty).elementType, NFType::ARRAY).clone(),
            CONDITIONAL_ARRAY { .. } => {
                ty = var_field!((**ty).trueType, NFType::CONDITIONAL_ARRAY);
                continue '__tco;
            }
            UNTYPED { .. }
                if (!(var_field!((**ty).dimensions, NFType::UNTYPED)
                    .clone()
                    .borrow()
                    .is_empty())) =>
            {
                return metamodelica::Ref::new(NFType::UNTYPED {
                    typeNode: var_field!((**ty).typeNode, NFType::UNTYPED).clone(),
                    dimensions: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
                });
            }
            _ => return ty.clone(),
        }
    }
}

pub(crate) fn setArrayElementType(
    mut arrayTy: &metamodelica::Ref<NFType>,
    mut elementTy: &metamodelica::Ref<NFType>,
) -> metamodelica::Ref<NFType> {
    let mut ty: metamodelica::Ref<NFType>;
    ty = (match &**arrayTy {
        ARRAY {
            dimensions: __arrayTy_dimensions,
            ..
        } => liftArrayLeftList(elementTy.clone(), metamodelica::AsArg::as_arg(&__arrayTy_dimensions)),
        CONDITIONAL_ARRAY {
            falseType: __arrayTy_falseType,
            matchedBranch: __arrayTy_matchedBranch,
            trueType: __arrayTy_trueType,
        } => metamodelica::Ref::new(NFType::CONDITIONAL_ARRAY {
            trueType: setArrayElementType(metamodelica::AsArg::as_arg(&__arrayTy_trueType), elementTy),
            falseType: setArrayElementType(metamodelica::AsArg::as_arg(&__arrayTy_falseType), elementTy),
            matchedBranch: __arrayTy_matchedBranch.clone(),
        }),
        _ => elementTy.clone(),
    });
    ty
}

pub fn elementType(mut ty: metamodelica::Ref<NFType>) -> metamodelica::Ref<NFType> {
    '__tco: loop {
        match &*ty {
            ARRAY {
                elementType: __ty_elementType,
                ..
            } => return __ty_elementType.clone(),
            CONDITIONAL_ARRAY {
                trueType: __ty_trueType,
                ..
            } => {
                ty = __ty_trueType.clone();
                continue '__tco;
            }
            FUNCTION { r#fn: __ty_fn, .. } => {
                ty = Function::returnType(metamodelica::AsArg::as_arg(&__ty_fn));
                continue '__tco;
            }
            _ => return ty,
        }
    }
}

pub(crate) fn copyElementType(
    mut dstType: &metamodelica::Ref<NFType>,
    mut srcType: &metamodelica::Ref<NFType>,
) -> metamodelica::Ref<NFType> {
    let mut ty: metamodelica::Ref<NFType>;
    ty = setArrayElementType(dstType, &(arrayElementType(srcType)));
    ty
}

pub fn arrayDims(mut ty: metamodelica::Ref<NFType>) -> metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> {
    '__tco: loop {
        match &*ty {
            ARRAY {
                dimensions: __ty_dimensions,
                ..
            } => return __ty_dimensions.clone(),
            FUNCTION { r#fn: __ty_fn, .. } => {
                ty = Function::returnType(metamodelica::AsArg::as_arg(&__ty_fn));
                continue '__tco;
            }
            METABOXED { ty: __ty_ty } => {
                ty = __ty_ty.clone();
                continue '__tco;
            }
            CONDITIONAL_ARRAY {
                trueType: __ty_trueType,
                ..
            } => {
                return List::fill(
                    crate::NFDimension::interned_UNKNOWN(),
                    dimensionCount(__ty_trueType.clone()),
                );
            }
            UNTYPED { .. } => {
                return var_field!((*ty).dimensions, NFType::UNTYPED)
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<metamodelica::List<_>>();
            }
            _ => return metamodelica::nil(),
        }
    }
}

pub(crate) fn copyDims(
    mut srcType: metamodelica::Ref<NFType>,
    mut dstType: metamodelica::Ref<NFType>,
) -> metamodelica::Ref<NFType> {
    let mut ty: metamodelica::Ref<NFType>;
    if (arrayDims(srcType.clone())).is_empty() {
        ty = arrayElementType(&dstType);
    } else {
        ty = (match &*dstType {
            ARRAY {
                elementType: __dstType_elementType,
                ..
            } => metamodelica::Ref::new(NFType::ARRAY {
                elementType: __dstType_elementType.clone(),
                dimensions: arrayDims(srcType),
            }),
            _ => metamodelica::Ref::new(NFType::ARRAY {
                elementType: dstType,
                dimensions: arrayDims(srcType),
            }),
        });
    }
    ty
}

pub fn applyToDims(
    mut ty: metamodelica::Ref<NFType>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Dimension::NFDimension>) -> Result<metamodelica::Ref<Dimension::NFDimension>>,
) -> Result<metamodelica::Ref<NFType>> {
    pub type dimFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Dimension::NFDimension>,
            ) -> Result<metamodelica::Ref<Dimension::NFDimension>>
            + 'static,
    >;

    let mut ty: metamodelica::Ref<NFType> = ty;
    ty = (match &*ty {
        ARRAY {
            dimensions: __ty_dimensions,
            ..
        } => {
            assign_variant_field!(ty => NFType::ARRAY; dimensions = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
                for mut d in (__ty_dimensions.clone()).into_iter().cloned() {
                    let __x = func(d.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ty
        }
        FUNCTION { r#fn, .. } => {
            let mut r#fn = (*r#fn).clone();
            assign_field!(r#fn.returnType = applyToDims(r#fn.returnType.clone(), func)?);
            assign_variant_field!(ty => NFType::FUNCTION; r#fn = r#fn.clone());
            ty
        }
        METABOXED { ty: __ty_ty } => {
            assign_variant_field!(ty => NFType::METABOXED; ty = applyToDims(__ty_ty.clone(), func)?);
            ty
        }
        CONDITIONAL_ARRAY {
            trueType: __ty_trueType,
            ..
        } => {
            assign_variant_field!(ty => NFType::CONDITIONAL_ARRAY; trueType = applyToDims(__ty_trueType.clone(), func)?);
            ty
        }
        UNTYPED { .. } => {
            for mut i in 1..=metamodelica::arrayLength(var_field!((*ty).dimensions, NFType::UNTYPED).clone()) {
                metamodelica::arrayUpdate(
                    var_field!((*ty).dimensions, NFType::UNTYPED).clone(),
                    i,
                    func(
                        ({
                            let __elt = (*metamodelica::index_checked(
                                &var_field!((*ty).dimensions, NFType::UNTYPED).borrow(),
                                i,
                            )?)
                            .clone();
                            __elt
                        }),
                    )?,
                )?;
            }
            ty
        }
        _ => ty,
    });
    Ok(ty)
}

pub fn nthDimension(
    mut ty: metamodelica::Ref<NFType>,
    mut index: i32,
) -> Result<metamodelica::Ref<Dimension::NFDimension>> {
    '__tco: loop {
        match &*ty {
            ARRAY {
                dimensions: __ty_dimensions,
                ..
            } => return Ok((__ty_dimensions).get(index)?),
            CONDITIONAL_ARRAY { .. } => {
                (ty, index) = (matchedConditionalArrayType(&ty)?, index);
                continue '__tco;
            }
            FUNCTION { r#fn: __ty_fn, .. } => {
                (ty, index) = (Function::returnType(metamodelica::AsArg::as_arg(&__ty_fn)), index);
                continue '__tco;
            }
            METABOXED { ty: __ty_ty } => {
                (ty, index) = (__ty_ty.clone(), index);
                continue '__tco;
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

pub fn dimensionCount(mut ty: metamodelica::Ref<NFType>) -> i32 {
    '__tco: loop {
        match &*ty {
            ARRAY {
                dimensions: __ty_dimensions,
                ..
            } => return ((__ty_dimensions).len() as i32),
            CONDITIONAL_ARRAY {
                trueType: __ty_trueType,
                ..
            } => {
                ty = __ty_trueType.clone();
                continue '__tco;
            }
            FUNCTION { r#fn: __ty_fn, .. } => {
                ty = Function::returnType(metamodelica::AsArg::as_arg(&__ty_fn));
                continue '__tco;
            }
            METABOXED { ty: __ty_ty } => {
                ty = __ty_ty.clone();
                continue '__tco;
            }
            UNTYPED { .. } => return metamodelica::arrayLength(var_field!((*ty).dimensions, NFType::UNTYPED).clone()),
            _ => return 0,
        }
    }
}

pub(crate) fn dimensionDiff(mut ty1: metamodelica::Ref<NFType>, mut ty2: metamodelica::Ref<NFType>) -> i32 {
    let mut diff: i32 = dimensionCount(ty1.clone()) - dimensionCount(ty2.clone());
    diff
}

pub fn hasKnownSize(mut ty: metamodelica::Ref<NFType>) -> Result<bool> {
    '__tco: loop {
        match &*ty {
            ARRAY {
                dimensions: __ty_dimensions,
                ..
            } => {
                return Ok(List::all(
                    metamodelica::AsArg::as_arg(&__ty_dimensions),
                    &({
                        let __pe_b1 = false;
                        move |__pe_a0| Ok(Dimension::isKnown(&__pe_a0, __pe_b1.clone()))
                    }),
                )?);
            }
            CONDITIONAL_ARRAY { .. } => return Ok(false),
            FUNCTION { r#fn: __ty_fn, .. } => {
                ty = Function::returnType(metamodelica::AsArg::as_arg(&__ty_fn));
                continue '__tco;
            }
            _ => return Ok(true),
        }
    }
}

pub(crate) fn hasZeroDimension(mut ty: &metamodelica::Ref<NFType>) -> Result<bool> {
    let mut hasZero: bool;
    hasZero = (match &**ty {
        ARRAY {
            dimensions: __ty_dimensions,
            ..
        } => List::any(
            metamodelica::AsArg::as_arg(&__ty_dimensions),
            &move |__a0: metamodelica::Ref<Dimension::NFDimension>| Dimension::isZero(&__a0),
        )?,
        CONDITIONAL_ARRAY {
            falseType: __ty_falseType,
            trueType: __ty_trueType,
            ..
        } => {
            hasZeroDimension(metamodelica::AsArg::as_arg(&__ty_trueType))?
                && hasZeroDimension(metamodelica::AsArg::as_arg(&__ty_falseType))?
        }
        _ => false,
    });
    Ok(hasZero)
}

pub(crate) fn mapDims(
    mut ty: metamodelica::Ref<NFType>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Dimension::NFDimension>) -> Result<metamodelica::Ref<Dimension::NFDimension>>,
) -> Result<metamodelica::Ref<NFType>> {
    pub type FuncT = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Dimension::NFDimension>,
            ) -> Result<metamodelica::Ref<Dimension::NFDimension>>
            + 'static,
    >;

    let mut ty: metamodelica::Ref<NFType> = ty;
    let () = (match &*ty {
        ARRAY {
            dimensions: __ty_dimensions,
            ..
        } => {
            assign_variant_field!(ty => NFType::ARRAY; dimensions = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
                for mut d in (__ty_dimensions.clone()).into_iter().cloned() {
                    let __x = func(d.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        TUPLE { types: __ty_types, .. } => {
            assign_variant_field!(ty => NFType::TUPLE; types = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<NFType>> = metamodelica::nil();
                for mut t in (__ty_types.clone()).into_iter().cloned() {
                    let __x = mapDims(t.clone(), func)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        FUNCTION { r#fn, .. } => {
            assign_variant_field!(ty => NFType::FUNCTION; r#fn = Function::setReturnType(mapDims(Function::returnType(metamodelica::AsArg::as_arg(&r#fn)), func)?, r#fn.clone()));
            ()
        }
        METABOXED { ty: __ty_ty } => {
            assign_variant_field!(ty => NFType::METABOXED; ty = mapDims(__ty_ty.clone(), func)?);
            ()
        }
        CONDITIONAL_ARRAY {
            trueType: __ty_trueType,
            ..
        } => {
            assign_variant_field!(ty => NFType::CONDITIONAL_ARRAY;
                trueType = mapDims(__ty_trueType.clone(), func)?,
                falseType = mapDims(var_field!((*ty).falseType, NFType::CONDITIONAL_ARRAY).clone(), func)?
            );
            ()
        }
        _ => (),
    });
    Ok(ty)
}

pub(crate) fn foldDims<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut ty: metamodelica::Ref<NFType>,
    mut func: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Dimension::NFDimension>, ArgT) -> Result<ArgT> + 'static>,
    mut arg: ArgT,
) -> Result<ArgT> {
    pub type FuncT<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Dimension::NFDimension>, ArgT) -> Result<ArgT> + 'static>;

    '__tco: loop {
        match &*ty {
            ARRAY {
                dimensions: __ty_dimensions,
                ..
            } => return Ok(List::fold(metamodelica::AsArg::as_arg(&__ty_dimensions), &*func, arg)?),
            TUPLE { types: __ty_types, .. } => {
                return Ok(List::fold(
                    metamodelica::AsArg::as_arg(&__ty_types),
                    &({
                        let __pe_b1: Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<Dimension::NFDimension>, _) -> Result<_> + 'static,
                        > = func.clone();
                        move |__pe_a0, __pe_a2| foldDims(__pe_a0, __pe_b1.clone(), __pe_a2)
                    }),
                    arg,
                )?);
            }
            FUNCTION { r#fn: __ty_fn, .. } => {
                (ty, func, arg) = (
                    Function::returnType(metamodelica::AsArg::as_arg(&__ty_fn)),
                    func.clone(),
                    arg,
                );
                continue '__tco;
            }
            METABOXED { ty: __ty_ty } => {
                (ty, func, arg) = (__ty_ty.clone(), func.clone(), arg);
                continue '__tco;
            }
            _ => return Ok(arg),
        }
    }
}

pub(crate) fn nthEnumLiteral(mut ty: &metamodelica::Ref<NFType>, mut index: i32) -> Result<ArcStr> {
    let mut literal: ArcStr;
    let mut literals: metamodelica::List<ArcStr>;
    let __pa0 = ::match_deref::match_deref! { match &((*ty)) {
        Deref @ ENUMERATION { literals: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    literals = metamodelica::Own::own(__pa0);
    literal = (literals).get(index)?;
    Ok(literal)
}

pub fn toString<'__b>(mut ty: &'__b metamodelica::Ref<NFType>) -> Result<ArcStr> {
    '__tco: loop {
        match &**ty {
            INTEGER => return Ok(literal!("Integer")),
            REAL => return Ok(literal!("Real")),
            STRING => return Ok(literal!("String")),
            BOOLEAN => return Ok(literal!("Boolean")),
            CLOCK => return Ok(literal!("Clock")),
            ENUMERATION { .. } => {
                if ((var_field!((**ty).literals, NFType::ENUMERATION)).is_empty()) {
                    return Ok(literal!("enumeration(:)"));
                } else {
                    return Ok({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("enumeration "));
                        __mm_s.push_str(&*AbsynUtil::pathString(
                            var_field!((**ty).typePath, NFType::ENUMERATION).clone(),
                            literal!("."),
                            true,
                            false,
                        )?);
                        __mm_s.push_str(&*literal!("("));
                        __mm_s.push_str(&*stringDelimitList(
                            var_field!((**ty).literals, NFType::ENUMERATION).clone(),
                            literal!(", "),
                        ));
                        __mm_s.push_str(&*literal!(")"));
                        ArcStr::from(__mm_s)
                    });
                }
            }
            ARRAY { .. } => {
                return Ok(List::toStringCustom(
                    var_field!((**ty).dimensions, NFType::ARRAY).clone(),
                    &move |__a0: metamodelica::Ref<Dimension::NFDimension>| Dimension::toString(&__a0),
                    toString(var_field!((**ty).elementType, NFType::ARRAY))?,
                    literal!("["),
                    literal!(", "),
                    literal!("]"),
                    false,
                    0,
                )?);
            }
            TUPLE { .. } => {
                return Ok({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("("));
                    __mm_s.push_str(&*stringDelimitList(
                        List::map(
                            var_field!((**ty).types, NFType::TUPLE).clone(),
                            &move |__a0: metamodelica::Ref<NFType>| toString(&__a0),
                        )?,
                        literal!(", "),
                    ));
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                });
            }
            NORETCALL => return Ok(literal!("()")),
            UNKNOWN => return Ok(literal!("unknown()")),
            COMPLEX { .. } => {
                return Ok(AbsynUtil::pathString(
                    NFInstNode::InstNode::scopePath(
                        complexNode(ty)?,
                        NFInstNode::InstNode::ScopeType::RELATIVE.clone(),
                        false,
                    )?,
                    literal!("."),
                    true,
                    false,
                )?);
            }
            FUNCTION { .. } => return Ok(Function::typeString(var_field!((**ty).r#fn, NFType::FUNCTION))?),
            METABOXED { .. } => {
                ty = var_field!((**ty).ty, NFType::METABOXED);
                continue '__tco;
            }
            POLYMORPHIC { .. } => {
                if (StringUtil::startsWith(var_field!((**ty).name, NFType::POLYMORPHIC).clone(), literal!("__"))) {
                    return Ok(substring(
                        var_field!((**ty).name, NFType::POLYMORPHIC).clone(),
                        3,
                        ((var_field!((**ty).name, NFType::POLYMORPHIC)).len() as i32),
                    )?);
                } else {
                    return Ok({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("<"));
                        __mm_s.push_str(&*var_field!((**ty).name, NFType::POLYMORPHIC));
                        __mm_s.push_str(&*literal!(">"));
                        ArcStr::from(__mm_s)
                    });
                }
            }
            ANY => return Ok(literal!("$ANY$")),
            CONDITIONAL_ARRAY { .. } => {
                return Ok({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*toString(var_field!((**ty).trueType, NFType::CONDITIONAL_ARRAY))?);
                    __mm_s.push_str(&*literal!("|"));
                    __mm_s.push_str(&*toString(var_field!((**ty).falseType, NFType::CONDITIONAL_ARRAY))?);
                    ArcStr::from(__mm_s)
                });
            }
            UNTYPED { .. } => {
                return Ok(Array::toString(
                    var_field!((**ty).dimensions, NFType::UNTYPED).clone(),
                    &move |__a0: metamodelica::Ref<Dimension::NFDimension>| Dimension::toString(&__a0),
                    NFInstNode::InstNode::name(var_field!((**ty).typeNode, NFType::UNTYPED))?,
                    literal!("["),
                    literal!(", "),
                    literal!("]"),
                    false,
                    0,
                )?);
            }
            _ => {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFType.toString"));
                        __mm_s.push_str(&*literal!(" got unknown type: "));
                        __mm_s.push_str(&*anyString(ty.clone()));
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFType.mo")),
                )?;
                return Ok(return Err("fail"));
            }
        }
    }
}

pub(crate) fn toFlatString<'__b>(
    mut ty: &'__b metamodelica::Ref<NFType>,
    mut format: BaseModelica::OutputFormat,
) -> Result<ArcStr> {
    '__tco: loop {
        match &**ty {
            INTEGER => return Ok(literal!("Integer")),
            REAL => return Ok(literal!("Real")),
            STRING => return Ok(literal!("String")),
            BOOLEAN => return Ok(literal!("Boolean")),
            CLOCK => return Ok(literal!("Clock")),
            ENUMERATION { .. } => {
                if ((var_field!((**ty).literals, NFType::ENUMERATION)).is_empty()) {
                    return Ok(literal!("enumeration(:)"));
                } else if (isBuiltinEnumeration(ty)) {
                    return Ok(AbsynUtil::pathString(
                        var_field!((**ty).typePath, NFType::ENUMERATION).clone(),
                        literal!("."),
                        true,
                        false,
                    )?);
                } else {
                    return Ok(Util::makeQuotedIdentifier(AbsynUtil::pathString(
                        var_field!((**ty).typePath, NFType::ENUMERATION).clone(),
                        literal!("."),
                        true,
                        false,
                    )?)?);
                }
            }
            ARRAY { .. } => {
                return Ok(Dimension::toFlatStringList(
                    var_field!((**ty).dimensions, NFType::ARRAY).clone(),
                    format,
                    toFlatString(var_field!((**ty).elementType, NFType::ARRAY), format)?,
                )?);
            }
            TUPLE { .. } => {
                return Ok({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("("));
                    __mm_s.push_str(&*stringDelimitList(
                        List::map(
                            var_field!((**ty).types, NFType::TUPLE).clone(),
                            &({
                                let __pe_b1 = format;
                                move |__pe_a0| toFlatString(&__pe_a0, __pe_b1.clone())
                            }),
                        )?,
                        literal!(", "),
                    ));
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                });
            }
            NORETCALL => return Ok(literal!("()")),
            UNKNOWN => return Ok(literal!("unknown()")),
            COMPLEX { .. } => {
                return Ok(Util::makeQuotedIdentifier(AbsynUtil::pathString(
                    NFInstNode::InstNode::scopePath(
                        complexNode(ty)?,
                        NFInstNode::InstNode::ScopeType::RELATIVE.clone(),
                        false,
                    )?,
                    literal!("."),
                    true,
                    false,
                )?)?);
            }
            FUNCTION { .. } => {
                return Ok(Util::makeQuotedIdentifier(AbsynUtil::pathString(
                    NFInstNode::InstNode::scopePath(
                        NFInstNode::InstNode::fromHandle(&var_field!((**ty).r#fn, NFType::FUNCTION).node)?,
                        NFInstNode::InstNode::ScopeType::RELATIVE.clone(),
                        false,
                    )?,
                    literal!("."),
                    true,
                    false,
                )?)?);
            }
            METABOXED { .. } => {
                (ty, format) = (var_field!((**ty).ty, NFType::METABOXED), format);
                continue '__tco;
            }
            POLYMORPHIC { .. } => {
                return Ok({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("<"));
                    __mm_s.push_str(&*var_field!((**ty).name, NFType::POLYMORPHIC));
                    __mm_s.push_str(&*literal!(">"));
                    ArcStr::from(__mm_s)
                });
            }
            ANY => return Ok(literal!("$ANY$")),
            CONDITIONAL_ARRAY { .. } => {
                return Ok({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*toFlatString(
                        var_field!((**ty).trueType, NFType::CONDITIONAL_ARRAY),
                        format,
                    )?);
                    __mm_s.push_str(&*literal!("|"));
                    __mm_s.push_str(&*toFlatString(
                        var_field!((**ty).falseType, NFType::CONDITIONAL_ARRAY),
                        format,
                    )?);
                    ArcStr::from(__mm_s)
                });
            }
            UNTYPED { .. } => {
                return Ok(Array::toString(
                    var_field!((**ty).dimensions, NFType::UNTYPED).clone(),
                    &({
                        let __pe_b1 = format;
                        move |__pe_a0| Dimension::toFlatString(&__pe_a0, __pe_b1.clone())
                    }),
                    NFInstNode::InstNode::name(var_field!((**ty).typeNode, NFType::UNTYPED))?,
                    literal!("["),
                    literal!(", "),
                    literal!("]"),
                    false,
                    0,
                )?);
            }
            _ => {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFType.toFlatString"));
                        __mm_s.push_str(&*literal!(" got unknown type: "));
                        __mm_s.push_str(&*anyString(ty.clone()));
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFType.mo")),
                )?;
                return Ok(return Err("fail"));
            }
        }
    }
}

pub(crate) fn dimensionsToFlatString(
    mut ty: metamodelica::Ref<NFType>,
    mut format: BaseModelica::OutputFormat,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match &*ty {
        ARRAY {
            dimensions: __ty_dimensions,
            ..
        } => stringDelimitList(
            List::map(
                __ty_dimensions.clone(),
                &({
                    let __pe_b1 = format;
                    move |__pe_a0| Dimension::toFlatString(&__pe_a0, __pe_b1.clone())
                }),
            )?,
            literal!(", "),
        ),
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFType.dimensionsToFlatString"));
                    __mm_s.push_str(&*literal!(" got unknown or not array type: "));
                    __mm_s.push_str(&*anyString(ty));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFType.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(r#str)
}

pub(crate) fn toFlatDeclarationStream(
    mut ty: &metamodelica::Ref<NFType>,
    mut format: BaseModelica::OutputFormat,
    mut indent: ArcStr,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut name: ArcStr;
    let mut complexTy: metamodelica::Ref<ComplexType::NFComplexType>;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut constructor: metamodelica::Ref<InstNode::InstNode>;
    let mut destructor: metamodelica::Ref<InstNode::InstNode>;
    let mut f: metamodelica::Ref<Function::Function>;
    s = (::match_deref::match_deref! { match ty {
        Deref @ ENUMERATION { literals: __ty_literals, typePath: __ty_typePath } => {
            s = IOStream::append(s, indent)?;
            s = IOStream::append(s, literal!("type "))?;
            s = IOStream::append(s, Util::makeQuotedIdentifier(AbsynUtil::pathString(__ty_typePath.clone(), literal!("."), true, false)?)?)?;
            s = IOStream::append(s, literal!(" = enumeration("))?;
            s = IOStream::append(s, stringDelimitList(({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut l in (__ty_literals.clone()).into_iter().cloned() {
            let __x = Util::makeQuotedIdentifier(l.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), literal!(", ")))?;
            s = IOStream::append(s, literal!(")"))?;
            s
        },
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::RECORD { .. }, .. } => Record::toFlatDeclarationStream(complexNode(ty)?, format, indent, s)?,
        Deref @ COMPLEX { complexTy: __esc_complexTy @ Deref @ ComplexType::EXTERNAL_OBJECT { .. }, .. } => {
            complexTy = (*__esc_complexTy).clone();
            path = NFInstNode::InstNode::scopePath(complexNode(ty)?, NFInstNode::InstNode::ScopeType::RELATIVE.clone(), false)?;
            name = Util::makeQuotedIdentifier(AbsynUtil::pathString(path, literal!("."), true, false)?)?;
            s = IOStream::append(s, indent.clone())?;
            s = IOStream::append(s, literal!("class "))?;
            s = IOStream::append(s, name.clone())?;
            s = IOStream::append(s, literal!("\n  extends ExternalObject;\n\n"))?;
            let __pa0 = ::match_deref::match_deref! { match &(Function::typeNodeCache(NFInstNode::InstNode::borrow(var_field!((*complexTy).constructor, ComplexType::NFComplexType::EXTERNAL_OBJECT).clone())?, NFInstContext::FUNCTION.clone())?) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            f = metamodelica::Own::own(__pa0);
            s = Function::toFlatStream(&f, format, { let mut __mm_s = String::new(); __mm_s.push_str(&*indent); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }, s, literal!("constructor"))?;
            s = IOStream::append(s, literal!(";\n\n"))?;
            let __pa2 = ::match_deref::match_deref! { match &(Function::typeNodeCache(NFInstNode::InstNode::borrow(var_field!((*complexTy).destructor, ComplexType::NFComplexType::EXTERNAL_OBJECT).clone())?, NFInstContext::FUNCTION.clone())?) {
                Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil } => __pa2.clone(),
                _ => return Err("pattern mismatch"),
            } };
            f = metamodelica::Own::own(__pa2);
            s = Function::toFlatStream(&f, format, { let mut __mm_s = String::new(); __mm_s.push_str(&*indent); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }, s, literal!("destructor"))?;
            s = IOStream::append(s, literal!(";\n\nend "))?;
            s = IOStream::append(s, name)?;
            s
        },
        Deref @ FUNCTION { r#fn: __ty_fn, .. } => Function::toFlatStream(metamodelica::AsArg::as_arg(&__ty_fn), format, indent, s, Util::makeQuotedIdentifier(AbsynUtil::pathString(NFInstNode::InstNode::scopePath(NFInstNode::InstNode::fromHandle(&__ty_fn.node)?, NFInstNode::InstNode::ScopeType::RELATIVE.clone(), false)?, literal!("."), true, false)?)?)?,
        _ => s,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(s)
}

pub(crate) fn typenameString(mut ty: &metamodelica::Ref<NFType>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match &**ty {
        ENUMERATION {
            typePath: __ty_typePath,
            ..
        } => AbsynUtil::pathString(__ty_typePath.clone(), literal!("."), true, false)?,
        _ => toString(ty)?,
    });
    Ok(r#str)
}

pub fn toDAE(mut ty: &metamodelica::Ref<NFType>, mut makeTypeVars: bool) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut daeTy: metamodelica::Ref<DAE::Type>;
    daeTy = (match &**ty {
        INTEGER => DAE::T_INTEGER_DEFAULT().clone(),
        REAL => DAE::T_REAL_DEFAULT().clone(),
        STRING => DAE::T_STRING_DEFAULT().clone(),
        BOOLEAN => DAE::T_BOOL_DEFAULT().clone(),
        ENUMERATION {
            literals: __ty_literals,
            typePath: __ty_typePath,
        } => metamodelica::Ref::new(DAE::Type::T_ENUMERATION {
            index: None,
            path: __ty_typePath.clone(),
            names: __ty_literals.clone(),
            literalVarLst: metamodelica::nil(),
            attributeLst: metamodelica::nil(),
        }),
        CLOCK => DAE::T_CLOCK_DEFAULT().clone(),
        ARRAY {
            dimensions: __ty_dimensions,
            elementType: __ty_elementType,
        } => metamodelica::Ref::new(DAE::Type::T_ARRAY {
            ty: toDAE(metamodelica::AsArg::as_arg(&__ty_elementType), makeTypeVars)?,
            dims: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Dimension>> = metamodelica::nil();
                for mut d in (__ty_dimensions.clone()).into_iter().cloned() {
                    let __x = Dimension::toDAE(&(d.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        }),
        TUPLE {
            names: __ty_names,
            types: __ty_types,
        } => metamodelica::Ref::new(DAE::Type::T_TUPLE {
            types: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
                for mut t in (__ty_types.clone()).into_iter().cloned() {
                    let __x = toDAE(&(t.clone()), true)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            names: __ty_names.clone(),
        }),
        FUNCTION {
            r#fn: __ty_fn,
            fnType: __ty_fnType,
        } => {
            (match __ty_fnType.clone() {
                FunctionType::FUNCTIONAL_PARAMETER => {
                    Function::makeDAEType(metamodelica::AsArg::as_arg(&__ty_fn), false)?
                }
                FunctionType::FUNCTION_REFERENCE => metamodelica::Ref::new(DAE::Type::T_FUNCTION_REFERENCE_FUNC {
                    builtin: Function::isBuiltin(metamodelica::AsArg::as_arg(&__ty_fn)),
                    functionType: Function::makeDAEType(metamodelica::AsArg::as_arg(&__ty_fn), false)?,
                }),
                FunctionType::FUNCTIONAL_VARIABLE => metamodelica::Ref::new(DAE::Type::T_FUNCTION_REFERENCE_VAR {
                    functionType: Function::makeDAEType(metamodelica::AsArg::as_arg(&__ty_fn), true)?,
                }),
            })
        }
        NORETCALL => DAE::T_NORETCALL_DEFAULT().clone(),
        UNKNOWN => DAE::T_UNKNOWN_DEFAULT().clone(),
        COMPLEX { .. } => {
            if (makeTypeVars) {
                NFInstNode::InstNode::toFullDAEType(complexNode(ty)?)?
            } else {
                NFInstNode::InstNode::toPartialDAEType(complexNode(ty)?)?
            }
        }
        METABOXED { ty: __ty_ty } => metamodelica::Ref::new(DAE::Type::T_METABOXED {
            ty: toDAE(metamodelica::AsArg::as_arg(&__ty_ty), true)?,
        }),
        POLYMORPHIC { name: __ty_name } => metamodelica::Ref::new(DAE::Type::T_METAPOLYMORPHIC {
            name: __ty_name.clone(),
        }),
        ANY => metamodelica::Ref::new(DAE::Type::T_ANYTYPE { anyClassType: None }),
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFType.toDAE"));
                    __mm_s.push_str(&*literal!(" got unknown type: "));
                    __mm_s.push_str(&*anyString(ty.clone()));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFType.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(daeTy)
}

pub(crate) fn subscript(
    mut ty: metamodelica::Ref<NFType>,
    mut subs: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut failOnError: bool,
) -> Result<metamodelica::Ref<NFType>> {
    let mut ty: metamodelica::Ref<NFType> = ty;
    let mut dim: metamodelica::Ref<Dimension::NFDimension>;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut subbed_dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
    let mut el_ty: metamodelica::Ref<NFType>;
    if (subs).is_empty() {
        return Ok(ty);
    }
    ty = (match &*ty {
        ARRAY { dimensions: dims, .. } if (!(failOnError) && ((subs).len() as i32) > ((dims).len() as i32)) => {
            crate::NFType::interned_UNKNOWN()
        }
        ARRAY {
            dimensions: __esc_dims, ..
        } => {
            dims = (*__esc_dims).clone();
            for mut sub in &**subs {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dims.clone()) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                dim = metamodelica::Own::own(__pa0);
                dims = metamodelica::Own::own(__pa1);
                subbed_dims = (match &*sub.clone() {
                    Subscript::INDEX { .. } => subbed_dims,
                    Subscript::SLICE { .. } => {
                        metamodelica::cons(Subscript::toDimension(metamodelica::AsArg::as_arg(&sub))?, subbed_dims)
                    }
                    Subscript::WHOLE => metamodelica::cons(dim, subbed_dims),
                    Subscript::SPLIT_INDEX { .. } => subbed_dims,
                    _ => {
                        Error::terminate(
                            {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("NFType.subscript"));
                                __mm_s.push_str(&*literal!(" got wrong subscript "));
                                __mm_s.push_str(&*Subscript::toString(metamodelica::AsArg::as_arg(&sub))?);
                                __mm_s.push_str(&*literal!("\n"));
                                ArcStr::from(__mm_s)
                            },
                            &(metamodelica::sourceInfo!("NFFrontEnd/NFType.mo")),
                        )?;
                        return Err("fail");
                    }
                });
            }
            el_ty = arrayElementType(&ty);
            if (!((subbed_dims).is_empty() && (dims).is_empty())) {
                metamodelica::Ref::new(NFType::ARRAY {
                    elementType: el_ty,
                    dimensions: listAppend(subbed_dims.reverse(), dims.clone()),
                })
            } else {
                el_ty
            }
        }
        CONDITIONAL_ARRAY {
            falseType: __ty_falseType,
            matchedBranch: __ty_matchedBranch,
            trueType: __ty_trueType,
        } => metamodelica::Ref::new(NFType::CONDITIONAL_ARRAY {
            trueType: subscript(__ty_trueType.clone(), subs, true)?,
            falseType: subscript(__ty_falseType.clone(), subs, true)?,
            matchedBranch: __ty_matchedBranch.clone(),
        }),
        METABOXED { ty: __ty_ty } => metamodelica::Ref::new(NFType::METABOXED {
            ty: subscript(__ty_ty.clone(), subs, true)?,
        }),
        UNKNOWN { .. } => ty,
        _ => {
            if failOnError {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFType.subscript"));
                        __mm_s.push_str(&*literal!(" got unsubscriptable type "));
                        __mm_s.push_str(&*toString(&ty)?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFType.mo")),
                )?;
                return Err("fail");
            }
            crate::NFType::interned_UNKNOWN()
        }
    });
    Ok(ty)
}

pub fn isEqual(mut ty1: &metamodelica::Ref<NFType>, mut ty2: &metamodelica::Ref<NFType>) -> Result<bool> {
    let mut equal: bool;
    if referenceEq(&*(&**ty1), &*(&**ty2)) {
        equal = true;
        return Ok(equal);
    }
    if metamodelica::valueConstructor((&*&**ty1))? != metamodelica::valueConstructor((&*&**ty2))? {
        equal = false;
        return Ok(equal);
    }
    equal = (::match_deref::match_deref! { match (ty1, ty2) {
        (Deref @ ENUMERATION { .. }, Deref @ ENUMERATION { .. }) => {
            List::isEqualOnTrue(var_field!((**ty1).literals, NFType::ENUMERATION).clone(), var_field!((**ty2).literals, NFType::ENUMERATION).clone(), &fnptr!(stringEq, ArcStr, ArcStr))?
        },
        (Deref @ ARRAY { .. }, Deref @ ARRAY { .. }) => {
            isEqual(var_field!((**ty1).elementType, NFType::ARRAY), var_field!((**ty2).elementType, NFType::ARRAY))? && List::isEqualOnTrue(var_field!((**ty1).dimensions, NFType::ARRAY).clone(), var_field!((**ty2).dimensions, NFType::ARRAY).clone(), &move |__a0: metamodelica::Ref<Dimension::NFDimension>, __a1: metamodelica::Ref<Dimension::NFDimension>| Dimension::isEqualKnown(&__a0, &__a1))?
        },
        (Deref @ CONDITIONAL_ARRAY { .. }, Deref @ CONDITIONAL_ARRAY { .. }) => {
            isEqual(var_field!((**ty1).trueType, NFType::CONDITIONAL_ARRAY), var_field!((**ty2).trueType, NFType::CONDITIONAL_ARRAY))? && isEqual(var_field!((**ty1).falseType, NFType::CONDITIONAL_ARRAY), var_field!((**ty2).falseType, NFType::CONDITIONAL_ARRAY))?
        },
        (Deref @ TUPLE { names: Some(names1), .. }, Deref @ TUPLE { names: Some(names2), .. }) => {
            List::isEqualOnTrue(names1.clone(), names2.clone(), &fnptr!(stringEq, ArcStr, ArcStr))? && List::isEqualOnTrue(var_field!((**ty1).types, NFType::TUPLE).clone(), var_field!((**ty2).types, NFType::TUPLE).clone(), &move |__a0: metamodelica::Ref<NFType>, __a1: metamodelica::Ref<NFType>| isEqual(&__a0, &__a1))?
        },
        (Deref @ TUPLE { names: None, .. }, Deref @ TUPLE { names: None, .. }) => {
            List::isEqualOnTrue(var_field!((**ty1).types, NFType::TUPLE).clone(), var_field!((**ty2).types, NFType::TUPLE).clone(), &move |__a0: metamodelica::Ref<NFType>, __a1: metamodelica::Ref<NFType>| isEqual(&__a0, &__a1))?
        },
        (Deref @ TUPLE { .. }, Deref @ TUPLE { .. }) => {
            false
        },
        (Deref @ COMPLEX { .. }, Deref @ COMPLEX { .. }) => {
            NFInstNode::InstNode::isSame(complexNode(ty1)?, complexNode(ty2)?)
        },
        (Deref @ UNTYPED { .. }, Deref @ UNTYPED { .. }) => {
            NFInstNode::InstNode::isSame(var_field!((**ty1).typeNode, NFType::UNTYPED).clone(), var_field!((**ty2).typeNode, NFType::UNTYPED).clone()) && Array::isEqualOnTrue(var_field!((**ty1).dimensions, NFType::UNTYPED).clone(), var_field!((**ty2).dimensions, NFType::UNTYPED).clone(), &move |__a0: metamodelica::Ref<Dimension::NFDimension>, __a1: metamodelica::Ref<Dimension::NFDimension>| Dimension::isEqualKnown(&__a0, &__a1))?
        },
        _ => {
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

pub(crate) fn hashContinue(mut ty: metamodelica::Ref<NFType>, mut hash: i32) -> Result<i32> {
    '__tco: loop {
        match &*ty {
            INTEGER => return Ok(stringHashDjb2Continue(&(literal!("Integer")), hash)),
            REAL => return Ok(stringHashDjb2Continue(&(literal!("Real")), hash)),
            STRING => return Ok(stringHashDjb2Continue(&(literal!("String")), hash)),
            BOOLEAN => return Ok(stringHashDjb2Continue(&(literal!("Boolean")), hash)),
            CLOCK => return Ok(stringHashDjb2Continue(&(literal!("Clock")), hash)),
            ENUMERATION {
                literals: __ty_literals,
                typePath: __ty_typePath,
            } => {
                if (__ty_literals).is_empty() {
                    hash = stringHashDjb2Continue(&(literal!("enumeration(:)")), hash);
                } else {
                    hash = stringHashDjb2Continue(&(literal!("enumeration")), hash);
                    hash = AbsynUtil::pathHashContinue(metamodelica::AsArg::as_arg(&__ty_typePath), hash);
                    hash = stringHashDjb2Continue(&(literal!("(")), hash);
                    for mut lit in &*__ty_literals.clone() {
                        hash = stringHashDjb2Continue(&lit, hash);
                        hash = stringHashDjb2Continue(&(literal!(", ")), hash);
                    }
                    hash = stringHashDjb2Continue(&(literal!(")")), hash);
                }
                return Ok(hash);
            }
            ARRAY {
                dimensions: __ty_dimensions,
                elementType: __ty_elementType,
            } => {
                hash = hashContinue(__ty_elementType.clone(), hash)?;
                hash = stringHashDjb2Continue(&(literal!("[")), hash);
                for mut dim in &*__ty_dimensions.clone() {
                    hash = stringHashDjb2Continue(&(Dimension::toString(metamodelica::AsArg::as_arg(&dim))?), hash);
                    hash = stringHashDjb2Continue(&(literal!(", ")), hash);
                }
                return Ok(stringHashDjb2Continue(&(literal!("]")), hash));
            }
            TUPLE { types: __ty_types, .. } => {
                hash = stringHashDjb2Continue(&(literal!("(")), hash);
                for mut t in &*__ty_types.clone() {
                    hash = hashContinue(t.clone(), hash)?;
                    hash = stringHashDjb2Continue(&(literal!(", ")), hash);
                }
                return Ok(stringHashDjb2Continue(&(literal!(")")), hash));
            }
            NORETCALL => return Ok(stringHashDjb2Continue(&(literal!("()")), hash)),
            UNKNOWN => return Ok(stringHashDjb2Continue(&(literal!("unknown()")), hash)),
            COMPLEX { .. } => {
                return Ok(AbsynUtil::pathHashContinue(
                    &(NFInstNode::InstNode::scopePath(
                        complexNode(&ty)?,
                        NFInstNode::InstNode::ScopeType::RELATIVE.clone(),
                        false,
                    )?),
                    hash,
                ));
            }
            FUNCTION { r#fn: __ty_fn, .. } => {
                return Ok(stringHashDjb2Continue(
                    &(Function::typeString(metamodelica::AsArg::as_arg(&__ty_fn))?),
                    hash,
                ));
            }
            METABOXED { ty: __ty_ty } => {
                (ty, hash) = (__ty_ty.clone(), hash);
                continue '__tco;
            }
            POLYMORPHIC { name: __ty_name } => return Ok(stringHashDjb2Continue(&__ty_name, hash)),
            ANY => return Ok(stringHashDjb2Continue(&(literal!("$ANY$")), hash)),
            CONDITIONAL_ARRAY {
                falseType: __ty_falseType,
                trueType: __ty_trueType,
                ..
            } => {
                hash = hashContinue(__ty_trueType.clone(), hash)?;
                {
                    (ty, hash) = (__ty_falseType.clone(), hash);
                    continue '__tco;
                }
            }
            UNTYPED {
                typeNode: __ty_typeNode,
                ..
            } => {
                hash = NFInstNode::InstNode::hashContinue(metamodelica::AsArg::as_arg(&__ty_typeNode), hash)?;
                hash = stringHashDjb2Continue(&(literal!("[")), hash);
                let __range0 = var_field!((*ty).dimensions, NFType::UNTYPED)
                    .clone()
                    .borrow()
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>();
                for mut dim in __range0 {
                    hash = stringHashDjb2Continue(&(Dimension::toString(&dim)?), hash);
                    hash = stringHashDjb2Continue(&(literal!(", ")), hash);
                }
                return Ok(stringHashDjb2Continue(&(literal!("]")), hash));
            }
            _ => return Ok(hash),
        }
    }
}

pub fn isDiscrete(mut ty: metamodelica::Ref<NFType>) -> Result<bool> {
    '__tco: loop {
        match &*ty {
            INTEGER { .. } => return Ok(true),
            STRING { .. } => return Ok(true),
            BOOLEAN { .. } => return Ok(true),
            ENUMERATION { .. } => return Ok(true),
            ARRAY {
                elementType: __ty_elementType,
                ..
            } => {
                ty = __ty_elementType.clone();
                continue '__tco;
            }
            CONDITIONAL_ARRAY {
                trueType: __ty_trueType,
                ..
            } => {
                ty = __ty_trueType.clone();
                continue '__tco;
            }
            FUNCTION { r#fn: __ty_fn, .. } => {
                ty = Function::returnType(metamodelica::AsArg::as_arg(&__ty_fn));
                continue '__tco;
            }
            _ => return Ok(false),
        }
    }
}

pub(crate) fn lookupRecordFieldType(
    mut name: &ArcStr,
    mut recordType: &metamodelica::Ref<NFType>,
) -> Result<metamodelica::Ref<NFType>> {
    let mut fieldType: metamodelica::Ref<NFType>;
    fieldType = (match &**recordType {
        COMPLEX { .. } => NFInstNode::InstNode::getType(
            (Class::lookupElement(name.clone(), NFInstNode::InstNode::getClass(complexNode(recordType)?)?)?).0,
        )?,
        ARRAY {
            dimensions: __recordType_dimensions,
            elementType: __recordType_elementType,
        } => liftArrayLeftList(
            lookupRecordFieldType(name, metamodelica::AsArg::as_arg(&__recordType_elementType))?,
            metamodelica::AsArg::as_arg(&__recordType_dimensions),
        ),
        CONDITIONAL_ARRAY {
            falseType: __recordType_falseType,
            matchedBranch: __recordType_matchedBranch,
            trueType: __recordType_trueType,
        } => metamodelica::Ref::new(NFType::CONDITIONAL_ARRAY {
            trueType: lookupRecordFieldType(name, metamodelica::AsArg::as_arg(&__recordType_trueType))?,
            falseType: lookupRecordFieldType(name, metamodelica::AsArg::as_arg(&__recordType_falseType))?,
            matchedBranch: __recordType_matchedBranch.clone(),
        }),
        _ => return Err("match: no arm matched"),
    });
    Ok(fieldType)
}

pub fn recordFieldCount(mut recordType: &metamodelica::Ref<NFType>) -> i32 {
    let mut fieldCount: i32;
    let mut fields: metamodelica::Array<metamodelica::Ref<Record::Field::Field>>;
    fieldCount = (::match_deref::match_deref! { match recordType {
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::RECORD { fields: __esc_fields, .. }, .. } => {
            fields = (*__esc_fields).clone();
            metamodelica::arrayLength(fields.clone())
        },
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    fieldCount
}

pub fn recordFields(
    mut recordType: &metamodelica::Ref<NFType>,
) -> metamodelica::List<metamodelica::Ref<Record::Field::Field>> {
    let mut field_lst: metamodelica::List<metamodelica::Ref<Record::Field::Field>>;
    field_lst = (::match_deref::match_deref! { match recordType {
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::RECORD { fields, .. }, .. } => {
            fields.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    field_lst
}

pub(crate) fn setRecordFields(
    mut field_lst: metamodelica::List<metamodelica::Ref<Record::Field::Field>>,
    mut recordType: metamodelica::Ref<NFType>,
) -> Result<metamodelica::Ref<NFType>> {
    let mut recordType: metamodelica::Ref<NFType> = recordType;
    recordType = ({
        let mut fields: metamodelica::Array<metamodelica::Ref<Record::Field::Field>> =
            metamodelica::arrayFromVec(field_lst.into_iter().cloned().collect());
        (::match_deref::match_deref! { match &(recordType.clone()) {
            Deref @ COMPLEX { complexTy: Deref @ ComplexType::RECORD { constructor: rec_node, .. }, cls: __recordType_cls } => {
                let mut indexMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>>;
                indexMap = UnorderedMap::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), metamodelica::arrayLength(fields.clone()));
                updateRecordFieldsIndexMap(fields.clone(), indexMap.clone())?;
                metamodelica::Ref::new(NFType::COMPLEX { cls: __recordType_cls.clone(), complexTy: metamodelica::Ref::new(ComplexType::NFComplexType::RECORD { constructor: rec_node.clone(), fields: fields.clone(), indexMap: indexMap }) })
            },
            _ => {
                recordType
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    });
    Ok(recordType)
}

pub(crate) fn updateRecordFieldsIndexMap(
    mut fields: metamodelica::Array<metamodelica::Ref<Record::Field::Field>>,
    mut indexMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>>,
) -> Result<()> {
    for mut i in 1..=metamodelica::arrayLength(fields.clone()) {
        UnorderedMap::add(
            Record::Field::name(
                &({
                    let __elt = (*metamodelica::index_checked(&fields.borrow(), i)?).clone();
                    __elt
                }),
            ),
            i,
            indexMap.clone(),
        )?;
    }
    Ok(())
}

pub fn tupleFieldCount(mut tupleType: &metamodelica::Ref<NFType>) -> i32 {
    let mut fieldCount: i32;
    fieldCount = (match &**tupleType {
        TUPLE {
            types: __tupleType_types,
            ..
        } => ((__tupleType_types).len() as i32),
        _ => 0,
    });
    fieldCount
}

pub(crate) fn enumName(mut ty: &metamodelica::Ref<NFType>) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut name: metamodelica::Ref<Absyn::Path>;
    let __pa0 = ::match_deref::match_deref! { match &((*ty)) {
        Deref @ ENUMERATION { typePath: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    name = metamodelica::Own::own(__pa0);
    Ok(name)
}

pub(crate) fn enumSize(mut ty: &metamodelica::Ref<NFType>) -> Result<i32> {
    let mut size: i32;
    let mut literals: metamodelica::List<ArcStr>;
    let __pa0 = ::match_deref::match_deref! { match &((*ty)) {
        Deref @ ENUMERATION { literals: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    literals = metamodelica::Own::own(__pa0);
    size = ((literals).len() as i32);
    Ok(size)
}

pub(crate) fn r#box(mut ty: &metamodelica::Ref<NFType>) -> metamodelica::Ref<NFType> {
    let mut boxedType: metamodelica::Ref<NFType>;
    boxedType = (match &**ty {
        STRING { .. } => ty.clone(),
        TUPLE {
            names: __ty_names,
            types: __ty_types,
        } => metamodelica::Ref::new(NFType::TUPLE {
            types: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<NFType>> = metamodelica::nil();
                for mut t in (__ty_types.clone()).into_iter().cloned() {
                    let __x = r#box(&(t.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            names: __ty_names.clone(),
        }),
        FUNCTION { .. } => ty.clone(),
        METABOXED { .. } => ty.clone(),
        POLYMORPHIC { .. } => ty.clone(),
        ANY { .. } => ty.clone(),
        CONDITIONAL_ARRAY {
            falseType: __ty_falseType,
            matchedBranch: __ty_matchedBranch,
            trueType: __ty_trueType,
        } => metamodelica::Ref::new(NFType::CONDITIONAL_ARRAY {
            trueType: r#box(metamodelica::AsArg::as_arg(&__ty_trueType)),
            falseType: r#box(metamodelica::AsArg::as_arg(&__ty_falseType)),
            matchedBranch: __ty_matchedBranch.clone(),
        }),
        _ => metamodelica::Ref::new(NFType::METABOXED { ty: ty.clone() }),
    });
    boxedType
}

pub(crate) fn unbox(mut ty: metamodelica::Ref<NFType>) -> metamodelica::Ref<NFType> {
    let mut unboxedType: metamodelica::Ref<NFType>;
    unboxedType = (match &*ty {
        METABOXED { ty: __ty_ty } => __ty_ty.clone(),
        _ => ty,
    });
    unboxedType
}

pub(crate) fn isBoxed(mut ty: &metamodelica::Ref<NFType>) -> bool {
    let mut isBoxed: bool;
    isBoxed = (match &**ty {
        METABOXED { .. } => true,
        _ => false,
    });
    isBoxed
}

pub(crate) fn sizeType(mut arrayTy: metamodelica::Ref<NFType>) -> metamodelica::Ref<NFType> {
    let mut sizeTy: metamodelica::Ref<NFType>;
    if isUnknown(&arrayTy) {
        sizeTy = crate::NFType::interned_UNKNOWN();
    } else {
        sizeTy = metamodelica::Ref::new(NFType::ARRAY {
            elementType: crate::NFType::interned_INTEGER(),
            dimensions: list![Dimension::fromInteger(
                dimensionCount(arrayTy),
                NFPrefixes::Variability::CONSTANT.clone()
            )],
        });
    }
    sizeTy
}

pub(crate) fn simplify(mut ty: metamodelica::Ref<NFType>) -> Result<metamodelica::Ref<NFType>> {
    let mut ty: metamodelica::Ref<NFType> = ty;
    let () = (match &*ty {
        ARRAY {
            dimensions: __ty_dimensions,
            ..
        } => {
            assign_variant_field!(ty => NFType::ARRAY; dimensions = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
                for mut d in (__ty_dimensions.clone()).into_iter().cloned() {
                    let __x = Dimension::simplify(d.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        _ => (),
    });
    Ok(ty)
}

pub fn sizeOf(mut ty: &metamodelica::Ref<NFType>, mut resize: bool) -> Result<i32> {
    pub(crate) fn fold_comp_size(mut comp: metamodelica::Ref<InstNode::InstNode>, mut sz: i32) -> Result<i32> {
        let mut outSize: i32 = sz + sizeOf(&(NFInstNode::InstNode::getType(comp.clone())?), false)?;
        Ok(outSize)
    }

    let mut sz: i32;
    sz = (::match_deref::match_deref! { match ty {
        Deref @ INTEGER { .. } => 1,
        Deref @ REAL { .. } => 1,
        Deref @ STRING { .. } => 1,
        Deref @ BOOLEAN { .. } => 1,
        Deref @ CLOCK { .. } => 1,
        Deref @ ENUMERATION { .. } => 1,
        Deref @ ARRAY { dimensions: __ty_dimensions, elementType: __ty_elementType } => sizeOf(metamodelica::AsArg::as_arg(&__ty_elementType), false)? * Dimension::sizesProduct(__ty_dimensions.clone(), resize)?,
        Deref @ TUPLE { types: __ty_types, .. } => ({
        let mut __acc: i32 = 0;
        for mut t in (__ty_types.clone()).into_iter().cloned() {
            let __x = sizeOf(&(t.clone()), false)?;
            __acc += __x;
        }
        __acc
    }),
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { .. }, .. } => 1,
        Deref @ COMPLEX { complexTy: Deref @ ComplexType::RECORD { .. }, .. } => ClassTree::foldComponents(&(Class::classTree(NFInstNode::InstNode::getClass(complexNode(ty)?)?)?), &fold_comp_size, 0)?,
        Deref @ COMPLEX { .. } => 1,
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(sz)
}

pub fn complexSize<'__b>(mut ty: &'__b metamodelica::Ref<NFType>, mut resize: bool) -> Result<Option<i32>> {
    '__tco: loop {
        ::match_deref::match_deref! { match ty {
            Deref @ ARRAY { .. } => { (ty, resize) = (var_field!((**ty).elementType, NFType::ARRAY), resize); continue '__tco; },
            Deref @ COMPLEX { complexTy: Deref @ ComplexType::RECORD { .. }, .. } => return Ok(Some(sizeOf(ty, resize)?)),
            _ => return Ok(None),
            _ => return Err("match: no arm matched"),
        } }
    }
}
