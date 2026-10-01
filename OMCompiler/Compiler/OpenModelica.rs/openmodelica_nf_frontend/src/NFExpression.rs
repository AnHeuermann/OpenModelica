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
use crate::NFBackendExtension::BackendInfo;
use crate::NFBackendExtension::VariableKind;
use crate::NFBinding as Binding;
use crate::NFBuiltin as Builtin;
use crate::NFBuiltinCall as BuiltinCall;
use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFClassTree::ClassTree;
use crate::NFClockKind as ClockKind;
use crate::NFComplexType as ComplexType;
use crate::NFComponentRef as ComponentRef;
use crate::NFComponentRef::Origin;
use crate::NFDimension as Dimension;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpressionIterator as ExpressionIterator;
use crate::NFFunction as Function;
use crate::NFInstContext as InstContext;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes as Prefixes;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFRangeIterator as RangeIterator;
use crate::NFRecord as Record;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFTypeCheck as TypeCheck;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_ast::Absyn::Path;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::JSON;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::Pointer;
use openmodelica_util_datatypes_basic::PointerWeak;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NFExpression {
    INTEGER {
        value: i32,
    },
    REAL {
        value: metamodelica::Real,
    },
    STRING {
        value: ArcStr,
    },
    BOOLEAN {
        value: bool,
    },
    ENUM_LITERAL {
        ty: metamodelica::Ref<Type::NFType>,
        name: ArcStr,
        index: i32,
    },
    /// Clock constructors
    CLKCONST {
        /// Clock kinds
        clk: metamodelica::Ref<ClockKind::NFClockKind>,
    },
    CREF {
        ty: metamodelica::Ref<Type::NFType>,
        cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    },
    /// Represents a type used as a range, e.g. Boolean.
    TYPENAME {
        ty: metamodelica::Ref<Type::NFType>,
    },
    ARRAY {
        ty: metamodelica::Ref<Type::NFType>,
        elements: metamodelica::Array<metamodelica::Ref<NFExpression>>,
        /// True if the array is known to only contain literal expressions.
        literal: bool,
    },
    /// The array concatentation operator [a,b; c,d]; this should be removed during type-checking
    MATRIX {
        elements: metamodelica::List<metamodelica::List<metamodelica::Ref<NFExpression>>>,
    },
    RANGE {
        ty: metamodelica::Ref<Type::NFType>,
        start: metamodelica::Ref<NFExpression>,
        step: Option<metamodelica::Ref<NFExpression>>,
        stop: metamodelica::Ref<NFExpression>,
    },
    TUPLE {
        ty: metamodelica::Ref<Type::NFType>,
        elements: metamodelica::List<metamodelica::Ref<NFExpression>>,
    },
    RECORD {
        path: metamodelica::Ref<Path>,
        ty: metamodelica::Ref<Type::NFType>,
        elements: metamodelica::List<metamodelica::Ref<NFExpression>>,
    },
    CALL {
        call: metamodelica::Ref<Call::NFCall>,
    },
    SIZE {
        exp: metamodelica::Ref<NFExpression>,
        dimIndex: Option<metamodelica::Ref<NFExpression>>,
    },
    END,
    /// Binary operations, e.g. a+4
    BINARY {
        exp1: metamodelica::Ref<NFExpression>,
        operator: metamodelica::Ref<Operator::NFOperator>,
        exp2: metamodelica::Ref<NFExpression>,
    },
    /// Unary operations, -(4x)
    UNARY {
        operator: metamodelica::Ref<Operator::NFOperator>,
        exp: metamodelica::Ref<NFExpression>,
    },
    /// Logical binary operations: and, or
    LBINARY {
        exp1: metamodelica::Ref<NFExpression>,
        operator: metamodelica::Ref<Operator::NFOperator>,
        exp2: metamodelica::Ref<NFExpression>,
    },
    /// Logical unary operations: not
    LUNARY {
        operator: metamodelica::Ref<Operator::NFOperator>,
        exp: metamodelica::Ref<NFExpression>,
    },
    /// Relation, e.g. a <= 0
    RELATION {
        exp1: metamodelica::Ref<NFExpression>,
        operator: metamodelica::Ref<Operator::NFOperator>,
        exp2: metamodelica::Ref<NFExpression>,
        /// index for event codegen
        index: i32,
    },
    /// Multary expressions with the same operator, e.g. a+b+c
    ///    An empty list has to be interpreted as the neutral element of the operator space
    MULTARY {
        /// arguments that are chained with the operator (+, *)
        arguments: metamodelica::List<metamodelica::Ref<NFExpression>>,
        /// arguments that are chained with the inverse operator (-, :)
        inv_arguments: metamodelica::List<metamodelica::Ref<NFExpression>>,
        /// Can only be + or * (commutative)
        operator: metamodelica::Ref<Operator::NFOperator>,
    },
    IF {
        ty: metamodelica::Ref<Type::NFType>,
        condition: metamodelica::Ref<NFExpression>,
        trueBranch: metamodelica::Ref<NFExpression>,
        falseBranch: metamodelica::Ref<NFExpression>,
    },
    CAST {
        ty: metamodelica::Ref<Type::NFType>,
        exp: metamodelica::Ref<NFExpression>,
    },
    /// MetaModelica boxed value
    BOX {
        exp: metamodelica::Ref<NFExpression>,
    },
    /// MetaModelica value unboxing (similar to a cast)
    UNBOX {
        exp: metamodelica::Ref<NFExpression>,
        ty: metamodelica::Ref<Type::NFType>,
    },
    SUBSCRIPTED_EXP {
        exp: metamodelica::Ref<NFExpression>,
        subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
        ty: metamodelica::Ref<Type::NFType>,
        split: bool,
    },
    TUPLE_ELEMENT {
        tupleExp: metamodelica::Ref<NFExpression>,
        index: i32,
        ty: metamodelica::Ref<Type::NFType>,
    },
    RECORD_ELEMENT {
        recordExp: metamodelica::Ref<NFExpression>,
        index: i32,
        fieldName: ArcStr,
        ty: metamodelica::Ref<Type::NFType>,
    },
    MUTABLE {
        exp: Mutable::Mutable<metamodelica::Ref<NFExpression>>,
    },
    EMPTY {
        ty: metamodelica::Ref<Type::NFType>,
    },
    PARTIAL_FUNCTION_APPLICATION {
        r#fn: metamodelica::Ref<ComponentRef::NFComponentRef>,
        args: metamodelica::List<metamodelica::Ref<NFExpression>>,
        argNames: metamodelica::List<ArcStr>,
        ty: metamodelica::Ref<Type::NFType>,
    },
    FILENAME {
        filename: ArcStr,
    },
    /// Before code generation, we make a pass that replaces constant literals
    ///    with a SHARED_LITERAL expression. Any immutable type can be shared:
    ///    basic MetaModelica types and Modelica strings are fine. There is no point
    ///    to share Real, Integer, Boolean or Enum though.
    SHARED_LITERAL {
        /// A unique indexing that can be used to point to a single shared literal in generated code
        index: i32,
        /// For printing strings, code generators that do not support this kind of literal, or for getting the type in case the code generator needs that
        exp: metamodelica::Ref<NFExpression>,
    },
    INSTANCE_NAME {
        scope: metamodelica::Ref<InstNode::InstNode>,
    },
}
impl metamodelica::gc::MMTrace for NFExpression {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFExpression::INTEGER { value } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            NFExpression::REAL { value } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            NFExpression::STRING { value } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            NFExpression::BOOLEAN { value } => {
                metamodelica::gc::MMTrace::mm_accept(value, __mmv)?;
                Ok(())
            }
            NFExpression::ENUM_LITERAL { ty, name, index } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                Ok(())
            }
            NFExpression::CLKCONST { clk } => {
                metamodelica::gc::MMTrace::mm_accept(clk, __mmv)?;
                Ok(())
            }
            NFExpression::CREF { ty, cref } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cref, __mmv)?;
                Ok(())
            }
            NFExpression::TYPENAME { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            NFExpression::ARRAY { ty, elements, literal } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(elements, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(literal, __mmv)?;
                Ok(())
            }
            NFExpression::MATRIX { elements } => {
                metamodelica::gc::MMTrace::mm_accept(elements, __mmv)?;
                Ok(())
            }
            NFExpression::RANGE { ty, start, step, stop } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(start, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(step, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(stop, __mmv)?;
                Ok(())
            }
            NFExpression::TUPLE { ty, elements } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(elements, __mmv)?;
                Ok(())
            }
            NFExpression::RECORD { path, ty, elements } => {
                metamodelica::gc::MMTrace::mm_accept(path, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(elements, __mmv)?;
                Ok(())
            }
            NFExpression::CALL { call } => {
                metamodelica::gc::MMTrace::mm_accept(call, __mmv)?;
                Ok(())
            }
            NFExpression::SIZE { exp, dimIndex } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dimIndex, __mmv)?;
                Ok(())
            }
            NFExpression::END => Ok(()),
            NFExpression::BINARY { exp1, operator, exp2 } => {
                metamodelica::gc::MMTrace::mm_accept(exp1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(operator, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp2, __mmv)?;
                Ok(())
            }
            NFExpression::UNARY { operator, exp } => {
                metamodelica::gc::MMTrace::mm_accept(operator, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            NFExpression::LBINARY { exp1, operator, exp2 } => {
                metamodelica::gc::MMTrace::mm_accept(exp1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(operator, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp2, __mmv)?;
                Ok(())
            }
            NFExpression::LUNARY { operator, exp } => {
                metamodelica::gc::MMTrace::mm_accept(operator, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            NFExpression::RELATION {
                exp1,
                operator,
                exp2,
                index,
            } => {
                metamodelica::gc::MMTrace::mm_accept(exp1, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(operator, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp2, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                Ok(())
            }
            NFExpression::MULTARY {
                arguments,
                inv_arguments,
                operator,
            } => {
                metamodelica::gc::MMTrace::mm_accept(arguments, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(inv_arguments, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(operator, __mmv)?;
                Ok(())
            }
            NFExpression::IF {
                ty,
                condition,
                trueBranch,
                falseBranch,
            } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(trueBranch, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(falseBranch, __mmv)?;
                Ok(())
            }
            NFExpression::CAST { ty, exp } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            NFExpression::BOX { exp } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            NFExpression::UNBOX { exp, ty } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            NFExpression::SUBSCRIPTED_EXP {
                exp,
                subscripts,
                ty,
                split,
            } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(subscripts, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(split, __mmv)?;
                Ok(())
            }
            NFExpression::TUPLE_ELEMENT { tupleExp, index, ty } => {
                metamodelica::gc::MMTrace::mm_accept(tupleExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            NFExpression::RECORD_ELEMENT {
                recordExp,
                index,
                fieldName,
                ty,
            } => {
                metamodelica::gc::MMTrace::mm_accept(recordExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fieldName, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            NFExpression::MUTABLE { exp } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            NFExpression::EMPTY { ty } => {
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            NFExpression::PARTIAL_FUNCTION_APPLICATION {
                r#fn,
                args,
                argNames,
                ty,
            } => {
                metamodelica::gc::MMTrace::mm_accept(r#fn, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(args, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(argNames, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                Ok(())
            }
            NFExpression::FILENAME { filename } => {
                metamodelica::gc::MMTrace::mm_accept(filename, __mmv)?;
                Ok(())
            }
            NFExpression::SHARED_LITERAL { index, exp } => {
                metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                Ok(())
            }
            NFExpression::INSTANCE_NAME { scope } => {
                metamodelica::gc::MMTrace::mm_accept(scope, __mmv)?;
                Ok(())
            }
        }
    }
}
impl NFExpression {
    pub fn interned_END() -> metamodelica::Ref<NFExpression> {
        thread_local! {
            static INTERNED: metamodelica::Ref<NFExpression> = metamodelica::Ref::new(NFExpression::END);
        }
        INTERNED.with(|i| i.clone())
    }
}
pub fn interned_END() -> metamodelica::Ref<NFExpression> {
    NFExpression::interned_END()
}
impl Default for NFExpression {
    fn default() -> Self {
        Self::END
    }
}
pub use self::NFExpression::{
    ARRAY, BINARY, BOOLEAN, BOX, CALL, CAST, CLKCONST, CREF, EMPTY, END, ENUM_LITERAL, FILENAME, IF, INSTANCE_NAME,
    INTEGER, LBINARY, LUNARY, MATRIX, MULTARY, MUTABLE, PARTIAL_FUNCTION_APPLICATION, RANGE, REAL, RECORD,
    RECORD_ELEMENT, RELATION, SHARED_LITERAL, SIZE, STRING, SUBSCRIPTED_EXP, TUPLE, TUPLE_ELEMENT, TYPENAME, UNARY,
    UNBOX,
};
pub(crate) fn isArray(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut isArray: bool;
    isArray = (match &**exp {
        ARRAY { .. } => true,
        _ => false,
    });
    isArray
}

pub(crate) fn isEmptyArray(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut emptyArray: bool;
    emptyArray = (match &**exp {
        ARRAY { .. } => var_field!((**exp).elements, NFExpression::ARRAY)
            .clone()
            .borrow()
            .is_empty(),
        _ => false,
    });
    emptyArray
}

pub(crate) fn isVector(mut exp: &metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut res: bool;
    res = (match &**exp {
        ARRAY { ty: __exp_ty, .. } => Type::isVector(metamodelica::AsArg::as_arg(&__exp_ty))?,
        _ => false,
    });
    Ok(res)
}

pub fn isCref(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut isCref: bool;
    isCref = (match &**exp {
        CREF { .. } => true,
        _ => false,
    });
    isCref
}

pub(crate) fn isFunctionInputCref(mut exp: &metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut res: bool;
    res = (match &**exp {
        CREF { cref: __exp_cref, .. } => {
            ComponentRef::isInput(&(ComponentRef::last(metamodelica::AsArg::as_arg(&__exp_cref))))?
        }
        _ => false,
    });
    Ok(res)
}

pub fn isWildCref(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut wild: bool;
    wild = (::match_deref::match_deref! { match exp {
        Deref @ CREF { cref: Deref @ ComponentRef::WILD, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    wild
}

pub fn isCall(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut isCall: bool;
    isCall = (match &**exp {
        CALL { .. } => true,
        _ => false,
    });
    isCall
}

pub(crate) fn isImpureCall(mut exp: &metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut isImpure: bool;
    isImpure = (match &**exp {
        CALL { call: __exp_call } => Call::isImpure(metamodelica::AsArg::as_arg(&__exp_call))?,
        _ => false,
    });
    Ok(isImpure)
}

pub(crate) fn isExternalCall(mut exp: &metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut res: bool;
    res = (match &**exp {
        CALL { call: __exp_call } => Call::isExternal(metamodelica::AsArg::as_arg(&__exp_call))?,
        _ => false,
    });
    Ok(res)
}

pub fn isCallNamed(mut exp: &metamodelica::Ref<NFExpression>, mut name: &ArcStr) -> Result<bool> {
    let mut res: bool;
    res = (match &**exp {
        CALL { call: __exp_call } => Call::isNamed(metamodelica::AsArg::as_arg(&__exp_call), name)?,
        _ => false,
    });
    Ok(res)
}

pub(crate) fn isConnectionCall(mut exp: &metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut isConnection: bool;
    isConnection = (match &**exp {
        CALL { call: __exp_call } => {
            Call::isConnectionsOperator(metamodelica::AsArg::as_arg(&__exp_call))
                || Call::isStreamOperator(metamodelica::AsArg::as_arg(&__exp_call))?
                || Call::isCardinality(metamodelica::AsArg::as_arg(&__exp_call))?
        }
        _ => false,
    });
    Ok(isConnection)
}

pub fn isTrue(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut isTrue: bool;
    isTrue = (match &**exp {
        BOOLEAN { value: true } => true,
        _ => false,
    });
    isTrue
}

pub fn isAllTrue(mut exp: metamodelica::Ref<NFExpression>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(&*exp) {
            Deref @ BOOLEAN { value: true } => {
                return Ok(true)
            },
            Deref @ ARRAY { .. } => {
                return Ok(Array::all(var_field!((*exp).elements, NFExpression::ARRAY).clone(), &isAllTrue)?)
            },
            Deref @ CALL { call: Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { exp: e, .. } } => {
                { exp = e.clone(); continue '__tco; }
            },
            _ => {
                return Ok(false)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn isFalse(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut isTrue: bool;
    isTrue = (match &**exp {
        BOOLEAN { value: false } => true,
        _ => false,
    });
    isTrue
}

pub fn isTrivialCref(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match exp {
        Deref @ CREF { .. } => true,
        Deref @ UNARY { exp: Deref @ CREF { .. }, .. } => true,
        Deref @ LUNARY { exp: Deref @ CREF { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn hash(mut exp: metamodelica::Ref<NFExpression>) -> Result<i32> {
    let mut hash: i32 = hashContinue(exp.clone(), Util::HASH_SEED.clone())?;
    Ok(hash)
}

pub(crate) fn hashContinue(mut exp: metamodelica::Ref<NFExpression>, mut hash: i32) -> Result<i32> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(&*exp) {
            Deref @ INTEGER { value: __exp_value } => {
                return Ok(intHashDjb2Continue(__exp_value.clone(), hash))
            },
            Deref @ REAL { value: __exp_value } => {
                return Ok(stringHashDjb2Continue(&(realString(__exp_value.clone())), hash))
            },
            Deref @ STRING { value: __exp_value } => {
                return Ok(stringHashDjb2Continue(&__exp_value, hash))
            },
            Deref @ BOOLEAN { value: __exp_value } => {
                return Ok(stringHashDjb2Continue(&(boolString(__exp_value.clone())), hash))
            },
            Deref @ ENUM_LITERAL { ty: Deref @ Type::ENUMERATION { typePath: path, .. }, name: __exp_name, .. } => {
                hash = AbsynUtil::pathHashContinue(metamodelica::AsArg::as_arg(&path), hash);
                hash = stringHashDjb2Continue(&(literal!(".")), hash);
                return Ok(stringHashDjb2Continue(&__exp_name, hash))
            },
            Deref @ CLKCONST { clk: __exp_clk } => {
                return Ok(ClockKind::hashContinue(metamodelica::AsArg::as_arg(&__exp_clk), hash)?)
            },
            Deref @ CREF { cref: __exp_cref, .. } => {
                return Ok(ComponentRef::hashContinue(metamodelica::AsArg::as_arg(&__exp_cref), false, hash)?)
            },
            Deref @ TYPENAME { ty: __exp_ty } => {
                return Ok(Type::hashContinue(Type::arrayElementType(metamodelica::AsArg::as_arg(&__exp_ty)), hash)?)
            },
            Deref @ ARRAY { .. } => {
                hash = stringHashDjb2Continue(&(literal!("{")), hash);
                let __range0 = var_field!((*exp).elements, NFExpression::ARRAY).clone().borrow().iter().cloned().collect::<Vec<_>>();
                for mut e in __range0 {
                    hash = hashContinue(e, hash)?;
                    hash = stringHashDjb2Continue(&(literal!(", ")), hash);
                }
                return Ok(stringHashDjb2Continue(&(literal!("}")), hash))
            },
            Deref @ MATRIX { elements: __exp_elements } => {
                hash = stringHashDjb2Continue(&(literal!("[")), hash);
                for mut el in &*__exp_elements.clone() {
                    for mut e in &*el.clone() {
                        hash = hashContinue(e.clone(), hash)?;
                        hash = stringHashDjb2Continue(&(literal!(", ")), hash);
                    }
                    hash = stringHashDjb2Continue(&(literal!("; ")), hash);
                }
                return Ok(stringHashDjb2Continue(&(literal!("]")), hash))
            },
            Deref @ RANGE { start: __exp_start, step: __exp_step, stop: __exp_stop, .. } => {
                hash = hashContinue(__exp_start.clone(), hash)?;
                hash = stringHashDjb2Continue(&(literal!(":")), hash);
                if (__exp_step).is_some() {
                    hash = hashContinue(Util::getOption(__exp_step.clone())?, hash)?;
                    hash = stringHashDjb2Continue(&(literal!(":")), hash);
                }
                { (exp, hash) = (__exp_stop.clone(), hash); continue '__tco; }
            },
            Deref @ TUPLE { elements: __exp_elements, .. } => {
                hash = stringHashDjb2Continue(&(literal!("(")), hash);
                for mut e in &*__exp_elements.clone() {
                    hash = hashContinue(e.clone(), hash)?;
                    hash = stringHashDjb2Continue(&(literal!(", ")), hash);
                }
                return Ok(stringHashDjb2Continue(&(literal!(")")), hash))
            },
            Deref @ RECORD { elements: __exp_elements, path: __exp_path, .. } => {
                hash = AbsynUtil::pathHashContinue(metamodelica::AsArg::as_arg(&__exp_path), hash);
                hash = stringHashDjb2Continue(&(literal!("(")), hash);
                for mut e in &*__exp_elements.clone() {
                    hash = hashContinue(e.clone(), hash)?;
                    hash = stringHashDjb2Continue(&(literal!(", ")), hash);
                }
                return Ok(stringHashDjb2Continue(&(literal!(")")), hash))
            },
            Deref @ CALL { call: __exp_call } => {
                return Ok(stringHashDjb2Continue(&(Call::toString(metamodelica::AsArg::as_arg(&__exp_call))?), hash))
            },
            Deref @ SIZE { dimIndex: __exp_dimIndex, exp: __exp_exp } => {
                hash = stringHashDjb2Continue(&(literal!("size(")), hash);
                hash = hashContinue(__exp_exp.clone(), hash)?;
                if (__exp_dimIndex).is_some() {
                    hash = stringHashDjb2Continue(&(literal!(", ")), hash);
                    hash = hashContinue(Util::getOption(__exp_dimIndex.clone())?, hash)?;
                }
                return Ok(stringHashDjb2Continue(&(literal!(")")), hash))
            },
            Deref @ END { .. } => {
                return Ok(stringHashDjb2Continue(&(literal!("end")), hash))
            },
            Deref @ BINARY { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator } => {
                hash = hashContinue(__exp_exp1.clone(), hash)?;
                hash = stringHashDjb2Continue(&(Operator::symbol(metamodelica::AsArg::as_arg(&__exp_operator), &(literal!(" ")))?), hash);
                { (exp, hash) = (__exp_exp2.clone(), hash); continue '__tco; }
            },
            Deref @ UNARY { exp: __exp_exp, operator: __exp_operator } => {
                hash = stringHashDjb2Continue(&(Operator::symbol(metamodelica::AsArg::as_arg(&__exp_operator), &(literal!("")))?), hash);
                { (exp, hash) = (__exp_exp.clone(), hash); continue '__tco; }
            },
            Deref @ LBINARY { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator } => {
                hash = hashContinue(__exp_exp1.clone(), hash)?;
                hash = stringHashDjb2Continue(&(Operator::symbol(metamodelica::AsArg::as_arg(&__exp_operator), &(literal!(" ")))?), hash);
                { (exp, hash) = (__exp_exp2.clone(), hash); continue '__tco; }
            },
            Deref @ LUNARY { exp: __exp_exp, operator: __exp_operator } => {
                hash = stringHashDjb2Continue(&(Operator::symbol(metamodelica::AsArg::as_arg(&__exp_operator), &(literal!("")))?), hash);
                hash = stringHashDjb2Continue(&(literal!(" ")), hash);
                { (exp, hash) = (__exp_exp.clone(), hash); continue '__tco; }
            },
            Deref @ RELATION { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator, .. } => {
                hash = hashContinue(__exp_exp1.clone(), hash)?;
                hash = stringHashDjb2Continue(&(Operator::symbol(metamodelica::AsArg::as_arg(&__exp_operator), &(literal!(" ")))?), hash);
                { (exp, hash) = (__exp_exp2.clone(), hash); continue '__tco; }
            },
            Deref @ MULTARY { arguments: __exp_arguments, inv_arguments: __exp_inv_arguments, operator: __exp_operator } => {
                hash = stringHashDjb2Continue(&(literal!("(")), hash);
                for mut e in &*__exp_arguments.clone() {
                    hash = stringHashDjb2Continue(&(Operator::symbol(metamodelica::AsArg::as_arg(&__exp_operator), &(literal!(" ")))?), hash);
                    hash = hashContinue(e.clone(), hash)?;
                }
                hash = stringHashDjb2Continue(&(literal!(")")), hash);
                hash = stringHashDjb2Continue(&(Operator::symbol(&(Operator::invert(__exp_operator.clone())?), &(literal!(" ")))?), hash);
                hash = stringHashDjb2Continue(&(literal!("(")), hash);
                for mut e in &*__exp_inv_arguments.clone() {
                    hash = stringHashDjb2Continue(&(Operator::symbol(metamodelica::AsArg::as_arg(&__exp_operator), &(literal!(" ")))?), hash);
                    hash = hashContinue(e.clone(), hash)?;
                }
                return Ok(stringHashDjb2Continue(&(literal!(")")), hash))
            },
            Deref @ IF { condition: __exp_condition, falseBranch: __exp_falseBranch, trueBranch: __exp_trueBranch, .. } => {
                hash = stringHashDjb2Continue(&(literal!("if ")), hash);
                hash = hashContinue(__exp_condition.clone(), hash)?;
                hash = stringHashDjb2Continue(&(literal!(" then ")), hash);
                hash = hashContinue(__exp_trueBranch.clone(), hash)?;
                hash = stringHashDjb2Continue(&(literal!(" else ")), hash);
                { (exp, hash) = (__exp_falseBranch.clone(), hash); continue '__tco; }
            },
            Deref @ CAST { exp: __exp_exp, ty: __exp_ty } => {
                hash = stringHashDjb2Continue(&(literal!("CAST(")), hash);
                hash = Type::hashContinue(__exp_ty.clone(), hash)?;
                hash = stringHashDjb2Continue(&(literal!(", ")), hash);
                hash = hashContinue(__exp_exp.clone(), hash)?;
                return Ok(stringHashDjb2Continue(&(literal!(")")), hash))
            },
            Deref @ BOX { exp: __exp_exp } => {
                hash = stringHashDjb2Continue(&(literal!("BOX(")), hash);
                hash = hashContinue(__exp_exp.clone(), hash)?;
                return Ok(stringHashDjb2Continue(&(literal!(")")), hash))
            },
            Deref @ UNBOX { exp: __exp_exp, .. } => {
                hash = stringHashDjb2Continue(&(literal!("UNBOX(")), hash);
                hash = hashContinue(__exp_exp.clone(), hash)?;
                return Ok(stringHashDjb2Continue(&(literal!(")")), hash))
            },
            Deref @ SUBSCRIPTED_EXP { exp: __exp_exp, subscripts: __exp_subscripts, .. } => {
                hash = stringHashDjb2Continue(&(literal!("(")), hash);
                hash = hashContinue(__exp_exp.clone(), hash)?;
                hash = stringHashDjb2Continue(&(literal!(")[")), hash);
                for mut sub in &*__exp_subscripts.clone() {
                    hash = Subscript::hashContinue(metamodelica::AsArg::as_arg(&sub), hash)?;
                    hash = stringHashDjb2Continue(&(literal!(", ")), hash);
                }
                return Ok(stringHashDjb2Continue(&(literal!("]")), hash))
            },
            Deref @ TUPLE_ELEMENT { index: __exp_index, tupleExp: __exp_tupleExp, .. } => {
                hash = hashContinue(__exp_tupleExp.clone(), hash)?;
                hash = stringHashDjb2Continue(&(literal!("[")), hash);
                hash = stringHashDjb2Continue(&(intString(__exp_index.clone())), hash);
                return Ok(stringHashDjb2Continue(&(literal!("]")), hash))
            },
            Deref @ RECORD_ELEMENT { fieldName: __exp_fieldName, recordExp: __exp_recordExp, .. } => {
                hash = stringHashDjb2Continue(&(literal!("(")), hash);
                hash = hashContinue(__exp_recordExp.clone(), hash)?;
                hash = stringHashDjb2Continue(&(literal!(").")), hash);
                return Ok(stringHashDjb2Continue(&__exp_fieldName, hash))
            },
            Deref @ MUTABLE { exp: __exp_exp } => {
                { (exp, hash) = (Mutable::access(__exp_exp.clone()), hash); continue '__tco; }
            },
            Deref @ EMPTY { .. } => {
                return Ok(stringHashDjb2Continue(&(literal!("#EMPTY#")), hash))
            },
            Deref @ PARTIAL_FUNCTION_APPLICATION { argNames: __exp_argNames, args: __exp_args, r#fn: __exp_fn, .. } => {
                hash = stringHashDjb2Continue(&(literal!("function ")), hash);
                hash = ComponentRef::hashContinue(metamodelica::AsArg::as_arg(&__exp_fn), false, hash)?;
                hash = stringHashDjb2Continue(&(literal!("(")), hash);
                for mut n in &*__exp_argNames.clone() {
                    hash = stringHashDjb2Continue(&n, hash);
                    hash = stringHashDjb2Continue(&(literal!(", ")), hash);
                }
                hash = stringHashDjb2Continue(&(literal!(" = ")), hash);
                for mut a in &*__exp_args.clone() {
                    hash = hashContinue(a.clone(), hash)?;
                    hash = stringHashDjb2Continue(&(literal!(", ")), hash);
                }
                return Ok(stringHashDjb2Continue(&(literal!(")")), hash))
            },
            Deref @ FILENAME { filename: __exp_filename } => {
                return Ok(stringHashDjb2Continue(&__exp_filename, hash))
            },
            Deref @ SHARED_LITERAL { exp: __exp_exp, index: __exp_index } => {
                hash = stringHashDjb2Continue(&(literal!("LITERAL(")), hash);
                hash = stringHashDjb2Continue(&(intString(__exp_index.clone())), hash);
                hash = stringHashDjb2Continue(&(literal!(", ")), hash);
                hash = hashContinue(__exp_exp.clone(), hash)?;
                return Ok(stringHashDjb2Continue(&(literal!(")")), hash))
            },
            Deref @ INSTANCE_NAME { .. } => {
                return Ok(stringHashDjb2Continue(&(literal!("getInstanceName()")), hash))
            },
            _ => {
                return Ok(hash)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn isEqual(mut exp1: metamodelica::Ref<NFExpression>, mut exp2: metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut isEqual: bool;
    isEqual = 0 == compare(exp1, exp2)?;
    Ok(isEqual)
}

pub fn compare(mut exp1: metamodelica::Ref<NFExpression>, mut exp2: metamodelica::Ref<NFExpression>) -> Result<i32> {
    let mut comp: i32;
    if referenceEq(&*(&*exp1), &*(&*exp2)) {
        comp = 0;
        return Ok(comp);
    }
    comp = Util::intCompare(
        metamodelica::valueConstructor((&*&*exp1))?,
        metamodelica::valueConstructor((&*&*exp2))?,
    );
    if comp != 0 {
        return Ok(comp);
    }
    comp = (match &*exp1 {
        INTEGER { value: __exp1_value } => {
            let mut i: i32;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ INTEGER { value: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            i = metamodelica::Own::own(__pa0);
            Util::intCompare(__exp1_value.clone(), i)
        }
        REAL { value: __exp1_value } => {
            let mut r: metamodelica::Real;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ REAL { value: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            r = metamodelica::Own::own(__pa0);
            Util::realCompare(__exp1_value.clone(), r)
        }
        STRING { value: __exp1_value } => {
            let mut s: ArcStr;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ STRING { value: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            s = metamodelica::Own::own(__pa0);
            stringCompare(&__exp1_value, &s)
        }
        BOOLEAN { value: __exp1_value } => {
            let mut b: bool;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ BOOLEAN { value: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            b = metamodelica::Own::own(__pa0);
            Util::boolCompare(__exp1_value.clone(), b)
        }
        ENUM_LITERAL {
            index: __exp1_index,
            ty: __exp1_ty,
            ..
        } => {
            let mut i: i32;
            let mut ty: metamodelica::Ref<Type::NFType>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ ENUM_LITERAL { ty: __pa0, index: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa0);
            i = metamodelica::Own::own(__pa1);
            comp = AbsynUtil::pathCompare(
                &(Type::enumName(metamodelica::AsArg::as_arg(&__exp1_ty))?),
                &(Type::enumName(&ty)?),
            )?;
            if comp == 0 {
                comp = Util::intCompare(__exp1_index.clone(), i);
            }
            comp
        }
        CLKCONST { clk: __exp1_clk } => {
            let mut clk: metamodelica::Ref<ClockKind::NFClockKind>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ CLKCONST { clk: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            clk = metamodelica::Own::own(__pa0);
            ClockKind::compare(__exp1_clk.clone(), clk)?
        }
        CREF { cref: __exp1_cref, .. } => {
            let mut cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ CREF { cref: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa0);
            ComponentRef::compare(metamodelica::AsArg::as_arg(&__exp1_cref), &cr)?
        }
        TYPENAME { ty: __exp1_ty } => {
            let mut ty: metamodelica::Ref<Type::NFType>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ TYPENAME { ty: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa0);
            valueCompare(__exp1_ty.clone(), ty)
        }
        ARRAY { ty: __exp1_ty, .. } => {
            let mut ty: metamodelica::Ref<Type::NFType>;
            let mut arr: metamodelica::Array<metamodelica::Ref<NFExpression>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ ARRAY { ty: __pa0, elements: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa0);
            arr = metamodelica::Own::own(__pa1);
            comp = valueCompare(ty, __exp1_ty.clone());
            if (comp == 0) {
                Array::compare(
                    var_field!((*exp1).elements, NFExpression::ARRAY).clone(),
                    arr.clone(),
                    &compare,
                )?
            } else {
                comp
            }
        }
        MATRIX {
            elements: __exp1_elements,
        } => {
            let mut mat: metamodelica::List<metamodelica::List<metamodelica::Ref<NFExpression>>>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ MATRIX { elements: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            mat = metamodelica::Own::own(__pa0);
            List::compare(
                __exp1_elements.clone(),
                mat,
                &({
                    let __pe_b2: Arc<dyn ::std::ops::Fn(_, _) -> Result<i32> + 'static> = (std::sync::Arc::new(compare)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<NFExpression>,
                                    metamodelica::Ref<NFExpression>,
                                ) -> Result<i32>
                                + 'static,
                        >);
                    move |__pe_a0, __pe_a1| List::compare(__pe_a0, __pe_a1, &*__pe_b2)
                }),
            )?
        }
        RANGE {
            start: __exp1_start,
            step: __exp1_step,
            stop: __exp1_stop,
            ..
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            let mut oe: Option<metamodelica::Ref<NFExpression>>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ RANGE { start: __pa0, step: __pa1, stop: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            oe = metamodelica::Own::own(__pa1);
            e2 = metamodelica::Own::own(__pa2);
            comp = compare(__exp1_start.clone(), e1)?;
            if comp == 0 {
                comp = compare(__exp1_stop.clone(), e2)?;
                if comp == 0 {
                    comp = compareOpt(__exp1_step.clone(), oe)?;
                }
            }
            comp
        }
        TUPLE {
            elements: __exp1_elements,
            ..
        } => {
            let mut expl: metamodelica::List<metamodelica::Ref<NFExpression>>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ TUPLE { elements: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            expl = metamodelica::Own::own(__pa0);
            List::compare(__exp1_elements.clone(), expl, &compare)?
        }
        RECORD {
            elements: __exp1_elements,
            path: __exp1_path,
            ..
        } => {
            let mut expl: metamodelica::List<metamodelica::Ref<NFExpression>>;
            let mut p: metamodelica::Ref<Path>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ RECORD { path: __pa0, elements: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            p = metamodelica::Own::own(__pa0);
            expl = metamodelica::Own::own(__pa1);
            comp = AbsynUtil::pathCompare(metamodelica::AsArg::as_arg(&__exp1_path), &p)?;
            if (comp == 0) {
                List::compare(__exp1_elements.clone(), expl, &compare)?
            } else {
                comp
            }
        }
        CALL { call: __exp1_call } => {
            let mut c: metamodelica::Ref<Call::NFCall>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ CALL { call: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            c = metamodelica::Own::own(__pa0);
            Call::compare(metamodelica::AsArg::as_arg(&__exp1_call), &c)?
        }
        SIZE {
            dimIndex: __exp1_dimIndex,
            exp: __exp1_exp,
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut oe: Option<metamodelica::Ref<NFExpression>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ SIZE { exp: __pa0, dimIndex: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            oe = metamodelica::Own::own(__pa1);
            comp = compareOpt(__exp1_dimIndex.clone(), oe)?;
            if (comp == 0) {
                compare(__exp1_exp.clone(), e1)?
            } else {
                comp
            }
        }
        END { .. } => 0,
        MULTARY {
            arguments: __exp1_arguments,
            inv_arguments: __exp1_inv_arguments,
            operator: __exp1_operator,
        } => {
            let mut expl: metamodelica::List<metamodelica::Ref<NFExpression>>;
            let mut inv_expl: metamodelica::List<metamodelica::Ref<NFExpression>>;
            let mut op: metamodelica::Ref<Operator::NFOperator>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ MULTARY { arguments: __pa0, inv_arguments: __pa1, operator: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            expl = metamodelica::Own::own(__pa0);
            inv_expl = metamodelica::Own::own(__pa1);
            op = metamodelica::Own::own(__pa2);
            comp = Operator::compare(metamodelica::AsArg::as_arg(&__exp1_operator), &op);
            if comp == 0 {
                comp = compareList(__exp1_arguments.clone(), expl)?;
            }
            if comp == 0 {
                comp = compareList(__exp1_inv_arguments.clone(), inv_expl)?;
            }
            comp
        }
        BINARY {
            exp1: __exp1_exp1,
            exp2: __exp1_exp2,
            operator: __exp1_operator,
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            let mut op: metamodelica::Ref<Operator::NFOperator>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ BINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            op = metamodelica::Own::own(__pa1);
            e2 = metamodelica::Own::own(__pa2);
            comp = Operator::compare(metamodelica::AsArg::as_arg(&__exp1_operator), &op);
            if comp == 0 {
                comp = compare(__exp1_exp1.clone(), e1)?;
                if comp == 0 {
                    comp = compare(__exp1_exp2.clone(), e2)?;
                }
            }
            comp
        }
        UNARY {
            exp: __exp1_exp,
            operator: __exp1_operator,
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut op: metamodelica::Ref<Operator::NFOperator>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ UNARY { operator: __pa0, exp: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            op = metamodelica::Own::own(__pa0);
            e1 = metamodelica::Own::own(__pa1);
            comp = Operator::compare(metamodelica::AsArg::as_arg(&__exp1_operator), &op);
            if (comp == 0) {
                compare(__exp1_exp.clone(), e1)?
            } else {
                comp
            }
        }
        LBINARY {
            exp1: __exp1_exp1,
            exp2: __exp1_exp2,
            operator: __exp1_operator,
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            let mut op: metamodelica::Ref<Operator::NFOperator>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ LBINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            op = metamodelica::Own::own(__pa1);
            e2 = metamodelica::Own::own(__pa2);
            comp = Operator::compare(metamodelica::AsArg::as_arg(&__exp1_operator), &op);
            if comp == 0 {
                comp = compare(__exp1_exp1.clone(), e1)?;
                if comp == 0 {
                    comp = compare(__exp1_exp2.clone(), e2)?;
                }
            }
            comp
        }
        LUNARY {
            exp: __exp1_exp,
            operator: __exp1_operator,
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut op: metamodelica::Ref<Operator::NFOperator>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ LUNARY { operator: __pa0, exp: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            op = metamodelica::Own::own(__pa0);
            e1 = metamodelica::Own::own(__pa1);
            comp = Operator::compare(metamodelica::AsArg::as_arg(&__exp1_operator), &op);
            if (comp == 0) {
                compare(__exp1_exp.clone(), e1)?
            } else {
                comp
            }
        }
        RELATION {
            exp1: __exp1_exp1,
            exp2: __exp1_exp2,
            operator: __exp1_operator,
            ..
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            let mut op: metamodelica::Ref<Operator::NFOperator>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ RELATION { exp1: __pa0, operator: __pa1, exp2: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            op = metamodelica::Own::own(__pa1);
            e2 = metamodelica::Own::own(__pa2);
            comp = Operator::compare(metamodelica::AsArg::as_arg(&__exp1_operator), &op);
            if comp == 0 {
                comp = compare(__exp1_exp1.clone(), e1)?;
                if comp == 0 {
                    comp = compare(__exp1_exp2.clone(), e2)?;
                }
            }
            comp
        }
        IF {
            condition: __exp1_condition,
            falseBranch: __exp1_falseBranch,
            trueBranch: __exp1_trueBranch,
            ..
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            let mut e3: metamodelica::Ref<NFExpression>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ IF { condition: __pa0, trueBranch: __pa1, falseBranch: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            e2 = metamodelica::Own::own(__pa1);
            e3 = metamodelica::Own::own(__pa2);
            comp = compare(__exp1_condition.clone(), e1)?;
            if comp == 0 {
                comp = compare(__exp1_trueBranch.clone(), e2)?;
                if comp == 0 {
                    comp = compare(__exp1_falseBranch.clone(), e3)?;
                }
            }
            comp
        }
        CAST { exp: __exp1_exp, .. } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = (::match_deref::match_deref! { match &(exp2) {
                Deref @ CAST { exp: __esc_e1, .. } => {
                    e1 = (*__esc_e1).clone();
                    e1.clone()
                },
                __esc_e1 => {
                    e1 = (*__esc_e1).clone();
                    e1.clone()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            compare(__exp1_exp.clone(), e1)?
        }
        BOX { exp: __exp1_exp } => {
            let mut e2: metamodelica::Ref<NFExpression>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ BOX { exp: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            e2 = metamodelica::Own::own(__pa0);
            compare(__exp1_exp.clone(), e2)?
        }
        UNBOX { exp: __exp1_exp, .. } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ UNBOX { exp: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            compare(__exp1_exp.clone(), e1)?
        }
        SUBSCRIPTED_EXP {
            exp: __exp1_exp,
            subscripts: __exp1_subscripts,
            ..
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ SUBSCRIPTED_EXP { exp: __pa0, subscripts: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            subs = metamodelica::Own::own(__pa1);
            comp = compare(__exp1_exp.clone(), e1)?;
            if comp == 0 {
                comp = Subscript::compareList(metamodelica::AsArg::as_arg(&__exp1_subscripts), subs)?;
            }
            comp
        }
        TUPLE_ELEMENT {
            index: __exp1_index,
            tupleExp: __exp1_tupleExp,
            ..
        } => {
            let mut i: i32;
            let mut e1: metamodelica::Ref<NFExpression>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ TUPLE_ELEMENT { tupleExp: __pa0, index: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            i = metamodelica::Own::own(__pa1);
            comp = Util::intCompare(__exp1_index.clone(), i);
            if comp == 0 {
                comp = compare(__exp1_tupleExp.clone(), e1)?;
            }
            comp
        }
        RECORD_ELEMENT {
            index: __exp1_index,
            recordExp: __exp1_recordExp,
            ..
        } => {
            let mut i: i32;
            let mut e1: metamodelica::Ref<NFExpression>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ RECORD_ELEMENT { recordExp: __pa0, index: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            i = metamodelica::Own::own(__pa1);
            comp = Util::intCompare(__exp1_index.clone(), i);
            if comp == 0 {
                comp = compare(__exp1_recordExp.clone(), e1)?;
            }
            comp
        }
        MUTABLE { exp: __exp1_exp } => {
            let mut me: Mutable::Mutable<metamodelica::Ref<NFExpression>>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ MUTABLE { exp: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            me = metamodelica::Own::own(__pa0);
            compare(Mutable::access(__exp1_exp.clone()), Mutable::access(me))?
        }
        SHARED_LITERAL { exp: __exp1_exp, .. } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ SHARED_LITERAL { exp: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            compare(__exp1_exp.clone(), e1)?
        }
        EMPTY { ty: __exp1_ty } => {
            let mut ty: metamodelica::Ref<Type::NFType>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ EMPTY { ty: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa0);
            valueCompare(__exp1_ty.clone(), ty)
        }
        PARTIAL_FUNCTION_APPLICATION {
            args: __exp1_args,
            r#fn: __exp1_fn,
            ..
        } => {
            let mut cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut expl: metamodelica::List<metamodelica::Ref<NFExpression>>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp2) {
                Deref @ PARTIAL_FUNCTION_APPLICATION { r#fn: __pa0, args: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa0);
            expl = metamodelica::Own::own(__pa1);
            comp = ComponentRef::compare(metamodelica::AsArg::as_arg(&__exp1_fn), &cr)?;
            if comp == 0 {
                comp = List::compare(__exp1_args.clone(), expl, &compare)?;
            }
            comp
        }
        FILENAME {
            filename: __exp1_filename,
        } => {
            let mut s: ArcStr;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ FILENAME { filename: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            s = metamodelica::Own::own(__pa0);
            stringCompare(&__exp1_filename, &s)
        }
        INSTANCE_NAME { scope: __exp1_scope } => {
            let mut node: metamodelica::Ref<InstNode::InstNode>;
            let __pa0 = ::match_deref::match_deref! { match &(exp2) {
                Deref @ INSTANCE_NAME { scope: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            node = metamodelica::Own::own(__pa0);
            InstNode::refCompare(metamodelica::AsArg::as_arg(&__exp1_scope), &node)?
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFExpression.compare"));
                    __mm_s.push_str(&*literal!(" got unknown expression."));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(comp)
}

pub(crate) fn compareOpt(
    mut expl1: Option<metamodelica::Ref<NFExpression>>,
    mut expl2: Option<metamodelica::Ref<NFExpression>>,
) -> Result<i32> {
    let mut comp: i32;
    let mut e1: metamodelica::Ref<NFExpression>;
    let mut e2: metamodelica::Ref<NFExpression>;
    comp = (::match_deref::match_deref! { match &((expl1, expl2)) {
        (None, None) => 0,
        (None, _) => -1,
        (_, None) => 1,
        (Some(__esc_e1), Some(__esc_e2)) => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            compare(e1.clone(), e2.clone())?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(comp)
}

pub(crate) fn compareList(
    mut expl1: metamodelica::List<metamodelica::Ref<NFExpression>>,
    mut expl2: metamodelica::List<metamodelica::Ref<NFExpression>>,
) -> Result<i32> {
    let mut comp: i32 = List::compare(expl1.clone(), expl2.clone(), &compare)?;
    Ok(comp)
}

pub fn typeOf(mut exp: metamodelica::Ref<NFExpression>) -> metamodelica::Ref<Type::NFType> {
    '__tco: loop {
        match &*exp {
            INTEGER { .. } => return crate::NFType::interned_INTEGER(),
            REAL { .. } => return crate::NFType::interned_REAL(),
            STRING { .. } => return crate::NFType::interned_STRING(),
            BOOLEAN { .. } => return crate::NFType::interned_BOOLEAN(),
            ENUM_LITERAL { ty: __exp_ty, .. } => return __exp_ty.clone(),
            CLKCONST { .. } => return crate::NFType::interned_CLOCK(),
            CREF { ty: __exp_ty, .. } => return __exp_ty.clone(),
            TYPENAME { ty: __exp_ty } => return __exp_ty.clone(),
            ARRAY { ty: __exp_ty, .. } => return __exp_ty.clone(),
            RANGE { ty: __exp_ty, .. } => return __exp_ty.clone(),
            TUPLE { ty: __exp_ty, .. } => return __exp_ty.clone(),
            RECORD { ty: __exp_ty, .. } => return __exp_ty.clone(),
            CALL { call: __exp_call } => return Call::typeOf(metamodelica::AsArg::as_arg(&__exp_call)),
            SIZE {
                dimIndex: __exp_dimIndex,
                exp: __exp_exp,
            } => {
                if ((__exp_dimIndex).is_some()) {
                    return crate::NFType::interned_INTEGER();
                } else {
                    return Type::sizeType(typeOf(__exp_exp.clone()));
                }
            }
            END { .. } => return crate::NFType::interned_INTEGER(),
            MULTARY {
                operator: __exp_operator,
                ..
            } => return Operator::typeOf(metamodelica::AsArg::as_arg(&__exp_operator)),
            BINARY {
                operator: __exp_operator,
                ..
            } => return Operator::typeOf(metamodelica::AsArg::as_arg(&__exp_operator)),
            UNARY {
                operator: __exp_operator,
                ..
            } => return Operator::typeOf(metamodelica::AsArg::as_arg(&__exp_operator)),
            LBINARY {
                operator: __exp_operator,
                ..
            } => return Operator::typeOf(metamodelica::AsArg::as_arg(&__exp_operator)),
            LUNARY {
                operator: __exp_operator,
                ..
            } => return Operator::typeOf(metamodelica::AsArg::as_arg(&__exp_operator)),
            RELATION {
                operator: __exp_operator,
                ..
            } => {
                return Type::copyDims(
                    Operator::typeOf(metamodelica::AsArg::as_arg(&__exp_operator)),
                    crate::NFType::interned_BOOLEAN(),
                );
            }
            IF { ty: __exp_ty, .. } => return __exp_ty.clone(),
            CAST { ty: __exp_ty, .. } => return __exp_ty.clone(),
            BOX { exp: __exp_exp } => {
                return metamodelica::Ref::new(Type::NFType::METABOXED {
                    ty: typeOf(__exp_exp.clone()),
                });
            }
            UNBOX { ty: __exp_ty, .. } => return __exp_ty.clone(),
            SUBSCRIPTED_EXP { ty: __exp_ty, .. } => return __exp_ty.clone(),
            TUPLE_ELEMENT { ty: __exp_ty, .. } => return __exp_ty.clone(),
            RECORD_ELEMENT { ty: __exp_ty, .. } => return __exp_ty.clone(),
            MUTABLE { exp: __exp_exp } => {
                exp = Mutable::access(__exp_exp.clone());
                continue '__tco;
            }
            SHARED_LITERAL { exp: __exp_exp, .. } => {
                exp = __exp_exp.clone();
                continue '__tco;
            }
            EMPTY { ty: __exp_ty } => return __exp_ty.clone(),
            PARTIAL_FUNCTION_APPLICATION { ty: __exp_ty, .. } => return __exp_ty.clone(),
            FILENAME { .. } => return crate::NFType::interned_STRING(),
            INSTANCE_NAME { .. } => return crate::NFType::interned_STRING(),
            _ => return crate::NFType::interned_UNKNOWN(),
        }
    }
}

pub(crate) fn sizeOf(mut exp: metamodelica::Ref<NFExpression>) -> Result<i32> {
    let mut sz: i32 = Type::sizeOf(&(typeOf(exp.clone())), false)?;
    Ok(sz)
}

pub(crate) fn callOf(mut exp: &metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<Call::NFCall>> {
    let mut call: metamodelica::Ref<Call::NFCall>;
    let __pa0 = ::match_deref::match_deref! { match &((*exp)) {
        Deref @ CALL { call: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    call = metamodelica::Own::own(__pa0);
    Ok(call)
}

pub(crate) fn sizeZero(mut exp: metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut b: bool;
    match '__try0: {
        b = 0 == unwrap_break_err!(sizeOf(exp.clone()), '__try0);
        Ok::<_, &'static str>((b.clone(),))
    } {
        Ok((__try0_o0,)) => {
            b = __try0_o0;
        }
        Err(_) => {
            b = false;
            if isCallNamed(&exp, &(literal!("fill")))? {
                for mut arg in &*(Call::arguments(&(callOf(&exp)?))?).rest()? {
                    if isZero(metamodelica::AsArg::as_arg(&arg))? {
                        b = true;
                    }
                }
            }
        }
    }
    Ok(b)
}

pub(crate) fn setType(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut exp: metamodelica::Ref<NFExpression>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression> = exp;
    exp = (match &*exp {
        ENUM_LITERAL { .. } => {
            assign_variant_field!(exp => NFExpression::ENUM_LITERAL; ty = ty);
            exp
        }
        CREF { .. } => {
            assign_variant_field!(exp => NFExpression::CREF; ty = ty);
            exp
        }
        TYPENAME { .. } => {
            assign_variant_field!(exp => NFExpression::TYPENAME; ty = ty);
            exp
        }
        ARRAY { .. } => {
            assign_variant_field!(exp => NFExpression::ARRAY; ty = ty);
            exp
        }
        RANGE { .. } => {
            assign_variant_field!(exp => NFExpression::RANGE; ty = ty);
            exp
        }
        TUPLE { .. } => {
            assign_variant_field!(exp => NFExpression::TUPLE; ty = ty);
            exp
        }
        RECORD { .. } => {
            assign_variant_field!(exp => NFExpression::RECORD; ty = ty);
            exp
        }
        CALL { call: __exp_call } => {
            assign_variant_field!(exp => NFExpression::CALL; call = Call::setType(__exp_call.clone(), ty)?);
            exp
        }
        BINARY {
            operator: __exp_operator,
            ..
        } => {
            assign_variant_field!(exp => NFExpression::BINARY; operator = Operator::setType(ty, __exp_operator.clone()));
            exp
        }
        UNARY {
            operator: __exp_operator,
            ..
        } => {
            assign_variant_field!(exp => NFExpression::UNARY; operator = Operator::setType(ty, __exp_operator.clone()));
            exp
        }
        LBINARY {
            operator: __exp_operator,
            ..
        } => {
            assign_variant_field!(exp => NFExpression::LBINARY; operator = Operator::setType(ty, __exp_operator.clone()));
            exp
        }
        LUNARY {
            operator: __exp_operator,
            ..
        } => {
            assign_variant_field!(exp => NFExpression::LUNARY; operator = Operator::setType(ty, __exp_operator.clone()));
            exp
        }
        RELATION {
            operator: __exp_operator,
            ..
        } => {
            assign_variant_field!(exp => NFExpression::RELATION; operator = Operator::setType(ty, __exp_operator.clone()));
            exp
        }
        IF { .. } => {
            assign_variant_field!(exp => NFExpression::IF; ty = ty);
            exp
        }
        CAST { .. } => {
            assign_variant_field!(exp => NFExpression::CAST; ty = ty);
            exp
        }
        UNBOX { .. } => {
            assign_variant_field!(exp => NFExpression::UNBOX; ty = ty);
            exp
        }
        SUBSCRIPTED_EXP { .. } => {
            assign_variant_field!(exp => NFExpression::SUBSCRIPTED_EXP; ty = ty);
            exp
        }
        TUPLE_ELEMENT { .. } => {
            assign_variant_field!(exp => NFExpression::TUPLE_ELEMENT; ty = ty);
            exp
        }
        RECORD_ELEMENT { .. } => {
            assign_variant_field!(exp => NFExpression::RECORD_ELEMENT; ty = ty);
            exp
        }
        PARTIAL_FUNCTION_APPLICATION { .. } => {
            assign_variant_field!(exp => NFExpression::PARTIAL_FUNCTION_APPLICATION; ty = ty);
            exp
        }
        _ => exp,
    });
    Ok(exp)
}

pub fn applyToType(
    mut exp: metamodelica::Ref<NFExpression>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>>,
) -> Result<metamodelica::Ref<NFExpression>> {
    pub type typeFunc = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>> + 'static,
    >;

    let mut exp: metamodelica::Ref<NFExpression> = exp;
    exp = (match &*exp {
        ENUM_LITERAL { ty: __exp_ty, .. } => {
            assign_variant_field!(exp => NFExpression::ENUM_LITERAL; ty = func(__exp_ty.clone())?);
            exp
        }
        CREF { ty: __exp_ty, .. } => {
            assign_variant_field!(exp => NFExpression::CREF;
                ty = func(__exp_ty.clone())?,
                cref = ComponentRef::applyToType(var_field!((*exp).cref, NFExpression::CREF).clone(), func)?
            );
            exp
        }
        TYPENAME { ty: __exp_ty } => {
            assign_variant_field!(exp => NFExpression::TYPENAME; ty = func(__exp_ty.clone())?);
            exp
        }
        ARRAY { ty: __exp_ty, .. } => {
            assign_variant_field!(exp => NFExpression::ARRAY; ty = func(__exp_ty.clone())?);
            exp
        }
        RANGE { ty: __exp_ty, .. } => {
            assign_variant_field!(exp => NFExpression::RANGE; ty = func(__exp_ty.clone())?);
            exp
        }
        TUPLE { ty: __exp_ty, .. } => {
            assign_variant_field!(exp => NFExpression::TUPLE; ty = func(__exp_ty.clone())?);
            exp
        }
        RECORD { ty: __exp_ty, .. } => {
            assign_variant_field!(exp => NFExpression::RECORD; ty = func(__exp_ty.clone())?);
            exp
        }
        CALL { call: __exp_call } => {
            assign_variant_field!(exp => NFExpression::CALL; call = Call::setType(__exp_call.clone(), func(Call::typeOf(metamodelica::AsArg::as_arg(&__exp_call)))?)?);
            exp
        }
        SIZE { exp: __exp_exp, .. } => {
            assign_variant_field!(exp => NFExpression::SIZE; exp = applyToType(__exp_exp.clone(), func)?);
            exp
        }
        MULTARY { operator: o, .. } => {
            let mut o = (*o).clone();
            assign_field!(o.ty = func(o.ty.clone())?);
            assign_variant_field!(exp => NFExpression::MULTARY; operator = o.clone());
            exp
        }
        BINARY { operator: o, .. } => {
            let mut o = (*o).clone();
            assign_field!(o.ty = func(o.ty.clone())?);
            assign_variant_field!(exp => NFExpression::BINARY; operator = o.clone());
            exp
        }
        UNARY { operator: o, .. } => {
            let mut o = (*o).clone();
            assign_field!(o.ty = func(o.ty.clone())?);
            assign_variant_field!(exp => NFExpression::UNARY; operator = o.clone());
            exp
        }
        LBINARY { operator: o, .. } => {
            let mut o = (*o).clone();
            assign_field!(o.ty = func(o.ty.clone())?);
            assign_variant_field!(exp => NFExpression::LBINARY; operator = o.clone());
            exp
        }
        LUNARY { operator: o, .. } => {
            let mut o = (*o).clone();
            assign_field!(o.ty = func(o.ty.clone())?);
            assign_variant_field!(exp => NFExpression::LUNARY; operator = o.clone());
            exp
        }
        RELATION { operator: o, .. } => {
            let mut o = (*o).clone();
            assign_field!(o.ty = func(o.ty.clone())?);
            assign_variant_field!(exp => NFExpression::RELATION; operator = o.clone());
            exp
        }
        IF { ty: __exp_ty, .. } => {
            assign_variant_field!(exp => NFExpression::IF; ty = func(__exp_ty.clone())?);
            exp
        }
        CAST { ty: __exp_ty, .. } => {
            assign_variant_field!(exp => NFExpression::CAST; ty = func(__exp_ty.clone())?);
            exp
        }
        BOX { exp: __exp_exp } => {
            assign_variant_field!(exp => NFExpression::BOX; exp = applyToType(__exp_exp.clone(), func)?);
            exp
        }
        UNBOX { ty: __exp_ty, .. } => {
            assign_variant_field!(exp => NFExpression::UNBOX; ty = func(__exp_ty.clone())?);
            exp
        }
        SUBSCRIPTED_EXP { ty: __exp_ty, .. } => {
            assign_variant_field!(exp => NFExpression::SUBSCRIPTED_EXP; ty = func(__exp_ty.clone())?);
            exp
        }
        TUPLE_ELEMENT { ty: __exp_ty, .. } => {
            assign_variant_field!(exp => NFExpression::TUPLE_ELEMENT; ty = func(__exp_ty.clone())?);
            exp
        }
        RECORD_ELEMENT { ty: __exp_ty, .. } => {
            assign_variant_field!(exp => NFExpression::RECORD_ELEMENT; ty = func(__exp_ty.clone())?);
            exp
        }
        MUTABLE { exp: __exp_exp } => {
            Mutable::update(
                __exp_exp.clone(),
                applyToType(Mutable::access(__exp_exp.clone()), func)?,
            );
            exp
        }
        SHARED_LITERAL { exp: __exp_exp, .. } => {
            assign_variant_field!(exp => NFExpression::SHARED_LITERAL; exp = applyToType(__exp_exp.clone(), func)?);
            exp
        }
        EMPTY { ty: __exp_ty } => {
            assign_variant_field!(exp => NFExpression::EMPTY; ty = func(__exp_ty.clone())?);
            exp
        }
        PARTIAL_FUNCTION_APPLICATION { ty: __exp_ty, .. } => {
            assign_variant_field!(exp => NFExpression::PARTIAL_FUNCTION_APPLICATION; ty = func(__exp_ty.clone())?);
            exp
        }
        _ => exp,
    });
    Ok(exp)
}

pub(crate) fn typeCastOpt(
    mut exp: Option<metamodelica::Ref<NFExpression>>,
    mut ty: &metamodelica::Ref<Type::NFType>,
) -> Result<Option<metamodelica::Ref<NFExpression>>> {
    let mut outExp: Option<metamodelica::Ref<NFExpression>> = Util::applyOption(
        exp.clone(),
        &({
            let __pe_b1 = ty.clone();
            move |__pe_a0| typeCast(__pe_a0, __pe_b1.clone())
        }),
    )?;
    Ok(outExp)
}

pub(crate) fn typeCast(
    mut exp: metamodelica::Ref<NFExpression>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<NFExpression>> {
    '__tco: loop {
        let mut t: metamodelica::Ref<Type::NFType>;
        let mut ety: metamodelica::Ref<Type::NFType>;
        let mut e1: metamodelica::Ref<NFExpression>;
        let mut e2: metamodelica::Ref<NFExpression>;
        let mut arr: metamodelica::Array<metamodelica::Ref<NFExpression>>;
        ety = Type::arrayElementType(&ty);
        match &*exp {
            INTEGER { value: __exp_value } => {
                if (Type::isReal(&ety)?) {
                    return Ok(metamodelica::Ref::new(NFExpression::REAL {
                        value: intReal(__exp_value.clone()),
                    }));
                } else if (Type::isEnumeration(&ety)
                    && Flags::isConfigFlagSet(
                        Flags::ALLOW_NON_STANDARD_MODELICA.clone(),
                        literal!("nonStdIntegersAsEnumeration"),
                    )?)
                {
                    return Ok(metamodelica::Ref::new(NFExpression::ENUM_LITERAL {
                        ty: ety.clone(),
                        name: Type::nthEnumLiteral(&ety, __exp_value.clone())?,
                        index: __exp_value.clone(),
                    }));
                } else {
                    return Ok(typeCastGeneric(exp, &ety)?);
                }
            }
            ENUM_LITERAL { .. }
                if (Flags::isConfigFlagSet(
                    Flags::ALLOW_NON_STANDARD_MODELICA.clone(),
                    literal!("nonStdEnumerationAsIntegers"),
                )?) =>
            {
                if (Type::isInteger(&ety)?) {
                    return Ok(metamodelica::Ref::new(NFExpression::INTEGER {
                        value: toInteger(&exp)?,
                    }));
                } else {
                    return Ok(typeCastGeneric(exp, &ety)?);
                }
            }
            BOOLEAN { value: __exp_value } => {
                if (Type::isReal(&ety)? && Flags::isSet(Flags::NF_API.clone())?) {
                    return Ok(metamodelica::Ref::new(NFExpression::REAL {
                        value: if (__exp_value.clone()) {
                            metamodelica::OrderedFloat(1.0_f64)
                        } else {
                            metamodelica::OrderedFloat(0.0_f64)
                        },
                    }));
                } else {
                    return Ok(typeCastGeneric(exp, &ety)?);
                }
            }
            REAL { .. } => {
                if (Type::isReal(&ety)?) {
                    return Ok(exp);
                } else {
                    return Ok(typeCastGeneric(exp, &ety)?);
                }
            }
            ARRAY {
                ty: __esc_t,
                elements: __esc_arr,
                literal: __exp_literal,
            } => {
                t = (*__esc_t).clone();
                arr = (*__esc_arr).clone();
                arr = Array::map(
                    arr.clone(),
                    &({
                        let __pe_b1 = ety.clone();
                        move |__pe_a0| typeCast(__pe_a0, __pe_b1.clone())
                    }),
                )?;
                t = Type::setArrayElementType(metamodelica::AsArg::as_arg(&t), &ety);
                return Ok(makeArray(t.clone(), arr.clone(), __exp_literal.clone()));
            }
            RANGE {
                ty: __esc_t,
                start: __exp_start,
                step: __exp_step,
                stop: __exp_stop,
            } => {
                t = (*__esc_t).clone();
                t = Type::setArrayElementType(metamodelica::AsArg::as_arg(&t), &ety);
                return Ok(metamodelica::Ref::new(NFExpression::RANGE {
                    ty: t.clone(),
                    start: typeCast(__exp_start.clone(), ety.clone())?,
                    step: typeCastOpt(__exp_step.clone(), &ety)?,
                    stop: typeCast(__exp_stop.clone(), ety.clone())?,
                }));
            }
            UNARY {
                exp: __exp_exp,
                operator: __exp_operator,
            } => {
                t = Type::setArrayElementType(&(Operator::typeOf(metamodelica::AsArg::as_arg(&__exp_operator))), &ety);
                return Ok(metamodelica::Ref::new(NFExpression::UNARY {
                    operator: Operator::setType(t, __exp_operator.clone()),
                    exp: typeCast(__exp_exp.clone(), ety.clone())?,
                }));
            }
            BINARY {
                exp1: __exp_exp1,
                exp2: __exp_exp2,
                operator: __exp_operator,
            } if (Type::isReal(&ety)?
                && isCastableIntegerArithmetic(metamodelica::AsArg::as_arg(&__exp_operator))?) =>
            {
                t = Type::setArrayElementType(&(Operator::typeOf(metamodelica::AsArg::as_arg(&__exp_operator))), &ety);
                return Ok(metamodelica::Ref::new(NFExpression::BINARY {
                    exp1: typeCast(__exp_exp1.clone(), ety.clone())?,
                    operator: Operator::setType(t, __exp_operator.clone()),
                    exp2: typeCast(__exp_exp2.clone(), ety.clone())?,
                }));
            }
            IF {
                condition: __exp_condition,
                falseBranch: __exp_falseBranch,
                trueBranch: __exp_trueBranch,
                ..
            } => {
                e1 = typeCast(__exp_trueBranch.clone(), ety.clone())?;
                e2 = typeCast(__exp_falseBranch.clone(), ety.clone())?;
                t = if (Type::isConditionalArray(&ty)) {
                    Type::setConditionalArrayTypes(&ty, typeOf(e1.clone()), typeOf(e2.clone()))?
                } else {
                    typeOf(e1.clone())
                };
                return Ok(metamodelica::Ref::new(NFExpression::IF {
                    ty: t,
                    condition: __exp_condition.clone(),
                    trueBranch: e1,
                    falseBranch: e2,
                }));
            }
            CALL { .. } => return Ok(Call::typeCast(exp, ety.clone())?),
            CAST { exp: __exp_exp, .. } => {
                (exp, ty) = (__exp_exp.clone(), ty);
                continue '__tco;
            }
            SUBSCRIPTED_EXP {
                exp: __exp_exp,
                split: __exp_split,
                subscripts: __exp_subscripts,
                ty: __exp_ty,
            } => {
                e1 = typeCast(__exp_exp.clone(), ety.clone())?;
                t = Type::setArrayElementType(metamodelica::AsArg::as_arg(&__exp_ty), &ety);
                return Ok(metamodelica::Ref::new(NFExpression::SUBSCRIPTED_EXP {
                    exp: e1,
                    subscripts: __exp_subscripts.clone(),
                    ty: t,
                    split: __exp_split.clone(),
                }));
            }
            _ => return Ok(typeCastGeneric(exp, &ety)?),
        }
    }
}

pub(crate) fn isCastableIntegerArithmetic(mut op: &metamodelica::Ref<Operator::NFOperator>) -> Result<bool> {
    use crate::NFOperator::Op;
    let mut res: bool;
    res = Type::isInteger(&(Type::arrayElementType(&(Operator::typeOf(op)))))?
        && (match op.op.clone() {
            Operator::Op::ADD => true,
            Operator::Op::SUB => true,
            Operator::Op::MUL => true,
            Operator::Op::ADD_EW => true,
            Operator::Op::SUB_EW => true,
            Operator::Op::MUL_EW => true,
            Operator::Op::ADD_SCALAR_ARRAY => true,
            Operator::Op::ADD_ARRAY_SCALAR { .. } => true,
            Operator::Op::SUB_SCALAR_ARRAY { .. } => true,
            Operator::Op::SUB_ARRAY_SCALAR => true,
            Operator::Op::MUL_SCALAR_ARRAY => true,
            Operator::Op::MUL_ARRAY_SCALAR { .. } => true,
            Operator::Op::MUL_VECTOR_MATRIX => true,
            Operator::Op::MUL_MATRIX_VECTOR => true,
            Operator::Op::SCALAR_PRODUCT => true,
            Operator::Op::MATRIX_PRODUCT => true,
            _ => false,
        });
    Ok(res)
}

pub(crate) fn typeCastGeneric(
    mut exp: metamodelica::Ref<NFExpression>,
    mut ty: &metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression> = exp;
    let mut exp_ty: metamodelica::Ref<Type::NFType> = typeOf(exp.clone());
    if !(Type::isEqual(ty, &(Type::arrayElementType(&exp_ty)))?) {
        exp = metamodelica::Ref::new(NFExpression::CAST {
            ty: Type::setArrayElementType(&exp_ty, ty),
            exp: exp,
        });
    }
    Ok(exp)
}

pub fn realValue(mut exp: &metamodelica::Ref<NFExpression>) -> Result<metamodelica::Real> {
    let mut value: metamodelica::Real;
    value = (match &**exp {
        REAL { value: __exp_value } => __exp_value.clone(),
        INTEGER { value: __exp_value } => intReal(__exp_value.clone()),
        _ => return Err("match: no arm matched"),
    });
    Ok(value)
}

pub(crate) fn makeReal(mut value: metamodelica::Real) -> metamodelica::Ref<NFExpression> {
    let mut exp: metamodelica::Ref<NFExpression> = metamodelica::Ref::new(NFExpression::REAL { value: value });
    exp
}

pub fn integerValue(mut exp: metamodelica::Ref<NFExpression>) -> Result<i32> {
    let mut value: i32;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ INTEGER { value: __pa1 } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        value = metamodelica::Own::own(__pa1);
        Ok::<_, &'static str>((value.clone(),))
    } {
        Ok((__try0_o0,)) => {
            value = __try0_o0;
        }
        Err(__try0_err) => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFExpression.integerValue"));
                    __mm_s.push_str(&*literal!(" failed because expression is not an integer:\n"));
                    __mm_s.push_str(&*toString(exp.clone())?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err(__try0_err);
        }
    }
    Ok(value)
}

pub fn integerValueOrDefault(mut exp: &metamodelica::Ref<NFExpression>, mut value: i32) -> i32 {
    let mut value: i32 = value;
    value = (match &**exp {
        INTEGER { value: __exp_value } => __exp_value.clone(),
        _ => value,
    });
    value
}

pub fn makeInteger(mut value: i32) -> metamodelica::Ref<NFExpression> {
    let mut exp: metamodelica::Ref<NFExpression> = metamodelica::Ref::new(NFExpression::INTEGER { value: value });
    exp
}

pub fn stringValue(mut exp: &metamodelica::Ref<NFExpression>) -> ArcStr {
    let mut value: ArcStr;
    value = (match &**exp {
        STRING { value: __exp_value } => __exp_value.clone(),
        FILENAME {
            filename: __exp_filename,
        } => __exp_filename.clone(),
        _ => literal!(""),
    });
    value
}

pub(crate) fn booleanValue(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut value: bool;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &((*exp)) {
            Deref @ BOOLEAN { value: __pa1 } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        value = metamodelica::Own::own(__pa1);
        Ok::<_, &'static str>((value.clone(),))
    } {
        Ok((__try0_o0,)) => {
            value = __try0_o0;
        }
        Err(_) => {
            value = false;
        }
    }
    value
}

pub fn makeArray(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut expl: metamodelica::Array<metamodelica::Ref<NFExpression>>,
    mut literal: bool,
) -> metamodelica::Ref<NFExpression> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    outExp = metamodelica::Ref::new(NFExpression::ARRAY {
        ty: ty,
        elements: expl.clone(),
        literal: literal,
    });
    outExp
}

pub fn makeArrayCheckLiteral(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut expl: metamodelica::Array<metamodelica::Ref<NFExpression>>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    outExp = metamodelica::Ref::new(NFExpression::ARRAY {
        ty: ty,
        elements: expl.clone(),
        literal: Array::all(expl.clone(), &move |__a0: metamodelica::Ref<NFExpression>| {
            isLiteral(&__a0)
        })?,
    });
    Ok(outExp)
}

pub(crate) fn makeEmptyArray(mut ty: metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut non_empty_dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
    let mut arr_ty: metamodelica::Ref<Type::NFType>;
    dims = Type::arrayDims(ty.clone());
    while !(Dimension::isZero(&((dims).head().cloned()?))?) {
        non_empty_dims = metamodelica::cons((dims).head().cloned()?, non_empty_dims);
        dims = (dims).rest()?;
    }
    arr_ty = metamodelica::Ref::new(Type::NFType::ARRAY {
        elementType: Type::arrayElementType(&ty),
        dimensions: dims,
    });
    outExp = metamodelica::Ref::new(NFExpression::ARRAY {
        ty: arr_ty,
        elements: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
        literal: true,
    });
    (outExp, _) = liftArrayList(non_empty_dims, outExp)?;
    Ok(outExp)
}

pub(crate) fn makeIntegerArray(mut values: &metamodelica::List<i32>) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression>;
    exp = makeArray(
        metamodelica::Ref::new(Type::NFType::ARRAY {
            elementType: crate::NFType::interned_INTEGER(),
            dimensions: list![Dimension::fromInteger(
                ((values).len() as i32),
                Prefixes::Variability::CONSTANT.clone()
            )],
        }),
        Array::mapList(values, &fnptr!(makeInteger, i32))?,
        true,
    );
    Ok(exp)
}

pub(crate) fn makeRealArray(
    mut values: &metamodelica::List<metamodelica::Real>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression>;
    exp = makeArray(
        metamodelica::Ref::new(Type::NFType::ARRAY {
            elementType: crate::NFType::interned_REAL(),
            dimensions: list![Dimension::fromInteger(
                ((values).len() as i32),
                Prefixes::Variability::CONSTANT.clone()
            )],
        }),
        Array::mapList(values, &fnptr!(makeReal, metamodelica::Real))?,
        true,
    );
    Ok(exp)
}

pub(crate) fn makeRealMatrix(
    mut values: metamodelica::List<metamodelica::List<metamodelica::Real>>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut expl: metamodelica::List<metamodelica::Ref<NFExpression>>;
    if (values).is_empty() {
        ty = metamodelica::Ref::new(Type::NFType::ARRAY {
            elementType: crate::NFType::interned_REAL(),
            dimensions: list![
                Dimension::fromInteger(0, Prefixes::Variability::CONSTANT.clone()),
                crate::NFDimension::interned_UNKNOWN()
            ],
        });
        exp = makeEmptyArray(ty)?;
    } else {
        ty = metamodelica::Ref::new(Type::NFType::ARRAY {
            elementType: crate::NFType::interned_REAL(),
            dimensions: list![Dimension::fromInteger(
                (((values).head().cloned()?).len() as i32),
                Prefixes::Variability::CONSTANT.clone()
            )],
        });
        expl = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
            for mut row in (values).into_iter().cloned() {
                let __x = makeArray(
                    ty.clone(),
                    metamodelica::arrayFromVec(
                        ({
                            let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
                            for mut v in (row.clone()).into_iter().cloned() {
                                let __x = metamodelica::Ref::new(NFExpression::REAL { value: v.clone() });
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        })
                        .into_iter()
                        .cloned()
                        .collect(),
                    ),
                    true,
                );
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        ty = Type::liftArrayLeft(
            ty,
            &(Dimension::fromInteger(((expl).len() as i32), Prefixes::Variability::CONSTANT.clone())),
        );
        exp = makeArray(
            ty,
            metamodelica::arrayFromVec(expl.into_iter().cloned().collect()),
            true,
        );
    }
    Ok(exp)
}

pub fn makeExpArray(
    mut elements: metamodelica::Array<metamodelica::Ref<NFExpression>>,
    mut elementType: metamodelica::Ref<Type::NFType>,
    mut isLiteral: bool,
) -> metamodelica::Ref<NFExpression> {
    let mut exp: metamodelica::Ref<NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    ty = Type::liftArrayLeft(
        elementType,
        &(Dimension::fromInteger(
            metamodelica::arrayLength(elements.clone()),
            Prefixes::Variability::CONSTANT.clone(),
        )),
    );
    exp = makeArray(ty, elements.clone(), isLiteral);
    exp
}

pub fn makeRecord(
    mut recordName: metamodelica::Ref<Path>,
    mut recordType: metamodelica::Ref<Type::NFType>,
    mut fields: metamodelica::List<metamodelica::Ref<NFExpression>>,
) -> metamodelica::Ref<NFExpression> {
    let mut exp: metamodelica::Ref<NFExpression>;
    exp = metamodelica::Ref::new(NFExpression::RECORD {
        path: recordName,
        ty: recordType,
        elements: fields,
    });
    exp
}

pub fn makeRange(
    mut start: metamodelica::Ref<NFExpression>,
    mut step: Option<metamodelica::Ref<NFExpression>>,
    mut stop: metamodelica::Ref<NFExpression>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut rangeExp: metamodelica::Ref<NFExpression>;
    rangeExp = metamodelica::Ref::new(NFExpression::RANGE {
        ty: TypeCheck::getRangeType(
            start.clone(),
            step.clone(),
            stop.clone(),
            typeOf(start.clone()),
            &(Absyn::dummyInfo.clone()),
        )?,
        start: start,
        step: step,
        stop: stop,
    });
    Ok(rangeExp)
}

pub(crate) fn makeIntegerRange(
    mut start: i32,
    mut step: i32,
    mut stop: i32,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut rangeExp: metamodelica::Ref<NFExpression>;
    let mut start_exp: metamodelica::Ref<NFExpression>;
    let mut stop_exp: metamodelica::Ref<NFExpression>;
    let mut step_exp: Option<metamodelica::Ref<NFExpression>>;
    start_exp = metamodelica::Ref::new(NFExpression::INTEGER { value: start });
    stop_exp = metamodelica::Ref::new(NFExpression::INTEGER { value: stop });
    if start == stop || step == 1 && start <= stop || step == -1 && start >= stop {
        step_exp = None;
    } else {
        step_exp = Some(metamodelica::Ref::new(NFExpression::INTEGER { value: step }));
    }
    rangeExp = makeRange(start_exp, step_exp, stop_exp)?;
    Ok(rangeExp)
}

pub fn getIntegerRange(mut range: metamodelica::Ref<NFExpression>, mut resize: bool) -> Result<(i32, i32, i32)> {
    let mut start: i32;
    let mut step: i32;
    let mut stop: i32;
    (start, step, stop) = (match &*range {
        RANGE {
            start: __range_start,
            step: __range_step,
            stop: __range_stop,
            ..
        } => {
            match '__try0: {
                start = unwrap_break_err!(getInteger(__range_start.clone(), resize), '__try0);
                stop = unwrap_break_err!(getInteger(__range_stop.clone(), resize), '__try0);
                if (__range_step).is_some() {
                    step = unwrap_break_err!(getInteger(unwrap_break_err!(Util::getOption(__range_step.clone()), '__try0), resize), '__try0);
                } else {
                    step = if (start > stop) { -1 } else { 1 };
                }
                Ok::<_, &'static str>((start.clone(), step.clone(), stop.clone()))
            } {
                Ok((__try0_o0, __try0_o1, __try0_o2)) => {
                    start = __try0_o0;
                    step = __try0_o1;
                    stop = __try0_o2;
                }
                Err(__try0_err) => {
                    Error::terminate(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NFExpression.getIntegerRange"));
                            __mm_s.push_str(&*literal!(" range could not be parsed to integer values: "));
                            __mm_s.push_str(&*toString(range.clone())?);
                            ArcStr::from(__mm_s)
                        },
                        &(metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo")),
                    )?;
                    return Err(__try0_err);
                }
            }
            (start, step, stop)
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFExpression.getIntegerRange"));
                    __mm_s.push_str(&*literal!(" expression not RANGE(): "));
                    __mm_s.push_str(&*toString(range)?);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok((start, step, stop))
}

pub fn getInteger(mut exp: metamodelica::Ref<NFExpression>, mut resize: bool) -> Result<i32> {
    let mut i: i32;
    let mut e: metamodelica::Ref<NFExpression>;
    if resize {
        e = map(
            exp.clone(),
            (std::sync::Arc::new(replaceResizableParameter)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>>
                        + 'static,
                >),
        )?;
    } else {
        e = map(
            exp.clone(),
            (std::sync::Arc::new(replaceResizableParameterWithOriginal)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>>
                        + 'static,
                >),
        )?;
    }
    i = (match &*(SimplifyExp::simplify(e, false)?) {
        INTEGER { value: __esc_i } => {
            i = (*__esc_i).clone();
            i.clone()
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFExpression.getInteger"));
                    __mm_s.push_str(&*literal!(" cannot be parsed to an integer: "));
                    __mm_s.push_str(&*toString(exp)?);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(i)
}

pub(crate) fn makeTuple(
    mut expl: metamodelica::List<metamodelica::Ref<NFExpression>>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut tupleExp: metamodelica::Ref<NFExpression>;
    let mut tyl: metamodelica::List<metamodelica::Ref<Type::NFType>>;
    if ((expl).len() as i32) == 1 {
        tupleExp = (expl).head().cloned()?;
    } else {
        tyl = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Type::NFType>> = metamodelica::nil();
            for mut e in (expl.clone()).into_iter().cloned() {
                let __x = typeOf(e.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        tupleExp = metamodelica::Ref::new(NFExpression::TUPLE {
            ty: metamodelica::Ref::new(Type::NFType::TUPLE {
                types: tyl,
                names: None,
            }),
            elements: expl,
        });
    }
    Ok(tupleExp)
}

pub fn rangeSize(mut range: metamodelica::Ref<NFExpression>, mut resize: bool) -> Result<i32> {
    let mut size: i32 = Dimension::size(&(Type::nthDimension(typeOf(range.clone()), 1)?), resize)?;
    Ok(size)
}

pub fn rangeSizeExp(mut range: metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> {
    let mut size: metamodelica::Ref<NFExpression> =
        Dimension::sizeExp(&(Type::nthDimension(typeOf(range.clone()), 1)?))?;
    Ok(size)
}

pub fn applySubscripts(
    mut subscripts: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut exp: metamodelica::Ref<NFExpression>,
    mut applyToScope: bool,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    if (subscripts).is_empty() {
        outExp = exp;
    } else {
        outExp = applySubscript(
            &((subscripts).head().cloned()?),
            &exp,
            &((subscripts).rest()?),
            applyToScope,
        )?;
    }
    Ok(outExp)
}

pub fn applySubscript(
    mut subscript: &metamodelica::Ref<Subscript::NFSubscript>,
    mut exp: &metamodelica::Ref<NFExpression>,
    mut restSubscripts: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut applyToScope: bool,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    outExp = (match &**exp {
        CREF { cref: __exp_cref, .. } => applySubscriptCref(
            subscript.clone(),
            __exp_cref.clone(),
            restSubscripts.clone(),
            applyToScope,
        )?,
        TYPENAME { ty: __exp_ty } if ((restSubscripts).is_empty()) => {
            applySubscriptTypename(subscript.clone(), __exp_ty.clone())?
        }
        ARRAY { .. } => applySubscriptArray(subscript.clone(), exp.clone(), restSubscripts.clone(), applyToScope)?,
        RANGE { .. } if ((restSubscripts).is_empty()) => applySubscriptRange(subscript.clone(), exp.clone())?,
        CALL { .. } => applySubscriptCall(subscript.clone(), exp.clone(), restSubscripts.clone(), applyToScope)?,
        IF { .. } => applySubscriptIf(subscript.clone(), exp.clone(), restSubscripts.clone(), applyToScope)?,
        BINARY { .. }
            if (List::all(
                &(metamodelica::cons(subscript.clone(), restSubscripts.clone())),
                &move |__a0: metamodelica::Ref<Subscript::NFSubscript>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(isCheapSubscript(&__a0))
                },
            )?) =>
        {
            applySubscriptBinary(subscript.clone(), exp.clone(), restSubscripts.clone(), applyToScope)?
        }
        UNARY {
            exp: __exp_exp,
            operator: __exp_operator,
        } if (Type::isArray(&(Operator::typeOf(metamodelica::AsArg::as_arg(&__exp_operator))))
            && List::all(
                &(metamodelica::cons(subscript.clone(), restSubscripts.clone())),
                &move |__a0: metamodelica::Ref<Subscript::NFSubscript>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(isCheapSubscript(&__a0))
                },
            )?) =>
        {
            outExp = applySubscript(
                subscript,
                metamodelica::AsArg::as_arg(&__exp_exp),
                restSubscripts,
                applyToScope,
            )?;
            metamodelica::Ref::new(NFExpression::UNARY {
                operator: Operator::setType(typeOf(outExp.clone()), __exp_operator.clone()),
                exp: outExp,
            })
        }
        UNBOX { exp: __exp_exp, .. } => {
            outExp = applySubscript(
                subscript,
                metamodelica::AsArg::as_arg(&__exp_exp),
                restSubscripts,
                applyToScope,
            )?;
            unbox(outExp)
        }
        BOX { exp: __exp_exp } => r#box(
            &(applySubscript(
                subscript,
                metamodelica::AsArg::as_arg(&__exp_exp),
                restSubscripts,
                applyToScope,
            )?),
        ),
        CAST {
            exp: __exp_exp,
            ty: __exp_ty,
        } => {
            outExp = applySubscript(
                subscript,
                metamodelica::AsArg::as_arg(&__exp_exp),
                restSubscripts,
                applyToScope,
            )?;
            metamodelica::Ref::new(NFExpression::CAST {
                ty: Type::copyElementType(&(typeOf(outExp.clone())), metamodelica::AsArg::as_arg(&__exp_ty)),
                exp: outExp,
            })
        }
        _ => makeSubscriptedExp(
            metamodelica::cons(subscript.clone(), restSubscripts.clone()),
            exp.clone(),
            false,
        )?,
    });
    Ok(outExp)
}

pub(crate) fn applySubscriptCref(
    mut subscript: metamodelica::Ref<Subscript::NFSubscript>,
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut restSubscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut applyToScope: bool,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    let mut cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    cr = ComponentRef::mergeSubscripts(
        metamodelica::cons(subscript, restSubscripts),
        cref,
        applyToScope,
        false,
        false,
    )?;
    ty = ComponentRef::getSubscriptedType(&cr, false)?;
    outExp = metamodelica::Ref::new(NFExpression::CREF { ty: ty, cref: cr });
    Ok(outExp)
}

pub(crate) fn applySubscriptTypename(
    mut subscript: metamodelica::Ref<Subscript::NFSubscript>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    let mut sub: metamodelica::Ref<Subscript::NFSubscript>;
    let mut expl: metamodelica::Array<metamodelica::Ref<NFExpression>>;
    (sub, _) = Subscript::expandSlice(subscript.clone(), false)?;
    outExp = (match &*sub {
        Subscript::INDEX { .. } => applyIndexSubscriptTypename(ty, sub)?,
        Subscript::SLICE { .. } => metamodelica::Ref::new(NFExpression::SUBSCRIPTED_EXP {
            exp: metamodelica::Ref::new(NFExpression::TYPENAME { ty: ty.clone() }),
            subscripts: list![subscript],
            ty: metamodelica::Ref::new(Type::NFType::ARRAY {
                elementType: ty,
                dimensions: list![Subscript::toDimension(&sub)?],
            }),
            split: false,
        }),
        Subscript::WHOLE => metamodelica::Ref::new(NFExpression::TYPENAME { ty: ty }),
        Subscript::EXPANDED_SLICE { indices: __sub_indices } => {
            expl = Array::mapList(
                metamodelica::AsArg::as_arg(&__sub_indices),
                &({
                    let __pe_b0 = ty.clone();
                    move |__pe_a1| applyIndexSubscriptTypename(__pe_b0.clone(), __pe_a1)
                }),
            )?;
            makeArray(
                Type::liftArrayLeft(
                    ty,
                    &(Dimension::fromInteger(
                        metamodelica::arrayLength(expl.clone()),
                        Prefixes::Variability::CONSTANT.clone(),
                    )),
                ),
                expl.clone(),
                true,
            )
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outExp)
}

pub(crate) fn applyIndexSubscriptTypename(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut index: metamodelica::Ref<Subscript::NFSubscript>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut subscriptedExp: metamodelica::Ref<NFExpression>;
    let mut idx_exp: metamodelica::Ref<NFExpression>;
    let mut idx: i32;
    idx_exp = Subscript::toExp(&index)?;
    if isScalarLiteral(&idx_exp) {
        idx = toInteger(&idx_exp)?;
        subscriptedExp = (match &*ty {
            Type::BOOLEAN if (idx <= 2) => {
                if (idx == 1) {
                    metamodelica::Ref::new(NFExpression::BOOLEAN { value: false })
                } else {
                    metamodelica::Ref::new(NFExpression::BOOLEAN { value: true })
                }
            }
            Type::ENUMERATION { .. } => nthEnumLiteral(ty, idx)?,
            _ => return Err("match: no arm matched"),
        });
    } else {
        subscriptedExp = metamodelica::Ref::new(NFExpression::SUBSCRIPTED_EXP {
            exp: metamodelica::Ref::new(NFExpression::TYPENAME { ty: ty.clone() }),
            subscripts: list![index],
            ty: ty,
            split: false,
        });
    }
    Ok(subscriptedExp)
}

pub(crate) fn applySubscriptArray(
    mut subscript: metamodelica::Ref<Subscript::NFSubscript>,
    mut exp: metamodelica::Ref<NFExpression>,
    mut restSubscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut applyToScope: bool,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    let mut sub: metamodelica::Ref<Subscript::NFSubscript>;
    let mut s: metamodelica::Ref<Subscript::NFSubscript>;
    let mut rest_subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut expl: metamodelica::Array<metamodelica::Ref<NFExpression>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut literal: bool;
    if isEmptyArray(&exp) {
        outExp = makeSubscriptedExp(metamodelica::cons(subscript, restSubscripts), exp, false)?;
        return Ok(outExp);
    }
    (sub, _) = Subscript::expandSlice(subscript.clone(), false)?;
    outExp = (match &*sub {
        Subscript::INDEX { .. } => applyIndexSubscriptArray(exp, &sub, restSubscripts)?,
        Subscript::SLICE { .. } => makeSubscriptedExp(metamodelica::cons(subscript, restSubscripts), exp, false)?,
        Subscript::WHOLE => {
            if (restSubscripts).is_empty() {
                outExp = exp;
            } else {
                let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp) {
                    Deref @ ARRAY { ty: __pa0, elements: __pa1, literal: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                ty = metamodelica::Own::own(__pa0);
                expl = metamodelica::Own::own(__pa1);
                literal = metamodelica::Own::own(__pa2);
                let (__pa3, __pa4) = ::match_deref::match_deref! { match &(restSubscripts.clone()) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                s = metamodelica::Own::own(__pa3);
                rest_subs = metamodelica::Own::own(__pa4);
                expl = Array::map(
                    expl.clone(),
                    &({
                        let __pe_b0 = s;
                        let __pe_b2 = rest_subs;
                        let __pe_b3 = applyToScope;
                        move |__pe_a1| applySubscript(&__pe_b0, &__pe_a1, &__pe_b2, __pe_b3.clone())
                    }),
                )?;
                (ty, literal) = typeSubscriptedArray(expl.clone(), &restSubscripts, ty, literal)?;
                outExp = makeArray(ty, expl.clone(), literal);
            }
            outExp
        }
        Subscript::EXPANDED_SLICE { indices: __sub_indices } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exp.clone()) {
                Deref @ ARRAY { ty: __pa0, literal: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa0);
            literal = metamodelica::Own::own(__pa1);
            expl = Array::mapList(
                metamodelica::AsArg::as_arg(&__sub_indices),
                &({
                    let __pe_b0 = exp;
                    let __pe_b2 = restSubscripts.clone();
                    move |__pe_a1| applyIndexSubscriptArray(__pe_b0.clone(), &__pe_a1, __pe_b2.clone())
                }),
            )?;
            (ty, literal) = typeSubscriptedArray(expl.clone(), &restSubscripts, ty, literal)?;
            makeArray(ty, expl.clone(), literal)
        }
        Subscript::SPLIT_INDEX { .. } => makeSubscriptedExp(metamodelica::cons(subscript, restSubscripts), exp, false)?,
        _ => return Err("match: no arm matched"),
    });
    Ok(outExp)
}

pub(crate) fn typeSubscriptedArray(
    mut elements: metamodelica::Array<metamodelica::Ref<NFExpression>>,
    mut subscripts: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut literal: bool,
) -> Result<(metamodelica::Ref<Type::NFType>, bool)> {
    let mut ty: metamodelica::Ref<Type::NFType> = ty;
    let mut literal: bool = literal;
    let mut count: i32;
    let mut e: metamodelica::Ref<NFExpression>;
    count = metamodelica::arrayLength(elements.clone());
    if count > 0 {
        e = ({
            let __elt = (*metamodelica::index_checked(&elements.borrow(), 1)?).clone();
            __elt
        });
        ty = typeOf(e.clone());
        literal = literal && isLiteral(&e)?;
    } else {
        ty = Type::subscript(Type::unliftArray(ty)?, subscripts, true)?;
    }
    ty = Type::liftArrayLeft(
        ty,
        &(Dimension::fromInteger(count, Prefixes::Variability::CONSTANT.clone())),
    );
    Ok((ty, literal))
}

pub(crate) fn applyIndexSubscriptArray(
    mut exp: metamodelica::Ref<NFExpression>,
    mut index: &metamodelica::Ref<Subscript::NFSubscript>,
    mut restSubscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    outExp = applyIndexExpArray(exp, Subscript::toExp(index)?, restSubscripts)?;
    Ok(outExp)
}

pub(crate) fn applyIndexExpArray(
    mut exp: metamodelica::Ref<NFExpression>,
    mut index: metamodelica::Ref<NFExpression>,
    mut restSubscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    let mut expl: metamodelica::Array<metamodelica::Ref<NFExpression>>;
    let mut idx: i32;
    if isScalarLiteral(&index) {
        let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ ARRAY { elements: __pa0, .. } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        expl = metamodelica::Own::own(__pa0);
        idx = toInteger(&index)?;
        if idx > 0 && idx <= metamodelica::arrayLength(expl.clone()) {
            outExp = applySubscripts(
                &restSubscripts,
                ({
                    let __elt = (*metamodelica::index_checked(&expl.borrow(), idx)?).clone();
                    __elt
                }),
                false,
            )?;
            return Ok(outExp);
        }
    }
    outExp = makeSubscriptedExp(
        metamodelica::cons(
            metamodelica::Ref::new(Subscript::NFSubscript::INDEX { index: index }),
            restSubscripts,
        ),
        exp,
        false,
    )?;
    Ok(outExp)
}

pub(crate) fn applySubscriptRange(
    mut subscript: metamodelica::Ref<Subscript::NFSubscript>,
    mut exp: metamodelica::Ref<NFExpression>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    let mut sub: metamodelica::Ref<Subscript::NFSubscript>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut expl: metamodelica::Array<metamodelica::Ref<NFExpression>>;
    (sub, _) = Subscript::expandSlice(subscript.clone(), false)?;
    outExp = (match &*sub {
        Subscript::INDEX { .. } => applyIndexSubscriptRange(exp, sub)?,
        Subscript::SLICE { .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
                Deref @ RANGE { ty: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa0);
            ty = metamodelica::Ref::new(Type::NFType::ARRAY {
                elementType: Type::unliftArray(ty)?,
                dimensions: list![Subscript::toDimension(&sub)?],
            });
            metamodelica::Ref::new(NFExpression::SUBSCRIPTED_EXP {
                exp: exp,
                subscripts: list![subscript],
                ty: ty,
                split: false,
            })
        }
        Subscript::WHOLE => exp,
        Subscript::EXPANDED_SLICE { indices: __sub_indices } => {
            expl = Array::mapList(
                metamodelica::AsArg::as_arg(&__sub_indices),
                &({
                    let __pe_b0 = exp.clone();
                    move |__pe_a1| applyIndexSubscriptRange(__pe_b0.clone(), __pe_a1)
                }),
            )?;
            let __pa0 = ::match_deref::match_deref! { match &(exp) {
                Deref @ RANGE { ty: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa0);
            makeArray(
                Type::liftArrayLeft(
                    ty,
                    &(Dimension::fromInteger(
                        metamodelica::arrayLength(expl.clone()),
                        Prefixes::Variability::CONSTANT.clone(),
                    )),
                ),
                expl.clone(),
                false,
            )
        }
        Subscript::SPLIT_INDEX { .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
                Deref @ RANGE { ty: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa0);
            ty = Type::unliftArray(ty)?;
            metamodelica::Ref::new(NFExpression::SUBSCRIPTED_EXP {
                exp: exp,
                subscripts: list![sub],
                ty: ty,
                split: true,
            })
        }
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFExpression.applySubscriptRange"));
                    __mm_s.push_str(&*literal!(" got unknown subscript '"));
                    __mm_s.push_str(&*Subscript::toString(&sub)?);
                    __mm_s.push_str(&*literal!("'"));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(outExp)
}

pub(crate) fn applyIndexSubscriptRange(
    mut rangeExp: metamodelica::Ref<NFExpression>,
    mut index: metamodelica::Ref<Subscript::NFSubscript>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    let mut index_exp: metamodelica::Ref<NFExpression>;
    let mut start_exp: metamodelica::Ref<NFExpression>;
    let mut stop_exp: metamodelica::Ref<NFExpression>;
    let mut step_exp: Option<metamodelica::Ref<NFExpression>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let __pa0 = ::match_deref::match_deref! { match &(index.clone()) {
        Deref @ Subscript::INDEX { index: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    index_exp = metamodelica::Own::own(__pa0);
    let (__pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(rangeExp.clone()) {
        Deref @ RANGE { ty: __pa1, start: __pa2, step: __pa3, stop: __pa4 } => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa1);
    start_exp = metamodelica::Own::own(__pa2);
    step_exp = metamodelica::Own::own(__pa3);
    stop_exp = metamodelica::Own::own(__pa4);
    if isScalarLiteral(&index_exp)
        && isScalarLiteral(&start_exp)
        && Util::applyOptionOrDefault(
            step_exp.clone(),
            &move |__a0: metamodelica::Ref<NFExpression>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isScalarLiteral(&__a0))
            },
            true,
        )?
    {
        outExp = applyIndexSubscriptRange2(start_exp, step_exp, stop_exp, toInteger(&index_exp)?)?;
    } else if isScalarLiteral(&index_exp) && toInteger(&index_exp)? == 1 {
        outExp = start_exp;
    } else {
        subs = list![index];
        ty = Type::subscript(ty, &subs, true)?;
        outExp = metamodelica::Ref::new(NFExpression::SUBSCRIPTED_EXP {
            exp: rangeExp,
            subscripts: subs,
            ty: ty,
            split: false,
        });
    }
    Ok(outExp)
}

pub(crate) fn applyIndexSubscriptRange2(
    mut startExp: metamodelica::Ref<NFExpression>,
    mut stepExp: Option<metamodelica::Ref<NFExpression>>,
    mut stopExp: metamodelica::Ref<NFExpression>,
    mut index: i32,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut subscriptedExp: metamodelica::Ref<NFExpression>;
    let mut iidx: i32;
    let mut ridx: metamodelica::Real;
    subscriptedExp = (::match_deref::match_deref! { match &((startExp.clone(), stepExp)) {
        (Deref @ INTEGER { .. }, Some(Deref @ INTEGER { value: __esc_iidx })) => {
            iidx = (*__esc_iidx).clone();
            metamodelica::Ref::new(NFExpression::INTEGER { value: var_field!((*startExp).value, NFExpression::INTEGER).clone() + (index - 1) * iidx.clone() })
        },
        (Deref @ INTEGER { .. }, _) => metamodelica::Ref::new(NFExpression::INTEGER { value: var_field!((*startExp).value, NFExpression::INTEGER).clone() + index - 1 }),
        (Deref @ REAL { .. }, Some(Deref @ REAL { value: __esc_ridx })) => {
            ridx = (*__esc_ridx).clone();
            metamodelica::Ref::new(NFExpression::REAL { value: var_field!((*startExp).value, NFExpression::REAL).clone() + (metamodelica::OrderedFloat((index - 1) as f64)) * ridx.clone() })
        },
        (Deref @ REAL { .. }, _) => metamodelica::Ref::new(NFExpression::REAL { value: var_field!((*startExp).value, NFExpression::REAL).clone() + metamodelica::OrderedFloat((index) as f64) - metamodelica::OrderedFloat(1.0_f64) }),
        (Deref @ BOOLEAN { .. }, _) => if (index == 1) {startExp} else {stopExp},
        (Deref @ ENUM_LITERAL { index: __esc_iidx, .. }, _) => {
            iidx = (*__esc_iidx).clone();
            iidx = iidx.clone() + index - 1;
            nthEnumLiteral(var_field!((*startExp).ty, NFExpression::ENUM_LITERAL).clone(), iidx.clone())?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(subscriptedExp)
}

pub(crate) fn applySubscriptCall(
    mut subscript: metamodelica::Ref<Subscript::NFSubscript>,
    mut exp: metamodelica::Ref<NFExpression>,
    mut restSubscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut applyToScope: bool,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    let mut call: metamodelica::Ref<Call::NFCall>;
    let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CALL { call: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    call = metamodelica::Own::own(__pa0);
    outExp = (::match_deref::match_deref! { match &(call.clone()) {
        Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::ListNode::Cons { head: arg, tail: Deref @ metamodelica::ListNode::Nil }, attributes: __call_attributes, r#fn: __call_fn, purity: __call_purity, ty: __call_ty, var: __call_var } if (Function::Function::isSubscriptableBuiltin(metamodelica::AsArg::as_arg(&__call_fn))) => {
            let mut ty: metamodelica::Ref<Type::NFType>;
            let mut arg = (*arg).clone();
            arg = applySubscript(&subscript, metamodelica::AsArg::as_arg(&arg), &restSubscripts, applyToScope)?;
            ty = Type::copyDims(typeOf(arg.clone()), __call_ty.clone());
            metamodelica::Ref::new(NFExpression::CALL { call: metamodelica::Ref::new(Call::NFCall::TYPED_CALL { r#fn: __call_fn.clone(), ty: ty, var: __call_var.clone(), purity: __call_purity.clone(), arguments: list![arg.clone()], attributes: __call_attributes.clone() }) })
        },
        Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } => {
            applySubscriptArrayConstructor(subscript, call, restSubscripts)?
        },
        _ => {
            makeSubscriptedExp(metamodelica::cons(subscript, restSubscripts), exp, false)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn applySubscriptArrayConstructor(
    mut subscript: metamodelica::Ref<Subscript::NFSubscript>,
    mut call: metamodelica::Ref<Call::NFCall>,
    mut restSubscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    if Subscript::isIndex(&subscript) && (restSubscripts).is_empty() {
        outExp = applyIndexSubscriptArrayConstructor(&call, &subscript)?;
    } else {
        outExp = makeSubscriptedExp(
            metamodelica::cons(subscript, restSubscripts),
            metamodelica::Ref::new(NFExpression::CALL { call: call }),
            false,
        )?;
    }
    Ok(outExp)
}

pub(crate) fn applyIndexSubscriptArrayConstructor(
    mut call: &metamodelica::Ref<Call::NFCall>,
    mut index: &metamodelica::Ref<Subscript::NFSubscript>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut subscriptedExp: metamodelica::Ref<NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut var: Variability;
    let mut pur: Purity;
    let mut exp: metamodelica::Ref<NFExpression>;
    let mut iter_exp: metamodelica::Ref<NFExpression>;
    let mut iters: metamodelica::List<(metamodelica::Ref<InstNode::InstNode>, metamodelica::Ref<NFExpression>)>;
    let mut iter: metamodelica::Ref<InstNode::InstNode>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &((*call)) {
        Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { ty: __pa0, var: __pa1, purity: __pa2, exp: __pa3, iters: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    var = metamodelica::Own::own(__pa1);
    pur = metamodelica::Own::own(__pa2);
    exp = metamodelica::Own::own(__pa3);
    iters = metamodelica::Own::own(__pa4);
    let ((__pa5, __pa6), __pa7) = List::splitLast(iters)?;
    iter = metamodelica::Own::own(__pa5);
    iter_exp = metamodelica::Own::own(__pa6);
    iters = metamodelica::Own::own(__pa7);
    iter_exp = applySubscript(index, &iter_exp, &(metamodelica::nil()), false)?;
    subscriptedExp = replaceIterator(exp, &iter, &iter_exp)?;
    if !((iters).is_empty()) {
        subscriptedExp = metamodelica::Ref::new(NFExpression::CALL {
            call: metamodelica::Ref::new(Call::NFCall::TYPED_ARRAY_CONSTRUCTOR {
                ty: Type::unliftArray(ty)?,
                var: var,
                purity: pur,
                exp: subscriptedExp,
                iters: iters,
            }),
        });
    }
    Ok(subscriptedExp)
}

pub(crate) fn applySubscriptIf(
    mut subscript: metamodelica::Ref<Subscript::NFSubscript>,
    mut exp: metamodelica::Ref<NFExpression>,
    mut restSubscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut applyToScope: bool,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    let mut cond: metamodelica::Ref<NFExpression>;
    let mut tb: metamodelica::Ref<NFExpression>;
    let mut fb: metamodelica::Ref<NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ IF { ty: __pa0, condition: __pa1, trueBranch: __pa2, falseBranch: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    cond = metamodelica::Own::own(__pa1);
    tb = metamodelica::Own::own(__pa2);
    fb = metamodelica::Own::own(__pa3);
    if Type::isConditionalArray(&ty) {
        match '__try4: {
            tb = unwrap_break_err!(applySubscript(&subscript, &tb, &restSubscripts, applyToScope), '__try4);
            fb = unwrap_break_err!(applySubscript(&subscript, &fb, &restSubscripts, applyToScope), '__try4);
            ty =
                unwrap_break_err!(Type::setConditionalArrayTypes(&ty, typeOf(tb.clone()), typeOf(fb.clone())), '__try4);
            outExp = metamodelica::Ref::new(NFExpression::IF {
                ty: ty.clone(),
                condition: cond.clone(),
                trueBranch: tb.clone(),
                falseBranch: fb.clone(),
            });
            Ok::<_, &'static str>((outExp.clone(),))
        } {
            Ok((__try4_o0,)) => {
                outExp = __try4_o0;
            }
            Err(_) => {
                outExp = makeSubscriptedExp(
                    metamodelica::cons(subscript.clone(), restSubscripts.clone()),
                    exp.clone(),
                    false,
                )?;
            }
        }
    } else {
        tb = applySubscript(&subscript, &tb, &restSubscripts, applyToScope)?;
        fb = applySubscript(&subscript, &fb, &restSubscripts, applyToScope)?;
        ty = typeOf(tb.clone());
        outExp = metamodelica::Ref::new(NFExpression::IF {
            ty: ty,
            condition: cond,
            trueBranch: tb,
            falseBranch: fb,
        });
    }
    Ok(outExp)
}

pub(crate) fn isCheapSubscript(mut subscript: &metamodelica::Ref<Subscript::NFSubscript>) -> bool {
    let mut cheap: bool;
    cheap = (match &**subscript {
        Subscript::INDEX {
            index: __subscript_index,
        } => {
            isCref(metamodelica::AsArg::as_arg(&__subscript_index))
                || isScalarLiteral(metamodelica::AsArg::as_arg(&__subscript_index))
        }
        Subscript::WHOLE => true,
        _ => false,
    });
    cheap
}

pub(crate) fn applySubscriptBinary(
    mut subscript: metamodelica::Ref<Subscript::NFSubscript>,
    mut exp: metamodelica::Ref<NFExpression>,
    mut restSubscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut applyToScope: bool,
) -> Result<metamodelica::Ref<NFExpression>> {
    use crate::NFOperator::Op;
    let mut outExp: metamodelica::Ref<NFExpression>;
    let mut e1: metamodelica::Ref<NFExpression>;
    let mut e2: metamodelica::Ref<NFExpression>;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    let mut scalar_op: Op;
    let mut sub1: bool;
    let mut sub2: bool;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ BINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e1 = metamodelica::Own::own(__pa0);
    op = metamodelica::Own::own(__pa1);
    e2 = metamodelica::Own::own(__pa2);
    (sub1, sub2, scalar_op) = (match op.op.clone() {
        Operator::Op::ADD if (Type::isArray(&op.ty)) => (true, true, Op::ADD.clone()),
        Operator::Op::SUB if (Type::isArray(&op.ty)) => (true, true, Op::SUB.clone()),
        Operator::Op::ADD_EW => (true, true, Op::ADD.clone()),
        Operator::Op::SUB_EW => (true, true, Op::SUB.clone()),
        Operator::Op::MUL_EW => (true, true, Op::MUL.clone()),
        Operator::Op::DIV_EW => (true, true, Op::DIV.clone()),
        Operator::Op::POW_EW => (true, true, Op::POW.clone()),
        Operator::Op::ADD_ARRAY_SCALAR { .. } => (true, false, Op::ADD.clone()),
        Operator::Op::SUB_ARRAY_SCALAR => (true, false, Op::SUB.clone()),
        Operator::Op::MUL_ARRAY_SCALAR { .. } => (true, false, Op::MUL.clone()),
        Operator::Op::DIV_ARRAY_SCALAR { .. } => (true, false, Op::DIV.clone()),
        Operator::Op::POW_ARRAY_SCALAR { .. } => (true, false, Op::POW.clone()),
        Operator::Op::ADD_SCALAR_ARRAY => (false, true, Op::ADD.clone()),
        Operator::Op::SUB_SCALAR_ARRAY { .. } => (false, true, Op::SUB.clone()),
        Operator::Op::MUL_SCALAR_ARRAY => (false, true, Op::MUL.clone()),
        Operator::Op::DIV_SCALAR_ARRAY { .. } => (false, true, Op::DIV.clone()),
        Operator::Op::POW_SCALAR_ARRAY { .. } => (false, true, Op::POW.clone()),
        _ => (false, false, op.op.clone()),
    });
    if !(sub1 || sub2) {
        outExp = makeSubscriptedExp(metamodelica::cons(subscript, restSubscripts), exp, false)?;
        return Ok(outExp);
    }
    if sub1 {
        e1 = applySubscript(&subscript, &e1, &restSubscripts, applyToScope)?;
    }
    if sub2 {
        e2 = applySubscript(&subscript, &e2, &restSubscripts, applyToScope)?;
    }
    assign_field!(op.ty = typeOf(if (sub1) { e1.clone() } else { e2.clone() }));
    if Type::isScalar(&op.ty) {
        assign_field!(op.op = scalar_op);
    }
    outExp = metamodelica::Ref::new(NFExpression::BINARY {
        exp1: e1,
        operator: op,
        exp2: e2,
    });
    Ok(outExp)
}

pub(crate) fn makeSubscriptedExp(
    mut subscripts: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut exp: metamodelica::Ref<NFExpression>,
    mut backend: bool,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    let mut e: metamodelica::Ref<NFExpression>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut extra_subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut dim_count: i32;
    let mut split: bool;
    (e, subs, ty, split) = (match &*exp {
        SUBSCRIPTED_EXP {
            exp: __exp_exp,
            split: __exp_split,
            subscripts: __exp_subscripts,
            ..
        } => (
            __exp_exp.clone(),
            __exp_subscripts.clone(),
            typeOf(__exp_exp.clone()),
            __exp_split.clone(),
        ),
        _ => (exp.clone(), metamodelica::nil(), typeOf(exp.clone()), false),
    });
    if !(split) {
        split = List::any(
            &subscripts,
            &move |__a0: metamodelica::Ref<Subscript::NFSubscript>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(Subscript::isSplitIndex(&__a0))
            },
        )?;
    }
    dim_count = Type::dimensionCount(ty.clone());
    (subs, extra_subs) = Subscript::mergeList(subscripts.clone(), subs, dim_count, backend)?;
    if !((extra_subs).is_empty()) {
        Error::terminate(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFExpression.makeSubscriptedExp"));
                __mm_s.push_str(&*literal!(": too few dimensions in "));
                __mm_s.push_str(&*toString(exp)?);
                __mm_s.push_str(&*literal!(" to apply subscripts "));
                __mm_s.push_str(&*Subscript::toStringList(subscripts)?);
                ArcStr::from(__mm_s)
            },
            &(metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo")),
        )?;
    }
    ty = Type::subscript(ty, &subs, true)?;
    outExp = metamodelica::Ref::new(NFExpression::SUBSCRIPTED_EXP {
        exp: e,
        subscripts: subs,
        ty: ty,
        split: split,
    });
    Ok(outExp)
}

pub fn replaceIterator(
    mut exp: metamodelica::Ref<NFExpression>,
    mut iterator: &metamodelica::Ref<InstNode::InstNode>,
    mut iteratorValue: &metamodelica::Ref<NFExpression>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression> = exp;
    exp = map(
        exp,
        (std::sync::Arc::new({
            let __pe_b1 = iterator.clone();
            let __pe_b2 = iteratorValue.clone();
            move |__pe_a0| replaceIterator2(__pe_a0, &__pe_b1, __pe_b2.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(exp)
}

pub(crate) fn replaceIterator2(
    mut exp: metamodelica::Ref<NFExpression>,
    mut iterator: &metamodelica::Ref<InstNode::InstNode>,
    mut iteratorValue: metamodelica::Ref<NFExpression>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    outExp = (::match_deref::match_deref! { match &(&*exp) {
        Deref @ CREF { cref: Deref @ ComponentRef::CREF { .. }, .. } if (ComponentRef::isSimple(var_field!((*exp).cref, NFExpression::CREF))) => {
            if (InstNode::refEqual(iterator, &(ComponentRef::node(var_field!((*exp).cref, NFExpression::CREF))?))?) {iteratorValue} else {exp.clone()}
        },
        Deref @ CREF { cref: Deref @ ComponentRef::CREF { .. }, .. } => {
            let mut node: metamodelica::Ref<InstNode::InstNode>;
            let mut fields: metamodelica::List<ArcStr>;
            node = ComponentRef::node(&(ComponentRef::last(var_field!((*exp).cref, NFExpression::CREF))))?;
            if InstNode::refEqual(iterator, &node)? {
                outExp = iteratorValue;
                fields = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut n in (((ComponentRef::nodes(var_field!((*exp).cref, NFExpression::CREF), metamodelica::nil())?)).rest()?).into_iter().cloned() {
            let __x = InstNode::name(&(n.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                for mut f in &*fields {
                    outExp = recordElement(metamodelica::AsArg::as_arg(&f), &outExp)?;
                }
            } else {
                outExp = exp.clone();
            }
            outExp
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn containsIterator(
    mut exp: metamodelica::Ref<NFExpression>,
    mut iterator: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<bool> {
    fn containsIterator2(
        mut exp: &metamodelica::Ref<NFExpression>,
        mut iterator: &metamodelica::Ref<InstNode::InstNode>,
    ) -> Result<bool> {
        let mut res: bool;
        res = (match &**exp {
            CREF { cref: __exp_cref, .. } if (ComponentRef::isIterator(metamodelica::AsArg::as_arg(&__exp_cref))) => {
                InstNode::refEqual(
                    &(ComponentRef::node(&(ComponentRef::last(metamodelica::AsArg::as_arg(&__exp_cref))))?),
                    iterator,
                )?
            }
            _ => false,
        });
        Ok(res)
    }

    let mut res: bool;
    res = contains(
        exp,
        &({
            let __pe_b1 = iterator.clone();
            move |__pe_a0| containsIterator2(&__pe_a0, &__pe_b1)
        }),
    )?;
    Ok(res)
}

pub(crate) fn arrayFromList(
    mut inExps: metamodelica::List<metamodelica::Ref<NFExpression>>,
    mut elemTy: metamodelica::Ref<Type::NFType>,
    mut inDims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    outExp = arrayFromList_impl(inExps, elemTy, inDims.reverse())?;
    Ok(outExp)
}

pub(crate) fn arrayFromList_impl(
    mut inExps: metamodelica::List<metamodelica::Ref<NFExpression>>,
    mut elemTy: metamodelica::Ref<Type::NFType>,
    mut inDims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    let mut ldim: metamodelica::Ref<Dimension::NFDimension>;
    let mut restdims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut newlst: metamodelica::List<metamodelica::Ref<NFExpression>>;
    let mut partexps: metamodelica::List<metamodelica::List<metamodelica::Ref<NFExpression>>>;
    let mut dimsize: i32;
    Error::assertion(
        !((inDims).is_empty()),
        literal!("Empty dimension list given in arrayFromList."),
        &(metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo")),
    )?;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inDims.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ldim = metamodelica::Own::own(__pa0);
    restdims = metamodelica::Own::own(__pa1);
    dimsize = Dimension::size(&ldim, false)?;
    ty = Type::liftArrayLeft(elemTy, &ldim);
    if List::hasOneElement(&inDims) {
        Error::assertion(
            dimsize == ((inExps).len() as i32),
            literal!("Length mismatch in arrayFromList."),
            &(metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo")),
        )?;
        outExp = makeArray(
            ty,
            metamodelica::arrayFromVec(inExps.into_iter().cloned().collect()),
            false,
        );
        return Ok(outExp);
    }
    partexps = List::partition(inExps, dimsize)?;
    newlst = metamodelica::nil();
    for mut arrexp in &*partexps {
        newlst = metamodelica::cons(
            makeArray(
                ty.clone(),
                metamodelica::arrayFromVec(arrexp.clone().into_iter().cloned().collect()),
                false,
            ),
            newlst,
        );
    }
    newlst = newlst.reverse();
    outExp = arrayFromList_impl(newlst, ty, restdims)?;
    Ok(outExp)
}

pub(crate) fn makeEnumLiteral(
    mut enumType: metamodelica::Ref<Type::NFType>,
    mut index: i32,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut literal: metamodelica::Ref<NFExpression>;
    let mut literals: metamodelica::List<ArcStr>;
    let __pa0 = ::match_deref::match_deref! { match &(enumType.clone()) {
        Deref @ Type::ENUMERATION { literals: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    literals = metamodelica::Own::own(__pa0);
    literal = metamodelica::Ref::new(NFExpression::ENUM_LITERAL {
        ty: enumType,
        name: (literals).get(index)?,
        index: index,
    });
    Ok(literal)
}

pub(crate) fn makeEnumLiterals(
    mut enumType: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::List<metamodelica::Ref<NFExpression>>> {
    let mut literals: metamodelica::List<metamodelica::Ref<NFExpression>>;
    let mut lits: metamodelica::List<ArcStr>;
    let __pa0 = ::match_deref::match_deref! { match &(enumType.clone()) {
        Deref @ Type::ENUMERATION { literals: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    lits = metamodelica::Own::own(__pa0);
    literals = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
        let __thr_src0 = lits.clone();
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let mut __thr_it1 = (1..=((lits).len() as i32)).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(l), Some(i)) => {
                    let __x = metamodelica::Ref::new(NFExpression::ENUM_LITERAL {
                        ty: enumType.clone(),
                        name: l.clone(),
                        index: i.clone(),
                    });
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    Ok(literals)
}

pub(crate) fn isIntegerValue(mut exp: &metamodelica::Ref<NFExpression>, mut value: i32) -> bool {
    let mut result: bool;
    result = (match &**exp {
        INTEGER { value: __exp_value } => __exp_value.clone() == value,
        _ => false,
    });
    result
}

pub(crate) fn toInteger(mut exp: &metamodelica::Ref<NFExpression>) -> Result<i32> {
    let mut i: i32;
    i = (match &**exp {
        INTEGER { value: __exp_value } => __exp_value.clone(),
        BOOLEAN { value: __exp_value } => {
            if (__exp_value.clone()) {
                2
            } else {
                1
            }
        }
        ENUM_LITERAL { index: __exp_index, .. } => __exp_index.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(i)
}

pub(crate) fn toStringTyped(mut exp: metamodelica::Ref<NFExpression>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("/*"));
        __mm_s.push_str(&*Type::toString(&(typeOf(exp.clone())))?);
        __mm_s.push_str(&*literal!("*/ "));
        __mm_s.push_str(&*toString(exp)?);
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

pub fn toString(mut exp: metamodelica::Ref<NFExpression>) -> Result<ArcStr> {
    '__tco: loop {
        let mut t: metamodelica::Ref<Type::NFType>;
        ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ INTEGER { value: __exp_value } => return Ok(intString(__exp_value.clone())),
            Deref @ REAL { value: __exp_value } => return Ok(realString(__exp_value.clone())),
            Deref @ STRING { value: __exp_value } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*System::escapedString(__exp_value.clone(), false)); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }),
            Deref @ BOOLEAN { value: __exp_value } => return Ok(boolString(__exp_value.clone())),
            Deref @ ENUM_LITERAL { ty: __esc_t @ Deref @ Type::ENUMERATION { .. }, name: __exp_name, .. } => {
                t = (*__esc_t).clone();
                return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(var_field!((*t).typePath, Type::NFType::ENUMERATION).clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*__exp_name); ArcStr::from(__mm_s) })
            },
            Deref @ CLKCONST { clk: __exp_clk } => return Ok(ClockKind::toString(metamodelica::AsArg::as_arg(&__exp_clk))?),
            Deref @ CREF { cref: __exp_cref, .. } => return Ok(ComponentRef::toString(metamodelica::AsArg::as_arg(&__exp_cref))?),
            Deref @ TYPENAME { ty: __exp_ty } => return Ok(Type::typenameString(&(Type::arrayElementType(metamodelica::AsArg::as_arg(&__exp_ty))))?),
            Deref @ ARRAY { .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut e in (var_field!((*exp).elements, NFExpression::ARRAY).clone()).borrow().iter() {
                let __x = toString(e.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }), literal!(", "))); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }),
            Deref @ MATRIX { elements: __exp_elements } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*stringDelimitList(({
            let mut __acc: metamodelica::List<_> = metamodelica::nil();
            for mut el in (__exp_elements.clone()).into_iter().cloned() {
                let __x = stringDelimitList(({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut e in (el.clone()).into_iter().cloned() {
                let __x = toString(e.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }), literal!(", "));
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }), literal!("; "))); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }),
            Deref @ RANGE { start: __exp_start, step: __exp_step, stop: __exp_stop, .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*operandString(__exp_start.clone(), &exp, false)?); __mm_s.push_str(&*if ((__exp_step).is_some()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*operandString(Util::getOption(__exp_step.clone())?, &exp, false)?); ArcStr::from(__mm_s) }} else {literal!("")}); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*operandString(__exp_stop.clone(), &exp, false)?); ArcStr::from(__mm_s) }),
            Deref @ TUPLE { elements: __exp_elements, .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*stringDelimitList(({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut e in (__exp_elements.clone()).into_iter().cloned() {
                let __x = toString(e.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }), literal!(", "))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }),
            Deref @ RECORD { elements: __exp_elements, path: __exp_path, .. } => return Ok(List::toStringCustom(__exp_elements.clone(), &toString, AbsynUtil::pathString(__exp_path.clone(), literal!("."), true, false)?, literal!("("), literal!(", "), literal!(")"), true, 0)?),
            Deref @ CALL { call: __exp_call } => return Ok(Call::toString(metamodelica::AsArg::as_arg(&__exp_call))?),
            Deref @ SIZE { dimIndex: __exp_dimIndex, exp: __exp_exp } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("size(")); __mm_s.push_str(&*toString(__exp_exp.clone())?); __mm_s.push_str(&*if ((__exp_dimIndex).is_some()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*toString(Util::getOption(__exp_dimIndex.clone())?)?); ArcStr::from(__mm_s) }} else {literal!("")}); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }),
            Deref @ END { .. } => return Ok(literal!("end")),
            Deref @ MULTARY { arguments: __exp_arguments, inv_arguments: __exp_inv_arguments, operator: __exp_operator } if ((__exp_inv_arguments).is_empty()) => return Ok(multaryString(__exp_arguments.clone(), &exp, metamodelica::AsArg::as_arg(&__exp_operator), false)?),
            Deref @ MULTARY { arguments: __exp_arguments, inv_arguments: __exp_inv_arguments, operator: __exp_operator } if ((__exp_arguments).is_empty() && Operator::isDashClassification(Operator::getMathClassification(metamodelica::AsArg::as_arg(&__exp_operator))?)) => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-")); __mm_s.push_str(&*multaryString(__exp_inv_arguments.clone(), &exp, metamodelica::AsArg::as_arg(&__exp_operator), true)?); ArcStr::from(__mm_s) }),
            Deref @ MULTARY { arguments: __exp_arguments, inv_arguments: __exp_inv_arguments, operator: __exp_operator } if ((__exp_arguments).is_empty()) => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("1/")); __mm_s.push_str(&*multaryString(__exp_inv_arguments.clone(), &exp, metamodelica::AsArg::as_arg(&__exp_operator), true)?); ArcStr::from(__mm_s) }),
            Deref @ MULTARY { arguments: __exp_arguments, inv_arguments: __exp_inv_arguments, operator: __exp_operator } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*multaryString(__exp_arguments.clone(), &exp, metamodelica::AsArg::as_arg(&__exp_operator), true)?); __mm_s.push_str(&*Operator::symbol(&(Operator::invert(__exp_operator.clone())?), &(literal!(" ")))?); __mm_s.push_str(&*multaryString(__exp_inv_arguments.clone(), &exp, metamodelica::AsArg::as_arg(&__exp_operator), true)?); ArcStr::from(__mm_s) }),
            Deref @ BINARY { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*operandString(__exp_exp1.clone(), &exp, true)?); __mm_s.push_str(&*Operator::symbol(metamodelica::AsArg::as_arg(&__exp_operator), &(literal!(" ")))?); __mm_s.push_str(&*operandString(__exp_exp2.clone(), &exp, false)?); ArcStr::from(__mm_s) }),
            Deref @ UNARY { exp: __exp_exp, operator: __exp_operator } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*Operator::symbol(metamodelica::AsArg::as_arg(&__exp_operator), &(literal!("")))?); __mm_s.push_str(&*operandString(__exp_exp.clone(), &exp, false)?); ArcStr::from(__mm_s) }),
            Deref @ LBINARY { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*operandString(__exp_exp1.clone(), &exp, true)?); __mm_s.push_str(&*Operator::symbol(metamodelica::AsArg::as_arg(&__exp_operator), &(literal!(" ")))?); __mm_s.push_str(&*operandString(__exp_exp2.clone(), &exp, false)?); ArcStr::from(__mm_s) }),
            Deref @ LUNARY { exp: __exp_exp, operator: __exp_operator } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*Operator::symbol(metamodelica::AsArg::as_arg(&__exp_operator), &(literal!("")))?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*operandString(__exp_exp.clone(), &exp, false)?); ArcStr::from(__mm_s) }),
            Deref @ RELATION { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator, .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*operandString(__exp_exp1.clone(), &exp, true)?); __mm_s.push_str(&*Operator::symbol(metamodelica::AsArg::as_arg(&__exp_operator), &(literal!(" ")))?); __mm_s.push_str(&*operandString(__exp_exp2.clone(), &exp, false)?); ArcStr::from(__mm_s) }),
            Deref @ IF { condition: __exp_condition, falseBranch: __exp_falseBranch, trueBranch: __exp_trueBranch, .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("if ")); __mm_s.push_str(&*toString(__exp_condition.clone())?); __mm_s.push_str(&*literal!(" then ")); __mm_s.push_str(&*toString(__exp_trueBranch.clone())?); __mm_s.push_str(&*literal!(" else ")); __mm_s.push_str(&*toString(__exp_falseBranch.clone())?); ArcStr::from(__mm_s) }),
            Deref @ CAST { exp: __exp_exp, ty: __exp_ty } => if (Flags::isSet(Flags::NF_API.clone())?) {{ exp = __exp_exp.clone(); continue '__tco; }} else {return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CAST(")); __mm_s.push_str(&*Type::toString(metamodelica::AsArg::as_arg(&__exp_ty))?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*toString(__exp_exp.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })},
            Deref @ BOX { exp: __exp_exp } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BOX(")); __mm_s.push_str(&*toString(__exp_exp.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }),
            Deref @ UNBOX { exp: __exp_exp, .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("UNBOX(")); __mm_s.push_str(&*toString(__exp_exp.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }),
            Deref @ SUBSCRIPTED_EXP { exp: __exp_exp, subscripts: __exp_subscripts, .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*toString(__exp_exp.clone())?); __mm_s.push_str(&*literal!(")")); __mm_s.push_str(&*Subscript::toStringList(__exp_subscripts.clone())?); ArcStr::from(__mm_s) }),
            Deref @ TUPLE_ELEMENT { index: __exp_index, tupleExp: __exp_tupleExp, .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*toString(__exp_tupleExp.clone())?); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*intString(__exp_index.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }),
            Deref @ RECORD_ELEMENT { fieldName: __exp_fieldName, recordExp: __exp_recordExp, .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*toString(__exp_recordExp.clone())?); __mm_s.push_str(&*literal!(").")); __mm_s.push_str(&*__exp_fieldName); ArcStr::from(__mm_s) }),
            Deref @ MUTABLE { exp: __exp_exp } => { exp = Mutable::access(__exp_exp.clone()); continue '__tco; },
            Deref @ SHARED_LITERAL { exp: __exp_exp, index: __exp_index } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("LITERAL(")); __mm_s.push_str(&*intString(__exp_index.clone())); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*toString(__exp_exp.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }),
            Deref @ EMPTY { .. } => return Ok(literal!("#EMPTY#")),
            Deref @ PARTIAL_FUNCTION_APPLICATION { argNames: __exp_argNames, args: __exp_args, r#fn: __exp_fn, .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function ")); __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&__exp_fn))?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*stringDelimitList(({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            let __thr_src0 = __exp_args.clone();
            let mut __thr_it0 = (&__thr_src0).into_iter();
            let __thr_src1 = __exp_argNames.clone();
            let mut __thr_it1 = (&__thr_src1).into_iter();
            loop {
                match (__thr_it0.next(), __thr_it1.next()) {
                    (Some(a), Some(n)) => {
                        let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*n); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*toString(a.clone())?); ArcStr::from(__mm_s) };
                        __acc = cons(__x, __acc);
                    }
                    (None, None) => break,
                    _ => return Err("threaded for: ranges of unequal length"),
                }
            }
            __acc.reverse()
        }), literal!(", "))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }),
            Deref @ FILENAME { filename: __exp_filename } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*System::escapedString(__exp_filename.clone(), false)); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }),
            Deref @ INSTANCE_NAME { .. } => return Ok(literal!("getInstanceName()")),
            _ => return Ok(anyString(exp)),
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn toFlatString(
    mut exp: metamodelica::Ref<NFExpression>,
    mut format: BaseModelica::OutputFormat,
) -> Result<ArcStr> {
    '__tco: loop {
        let mut t: metamodelica::Ref<Type::NFType>;
        ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ INTEGER { value: __exp_value } => return Ok(intString(__exp_value.clone())),
            Deref @ REAL { value: __exp_value } => return Ok(realString(__exp_value.clone())),
            Deref @ STRING { value: __exp_value } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*Util::escapeModelicaStringToCString(__exp_value.clone())); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }),
            Deref @ BOOLEAN { value: __exp_value } => return Ok(boolString(__exp_value.clone())),
            Deref @ ENUM_LITERAL { ty: __esc_t @ Deref @ Type::ENUMERATION { .. }, name: __exp_name, .. } => {
                t = (*__esc_t).clone();
                if (Type::isBuiltinEnumeration(metamodelica::AsArg::as_arg(&t))) {return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(var_field!((*t).typePath, Type::NFType::ENUMERATION).clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*__exp_name); ArcStr::from(__mm_s) })} else {return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*Util::makeQuotedIdentifier(AbsynUtil::pathString(var_field!((*t).typePath, Type::NFType::ENUMERATION).clone(), literal!("."), true, false)?)?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*Util::makeQuotedIdentifier(__exp_name.clone())?); ArcStr::from(__mm_s) })}
            },
            Deref @ CLKCONST { clk: __exp_clk } => return Ok(ClockKind::toFlatString(metamodelica::AsArg::as_arg(&__exp_clk), format)?),
            Deref @ CREF { cref: __exp_cref, .. } => return Ok(ComponentRef::toFlatString(metamodelica::AsArg::as_arg(&__exp_cref), format)?),
            Deref @ TYPENAME { ty: __exp_ty } => return Ok(Type::typenameString(&(Type::arrayElementType(metamodelica::AsArg::as_arg(&__exp_ty))))?),
            Deref @ ARRAY { ty: __exp_ty, .. } => if (var_field!((*exp).elements, NFExpression::ARRAY).clone().borrow().is_empty()) {return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("fill(")); __mm_s.push_str(&*toFlatString(makeDefaultValue(&(Type::elementType(__exp_ty.clone())), None, None)?, format)?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Type::dimensionsToFlatString(__exp_ty.clone(), format)?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })} else {return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut e in (var_field!((*exp).elements, NFExpression::ARRAY).clone()).borrow().iter() {
                let __x = toFlatString(e.clone(), format)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }), literal!(", "))); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) })},
            Deref @ MATRIX { elements: __exp_elements } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*stringDelimitList(({
            let mut __acc: metamodelica::List<_> = metamodelica::nil();
            for mut el in (__exp_elements.clone()).into_iter().cloned() {
                let __x = stringDelimitList(({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut e in (el.clone()).into_iter().cloned() {
                let __x = toFlatString(e.clone(), format)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }), literal!(", "));
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }), literal!("; "))); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }),
            Deref @ RANGE { start: __exp_start, step: __exp_step, stop: __exp_stop, .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*operandFlatString(__exp_start.clone(), &exp, false, format)?); __mm_s.push_str(&*if ((__exp_step).is_some()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*operandFlatString(Util::getOption(__exp_step.clone())?, &exp, false, format)?); ArcStr::from(__mm_s) }} else {literal!("")}); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*operandFlatString(__exp_stop.clone(), &exp, false, format)?); ArcStr::from(__mm_s) }),
            Deref @ TUPLE { elements: __exp_elements, .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*stringDelimitList(({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut e in (__exp_elements.clone()).into_iter().cloned() {
                let __x = toFlatString(e.clone(), format)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }), literal!(", "))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }),
            Deref @ RECORD { elements: __exp_elements, ty: __exp_ty, .. } => return Ok(List::toStringCustom(__exp_elements.clone(), &({ let __pe_b1 = format; move |__pe_a0| toFlatString(__pe_a0, __pe_b1.clone()) }), Type::toFlatString(metamodelica::AsArg::as_arg(&__exp_ty), format)?, literal!("("), literal!(", "), literal!(")"), true, 0)?),
            Deref @ CALL { call: __exp_call } => return Ok(Call::toFlatString(metamodelica::AsArg::as_arg(&__exp_call), format)?),
            Deref @ SIZE { dimIndex: __exp_dimIndex, exp: __exp_exp } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("size(")); __mm_s.push_str(&*toFlatString(__exp_exp.clone(), format)?); __mm_s.push_str(&*if ((__exp_dimIndex).is_some()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*toFlatString(Util::getOption(__exp_dimIndex.clone())?, format)?); ArcStr::from(__mm_s) }} else {literal!("")}); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }),
            Deref @ END { .. } => return Ok(literal!("end")),
            Deref @ MULTARY { arguments: __exp_arguments, inv_arguments: __exp_inv_arguments, operator: __exp_operator } if ((__exp_inv_arguments).is_empty()) => return Ok(multaryFlatString(__exp_arguments.clone(), &exp, metamodelica::AsArg::as_arg(&__exp_operator), format, false)?),
            Deref @ MULTARY { arguments: __exp_arguments, inv_arguments: __exp_inv_arguments, operator: __exp_operator } if ((__exp_arguments).is_empty() && Operator::isDashClassification(Operator::getMathClassification(metamodelica::AsArg::as_arg(&__exp_operator))?)) => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-")); __mm_s.push_str(&*multaryFlatString(__exp_inv_arguments.clone(), &exp, metamodelica::AsArg::as_arg(&__exp_operator), format, true)?); ArcStr::from(__mm_s) }),
            Deref @ MULTARY { arguments: __exp_arguments, inv_arguments: __exp_inv_arguments, operator: __exp_operator } if ((__exp_arguments).is_empty()) => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("1/")); __mm_s.push_str(&*multaryFlatString(__exp_inv_arguments.clone(), &exp, metamodelica::AsArg::as_arg(&__exp_operator), format, true)?); ArcStr::from(__mm_s) }),
            Deref @ MULTARY { arguments: __exp_arguments, inv_arguments: __exp_inv_arguments, operator: __exp_operator } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*multaryFlatString(__exp_arguments.clone(), &exp, metamodelica::AsArg::as_arg(&__exp_operator), format, true)?); __mm_s.push_str(&*Operator::symbol(&(Operator::invert(__exp_operator.clone())?), &(literal!(" ")))?); __mm_s.push_str(&*multaryFlatString(__exp_inv_arguments.clone(), &exp, metamodelica::AsArg::as_arg(&__exp_operator), format, true)?); ArcStr::from(__mm_s) }),
            Deref @ BINARY { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*operandFlatString(__exp_exp1.clone(), &exp, true, format)?); __mm_s.push_str(&*Operator::symbol(metamodelica::AsArg::as_arg(&__exp_operator), &(literal!(" ")))?); __mm_s.push_str(&*operandFlatString(__exp_exp2.clone(), &exp, false, format)?); ArcStr::from(__mm_s) }),
            Deref @ UNARY { exp: __exp_exp, operator: __exp_operator } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*Operator::symbol(metamodelica::AsArg::as_arg(&__exp_operator), &(literal!("")))?); __mm_s.push_str(&*operandFlatString(__exp_exp.clone(), &exp, false, format)?); ArcStr::from(__mm_s) }),
            Deref @ LBINARY { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*operandFlatString(__exp_exp1.clone(), &exp, true, format)?); __mm_s.push_str(&*Operator::symbol(metamodelica::AsArg::as_arg(&__exp_operator), &(literal!(" ")))?); __mm_s.push_str(&*operandFlatString(__exp_exp2.clone(), &exp, false, format)?); ArcStr::from(__mm_s) }),
            Deref @ LUNARY { exp: __exp_exp, operator: __exp_operator } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*Operator::symbol(metamodelica::AsArg::as_arg(&__exp_operator), &(literal!("")))?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*operandFlatString(__exp_exp.clone(), &exp, false, format)?); ArcStr::from(__mm_s) }),
            Deref @ RELATION { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator, .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*operandFlatString(__exp_exp1.clone(), &exp, true, format)?); __mm_s.push_str(&*Operator::symbol(metamodelica::AsArg::as_arg(&__exp_operator), &(literal!(" ")))?); __mm_s.push_str(&*operandFlatString(__exp_exp2.clone(), &exp, false, format)?); ArcStr::from(__mm_s) }),
            Deref @ IF { condition: __exp_condition, falseBranch: __exp_falseBranch, trueBranch: __exp_trueBranch, .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("if ")); __mm_s.push_str(&*toFlatString(__exp_condition.clone(), format)?); __mm_s.push_str(&*literal!(" then ")); __mm_s.push_str(&*toFlatString(__exp_trueBranch.clone(), format)?); __mm_s.push_str(&*literal!(" else ")); __mm_s.push_str(&*toFlatString(__exp_falseBranch.clone(), format)?); ArcStr::from(__mm_s) }),
            Deref @ CAST { exp: __exp_exp, .. } => { (exp, format) = (__exp_exp.clone(), format); continue '__tco; },
            Deref @ UNBOX { exp: __exp_exp, .. } => { (exp, format) = (__exp_exp.clone(), format); continue '__tco; },
            Deref @ BOX { exp: __exp_exp } => { (exp, format) = (__exp_exp.clone(), format); continue '__tco; },
            Deref @ SUBSCRIPTED_EXP { exp: __exp_exp, subscripts: __exp_subscripts, .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*toFlatString(__exp_exp.clone(), format)?); __mm_s.push_str(&*literal!(")")); __mm_s.push_str(&*Subscript::toFlatStringList(__exp_subscripts.clone(), format, false)?); ArcStr::from(__mm_s) }),
            Deref @ TUPLE_ELEMENT { tupleExp: __exp_tupleExp, .. } => { (exp, format) = (__exp_tupleExp.clone(), format); continue '__tco; },
            Deref @ RECORD_ELEMENT { fieldName: __exp_fieldName, recordExp: __exp_recordExp, .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*toFlatString(__exp_recordExp.clone(), format)?); __mm_s.push_str(&*literal!(").")); __mm_s.push_str(&*__exp_fieldName); ArcStr::from(__mm_s) }),
            Deref @ MUTABLE { exp: __exp_exp } => { (exp, format) = (Mutable::access(__exp_exp.clone()), format); continue '__tco; },
            Deref @ SHARED_LITERAL { exp: __exp_exp, index: __exp_index } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[literal: ")); __mm_s.push_str(&*intString(__exp_index.clone())); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*toString(__exp_exp.clone())?); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }),
            Deref @ EMPTY { .. } => return Ok(literal!("#EMPTY#")),
            Deref @ PARTIAL_FUNCTION_APPLICATION { argNames: __exp_argNames, args: __exp_args, r#fn: __exp_fn, .. } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function ")); __mm_s.push_str(&*ComponentRef::toFlatString(metamodelica::AsArg::as_arg(&__exp_fn), format)?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*stringDelimitList(({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            let __thr_src0 = __exp_args.clone();
            let mut __thr_it0 = (&__thr_src0).into_iter();
            let __thr_src1 = __exp_argNames.clone();
            let mut __thr_it1 = (&__thr_src1).into_iter();
            loop {
                match (__thr_it0.next(), __thr_it1.next()) {
                    (Some(a), Some(n)) => {
                        let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*n); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*toFlatString(a.clone(), format)?); ArcStr::from(__mm_s) };
                        __acc = cons(__x, __acc);
                    }
                    (None, None) => break,
                    _ => return Err("threaded for: ranges of unequal length"),
                }
            }
            __acc.reverse()
        }), literal!(", "))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }),
            Deref @ FILENAME { filename: __exp_filename } => return Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*Util::escapeModelicaStringToCString(__exp_filename.clone())); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }),
            Deref @ INSTANCE_NAME { .. } => return Ok(literal!("getInstanceName()")),
            _ => return Ok(anyString(exp)),
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn operandString(
    mut operand: metamodelica::Ref<NFExpression>,
    mut operator: &metamodelica::Ref<NFExpression>,
    mut lhs: bool,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut operand_prio: i32;
    let mut operator_prio: i32;
    let mut parenthesize: bool = false;
    r#str = toString(operand.clone())?;
    operand_prio = priority(&operand, lhs)?;
    if operand_prio == 4 {
        parenthesize = true;
    } else {
        operator_prio = priority(operator, lhs)?;
        if operand_prio > operator_prio {
            parenthesize = true;
        } else if operand_prio == operator_prio {
            parenthesize = if (lhs) {
                isNonAssociativeExp(&operand)
            } else {
                !(isAssociativeExp(&operand))
            };
        }
    }
    if parenthesize {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        };
    }
    Ok(r#str)
}

pub(crate) fn operandFlatString(
    mut operand: metamodelica::Ref<NFExpression>,
    mut operator: &metamodelica::Ref<NFExpression>,
    mut lhs: bool,
    mut format: BaseModelica::OutputFormat,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut operand_prio: i32;
    let mut operator_prio: i32;
    let mut parenthesize: bool = false;
    r#str = toFlatString(operand.clone(), format)?;
    operand_prio = priority(&operand, lhs)?;
    if operand_prio == 4 {
        parenthesize = true;
    } else {
        operator_prio = priority(operator, lhs)?;
        if operand_prio > operator_prio {
            parenthesize = true;
        } else if operand_prio == operator_prio {
            parenthesize = if (lhs) {
                isNonAssociativeExp(&operand)
            } else {
                !(isAssociativeExp(&operand))
            };
        }
    }
    if parenthesize {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        };
    }
    Ok(r#str)
}

pub(crate) fn multaryString(
    mut arguments: metamodelica::List<metamodelica::Ref<NFExpression>>,
    mut exp: &metamodelica::Ref<NFExpression>,
    mut operator: &metamodelica::Ref<Operator::NFOperator>,
    mut parenthesize: bool,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(
        ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut e in (arguments.clone()).into_iter().cloned() {
                let __x = operandString(e.clone(), exp, false)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        Operator::symbol(operator, &(literal!(" ")))?,
    );
    if parenthesize && ((arguments).len() as i32) > 1 {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        };
    }
    Ok(r#str)
}

pub(crate) fn multaryFlatString(
    mut arguments: metamodelica::List<metamodelica::Ref<NFExpression>>,
    mut exp: &metamodelica::Ref<NFExpression>,
    mut operator: &metamodelica::Ref<Operator::NFOperator>,
    mut format: BaseModelica::OutputFormat,
    mut parenthesize: bool,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(
        ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut e in (arguments.clone()).into_iter().cloned() {
                let __x = operandFlatString(e.clone(), exp, false, format)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        Operator::symbol(operator, &(literal!(" ")))?,
    );
    if parenthesize && ((arguments).len() as i32) > 1 {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        };
    }
    Ok(r#str)
}

pub(crate) fn priority<'__b>(mut exp: &'__b metamodelica::Ref<NFExpression>, mut lhs: bool) -> Result<i32> {
    '__tco: loop {
        match &**exp {
            INTEGER { .. } => {
                if (var_field!((**exp).value, NFExpression::INTEGER).clone() < 0) {
                    return Ok(4);
                } else {
                    return Ok(0);
                }
            }
            REAL { .. } => {
                if (var_field!((**exp).value, NFExpression::REAL).clone() < metamodelica::OrderedFloat(0.0_f64)) {
                    return Ok(4);
                } else {
                    return Ok(0);
                }
            }
            MULTARY { .. } => {
                return Ok(Operator::priority(
                    var_field!((**exp).operator, NFExpression::MULTARY),
                    lhs,
                ));
            }
            BINARY { .. } => {
                return Ok(Operator::priority(
                    var_field!((**exp).operator, NFExpression::BINARY),
                    lhs,
                ));
            }
            UNARY { .. } => return Ok(4),
            LBINARY { .. } => {
                return Ok(Operator::priority(
                    var_field!((**exp).operator, NFExpression::LBINARY),
                    lhs,
                ));
            }
            LUNARY { .. } => return Ok(7),
            RELATION { .. } => return Ok(6),
            RANGE { .. } => return Ok(10),
            IF { .. } => return Ok(11),
            CAST { .. } => {
                (exp, lhs) = (var_field!((**exp).exp, NFExpression::CAST), lhs);
                continue '__tco;
            }
            BOX { .. } => {
                (exp, lhs) = (var_field!((**exp).exp, NFExpression::BOX), lhs);
                continue '__tco;
            }
            UNBOX { .. } => {
                (exp, lhs) = (var_field!((**exp).exp, NFExpression::UNBOX), lhs);
                continue '__tco;
            }
            _ => return Ok(0),
        }
    }
}

pub(crate) fn isAssociativeExp(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut isAssociative: bool;
    isAssociative = (match &**exp {
        BINARY {
            operator: __exp_operator,
            ..
        } => Operator::isAssociative(metamodelica::AsArg::as_arg(&__exp_operator)),
        LBINARY { .. } => true,
        _ => false,
    });
    isAssociative
}

pub(crate) fn isNonAssociativeExp(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut isAssociative: bool;
    isAssociative = (match &**exp {
        BINARY {
            operator: __exp_operator,
            ..
        } => Operator::isNonAssociative(metamodelica::AsArg::as_arg(&__exp_operator)),
        LBINARY { .. } => true,
        _ => false,
    });
    isAssociative
}

pub(crate) fn getName(mut exp: metamodelica::Ref<NFExpression>) -> Result<ArcStr> {
    '__tco: loop {
        match &*exp {
            RECORD { path: __exp_path, .. } => {
                return Ok(AbsynUtil::pathString(__exp_path.clone(), literal!("."), true, false)?);
            }
            CALL { call: __exp_call } => {
                return Ok(AbsynUtil::pathString(
                    Call::functionName(metamodelica::AsArg::as_arg(&__exp_call))?,
                    literal!("."),
                    true,
                    false,
                )?);
            }
            CAST { exp: __exp_exp, .. } => {
                exp = __exp_exp.clone();
                continue '__tco;
            }
            BOX { exp: __exp_exp } => {
                exp = __exp_exp.clone();
                continue '__tco;
            }
            UNBOX { exp: __exp_exp, .. } => {
                exp = __exp_exp.clone();
                continue '__tco;
            }
            MUTABLE { exp: __exp_exp } => {
                exp = Mutable::access(__exp_exp.clone());
                continue '__tco;
            }
            SHARED_LITERAL { exp: __exp_exp, .. } => {
                exp = __exp_exp.clone();
                continue '__tco;
            }
            PARTIAL_FUNCTION_APPLICATION { r#fn: __exp_fn, .. } => {
                return Ok(ComponentRef::toString(metamodelica::AsArg::as_arg(&__exp_fn))?);
            }
            INSTANCE_NAME { .. } => return Ok(literal!("getInstanceName")),
            _ => return Ok(toString(exp)?),
        }
    }
}

pub(crate) fn enumLiteralPath(mut exp: &metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<Path>> {
    let mut path: metamodelica::Ref<Path>;
    let mut name: ArcStr;
    let mut ty_path: metamodelica::Ref<Path>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*exp)) {
        Deref @ ENUM_LITERAL { name: __pa0, ty: Deref @ Type::ENUMERATION { typePath: __pa1, .. }, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    name = metamodelica::Own::own(__pa0);
    ty_path = metamodelica::Own::own(__pa1);
    path = AbsynUtil::suffixPath(&ty_path, &name);
    Ok(path)
}

pub fn getNominal(mut exp: metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression> = exp;
    exp = map(
        exp,
        (std::sync::Arc::new(computeNominal)
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>>
                    + 'static,
            >),
    )?;
    exp = SimplifyExp::simplify(exp, false)?;
    Ok(exp)
}

pub(crate) fn computeNominal(mut exp: metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { cref: Deref @ ComponentRef::CREF { .. }, .. } if (InstNode::isVar(&(ComponentRef::node(var_field!((*exp).cref, NFExpression::CREF))?))) => {
            let mut varPointer: PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>;
            let mut nominal: Option<metamodelica::Ref<NFExpression>>;
            let __pa0 = ::match_deref::match_deref! { match &(ComponentRef::node(var_field!((*exp).cref, NFExpression::CREF))?) {
                Deref @ InstNode::VAR_NODE { varPointer: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            varPointer = metamodelica::Own::own(__pa0);
            nominal = Variable::getNominal(&(Pointer::access(PointerWeak::upgrade(varPointer)?)))?;
            Util::getOptionOrDefault(nominal, exp.clone())
        },
        Deref @ INTEGER { value: __exp_value } => {
            metamodelica::Ref::new(NFExpression::INTEGER { value: (__exp_value.clone()).abs() })
        },
        Deref @ REAL { value: __exp_value } => {
            metamodelica::Ref::new(NFExpression::REAL { value: (__exp_value.clone()).abs() })
        },
        Deref @ UNARY { exp: __exp_exp, .. } => {
            __exp_exp.clone()
        },
        Deref @ BINARY { operator, .. } if (Operator::getMathClassification(metamodelica::AsArg::as_arg(&operator))? == Operator::MathClassification::SUBTRACTION.clone()) => {
            let mut sizeClass: Operator::SizeClassification;
            (_, sizeClass) = Operator::classify(metamodelica::AsArg::as_arg(&operator))?;
            assign_variant_field!(exp => NFExpression::BINARY; operator = Operator::fromClassification((Operator::MathClassification::ADDITION.clone(), sizeClass), operator.ty.clone())?);
            exp.clone()
        },
        Deref @ MULTARY { operator, arguments: __exp_arguments, inv_arguments: __exp_inv_arguments } if (Operator::getMathClassification(metamodelica::AsArg::as_arg(&operator))? == Operator::MathClassification::ADDITION.clone()) => {
            assign_variant_field!(exp => NFExpression::MULTARY;
                arguments = listAppend(__exp_arguments.clone(), __exp_inv_arguments.clone()),
                inv_arguments = metamodelica::nil()
            );
            exp.clone()
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn toAbsyn(mut exp: metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<Absyn::Exp>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ INTEGER { value: __exp_value } => {
                return Ok(metamodelica::Ref::new(Absyn::Exp::INTEGER { value: __exp_value.clone() }))
            },
            Deref @ REAL { value: __exp_value } => {
                return Ok(metamodelica::Ref::new(Absyn::Exp::REAL { value: ArcStr::from(::std::format!("{}", __exp_value.clone())) }))
            },
            Deref @ STRING { value: __exp_value } => {
                return Ok(metamodelica::Ref::new(Absyn::Exp::STRING { value: __exp_value.clone() }))
            },
            Deref @ BOOLEAN { value: __exp_value } => {
                return Ok(metamodelica::Ref::new(Absyn::Exp::BOOL { value: __exp_value.clone() }))
            },
            Deref @ ENUM_LITERAL { ty: Deref @ Type::ENUMERATION { .. }, .. } => {
                return Ok(metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: AbsynUtil::pathToCref(&(enumLiteralPath(&exp)?)) }))
            },
            Deref @ CLKCONST { clk: __exp_clk } => {
                return Ok(ClockKind::toAbsyn(metamodelica::AsArg::as_arg(&__exp_clk))?)
            },
            Deref @ CREF { cref: __exp_cref, .. } => {
                return Ok(metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: ComponentRef::toAbsyn(metamodelica::AsArg::as_arg(&__exp_cref))? }))
            },
            Deref @ TYPENAME { ty: __exp_ty } => {
                return Ok(metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: Type::toString(metamodelica::AsArg::as_arg(&__exp_ty))?, subscripts: metamodelica::nil() }) }))
            },
            Deref @ ARRAY { .. } => {
                return Ok(metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
            for mut e in (var_field!((*exp).elements, NFExpression::ARRAY).clone()).borrow().iter() {
                let __x = toAbsyn(e.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }) }))
            },
            Deref @ MATRIX { elements: __exp_elements } => {
                return Ok(metamodelica::Ref::new(Absyn::Exp::MATRIX { matrix: ({
            let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>> = metamodelica::nil();
            for mut l in (__exp_elements.clone()).into_iter().cloned() {
                let __x = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
            for mut e in (l.clone()).into_iter().cloned() {
                let __x = toAbsyn(e.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }) }))
            },
            Deref @ RANGE { start: __exp_start, step: __exp_step, stop: __exp_stop, .. } => {
                return Ok(metamodelica::Ref::new(Absyn::Exp::RANGE { start: toAbsyn(__exp_start.clone())?, step: Util::applyOption(__exp_step.clone(), &toAbsyn)?, stop: toAbsyn(__exp_stop.clone())? }))
            },
            Deref @ TUPLE { elements: __exp_elements, .. } => {
                return Ok(metamodelica::Ref::new(Absyn::Exp::TUPLE { expressions: ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
            for mut e in (__exp_elements.clone()).into_iter().cloned() {
                let __x = toAbsyn(e.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }) }))
            },
            Deref @ RECORD { elements: __exp_elements, path: __exp_path, .. } => {
                return Ok(AbsynUtil::makeCall(AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&__exp_path)), ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
            for mut e in (__exp_elements.clone()).into_iter().cloned() {
                let __x = toAbsyn(e.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }), metamodelica::nil()))
            },
            Deref @ CALL { call: __exp_call } => {
                return Ok(Call::toAbsyn(metamodelica::AsArg::as_arg(&__exp_call))?)
            },
            Deref @ SIZE { dimIndex: __exp_dimIndex, .. } => {
                return Ok(AbsynUtil::makeCall(metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("size"), subscripts: metamodelica::nil() }), if ((__exp_dimIndex).is_some()) {list![toAbsyn(Util::getOption(__exp_dimIndex.clone())?)?]} else {metamodelica::nil()}, metamodelica::nil()))
            },
            Deref @ END { .. } => {
                return Ok(openmodelica_ast::Absyn::Exp::interned_END())
            },
            Deref @ BINARY { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator } => {
                return Ok(metamodelica::Ref::new(Absyn::Exp::BINARY { exp1: toAbsyn(__exp_exp1.clone())?, op: Operator::toAbsyn(metamodelica::AsArg::as_arg(&__exp_operator))?, exp2: toAbsyn(__exp_exp2.clone())? }))
            },
            Deref @ UNARY { exp: __exp_exp, operator: __exp_operator } => {
                return Ok(metamodelica::Ref::new(Absyn::Exp::UNARY { op: Operator::toAbsyn(metamodelica::AsArg::as_arg(&__exp_operator))?, exp: toAbsyn(__exp_exp.clone())? }))
            },
            Deref @ LBINARY { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator } => {
                return Ok(metamodelica::Ref::new(Absyn::Exp::LBINARY { exp1: toAbsyn(__exp_exp1.clone())?, op: Operator::toAbsyn(metamodelica::AsArg::as_arg(&__exp_operator))?, exp2: toAbsyn(__exp_exp2.clone())? }))
            },
            Deref @ LUNARY { exp: __exp_exp, operator: __exp_operator } => {
                return Ok(metamodelica::Ref::new(Absyn::Exp::LUNARY { op: Operator::toAbsyn(metamodelica::AsArg::as_arg(&__exp_operator))?, exp: toAbsyn(__exp_exp.clone())? }))
            },
            Deref @ RELATION { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator, .. } => {
                return Ok(metamodelica::Ref::new(Absyn::Exp::RELATION { exp1: toAbsyn(__exp_exp1.clone())?, op: Operator::toAbsyn(metamodelica::AsArg::as_arg(&__exp_operator))?, exp2: toAbsyn(__exp_exp2.clone())? }))
            },
            Deref @ IF { condition: __exp_condition, falseBranch: __exp_falseBranch, trueBranch: __exp_trueBranch, .. } => {
                return Ok(metamodelica::Ref::new(Absyn::Exp::IFEXP { ifExp: toAbsyn(__exp_condition.clone())?, trueBranch: toAbsyn(__exp_trueBranch.clone())?, elseBranch: toAbsyn(__exp_falseBranch.clone())?, elseIfBranch: metamodelica::nil() }))
            },
            Deref @ CAST { exp: __exp_exp, .. } => {
                { exp = __exp_exp.clone(); continue '__tco; }
            },
            Deref @ BOX { exp: __exp_exp } => {
                { exp = __exp_exp.clone(); continue '__tco; }
            },
            Deref @ UNBOX { exp: __exp_exp, .. } => {
                { exp = __exp_exp.clone(); continue '__tco; }
            },
            Deref @ MUTABLE { exp: __exp_exp } => {
                { exp = Mutable::access(__exp_exp.clone()); continue '__tco; }
            },
            Deref @ SHARED_LITERAL { exp: __exp_exp, .. } => {
                { exp = __exp_exp.clone(); continue '__tco; }
            },
            Deref @ PARTIAL_FUNCTION_APPLICATION { args: __exp_args, r#fn: __exp_fn, .. } => {
                return Ok(metamodelica::Ref::new(Absyn::Exp::PARTEVALFUNCTION { function_: ComponentRef::toAbsyn(metamodelica::AsArg::as_arg(&__exp_fn))?, functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
            for mut e in (__exp_args.clone()).into_iter().cloned() {
                let __x = toAbsyn(e.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }), argNames: metamodelica::nil() }) }))
            },
            Deref @ FILENAME { filename: __exp_filename } => {
                return Ok(metamodelica::Ref::new(Absyn::Exp::STRING { value: __exp_filename.clone() }))
            },
            Deref @ INSTANCE_NAME { .. } => {
                return Ok(AbsynUtil::makeCall(metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("getInstanceName"), subscripts: metamodelica::nil() }), metamodelica::nil(), metamodelica::nil()))
            },
            _ => {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.toAbsyn")); __mm_s.push_str(&*literal!(" got unknown expression '")); __mm_s.push_str(&*toString(exp)?); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo")))?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn toDAE(mut exp: metamodelica::Ref<NFExpression>, mut allowEmpty: bool) -> Result<metamodelica::Ref<DAE::Exp>> {
    '__tco: loop {
        match &*exp.clone() {
            INTEGER { value: __exp_value } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::ICONST {
                    integer: __exp_value.clone(),
                }));
            }
            REAL { value: __exp_value } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: __exp_value.clone(),
                }));
            }
            STRING { value: __exp_value } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::SCONST {
                    string: __exp_value.clone(),
                }));
            }
            BOOLEAN { value: __exp_value } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::BCONST {
                    bool: __exp_value.clone(),
                }));
            }
            ENUM_LITERAL { index: __exp_index, .. } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::ENUM_LITERAL {
                    name: enumLiteralPath(&exp)?,
                    index: __exp_index.clone(),
                }));
            }
            CLKCONST { clk: __exp_clk } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::CLKCONST {
                    clk: ClockKind::toDAE(metamodelica::AsArg::as_arg(&__exp_clk))?,
                }));
            }
            CREF {
                cref: __exp_cref,
                ty: __exp_ty,
            } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: ComponentRef::toDAE(metamodelica::AsArg::as_arg(&__exp_cref))?,
                    ty: Type::toDAE(metamodelica::AsArg::as_arg(&__exp_ty), true)?,
                }));
            }
            TYPENAME { ty: __exp_ty } => {
                (exp, allowEmpty) = (ExpandExp::expandTypename(__exp_ty.clone())?, false);
                continue '__tco;
            }
            ARRAY { ty: __exp_ty, .. } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::ARRAY {
                    ty: Type::toDAE(metamodelica::AsArg::as_arg(&__exp_ty), true)?,
                    scalar: Type::isVector(metamodelica::AsArg::as_arg(&__exp_ty))?,
                    array: ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                        for mut e in (var_field!((*exp).elements, NFExpression::ARRAY).clone())
                            .borrow()
                            .iter()
                        {
                            let __x = toDAE(e.clone(), false)?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                }));
            }
            RECORD {
                elements: __exp_elements,
                path: __exp_path,
                ty: __exp_ty,
            } => {
                return Ok(toDAERecord(
                    __exp_ty.clone(),
                    __exp_path.clone(),
                    __exp_elements.clone(),
                )?);
            }
            RANGE {
                start: __exp_start,
                step: __exp_step,
                stop: __exp_stop,
                ty: __exp_ty,
            } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::RANGE {
                    ty: Type::toDAE(metamodelica::AsArg::as_arg(&__exp_ty), true)?,
                    start: toDAE(__exp_start.clone(), false)?,
                    step: if ((__exp_step).is_some()) {
                        Some(toDAE(Util::getOption(__exp_step.clone())?, false)?)
                    } else {
                        None
                    },
                    stop: toDAE(__exp_stop.clone(), false)?,
                }));
            }
            TUPLE {
                elements: __exp_elements,
                ..
            } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::TUPLE {
                    PR: ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                        for mut e in (__exp_elements.clone()).into_iter().cloned() {
                            let __x = toDAE(e.clone(), false)?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                }));
            }
            CALL { call: __exp_call } => return Ok(Call::toDAE(__exp_call.clone())?),
            SIZE {
                dimIndex: __exp_dimIndex,
                exp: __exp_exp,
            } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::SIZE {
                    exp: toDAE(__exp_exp.clone(), false)?,
                    sz: if ((__exp_dimIndex).is_some()) {
                        Some(toDAE(Util::getOption(__exp_dimIndex.clone())?, false)?)
                    } else {
                        None
                    },
                }));
            }
            MULTARY { .. } => {
                (exp, allowEmpty) = (SimplifyExp::splitMultary(exp)?, false);
                continue '__tco;
            }
            BINARY {
                exp1: __exp_exp1,
                exp2: __exp_exp2,
                operator: __exp_operator,
            } => {
                let mut daeOp: DAE::Operator;
                let mut swap: bool;
                let mut negate: bool;
                let mut dae1: metamodelica::Ref<DAE::Exp>;
                let mut dae2: metamodelica::Ref<DAE::Exp>;
                (daeOp, swap, negate) = Operator::toDAE(metamodelica::AsArg::as_arg(&__exp_operator))?;
                dae1 = toDAE(__exp_exp1.clone(), false)?;
                dae2 = toDAE(
                    if (negate) {
                        self::negate(__exp_exp2.clone())
                    } else {
                        __exp_exp2.clone()
                    },
                    false,
                )?;
                return Ok(metamodelica::Ref::new(DAE::Exp::BINARY {
                    exp1: if (swap) { dae2.clone() } else { dae1.clone() },
                    operator: daeOp,
                    exp2: if (swap) { dae1 } else { dae2 },
                }));
            }
            UNARY {
                exp: __exp_exp,
                operator: __exp_operator,
            } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::UNARY {
                    operator: Operator::toDAE(metamodelica::AsArg::as_arg(&__exp_operator))?.0,
                    exp: toDAE(__exp_exp.clone(), false)?,
                }));
            }
            LBINARY {
                exp1: __exp_exp1,
                exp2: __exp_exp2,
                operator: __exp_operator,
            } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::LBINARY {
                    exp1: toDAE(__exp_exp1.clone(), false)?,
                    operator: Operator::toDAE(metamodelica::AsArg::as_arg(&__exp_operator))?.0,
                    exp2: toDAE(__exp_exp2.clone(), false)?,
                }));
            }
            LUNARY {
                exp: __exp_exp,
                operator: __exp_operator,
            } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::LUNARY {
                    operator: Operator::toDAE(metamodelica::AsArg::as_arg(&__exp_operator))?.0,
                    exp: toDAE(__exp_exp.clone(), false)?,
                }));
            }
            RELATION {
                exp1: __exp_exp1,
                exp2: __exp_exp2,
                index: __exp_index,
                operator: __exp_operator,
            } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::RELATION {
                    exp1: toDAE(__exp_exp1.clone(), false)?,
                    operator: Operator::toDAE(metamodelica::AsArg::as_arg(&__exp_operator))?.0,
                    exp2: toDAE(__exp_exp2.clone(), false)?,
                    index: __exp_index.clone(),
                    optionExpisASUB: None,
                }));
            }
            IF {
                condition: __exp_condition,
                falseBranch: __exp_falseBranch,
                trueBranch: __exp_trueBranch,
                ..
            } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::IFEXP {
                    expCond: toDAE(__exp_condition.clone(), false)?,
                    expThen: toDAE(__exp_trueBranch.clone(), false)?,
                    expElse: toDAE(__exp_falseBranch.clone(), false)?,
                }));
            }
            CAST {
                exp: __exp_exp,
                ty: __exp_ty,
            } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::CAST {
                    ty: Type::toDAE(metamodelica::AsArg::as_arg(&__exp_ty), true)?,
                    exp: toDAE(__exp_exp.clone(), false)?,
                }));
            }
            BOX { exp: __exp_exp } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::BOX {
                    exp: toDAE(__exp_exp.clone(), false)?,
                }));
            }
            UNBOX {
                exp: __exp_exp,
                ty: __exp_ty,
            } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::UNBOX {
                    exp: toDAE(__exp_exp.clone(), false)?,
                    ty: Type::toDAE(metamodelica::AsArg::as_arg(&__exp_ty), true)?,
                }));
            }
            SUBSCRIPTED_EXP {
                exp: __exp_exp,
                subscripts: __exp_subscripts,
                ..
            } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::ASUB {
                    exp: toDAE(__exp_exp.clone(), false)?,
                    sub: ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = metamodelica::nil();
                        for mut s in (__exp_subscripts.clone()).into_iter().cloned() {
                            let __x = Subscript::toDAE(&(s.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                }));
            }
            TUPLE_ELEMENT {
                index: __exp_index,
                tupleExp: __exp_tupleExp,
                ty: __exp_ty,
            } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::TSUB {
                    exp: toDAE(__exp_tupleExp.clone(), false)?,
                    ix: __exp_index.clone(),
                    ty: Type::toDAE(metamodelica::AsArg::as_arg(&__exp_ty), true)?,
                }));
            }
            RECORD_ELEMENT {
                fieldName: __exp_fieldName,
                recordExp: __exp_recordExp,
                ty: __exp_ty,
                ..
            } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::RSUB {
                    exp: toDAE(__exp_recordExp.clone(), false)?,
                    ix: -1,
                    fieldName: __exp_fieldName.clone(),
                    ty: Type::toDAE(metamodelica::AsArg::as_arg(&__exp_ty), true)?,
                }));
            }
            PARTIAL_FUNCTION_APPLICATION {
                args: __exp_args,
                r#fn: __exp_fn,
                ty: __exp_ty,
                ..
            } => {
                let mut r#fn: metamodelica::Ref<Function::Function::Function>;
                let __pa0 = ::match_deref::match_deref! { match &(Function::Function::typeRefCache(metamodelica::AsArg::as_arg(&__exp_fn), InstContext::FUNCTION.clone())?) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                r#fn = metamodelica::Own::own(__pa0);
                return Ok(metamodelica::Ref::new(DAE::Exp::PARTEVALFUNCTION {
                    path: Function::Function::nameConsiderBuiltin(&r#fn),
                    expList: ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                        for mut arg in (__exp_args.clone()).into_iter().cloned() {
                            let __x = toDAE(arg.clone(), false)?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    ty: Type::toDAE(metamodelica::AsArg::as_arg(&__exp_ty), true)?,
                    origType: Type::toDAE(
                        &(metamodelica::Ref::new(Type::NFType::FUNCTION {
                            r#fn: r#fn,
                            fnType: Type::FunctionType::FUNCTIONAL_VARIABLE.clone(),
                        })),
                        true,
                    )?,
                }));
            }
            MUTABLE { exp: __exp_exp } => {
                (exp, allowEmpty) = (Mutable::access(__exp_exp.clone()), false);
                continue '__tco;
            }
            EMPTY { ty: __exp_ty } if (allowEmpty) => {
                let mut dty: metamodelica::Ref<DAE::Type>;
                dty = Type::toDAE(metamodelica::AsArg::as_arg(&__exp_ty), true)?;
                return Ok(metamodelica::Ref::new(DAE::Exp::EMPTY {
                    scope: literal!(""),
                    name: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                        ident: literal!("$dummy"),
                        identType: dty.clone(),
                        subscriptLst: metamodelica::nil(),
                    }),
                    ty: dty,
                    tyStr: Type::toString(metamodelica::AsArg::as_arg(&__exp_ty))?,
                }));
            }
            SHARED_LITERAL {
                exp: __exp_exp,
                index: __exp_index,
            } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::SHARED_LITERAL {
                    index: __exp_index.clone(),
                    exp: toDAE(__exp_exp.clone(), false)?,
                }));
            }
            FILENAME {
                filename: __exp_filename,
            } => {
                if (Flags::getConfigBool(Flags::BUILDING_FMU.clone())?) {
                    return Ok(metamodelica::Ref::new(DAE::Exp::CALL {
                        path: metamodelica::Ref::new(Path::IDENT {
                            name: literal!("OpenModelica_fmuLoadResource"),
                        }),
                        expLst: list![metamodelica::Ref::new(DAE::Exp::SCONST {
                            string: __exp_filename.clone()
                        })],
                        attr: DAE::callAttrBuiltinImpureString().clone(),
                    }));
                } else {
                    return Ok(metamodelica::Ref::new(DAE::Exp::SCONST {
                        string: __exp_filename.clone(),
                    }));
                }
            }
            INSTANCE_NAME { .. } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::CALL {
                    path: metamodelica::Ref::new(Path::IDENT {
                        name: literal!("getInstanceName"),
                    }),
                    expLst: metamodelica::nil(),
                    attr: DAE::callAttrBuiltinString().clone(),
                }));
            }
            _ => {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFExpression.toDAE"));
                        __mm_s.push_str(&*literal!(" got unknown expression '"));
                        __mm_s.push_str(&*toString(exp)?);
                        __mm_s.push_str(&*literal!("'"));
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo")),
                )?;
                return Ok(return Err("fail"));
            }
        }
    }
}

pub(crate) fn toDAERecord(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut path: metamodelica::Ref<Path>,
    mut args: metamodelica::List<metamodelica::Ref<NFExpression>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut field_names: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut arg: metamodelica::Ref<NFExpression>;
    let mut rest_args: metamodelica::List<metamodelica::Ref<NFExpression>> = args;
    let mut dargs: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    for mut field in &*Type::recordFields(&(Type::unbox(ty.clone()))) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_args) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        arg = metamodelica::Own::own(__pa0);
        rest_args = metamodelica::Own::own(__pa1);
        let () = (match &*field.clone() {
            Record::Field::INPUT { name: __field_name } => {
                field_names = metamodelica::cons(__field_name.clone(), field_names);
                dargs = metamodelica::cons(toDAE(arg, true)?, dargs);
                ()
            }
            Record::Field::LOCAL { name: __field_name } => {
                field_names = metamodelica::cons(__field_name.clone(), field_names);
                dargs = metamodelica::cons(toDAE(arg, true)?, dargs);
                ()
            }
            _ => (),
        });
    }
    field_names = metamodelica::Dangerous::listReverseInPlace(field_names);
    dargs = metamodelica::Dangerous::listReverseInPlace(dargs);
    exp = if (Type::isBoxed(&ty)) {
        metamodelica::Ref::new(DAE::Exp::METARECORDCALL {
            path: path,
            args: dargs,
            fieldNames: field_names,
            index: -1,
            typeVars: metamodelica::nil(),
        })
    } else {
        metamodelica::Ref::new(DAE::Exp::RECORD {
            path: path,
            exps: dargs,
            comp: field_names,
            ty: Type::toDAE(&ty, true)?,
        })
    };
    Ok(exp)
}

pub(crate) fn toDAEValue(mut exp: &metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut value: metamodelica::Ref<Values::Value>;
    value = (::match_deref::match_deref! { match exp {
        Deref @ INTEGER { value: __exp_value } => {
            metamodelica::Ref::new(Values::Value::INTEGER { integer: __exp_value.clone() })
        },
        Deref @ REAL { value: __exp_value } => {
            metamodelica::Ref::new(Values::Value::REAL { real: __exp_value.clone() })
        },
        Deref @ STRING { value: __exp_value } => {
            metamodelica::Ref::new(Values::Value::STRING { string: __exp_value.clone() })
        },
        Deref @ BOOLEAN { value: __exp_value } => {
            metamodelica::Ref::new(Values::Value::BOOL { boolean: __exp_value.clone() })
        },
        Deref @ ENUM_LITERAL { ty: ty @ Deref @ Type::ENUMERATION { .. }, index: __exp_index, name: __exp_name } => {
            metamodelica::Ref::new(Values::Value::ENUM_LITERAL { name: AbsynUtil::suffixPath(var_field!((**ty).typePath, Type::NFType::ENUMERATION), metamodelica::AsArg::as_arg(&__exp_name)), index: __exp_index.clone() })
        },
        Deref @ ARRAY { .. } => {
            ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut e in (var_field!((**exp).elements, NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = toDAEValue(&(e.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))
        },
        Deref @ RECORD { elements: __exp_elements, path: __exp_path, ty: __exp_ty } => {
            toDAEValueRecord(metamodelica::AsArg::as_arg(&__exp_ty), __exp_path.clone(), __exp_elements.clone())?
        },
        Deref @ FILENAME { filename: __exp_filename } => {
            metamodelica::Ref::new(Values::Value::STRING { string: __exp_filename.clone() })
        },
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.toDAEValue")); __mm_s.push_str(&*literal!(" got unhandled expression ")); __mm_s.push_str(&*toString(exp.clone())?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(value)
}

pub(crate) fn toDAEValueRecord(
    mut ty: &metamodelica::Ref<Type::NFType>,
    mut path: metamodelica::Ref<Path>,
    mut args: metamodelica::List<metamodelica::Ref<NFExpression>>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut value: metamodelica::Ref<Values::Value>;
    let mut field_names: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut arg: metamodelica::Ref<NFExpression>;
    let mut rest_args: metamodelica::List<metamodelica::Ref<NFExpression>> = args;
    let mut values: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
    for mut field in &*Type::recordFields(ty) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_args) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        arg = metamodelica::Own::own(__pa0);
        rest_args = metamodelica::Own::own(__pa1);
        let () = (match &*field.clone() {
            Record::Field::INPUT { name: __field_name } => {
                field_names = metamodelica::cons(__field_name.clone(), field_names);
                values = metamodelica::cons(toDAEValue(&arg)?, values);
                ()
            }
            _ => (),
        });
    }
    field_names = metamodelica::Dangerous::listReverseInPlace(field_names);
    values = metamodelica::Dangerous::listReverseInPlace(values);
    value = metamodelica::Ref::new(Values::Value::RECORD {
        record_: path,
        orderd: values,
        comp: field_names,
        index: -1,
    });
    Ok(value)
}

pub(crate) fn dimensionCount(mut exp: metamodelica::Ref<NFExpression>, mut isDim: bool) -> Result<i32> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ TYPENAME { .. } => if (isDim) {return Ok(0)} else {return Ok(1)},
            Deref @ ARRAY { ty: Deref @ Type::UNKNOWN, .. } => return Ok(1 + dimensionCount(metamodelica::arrayGet(var_field!((*exp).elements, NFExpression::ARRAY).clone(), 1)?, false)?),
            Deref @ MATRIX { .. } => return Ok(2),
            Deref @ SIZE { dimIndex: __exp_dimIndex, exp: __exp_exp } => if ((__exp_dimIndex).is_none()) {{ (exp, isDim) = (__exp_exp.clone(), false); continue '__tco; }} else {return Ok(0)},
            _ => return Ok(Type::dimensionCount(typeOf(exp))),
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn dimensions(
    mut exp: metamodelica::Ref<NFExpression>,
) -> metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> {
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    dims = Type::arrayDims(typeOf(exp));
    dims
}

pub fn map(
    mut exp: metamodelica::Ref<NFExpression>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static,
    >,
) -> Result<metamodelica::Ref<NFExpression>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static,
    >;

    let mut outExp: metamodelica::Ref<NFExpression>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { clk: __exp_clk } => {
            metamodelica::Ref::new(NFExpression::CLKCONST { clk: ClockKind::mapExp(__exp_clk.clone(), func.clone())? })
        },
        Deref @ CREF { cref: __exp_cref, ty: __exp_ty } => {
            metamodelica::Ref::new(NFExpression::CREF { ty: __exp_ty.clone(), cref: ComponentRef::mapExp(metamodelica::AsArg::as_arg(&__exp_cref), func.clone())? })
        },
        Deref @ ARRAY { literal: __exp_literal, ty: __exp_ty, .. } if (!(__exp_literal.clone())) => {
            makeArray(__exp_ty.clone(), Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static> = func.clone(); move |__pe_a0| map(__pe_a0, __pe_b1.clone()) }))?, __exp_literal.clone())
        },
        Deref @ MATRIX { elements: __exp_elements } => {
            metamodelica::Ref::new(NFExpression::MATRIX { elements: ({
        let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<NFExpression>>> = metamodelica::nil();
        for mut row in (__exp_elements.clone()).into_iter().cloned() {
            let __x = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
        for mut e in (row.clone()).into_iter().cloned() {
            let __x = map(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) })
        },
        Deref @ RANGE { step: Some(e2), start: __exp_start, stop: __exp_stop, ty: __exp_ty } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e3: metamodelica::Ref<NFExpression>;
            let mut e4: metamodelica::Ref<NFExpression>;
            e1 = map(__exp_start.clone(), func.clone())?;
            e4 = map(e2.clone(), func.clone())?;
            e3 = map(__exp_stop.clone(), func.clone())?;
            if (referenceEq(&*(__exp_start.clone()),&*(&*e1)) && referenceEq(&*(e2.clone()),&*(&*e4)) && referenceEq(&*(__exp_stop.clone()),&*(&*e3))) {exp} else {metamodelica::Ref::new(NFExpression::RANGE { ty: __exp_ty.clone(), start: e1, step: Some(e4), stop: e3 })}
        },
        Deref @ RANGE { start: __exp_start, stop: __exp_stop, ty: __exp_ty, .. } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e3: metamodelica::Ref<NFExpression>;
            e1 = map(__exp_start.clone(), func.clone())?;
            e3 = map(__exp_stop.clone(), func.clone())?;
            if (referenceEq(&*(__exp_start.clone()),&*(&*e1)) && referenceEq(&*(__exp_stop.clone()),&*(&*e3))) {exp} else {metamodelica::Ref::new(NFExpression::RANGE { ty: __exp_ty.clone(), start: e1, step: None, stop: e3 })}
        },
        Deref @ TUPLE { elements: __exp_elements, ty: __exp_ty } => {
            metamodelica::Ref::new(NFExpression::TUPLE { ty: __exp_ty.clone(), elements: ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
        for mut e in (__exp_elements.clone()).into_iter().cloned() {
            let __x = map(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) })
        },
        Deref @ RECORD { elements: __exp_elements, path: __exp_path, ty: __exp_ty } => {
            metamodelica::Ref::new(NFExpression::RECORD { path: __exp_path.clone(), ty: __exp_ty.clone(), elements: ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
        for mut e in (__exp_elements.clone()).into_iter().cloned() {
            let __x = map(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) })
        },
        Deref @ CALL { call: __exp_call } => {
            metamodelica::Ref::new(NFExpression::CALL { call: Call::mapExp(metamodelica::AsArg::as_arg(&__exp_call), func.clone())? })
        },
        Deref @ SIZE { dimIndex: Some(e2), exp: __exp_exp } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e3: metamodelica::Ref<NFExpression>;
            e1 = map(__exp_exp.clone(), func.clone())?;
            e3 = map(e2.clone(), func.clone())?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1)) && referenceEq(&*(e2.clone()),&*(&*e3))) {exp} else {metamodelica::Ref::new(NFExpression::SIZE { exp: e1, dimIndex: Some(e3) })}
        },
        Deref @ SIZE { exp: __exp_exp, .. } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = map(__exp_exp.clone(), func.clone())?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::SIZE { exp: e1, dimIndex: None })}
        },
        Deref @ BINARY { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            e1 = map(__exp_exp1.clone(), func.clone())?;
            e2 = map(__exp_exp2.clone(), func.clone())?;
            if (referenceEq(&*(__exp_exp1.clone()),&*(&*e1)) && referenceEq(&*(__exp_exp2.clone()),&*(&*e2))) {exp} else {metamodelica::Ref::new(NFExpression::BINARY { exp1: e1, operator: __exp_operator.clone(), exp2: e2 })}
        },
        Deref @ MULTARY { arguments: __exp_arguments, .. } => {
            assign_variant_field!(exp => NFExpression::MULTARY;
                arguments = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
        for mut arg in (__exp_arguments.clone()).into_iter().cloned() {
            let __x = map(arg.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
                inv_arguments = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
        for mut arg in (var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = map(arg.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
            );
            exp
        },
        Deref @ UNARY { exp: __exp_exp, operator: __exp_operator } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = map(__exp_exp.clone(), func.clone())?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::UNARY { operator: __exp_operator.clone(), exp: e1 })}
        },
        Deref @ LBINARY { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            e1 = map(__exp_exp1.clone(), func.clone())?;
            e2 = map(__exp_exp2.clone(), func.clone())?;
            if (referenceEq(&*(__exp_exp1.clone()),&*(&*e1)) && referenceEq(&*(__exp_exp2.clone()),&*(&*e2))) {exp} else {metamodelica::Ref::new(NFExpression::LBINARY { exp1: e1, operator: __exp_operator.clone(), exp2: e2 })}
        },
        Deref @ LUNARY { exp: __exp_exp, operator: __exp_operator } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = map(__exp_exp.clone(), func.clone())?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::LUNARY { operator: __exp_operator.clone(), exp: e1 })}
        },
        Deref @ RELATION { exp1: __exp_exp1, exp2: __exp_exp2, index: __exp_index, operator: __exp_operator } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            e1 = map(__exp_exp1.clone(), func.clone())?;
            e2 = map(__exp_exp2.clone(), func.clone())?;
            if (referenceEq(&*(__exp_exp1.clone()),&*(&*e1)) && referenceEq(&*(__exp_exp2.clone()),&*(&*e2))) {exp} else {metamodelica::Ref::new(NFExpression::RELATION { exp1: e1, operator: __exp_operator.clone(), exp2: e2, index: __exp_index.clone() })}
        },
        Deref @ IF { condition: __exp_condition, falseBranch: __exp_falseBranch, trueBranch: __exp_trueBranch, ty: __exp_ty } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            let mut e3: metamodelica::Ref<NFExpression>;
            e1 = map(__exp_condition.clone(), func.clone())?;
            e2 = map(__exp_trueBranch.clone(), func.clone())?;
            e3 = map(__exp_falseBranch.clone(), func.clone())?;
            if (referenceEq(&*(__exp_condition.clone()),&*(&*e1)) && referenceEq(&*(__exp_trueBranch.clone()),&*(&*e2)) && referenceEq(&*(__exp_falseBranch.clone()),&*(&*e3))) {exp} else {metamodelica::Ref::new(NFExpression::IF { ty: __exp_ty.clone(), condition: e1, trueBranch: e2, falseBranch: e3 })}
        },
        Deref @ CAST { exp: __exp_exp, ty: __exp_ty } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = map(__exp_exp.clone(), func.clone())?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::CAST { ty: __exp_ty.clone(), exp: e1 })}
        },
        Deref @ BOX { exp: __exp_exp } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = map(__exp_exp.clone(), func.clone())?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {r#box(&e1)}
        },
        Deref @ UNBOX { exp: __exp_exp, .. } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = map(__exp_exp.clone(), func.clone())?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {unbox(e1)}
        },
        Deref @ SUBSCRIPTED_EXP { exp: __exp_exp, split: __exp_split, subscripts: __exp_subscripts, ty: __exp_ty } => {
            metamodelica::Ref::new(NFExpression::SUBSCRIPTED_EXP { exp: map(__exp_exp.clone(), func.clone())?, subscripts: ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
        for mut s in (__exp_subscripts.clone()).into_iter().cloned() {
            let __x = Subscript::mapExp(s.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), ty: __exp_ty.clone(), split: __exp_split.clone() })
        },
        Deref @ TUPLE_ELEMENT { index: __exp_index, tupleExp: __exp_tupleExp, ty: __exp_ty } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = map(__exp_tupleExp.clone(), func.clone())?;
            if (referenceEq(&*(__exp_tupleExp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::TUPLE_ELEMENT { tupleExp: e1, index: __exp_index.clone(), ty: __exp_ty.clone() })}
        },
        Deref @ RECORD_ELEMENT { fieldName: __exp_fieldName, index: __exp_index, recordExp: __exp_recordExp, ty: __exp_ty } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = map(__exp_recordExp.clone(), func.clone())?;
            if (referenceEq(&*(__exp_recordExp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::RECORD_ELEMENT { recordExp: e1, index: __exp_index.clone(), fieldName: __exp_fieldName.clone(), ty: __exp_ty.clone() })}
        },
        Deref @ MUTABLE { exp: __exp_exp } => {
            Mutable::update(__exp_exp.clone(), map(Mutable::access(__exp_exp.clone()), func.clone())?);
            exp
        },
        Deref @ SHARED_LITERAL { exp: __exp_exp, .. } => {
            assign_variant_field!(exp => NFExpression::SHARED_LITERAL; exp = map(__exp_exp.clone(), func.clone())?);
            exp
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { args: __exp_args, .. } => {
            assign_variant_field!(exp => NFExpression::PARTIAL_FUNCTION_APPLICATION; args = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
        for mut e in (__exp_args.clone()).into_iter().cloned() {
            let __x = map(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            exp
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp = func(outExp)?;
    Ok(outExp)
}

pub fn fakeMap(
    mut exp: metamodelica::Ref<NFExpression>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>>,
) -> Result<metamodelica::Ref<NFExpression>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static,
    >;

    let mut outExp: metamodelica::Ref<NFExpression> = func(exp.clone())?;
    Ok(outExp)
}

pub(crate) fn mapOpt(
    mut exp: Option<metamodelica::Ref<NFExpression>>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static,
    >,
) -> Result<Option<metamodelica::Ref<NFExpression>>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static,
    >;

    let mut outExp: Option<metamodelica::Ref<NFExpression>>;
    let mut e: metamodelica::Ref<NFExpression>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Some(__esc_e) => {
            e = (*__esc_e).clone();
            Some(map(e.clone(), func.clone())?)
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn mapReverse(
    mut exp: metamodelica::Ref<NFExpression>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static,
    >,
) -> Result<metamodelica::Ref<NFExpression>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static,
    >;

    let mut exp: metamodelica::Ref<NFExpression> = exp;
    exp = func(exp)?;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { clk: __exp_clk } => {
            metamodelica::Ref::new(NFExpression::CLKCONST { clk: ClockKind::mapExp(__exp_clk.clone(), func.clone())? })
        },
        Deref @ CREF { cref: __exp_cref, ty: __exp_ty } => {
            metamodelica::Ref::new(NFExpression::CREF { ty: __exp_ty.clone(), cref: ComponentRef::mapExp(metamodelica::AsArg::as_arg(&__exp_cref), func.clone())? })
        },
        Deref @ ARRAY { literal: __exp_literal, ty: __exp_ty, .. } if (!(__exp_literal.clone())) => {
            makeArray(__exp_ty.clone(), Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static> = func.clone(); move |__pe_a0| mapReverse(__pe_a0, __pe_b1.clone()) }))?, __exp_literal.clone())
        },
        Deref @ MATRIX { elements: __exp_elements } => {
            metamodelica::Ref::new(NFExpression::MATRIX { elements: ({
        let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<NFExpression>>> = metamodelica::nil();
        for mut row in (__exp_elements.clone()).into_iter().cloned() {
            let __x = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
        for mut e in (row.clone()).into_iter().cloned() {
            let __x = mapReverse(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) })
        },
        Deref @ RANGE { step: Some(e2), start: __exp_start, stop: __exp_stop, ty: __exp_ty } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e3: metamodelica::Ref<NFExpression>;
            let mut e4: metamodelica::Ref<NFExpression>;
            e1 = mapReverse(__exp_start.clone(), func.clone())?;
            e4 = mapReverse(e2.clone(), func.clone())?;
            e3 = mapReverse(__exp_stop.clone(), func.clone())?;
            if (referenceEq(&*(__exp_start.clone()),&*(&*e1)) && referenceEq(&*(e2.clone()),&*(&*e4)) && referenceEq(&*(__exp_stop.clone()),&*(&*e3))) {exp} else {metamodelica::Ref::new(NFExpression::RANGE { ty: __exp_ty.clone(), start: e1, step: Some(e4), stop: e3 })}
        },
        Deref @ RANGE { start: __exp_start, stop: __exp_stop, ty: __exp_ty, .. } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e3: metamodelica::Ref<NFExpression>;
            e1 = mapReverse(__exp_start.clone(), func.clone())?;
            e3 = mapReverse(__exp_stop.clone(), func.clone())?;
            if (referenceEq(&*(__exp_start.clone()),&*(&*e1)) && referenceEq(&*(__exp_stop.clone()),&*(&*e3))) {exp} else {metamodelica::Ref::new(NFExpression::RANGE { ty: __exp_ty.clone(), start: e1, step: None, stop: e3 })}
        },
        Deref @ TUPLE { elements: __exp_elements, ty: __exp_ty } => {
            metamodelica::Ref::new(NFExpression::TUPLE { ty: __exp_ty.clone(), elements: ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
        for mut e in (__exp_elements.clone()).into_iter().cloned() {
            let __x = mapReverse(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) })
        },
        Deref @ RECORD { elements: __exp_elements, path: __exp_path, ty: __exp_ty } => {
            metamodelica::Ref::new(NFExpression::RECORD { path: __exp_path.clone(), ty: __exp_ty.clone(), elements: ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
        for mut e in (__exp_elements.clone()).into_iter().cloned() {
            let __x = mapReverse(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) })
        },
        Deref @ CALL { call: __exp_call } => {
            metamodelica::Ref::new(NFExpression::CALL { call: Call::mapExp(metamodelica::AsArg::as_arg(&__exp_call), func.clone())? })
        },
        Deref @ SIZE { dimIndex: Some(e2), exp: __exp_exp } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e3: metamodelica::Ref<NFExpression>;
            e1 = mapReverse(__exp_exp.clone(), func.clone())?;
            e3 = mapReverse(e2.clone(), func.clone())?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1)) && referenceEq(&*(e2.clone()),&*(&*e3))) {exp} else {metamodelica::Ref::new(NFExpression::SIZE { exp: e1, dimIndex: Some(e3) })}
        },
        Deref @ SIZE { exp: __exp_exp, .. } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = mapReverse(__exp_exp.clone(), func.clone())?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::SIZE { exp: e1, dimIndex: None })}
        },
        Deref @ BINARY { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            e1 = mapReverse(__exp_exp1.clone(), func.clone())?;
            e2 = mapReverse(__exp_exp2.clone(), func.clone())?;
            if (referenceEq(&*(__exp_exp1.clone()),&*(&*e1)) && referenceEq(&*(__exp_exp2.clone()),&*(&*e2))) {exp} else {metamodelica::Ref::new(NFExpression::BINARY { exp1: e1, operator: __exp_operator.clone(), exp2: e2 })}
        },
        Deref @ MULTARY { arguments: __exp_arguments, .. } => {
            assign_variant_field!(exp => NFExpression::MULTARY;
                arguments = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
        for mut arg in (__exp_arguments.clone()).into_iter().cloned() {
            let __x = mapReverse(arg.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
                inv_arguments = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
        for mut arg in (var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = mapReverse(arg.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
            );
            exp
        },
        Deref @ UNARY { exp: __exp_exp, operator: __exp_operator } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = mapReverse(__exp_exp.clone(), func.clone())?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::UNARY { operator: __exp_operator.clone(), exp: e1 })}
        },
        Deref @ LBINARY { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            e1 = mapReverse(__exp_exp1.clone(), func.clone())?;
            e2 = mapReverse(__exp_exp2.clone(), func.clone())?;
            if (referenceEq(&*(__exp_exp1.clone()),&*(&*e1)) && referenceEq(&*(__exp_exp2.clone()),&*(&*e2))) {exp} else {metamodelica::Ref::new(NFExpression::LBINARY { exp1: e1, operator: __exp_operator.clone(), exp2: e2 })}
        },
        Deref @ LUNARY { exp: __exp_exp, operator: __exp_operator } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = mapReverse(__exp_exp.clone(), func.clone())?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::LUNARY { operator: __exp_operator.clone(), exp: e1 })}
        },
        Deref @ RELATION { exp1: __exp_exp1, exp2: __exp_exp2, index: __exp_index, operator: __exp_operator } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            e1 = mapReverse(__exp_exp1.clone(), func.clone())?;
            e2 = mapReverse(__exp_exp2.clone(), func.clone())?;
            if (referenceEq(&*(__exp_exp1.clone()),&*(&*e1)) && referenceEq(&*(__exp_exp2.clone()),&*(&*e2))) {exp} else {metamodelica::Ref::new(NFExpression::RELATION { exp1: e1, operator: __exp_operator.clone(), exp2: e2, index: __exp_index.clone() })}
        },
        Deref @ IF { condition: __exp_condition, falseBranch: __exp_falseBranch, trueBranch: __exp_trueBranch, ty: __exp_ty } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            let mut e3: metamodelica::Ref<NFExpression>;
            e1 = mapReverse(__exp_condition.clone(), func.clone())?;
            e2 = mapReverse(__exp_trueBranch.clone(), func.clone())?;
            e3 = mapReverse(__exp_falseBranch.clone(), func.clone())?;
            if (referenceEq(&*(__exp_condition.clone()),&*(&*e1)) && referenceEq(&*(__exp_trueBranch.clone()),&*(&*e2)) && referenceEq(&*(__exp_falseBranch.clone()),&*(&*e3))) {exp} else {metamodelica::Ref::new(NFExpression::IF { ty: __exp_ty.clone(), condition: e1, trueBranch: e2, falseBranch: e3 })}
        },
        Deref @ CAST { exp: __exp_exp, ty: __exp_ty } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = mapReverse(__exp_exp.clone(), func.clone())?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::CAST { ty: __exp_ty.clone(), exp: e1 })}
        },
        Deref @ BOX { exp: __exp_exp } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = mapReverse(__exp_exp.clone(), func.clone())?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {r#box(&e1)}
        },
        Deref @ UNBOX { exp: __exp_exp, .. } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = mapReverse(__exp_exp.clone(), func.clone())?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {unbox(e1)}
        },
        Deref @ SUBSCRIPTED_EXP { exp: __exp_exp, split: __exp_split, subscripts: __exp_subscripts, ty: __exp_ty } => {
            metamodelica::Ref::new(NFExpression::SUBSCRIPTED_EXP { exp: mapReverse(__exp_exp.clone(), func.clone())?, subscripts: ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
        for mut s in (__exp_subscripts.clone()).into_iter().cloned() {
            let __x = Subscript::mapExp(s.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), ty: __exp_ty.clone(), split: __exp_split.clone() })
        },
        Deref @ TUPLE_ELEMENT { index: __exp_index, tupleExp: __exp_tupleExp, ty: __exp_ty } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = mapReverse(__exp_tupleExp.clone(), func.clone())?;
            if (referenceEq(&*(__exp_tupleExp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::TUPLE_ELEMENT { tupleExp: e1, index: __exp_index.clone(), ty: __exp_ty.clone() })}
        },
        Deref @ RECORD_ELEMENT { fieldName: __exp_fieldName, index: __exp_index, recordExp: __exp_recordExp, ty: __exp_ty } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = mapReverse(__exp_recordExp.clone(), func.clone())?;
            if (referenceEq(&*(__exp_recordExp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::RECORD_ELEMENT { recordExp: e1, index: __exp_index.clone(), fieldName: __exp_fieldName.clone(), ty: __exp_ty.clone() })}
        },
        Deref @ MUTABLE { exp: __exp_exp } => {
            Mutable::update(__exp_exp.clone(), mapReverse(Mutable::access(__exp_exp.clone()), func.clone())?);
            exp
        },
        Deref @ SHARED_LITERAL { exp: __exp_exp, .. } => {
            assign_variant_field!(exp => NFExpression::SHARED_LITERAL; exp = mapReverse(__exp_exp.clone(), func.clone())?);
            exp
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { args: __exp_args, .. } => {
            assign_variant_field!(exp => NFExpression::PARTIAL_FUNCTION_APPLICATION; args = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
        for mut e in (__exp_args.clone()).into_iter().cloned() {
            let __x = mapReverse(e.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            exp
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub fn mapShallow(
    mut exp: metamodelica::Ref<NFExpression>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static,
    >,
) -> Result<metamodelica::Ref<NFExpression>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static,
    >;

    let mut outExp: metamodelica::Ref<NFExpression>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { clk: __exp_clk } => {
            metamodelica::Ref::new(NFExpression::CLKCONST { clk: ClockKind::mapExpShallow(__exp_clk.clone(), &*func)? })
        },
        Deref @ CREF { cref: __exp_cref, ty: __exp_ty } => {
            metamodelica::Ref::new(NFExpression::CREF { ty: __exp_ty.clone(), cref: ComponentRef::mapExpShallow(metamodelica::AsArg::as_arg(&__exp_cref), &*func)? })
        },
        Deref @ ARRAY { literal: __exp_literal, ty: __exp_ty, .. } if (!(__exp_literal.clone())) => {
            makeArray(__exp_ty.clone(), Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), &*func)?, __exp_literal.clone())
        },
        Deref @ MATRIX { elements: __exp_elements } => {
            metamodelica::Ref::new(NFExpression::MATRIX { elements: ({
        let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<NFExpression>>> = metamodelica::nil();
        for mut row in (__exp_elements.clone()).into_iter().cloned() {
            let __x = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
        for mut e in (row.clone()).into_iter().cloned() {
            let __x = func(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) })
        },
        Deref @ RANGE { step: Some(e2), start: __exp_start, stop: __exp_stop, ty: __exp_ty } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e3: metamodelica::Ref<NFExpression>;
            let mut e4: metamodelica::Ref<NFExpression>;
            e1 = func(__exp_start.clone())?;
            e4 = func(e2.clone())?;
            e3 = func(__exp_stop.clone())?;
            if (referenceEq(&*(__exp_start.clone()),&*(&*e1)) && referenceEq(&*(e2.clone()),&*(&*e4)) && referenceEq(&*(__exp_stop.clone()),&*(&*e3))) {exp} else {metamodelica::Ref::new(NFExpression::RANGE { ty: __exp_ty.clone(), start: e1, step: Some(e4), stop: e3 })}
        },
        Deref @ RANGE { start: __exp_start, stop: __exp_stop, ty: __exp_ty, .. } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e3: metamodelica::Ref<NFExpression>;
            e1 = func(__exp_start.clone())?;
            e3 = func(__exp_stop.clone())?;
            if (referenceEq(&*(__exp_start.clone()),&*(&*e1)) && referenceEq(&*(__exp_stop.clone()),&*(&*e3))) {exp} else {metamodelica::Ref::new(NFExpression::RANGE { ty: __exp_ty.clone(), start: e1, step: None, stop: e3 })}
        },
        Deref @ TUPLE { elements: __exp_elements, ty: __exp_ty } => {
            metamodelica::Ref::new(NFExpression::TUPLE { ty: __exp_ty.clone(), elements: ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
        for mut e in (__exp_elements.clone()).into_iter().cloned() {
            let __x = func(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) })
        },
        Deref @ RECORD { elements: __exp_elements, path: __exp_path, ty: __exp_ty } => {
            metamodelica::Ref::new(NFExpression::RECORD { path: __exp_path.clone(), ty: __exp_ty.clone(), elements: ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
        for mut e in (__exp_elements.clone()).into_iter().cloned() {
            let __x = func(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) })
        },
        Deref @ CALL { call: __exp_call } => {
            metamodelica::Ref::new(NFExpression::CALL { call: Call::mapExpShallow(metamodelica::AsArg::as_arg(&__exp_call), func.clone())? })
        },
        Deref @ SIZE { dimIndex: Some(e2), exp: __exp_exp } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e3: metamodelica::Ref<NFExpression>;
            e1 = func(__exp_exp.clone())?;
            e3 = func(e2.clone())?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1)) && referenceEq(&*(e2.clone()),&*(&*e3))) {exp} else {metamodelica::Ref::new(NFExpression::SIZE { exp: e1, dimIndex: Some(e3) })}
        },
        Deref @ SIZE { exp: __exp_exp, .. } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = func(__exp_exp.clone())?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::SIZE { exp: e1, dimIndex: None })}
        },
        Deref @ BINARY { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            e1 = func(__exp_exp1.clone())?;
            e2 = func(__exp_exp2.clone())?;
            if (referenceEq(&*(__exp_exp1.clone()),&*(&*e1)) && referenceEq(&*(__exp_exp2.clone()),&*(&*e2))) {exp} else {metamodelica::Ref::new(NFExpression::BINARY { exp1: e1, operator: __exp_operator.clone(), exp2: e2 })}
        },
        Deref @ MULTARY { arguments: __exp_arguments, .. } => {
            assign_variant_field!(exp => NFExpression::MULTARY;
                arguments = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
        for mut arg in (__exp_arguments.clone()).into_iter().cloned() {
            let __x = func(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
                inv_arguments = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
        for mut arg in (var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone()).into_iter().cloned() {
            let __x = func(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
            );
            exp
        },
        Deref @ UNARY { exp: __exp_exp, operator: __exp_operator } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = func(__exp_exp.clone())?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::UNARY { operator: __exp_operator.clone(), exp: e1 })}
        },
        Deref @ LBINARY { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            e1 = func(__exp_exp1.clone())?;
            e2 = func(__exp_exp2.clone())?;
            if (referenceEq(&*(__exp_exp1.clone()),&*(&*e1)) && referenceEq(&*(__exp_exp2.clone()),&*(&*e2))) {exp} else {metamodelica::Ref::new(NFExpression::LBINARY { exp1: e1, operator: __exp_operator.clone(), exp2: e2 })}
        },
        Deref @ LUNARY { exp: __exp_exp, operator: __exp_operator } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = func(__exp_exp.clone())?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::LUNARY { operator: __exp_operator.clone(), exp: e1 })}
        },
        Deref @ RELATION { exp1: __exp_exp1, exp2: __exp_exp2, index: __exp_index, operator: __exp_operator } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            e1 = func(__exp_exp1.clone())?;
            e2 = func(__exp_exp2.clone())?;
            if (referenceEq(&*(__exp_exp1.clone()),&*(&*e1)) && referenceEq(&*(__exp_exp2.clone()),&*(&*e2))) {exp} else {metamodelica::Ref::new(NFExpression::RELATION { exp1: e1, operator: __exp_operator.clone(), exp2: e2, index: __exp_index.clone() })}
        },
        Deref @ IF { condition: __exp_condition, falseBranch: __exp_falseBranch, trueBranch: __exp_trueBranch, ty: __exp_ty } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            let mut e3: metamodelica::Ref<NFExpression>;
            e1 = func(__exp_condition.clone())?;
            e2 = func(__exp_trueBranch.clone())?;
            e3 = func(__exp_falseBranch.clone())?;
            if (referenceEq(&*(__exp_condition.clone()),&*(&*e1)) && referenceEq(&*(__exp_trueBranch.clone()),&*(&*e2)) && referenceEq(&*(__exp_falseBranch.clone()),&*(&*e3))) {exp} else {metamodelica::Ref::new(NFExpression::IF { ty: __exp_ty.clone(), condition: e1, trueBranch: e2, falseBranch: e3 })}
        },
        Deref @ CAST { exp: __exp_exp, ty: __exp_ty } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = func(__exp_exp.clone())?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::CAST { ty: __exp_ty.clone(), exp: e1 })}
        },
        Deref @ BOX { exp: __exp_exp } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = func(__exp_exp.clone())?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {r#box(&e1)}
        },
        Deref @ UNBOX { exp: __exp_exp, .. } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = func(__exp_exp.clone())?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {unbox(e1)}
        },
        Deref @ SUBSCRIPTED_EXP { exp: __exp_exp, split: __exp_split, subscripts: __exp_subscripts, ty: __exp_ty } => {
            metamodelica::Ref::new(NFExpression::SUBSCRIPTED_EXP { exp: func(__exp_exp.clone())?, subscripts: ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
        for mut e in (__exp_subscripts.clone()).into_iter().cloned() {
            let __x = Subscript::mapShallowExp(e.clone(), &*func)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), ty: __exp_ty.clone(), split: __exp_split.clone() })
        },
        Deref @ TUPLE_ELEMENT { index: __exp_index, tupleExp: __exp_tupleExp, ty: __exp_ty } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = func(__exp_tupleExp.clone())?;
            if (referenceEq(&*(__exp_tupleExp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::TUPLE_ELEMENT { tupleExp: e1, index: __exp_index.clone(), ty: __exp_ty.clone() })}
        },
        Deref @ RECORD_ELEMENT { fieldName: __exp_fieldName, index: __exp_index, recordExp: __exp_recordExp, ty: __exp_ty } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            e1 = func(__exp_recordExp.clone())?;
            if (referenceEq(&*(__exp_recordExp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::RECORD_ELEMENT { recordExp: e1, index: __exp_index.clone(), fieldName: __exp_fieldName.clone(), ty: __exp_ty.clone() })}
        },
        Deref @ MUTABLE { exp: __exp_exp } => {
            Mutable::update(__exp_exp.clone(), func(Mutable::access(__exp_exp.clone()))?);
            exp
        },
        Deref @ SHARED_LITERAL { exp: __exp_exp, .. } => {
            assign_variant_field!(exp => NFExpression::SHARED_LITERAL; exp = func(__exp_exp.clone())?);
            exp
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { args: __exp_args, .. } => {
            assign_variant_field!(exp => NFExpression::PARTIAL_FUNCTION_APPLICATION; args = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
        for mut e in (__exp_args.clone()).into_iter().cloned() {
            let __x = func(e.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            exp
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn mapShallowOpt(
    mut exp: Option<metamodelica::Ref<NFExpression>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>>,
) -> Result<Option<metamodelica::Ref<NFExpression>>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static,
    >;

    let mut outExp: Option<metamodelica::Ref<NFExpression>>;
    let mut e: metamodelica::Ref<NFExpression>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Some(__esc_e) => {
            e = (*__esc_e).clone();
            Some(func(e.clone())?)
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn mapArrayElements(
    mut exp: metamodelica::Ref<NFExpression>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static,
    >,
) -> Result<metamodelica::Ref<NFExpression>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static,
    >;

    let mut outExp: metamodelica::Ref<NFExpression>;
    outExp = (match &*exp {
        ARRAY { .. } => {
            assign_variant_field!(exp => NFExpression::ARRAY; elements = Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static> = func.clone(); move |__pe_a0| mapArrayElements(__pe_a0, __pe_b1.clone()) }))?);
            assign_variant_field!(exp => NFExpression::ARRAY; literal = Array::all(var_field!((*exp).elements, NFExpression::ARRAY).clone(), &move |__a0: metamodelica::Ref<NFExpression>| isLiteral(&__a0))?);
            exp
        }
        _ => func(exp)?,
    });
    Ok(outExp)
}

pub(crate) fn foldArray<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut expl: metamodelica::Array<metamodelica::Ref<NFExpression>>,
    mut func: Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, ArgT) -> Result<ArgT> + 'static>,
    mut arg: ArgT,
) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut result: ArgT = arg;
    let __range0 = expl.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        result = fold(e, func.clone(), result)?;
    }
    Ok(result)
}

pub(crate) fn foldList<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut expl: &metamodelica::List<metamodelica::Ref<NFExpression>>,
    mut func: Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, ArgT) -> Result<ArgT> + 'static>,
    mut arg: ArgT,
) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut result: ArgT = arg;
    for mut e in &**expl {
        result = fold(e.clone(), func.clone(), result)?;
    }
    Ok(result)
}

pub(crate) fn foldOpt<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut exp: Option<metamodelica::Ref<NFExpression>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, ArgT) -> Result<ArgT>,
    mut arg: ArgT,
) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut result: ArgT;
    result = (::match_deref::match_deref! { match &(exp) {
        Some(e) => {
            func(e.clone(), arg)?
        },
        _ => {
            arg
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub fn fold<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut exp: metamodelica::Ref<NFExpression>,
    mut func: Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, ArgT) -> Result<ArgT> + 'static>,
    mut arg: ArgT,
) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut result: ArgT;
    result = (::match_deref::match_deref! { match &(&*exp) {
        Deref @ CLKCONST { clk: __exp_clk } => {
            ClockKind::foldExp(metamodelica::AsArg::as_arg(&__exp_clk), func.clone(), arg)?
        },
        Deref @ CREF { cref: __exp_cref, .. } => {
            ComponentRef::foldExp(metamodelica::AsArg::as_arg(&__exp_cref), func.clone(), arg)?
        },
        Deref @ ARRAY { .. } => {
            foldArray(var_field!((*exp).elements, NFExpression::ARRAY).clone(), func.clone(), arg)?
        },
        Deref @ MATRIX { elements: __exp_elements } => {
            result = arg;
            for mut row in &*__exp_elements.clone() {
                result = foldList(metamodelica::AsArg::as_arg(&row), func.clone(), result)?;
            }
            result
        },
        Deref @ RANGE { start: __exp_start, step: __exp_step, stop: __exp_stop, .. } => {
            result = fold(__exp_start.clone(), func.clone(), arg)?;
            result = foldOpt(__exp_step.clone(), &*func, result)?;
            fold(__exp_stop.clone(), func.clone(), result)?
        },
        Deref @ TUPLE { elements: __exp_elements, .. } => {
            foldList(metamodelica::AsArg::as_arg(&__exp_elements), func.clone(), arg)?
        },
        Deref @ RECORD { elements: __exp_elements, .. } => {
            foldList(metamodelica::AsArg::as_arg(&__exp_elements), func.clone(), arg)?
        },
        Deref @ CALL { call: __exp_call } => {
            Call::foldExp(metamodelica::AsArg::as_arg(&__exp_call), func.clone(), arg)?
        },
        Deref @ SIZE { dimIndex: Some(e), exp: __exp_exp } => {
            result = fold(__exp_exp.clone(), func.clone(), arg)?;
            fold(e.clone(), func.clone(), result)?
        },
        Deref @ SIZE { exp: __exp_exp, .. } => {
            fold(__exp_exp.clone(), func.clone(), arg)?
        },
        Deref @ BINARY { exp1: __exp_exp1, exp2: __exp_exp2, .. } => {
            result = fold(__exp_exp1.clone(), func.clone(), arg)?;
            fold(__exp_exp2.clone(), func.clone(), result)?
        },
        Deref @ MULTARY { arguments: __exp_arguments, inv_arguments: __exp_inv_arguments, .. } => {
            result = arg;
            for mut argument in &*__exp_arguments.clone() {
                result = fold(argument.clone(), func.clone(), result)?;
            }
            for mut argument in &*__exp_inv_arguments.clone() {
                result = fold(argument.clone(), func.clone(), result)?;
            }
            result
        },
        Deref @ UNARY { exp: __exp_exp, .. } => {
            fold(__exp_exp.clone(), func.clone(), arg)?
        },
        Deref @ LBINARY { exp1: __exp_exp1, exp2: __exp_exp2, .. } => {
            result = fold(__exp_exp1.clone(), func.clone(), arg)?;
            fold(__exp_exp2.clone(), func.clone(), result)?
        },
        Deref @ LUNARY { exp: __exp_exp, .. } => {
            fold(__exp_exp.clone(), func.clone(), arg)?
        },
        Deref @ RELATION { exp1: __exp_exp1, exp2: __exp_exp2, .. } => {
            result = fold(__exp_exp1.clone(), func.clone(), arg)?;
            fold(__exp_exp2.clone(), func.clone(), result)?
        },
        Deref @ IF { condition: __exp_condition, falseBranch: __exp_falseBranch, trueBranch: __exp_trueBranch, .. } => {
            result = fold(__exp_condition.clone(), func.clone(), arg)?;
            result = fold(__exp_trueBranch.clone(), func.clone(), result)?;
            fold(__exp_falseBranch.clone(), func.clone(), result)?
        },
        Deref @ CAST { exp: __exp_exp, .. } => {
            fold(__exp_exp.clone(), func.clone(), arg)?
        },
        Deref @ BOX { exp: __exp_exp } => {
            fold(__exp_exp.clone(), func.clone(), arg)?
        },
        Deref @ UNBOX { exp: __exp_exp, .. } => {
            fold(__exp_exp.clone(), func.clone(), arg)?
        },
        Deref @ SUBSCRIPTED_EXP { exp: __exp_exp, subscripts: __exp_subscripts, .. } => {
            result = fold(__exp_exp.clone(), func.clone(), arg)?;
            List::fold(metamodelica::AsArg::as_arg(&__exp_subscripts), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| Subscript::foldExp(&__pe_a0, __pe_b1.clone(), __pe_a2) }), result)?
        },
        Deref @ TUPLE_ELEMENT { tupleExp: __exp_tupleExp, .. } => {
            fold(__exp_tupleExp.clone(), func.clone(), arg)?
        },
        Deref @ RECORD_ELEMENT { recordExp: __exp_recordExp, .. } => {
            fold(__exp_recordExp.clone(), func.clone(), arg)?
        },
        Deref @ MUTABLE { exp: __exp_exp } => {
            fold(Mutable::access(__exp_exp.clone()), func.clone(), arg)?
        },
        Deref @ SHARED_LITERAL { exp: __exp_exp, .. } => {
            fold(__exp_exp.clone(), func.clone(), arg)?
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { args: __exp_args, .. } => {
            foldList(metamodelica::AsArg::as_arg(&__exp_args), func.clone(), arg)?
        },
        _ => {
            arg
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result = func(exp, result)?;
    Ok(result)
}

pub(crate) fn applyArray(
    mut expl: metamodelica::Array<metamodelica::Ref<NFExpression>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<()> + 'static>;

    let __range0 = expl.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        apply(e, func)?;
    }
    Ok(())
}

pub(crate) fn applyList(
    mut expl: &metamodelica::List<metamodelica::Ref<NFExpression>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<()> + 'static>;

    for mut e in &**expl {
        apply(e.clone(), func)?;
    }
    Ok(())
}

pub(crate) fn applyOpt(
    mut exp: Option<metamodelica::Ref<NFExpression>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<()> + 'static>;

    let mut e: metamodelica::Ref<NFExpression>;
    if (exp).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(exp) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        apply(e, func)?;
    }
    Ok(())
}

pub(crate) fn apply(
    mut exp: metamodelica::Ref<NFExpression>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<()> + 'static>;

    let () = (match &*exp {
        CLKCONST { clk: __exp_clk } => {
            ClockKind::applyExp(metamodelica::AsArg::as_arg(&__exp_clk), func)?;
            ()
        }
        CREF { cref: __exp_cref, .. } => {
            ComponentRef::applyExp(metamodelica::AsArg::as_arg(&__exp_cref), func)?;
            ()
        }
        ARRAY { .. } => {
            applyArray(var_field!((*exp).elements, NFExpression::ARRAY).clone(), func)?;
            ()
        }
        MATRIX {
            elements: __exp_elements,
        } => {
            for mut row in &*__exp_elements.clone() {
                applyList(metamodelica::AsArg::as_arg(&row), func)?;
            }
            ()
        }
        RANGE {
            start: __exp_start,
            step: __exp_step,
            stop: __exp_stop,
            ..
        } => {
            apply(__exp_start.clone(), func)?;
            applyOpt(__exp_step.clone(), func)?;
            apply(__exp_stop.clone(), func)?;
            ()
        }
        TUPLE {
            elements: __exp_elements,
            ..
        } => {
            applyList(metamodelica::AsArg::as_arg(&__exp_elements), func)?;
            ()
        }
        RECORD {
            elements: __exp_elements,
            ..
        } => {
            applyList(metamodelica::AsArg::as_arg(&__exp_elements), func)?;
            ()
        }
        CALL { call: __exp_call } => {
            Call::applyExp(metamodelica::AsArg::as_arg(&__exp_call), func)?;
            ()
        }
        SIZE {
            dimIndex: __exp_dimIndex,
            exp: __exp_exp,
        } => {
            apply(__exp_exp.clone(), func)?;
            applyOpt(__exp_dimIndex.clone(), func)?;
            ()
        }
        BINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            ..
        } => {
            apply(__exp_exp1.clone(), func)?;
            apply(__exp_exp2.clone(), func)?;
            ()
        }
        MULTARY {
            arguments: __exp_arguments,
            inv_arguments: __exp_inv_arguments,
            ..
        } => {
            for mut arg in &*__exp_arguments.clone() {
                apply(arg.clone(), func)?;
            }
            for mut arg in &*__exp_inv_arguments.clone() {
                apply(arg.clone(), func)?;
            }
            ()
        }
        UNARY { exp: __exp_exp, .. } => {
            apply(__exp_exp.clone(), func)?;
            ()
        }
        LBINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            ..
        } => {
            apply(__exp_exp1.clone(), func)?;
            apply(__exp_exp2.clone(), func)?;
            ()
        }
        LUNARY { exp: __exp_exp, .. } => {
            apply(__exp_exp.clone(), func)?;
            ()
        }
        RELATION {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            ..
        } => {
            apply(__exp_exp1.clone(), func)?;
            apply(__exp_exp2.clone(), func)?;
            ()
        }
        IF {
            condition: __exp_condition,
            falseBranch: __exp_falseBranch,
            trueBranch: __exp_trueBranch,
            ..
        } => {
            apply(__exp_condition.clone(), func)?;
            apply(__exp_trueBranch.clone(), func)?;
            apply(__exp_falseBranch.clone(), func)?;
            ()
        }
        CAST { exp: __exp_exp, .. } => {
            apply(__exp_exp.clone(), func)?;
            ()
        }
        BOX { exp: __exp_exp } => {
            apply(__exp_exp.clone(), func)?;
            ()
        }
        UNBOX { exp: __exp_exp, .. } => {
            apply(__exp_exp.clone(), func)?;
            ()
        }
        SUBSCRIPTED_EXP {
            exp: __exp_exp,
            subscripts: __exp_subscripts,
            ..
        } => {
            apply(__exp_exp.clone(), func)?;
            for mut s in &*__exp_subscripts.clone() {
                Subscript::applyExp(metamodelica::AsArg::as_arg(&s), func)?;
            }
            ()
        }
        TUPLE_ELEMENT {
            tupleExp: __exp_tupleExp,
            ..
        } => {
            apply(__exp_tupleExp.clone(), func)?;
            ()
        }
        RECORD_ELEMENT {
            recordExp: __exp_recordExp,
            ..
        } => {
            apply(__exp_recordExp.clone(), func)?;
            ()
        }
        MUTABLE { exp: __exp_exp } => {
            apply(Mutable::access(__exp_exp.clone()), func)?;
            ()
        }
        SHARED_LITERAL { exp: __exp_exp, .. } => {
            apply(__exp_exp.clone(), func)?;
            ()
        }
        PARTIAL_FUNCTION_APPLICATION { args: __exp_args, .. } => {
            applyList(metamodelica::AsArg::as_arg(&__exp_args), func)?;
            ()
        }
        _ => (),
    });
    func(exp)?;
    Ok(())
}

pub(crate) fn applyArrayShallow(
    mut expl: metamodelica::Array<metamodelica::Ref<NFExpression>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<()> + 'static>;

    let __range0 = expl.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        func(e)?;
    }
    Ok(())
}

pub(crate) fn applyListShallow(
    mut expl: &metamodelica::List<metamodelica::Ref<NFExpression>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<()> + 'static>;

    for mut e in &**expl {
        func(e.clone())?;
    }
    Ok(())
}

pub(crate) fn applyShallow(
    mut exp: &metamodelica::Ref<NFExpression>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<()> + 'static>;

    let () = (match &**exp {
        CLKCONST { clk: __exp_clk } => {
            ClockKind::applyExpShallow(metamodelica::AsArg::as_arg(&__exp_clk), func)?;
            ()
        }
        CREF { cref: __exp_cref, .. } => {
            ComponentRef::applyExpShallow(metamodelica::AsArg::as_arg(&__exp_cref), func)?;
            ()
        }
        ARRAY { .. } => {
            applyArrayShallow(var_field!((**exp).elements, NFExpression::ARRAY).clone(), func)?;
            ()
        }
        MATRIX {
            elements: __exp_elements,
        } => {
            for mut row in &*__exp_elements.clone() {
                applyListShallow(metamodelica::AsArg::as_arg(&row), func)?;
            }
            ()
        }
        RANGE {
            start: __exp_start,
            step: __exp_step,
            stop: __exp_stop,
            ..
        } => {
            func(__exp_start.clone())?;
            applyShallowOpt(__exp_step.clone(), func)?;
            func(__exp_stop.clone())?;
            ()
        }
        TUPLE {
            elements: __exp_elements,
            ..
        } => {
            applyListShallow(metamodelica::AsArg::as_arg(&__exp_elements), func)?;
            ()
        }
        RECORD {
            elements: __exp_elements,
            ..
        } => {
            applyListShallow(metamodelica::AsArg::as_arg(&__exp_elements), func)?;
            ()
        }
        CALL { call: __exp_call } => {
            Call::applyExpShallow(metamodelica::AsArg::as_arg(&__exp_call), func)?;
            ()
        }
        SIZE {
            dimIndex: __exp_dimIndex,
            exp: __exp_exp,
        } => {
            func(__exp_exp.clone())?;
            applyShallowOpt(__exp_dimIndex.clone(), func)?;
            ()
        }
        BINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            ..
        } => {
            func(__exp_exp1.clone())?;
            func(__exp_exp2.clone())?;
            ()
        }
        MULTARY {
            arguments: __exp_arguments,
            inv_arguments: __exp_inv_arguments,
            ..
        } => {
            for mut arg in &*__exp_arguments.clone() {
                func(arg.clone())?;
            }
            for mut arg in &*__exp_inv_arguments.clone() {
                func(arg.clone())?;
            }
            ()
        }
        UNARY { exp: __exp_exp, .. } => {
            func(__exp_exp.clone())?;
            ()
        }
        LBINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            ..
        } => {
            func(__exp_exp1.clone())?;
            func(__exp_exp2.clone())?;
            ()
        }
        LUNARY { exp: __exp_exp, .. } => {
            func(__exp_exp.clone())?;
            ()
        }
        RELATION {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            ..
        } => {
            func(__exp_exp1.clone())?;
            func(__exp_exp2.clone())?;
            ()
        }
        IF {
            condition: __exp_condition,
            falseBranch: __exp_falseBranch,
            trueBranch: __exp_trueBranch,
            ..
        } => {
            func(__exp_condition.clone())?;
            func(__exp_trueBranch.clone())?;
            func(__exp_falseBranch.clone())?;
            ()
        }
        CAST { exp: __exp_exp, .. } => {
            func(__exp_exp.clone())?;
            ()
        }
        BOX { exp: __exp_exp } => {
            func(__exp_exp.clone())?;
            ()
        }
        UNBOX { exp: __exp_exp, .. } => {
            func(__exp_exp.clone())?;
            ()
        }
        SUBSCRIPTED_EXP {
            exp: __exp_exp,
            subscripts: __exp_subscripts,
            ..
        } => {
            func(__exp_exp.clone())?;
            for mut s in &*__exp_subscripts.clone() {
                Subscript::applyExpShallow(metamodelica::AsArg::as_arg(&s), func)?;
            }
            ()
        }
        TUPLE_ELEMENT {
            tupleExp: __exp_tupleExp,
            ..
        } => {
            func(__exp_tupleExp.clone())?;
            ()
        }
        RECORD_ELEMENT {
            recordExp: __exp_recordExp,
            ..
        } => {
            func(__exp_recordExp.clone())?;
            ()
        }
        MUTABLE { exp: __exp_exp } => {
            func(Mutable::access(__exp_exp.clone()))?;
            ()
        }
        SHARED_LITERAL { exp: __exp_exp, .. } => {
            func(__exp_exp.clone())?;
            ()
        }
        PARTIAL_FUNCTION_APPLICATION { args: __exp_args, .. } => {
            applyListShallow(metamodelica::AsArg::as_arg(&__exp_args), func)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn applyShallowOpt(
    mut exp: Option<metamodelica::Ref<NFExpression>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<()> + 'static>;

    let mut e: metamodelica::Ref<NFExpression>;
    if (exp).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(exp) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        func(e)?;
    }
    Ok(())
}

pub fn mapFold<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut exp: metamodelica::Ref<NFExpression>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, ArgT) -> Result<(metamodelica::Ref<NFExpression>, ArgT)>
            + 'static,
    >,
    mut arg: ArgT,
) -> Result<(metamodelica::Ref<NFExpression>, ArgT)> {
    pub type MapFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, ArgT) -> Result<(metamodelica::Ref<NFExpression>, ArgT)>
            + 'static,
    >;

    let mut outExp: metamodelica::Ref<NFExpression>;
    let mut arg: ArgT = arg;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CLKCONST { clk: __exp_clk } => {
            let mut ck: metamodelica::Ref<ClockKind::NFClockKind>;
            (ck, arg) = ClockKind::mapFoldExp(__exp_clk.clone(), func.clone(), arg)?;
            if (referenceEq(&*(__exp_clk.clone()),&*(&*ck))) {exp} else {metamodelica::Ref::new(NFExpression::CLKCONST { clk: ck })}
        },
        Deref @ CREF { cref: __exp_cref, ty: __exp_ty } => {
            let mut cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
            (cr, arg) = ComponentRef::mapFoldExp(metamodelica::AsArg::as_arg(&__exp_cref), func.clone(), arg)?;
            if (referenceEq(&*(__exp_cref.clone()),&*(&*cr))) {exp} else {metamodelica::Ref::new(NFExpression::CREF { ty: __exp_ty.clone(), cref: cr })}
        },
        Deref @ ARRAY { literal: __exp_literal, ty: __exp_ty, .. } => {
            let mut arr: metamodelica::Array<metamodelica::Ref<NFExpression>>;
            (arr, arg) = Array::mapFold(var_field!((*exp).elements, NFExpression::ARRAY).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| mapFold(__pe_a0, __pe_b1.clone(), __pe_a2) }), arg)?;
            makeArray(__exp_ty.clone(), arr.clone(), __exp_literal.clone())
        },
        Deref @ MATRIX { elements: __exp_elements } => {
            let mut mat: metamodelica::List<metamodelica::List<metamodelica::Ref<NFExpression>>>;
            (mat, arg) = List::mapFoldList(metamodelica::AsArg::as_arg(&__exp_elements), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| mapFold(__pe_a0, __pe_b1.clone(), __pe_a2) }), arg)?;
            metamodelica::Ref::new(NFExpression::MATRIX { elements: mat })
        },
        Deref @ RANGE { step: Some(e2), start: __exp_start, stop: __exp_stop, ty: __exp_ty } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e3: metamodelica::Ref<NFExpression>;
            let mut e4: metamodelica::Ref<NFExpression>;
            (e1, arg) = mapFold(__exp_start.clone(), func.clone(), arg)?;
            (e4, arg) = mapFold(e2.clone(), func.clone(), arg)?;
            (e3, arg) = mapFold(__exp_stop.clone(), func.clone(), arg)?;
            if (referenceEq(&*(__exp_start.clone()),&*(&*e1)) && referenceEq(&*(e2.clone()),&*(&*e4)) && referenceEq(&*(__exp_stop.clone()),&*(&*e3))) {exp} else {metamodelica::Ref::new(NFExpression::RANGE { ty: __exp_ty.clone(), start: e1, step: Some(e4), stop: e3 })}
        },
        Deref @ RANGE { start: __exp_start, stop: __exp_stop, ty: __exp_ty, .. } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e3: metamodelica::Ref<NFExpression>;
            (e1, arg) = mapFold(__exp_start.clone(), func.clone(), arg)?;
            (e3, arg) = mapFold(__exp_stop.clone(), func.clone(), arg)?;
            if (referenceEq(&*(__exp_start.clone()),&*(&*e1)) && referenceEq(&*(__exp_stop.clone()),&*(&*e3))) {exp} else {metamodelica::Ref::new(NFExpression::RANGE { ty: __exp_ty.clone(), start: e1, step: None, stop: e3 })}
        },
        Deref @ TUPLE { elements: __exp_elements, ty: __exp_ty } => {
            let mut expl: metamodelica::List<metamodelica::Ref<NFExpression>>;
            (expl, arg) = List::map1Fold(metamodelica::AsArg::as_arg(&__exp_elements), &mapFold, func.clone(), arg)?;
            metamodelica::Ref::new(NFExpression::TUPLE { ty: __exp_ty.clone(), elements: expl })
        },
        Deref @ RECORD { elements: __exp_elements, path: __exp_path, ty: __exp_ty } => {
            let mut expl: metamodelica::List<metamodelica::Ref<NFExpression>>;
            (expl, arg) = List::map1Fold(metamodelica::AsArg::as_arg(&__exp_elements), &mapFold, func.clone(), arg)?;
            metamodelica::Ref::new(NFExpression::RECORD { path: __exp_path.clone(), ty: __exp_ty.clone(), elements: expl })
        },
        Deref @ CALL { call: __exp_call } => {
            let mut call: metamodelica::Ref<Call::NFCall>;
            (call, arg) = Call::mapFoldExp(metamodelica::AsArg::as_arg(&__exp_call), func.clone(), arg)?;
            if (referenceEq(&*(__exp_call.clone()),&*(&*call))) {exp} else {metamodelica::Ref::new(NFExpression::CALL { call: call })}
        },
        Deref @ SIZE { dimIndex: Some(e2), exp: __exp_exp } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e3: metamodelica::Ref<NFExpression>;
            (e1, arg) = mapFold(__exp_exp.clone(), func.clone(), arg)?;
            (e3, arg) = mapFold(e2.clone(), func.clone(), arg)?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1)) && referenceEq(&*(e2.clone()),&*(&*e3))) {exp} else {metamodelica::Ref::new(NFExpression::SIZE { exp: e1, dimIndex: Some(e3) })}
        },
        Deref @ SIZE { exp: __exp_exp, .. } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            (e1, arg) = mapFold(__exp_exp.clone(), func.clone(), arg)?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::SIZE { exp: e1, dimIndex: None })}
        },
        Deref @ BINARY { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            (e1, arg) = mapFold(__exp_exp1.clone(), func.clone(), arg)?;
            (e2, arg) = mapFold(__exp_exp2.clone(), func.clone(), arg)?;
            if (referenceEq(&*(__exp_exp1.clone()),&*(&*e1)) && referenceEq(&*(__exp_exp2.clone()),&*(&*e2))) {exp} else {metamodelica::Ref::new(NFExpression::BINARY { exp1: e1, operator: __exp_operator.clone(), exp2: e2 })}
        },
        Deref @ MULTARY { arguments: __exp_arguments, .. } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut expl: metamodelica::List<metamodelica::Ref<NFExpression>>;
            expl = metamodelica::nil();
            for mut argument in &*__exp_arguments.clone() {
                (e1, arg) = mapFold(argument.clone(), func.clone(), arg)?;
                expl = metamodelica::cons(e1, expl);
            }
            assign_variant_field!(exp => NFExpression::MULTARY; arguments = expl.reverse());
            expl = metamodelica::nil();
            for mut argument in &*var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone() {
                (e1, arg) = mapFold(argument.clone(), func.clone(), arg)?;
                expl = metamodelica::cons(e1, expl);
            }
            assign_variant_field!(exp => NFExpression::MULTARY; inv_arguments = expl.reverse());
            exp
        },
        Deref @ UNARY { exp: __exp_exp, operator: __exp_operator } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            (e1, arg) = mapFold(__exp_exp.clone(), func.clone(), arg)?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::UNARY { operator: __exp_operator.clone(), exp: e1 })}
        },
        Deref @ LBINARY { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            (e1, arg) = mapFold(__exp_exp1.clone(), func.clone(), arg)?;
            (e2, arg) = mapFold(__exp_exp2.clone(), func.clone(), arg)?;
            if (referenceEq(&*(__exp_exp1.clone()),&*(&*e1)) && referenceEq(&*(__exp_exp2.clone()),&*(&*e2))) {exp} else {metamodelica::Ref::new(NFExpression::LBINARY { exp1: e1, operator: __exp_operator.clone(), exp2: e2 })}
        },
        Deref @ LUNARY { exp: __exp_exp, operator: __exp_operator } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            (e1, arg) = mapFold(__exp_exp.clone(), func.clone(), arg)?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::LUNARY { operator: __exp_operator.clone(), exp: e1 })}
        },
        Deref @ RELATION { exp1: __exp_exp1, exp2: __exp_exp2, index: __exp_index, operator: __exp_operator } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            (e1, arg) = mapFold(__exp_exp1.clone(), func.clone(), arg)?;
            (e2, arg) = mapFold(__exp_exp2.clone(), func.clone(), arg)?;
            if (referenceEq(&*(__exp_exp1.clone()),&*(&*e1)) && referenceEq(&*(__exp_exp2.clone()),&*(&*e2))) {exp} else {metamodelica::Ref::new(NFExpression::RELATION { exp1: e1, operator: __exp_operator.clone(), exp2: e2, index: __exp_index.clone() })}
        },
        Deref @ IF { condition: __exp_condition, falseBranch: __exp_falseBranch, trueBranch: __exp_trueBranch, ty: __exp_ty } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            let mut e3: metamodelica::Ref<NFExpression>;
            (e1, arg) = mapFold(__exp_condition.clone(), func.clone(), arg)?;
            (e2, arg) = mapFold(__exp_trueBranch.clone(), func.clone(), arg)?;
            (e3, arg) = mapFold(__exp_falseBranch.clone(), func.clone(), arg)?;
            if (referenceEq(&*(__exp_condition.clone()),&*(&*e1)) && referenceEq(&*(__exp_trueBranch.clone()),&*(&*e2)) && referenceEq(&*(__exp_falseBranch.clone()),&*(&*e3))) {exp} else {metamodelica::Ref::new(NFExpression::IF { ty: __exp_ty.clone(), condition: e1, trueBranch: e2, falseBranch: e3 })}
        },
        Deref @ CAST { exp: __exp_exp, ty: __exp_ty } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            (e1, arg) = mapFold(__exp_exp.clone(), func.clone(), arg)?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::CAST { ty: __exp_ty.clone(), exp: e1 })}
        },
        Deref @ BOX { exp: __exp_exp } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            (e1, arg) = mapFold(__exp_exp.clone(), func.clone(), arg)?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {r#box(&e1)}
        },
        Deref @ UNBOX { exp: __exp_exp, .. } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            (e1, arg) = mapFold(__exp_exp.clone(), func.clone(), arg)?;
            if (referenceEq(&*(__exp_exp.clone()),&*(&*e1))) {exp} else {unbox(e1)}
        },
        Deref @ SUBSCRIPTED_EXP { exp: __exp_exp, split: __exp_split, subscripts: __exp_subscripts, ty: __exp_ty } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
            (e1, arg) = mapFold(__exp_exp.clone(), func.clone(), arg)?;
            (subs, arg) = List::mapFold(metamodelica::AsArg::as_arg(&__exp_subscripts), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, _) -> Result<_> + 'static> = func.clone(); move |__pe_a0, __pe_a2| Subscript::mapFoldExp(__pe_a0, __pe_b1.clone(), __pe_a2) }), arg)?;
            metamodelica::Ref::new(NFExpression::SUBSCRIPTED_EXP { exp: e1, subscripts: subs, ty: __exp_ty.clone(), split: __exp_split.clone() })
        },
        Deref @ TUPLE_ELEMENT { index: __exp_index, tupleExp: __exp_tupleExp, ty: __exp_ty } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            (e1, arg) = mapFold(__exp_tupleExp.clone(), func.clone(), arg)?;
            if (referenceEq(&*(__exp_tupleExp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::TUPLE_ELEMENT { tupleExp: e1, index: __exp_index.clone(), ty: __exp_ty.clone() })}
        },
        Deref @ RECORD_ELEMENT { fieldName: __exp_fieldName, index: __exp_index, recordExp: __exp_recordExp, ty: __exp_ty } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            (e1, arg) = mapFold(__exp_recordExp.clone(), func.clone(), arg)?;
            if (referenceEq(&*(__exp_recordExp.clone()),&*(&*e1))) {exp} else {metamodelica::Ref::new(NFExpression::RECORD_ELEMENT { recordExp: e1, index: __exp_index.clone(), fieldName: __exp_fieldName.clone(), ty: __exp_ty.clone() })}
        },
        Deref @ MUTABLE { exp: __exp_exp } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            (e1, arg) = mapFold(Mutable::access(__exp_exp.clone()), func.clone(), arg)?;
            Mutable::update(__exp_exp.clone(), e1);
            exp
        },
        Deref @ SHARED_LITERAL { exp: __exp_exp, .. } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            (e1, arg) = mapFold(__exp_exp.clone(), func.clone(), arg)?;
            assign_variant_field!(exp => NFExpression::SHARED_LITERAL; exp = e1);
            exp
        },
        Deref @ PARTIAL_FUNCTION_APPLICATION { args: __exp_args, .. } => {
            let mut expl: metamodelica::List<metamodelica::Ref<NFExpression>>;
            (expl, arg) = List::map1Fold(metamodelica::AsArg::as_arg(&__exp_args), &mapFold, func.clone(), arg)?;
            assign_variant_field!(exp => NFExpression::PARTIAL_FUNCTION_APPLICATION; args = expl);
            exp
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, arg) = func(outExp, arg)?;
    Ok((outExp, arg))
}

pub(crate) fn mapFoldOpt<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut exp: Option<metamodelica::Ref<NFExpression>>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, ArgT) -> Result<(metamodelica::Ref<NFExpression>, ArgT)>
            + 'static,
    >,
    mut arg: ArgT,
) -> Result<(Option<metamodelica::Ref<NFExpression>>, ArgT)> {
    pub type MapFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, ArgT) -> Result<(metamodelica::Ref<NFExpression>, ArgT)>
            + 'static,
    >;

    let mut outExp: Option<metamodelica::Ref<NFExpression>>;
    let mut arg: ArgT = arg;
    let mut e: metamodelica::Ref<NFExpression>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Some(__esc_e) => {
            e = (*__esc_e).clone();
            (e, arg) = mapFold(e.clone(), func.clone(), arg)?;
            Some(e.clone())
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, arg))
}

pub fn mapFoldShallow<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut exp: metamodelica::Ref<NFExpression>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, ArgT) -> Result<(metamodelica::Ref<NFExpression>, ArgT)>
            + 'static,
    >,
    mut arg: ArgT,
) -> Result<(metamodelica::Ref<NFExpression>, ArgT)> {
    pub type MapFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, ArgT) -> Result<(metamodelica::Ref<NFExpression>, ArgT)>
            + 'static,
    >;

    let mut outExp: metamodelica::Ref<NFExpression>;
    let mut arg: ArgT = arg;
    outExp = (match &*exp {
        CLKCONST { clk: __exp_clk } => {
            let mut ck: metamodelica::Ref<ClockKind::NFClockKind>;
            (ck, arg) = ClockKind::mapFoldExpShallow(__exp_clk.clone(), func.clone(), arg)?;
            if (referenceEq(&*(__exp_clk.clone()), &*(&*ck))) {
                exp
            } else {
                metamodelica::Ref::new(NFExpression::CLKCONST { clk: ck })
            }
        }
        CREF {
            cref: __exp_cref,
            ty: __exp_ty,
        } => {
            let mut cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
            (cr, arg) = ComponentRef::mapFoldExpShallow(metamodelica::AsArg::as_arg(&__exp_cref), func.clone(), arg)?;
            if (referenceEq(&*(__exp_cref.clone()), &*(&*cr))) {
                exp
            } else {
                metamodelica::Ref::new(NFExpression::CREF {
                    ty: __exp_ty.clone(),
                    cref: cr,
                })
            }
        }
        ARRAY {
            literal: __exp_literal,
            ty: __exp_ty,
            ..
        } => {
            let mut arr: metamodelica::Array<metamodelica::Ref<NFExpression>>;
            (arr, arg) = Array::mapFold(var_field!((*exp).elements, NFExpression::ARRAY).clone(), &*func, arg)?;
            makeArray(__exp_ty.clone(), arr.clone(), __exp_literal.clone())
        }
        MATRIX {
            elements: __exp_elements,
        } => {
            let mut mat: metamodelica::List<metamodelica::List<metamodelica::Ref<NFExpression>>>;
            (mat, arg) = List::mapFoldList(metamodelica::AsArg::as_arg(&__exp_elements), &*func, arg)?;
            metamodelica::Ref::new(NFExpression::MATRIX { elements: mat })
        }
        RANGE {
            step: oe,
            start: __exp_start,
            stop: __exp_stop,
            ty: __exp_ty,
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e3: metamodelica::Ref<NFExpression>;
            let mut oe = (*oe).clone();
            (e1, arg) = func(__exp_start.clone(), arg)?;
            (oe, arg) = mapFoldOptShallow(var_field!((*exp).step, NFExpression::RANGE).clone(), &*func, arg)?;
            (e3, arg) = func(__exp_stop.clone(), arg)?;
            if (referenceEq(&*(&*e1), &*(__exp_start.clone()))
                && (match (&(oe), &(var_field!((*exp).step, NFExpression::RANGE))) {
                    (None, None) => true,
                    (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l), &*(*__refeq_r)),
                    _ => false,
                })
                && referenceEq(&*(&*e3), &*(__exp_stop.clone())))
            {
                exp
            } else {
                metamodelica::Ref::new(NFExpression::RANGE {
                    ty: __exp_ty.clone(),
                    start: e1,
                    step: oe.clone(),
                    stop: e3,
                })
            }
        }
        TUPLE {
            elements: __exp_elements,
            ty: __exp_ty,
        } => {
            let mut expl: metamodelica::List<metamodelica::Ref<NFExpression>>;
            (expl, arg) = List::mapFold(metamodelica::AsArg::as_arg(&__exp_elements), &*func, arg)?;
            metamodelica::Ref::new(NFExpression::TUPLE {
                ty: __exp_ty.clone(),
                elements: expl,
            })
        }
        RECORD {
            elements: __exp_elements,
            path: __exp_path,
            ty: __exp_ty,
        } => {
            let mut expl: metamodelica::List<metamodelica::Ref<NFExpression>>;
            (expl, arg) = List::mapFold(metamodelica::AsArg::as_arg(&__exp_elements), &*func, arg)?;
            metamodelica::Ref::new(NFExpression::RECORD {
                path: __exp_path.clone(),
                ty: __exp_ty.clone(),
                elements: expl,
            })
        }
        CALL { call: __exp_call } => {
            let mut call: metamodelica::Ref<Call::NFCall>;
            (call, arg) = Call::mapFoldExpShallow(metamodelica::AsArg::as_arg(&__exp_call), &*func, arg)?;
            if (referenceEq(&*(__exp_call.clone()), &*(&*call))) {
                exp
            } else {
                metamodelica::Ref::new(NFExpression::CALL { call: call })
            }
        }
        SIZE {
            dimIndex: __exp_dimIndex,
            exp: __exp_exp,
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut oe: Option<metamodelica::Ref<NFExpression>>;
            (e1, arg) = func(__exp_exp.clone(), arg)?;
            (oe, arg) = mapFoldOptShallow(__exp_dimIndex.clone(), &*func, arg)?;
            if (referenceEq(&*(__exp_exp.clone()), &*(&*e1))
                && (match (&(__exp_dimIndex), &(oe)) {
                    (None, None) => true,
                    (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l), &*(*__refeq_r)),
                    _ => false,
                }))
            {
                exp
            } else {
                metamodelica::Ref::new(NFExpression::SIZE { exp: e1, dimIndex: oe })
            }
        }
        BINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            operator: __exp_operator,
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            (e1, arg) = func(__exp_exp1.clone(), arg)?;
            (e2, arg) = func(__exp_exp2.clone(), arg)?;
            if (referenceEq(&*(__exp_exp1.clone()), &*(&*e1)) && referenceEq(&*(__exp_exp2.clone()), &*(&*e2))) {
                exp
            } else {
                metamodelica::Ref::new(NFExpression::BINARY {
                    exp1: e1,
                    operator: __exp_operator.clone(),
                    exp2: e2,
                })
            }
        }
        MULTARY {
            arguments: __exp_arguments,
            ..
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut expl: metamodelica::List<metamodelica::Ref<NFExpression>>;
            expl = metamodelica::nil();
            for mut argument in &*__exp_arguments.clone() {
                (e1, arg) = func(argument.clone(), arg)?;
                expl = metamodelica::cons(e1, expl);
            }
            assign_variant_field!(exp => NFExpression::MULTARY; arguments = expl.reverse());
            expl = metamodelica::nil();
            for mut argument in &*var_field!((*exp).inv_arguments, NFExpression::MULTARY).clone() {
                (e1, arg) = func(argument.clone(), arg)?;
                expl = metamodelica::cons(e1, expl);
            }
            assign_variant_field!(exp => NFExpression::MULTARY; inv_arguments = expl.reverse());
            exp
        }
        UNARY {
            exp: __exp_exp,
            operator: __exp_operator,
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            (e1, arg) = func(__exp_exp.clone(), arg)?;
            if (referenceEq(&*(__exp_exp.clone()), &*(&*e1))) {
                exp
            } else {
                metamodelica::Ref::new(NFExpression::UNARY {
                    operator: __exp_operator.clone(),
                    exp: e1,
                })
            }
        }
        LBINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            operator: __exp_operator,
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            (e1, arg) = func(__exp_exp1.clone(), arg)?;
            (e2, arg) = func(__exp_exp2.clone(), arg)?;
            if (referenceEq(&*(__exp_exp1.clone()), &*(&*e1)) && referenceEq(&*(__exp_exp2.clone()), &*(&*e2))) {
                exp
            } else {
                metamodelica::Ref::new(NFExpression::LBINARY {
                    exp1: e1,
                    operator: __exp_operator.clone(),
                    exp2: e2,
                })
            }
        }
        LUNARY {
            exp: __exp_exp,
            operator: __exp_operator,
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            (e1, arg) = func(__exp_exp.clone(), arg)?;
            if (referenceEq(&*(__exp_exp.clone()), &*(&*e1))) {
                exp
            } else {
                metamodelica::Ref::new(NFExpression::LUNARY {
                    operator: __exp_operator.clone(),
                    exp: e1,
                })
            }
        }
        RELATION {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            index: __exp_index,
            operator: __exp_operator,
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            (e1, arg) = func(__exp_exp1.clone(), arg)?;
            (e2, arg) = func(__exp_exp2.clone(), arg)?;
            if (referenceEq(&*(__exp_exp1.clone()), &*(&*e1)) && referenceEq(&*(__exp_exp2.clone()), &*(&*e2))) {
                exp
            } else {
                metamodelica::Ref::new(NFExpression::RELATION {
                    exp1: e1,
                    operator: __exp_operator.clone(),
                    exp2: e2,
                    index: __exp_index.clone(),
                })
            }
        }
        IF {
            condition: __exp_condition,
            falseBranch: __exp_falseBranch,
            trueBranch: __exp_trueBranch,
            ty: __exp_ty,
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut e2: metamodelica::Ref<NFExpression>;
            let mut e3: metamodelica::Ref<NFExpression>;
            (e1, arg) = func(__exp_condition.clone(), arg)?;
            (e2, arg) = func(__exp_trueBranch.clone(), arg)?;
            (e3, arg) = func(__exp_falseBranch.clone(), arg)?;
            if (referenceEq(&*(__exp_condition.clone()), &*(&*e1))
                && referenceEq(&*(__exp_trueBranch.clone()), &*(&*e2))
                && referenceEq(&*(__exp_falseBranch.clone()), &*(&*e3)))
            {
                exp
            } else {
                metamodelica::Ref::new(NFExpression::IF {
                    ty: __exp_ty.clone(),
                    condition: e1,
                    trueBranch: e2,
                    falseBranch: e3,
                })
            }
        }
        CAST {
            exp: __exp_exp,
            ty: __exp_ty,
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            (e1, arg) = func(__exp_exp.clone(), arg)?;
            if (referenceEq(&*(__exp_exp.clone()), &*(&*e1))) {
                exp
            } else {
                metamodelica::Ref::new(NFExpression::CAST {
                    ty: __exp_ty.clone(),
                    exp: e1,
                })
            }
        }
        BOX { exp: __exp_exp } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            (e1, arg) = func(__exp_exp.clone(), arg)?;
            if (referenceEq(&*(__exp_exp.clone()), &*(&*e1))) {
                exp
            } else {
                r#box(&e1)
            }
        }
        UNBOX { exp: __exp_exp, .. } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            (e1, arg) = func(__exp_exp.clone(), arg)?;
            if (referenceEq(&*(__exp_exp.clone()), &*(&*e1))) {
                exp
            } else {
                unbox(e1)
            }
        }
        SUBSCRIPTED_EXP {
            exp: __exp_exp,
            split: __exp_split,
            subscripts: __exp_subscripts,
            ty: __exp_ty,
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
            (e1, arg) = func(__exp_exp.clone(), arg)?;
            (subs, arg) = List::mapFold(
                metamodelica::AsArg::as_arg(&__exp_subscripts),
                &({
                    let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, _) -> Result<_> + 'static> =
                        func.clone();
                    move |__pe_a0, __pe_a2| Subscript::mapFoldExpShallow(__pe_a0, &*__pe_b1, __pe_a2)
                }),
                arg,
            )?;
            metamodelica::Ref::new(NFExpression::SUBSCRIPTED_EXP {
                exp: e1,
                subscripts: subs,
                ty: __exp_ty.clone(),
                split: __exp_split.clone(),
            })
        }
        TUPLE_ELEMENT {
            index: __exp_index,
            tupleExp: __exp_tupleExp,
            ty: __exp_ty,
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            (e1, arg) = func(__exp_tupleExp.clone(), arg)?;
            if (referenceEq(&*(__exp_tupleExp.clone()), &*(&*e1))) {
                exp
            } else {
                metamodelica::Ref::new(NFExpression::TUPLE_ELEMENT {
                    tupleExp: e1,
                    index: __exp_index.clone(),
                    ty: __exp_ty.clone(),
                })
            }
        }
        RECORD_ELEMENT {
            fieldName: __exp_fieldName,
            index: __exp_index,
            recordExp: __exp_recordExp,
            ty: __exp_ty,
        } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            (e1, arg) = func(__exp_recordExp.clone(), arg)?;
            if (referenceEq(&*(__exp_recordExp.clone()), &*(&*e1))) {
                exp
            } else {
                metamodelica::Ref::new(NFExpression::RECORD_ELEMENT {
                    recordExp: e1,
                    index: __exp_index.clone(),
                    fieldName: __exp_fieldName.clone(),
                    ty: __exp_ty.clone(),
                })
            }
        }
        MUTABLE { exp: __exp_exp } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            (e1, arg) = func(Mutable::access(__exp_exp.clone()), arg)?;
            Mutable::update(__exp_exp.clone(), e1);
            exp
        }
        SHARED_LITERAL { exp: __exp_exp, .. } => {
            let mut e1: metamodelica::Ref<NFExpression>;
            (e1, arg) = func(__exp_exp.clone(), arg)?;
            assign_variant_field!(exp => NFExpression::SHARED_LITERAL; exp = e1);
            exp
        }
        PARTIAL_FUNCTION_APPLICATION { args: __exp_args, .. } => {
            let mut expl: metamodelica::List<metamodelica::Ref<NFExpression>>;
            (expl, arg) = List::mapFold(metamodelica::AsArg::as_arg(&__exp_args), &*func, arg)?;
            assign_variant_field!(exp => NFExpression::PARTIAL_FUNCTION_APPLICATION; args = expl);
            exp
        }
        _ => exp,
    });
    Ok((outExp, arg))
}

pub(crate) fn mapFoldOptShallow<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut exp: Option<metamodelica::Ref<NFExpression>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, ArgT) -> Result<(metamodelica::Ref<NFExpression>, ArgT)>,
    mut arg: ArgT,
) -> Result<(Option<metamodelica::Ref<NFExpression>>, ArgT)> {
    pub type MapFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, ArgT) -> Result<(metamodelica::Ref<NFExpression>, ArgT)>
            + 'static,
    >;

    let mut outExp: Option<metamodelica::Ref<NFExpression>>;
    let mut arg: ArgT = arg;
    let mut e1: metamodelica::Ref<NFExpression>;
    let mut e2: metamodelica::Ref<NFExpression>;
    outExp = (::match_deref::match_deref! { match &(exp.clone()) {
        Some(__esc_e1) => {
            e1 = (*__esc_e1).clone();
            (e2, arg) = func(e1.clone(), arg)?;
            if (referenceEq(&*(e1.clone()),&*(&*e2))) {exp} else {Some(e2)}
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, arg))
}

pub(crate) fn containsOpt(
    mut exp: Option<metamodelica::Ref<NFExpression>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<bool>,
) -> Result<bool> {
    pub type ContainsPred =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    let mut e: metamodelica::Ref<NFExpression>;
    res = (::match_deref::match_deref! { match &(exp) {
        Some(__esc_e) => {
            e = (*__esc_e).clone();
            contains(e.clone(), func)?
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn contains<'__b>(
    mut exp: metamodelica::Ref<NFExpression>,
    mut func: &'__b dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<bool>,
) -> Result<bool> {
    pub type ContainsPred =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    if func(exp.clone())? {
        res = true;
        return Ok(res);
    }
    res = (match &*exp {
        CLKCONST { clk: __exp_clk } => ClockKind::containsExp(metamodelica::AsArg::as_arg(&__exp_clk), func)?,
        CREF { cref: __exp_cref, .. } => ComponentRef::containsExp(metamodelica::AsArg::as_arg(&__exp_cref), func)?,
        ARRAY { .. } => arrayContains(var_field!((*exp).elements, NFExpression::ARRAY).clone(), func)?,
        MATRIX {
            elements: __exp_elements,
        } => {
            res = false;
            for mut row in &*__exp_elements.clone() {
                if listContains(metamodelica::AsArg::as_arg(&row), func)? {
                    res = true;
                    break;
                }
            }
            res
        }
        RANGE {
            start: __exp_start,
            step: __exp_step,
            stop: __exp_stop,
            ..
        } => {
            contains(__exp_start.clone(), func)?
                || containsOpt(__exp_step.clone(), func)?
                || contains(__exp_stop.clone(), func)?
        }
        TUPLE {
            elements: __exp_elements,
            ..
        } => listContains(metamodelica::AsArg::as_arg(&__exp_elements), func)?,
        RECORD {
            elements: __exp_elements,
            ..
        } => listContains(metamodelica::AsArg::as_arg(&__exp_elements), func)?,
        CALL { call: __exp_call } => Call::containsExp(metamodelica::AsArg::as_arg(&__exp_call), func)?,
        SIZE {
            dimIndex: __exp_dimIndex,
            exp: __exp_exp,
        } => containsOpt(__exp_dimIndex.clone(), func)? || contains(__exp_exp.clone(), func)?,
        BINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            ..
        } => contains(__exp_exp1.clone(), func)? || contains(__exp_exp2.clone(), func)?,
        MULTARY {
            arguments: __exp_arguments,
            inv_arguments: __exp_inv_arguments,
            ..
        } => {
            res = false;
            for mut arg in &*__exp_arguments.clone() {
                if res {
                    break;
                }
                res = contains(arg.clone(), func)?;
            }
            for mut arg in &*__exp_inv_arguments.clone() {
                if res {
                    break;
                }
                res = contains(arg.clone(), func)?;
            }
            res
        }
        UNARY { exp: __exp_exp, .. } => contains(__exp_exp.clone(), func)?,
        LBINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            ..
        } => contains(__exp_exp1.clone(), func)? || contains(__exp_exp2.clone(), func)?,
        LUNARY { exp: __exp_exp, .. } => contains(__exp_exp.clone(), func)?,
        RELATION {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            ..
        } => contains(__exp_exp1.clone(), func)? || contains(__exp_exp2.clone(), func)?,
        IF {
            condition: __exp_condition,
            falseBranch: __exp_falseBranch,
            trueBranch: __exp_trueBranch,
            ..
        } => {
            contains(__exp_condition.clone(), func)?
                || contains(__exp_trueBranch.clone(), func)?
                || contains(__exp_falseBranch.clone(), func)?
        }
        CAST { exp: __exp_exp, .. } => contains(__exp_exp.clone(), func)?,
        BOX { exp: __exp_exp } => contains(__exp_exp.clone(), func)?,
        UNBOX { exp: __exp_exp, .. } => contains(__exp_exp.clone(), func)?,
        SUBSCRIPTED_EXP {
            exp: __exp_exp,
            subscripts: __exp_subscripts,
            ..
        } => {
            contains(__exp_exp.clone(), func)?
                || Subscript::listContainsExp(metamodelica::AsArg::as_arg(&__exp_subscripts), func)?
        }
        TUPLE_ELEMENT {
            tupleExp: __exp_tupleExp,
            ..
        } => contains(__exp_tupleExp.clone(), func)?,
        RECORD_ELEMENT {
            recordExp: __exp_recordExp,
            ..
        } => contains(__exp_recordExp.clone(), func)?,
        MUTABLE { exp: __exp_exp } => contains(Mutable::access(__exp_exp.clone()), func)?,
        SHARED_LITERAL { exp: __exp_exp, .. } => contains(__exp_exp.clone(), func)?,
        PARTIAL_FUNCTION_APPLICATION { args: __exp_args, .. } => {
            listContains(metamodelica::AsArg::as_arg(&__exp_args), func)?
        }
        _ => false,
    });
    Ok(res)
}

pub(crate) fn arrayContains(
    mut expl: metamodelica::Array<metamodelica::Ref<NFExpression>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<bool>,
) -> Result<bool> {
    pub type ContainsPred =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    let __range0 = expl.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        if contains(e, func)? {
            res = true;
            return Ok(res);
        }
    }
    res = false;
    Ok(res)
}

pub(crate) fn listContains(
    mut expl: &metamodelica::List<metamodelica::Ref<NFExpression>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<bool>,
) -> Result<bool> {
    pub type ContainsPred =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    for mut e in &**expl {
        if contains(e.clone(), func)? {
            res = true;
            return Ok(res);
        }
    }
    res = false;
    Ok(res)
}

pub(crate) fn containsShallow(
    mut exp: &metamodelica::Ref<NFExpression>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<bool>,
) -> Result<bool> {
    pub type ContainsPred =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<bool> + 'static>;

    let mut res: bool;
    res = (match &**exp {
        CLKCONST { clk: __exp_clk } => ClockKind::containsExpShallow(metamodelica::AsArg::as_arg(&__exp_clk), func)?,
        CREF { cref: __exp_cref, .. } => {
            ComponentRef::containsExpShallow(metamodelica::AsArg::as_arg(&__exp_cref), func)?
        }
        ARRAY { .. } => Array::any(var_field!((**exp).elements, NFExpression::ARRAY).clone(), func)?,
        MATRIX {
            elements: __exp_elements,
        } => {
            res = false;
            for mut row in &*__exp_elements.clone() {
                if List::any(metamodelica::AsArg::as_arg(&row), func)? {
                    res = true;
                    break;
                }
            }
            res
        }
        RANGE {
            start: __exp_start,
            step: __exp_step,
            stop: __exp_stop,
            ..
        } => {
            func(__exp_start.clone())?
                || Util::applyOptionOrDefault(__exp_step.clone(), func, false)?
                || func(__exp_stop.clone())?
        }
        TUPLE {
            elements: __exp_elements,
            ..
        } => List::any(metamodelica::AsArg::as_arg(&__exp_elements), func)?,
        RECORD {
            elements: __exp_elements,
            ..
        } => List::any(metamodelica::AsArg::as_arg(&__exp_elements), func)?,
        CALL { call: __exp_call } => Call::containsExpShallow(metamodelica::AsArg::as_arg(&__exp_call), func)?,
        SIZE {
            dimIndex: __exp_dimIndex,
            exp: __exp_exp,
        } => Util::applyOptionOrDefault(__exp_dimIndex.clone(), func, false)? || func(__exp_exp.clone())?,
        BINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            ..
        } => func(__exp_exp1.clone())? || func(__exp_exp2.clone())?,
        MULTARY {
            arguments: __exp_arguments,
            inv_arguments: __exp_inv_arguments,
            ..
        } => {
            res = false;
            for mut arg in &*__exp_arguments.clone() {
                if res {
                    break;
                }
                res = func(arg.clone())?;
            }
            for mut arg in &*__exp_inv_arguments.clone() {
                if res {
                    break;
                }
                res = func(arg.clone())?;
            }
            res
        }
        UNARY { exp: __exp_exp, .. } => func(__exp_exp.clone())?,
        LBINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            ..
        } => func(__exp_exp1.clone())? || func(__exp_exp2.clone())?,
        LUNARY { exp: __exp_exp, .. } => func(__exp_exp.clone())?,
        RELATION {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            ..
        } => func(__exp_exp1.clone())? || func(__exp_exp2.clone())?,
        IF {
            condition: __exp_condition,
            falseBranch: __exp_falseBranch,
            trueBranch: __exp_trueBranch,
            ..
        } => func(__exp_condition.clone())? || func(__exp_trueBranch.clone())? || func(__exp_falseBranch.clone())?,
        CAST { exp: __exp_exp, .. } => func(__exp_exp.clone())?,
        BOX { exp: __exp_exp } => func(__exp_exp.clone())?,
        UNBOX { exp: __exp_exp, .. } => func(__exp_exp.clone())?,
        SUBSCRIPTED_EXP {
            exp: __exp_exp,
            subscripts: __exp_subscripts,
            ..
        } => {
            func(__exp_exp.clone())?
                || Subscript::listContainsExpShallow(metamodelica::AsArg::as_arg(&__exp_subscripts), func)?
        }
        TUPLE_ELEMENT {
            tupleExp: __exp_tupleExp,
            ..
        } => func(__exp_tupleExp.clone())?,
        RECORD_ELEMENT {
            recordExp: __exp_recordExp,
            ..
        } => func(__exp_recordExp.clone())?,
        MUTABLE { exp: __exp_exp } => func(Mutable::access(__exp_exp.clone()))?,
        SHARED_LITERAL { exp: __exp_exp, .. } => func(__exp_exp.clone())?,
        PARTIAL_FUNCTION_APPLICATION { args: __exp_args, .. } => {
            listContains(metamodelica::AsArg::as_arg(&__exp_args), func)?
        }
        _ => false,
    });
    Ok(res)
}

pub(crate) fn arrayFirstScalar(
    mut arrayExp: metamodelica::Ref<NFExpression>,
) -> Result<metamodelica::Ref<NFExpression>> {
    '__tco: loop {
        match &*arrayExp {
            ARRAY { .. } => {
                arrayExp = metamodelica::arrayGet(var_field!((*arrayExp).elements, NFExpression::ARRAY).clone(), 1)?;
                continue '__tco;
            }
            _ => return Ok(arrayExp),
        }
    }
}

pub(crate) fn arrayAllEqual(mut arrayExp: metamodelica::Ref<NFExpression>) -> bool {
    let mut allEqual: bool;
    allEqual = 'mc: {
        let __mc_input = &*arrayExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ ARRAY { .. } => {
                    Ok(arrayAllEqual2(&(arrayExp.clone()), &(arrayFirstScalar(arrayExp.clone())?))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    allEqual
}

pub(crate) fn arrayAllEqual2(
    mut arrayExp: &metamodelica::Ref<NFExpression>,
    mut element: &metamodelica::Ref<NFExpression>,
) -> Result<bool> {
    let mut allEqual: bool;
    allEqual = (match &**arrayExp {
        ARRAY { .. }
            if (!(var_field!((**arrayExp).elements, NFExpression::ARRAY)
                .clone()
                .borrow()
                .is_empty())
                && isArray(
                    &(metamodelica::arrayGet(var_field!((**arrayExp).elements, NFExpression::ARRAY).clone(), 1)?),
                )) =>
        {
            Array::all(
                var_field!((**arrayExp).elements, NFExpression::ARRAY).clone(),
                &({
                    let __pe_b1 = element.clone();
                    move |__pe_a0| arrayAllEqual2(&__pe_a0, &__pe_b1)
                }),
            )?
        }
        ARRAY { .. } => Array::all(
            var_field!((**arrayExp).elements, NFExpression::ARRAY).clone(),
            &({
                let __pe_b1 = element.clone();
                move |__pe_a0| isEqual(__pe_a0, __pe_b1.clone())
            }),
        )?,
        _ => true,
    });
    Ok(allEqual)
}

pub fn fromCref(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut includeScope: bool,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression>;
    exp = metamodelica::Ref::new(NFExpression::CREF {
        ty: ComponentRef::getSubscriptedType(&cref, includeScope)?,
        cref: cref,
    });
    Ok(exp)
}

pub(crate) fn fromTypedCref(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> metamodelica::Ref<NFExpression> {
    let mut exp: metamodelica::Ref<NFExpression> = metamodelica::Ref::new(NFExpression::CREF {
        ty: ty.clone(),
        cref: cref.clone(),
    });
    exp
}

pub fn toCref(mut exp: &metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match &((*exp)) {
        Deref @ CREF { cref: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cref = metamodelica::Own::own(__pa0);
    Ok(cref)
}

pub fn extractCrefs(
    mut exp: metamodelica::Ref<NFExpression>,
) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>> {
    let mut crefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
        fold(
            exp.clone(),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<NFExpression>,
                      __a1: metamodelica::Ref<
                    UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                >| extractCref(&__a0, __a1),
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<NFExpression>,
                            metamodelica::Ref<
                                UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                            >,
                        ) -> Result<
                            metamodelica::Ref<
                                UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                            >,
                        > + 'static,
                >),
            UnorderedSet::new(
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
                13,
            ),
        )?;
    Ok(crefs)
}

pub(crate) fn extractCref(
    mut exp: &metamodelica::Ref<NFExpression>,
    mut crefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>> {
    let mut crefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
        crefs;
    crefs = (match &**exp {
        CREF { cref: __exp_cref, .. } => {
            UnorderedSet::add(__exp_cref.clone(), crefs.clone())?;
            crefs
        }
        _ => crefs,
    });
    Ok(crefs)
}

pub(crate) fn isResizableCref(mut exp: &metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut b: bool;
    b = (match &**exp {
        CREF { cref: __exp_cref, .. } => ComponentRef::isResizable(metamodelica::AsArg::as_arg(&__exp_cref))?,
        _ => false,
    });
    Ok(b)
}

pub(crate) fn isIterator(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut isIterator: bool;
    isIterator = (match &**exp {
        CREF { cref: __exp_cref, .. } => ComponentRef::isIterator(metamodelica::AsArg::as_arg(&__exp_cref)),
        _ => false,
    });
    isIterator
}

pub(crate) fn containsAnyIterator(mut exp: metamodelica::Ref<NFExpression>, mut context: i32) -> Result<bool> {
    let mut iter: bool;
    if InstContext::inFor(context) {
        iter = contains(
            exp,
            &move |__a0: metamodelica::Ref<NFExpression>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isIterator(&__a0))
            },
        )?;
    } else {
        iter = false;
    }
    Ok(iter)
}

pub fn isTime(mut exp: &metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut b: bool;
    b = (match &**exp {
        CREF { cref: __exp_cref, .. } => ComponentRef::isTime(metamodelica::AsArg::as_arg(&__exp_cref))?,
        _ => false,
    });
    Ok(b)
}

pub fn isSubstitute(mut exp: &metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut b: bool;
    b = (match &**exp {
        CREF { cref: __exp_cref, .. } => ComponentRef::isSubstitute(metamodelica::AsArg::as_arg(&__exp_cref))?,
        _ => false,
    });
    Ok(b)
}

pub fn isZero<'__b>(mut exp: &'__b metamodelica::Ref<NFExpression>) -> Result<bool> {
    '__tco: loop {
        match &**exp {
            INTEGER { .. } => return Ok(var_field!((**exp).value, NFExpression::INTEGER).clone() == 0),
            REAL { .. } => {
                return Ok(var_field!((**exp).value, NFExpression::REAL).clone() == metamodelica::OrderedFloat(0.0_f64));
            }
            CAST { .. } => {
                exp = var_field!((**exp).exp, NFExpression::CAST);
                continue '__tco;
            }
            UNARY { .. } => {
                exp = var_field!((**exp).exp, NFExpression::UNARY);
                continue '__tco;
            }
            ARRAY { .. } => {
                return Ok(Array::all(
                    var_field!((**exp).elements, NFExpression::ARRAY).clone(),
                    &move |__a0: metamodelica::Ref<NFExpression>| isZero(&__a0),
                )?);
            }
            _ => return Ok(false),
        }
    }
}

pub(crate) fn isNonZero(mut exp: &metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut res: bool = isPositive(exp)? || isNegative(exp)?;
    Ok(res)
}

pub fn isOne<'__b>(mut exp: &'__b metamodelica::Ref<NFExpression>) -> Result<bool> {
    '__tco: loop {
        match &**exp {
            INTEGER { .. } => return Ok(var_field!((**exp).value, NFExpression::INTEGER).clone() == 1),
            REAL { .. } => {
                return Ok(var_field!((**exp).value, NFExpression::REAL).clone() == metamodelica::OrderedFloat(1.0_f64));
            }
            CAST { .. } => {
                exp = var_field!((**exp).exp, NFExpression::CAST);
                continue '__tco;
            }
            UNARY { .. } => return Ok(isMinusOne(var_field!((**exp).exp, NFExpression::UNARY))?),
            ARRAY { .. } => {
                return Ok(Array::all(
                    var_field!((**exp).elements, NFExpression::ARRAY).clone(),
                    &move |__a0: metamodelica::Ref<NFExpression>| isOne(&__a0),
                )?);
            }
            _ => return Ok(false),
        }
    }
}

pub(crate) fn isMinusOne<'__b>(mut exp: &'__b metamodelica::Ref<NFExpression>) -> Result<bool> {
    '__tco: loop {
        match &**exp {
            INTEGER { .. } => return Ok(var_field!((**exp).value, NFExpression::INTEGER).clone() == -1),
            REAL { .. } => {
                return Ok(
                    var_field!((**exp).value, NFExpression::REAL).clone() == metamodelica::OrderedFloat(-1.0_f64)
                );
            }
            CAST { .. } => {
                exp = var_field!((**exp).exp, NFExpression::CAST);
                continue '__tco;
            }
            UNARY { .. } => return Ok(self::isOne(var_field!((**exp).exp, NFExpression::UNARY))?),
            _ => return Ok(false),
        }
    }
}

pub(crate) fn isNaN(mut nan: &metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut b: bool;
    b = (match &**nan {
        BINARY {
            exp1: __nan_exp1,
            exp2: __nan_exp2,
            operator: __nan_operator,
        } => {
            Operator::getMathClassification(metamodelica::AsArg::as_arg(&__nan_operator))?
                == Operator::MathClassification::DIVISION.clone()
                && isZero(metamodelica::AsArg::as_arg(&__nan_exp1))?
                && isZero(metamodelica::AsArg::as_arg(&__nan_exp2))?
        }
        _ => false,
    });
    Ok(b)
}

pub fn isPositive<'__b>(mut exp: &'__b metamodelica::Ref<NFExpression>) -> Result<bool> {
    '__tco: loop {
        match &**exp {
            INTEGER { .. } => return Ok(var_field!((**exp).value, NFExpression::INTEGER).clone() > 0),
            REAL { .. } => {
                return Ok(var_field!((**exp).value, NFExpression::REAL).clone() > metamodelica::OrderedFloat(0.0_f64));
            }
            CAST { .. } => {
                exp = var_field!((**exp).exp, NFExpression::CAST);
                continue '__tco;
            }
            UNARY { .. } => return Ok(isNegative(var_field!((**exp).exp, NFExpression::UNARY))?),
            CREF { .. } => {
                return Ok(Util::applyOptionOrDefault(
                    ComponentRef::lookupVarAttr(var_field!((**exp).cref, NFExpression::CREF), &(literal!("min")))?,
                    &move |__a0: metamodelica::Ref<NFExpression>| isPositive(&__a0),
                    false,
                )?);
            }
            CALL { .. } => return Ok(Call::isPositive(var_field!((**exp).call, NFExpression::CALL))?),
            _ => return Ok(false),
        }
    }
}

pub fn isNegative<'__b>(mut exp: &'__b metamodelica::Ref<NFExpression>) -> Result<bool> {
    '__tco: loop {
        match &**exp {
            INTEGER { .. } => return Ok(var_field!((**exp).value, NFExpression::INTEGER).clone() < 0),
            REAL { .. } => {
                return Ok(var_field!((**exp).value, NFExpression::REAL).clone() < metamodelica::OrderedFloat(0.0_f64));
            }
            CAST { .. } => {
                exp = var_field!((**exp).exp, NFExpression::CAST);
                continue '__tco;
            }
            UNARY { .. } => return Ok(isPositive(var_field!((**exp).exp, NFExpression::UNARY))?),
            CREF { .. } => {
                return Ok(Util::applyOptionOrDefault(
                    ComponentRef::lookupVarAttr(var_field!((**exp).cref, NFExpression::CREF), &(literal!("max")))?,
                    &move |__a0: metamodelica::Ref<NFExpression>| isNegative(&__a0),
                    false,
                )?);
            }
            CALL { .. } => return Ok(Call::isNegative(var_field!((**exp).call, NFExpression::CALL))?),
            _ => return Ok(false),
        }
    }
}

pub fn isNonPositive<'__b>(mut exp: &'__b metamodelica::Ref<NFExpression>) -> Result<bool> {
    '__tco: loop {
        match &**exp {
            INTEGER { .. } => return Ok(var_field!((**exp).value, NFExpression::INTEGER).clone() <= 0),
            REAL { .. } => {
                return Ok(var_field!((**exp).value, NFExpression::REAL).clone() <= metamodelica::OrderedFloat(0.0_f64));
            }
            CAST { .. } => {
                exp = var_field!((**exp).exp, NFExpression::CAST);
                continue '__tco;
            }
            UNARY { .. } => return Ok(isNonNegative(var_field!((**exp).exp, NFExpression::UNARY))?),
            CREF { .. } => {
                return Ok(Util::applyOptionOrDefault(
                    ComponentRef::lookupVarAttr(var_field!((**exp).cref, NFExpression::CREF), &(literal!("max")))?,
                    &move |__a0: metamodelica::Ref<NFExpression>| isNonPositive(&__a0),
                    false,
                )?);
            }
            CALL { .. } => return Ok(Call::isNonPositive(var_field!((**exp).call, NFExpression::CALL))?),
            ARRAY { .. } => {
                return Ok(Array::all(
                    var_field!((**exp).elements, NFExpression::ARRAY).clone(),
                    &move |__a0: metamodelica::Ref<NFExpression>| isNonPositive(&__a0),
                )?);
            }
            _ => return Ok(false),
        }
    }
}

pub(crate) fn isNonNegative<'__b>(mut exp: &'__b metamodelica::Ref<NFExpression>) -> Result<bool> {
    '__tco: loop {
        match &**exp {
            INTEGER { .. } => return Ok(var_field!((**exp).value, NFExpression::INTEGER).clone() >= 0),
            REAL { .. } => {
                return Ok(var_field!((**exp).value, NFExpression::REAL).clone() >= metamodelica::OrderedFloat(0.0_f64));
            }
            CAST { .. } => {
                exp = var_field!((**exp).exp, NFExpression::CAST);
                continue '__tco;
            }
            UNARY { .. } => return Ok(isNonPositive(var_field!((**exp).exp, NFExpression::UNARY))?),
            CREF { .. } => {
                return Ok(Util::applyOptionOrDefault(
                    ComponentRef::lookupVarAttr(var_field!((**exp).cref, NFExpression::CREF), &(literal!("min")))?,
                    &move |__a0: metamodelica::Ref<NFExpression>| isNonNegative(&__a0),
                    false,
                )?);
            }
            CALL { .. } => return Ok(Call::isNonNegative(var_field!((**exp).call, NFExpression::CALL))?),
            _ => return Ok(false),
        }
    }
}

pub(crate) fn isEven<'__b>(mut exp: &'__b metamodelica::Ref<NFExpression>) -> bool {
    '__tco: loop {
        match &**exp {
            INTEGER { .. } => return intMod(var_field!((**exp).value, NFExpression::INTEGER).clone(), 2) == 0,
            REAL { .. } => {
                return realMod(
                    var_field!((**exp).value, NFExpression::REAL).clone(),
                    metamodelica::OrderedFloat(2.0_f64),
                ) == metamodelica::OrderedFloat(0.0_f64);
            }
            CAST { .. } => {
                exp = var_field!((**exp).exp, NFExpression::CAST);
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub(crate) fn isGreaterOrEqual(
    mut lhs: metamodelica::Ref<NFExpression>,
    mut rhs: metamodelica::Ref<NFExpression>,
) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((lhs.clone(), rhs.clone())) {
            (Deref @ REAL { .. }, Deref @ REAL { .. }) => return Ok(var_field!((*lhs).value, NFExpression::REAL).clone() >= var_field!((*rhs).value, NFExpression::REAL).clone()),
            (Deref @ CREF { .. }, _) => return Ok(Util::applyOptionOrDefault(ComponentRef::lookupVarAttr(var_field!((*lhs).cref, NFExpression::CREF), &(literal!("min")))?, &({ let __pe_b1 = rhs; move |__pe_a0| isGreaterOrEqual(__pe_a0, __pe_b1.clone()) }), false)?),
            (_, Deref @ CREF { .. }) => return Ok(Util::applyOptionOrDefault(ComponentRef::lookupVarAttr(var_field!((*rhs).cref, NFExpression::CREF), &(literal!("max")))?, &({ let __pe_b0 = lhs; move |__pe_a1| isGreaterOrEqual(__pe_b0.clone(), __pe_a1) }), false)?),
            (Deref @ UNARY { exp: Deref @ CREF { .. }, .. }, _) => { (lhs, rhs) = (negate(rhs), var_field!((*lhs).exp, NFExpression::UNARY).clone()); continue '__tco; },
            (_, Deref @ UNARY { exp: Deref @ CREF { .. }, .. }) => { (lhs, rhs) = (var_field!((*rhs).exp, NFExpression::UNARY).clone(), negate(lhs)); continue '__tco; },
            _ => return Ok(false),
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn hasArrayType(mut exp: metamodelica::Ref<NFExpression>) -> bool {
    let mut b: bool = Type::isArray(&(typeOf(exp.clone())));
    b
}

pub fn isScalar(mut exp: metamodelica::Ref<NFExpression>) -> bool {
    let mut scalar: bool = Type::isScalar(&(typeOf(exp.clone())));
    scalar
}

pub(crate) fn isScalarLiteral(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut literal: bool;
    literal = (match &**exp {
        INTEGER { .. } => true,
        REAL { .. } => true,
        STRING { .. } => true,
        BOOLEAN { .. } => true,
        ENUM_LITERAL { .. } => true,
        FILENAME { .. } => true,
        _ => false,
    });
    literal
}

pub fn isLiteral(mut exp: &metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut literal: bool;
    literal = (match &**exp {
        INTEGER { .. } => true,
        REAL { .. } => true,
        STRING { .. } => true,
        BOOLEAN { .. } => true,
        ENUM_LITERAL { .. } => true,
        ARRAY {
            literal: __exp_literal, ..
        } => {
            __exp_literal.clone()
                || Array::all(
                    var_field!((**exp).elements, NFExpression::ARRAY).clone(),
                    &move |__a0: metamodelica::Ref<NFExpression>| isLiteral(&__a0),
                )?
        }
        RECORD {
            elements: __exp_elements,
            ..
        } => List::all(
            metamodelica::AsArg::as_arg(&__exp_elements),
            &move |__a0: metamodelica::Ref<NFExpression>| isLiteral(&__a0),
        )?,
        RANGE {
            start: __exp_start,
            step: __exp_step,
            stop: __exp_stop,
            ..
        } => {
            isLiteral(metamodelica::AsArg::as_arg(&__exp_start))?
                && isLiteral(metamodelica::AsArg::as_arg(&__exp_stop))?
                && Util::applyOptionOrDefault(
                    __exp_step.clone(),
                    &move |__a0: metamodelica::Ref<NFExpression>| isLiteral(&__a0),
                    true,
                )?
        }
        FILENAME { .. } => true,
        _ => false,
    });
    Ok(literal)
}

pub fn isLiteralXML(mut exp: metamodelica::Ref<NFExpression>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(&*exp) {
            Deref @ INTEGER { .. } => {
                return Ok(true)
            },
            Deref @ REAL { .. } => {
                return Ok(true)
            },
            Deref @ STRING { .. } => {
                return Ok(true)
            },
            Deref @ BOOLEAN { .. } => {
                return Ok(true)
            },
            Deref @ ENUM_LITERAL { .. } => {
                return Ok(true)
            },
            Deref @ ARRAY { .. } => {
                return Ok(Array::all(var_field!((*exp).elements, NFExpression::ARRAY).clone(), &isLiteralXML)?)
            },
            Deref @ RECORD { elements: __exp_elements, .. } => {
                return Ok(List::all(metamodelica::AsArg::as_arg(&__exp_elements), &isLiteralXML)?)
            },
            Deref @ RANGE { start: __exp_start, step: __exp_step, stop: __exp_stop, .. } => {
                return Ok(isLiteralXML(__exp_start.clone())? && isLiteralXML(__exp_stop.clone())? && Util::applyOptionOrDefault(__exp_step.clone(), &isLiteralXML, true)?)
            },
            Deref @ FILENAME { .. } => {
                return Ok(true)
            },
            Deref @ CALL { call: Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { exp: call_exp, .. } } => {
                { exp = call_exp.clone(); continue '__tco; }
            },
            _ => {
                return Ok(false)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn isLiteralReplace(mut exp: &metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match exp {
        Deref @ STRING { .. } => true,
        Deref @ BOX { exp: Deref @ STRING { .. } } => true,
        Deref @ RECORD { .. } => isLiteral(exp)?,
        Deref @ ARRAY { .. } => isLiteral(exp)?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isKnownSizeFill(mut exp: &metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut literal: bool;
    literal = (match &**exp {
        CALL { call: __exp_call } => Call::isKnownSizeFill(metamodelica::AsArg::as_arg(&__exp_call))?,
        _ => false,
    });
    Ok(literal)
}

pub fn isInteger(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut isInteger: bool;
    isInteger = (match &**exp {
        INTEGER { .. } => true,
        _ => false,
    });
    isInteger
}

pub fn isReal(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut isReal: bool;
    isReal = (match &**exp {
        REAL { .. } => true,
        _ => false,
    });
    isReal
}

pub fn isConstNumber<'__b>(mut exp: &'__b metamodelica::Ref<NFExpression>) -> bool {
    '__tco: loop {
        match &**exp {
            INTEGER { .. } => return true,
            REAL { .. } => return true,
            CAST { .. } => {
                exp = var_field!((**exp).exp, NFExpression::CAST);
                continue '__tco;
            }
            UNARY { .. } => {
                exp = var_field!((**exp).exp, NFExpression::UNARY);
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub fn isBoolean(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut isBool: bool;
    isBool = (match &**exp {
        BOOLEAN { .. } => true,
        _ => false,
    });
    isBool
}

pub(crate) fn isRecord(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut isRecord: bool;
    isRecord = (match &**exp {
        RECORD { .. } => true,
        _ => false,
    });
    isRecord
}

pub(crate) fn isRecordOrRecordArray(mut exp: &metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut isRecord: bool;
    isRecord = (match &**exp {
        RECORD { .. } => true,
        ARRAY { .. } => Array::all(
            var_field!((**exp).elements, NFExpression::ARRAY).clone(),
            &move |__a0: metamodelica::Ref<NFExpression>| isRecordOrRecordArray(&__a0),
        )?,
        _ => false,
    });
    Ok(isRecord)
}

pub fn fillType(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut fillExp: metamodelica::Ref<NFExpression>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression> = fillExp;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = Type::arrayDims(ty.clone());
    let mut arr_ty: metamodelica::Ref<Type::NFType> = Type::arrayElementType(&ty);
    let mut is_literal: bool = isLiteral(&exp)?;
    for mut dim in &*dims.reverse() {
        (exp, arr_ty) = fillArray_impl(
            Dimension::size(metamodelica::AsArg::as_arg(&dim), false)?,
            &exp,
            arr_ty,
            is_literal,
        )?;
    }
    Ok(exp)
}

pub(crate) fn fillArgs(
    mut fillExp: metamodelica::Ref<NFExpression>,
    mut dims: metamodelica::List<metamodelica::Ref<NFExpression>>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut result: metamodelica::Ref<NFExpression> = fillExp.clone();
    let mut arr_ty: metamodelica::Ref<Type::NFType> = typeOf(result.clone());
    let mut is_literal: bool = isLiteral(&fillExp)?;
    let mut d_resizable: metamodelica::Ref<NFExpression>;
    for mut d in &*dims.reverse() {
        d_resizable = map(
            d.clone(),
            (std::sync::Arc::new(move |__pe_a0| replaceResizableParameter(__pe_a0))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>>
                        + 'static,
                >),
        )?;
        (result, arr_ty) = fillArray_impl(toInteger(&d_resizable)?, &result, arr_ty, is_literal)?;
    }
    Ok(result)
}

pub(crate) fn fillArray(
    mut n: i32,
    mut fillExp: metamodelica::Ref<NFExpression>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut result: metamodelica::Ref<NFExpression>;
    (result, _) = fillArray_impl(n, &(fillExp.clone()), typeOf(fillExp.clone()), isLiteral(&fillExp)?)?;
    Ok(result)
}

pub(crate) fn fillArray_impl(
    mut n: i32,
    mut fillExp: &metamodelica::Ref<NFExpression>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut isLiteral: bool,
) -> Result<(metamodelica::Ref<NFExpression>, metamodelica::Ref<Type::NFType>)> {
    let mut result: metamodelica::Ref<NFExpression>;
    let mut resultType: metamodelica::Ref<Type::NFType>;
    let mut arr: metamodelica::Array<metamodelica::Ref<NFExpression>>;
    arr = Array::generate(
        n,
        &({
            let __pe_b0 = fillExp.clone();
            move || clone(__pe_b0.clone())
        }),
    )?;
    resultType = Type::liftArrayLeft(
        ty,
        &(Dimension::fromInteger(n, Prefixes::Variability::CONSTANT.clone())),
    );
    result = makeArray(resultType.clone(), arr.clone(), isLiteral);
    Ok((result, resultType))
}

pub(crate) fn liftArray(
    mut dim: &metamodelica::Ref<Dimension::NFDimension>,
    mut exp: metamodelica::Ref<NFExpression>,
) -> Result<(metamodelica::Ref<NFExpression>, metamodelica::Ref<Type::NFType>)> {
    let mut exp: metamodelica::Ref<NFExpression> = exp;
    let mut arrayType: metamodelica::Ref<Type::NFType> = typeOf(exp.clone());
    (exp, arrayType) = fillArray_impl(
        Dimension::size(dim, false)?,
        &(exp.clone()),
        arrayType,
        isLiteral(&exp)?,
    )?;
    Ok((exp, arrayType))
}

pub(crate) fn liftArrayList(
    mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
    mut exp: metamodelica::Ref<NFExpression>,
) -> Result<(metamodelica::Ref<NFExpression>, metamodelica::Ref<Type::NFType>)> {
    let mut exp: metamodelica::Ref<NFExpression> = exp;
    let mut arrayType: metamodelica::Ref<Type::NFType> = typeOf(exp.clone());
    let mut is_literal: bool = isLiteral(&exp)?;
    for mut dim in &*dims.reverse() {
        (exp, arrayType) = fillArray_impl(
            Dimension::size(metamodelica::AsArg::as_arg(&dim), false)?,
            &exp,
            arrayType,
            is_literal,
        )?;
    }
    Ok((exp, arrayType))
}

pub fn makeZero(mut ty: &metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<NFExpression>> {
    let mut zeroExp: metamodelica::Ref<NFExpression>;
    zeroExp = (match &**ty {
        Type::REAL => metamodelica::Ref::new(NFExpression::REAL {
            value: metamodelica::OrderedFloat(0.0_f64),
        }),
        Type::INTEGER => metamodelica::Ref::new(NFExpression::INTEGER { value: 0 }),
        Type::BOOLEAN => metamodelica::Ref::new(NFExpression::BOOLEAN { value: false }),
        Type::ARRAY { .. } => fillType(ty.clone(), makeZero(&(Type::arrayElementType(ty)))?)?,
        Type::COMPLEX { .. } => makeOperatorRecordZero(Type::complexNode(ty)?)?,
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFExpression.makeZero"));
                    __mm_s.push_str(&*literal!(" failed for: "));
                    __mm_s.push_str(&*Type::toString(ty)?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(zeroExp)
}

pub(crate) fn makeOperatorRecordZero(
    mut recordNode: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut zeroExp: metamodelica::Ref<NFExpression>;
    let mut op_node: metamodelica::Ref<InstNode::InstNode>;
    let mut r#fn: metamodelica::Ref<Function::Function::Function>;
    match '__try0: {
        (op_node, _) = unwrap_break_err!(Class::lookupElement(literal!("'0'"), unwrap_break_err!(InstNode::getClass(recordNode.clone()), '__try0)), '__try0);
        unwrap_break_err!(Function::Function::instFunctionNode(op_node.clone(), InstContext::NO_CONTEXT.clone(), InstNode::info(&(unwrap_break_err!(InstNode::parent(&op_node), '__try0)))), '__try0);
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(Function::Function::typeNodeCache(op_node.clone(), InstContext::FUNCTION.clone()), '__try0)) {
            Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        r#fn = metamodelica::Own::own(__pa1);
        zeroExp = metamodelica::Ref::new(NFExpression::CALL {
            call: Call::makeTypedCall(
                r#fn.clone(),
                metamodelica::nil(),
                Variability::CONSTANT.clone(),
                Purity::PURE.clone(),
                r#fn.returnType.clone(),
            ),
        });
        zeroExp = unwrap_break_err!(Ceval::evalExp(zeroExp.clone(), &(Ceval::noTarget().clone())), '__try0);
        Ok::<_, &'static str>((r#fn.clone(), op_node.clone(), zeroExp.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            r#fn = __try0_o0;
            op_node = __try0_o1;
            zeroExp = __try0_o2;
        }
        Err(__try0_err) => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFExpression.makeOperatorRecordZero"));
                    __mm_s.push_str(&*literal!(" failed for: "));
                    __mm_s.push_str(&*InstNode::toString(recordNode.clone())?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err(__try0_err);
        }
    }
    Ok(zeroExp)
}

pub fn makeOne(mut ty: &metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<NFExpression>> {
    let mut oneExp: metamodelica::Ref<NFExpression>;
    oneExp = (match &**ty {
        Type::REAL => metamodelica::Ref::new(NFExpression::REAL {
            value: metamodelica::OrderedFloat(1.0_f64),
        }),
        Type::INTEGER => metamodelica::Ref::new(NFExpression::INTEGER { value: 1 }),
        Type::ARRAY { .. } => fillType(ty.clone(), makeOne(&(Type::arrayElementType(ty)))?)?,
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFExpression.makeOne"));
                    __mm_s.push_str(&*literal!(" failed for: "));
                    __mm_s.push_str(&*Type::toString(ty)?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(oneExp)
}

pub(crate) fn makeMinusOne(mut ty: &metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<NFExpression>> {
    let mut oneExp: metamodelica::Ref<NFExpression>;
    oneExp = (match &**ty {
        Type::REAL => metamodelica::Ref::new(NFExpression::REAL {
            value: metamodelica::OrderedFloat(-1.0_f64),
        }),
        Type::INTEGER => metamodelica::Ref::new(NFExpression::INTEGER { value: -1 }),
        Type::ARRAY { .. } => fillType(ty.clone(), makeMinusOne(&(Type::arrayElementType(ty)))?)?,
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFExpression.makeMinusOne"));
                    __mm_s.push_str(&*literal!(" failed for: "));
                    __mm_s.push_str(&*Type::toString(ty)?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(oneExp)
}

pub(crate) fn makeNaN(mut ty: metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<NFExpression>> {
    let mut nan: metamodelica::Ref<NFExpression>;
    let mut zero: metamodelica::Ref<NFExpression> = makeZero(&ty)?;
    nan = metamodelica::Ref::new(NFExpression::BINARY {
        exp1: zero.clone(),
        operator: Operator::makeDiv(ty),
        exp2: zero,
    });
    Ok(nan)
}

pub fn makeMaxValue(mut ty: &metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression>;
    exp = (match &**ty {
        Type::REAL => metamodelica::Ref::new(NFExpression::REAL {
            value: System::realMaxLit(),
        }),
        Type::INTEGER => metamodelica::Ref::new(NFExpression::INTEGER {
            value: System::intMaxLit(),
        }),
        Type::BOOLEAN => metamodelica::Ref::new(NFExpression::BOOLEAN { value: true }),
        Type::ENUMERATION {
            literals: __ty_literals,
            ..
        } => metamodelica::Ref::new(NFExpression::ENUM_LITERAL {
            ty: ty.clone(),
            name: List::last(metamodelica::AsArg::as_arg(&__ty_literals))?,
            index: ((__ty_literals).len() as i32),
        }),
        Type::ARRAY { .. } => fillType(ty.clone(), makeMaxValue(&(Type::arrayElementType(ty)))?)?,
        _ => metamodelica::Ref::new(NFExpression::REAL {
            value: System::realMaxLit(),
        }),
    });
    Ok(exp)
}

pub(crate) fn makeMinValue(mut ty: &metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression>;
    exp = (match &**ty {
        Type::REAL => metamodelica::Ref::new(NFExpression::REAL {
            value: -(System::realMaxLit()),
        }),
        Type::INTEGER => metamodelica::Ref::new(NFExpression::INTEGER {
            value: -(System::intMaxLit()),
        }),
        Type::BOOLEAN => metamodelica::Ref::new(NFExpression::BOOLEAN { value: false }),
        Type::ENUMERATION {
            literals: __ty_literals,
            ..
        } => metamodelica::Ref::new(NFExpression::ENUM_LITERAL {
            ty: ty.clone(),
            name: (__ty_literals).head().cloned()?,
            index: 1,
        }),
        Type::ARRAY { .. } => fillType(ty.clone(), makeMinValue(&(Type::arrayElementType(ty)))?)?,
        _ => metamodelica::Ref::new(NFExpression::REAL {
            value: -(System::realMaxLit()),
        }),
    });
    Ok(exp)
}

pub(crate) fn makeDefaultValue(
    mut ty: &metamodelica::Ref<Type::NFType>,
    mut min: Option<metamodelica::Ref<NFExpression>>,
    mut max: Option<metamodelica::Ref<NFExpression>>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression>;
    exp = (match &**ty {
        Type::INTEGER => {
            if (min).is_some() && isNonNegative(&(Util::getOption(min.clone())?))? {
                let __pa0 = ::match_deref::match_deref! { match &(min) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                exp = metamodelica::Own::own(__pa0);
            } else if (max).is_some() && isNonPositive(&(Util::getOption(max.clone())?))? {
                let __pa1 = ::match_deref::match_deref! { match &(max) {
                    Some(__pa1) => __pa1.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                exp = metamodelica::Own::own(__pa1);
            } else {
                exp = metamodelica::Ref::new(NFExpression::INTEGER { value: 0 });
            }
            exp
        }
        Type::REAL => {
            if (min).is_some() && isNonNegative(&(Util::getOption(min.clone())?))? {
                let __pa0 = ::match_deref::match_deref! { match &(min) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                exp = metamodelica::Own::own(__pa0);
            } else if (max).is_some() && isNonPositive(&(Util::getOption(max.clone())?))? {
                let __pa1 = ::match_deref::match_deref! { match &(max) {
                    Some(__pa1) => __pa1.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                exp = metamodelica::Own::own(__pa1);
            } else {
                exp = metamodelica::Ref::new(NFExpression::REAL {
                    value: metamodelica::OrderedFloat(0.0_f64),
                });
            }
            exp
        }
        Type::STRING => metamodelica::Ref::new(NFExpression::STRING { value: literal!("") }),
        Type::BOOLEAN => metamodelica::Ref::new(NFExpression::BOOLEAN { value: false }),
        Type::ENUMERATION {
            literals: __ty_literals,
            ..
        } => {
            if (min).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(min) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                exp = metamodelica::Own::own(__pa0);
            } else {
                exp = metamodelica::Ref::new(NFExpression::ENUM_LITERAL {
                    ty: ty.clone(),
                    name: (__ty_literals).head().cloned()?,
                    index: 1,
                });
            }
            exp
        }
        Type::ARRAY { .. } => fillType(ty.clone(), makeDefaultValue(&(Type::arrayElementType(ty)), None, None)?)?,
        Type::TUPLE { types: __ty_types, .. } => metamodelica::Ref::new(NFExpression::TUPLE {
            ty: ty.clone(),
            elements: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
                for mut t in (__ty_types.clone()).into_iter().cloned() {
                    let __x = makeDefaultValue(&(t.clone()), None, None)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        }),
        _ => return Err("match: no arm matched"),
    });
    Ok(exp)
}

pub(crate) fn r#box(mut exp: &metamodelica::Ref<NFExpression>) -> metamodelica::Ref<NFExpression> {
    let mut boxedExp: metamodelica::Ref<NFExpression>;
    boxedExp = (match &**exp {
        STRING { .. } => exp.clone(),
        RECORD {
            elements: __exp_elements,
            path: __exp_path,
            ty: __exp_ty,
        } => metamodelica::Ref::new(NFExpression::RECORD {
            path: __exp_path.clone(),
            ty: Type::r#box(metamodelica::AsArg::as_arg(&__exp_ty)),
            elements: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
                for mut e in (__exp_elements.clone()).into_iter().cloned() {
                    let __x = r#box(&(e.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        }),
        BOX { .. } => exp.clone(),
        FILENAME { .. } => exp.clone(),
        _ => metamodelica::Ref::new(NFExpression::BOX { exp: exp.clone() }),
    });
    boxedExp
}

pub(crate) fn unbox(mut boxedExp: metamodelica::Ref<NFExpression>) -> metamodelica::Ref<NFExpression> {
    let mut exp: metamodelica::Ref<NFExpression>;
    exp = (match &*boxedExp {
        BOX { exp: __boxedExp_exp } => __boxedExp_exp.clone(),
        _ => {
            let mut ty: metamodelica::Ref<Type::NFType>;
            ty = typeOf(boxedExp.clone());
            if (Type::isBoxed(&ty)) {
                metamodelica::Ref::new(NFExpression::UNBOX {
                    exp: boxedExp,
                    ty: Type::unbox(ty),
                })
            } else {
                boxedExp
            }
        }
    });
    exp
}

pub(crate) fn isNegated<'__b>(mut exp: &'__b metamodelica::Ref<NFExpression>) -> bool {
    '__tco: loop {
        match &**exp {
            INTEGER { .. } => return var_field!((**exp).value, NFExpression::INTEGER).clone() < 0,
            REAL { .. } => {
                return var_field!((**exp).value, NFExpression::REAL).clone() < metamodelica::OrderedFloat((0) as f64);
            }
            CAST { .. } => {
                exp = var_field!((**exp).exp, NFExpression::CAST);
                continue '__tco;
            }
            UNARY { .. } => return true,
            _ => return false,
        }
    }
}

pub fn negate(mut exp: metamodelica::Ref<NFExpression>) -> metamodelica::Ref<NFExpression> {
    let mut exp: metamodelica::Ref<NFExpression> = exp;
    exp = (match &*exp {
        INTEGER { value: __exp_value } => metamodelica::Ref::new(NFExpression::INTEGER {
            value: -(__exp_value.clone()),
        }),
        REAL { value: __exp_value } => metamodelica::Ref::new(NFExpression::REAL {
            value: -(__exp_value.clone()),
        }),
        CAST {
            exp: __exp_exp,
            ty: __exp_ty,
        } => metamodelica::Ref::new(NFExpression::CAST {
            ty: __exp_ty.clone(),
            exp: negate(__exp_exp.clone()),
        }),
        UNARY { exp: __exp_exp, .. } => __exp_exp.clone(),
        _ => metamodelica::Ref::new(NFExpression::UNARY {
            operator: Operator::makeUMinus(typeOf(exp.clone())),
            exp: exp,
        }),
    });
    exp
}

pub fn logicNegate(mut exp: metamodelica::Ref<NFExpression>) -> metamodelica::Ref<NFExpression> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    outExp = (match &*exp {
        BOOLEAN { value: __exp_value } => metamodelica::Ref::new(NFExpression::BOOLEAN {
            value: !(__exp_value.clone()),
        }),
        LUNARY { exp: __exp_exp, .. } => __exp_exp.clone(),
        _ => metamodelica::Ref::new(NFExpression::LUNARY {
            operator: Operator::makeNot(typeOf(exp.clone())),
            exp: exp,
        }),
    });
    outExp
}

pub fn revertRange(mut range: metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> {
    let mut range: metamodelica::Ref<NFExpression> = range;
    range = (::match_deref::match_deref! { match &(range.clone()) {
        Deref @ RANGE { step: Some(step), start: __range_start, stop: __range_stop, ty: __range_ty } => {
            metamodelica::Ref::new(NFExpression::RANGE { ty: __range_ty.clone(), start: __range_stop.clone(), step: Some(negate(step.clone())), stop: __range_start.clone() })
        },
        Deref @ RANGE { start: __range_start, stop: __range_stop, ty: __range_ty, .. } => {
            metamodelica::Ref::new(NFExpression::RANGE { ty: __range_ty.clone(), start: __range_stop.clone(), step: Some(metamodelica::Ref::new(NFExpression::INTEGER { value: -1 })), stop: __range_start.clone() })
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.revertRange")); __mm_s.push_str(&*literal!(" failed because expression is not a range:\n")); __mm_s.push_str(&*toString(range)?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(range)
}

pub fn sliceRange(
    mut range: metamodelica::Ref<NFExpression>,
    mut slice: (i32, i32, i32),
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut range: metamodelica::Ref<NFExpression> = range;
    range = (::match_deref::match_deref! { match &((range.clone(), slice)) {
        (Deref @ RANGE { .. }, (slice_start, slice_step, slice_stop)) => {
            let mut start: i32;
            let mut step: i32;
            let mut stop: i32;
            step = Util::applyOptionOrDefault(var_field!((*range).step, NFExpression::RANGE).clone(), &integerValue, 1)?;
            start = integerValue(var_field!((*range).start, NFExpression::RANGE).clone())?;
            stop = start + slice_stop.clone() * step;
            start = start + slice_start.clone() * step;
            step = slice_step.clone() * step;
            range = metamodelica::Ref::new(NFExpression::RANGE { ty: var_field!((*range).ty, NFExpression::RANGE).clone(), start: metamodelica::Ref::new(NFExpression::INTEGER { value: start }), step: Some(metamodelica::Ref::new(NFExpression::INTEGER { value: step })), stop: metamodelica::Ref::new(NFExpression::INTEGER { value: stop }) });
            retype(range)?
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFExpression.sliceRange")); __mm_s.push_str(&*literal!(" failed because expression is not a range:\n")); __mm_s.push_str(&*toString(range)?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(range)
}

pub(crate) fn arrayElements(
    mut array: &metamodelica::Ref<NFExpression>,
) -> Result<metamodelica::Array<metamodelica::Ref<NFExpression>>> {
    let mut elements: metamodelica::Array<metamodelica::Ref<NFExpression>>;
    let __pa0 = ::match_deref::match_deref! { match &((*array)) {
        Deref @ ARRAY { elements: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    elements = metamodelica::Own::own(__pa0);
    Ok(elements)
}

pub fn arrayElementList(
    mut array: &metamodelica::Ref<NFExpression>,
) -> Result<metamodelica::List<metamodelica::Ref<NFExpression>>> {
    let mut elements: metamodelica::List<metamodelica::Ref<NFExpression>>;
    elements = (match &**array {
        ARRAY { .. } => var_field!((**array).elements, NFExpression::ARRAY)
            .clone()
            .borrow()
            .iter()
            .cloned()
            .collect::<metamodelica::List<_>>(),
        _ => return Err("match: no arm matched"),
    });
    Ok(elements)
}

pub(crate) fn arrayScalarElements(
    mut exp: &metamodelica::Ref<NFExpression>,
) -> metamodelica::List<metamodelica::Ref<NFExpression>> {
    let mut elements: metamodelica::List<metamodelica::Ref<NFExpression>>;
    elements = metamodelica::Dangerous::listReverseInPlace(arrayScalarElements_impl(exp, metamodelica::nil()));
    elements
}

pub(crate) fn arrayScalarElements_impl(
    mut exp: &metamodelica::Ref<NFExpression>,
    mut elements: metamodelica::List<metamodelica::Ref<NFExpression>>,
) -> metamodelica::List<metamodelica::Ref<NFExpression>> {
    let mut elements: metamodelica::List<metamodelica::Ref<NFExpression>> = elements;
    elements = (match &**exp {
        ARRAY { .. } => {
            let __range0 = var_field!((**exp).elements, NFExpression::ARRAY)
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            for mut e in __range0 {
                elements = arrayScalarElements_impl(&e, elements);
            }
            elements
        }
        _ => metamodelica::cons(exp.clone(), elements),
    });
    elements
}

pub(crate) fn arrayScalarElement(
    mut arrayExp: &metamodelica::Ref<NFExpression>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut scalarExp: metamodelica::Ref<NFExpression>;
    scalarExp = (match &**arrayExp {
        ARRAY { .. }
            if (metamodelica::arrayLength(var_field!((**arrayExp).elements, NFExpression::ARRAY).clone()) == 1) =>
        {
            metamodelica::arrayGet(var_field!((**arrayExp).elements, NFExpression::ARRAY).clone(), 1)?
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(scalarExp)
}

pub(crate) fn hasArrayCall(mut exp: metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut hasArrayCall: bool;
    hasArrayCall = contains(exp, &move |__a0: metamodelica::Ref<NFExpression>| hasArrayCall2(&__a0))?;
    Ok(hasArrayCall)
}

pub(crate) fn hasArrayCall2(mut exp: &metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut hasArrayCall: bool;
    let mut call: metamodelica::Ref<Call::NFCall>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    hasArrayCall = (::match_deref::match_deref! { match exp {
        Deref @ CALL { call: __esc_call } => {
            call = (*__esc_call).clone();
            ty = Call::typeOf(metamodelica::AsArg::as_arg(&call));
            Type::isArray(&ty) && Call::isVectorizeable(metamodelica::AsArg::as_arg(&call))
        },
        Deref @ TUPLE_ELEMENT { tupleExp: Deref @ CALL { call: __esc_call }, index: __exp_index, .. } => {
            call = (*__esc_call).clone();
            ty = Type::nthTupleType(&(Call::typeOf(metamodelica::AsArg::as_arg(&call))), __exp_index.clone())?;
            Type::isArray(&ty) && Call::isVectorizeable(metamodelica::AsArg::as_arg(&call))
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(hasArrayCall)
}

pub(crate) fn transposeArray(
    mut arrayExp: &metamodelica::Ref<NFExpression>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    let mut dim1: metamodelica::Ref<Dimension::NFDimension>;
    let mut dim2: metamodelica::Ref<Dimension::NFDimension>;
    let mut rest_dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut row_ty: metamodelica::Ref<Type::NFType>;
    let mut literal: bool;
    let mut arr: metamodelica::Array<metamodelica::Ref<NFExpression>>;
    let mut matrix_arr: metamodelica::Array<metamodelica::Array<metamodelica::Ref<NFExpression>>>;
    outExp = (::match_deref::match_deref! { match arrayExp {
        Deref @ ARRAY { ty: Deref @ Type::ARRAY { elementType: __esc_ty, dimensions: Deref @ metamodelica::ListNode::Cons { head: __esc_dim1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_dim2, tail: __esc_rest_dims } } }, elements: __esc_arr, literal: __esc_literal } => {
            ty = (*__esc_ty).clone();
            dim1 = (*__esc_dim1).clone();
            dim2 = (*__esc_dim2).clone();
            rest_dims = (*__esc_rest_dims).clone();
            arr = (*__esc_arr).clone();
            literal = (*__esc_literal).clone();
            if !(arr.clone().borrow().is_empty()) {
                row_ty = metamodelica::Ref::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: metamodelica::cons(dim1.clone(), rest_dims.clone()) });
                matrix_arr = Array::map(arr.clone(), &move |__a0: metamodelica::Ref<NFExpression>| arrayElements(&__a0))?;
                matrix_arr = Array::transpose(matrix_arr.clone());
                arr = Array::map(matrix_arr.clone(), &({ let __pe_b0 = row_ty; let __pe_b2 = literal.clone(); move |__pe_a1| Ok(makeArray(__pe_b0.clone(), __pe_a1, __pe_b2.clone())) }))?;
            }
            makeArray(metamodelica::Ref::new(Type::NFType::ARRAY { elementType: ty.clone(), dimensions: metamodelica::cons(dim2.clone(), metamodelica::cons(dim1.clone(), rest_dims.clone())) }), arr.clone(), literal.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

pub fn makeIdentityMatrix(
    mut n: i32,
    mut elementType: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut matrix: metamodelica::Ref<NFExpression>;
    let mut row: metamodelica::Array<metamodelica::Ref<NFExpression>>;
    let mut rows: metamodelica::Array<metamodelica::Ref<NFExpression>>;
    let mut zero: metamodelica::Ref<NFExpression>;
    let mut one: metamodelica::Ref<NFExpression>;
    let mut row_ty: metamodelica::Ref<Type::NFType>;
    zero = makeZero(&elementType)?;
    one = makeOne(&elementType)?;
    rows = metamodelica::arrayCreate(n, zero.clone());
    row_ty = metamodelica::Ref::new(Type::NFType::ARRAY {
        elementType: elementType,
        dimensions: list![Dimension::fromInteger(n, Prefixes::Variability::CONSTANT.clone())],
    });
    for mut i in 1..=n {
        row = metamodelica::arrayCreate(n, zero.clone());
        for mut j in 1..=n {
            unsafe {
                metamodelica::Dangerous::arrayInitSlot(
                    row.clone(),
                    j,
                    if (i == j) { one.clone() } else { zero.clone() },
                )
            };
        }
        unsafe {
            metamodelica::Dangerous::arrayInitSlot(rows.clone(), i, makeArray(row_ty.clone(), row.clone(), true))
        };
    }
    matrix = makeExpArray(rows.clone(), row_ty, true);
    Ok(matrix)
}

pub fn makeTriuMask(mut n: i32, mut elTy: metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<NFExpression>> {
    let mut mask: metamodelica::Ref<NFExpression>;
    let mut row: metamodelica::Array<metamodelica::Ref<NFExpression>>;
    let mut rows: metamodelica::Array<metamodelica::Ref<NFExpression>>;
    let mut zero: metamodelica::Ref<NFExpression>;
    let mut one: metamodelica::Ref<NFExpression>;
    let mut row_ty: metamodelica::Ref<Type::NFType>;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    zero = makeZero(&elTy)?;
    one = makeOne(&elTy)?;
    rows = metamodelica::arrayCreate(n, zero.clone());
    row_ty = metamodelica::Ref::new(Type::NFType::ARRAY {
        elementType: elTy,
        dimensions: list![Dimension::fromInteger(n, Prefixes::Variability::CONSTANT.clone())],
    });
    for mut i in 1..=n {
        row = metamodelica::arrayCreate(n, zero.clone());
        for mut j in 1..=n {
            unsafe {
                metamodelica::Dangerous::arrayInitSlot(
                    row.clone(),
                    j,
                    if (i <= j) { one.clone() } else { zero.clone() },
                )
            };
        }
        unsafe {
            metamodelica::Dangerous::arrayInitSlot(rows.clone(), i, makeArray(row_ty.clone(), row.clone(), true))
        };
    }
    mask = makeExpArray(rows.clone(), row_ty, true);
    Ok(mask)
}

pub(crate) fn promote(
    mut e: metamodelica::Ref<NFExpression>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut n: i32,
) -> Result<(metamodelica::Ref<NFExpression>, metamodelica::Ref<Type::NFType>)> {
    let mut e: metamodelica::Ref<NFExpression> = e;
    let mut ty: metamodelica::Ref<Type::NFType> = ty;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut ety: metamodelica::Ref<Type::NFType>;
    let mut tys: metamodelica::List<metamodelica::Ref<Type::NFType>> = metamodelica::nil();
    let mut is_array: bool;
    dims = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
        for mut i in (Type::dimensionCount(ty.clone())..=n - 1).into_iter() {
            let __x = Dimension::fromInteger(1, Prefixes::Variability::CONSTANT.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if !((dims).is_empty()) {
        dims = listAppend(Type::arrayDims(ty.clone()), dims);
        is_array = Type::isArray(&ty);
        ety = Type::arrayElementType(&ty);
        ty = Type::liftArrayLeftList(ety.clone(), &dims);
        while !((dims).is_empty()) {
            tys = metamodelica::cons(Type::liftArrayLeftList(ety.clone(), &dims), tys);
            dims = (dims).rest()?;
        }
        e = promote2(e, is_array, n, &(tys.reverse()))?;
    }
    Ok((e, ty))
}

pub(crate) fn promote2(
    mut exp: metamodelica::Ref<NFExpression>,
    mut isArray: bool,
    mut dims: i32,
    mut types: &metamodelica::List<metamodelica::Ref<Type::NFType>>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression> = exp.clone();
    outExp = (::match_deref::match_deref! { match &((exp.clone(), types.clone())) {
        (_, Deref @ metamodelica::ListNode::Nil) => {
            exp
        },
        (Deref @ ARRAY { .. }, Deref @ metamodelica::ListNode::Cons { head: ty, tail: rest_ty }) => {
            makeArray(ty.clone(), Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), &({ let __pe_b1 = false; let __pe_b2 = dims; let __pe_b3 = rest_ty.clone(); move |__pe_a0| promote2(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), &__pe_b3) }))?, false)
        },
        (_, _) if (isArray) => {
            let mut expanded: bool;
            if Flags::getConfigBool(Flags::NEW_BACKEND.clone())? && !(isLiteral(&exp)?) {
                expanded = false;
            } else {
                (outExp, expanded) = ExpandExp::expand(exp.clone(), false, false)?;
            }
            if expanded && self::isArray(&outExp) {
                outExp = promote2(outExp, true, dims, types)?;
            } else {
                outExp = metamodelica::Ref::new(NFExpression::CALL { call: Call::makeTypedCall(NFBuiltinFuncs::PROMOTE().clone(), list![exp.clone(), metamodelica::Ref::new(NFExpression::INTEGER { value: dims })], variability(exp.clone())?, purity(exp)?, (types).head().cloned()?) });
            }
            outExp
        },
        _ => {
            let mut ty: metamodelica::Ref<Type::NFType> = metamodelica::Ref::new(Type::ANY);
            outExp = exp;
            for mut ty in &*types.clone().reverse() {
                let mut ty = ty.clone();
                outExp = makeArray(ty, arrayCreate(1, outExp), false);
            }
            outExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn variability(mut exp: metamodelica::Ref<NFExpression>) -> Result<Variability> {
    let mut var: Variability;
    var = (match &*exp {
        INTEGER { .. } => Variability::CONSTANT.clone(),
        REAL { .. } => Variability::CONSTANT.clone(),
        STRING { .. } => Variability::CONSTANT.clone(),
        BOOLEAN { .. } => Variability::CONSTANT.clone(),
        ENUM_LITERAL { .. } => Variability::CONSTANT.clone(),
        CLKCONST { .. } => Variability::DISCRETE.clone(),
        CREF { cref: __exp_cref, .. } => ComponentRef::variability(metamodelica::AsArg::as_arg(&__exp_cref))?,
        TYPENAME { .. } => Variability::CONSTANT.clone(),
        ARRAY { .. } => variabilityArray(
            var_field!((*exp).elements, NFExpression::ARRAY).clone(),
            Prefixes::Variability::CONSTANT.clone(),
        )?,
        MATRIX {
            elements: __exp_elements,
        } => List::fold(
            metamodelica::AsArg::as_arg(&__exp_elements),
            &move |__a0: metamodelica::List<metamodelica::Ref<NFExpression>>, __a1: Variability| {
                variabilityList(&__a0, __a1)
            },
            Variability::CONSTANT.clone(),
        )?,
        RANGE {
            start: __exp_start,
            step: __exp_step,
            stop: __exp_stop,
            ..
        } => {
            var = variability(__exp_start.clone())?;
            var = Prefixes::variabilityMax(var, variability(__exp_stop.clone())?);
            if (__exp_step).is_some() {
                var = Prefixes::variabilityMax(var, variability(Util::getOption(__exp_step.clone())?)?);
            }
            var
        }
        TUPLE {
            elements: __exp_elements,
            ..
        } => variabilityList(
            metamodelica::AsArg::as_arg(&__exp_elements),
            Prefixes::Variability::CONSTANT.clone(),
        )?,
        RECORD {
            elements: __exp_elements,
            ..
        } => variabilityList(
            metamodelica::AsArg::as_arg(&__exp_elements),
            Prefixes::Variability::CONSTANT.clone(),
        )?,
        CALL { call: __exp_call } => Call::variability(metamodelica::AsArg::as_arg(&__exp_call))?,
        SIZE {
            dimIndex: __exp_dimIndex,
            ..
        } => {
            if (__exp_dimIndex).is_some() {
                var = Prefixes::variabilityMax(
                    Variability::PARAMETER.clone(),
                    variability(Util::getOption(__exp_dimIndex.clone())?)?,
                );
            } else {
                var = Variability::PARAMETER.clone();
            }
            var
        }
        END { .. } => Variability::PARAMETER.clone(),
        MULTARY {
            arguments: __exp_arguments,
            ..
        } => Prefixes::variabilityMax(
            variabilityList(
                metamodelica::AsArg::as_arg(&__exp_arguments),
                Prefixes::Variability::CONSTANT.clone(),
            )?,
            variabilityList(
                metamodelica::AsArg::as_arg(&__exp_arguments),
                Prefixes::Variability::CONSTANT.clone(),
            )?,
        ),
        BINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            ..
        } => Prefixes::variabilityMax(variability(__exp_exp1.clone())?, variability(__exp_exp2.clone())?),
        UNARY { exp: __exp_exp, .. } => variability(__exp_exp.clone())?,
        LBINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            ..
        } => Prefixes::variabilityMax(variability(__exp_exp1.clone())?, variability(__exp_exp2.clone())?),
        LUNARY { exp: __exp_exp, .. } => variability(__exp_exp.clone())?,
        RELATION {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            ..
        } => Prefixes::variabilityMin(
            Prefixes::variabilityMax(variability(__exp_exp1.clone())?, variability(__exp_exp2.clone())?),
            Variability::DISCRETE.clone(),
        ),
        IF {
            condition: __exp_condition,
            falseBranch: __exp_falseBranch,
            trueBranch: __exp_trueBranch,
            ..
        } => Prefixes::variabilityMax(
            variability(__exp_condition.clone())?,
            Prefixes::variabilityMax(
                variability(__exp_trueBranch.clone())?,
                variability(__exp_falseBranch.clone())?,
            ),
        ),
        CAST { exp: __exp_exp, .. } => variability(__exp_exp.clone())?,
        BOX { exp: __exp_exp } => variability(__exp_exp.clone())?,
        UNBOX { exp: __exp_exp, .. } => variability(__exp_exp.clone())?,
        SUBSCRIPTED_EXP {
            exp: __exp_exp,
            subscripts: __exp_subscripts,
            ..
        } => Prefixes::variabilityMax(
            variability(__exp_exp.clone())?,
            Subscript::variabilityList(metamodelica::AsArg::as_arg(&__exp_subscripts))?,
        ),
        TUPLE_ELEMENT {
            tupleExp: __exp_tupleExp,
            ..
        } => variability(__exp_tupleExp.clone())?,
        RECORD_ELEMENT {
            recordExp: __exp_recordExp,
            ..
        } => variability(__exp_recordExp.clone())?,
        MUTABLE { exp: __exp_exp } => variability(Mutable::access(__exp_exp.clone()))?,
        SHARED_LITERAL { exp: __exp_exp, .. } => variability(__exp_exp.clone())?,
        EMPTY { .. } => Variability::CONSTANT.clone(),
        PARTIAL_FUNCTION_APPLICATION { .. } => Variability::CONTINUOUS.clone(),
        FILENAME { .. } => Variability::CONSTANT.clone(),
        INSTANCE_NAME { .. } => Variability::CONSTANT.clone(),
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFExpression.variability"));
                    __mm_s.push_str(&*literal!(" got unknown expression."));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(var)
}

pub(crate) fn variabilityArray(
    mut expl: metamodelica::Array<metamodelica::Ref<NFExpression>>,
    mut var: Variability,
) -> Result<Variability> {
    let mut var: Variability = var;
    let __range0 = expl.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        var = Prefixes::variabilityMax(var, variability(e)?);
    }
    Ok(var)
}

pub(crate) fn variabilityList(
    mut expl: &metamodelica::List<metamodelica::Ref<NFExpression>>,
    mut var: Variability,
) -> Result<Variability> {
    let mut var: Variability = var;
    for mut e in &**expl {
        var = Prefixes::variabilityMax(var, variability(e.clone())?);
    }
    Ok(var)
}

pub fn purity(mut exp: metamodelica::Ref<NFExpression>) -> Result<Purity> {
    let mut pur: Purity;
    pur = (match &*exp {
        INTEGER { .. } => Purity::PURE.clone(),
        REAL { .. } => Purity::PURE.clone(),
        STRING { .. } => Purity::PURE.clone(),
        BOOLEAN { .. } => Purity::PURE.clone(),
        ENUM_LITERAL { .. } => Purity::PURE.clone(),
        CLKCONST { .. } => Purity::PURE.clone(),
        CREF { cref: __exp_cref, .. } => ComponentRef::purity(metamodelica::AsArg::as_arg(&__exp_cref))?,
        TYPENAME { .. } => Purity::PURE.clone(),
        ARRAY { .. } => purityArray(
            var_field!((*exp).elements, NFExpression::ARRAY).clone(),
            Prefixes::Purity::PURE.clone(),
        )?,
        MATRIX {
            elements: __exp_elements,
        } => List::fold(
            metamodelica::AsArg::as_arg(&__exp_elements),
            &move |__a0: metamodelica::List<metamodelica::Ref<NFExpression>>, __a1: Purity| purityList(&__a0, __a1),
            Purity::PURE.clone(),
        )?,
        RANGE {
            start: __exp_start,
            step: __exp_step,
            stop: __exp_stop,
            ..
        } => {
            pur = purity(__exp_start.clone())?;
            pur = Prefixes::purityMin(pur, purity(__exp_stop.clone())?);
            if (__exp_step).is_some() {
                pur = Prefixes::purityMin(pur, purity(Util::getOption(__exp_step.clone())?)?);
            }
            pur
        }
        TUPLE {
            elements: __exp_elements,
            ..
        } => purityList(
            metamodelica::AsArg::as_arg(&__exp_elements),
            Prefixes::Purity::PURE.clone(),
        )?,
        RECORD {
            elements: __exp_elements,
            ..
        } => purityList(
            metamodelica::AsArg::as_arg(&__exp_elements),
            Prefixes::Purity::PURE.clone(),
        )?,
        CALL { call: __exp_call } => Call::purity(metamodelica::AsArg::as_arg(&__exp_call)),
        SIZE {
            dimIndex: __exp_dimIndex,
            ..
        } => {
            if ((__exp_dimIndex).is_some()) {
                purity(Util::getOption(__exp_dimIndex.clone())?)?
            } else {
                Purity::PURE.clone()
            }
        }
        END { .. } => Purity::PURE.clone(),
        BINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            ..
        } => Prefixes::purityMin(purity(__exp_exp1.clone())?, purity(__exp_exp2.clone())?),
        UNARY { exp: __exp_exp, .. } => purity(__exp_exp.clone())?,
        LBINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            ..
        } => Prefixes::purityMin(purity(__exp_exp1.clone())?, purity(__exp_exp2.clone())?),
        LUNARY { exp: __exp_exp, .. } => purity(__exp_exp.clone())?,
        RELATION {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            ..
        } => Prefixes::purityMin(purity(__exp_exp1.clone())?, purity(__exp_exp2.clone())?),
        MULTARY {
            arguments: __exp_arguments,
            inv_arguments: __exp_inv_arguments,
            ..
        } => Prefixes::purityMin(
            purityList(
                metamodelica::AsArg::as_arg(&__exp_arguments),
                Prefixes::Purity::PURE.clone(),
            )?,
            purityList(
                metamodelica::AsArg::as_arg(&__exp_inv_arguments),
                Prefixes::Purity::PURE.clone(),
            )?,
        ),
        IF {
            condition: __exp_condition,
            falseBranch: __exp_falseBranch,
            trueBranch: __exp_trueBranch,
            ..
        } => Prefixes::purityMin(
            purity(__exp_condition.clone())?,
            Prefixes::purityMin(purity(__exp_trueBranch.clone())?, purity(__exp_falseBranch.clone())?),
        ),
        CAST { exp: __exp_exp, .. } => purity(__exp_exp.clone())?,
        BOX { exp: __exp_exp } => purity(__exp_exp.clone())?,
        UNBOX { exp: __exp_exp, .. } => purity(__exp_exp.clone())?,
        SUBSCRIPTED_EXP {
            exp: __exp_exp,
            subscripts: __exp_subscripts,
            ..
        } => Prefixes::purityMin(
            purity(__exp_exp.clone())?,
            Subscript::purityList(metamodelica::AsArg::as_arg(&__exp_subscripts))?,
        ),
        TUPLE_ELEMENT {
            tupleExp: __exp_tupleExp,
            ..
        } => purity(__exp_tupleExp.clone())?,
        RECORD_ELEMENT {
            recordExp: __exp_recordExp,
            ..
        } => purity(__exp_recordExp.clone())?,
        MUTABLE { exp: __exp_exp } => purity(Mutable::access(__exp_exp.clone()))?,
        SHARED_LITERAL { exp: __exp_exp, .. } => purity(__exp_exp.clone())?,
        EMPTY { .. } => Purity::PURE.clone(),
        PARTIAL_FUNCTION_APPLICATION { .. } => Purity::PURE.clone(),
        FILENAME { .. } => Purity::PURE.clone(),
        INSTANCE_NAME { .. } => Purity::PURE.clone(),
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFExpression.purity"));
                    __mm_s.push_str(&*literal!(" got unknown expression."));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFExpression.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(pur)
}

pub(crate) fn purityArray(
    mut expl: metamodelica::Array<metamodelica::Ref<NFExpression>>,
    mut pur: Purity,
) -> Result<Purity> {
    let mut pur: Purity = pur;
    let __range0 = expl.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut e in __range0 {
        pur = Prefixes::purityMin(pur, purity(e)?);
    }
    Ok(pur)
}

pub(crate) fn purityList(
    mut expl: &metamodelica::List<metamodelica::Ref<NFExpression>>,
    mut pur: Purity,
) -> Result<Purity> {
    let mut pur: Purity = pur;
    for mut e in &**expl {
        pur = Prefixes::purityMin(pur, purity(e.clone())?);
    }
    Ok(pur)
}

pub(crate) fn makeMutable(mut exp: metamodelica::Ref<NFExpression>) -> metamodelica::Ref<NFExpression> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    outExp = metamodelica::Ref::new(NFExpression::MUTABLE {
        exp: Mutable::create(exp),
    });
    outExp
}

pub(crate) fn makeImmutable(mut exp: metamodelica::Ref<NFExpression>) -> metamodelica::Ref<NFExpression> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    outExp = (match &*exp {
        MUTABLE { exp: __exp_exp } => Mutable::access(__exp_exp.clone()),
        _ => exp,
    });
    outExp
}

pub(crate) fn isMutable(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut isMutable: bool;
    isMutable = (match &**exp {
        MUTABLE { .. } => true,
        _ => false,
    });
    isMutable
}

pub(crate) fn updateMutable(
    mut mutableExp: &metamodelica::Ref<NFExpression>,
    mut value: metamodelica::Ref<NFExpression>,
) -> Result<()> {
    let mut exp_ptr: Mutable::Mutable<metamodelica::Ref<NFExpression>>;
    let __pa0 = ::match_deref::match_deref! { match &((*mutableExp)) {
        Deref @ MUTABLE { exp: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    exp_ptr = metamodelica::Own::own(__pa0);
    Mutable::update(exp_ptr, value);
    Ok(())
}

pub(crate) fn applyMutable(
    mut mutableExp: &metamodelica::Ref<NFExpression>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>>,
) -> Result<()> {
    pub type FuncType = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static,
    >;

    let mut exp_ptr: Mutable::Mutable<metamodelica::Ref<NFExpression>>;
    let __pa0 = ::match_deref::match_deref! { match &((*mutableExp)) {
        Deref @ MUTABLE { exp: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    exp_ptr = metamodelica::Own::own(__pa0);
    Mutable::update(exp_ptr.clone(), func(Mutable::access(exp_ptr))?);
    Ok(())
}

pub fn isEmpty(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut empty: bool;
    empty = (match &**exp {
        EMPTY { .. } => true,
        _ => false,
    });
    empty
}

pub fn isEnd(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut isend: bool;
    isend = (match &**exp {
        END { .. } => true,
        _ => false,
    });
    isend
}

pub(crate) fn enumIndexExp(mut enumExp: metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> {
    let mut indexExp: metamodelica::Ref<NFExpression>;
    indexExp = (match &*enumExp {
        ENUM_LITERAL {
            index: __enumExp_index, ..
        } => metamodelica::Ref::new(NFExpression::INTEGER {
            value: __enumExp_index.clone(),
        }),
        _ => metamodelica::Ref::new(NFExpression::CALL {
            call: Call::makeTypedCall(
                NFBuiltinFuncs::INTEGER_ENUM().clone(),
                list![enumExp.clone()],
                variability(enumExp)?,
                Purity::PURE.clone(),
                NFBuiltinFuncs::INTEGER_ENUM().returnType.clone(),
            ),
        }),
    });
    Ok(indexExp)
}

pub(crate) fn toScalar(mut exp: metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> {
    '__tco: loop {
        match &*exp {
            ARRAY { .. }
                if (metamodelica::arrayLength(var_field!((*exp).elements, NFExpression::ARRAY).clone()) == 1) =>
            {
                exp = metamodelica::arrayGet(var_field!((*exp).elements, NFExpression::ARRAY).clone(), 1)?;
                continue '__tco;
            }
            _ => return Ok(exp.clone()),
        }
    }
}

pub fn tupleElement(
    mut exp: metamodelica::Ref<NFExpression>,
    mut index: i32,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut tupleElem: metamodelica::Ref<NFExpression>;
    tupleElem = (match &*exp {
        TUPLE {
            elements: __exp_elements,
            ..
        } => (__exp_elements).get(index)?,
        ARRAY { .. } => {
            assign_variant_field!(exp => NFExpression::ARRAY; elements = Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), &({ let __pe_b1 = index; move |__pe_a0| tupleElement(__pe_a0, __pe_b1.clone()) }))?);
            exp
        }
        SUBSCRIPTED_EXP { split: true, .. } => mapSplitExpressions(
            exp,
            &({
                let __pe_b1 = index;
                move |__pe_a0| tupleElement(__pe_a0, __pe_b1.clone())
            }),
        )?,
        _ => metamodelica::Ref::new(NFExpression::TUPLE_ELEMENT {
            tupleExp: exp.clone(),
            index: index,
            ty: Type::nthTupleType(&(typeOf(exp)), index)?,
        }),
    });
    Ok(tupleElem)
}

pub fn recordElement(
    mut elementName: &ArcStr,
    mut recordExp: &metamodelica::Ref<NFExpression>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    outExp = (::match_deref::match_deref! { match recordExp {
        Deref @ RECORD { ty: Deref @ Type::COMPLEX { .. }, elements: __recordExp_elements, .. } => {
            let mut node: metamodelica::Ref<InstNode::InstNode>;
            let mut cls: metamodelica::Ref<Class::NFClass>;
            let mut index: i32;
            node = Type::complexNode(var_field!((**recordExp).ty, NFExpression::RECORD))?;
            cls = InstNode::getClass(node)?;
            index = Class::lookupComponentIndex(elementName.clone(), cls)?;
            (__recordExp_elements).get(index)?
        },
        Deref @ CREF { cref: __recordExp_cref, ty: __recordExp_ty } => {
            let mut node: metamodelica::Ref<InstNode::InstNode>;
            let mut cls_tree: metamodelica::Ref<ClassTree::ClassTree>;
            let mut ty: metamodelica::Ref<Type::NFType>;
            let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
            node = Type::complexNode(&(Type::arrayElementType(metamodelica::AsArg::as_arg(&__recordExp_ty))))?;
            cls_tree = Class::classTree(InstNode::getClass(node)?)?;
            let __pa0 = ::match_deref::match_deref! { match &(ClassTree::lookupElement(elementName.clone(), &cls_tree)?) {
                (__pa0, false) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            node = metamodelica::Own::own(__pa0);
            ty = InstNode::getType(node.clone())?;
            cref = ComponentRef::prefixCref(node, ty.clone(), metamodelica::nil(), __recordExp_cref.clone())?;
            ty = Type::liftArrayLeftList(ty, &(Type::arrayDims(__recordExp_ty.clone())));
            metamodelica::Ref::new(NFExpression::CREF { ty: ty, cref: cref })
        },
        Deref @ ARRAY { ty: Deref @ Type::ARRAY { elementType: Deref @ Type::COMPLEX { .. }, .. }, .. } if (var_field!((**recordExp).elements, NFExpression::ARRAY).clone().borrow().is_empty()) => {
            let mut node: metamodelica::Ref<InstNode::InstNode>;
            let mut cls: metamodelica::Ref<Class::NFClass>;
            let mut ty: metamodelica::Ref<Type::NFType>;
            let mut index: i32;
            node = Type::complexNode(&(Type::arrayElementType(var_field!((**recordExp).ty, NFExpression::ARRAY))))?;
            cls = InstNode::getClass(node)?;
            index = Class::lookupComponentIndex(elementName.clone(), cls.clone())?;
            ty = InstNode::getType(Class::nthComponent(index, cls)?)?;
            ty = Type::liftArrayLeftList(ty, &(Type::arrayDims(var_field!((**recordExp).ty, NFExpression::ARRAY).clone())));
            makeEmptyArray(ty)?
        },
        Deref @ ARRAY { ty: Deref @ Type::ARRAY { elementType: Deref @ Type::COMPLEX { .. }, .. }, literal: __recordExp_literal, .. } => {
            let mut node: metamodelica::Ref<InstNode::InstNode>;
            let mut ty: metamodelica::Ref<Type::NFType>;
            let mut index: i32;
            let mut arr: metamodelica::Array<metamodelica::Ref<NFExpression>>;
            node = Type::complexNode(&(Type::arrayElementType(var_field!((**recordExp).ty, NFExpression::ARRAY))))?;
            index = Class::lookupComponentIndex(elementName.clone(), InstNode::getClass(node)?)?;
            arr = Array::map(var_field!((**recordExp).elements, NFExpression::ARRAY).clone(), &({ let __pe_b0 = index; move |__pe_a1| nthRecordElement(__pe_b0.clone(), &__pe_a1) }))?;
            ty = Type::liftArrayLeft(typeOf(metamodelica::arrayGet(arr.clone(), 1)?), &(Dimension::fromInteger(metamodelica::arrayLength(arr.clone()), Prefixes::Variability::CONSTANT.clone())));
            makeArray(ty, arr.clone(), __recordExp_literal.clone())
        },
        Deref @ SUBSCRIPTED_EXP { exp: __recordExp_exp, split: __recordExp_split, subscripts: __recordExp_subscripts, .. } => {
            let mut ty: metamodelica::Ref<Type::NFType>;
            outExp = recordElement(elementName, metamodelica::AsArg::as_arg(&__recordExp_exp))?;
            ty = Type::subscript(typeOf(outExp.clone()), metamodelica::AsArg::as_arg(&__recordExp_subscripts), true)?;
            metamodelica::Ref::new(NFExpression::SUBSCRIPTED_EXP { exp: outExp, subscripts: __recordExp_subscripts.clone(), ty: ty, split: __recordExp_split.clone() })
        },
        Deref @ EMPTY { .. } => {
            return Err("fail")
        },
        _ => {
            let mut node: metamodelica::Ref<InstNode::InstNode>;
            let mut cls: metamodelica::Ref<Class::NFClass>;
            let mut ty: metamodelica::Ref<Type::NFType>;
            let mut index: i32;
            ty = typeOf(recordExp.clone());
            node = Type::complexNode(&(Type::arrayElementType(&ty)))?;
            cls = InstNode::getClass(node)?;
            index = Class::lookupComponentIndex(elementName.clone(), cls.clone())?;
            ty = Type::liftArrayLeftList(InstNode::getType(Class::nthComponent(index, cls)?)?, &(Type::arrayDims(ty)));
            metamodelica::Ref::new(NFExpression::RECORD_ELEMENT { recordExp: recordExp.clone(), index: index, fieldName: elementName.clone(), ty: ty })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn nthRecordElement(
    mut index: i32,
    mut recordExp: &metamodelica::Ref<NFExpression>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    outExp = (::match_deref::match_deref! { match recordExp {
        Deref @ RECORD { elements: __recordExp_elements, .. } => {
            (__recordExp_elements).get(index)?
        },
        Deref @ CREF { cref: __recordExp_cref, .. } => {
            let mut node: metamodelica::Ref<InstNode::InstNode>;
            node = Type::complexNode(&(Type::arrayElementType(&(typeOf(recordExp.clone())))))?;
            node = Class::nthComponent(index, InstNode::getClass(node)?)?;
            fromCref(ComponentRef::prefixCref(node.clone(), InstNode::getType(node)?, metamodelica::nil(), __recordExp_cref.clone())?, false)?
        },
        Deref @ ARRAY { ty: Deref @ Type::ARRAY { elementType: Deref @ Type::COMPLEX { .. }, .. }, .. } if (var_field!((**recordExp).elements, NFExpression::ARRAY).clone().borrow().is_empty()) => {
            makeEmptyArray(InstNode::getType(Class::nthComponent(index, InstNode::getClass(Type::complexNode(&(Type::arrayElementType(var_field!((**recordExp).ty, NFExpression::ARRAY))))?)?)?)?)?
        },
        Deref @ ARRAY { ty: __recordExp_ty, .. } => {
            let mut ty: metamodelica::Ref<Type::NFType>;
            let mut arr: metamodelica::Array<metamodelica::Ref<NFExpression>>;
            arr = Array::map(var_field!((**recordExp).elements, NFExpression::ARRAY).clone(), &({ let __pe_b0 = index; move |__pe_a1| nthRecordElement(__pe_b0.clone(), &__pe_a1) }))?;
            ty = Type::liftArrayLeft(typeOf(metamodelica::arrayGet(arr.clone(), 1)?), &(((Type::arrayDims(__recordExp_ty.clone()))).head().cloned()?));
            makeArray(ty, arr.clone(), false)
        },
        Deref @ RECORD_ELEMENT { ty: Deref @ Type::ARRAY { elementType: Deref @ Type::COMPLEX { .. }, .. }, .. } => {
            let mut node: metamodelica::Ref<InstNode::InstNode>;
            node = Type::complexNode(&(Type::arrayElementType(var_field!((**recordExp).ty, NFExpression::RECORD_ELEMENT))))?;
            node = Class::nthComponent(index, InstNode::getClass(node)?)?;
            metamodelica::Ref::new(NFExpression::RECORD_ELEMENT { recordExp: recordExp.clone(), index: index, fieldName: InstNode::name(&node)?, ty: Type::liftArrayLeftList(InstNode::getType(node)?, &(Type::arrayDims(var_field!((**recordExp).ty, NFExpression::RECORD_ELEMENT).clone()))) })
        },
        Deref @ SUBSCRIPTED_EXP { exp: __recordExp_exp, split: __recordExp_split, subscripts: __recordExp_subscripts, .. } => {
            let mut ty: metamodelica::Ref<Type::NFType>;
            outExp = nthRecordElement(index, metamodelica::AsArg::as_arg(&__recordExp_exp))?;
            ty = Type::subscript(typeOf(outExp.clone()), metamodelica::AsArg::as_arg(&__recordExp_subscripts), true)?;
            metamodelica::Ref::new(NFExpression::SUBSCRIPTED_EXP { exp: outExp, subscripts: __recordExp_subscripts.clone(), ty: ty, split: __recordExp_split.clone() })
        },
        Deref @ IF { condition: __recordExp_condition, falseBranch: __recordExp_falseBranch, trueBranch: __recordExp_trueBranch, .. } => {
            let mut trueBranch: metamodelica::Ref<NFExpression>;
            let mut falseBranch: metamodelica::Ref<NFExpression>;
            trueBranch = nthRecordElement(index, metamodelica::AsArg::as_arg(&__recordExp_trueBranch))?;
            falseBranch = nthRecordElement(index, metamodelica::AsArg::as_arg(&__recordExp_falseBranch))?;
            metamodelica::Ref::new(NFExpression::IF { ty: typeOf(trueBranch.clone()), condition: __recordExp_condition.clone(), trueBranch: trueBranch, falseBranch: falseBranch })
        },
        _ => {
            let mut node: metamodelica::Ref<InstNode::InstNode>;
            node = Type::complexNode(&(typeOf(recordExp.clone())))?;
            node = Class::nthComponent(index, InstNode::getClass(node)?)?;
            metamodelica::Ref::new(NFExpression::RECORD_ELEMENT { recordExp: recordExp.clone(), index: index, fieldName: InstNode::name(&node)?, ty: InstNode::getType(node)? })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn getRecordElements(
    mut exp: metamodelica::Ref<NFExpression>,
) -> Result<metamodelica::List<metamodelica::Ref<NFExpression>>> {
    let mut elements: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
    let mut ty: metamodelica::Ref<Type::NFType> = Type::arrayElementType(&(typeOf(exp.clone())));
    elements = (::match_deref::match_deref! { match &(ty) {
        Deref @ Type::COMPLEX { complexTy: complexTy @ Deref @ ComplexType::RECORD { .. }, .. } => {
            for mut i in ({let __s=metamodelica::arrayLength(var_field!((**complexTy).fields, ComplexType::NFComplexType::RECORD).clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
                elements = metamodelica::cons(recordElement(&(Record::Field::name(&({let __elt = (*metamodelica::index_checked(&var_field!((**complexTy).fields, ComplexType::NFComplexType::RECORD).borrow(), i)?).clone(); __elt}))), &exp)?, elements);
            }
            elements
        },
        _ => {
            elements
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(elements)
}

pub(crate) fn retype(mut exp: metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression> = exp;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ RANGE { start: __exp_start, step: __exp_step, stop: __exp_stop, ty: __exp_ty } => {
            assign_variant_field!(exp => NFExpression::RANGE; ty = TypeCheck::keepRangeSize(TypeCheck::getRangeType(__exp_start.clone(), __exp_step.clone(), __exp_stop.clone(), typeOf(__exp_start.clone()), &(Absyn::dummyInfo.clone()))?, __exp_ty.clone())?);
            ()
        },
        Deref @ CALL { call: Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => {
            assign_variant_field!(exp => NFExpression::CALL; call = Call::retype(var_field!((*exp).call, NFExpression::CALL).clone()));
            ()
        },
        _ => {
            let mut ty: metamodelica::Ref<Type::NFType>;
            ty = typeOf(exp.clone());
            if Type::isConditionalArray(&ty) {
                ty = Type::simplifyConditionalArray(ty);
                exp = setType(ty, exp)?;
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn nthEnumLiteral(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut n: i32,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression>;
    exp = metamodelica::Ref::new(NFExpression::ENUM_LITERAL {
        ty: ty.clone(),
        name: Type::nthEnumLiteral(&ty, n)?,
        index: n,
    });
    Ok(exp)
}

pub(crate) fn createIterationRanges(
    mut exp: metamodelica::Ref<NFExpression>,
    mut iterators: &metamodelica::List<(metamodelica::Ref<InstNode::InstNode>, metamodelica::Ref<NFExpression>)>,
) -> Result<(
    metamodelica::Ref<NFExpression>,
    metamodelica::List<metamodelica::Ref<NFExpression>>,
    metamodelica::List<Mutable::Mutable<metamodelica::Ref<NFExpression>>>,
)> {
    let mut exp: metamodelica::Ref<NFExpression> = exp;
    let mut ranges: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
    let mut iters: metamodelica::List<Mutable::Mutable<metamodelica::Ref<NFExpression>>> = metamodelica::nil();
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    let mut range: metamodelica::Ref<NFExpression>;
    let mut iter: Mutable::Mutable<metamodelica::Ref<NFExpression>>;
    for mut i in &**iterators {
        (node, range) = i.clone();
        iter = Mutable::create(metamodelica::Ref::new(NFExpression::INTEGER { value: 0 }));
        ranges = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
            for mut r in (ranges).into_iter().cloned() {
                let __x = replaceIterator(
                    r.clone(),
                    &node,
                    &(metamodelica::Ref::new(NFExpression::MUTABLE { exp: iter.clone() })),
                )?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        exp = replaceIterator(
            exp,
            &node,
            &(metamodelica::Ref::new(NFExpression::MUTABLE { exp: iter.clone() })),
        )?;
        iters = metamodelica::cons(iter, iters);
        ranges = metamodelica::cons(range, ranges);
    }
    Ok((exp, ranges, iters))
}

pub(crate) fn foldReduction(
    mut exp: metamodelica::Ref<NFExpression>,
    mut iterators: &metamodelica::List<(metamodelica::Ref<InstNode::InstNode>, metamodelica::Ref<NFExpression>)>,
    mut foldExp: metamodelica::Ref<NFExpression>,
    mut mapFn: &dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>>,
    mut foldFn: &dyn ::std::ops::Fn(
        metamodelica::Ref<NFExpression>,
        metamodelica::Ref<NFExpression>,
    ) -> Result<metamodelica::Ref<NFExpression>>,
) -> Result<metamodelica::Ref<NFExpression>> {
    pub type MapFn = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static,
    >;

    pub type FoldFn = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<NFExpression>,
                metamodelica::Ref<NFExpression>,
            ) -> Result<metamodelica::Ref<NFExpression>>
            + 'static,
    >;

    let mut result: metamodelica::Ref<NFExpression>;
    let mut e: metamodelica::Ref<NFExpression>;
    let mut ranges: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
    let mut iters: metamodelica::List<Mutable::Mutable<metamodelica::Ref<NFExpression>>> = metamodelica::nil();
    (e, ranges, iters) = createIterationRanges(exp, iterators)?;
    result = foldReduction2(e, &ranges, &iters, foldExp, mapFn, foldFn)?;
    Ok(result)
}

pub(crate) fn foldReduction2(
    mut exp: metamodelica::Ref<NFExpression>,
    mut ranges: &metamodelica::List<metamodelica::Ref<NFExpression>>,
    mut iterators: &metamodelica::List<Mutable::Mutable<metamodelica::Ref<NFExpression>>>,
    mut foldExp: metamodelica::Ref<NFExpression>,
    mut mapFn: &dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>>,
    mut foldFn: &dyn ::std::ops::Fn(
        metamodelica::Ref<NFExpression>,
        metamodelica::Ref<NFExpression>,
    ) -> Result<metamodelica::Ref<NFExpression>>,
) -> Result<metamodelica::Ref<NFExpression>> {
    pub type MapFn = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static,
    >;

    pub type FoldFn = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<NFExpression>,
                metamodelica::Ref<NFExpression>,
            ) -> Result<metamodelica::Ref<NFExpression>>
            + 'static,
    >;

    let mut result: metamodelica::Ref<NFExpression>;
    let mut range: metamodelica::Ref<NFExpression>;
    let mut value: metamodelica::Ref<NFExpression>;
    let mut ranges_rest: metamodelica::List<metamodelica::Ref<NFExpression>>;
    let mut iter: Mutable::Mutable<metamodelica::Ref<NFExpression>>;
    let mut iters_rest: metamodelica::List<Mutable::Mutable<metamodelica::Ref<NFExpression>>>;
    let mut range_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>;
    if (ranges).is_empty() {
        result = foldFn(foldExp, mapFn(exp)?)?;
    } else {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*ranges)) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        range = metamodelica::Own::own(__pa0);
        ranges_rest = metamodelica::Own::own(__pa1);
        range = Ceval::evalExp(range, &(Ceval::noTarget().clone()))?;
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &((*iterators)) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        iter = metamodelica::Own::own(__pa2);
        iters_rest = metamodelica::Own::own(__pa3);
        range_iter = ExpressionIterator::fromExp(range, false, false)?;
        result = foldExp;
        while ExpressionIterator::hasNext(&range_iter) {
            (range_iter, value) = ExpressionIterator::next(range_iter)?;
            Mutable::update(iter.clone(), value);
            result = foldReduction2(exp.clone(), &ranges_rest, &iters_rest, result, mapFn, foldFn)?;
        }
    }
    Ok(result)
}

pub(crate) fn isPure(mut exp: &metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut isPure: bool;
    isPure = (match &**exp {
        CREF { cref: __exp_cref, .. } => !(ComponentRef::isIterator(metamodelica::AsArg::as_arg(&__exp_cref))),
        CALL { call: __exp_call } => {
            (::match_deref::match_deref! { match &(AbsynUtil::pathFirstIdent(&(Call::functionName(metamodelica::AsArg::as_arg(&__exp_call))?))) {
                Deref @ "Connections" => false,
                Deref @ "cardinality" => false,
                _ => !(Call::isImpure(metamodelica::AsArg::as_arg(&__exp_call))?),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        }
        _ => true,
    });
    Ok(isPure)
}

pub fn containsCref(
    mut exp: metamodelica::Ref<NFExpression>,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<bool> {
    let mut b: bool;
    b = fold(
        exp,
        (std::sync::Arc::new({
            let __pe_b2 = cref.clone();
            move |__pe_a0, __pe_a1| isCrefEqual(&__pe_a0, __pe_a1, &__pe_b2)
        })
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, bool) -> Result<bool> + 'static>),
        false,
    )?;
    Ok(b)
}

pub(crate) fn isCrefEqual(
    mut exp: &metamodelica::Ref<NFExpression>,
    mut b: bool,
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<bool> {
    let mut b: bool = b;
    b = (::match_deref::match_deref! { match &((b, exp.clone())) {
        (false, Deref @ CREF { .. }) => ComponentRef::isEqual(var_field!((**exp).cref, NFExpression::CREF), cref)?,
        _ => b,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn containsCrefSet(
    mut exp: metamodelica::Ref<NFExpression>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<bool> {
    let mut b: bool;
    b = fold(
        exp,
        (std::sync::Arc::new({
            let __pe_b2 = set;
            move |__pe_a0, __pe_a1| isCrefEqualSet(&__pe_a0, __pe_a1, __pe_b2.clone())
        })
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>, bool) -> Result<bool> + 'static>),
        false,
    )?;
    Ok(b)
}

pub(crate) fn isCrefEqualSet(
    mut exp: &metamodelica::Ref<NFExpression>,
    mut b: bool,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<bool> {
    let mut b: bool = b;
    b = (::match_deref::match_deref! { match &((b, exp.clone())) {
        (false, Deref @ CREF { .. }) => UnorderedSet::contains(var_field!((**exp).cref, NFExpression::CREF).clone(), set)?,
        _ => b,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn filterSplitIndices(
    mut exp: metamodelica::Ref<NFExpression>,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression> = exp;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    exp = (match &*exp {
        SUBSCRIPTED_EXP {
            subscripts: __esc_subs,
            exp: __exp_exp,
            ty: __exp_ty,
            ..
        } => {
            subs = (*__esc_subs).clone();
            subs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
                for mut s in (subs.clone()).into_iter().cloned() {
                    if !(!(filterSplitIndices2(&(s.clone()), node)?)) {
                        continue;
                    }
                    let __x = s.clone();
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            if ((subs).is_empty()) {
                __exp_exp.clone()
            } else if (Type::isUnknown(metamodelica::AsArg::as_arg(&__exp_ty))) {
                metamodelica::Ref::new(NFExpression::SUBSCRIPTED_EXP {
                    exp: __exp_exp.clone(),
                    subscripts: subs.clone(),
                    ty: __exp_ty.clone(),
                    split: List::any(metamodelica::AsArg::as_arg(&subs), &move |__a0: metamodelica::Ref<
                        Subscript::NFSubscript,
                    >|
                          -> metamodelica::Result<
                        _,
                    > {
                        ::std::result::Result::Ok(Subscript::isSplit(&__a0))
                    })?,
                })
            } else {
                applySubscripts(metamodelica::AsArg::as_arg(&subs), __exp_exp.clone(), false)?
            }
        }
        _ => exp,
    });
    Ok(exp)
}

pub(crate) fn filterSplitIndices2(
    mut sub: &metamodelica::Ref<Subscript::NFSubscript>,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
) -> Result<bool> {
    let mut matching: bool;
    matching = (match &**sub {
        Subscript::SPLIT_INDEX { node: __sub_node, .. } => {
            InstNode::refEqual(&(InstNode::borrow(__sub_node.clone())?), node)?
        }
        Subscript::SPLIT_PROXY {
            parent: __sub_parent, ..
        } => InstNode::refEqual(&(InstNode::borrow(__sub_parent.clone())?), node)?,
        _ => false,
    });
    Ok(matching)
}

pub fn expandSplitIndices(mut exp: metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    outExp = (match &*exp {
        SUBSCRIPTED_EXP {
            exp: __exp_exp,
            subscripts: __exp_subscripts,
            ..
        } => applySubscripts(
            &(Subscript::expandSplitIndices(__exp_subscripts.clone(), &(metamodelica::nil()))?),
            __exp_exp.clone(),
            false,
        )?,
        CREF { cref: __exp_cref, .. } => {
            assign_variant_field!(exp => NFExpression::CREF; cref = ComponentRef::expandSplitSubscripts(__exp_cref.clone())?);
            exp
        }
        _ => exp,
    });
    Ok(outExp)
}

pub(crate) fn expandNonListedSplitIndices(
    mut exp: metamodelica::Ref<NFExpression>,
    mut indicesToKeep: &metamodelica::List<metamodelica::Ref<InstNode::InstNode>>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut outExp: metamodelica::Ref<NFExpression>;
    outExp = (match &*exp {
        SUBSCRIPTED_EXP {
            split: true,
            exp: __exp_exp,
            subscripts: __exp_subscripts,
            ..
        } => applySubscripts(
            &(Subscript::expandSplitIndices(__exp_subscripts.clone(), indicesToKeep)?),
            __exp_exp.clone(),
            false,
        )?,
        _ => exp,
    });
    Ok(outExp)
}

pub(crate) fn isSplitSubscriptedExp(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut split: bool;
    split = (match &**exp {
        SUBSCRIPTED_EXP { split: __esc_split, .. } => {
            split = (*__esc_split).clone();
            split.clone()
        }
        _ => false,
    });
    split
}

pub(crate) fn mapSplitExpressions(
    mut exp: metamodelica::Ref<NFExpression>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>>,
) -> Result<metamodelica::Ref<NFExpression>> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static,
    >;

    let mut outExp: metamodelica::Ref<NFExpression>;
    let mut osub_repls: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Subscript::NFSubscript>, metamodelica::Ref<NFExpression>>,
        >,
    >;
    let mut sub_repls: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Subscript::NFSubscript>, metamodelica::Ref<NFExpression>>,
    >;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut sub_exps: metamodelica::List<metamodelica::Ref<NFExpression>>;
    let mut dim_sizes: metamodelica::List<metamodelica::Ref<NFExpression>>;
    (outExp, osub_repls) = mapFold(
        exp.clone(),
        (std::sync::Arc::new(replaceSplitSubscripts)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<NFExpression>,
                        Option<
                            metamodelica::Ref<
                                UnorderedMap::UnorderedMap<
                                    metamodelica::Ref<Subscript::NFSubscript>,
                                    metamodelica::Ref<NFExpression>,
                                >,
                            >,
                        >,
                    ) -> Result<(
                        metamodelica::Ref<NFExpression>,
                        Option<
                            metamodelica::Ref<
                                UnorderedMap::UnorderedMap<
                                    metamodelica::Ref<Subscript::NFSubscript>,
                                    metamodelica::Ref<NFExpression>,
                                >,
                            >,
                        >,
                    )> + 'static,
            >),
        None,
    )?;
    if (osub_repls).is_none() {
        outExp = func(exp)?;
    } else {
        let __pa0 = ::match_deref::match_deref! { match &(osub_repls) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        sub_repls = metamodelica::Own::own(__pa0);
        subs = UnorderedMap::keyList(sub_repls.clone());
        sub_exps = UnorderedMap::valueList(sub_repls.clone());
        dim_sizes = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
            for mut s in (subs.clone()).into_iter().cloned() {
                let __x = Subscript::splitIndexDimExp(&(s.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        dim_sizes = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
            for mut d in (dim_sizes).into_iter().cloned() {
                let __x = (replaceSplitSubscripts(d.clone(), Some(sub_repls.clone()))?).0;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        outExp = mapSplitExpressions2(outExp, &dim_sizes, &sub_exps, func)?;
        outExp = applySubscripts(&subs, outExp, false)?;
    }
    Ok(outExp)
}

pub(crate) fn replaceSplitSubscripts(
    mut exp: metamodelica::Ref<NFExpression>,
    mut subRepls: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Subscript::NFSubscript>, metamodelica::Ref<NFExpression>>,
        >,
    >,
) -> Result<(
    metamodelica::Ref<NFExpression>,
    Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Subscript::NFSubscript>, metamodelica::Ref<NFExpression>>,
        >,
    >,
)> {
    let mut exp: metamodelica::Ref<NFExpression> = exp;
    let mut subRepls: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Subscript::NFSubscript>, metamodelica::Ref<NFExpression>>,
        >,
    > = subRepls;
    exp = (match &*exp {
        SUBSCRIPTED_EXP {
            split: true,
            exp: __exp_exp,
            subscripts: __exp_subscripts,
            ..
        } => {
            let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
            (subs, subRepls) = List::mapFold(
                metamodelica::AsArg::as_arg(&__exp_subscripts),
                &replaceSplitSubscripts2,
                subRepls,
            )?;
            applySubscripts(&subs, __exp_exp.clone(), false)?
        }
        _ => exp,
    });
    Ok((exp, subRepls))
}

pub(crate) fn replaceSplitSubscripts2(
    mut subscript: metamodelica::Ref<Subscript::NFSubscript>,
    mut subRepls: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Subscript::NFSubscript>, metamodelica::Ref<NFExpression>>,
        >,
    >,
) -> Result<(
    metamodelica::Ref<Subscript::NFSubscript>,
    Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Subscript::NFSubscript>, metamodelica::Ref<NFExpression>>,
        >,
    >,
)> {
    let mut subscript: metamodelica::Ref<Subscript::NFSubscript> = subscript;
    let mut subRepls: Option<
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Subscript::NFSubscript>, metamodelica::Ref<NFExpression>>,
        >,
    > = subRepls;
    let mut sub_exp: metamodelica::Ref<NFExpression>;
    let mut sub_repls: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Subscript::NFSubscript>, metamodelica::Ref<NFExpression>>,
    >;
    subscript = (match &*subscript {
        Subscript::SPLIT_INDEX { .. } => {
            if (subRepls).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(subRepls.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                sub_repls = metamodelica::Own::own(__pa0);
            } else {
                sub_repls = UnorderedMap::new(
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<Subscript::NFSubscript>| Subscript::hash(&__a0))
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<Subscript::NFSubscript>) -> Result<i32> + 'static,
                        >),
                    (std::sync::Arc::new(
                        move |__a0: metamodelica::Ref<Subscript::NFSubscript>,
                              __a1: metamodelica::Ref<Subscript::NFSubscript>| {
                            Subscript::isEqual(&__a0, &__a1)
                        },
                    )
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Subscript::NFSubscript>,
                                    metamodelica::Ref<Subscript::NFSubscript>,
                                ) -> Result<bool>
                                + 'static,
                        >),
                    1,
                );
                subRepls = Some(sub_repls.clone());
            }
            sub_exp = makeMutable(metamodelica::Ref::new(NFExpression::INTEGER { value: 0 }));
            sub_exp = UnorderedMap::tryAdd(subscript, sub_exp, sub_repls)?;
            metamodelica::Ref::new(Subscript::NFSubscript::INDEX { index: sub_exp })
        }
        _ => subscript,
    });
    Ok((subscript, subRepls))
}

pub(crate) fn mapSplitExpressions2(
    mut exp: metamodelica::Ref<NFExpression>,
    mut dimSizes: &metamodelica::List<metamodelica::Ref<NFExpression>>,
    mut subExps: &metamodelica::List<metamodelica::Ref<NFExpression>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>>,
) -> Result<metamodelica::Ref<NFExpression>> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static,
    >;

    let mut outExp: metamodelica::Ref<NFExpression>;
    let mut dim_size: metamodelica::Ref<NFExpression>;
    let mut rest_dims: metamodelica::List<metamodelica::Ref<NFExpression>>;
    let mut dim_size_int: i32;
    let mut sub_exp: metamodelica::Ref<NFExpression>;
    let mut rest_subs: metamodelica::List<metamodelica::Ref<NFExpression>>;
    let mut expl: metamodelica::Array<metamodelica::Ref<NFExpression>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    if (dimSizes).is_empty() {
        outExp = map(
            exp,
            (std::sync::Arc::new(mapSplitExpressions3)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>>
                        + 'static,
                >),
        )?;
        outExp = func(outExp)?;
    } else {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*dimSizes)) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        dim_size = metamodelica::Own::own(__pa0);
        rest_dims = metamodelica::Own::own(__pa1);
        dim_size_int = toInteger(&(Ceval::evalExp(dim_size, &(Ceval::noTarget().clone()))?))?;
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &((*subExps)) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        sub_exp = metamodelica::Own::own(__pa2);
        rest_subs = metamodelica::Own::own(__pa3);
        expl = metamodelica::arrayCreate(dim_size_int, exp.clone());
        for mut i in 1..=dim_size_int {
            updateMutable(&sub_exp, metamodelica::Ref::new(NFExpression::INTEGER { value: i }))?;
            unsafe {
                metamodelica::Dangerous::arrayInitSlot(
                    expl.clone(),
                    i,
                    mapSplitExpressions2(exp.clone(), &rest_dims, &rest_subs, func)?,
                )
            };
        }
        ty = typeOf(if (expl.clone().borrow().is_empty()) {
            exp
        } else {
            metamodelica::arrayGet(expl.clone(), 1)?
        });
        outExp = makeExpArray(
            expl.clone(),
            ty,
            Array::all(expl.clone(), &move |__a0: metamodelica::Ref<NFExpression>| {
                isLiteral(&__a0)
            })?,
        );
    }
    Ok(outExp)
}

pub(crate) fn mapSplitExpressions3(
    mut exp: metamodelica::Ref<NFExpression>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression> = exp;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    exp = (match &*exp {
        MUTABLE { exp: __exp_exp } => Mutable::access(__exp_exp.clone()),
        SUBSCRIPTED_EXP {
            subscripts: __esc_subs,
            exp: __exp_exp,
            ..
        } => {
            subs = (*__esc_subs).clone();
            applySubscripts(metamodelica::AsArg::as_arg(&subs), __exp_exp.clone(), false)?
        }
        _ => exp,
    });
    Ok(exp)
}

pub(crate) fn mapCrefScalars(
    mut crefExp: metamodelica::Ref<NFExpression>,
    mut mapFn: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<metamodelica::Ref<NFExpression>>
            + 'static,
    >,
) -> Result<metamodelica::Ref<NFExpression>> {
    pub type MapFn = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<metamodelica::Ref<NFExpression>>
            + 'static,
    >;

    let mut outExp: metamodelica::Ref<NFExpression>;
    (outExp, _) = ExpandExp::expand(crefExp, false, false)?;
    outExp = mapCrefScalars2(outExp, mapFn.clone())?;
    Ok(outExp)
}

pub(crate) fn mapCrefScalars2(
    mut exp: metamodelica::Ref<NFExpression>,
    mut mapFn: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<metamodelica::Ref<NFExpression>>
            + 'static,
    >,
) -> Result<metamodelica::Ref<NFExpression>> {
    pub type MapFn = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<metamodelica::Ref<NFExpression>>
            + 'static,
    >;

    let mut outExp: metamodelica::Ref<NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut literal: bool;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut arr: metamodelica::Array<metamodelica::Ref<NFExpression>>;
    outExp = (match &*exp {
        ARRAY { .. }
            if (!(var_field!((*exp).elements, NFExpression::ARRAY)
                .clone()
                .borrow()
                .is_empty())) =>
        {
            arr = Array::map(
                var_field!((*exp).elements, NFExpression::ARRAY).clone(),
                &({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                            ) -> Result<metamodelica::Ref<NFExpression>>
                            + 'static,
                    > = mapFn.clone();
                    move |__pe_a0| mapCrefScalars2(__pe_a0, __pe_b1.clone())
                }),
            )?;
            ty = typeOf(metamodelica::arrayGet(arr.clone(), 1)?);
            literal = Array::all(arr.clone(), &move |__a0: metamodelica::Ref<NFExpression>| {
                isLiteral(&__a0)
            })?;
            makeExpArray(arr.clone(), ty, literal)
        }
        CREF { cref: __exp_cref, .. } => mapFn(__exp_cref.clone())?,
        _ => exp.clone(),
    });
    Ok(outExp)
}

pub(crate) fn isFunctionPointer(mut exp: &metamodelica::Ref<NFExpression>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match exp {
        Deref @ CREF { ty: Deref @ Type::FUNCTION { .. }, .. } => true,
        Deref @ PARTIAL_FUNCTION_APPLICATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isClockOrSampleFunction(mut exp: &metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match exp {
        Deref @ CALL { call: call @ Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::ListNode::Cons { head: arg, tail: _ }, .. } } => {
            (::match_deref::match_deref! { match &(AbsynUtil::pathString(Function::Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL)), literal!("."), true, false)?) {
        Deref @ "sample" => !(isLiteral(metamodelica::AsArg::as_arg(&arg))?),
        Deref @ "subSample" => true,
        Deref @ "superSample" => true,
        Deref @ "shiftSample" => true,
        Deref @ "backSample" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        Deref @ CLKCONST { .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn isConnector(mut exp: &metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut res: bool;
    let mut node: metamodelica::Ref<InstNode::InstNode>;
    res = (match &**exp {
        CREF { cref: __exp_cref, .. } => {
            node = ComponentRef::node(metamodelica::AsArg::as_arg(&__exp_cref))?;
            InstNode::isComponent(&node)? && InstNode::isConnector(&node)?
        }
        _ => false,
    });
    Ok(res)
}

pub(crate) fn isComponentExpression(mut exp: &metamodelica::Ref<NFExpression>) -> Result<bool> {
    let mut res: bool;
    res = (match &**exp {
        CREF { cref: __exp_cref, .. } => {
            ComponentRef::isCref(metamodelica::AsArg::as_arg(&__exp_cref))
                && InstNode::isComponent(&(ComponentRef::node(metamodelica::AsArg::as_arg(&__exp_cref))?))?
        }
        _ => false,
    });
    Ok(res)
}

pub(crate) fn clone(mut exp: metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression> = exp;
    exp = (match &*exp {
        ARRAY { .. } => {
            assign_variant_field!(exp => NFExpression::ARRAY; elements = Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), &clone)?);
            exp
        }
        _ => mapShallow(
            exp,
            (std::sync::Arc::new(clone)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>>
                        + 'static,
                >),
        )?,
    });
    Ok(exp)
}

pub fn toJSON(mut exp: metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<JSON::JSON>> {
    fn dump_arg(mut name: ArcStr, mut arg: metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<JSON::JSON>> {
        let mut json: metamodelica::Ref<JSON::JSON> = JSON::emptyListObject();
        json = JSON::addPair(&(literal!("name")), &(JSON::makeString(name)), json)?;
        json = JSON::addPair(&(literal!("value")), &(toJSON(arg)?), json)?;
        Ok(json)
    }

    let mut json: metamodelica::Ref<JSON::JSON>;
    json = (match &*exp.clone() {
        INTEGER { value: __exp_value } => JSON::makeInteger(__exp_value.clone()),
        REAL { value: __exp_value } => JSON::makeNumber(__exp_value.clone()),
        STRING { value: __exp_value } => JSON::makeString(__exp_value.clone()),
        BOOLEAN { value: __exp_value } => JSON::makeBoolean(__exp_value.clone()),
        ENUM_LITERAL { index: __exp_index, .. } => {
            json = JSON::emptyListObject();
            json = JSON::addPair(
                &(literal!("$kind")),
                &(metamodelica::Ref::new(JSON::JSON::STRING {
                    r#str: literal!("enum"),
                })),
                json,
            )?;
            json = JSON::addPair(&(literal!("name")), &(JSON::makeString(toString(exp)?)), json)?;
            json = JSON::addPair(&(literal!("index")), &(JSON::makeInteger(__exp_index.clone())), json)?;
            json
        }
        CLKCONST { clk: __exp_clk } => ClockKind::toJSON(metamodelica::AsArg::as_arg(&__exp_clk))?,
        CREF { cref: __exp_cref, .. } => ComponentRef::toJSON(metamodelica::AsArg::as_arg(&__exp_cref))?,
        TYPENAME { ty: __exp_ty } => {
            json = JSON::emptyListObject();
            json = JSON::addPair(
                &(literal!("$kind")),
                &(metamodelica::Ref::new(JSON::JSON::STRING {
                    r#str: literal!("typename"),
                })),
                json,
            )?;
            json = JSON::addPair(
                &(literal!("name")),
                &(JSON::makeString(Type::typenameString(
                    &(Type::arrayElementType(metamodelica::AsArg::as_arg(&__exp_ty))),
                )?)),
                json,
            )?;
            json
        }
        ARRAY { .. } => JSON::makeList(
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<JSON::JSON>> = metamodelica::nil();
                for mut e in (var_field!((*exp).elements, NFExpression::ARRAY).clone())
                    .borrow()
                    .iter()
                {
                    let __x = toJSON(e.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        ),
        RANGE {
            start: __exp_start,
            step: __exp_step,
            stop: __exp_stop,
            ..
        } => {
            json = JSON::emptyListObject();
            json = JSON::addPair(
                &(literal!("$kind")),
                &(metamodelica::Ref::new(JSON::JSON::STRING {
                    r#str: literal!("range"),
                })),
                json,
            )?;
            json = JSON::addPair(&(literal!("start")), &(toJSON(__exp_start.clone())?), json)?;
            if (__exp_step).is_some() {
                json = JSON::addPair(
                    &(literal!("step")),
                    &(toJSON(Util::getOption(__exp_step.clone())?)?),
                    json,
                )?;
            }
            json = JSON::addPair(&(literal!("stop")), &(toJSON(__exp_stop.clone())?), json)?;
            json
        }
        TUPLE {
            elements: __exp_elements,
            ..
        } => {
            json = JSON::emptyListObject();
            json = JSON::addPair(
                &(literal!("$kind")),
                &(metamodelica::Ref::new(JSON::JSON::STRING {
                    r#str: literal!("tuple"),
                })),
                json,
            )?;
            json = JSON::addPair(
                &(literal!("elements")),
                &(JSON::makeList(
                    ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<JSON::JSON>> = metamodelica::nil();
                        for mut e in (__exp_elements.clone()).into_iter().cloned() {
                            let __x = toJSON(e.clone())?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                )),
                json,
            )?;
            json
        }
        RECORD {
            elements: __exp_elements,
            path: __exp_path,
            ..
        } => {
            json = JSON::emptyListObject();
            json = JSON::addPair(
                &(literal!("$kind")),
                &(metamodelica::Ref::new(JSON::JSON::STRING {
                    r#str: literal!("record"),
                })),
                json,
            )?;
            json = JSON::addPair(
                &(literal!("name")),
                &(JSON::makeString(AbsynUtil::pathString(__exp_path.clone(), literal!("."), true, false)?)),
                json,
            )?;
            json = JSON::addPair(
                &(literal!("elements")),
                &(JSON::makeList(
                    ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<JSON::JSON>> = metamodelica::nil();
                        for mut e in (__exp_elements.clone()).into_iter().cloned() {
                            let __x = toJSON(e.clone())?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                )),
                json,
            )?;
            json
        }
        CALL { call: __exp_call } => Call::toJSON(metamodelica::AsArg::as_arg(&__exp_call))?,
        SIZE {
            dimIndex: __exp_dimIndex,
            exp: __exp_exp,
        } => {
            json = JSON::emptyListObject();
            json = JSON::addPair(
                &(literal!("$kind")),
                &(metamodelica::Ref::new(JSON::JSON::STRING {
                    r#str: literal!("call"),
                })),
                json,
            )?;
            json = JSON::addPair(
                &(literal!("name")),
                &(metamodelica::Ref::new(JSON::JSON::STRING {
                    r#str: literal!("size"),
                })),
                json,
            )?;
            if (__exp_dimIndex).is_some() {
                json = JSON::addPair(
                    &(literal!("arguments")),
                    &(JSON::makeList(list![
                        toJSON(__exp_exp.clone())?,
                        toJSON(Util::getOption(__exp_dimIndex.clone())?)?
                    ])),
                    json,
                )?;
            } else {
                json = JSON::addPair(
                    &(literal!("arguments")),
                    &(JSON::makeArray(list![toJSON(__exp_exp.clone())?])),
                    json,
                )?;
            }
            json
        }
        BINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            operator: __exp_operator,
        } => {
            json = JSON::emptyListObject();
            json = JSON::addPair(
                &(literal!("$kind")),
                &(metamodelica::Ref::new(JSON::JSON::STRING {
                    r#str: literal!("binary_op"),
                })),
                json,
            )?;
            json = JSON::addPair(&(literal!("lhs")), &(toJSON(__exp_exp1.clone())?), json)?;
            json = JSON::addPair(
                &(literal!("op")),
                &(Operator::toJSON(metamodelica::AsArg::as_arg(&__exp_operator))?),
                json,
            )?;
            json = JSON::addPair(&(literal!("rhs")), &(toJSON(__exp_exp2.clone())?), json)?;
            json
        }
        UNARY {
            exp: __exp_exp,
            operator: __exp_operator,
        } => {
            json = JSON::emptyListObject();
            json = JSON::addPair(
                &(literal!("$kind")),
                &(metamodelica::Ref::new(JSON::JSON::STRING {
                    r#str: literal!("unary_op"),
                })),
                json,
            )?;
            json = JSON::addPair(
                &(literal!("op")),
                &(Operator::toJSON(metamodelica::AsArg::as_arg(&__exp_operator))?),
                json,
            )?;
            json = JSON::addPair(&(literal!("exp")), &(toJSON(__exp_exp.clone())?), json)?;
            json
        }
        LBINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            operator: __exp_operator,
        } => {
            json = JSON::emptyListObject();
            json = JSON::addPair(
                &(literal!("$kind")),
                &(metamodelica::Ref::new(JSON::JSON::STRING {
                    r#str: literal!("binary_op"),
                })),
                json,
            )?;
            json = JSON::addPair(&(literal!("lhs")), &(toJSON(__exp_exp1.clone())?), json)?;
            json = JSON::addPair(
                &(literal!("op")),
                &(Operator::toJSON(metamodelica::AsArg::as_arg(&__exp_operator))?),
                json,
            )?;
            json = JSON::addPair(&(literal!("rhs")), &(toJSON(__exp_exp2.clone())?), json)?;
            json
        }
        LUNARY {
            exp: __exp_exp,
            operator: __exp_operator,
        } => {
            json = JSON::emptyListObject();
            json = JSON::addPair(
                &(literal!("$kind")),
                &(metamodelica::Ref::new(JSON::JSON::STRING {
                    r#str: literal!("unary_op"),
                })),
                json,
            )?;
            json = JSON::addPair(
                &(literal!("op")),
                &(Operator::toJSON(metamodelica::AsArg::as_arg(&__exp_operator))?),
                json,
            )?;
            json = JSON::addPair(&(literal!("exp")), &(toJSON(__exp_exp.clone())?), json)?;
            json
        }
        RELATION {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            operator: __exp_operator,
            ..
        } => {
            json = JSON::emptyListObject();
            json = JSON::addPair(
                &(literal!("$kind")),
                &(metamodelica::Ref::new(JSON::JSON::STRING {
                    r#str: literal!("binary_op"),
                })),
                json,
            )?;
            json = JSON::addPair(&(literal!("lhs")), &(toJSON(__exp_exp1.clone())?), json)?;
            json = JSON::addPair(
                &(literal!("op")),
                &(Operator::toJSON(metamodelica::AsArg::as_arg(&__exp_operator))?),
                json,
            )?;
            json = JSON::addPair(&(literal!("rhs")), &(toJSON(__exp_exp2.clone())?), json)?;
            json
        }
        MULTARY {
            arguments: __exp_arguments,
            inv_arguments: __exp_inv_arguments,
            operator: __exp_operator,
        } => {
            json = JSON::emptyListObject();
            json = JSON::addPair(
                &(literal!("$kind")),
                &(metamodelica::Ref::new(JSON::JSON::STRING {
                    r#str: literal!("multary_op"),
                })),
                json,
            )?;
            json = JSON::addPair(
                &(literal!("args")),
                &(JSON::makeArray(
                    ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<JSON::JSON>> = metamodelica::nil();
                        for mut a in (__exp_arguments.clone()).into_iter().cloned() {
                            let __x = toJSON(a.clone())?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                )),
                json,
            )?;
            json = JSON::addPair(
                &(literal!("inv_args")),
                &(JSON::makeArray(
                    ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<JSON::JSON>> = metamodelica::nil();
                        for mut a in (__exp_inv_arguments.clone()).into_iter().cloned() {
                            let __x = toJSON(a.clone())?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                )),
                json,
            )?;
            json = JSON::addPair(
                &(literal!("op")),
                &(Operator::toJSON(metamodelica::AsArg::as_arg(&__exp_operator))?),
                json,
            )?;
            json
        }
        IF {
            condition: __exp_condition,
            falseBranch: __exp_falseBranch,
            trueBranch: __exp_trueBranch,
            ..
        } => {
            json = JSON::emptyListObject();
            json = JSON::addPair(
                &(literal!("$kind")),
                &(metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("if") })),
                json,
            )?;
            json = JSON::addPair(&(literal!("condition")), &(toJSON(__exp_condition.clone())?), json)?;
            json = JSON::addPair(&(literal!("true")), &(toJSON(__exp_trueBranch.clone())?), json)?;
            json = JSON::addPair(&(literal!("false")), &(toJSON(__exp_falseBranch.clone())?), json)?;
            json
        }
        CAST { exp: __exp_exp, .. } => toJSON(__exp_exp.clone())?,
        BOX { exp: __exp_exp } => toJSON(__exp_exp.clone())?,
        UNBOX { exp: __exp_exp, .. } => toJSON(__exp_exp.clone())?,
        SUBSCRIPTED_EXP {
            exp: __exp_exp,
            subscripts: __exp_subscripts,
            ..
        } => {
            json = JSON::emptyListObject();
            json = JSON::addPair(
                &(literal!("$kind")),
                &(metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("sub") })),
                json,
            )?;
            json = JSON::addPair(&(literal!("exp")), &(toJSON(__exp_exp.clone())?), json)?;
            json = JSON::addPair(
                &(literal!("subscripts")),
                &(Subscript::toJSONList(metamodelica::AsArg::as_arg(&__exp_subscripts))?),
                json,
            )?;
            json
        }
        TUPLE_ELEMENT {
            index: __exp_index,
            tupleExp: __exp_tupleExp,
            ..
        } => {
            json = JSON::emptyListObject();
            json = JSON::addPair(
                &(literal!("$kind")),
                &(metamodelica::Ref::new(JSON::JSON::STRING {
                    r#str: literal!("tuple_element"),
                })),
                json,
            )?;
            json = JSON::addPair(&(literal!("exp")), &(toJSON(__exp_tupleExp.clone())?), json)?;
            json = JSON::addPair(&(literal!("index")), &(JSON::makeInteger(__exp_index.clone())), json)?;
            json
        }
        RECORD_ELEMENT {
            fieldName: __exp_fieldName,
            index: __exp_index,
            recordExp: __exp_recordExp,
            ..
        } => {
            json = JSON::emptyListObject();
            json = JSON::addPair(
                &(literal!("$kind")),
                &(metamodelica::Ref::new(JSON::JSON::STRING {
                    r#str: literal!("record_element"),
                })),
                json,
            )?;
            json = JSON::addPair(&(literal!("exp")), &(toJSON(__exp_recordExp.clone())?), json)?;
            json = JSON::addPair(&(literal!("index")), &(JSON::makeInteger(__exp_index.clone())), json)?;
            json = JSON::addPair(&(literal!("field")), &(JSON::makeString(__exp_fieldName.clone())), json)?;
            json
        }
        PARTIAL_FUNCTION_APPLICATION {
            argNames: __exp_argNames,
            args: __exp_args,
            r#fn: __exp_fn,
            ..
        } => {
            json = JSON::emptyListObject();
            json = JSON::addPair(
                &(literal!("$kind")),
                &(metamodelica::Ref::new(JSON::JSON::STRING {
                    r#str: literal!("function"),
                })),
                json,
            )?;
            json = JSON::addPair(
                &(literal!("name")),
                &(JSON::makeString(ComponentRef::toString(metamodelica::AsArg::as_arg(&__exp_fn))?)),
                json,
            )?;
            json = JSON::addPair(
                &(literal!("arguments")),
                &(JSON::makeList(
                    ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<JSON::JSON>> = metamodelica::nil();
                        let __thr_src0 = __exp_args.clone();
                        let mut __thr_it0 = (&__thr_src0).into_iter();
                        let __thr_src1 = __exp_argNames.clone();
                        let mut __thr_it1 = (&__thr_src1).into_iter();
                        loop {
                            match (__thr_it0.next(), __thr_it1.next()) {
                                (Some(arg), Some(name)) => {
                                    let __x = dump_arg(name.clone(), arg.clone())?;
                                    __acc = cons(__x, __acc);
                                }
                                (None, None) => break,
                                _ => return Err("threaded for: ranges of unequal length"),
                            }
                        }
                        __acc.reverse()
                    }),
                )),
                json,
            )?;
            json
        }
        FILENAME {
            filename: __exp_filename,
        } => JSON::makeString(__exp_filename.clone()),
        _ => JSON::makeString(toString(exp)?),
    });
    Ok(json)
}

pub(crate) fn tupleElements(
    mut exp: metamodelica::Ref<NFExpression>,
) -> metamodelica::List<metamodelica::Ref<NFExpression>> {
    let mut expl: metamodelica::List<metamodelica::Ref<NFExpression>>;
    expl = (match &*exp {
        TUPLE {
            elements: __exp_elements,
            ..
        } => __exp_elements.clone(),
        _ => list![exp],
    });
    expl
}

pub(crate) fn wrapCall(
    mut exp: metamodelica::Ref<NFExpression>,
    mut fun: &dyn ::std::ops::Fn(metamodelica::Ref<Call::NFCall>) -> Result<metamodelica::Ref<Call::NFCall>>,
) -> Result<metamodelica::Ref<NFExpression>> {
    pub type callFun = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Call::NFCall>) -> Result<metamodelica::Ref<Call::NFCall>> + 'static,
    >;

    let mut exp: metamodelica::Ref<NFExpression> = exp;
    exp = (match &*exp {
        CALL { call: __exp_call } => {
            assign_variant_field!(exp => NFExpression::CALL; call = fun(__exp_call.clone())?);
            exp
        }
        _ => exp,
    });
    Ok(exp)
}

pub fn repairOperator(mut exp: metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression> = exp;
    exp = (match &*exp {
        BINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            operator: __exp_operator,
        } => {
            assign_variant_field!(exp => NFExpression::BINARY; operator = Operator::repairBinary(__exp_operator.clone(), typeOf(__exp_exp1.clone()), typeOf(__exp_exp2.clone()))?);
            exp
        }
        MULTARY {
            arguments: __exp_arguments,
            inv_arguments: __exp_inv_arguments,
            operator: __exp_operator,
        } => {
            assign_variant_field!(exp => NFExpression::MULTARY; operator = Operator::repairMultary(__exp_operator.clone(), ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Type::NFType>> = metamodelica::nil();
        for mut e in (listAppend(__exp_arguments.clone(), __exp_inv_arguments.clone())).into_iter().cloned() {
            let __x = typeOf(e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?);
            exp
        }
        _ => exp,
    });
    Ok(exp)
}

pub(crate) fn makeUnary(
    mut op: metamodelica::Ref<Operator::NFOperator>,
    mut exp: metamodelica::Ref<NFExpression>,
) -> metamodelica::Ref<NFExpression> {
    let mut unaryExp: metamodelica::Ref<NFExpression>;
    if op.op.clone() == Operator::Op::ADD.clone() {
        unaryExp = exp;
    } else if op.op.clone() == Operator::Op::UMINUS.clone() {
        unaryExp = negate(exp);
    } else {
        unaryExp = metamodelica::Ref::new(NFExpression::UNARY { operator: op, exp: exp });
    }
    unaryExp
}

pub fn replaceLiteral(
    mut exp: metamodelica::Ref<NFExpression>,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<NFExpression>, i32>>,
    mut idx_ptr: Pointer::Pointer<i32>,
) -> Result<metamodelica::Ref<NFExpression>> {
    fn replace(
        mut exp: metamodelica::Ref<NFExpression>,
        mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<NFExpression>, i32>>,
        mut idx_ptr: Pointer::Pointer<i32>,
    ) -> Result<metamodelica::Ref<NFExpression>> {
        let mut exp: metamodelica::Ref<NFExpression> = exp;
        let mut idx: i32;
        let mut idx_opt: Option<i32>;
        idx_opt = UnorderedMap::get(exp.clone(), map.clone())?;
        if (idx_opt).is_some() {
            idx = Util::getOption(idx_opt)?;
        } else {
            idx = Pointer::access(idx_ptr.clone());
            Pointer::update(idx_ptr, idx + 1);
            UnorderedMap::add(exp.clone(), idx, map)?;
        }
        exp = metamodelica::Ref::new(NFExpression::SHARED_LITERAL { index: idx, exp: exp });
        Ok(exp)
    }

    let mut exp: metamodelica::Ref<NFExpression> = exp;
    exp = (match &*exp {
        SHARED_LITERAL { .. } => exp.clone(),
        ARRAY { .. } if (isLiteralReplace(&exp)?) => replace(
            replaceLiteralArrayElements(exp.clone(), map.clone(), idx_ptr.clone())?,
            map,
            idx_ptr,
        )?,
        RECORD {
            elements: __exp_elements,
            ..
        } if (isLiteralReplace(&exp)?) => {
            assign_variant_field!(exp => NFExpression::RECORD; elements = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<NFExpression>> = metamodelica::nil();
                for mut elem in (__exp_elements.clone()).into_iter().cloned() {
                    let __x = replaceLiteral(elem.clone(), map.clone(), idx_ptr.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            replace(exp.clone(), map, idx_ptr)?
        }
        _ if (isLiteralReplace(&exp)?) => replace(exp.clone(), map, idx_ptr)?,
        _ => mapShallow(
            exp.clone(),
            (std::sync::Arc::new({
                let __pe_b1 = map;
                let __pe_b2 = idx_ptr;
                move |__pe_a0| replaceLiteral(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>>
                        + 'static,
                >),
        )?,
    });
    Ok(exp)
}

pub(crate) fn replaceLiteralArrayElements(
    mut exp: metamodelica::Ref<NFExpression>,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<NFExpression>, i32>>,
    mut idx_ptr: Pointer::Pointer<i32>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression> = exp;
    exp = (match &*exp {
        ARRAY { .. } => {
            assign_variant_field!(exp => NFExpression::ARRAY; elements = Array::map(var_field!((*exp).elements, NFExpression::ARRAY).clone(), &({ let __pe_b1 = map; let __pe_b2 = idx_ptr; move |__pe_a0| replaceLiteralArrayElements(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?);
            exp
        }
        _ => replaceLiteral(exp, map, idx_ptr)?,
    });
    Ok(exp)
}

pub(crate) fn replaceCrefWithBinding(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut exp: metamodelica::Ref<NFExpression>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static,
    >,
) -> Result<metamodelica::Ref<NFExpression>> {
    pub type recurse = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static,
    >;

    '__tco: loop {
        let mut e: metamodelica::Ref<NFExpression>;
        ::match_deref::match_deref! { match &(InstNode::getBindingExpOpt(&(ComponentRef::node(&cref)?))?) {
            Some(__esc_e @ Deref @ INTEGER { .. }) => {
                e = (*__esc_e).clone();
                return Ok(e.clone())
            },
            Some(__esc_e @ Deref @ CREF { .. }) => {
                e = (*__esc_e).clone();
                { (cref, exp, func) = (var_field!((*e).cref, NFExpression::CREF).clone(), e.clone(), func.clone()); continue '__tco; }
            },
            Some(Deref @ SUBSCRIPTED_EXP { exp: __esc_e @ Deref @ INTEGER { .. }, .. }) => {
                e = (*__esc_e).clone();
                return Ok(e.clone())
            },
            Some(Deref @ SUBSCRIPTED_EXP { exp: __esc_e @ Deref @ CREF { .. }, .. }) => {
                e = (*__esc_e).clone();
                { (cref, exp, func) = (var_field!((*e).cref, NFExpression::CREF).clone(), e.clone(), func.clone()); continue '__tco; }
            },
            Some(__esc_e) => {
                e = (*__esc_e).clone();
                return Ok(map(e.clone(), func.clone())?)
            },
            _ => return Ok(exp),
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn replaceResizableParameterWithOriginal(
    mut exp: metamodelica::Ref<NFExpression>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression> = exp;
    exp = (match &*exp.clone() {
        CREF { cref: __exp_cref, .. } if (ComponentRef::isResizable(metamodelica::AsArg::as_arg(&__exp_cref))?) => {
            replaceCrefWithBinding(
                __exp_cref.clone(),
                exp,
                (std::sync::Arc::new(replaceResizableParameterWithOriginal)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>>
                            + 'static,
                    >),
            )?
        }
        _ => exp,
    });
    Ok(exp)
}

pub fn replaceResizableParameter(mut exp: metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> {
    let mut exp: metamodelica::Ref<NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ CREF { cref: Deref @ ComponentRef::CREF { .. }, .. } if (InstNode::isVar(&(ComponentRef::node(var_field!((*exp).cref, NFExpression::CREF))?)) && ComponentRef::isResizable(var_field!((*exp).cref, NFExpression::CREF))?) => {
            let mut v: i32;
            (::match_deref::match_deref! { match &(Pointer::access(PointerWeak::upgrade(InstNode::varPointer(&(ComponentRef::node(var_field!((*exp).cref, NFExpression::CREF))?))?)?)) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { varKind: Deref @ VariableKind::PARAMETER { resize_value: Some(__esc_v) }, .. }, .. } => {
            v = (*__esc_v).clone();
            metamodelica::Ref::new(NFExpression::INTEGER { value: v.clone() })
        },
        _ => replaceCrefWithBinding(var_field!((*exp).cref, NFExpression::CREF).clone(), exp.clone(), (std::sync::Arc::new(replaceResizableParameter) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static>))?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        Deref @ CREF { cref: __exp_cref, .. } if (ComponentRef::isResizable(metamodelica::AsArg::as_arg(&__exp_cref))?) => {
            replaceCrefWithBinding(__exp_cref.clone(), exp.clone(), (std::sync::Arc::new(replaceResizableParameter) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFExpression>) -> Result<metamodelica::Ref<NFExpression>> + 'static>))?
        },
        _ => {
            exp.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn mulResultType(
    mut tl: metamodelica::Ref<Type::NFType>,
    mut tr: metamodelica::Ref<Type::NFType>,
) -> metamodelica::Ref<Type::NFType> {
    let mut tres: metamodelica::Ref<Type::NFType>;
    if Type::isArray(&tl) && Type::isArray(&tr) {
        tres = tl;
    } else if Type::isArray(&tl) {
        tres = tl;
    } else if Type::isArray(&tr) {
        tres = tr;
    } else {
        tres = tl;
    }
    tres
}

pub(crate) fn mmul(
    mut lhs: metamodelica::Ref<NFExpression>,
    mut rhs: metamodelica::Ref<NFExpression>,
    mut baseOp: &metamodelica::Ref<Operator::NFOperator>,
) -> Result<metamodelica::Ref<NFExpression>> {
    let mut prod: metamodelica::Ref<NFExpression>;
    let mut tl: metamodelica::Ref<Type::NFType> = typeOf(lhs.clone());
    let mut tr: metamodelica::Ref<Type::NFType> = typeOf(rhs.clone());
    let mut lArr: bool = Type::isArray(&tl);
    let mut rArr: bool = Type::isArray(&tr);
    let mut sizeClass: Operator::SizeClassification;
    let mut resTy: metamodelica::Ref<Type::NFType>;
    let mut op: metamodelica::Ref<Operator::NFOperator>;
    if !(lArr) && !(rArr) {
        sizeClass = Operator::SizeClassification::SCALAR.clone();
    } else if !(lArr) && rArr {
        sizeClass = Operator::SizeClassification::SCALAR_ARRAY.clone();
    } else if lArr && !(rArr) {
        sizeClass = Operator::SizeClassification::ARRAY_SCALAR.clone();
    } else {
        sizeClass = Operator::SizeClassification::ELEMENT_WISE.clone();
    }
    resTy = mulResultType(tl, tr);
    op = Operator::fromClassification((Operator::MathClassification::MULTIPLICATION.clone(), sizeClass), resTy)?;
    prod = metamodelica::Ref::new(NFExpression::BINARY {
        exp1: lhs,
        operator: op,
        exp2: rhs,
    });
    Ok(prod)
}

pub fn productOfListExceptSelf(
    mut arguments: &metamodelica::List<metamodelica::Ref<NFExpression>>,
    mut mulOp: &metamodelica::Ref<Operator::NFOperator>,
) -> Result<metamodelica::List<metamodelica::Ref<NFExpression>>> {
    let mut products: metamodelica::List<metamodelica::Ref<NFExpression>>;
    let mut n: i32 = ((arguments).len() as i32);
    let mut argsArr: metamodelica::Array<metamodelica::Ref<NFExpression>>;
    let mut pref: metamodelica::Array<metamodelica::Ref<NFExpression>>;
    let mut res: metamodelica::Array<metamodelica::Ref<NFExpression>>;
    let mut i: i32;
    let mut rightProd: metamodelica::Ref<NFExpression>;
    let mut baseTy: metamodelica::Ref<Type::NFType> = mulOp.ty.clone();
    let mut elTy: metamodelica::Ref<Type::NFType>;
    if n == 0 {
        products = metamodelica::nil();
        return Ok(products);
    }
    elTy = if (Type::isArray(&baseTy)) {
        Type::arrayElementType(&baseTy)
    } else {
        baseTy
    };
    argsArr = arrayCreate(n, makeOne(&elTy)?);
    i = 1;
    for mut a in &**arguments {
        {
            let __cell0 = a.clone();
            let __idx0 = i;
            *metamodelica::index_mut_checked(&mut argsArr.clone().borrow_mut(), __idx0)? = __cell0;
        }
        i = i + 1;
    }
    pref = arrayCreate(n, makeOne(&elTy)?);
    res = arrayCreate(n, makeOne(&elTy)?);
    for mut i in 2..=n {
        {
            let __cell1 = mmul(
                ({
                    let __elt = (*metamodelica::index_checked(&pref.borrow(), i - 1)?).clone();
                    __elt
                }),
                ({
                    let __elt = (*metamodelica::index_checked(&argsArr.borrow(), i - 1)?).clone();
                    __elt
                }),
                mulOp,
            )?;
            let __idx1 = i;
            *metamodelica::index_mut_checked(&mut pref.clone().borrow_mut(), __idx1)? = __cell1;
        }
    }
    rightProd = makeOne(&elTy)?;
    for mut i in ({
        let __s = n;
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        {
            let __cell2 = mmul(
                ({
                    let __elt = (*metamodelica::index_checked(&pref.borrow(), i)?).clone();
                    __elt
                }),
                rightProd.clone(),
                mulOp,
            )?;
            let __idx2 = i;
            *metamodelica::index_mut_checked(&mut res.clone().borrow_mut(), __idx2)? = __cell2;
        }
        rightProd = mmul(
            rightProd,
            ({
                let __elt = (*metamodelica::index_checked(&argsArr.borrow(), i)?).clone();
                __elt
            }),
            mulOp,
        )?;
    }
    products = metamodelica::nil();
    for mut i in ({
        let __s = n;
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        products = metamodelica::cons(
            SimplifyExp::simplify(
                ({
                    let __elt = (*metamodelica::index_checked(&res.borrow(), i)?).clone();
                    __elt
                }),
                false,
            )?,
            products,
        );
    }
    Ok(products)
}
