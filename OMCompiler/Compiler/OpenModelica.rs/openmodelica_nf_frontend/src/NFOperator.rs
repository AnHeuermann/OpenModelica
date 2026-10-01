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

use crate::NFType as Type;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::JSON;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct NFOperator {
    pub ty: metamodelica::Ref<Type::NFType>,
    pub op: Op,
}

impl metamodelica::gc::MMTrace for NFOperator {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.ty, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.op, __mmv)?;
        Ok(())
    }
}
impl Default for NFOperator {
    fn default() -> Self {
        Self {
            ty: Default::default(),
            op: Default::default(),
        }
    }
}

pub type OPERATOR = NFOperator;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum Op {
    ADD = 1,
    SUB = 2,
    MUL = 3,
    DIV = 4,
    POW = 5,
    ADD_EW = 6,
    SUB_EW = 7,
    MUL_EW = 8,
    DIV_EW = 9,
    POW_EW = 10,
    ADD_SCALAR_ARRAY = 11,
    ADD_ARRAY_SCALAR = 12,
    SUB_SCALAR_ARRAY = 13,
    SUB_ARRAY_SCALAR = 14,
    MUL_SCALAR_ARRAY = 15,
    MUL_ARRAY_SCALAR = 16,
    MUL_VECTOR_MATRIX = 17,
    MUL_MATRIX_VECTOR = 18,
    SCALAR_PRODUCT = 19,
    MATRIX_PRODUCT = 20,
    DIV_SCALAR_ARRAY = 21,
    DIV_ARRAY_SCALAR = 22,
    POW_SCALAR_ARRAY = 23,
    POW_ARRAY_SCALAR = 24,
    POW_MATRIX = 25,
    UMINUS = 26,
    AND = 27,
    OR = 28,
    NOT = 29,
    LESS = 30,
    LESSEQ = 31,
    GREATER = 32,
    GREATEREQ = 33,
    EQUAL = 34,
    NEQUAL = 35,
    USERDEFINED = 36,
}
impl PartialOrd for Op {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Op {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for Op {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for Op {
    fn default() -> Self {
        Self::ADD
    }
}

pub(crate) fn compare(mut op1: &metamodelica::Ref<NFOperator>, mut op2: &metamodelica::Ref<NFOperator>) -> i32 {
    let mut comp: i32;
    let mut o1: Op = op1.op.clone();
    let mut o2: Op = op2.op.clone();
    comp = Util::intCompare(((o1) as i32), ((o2) as i32));
    comp
}

pub fn invert(mut operator: metamodelica::Ref<NFOperator>) -> Result<metamodelica::Ref<NFOperator>> {
    let mut operator: metamodelica::Ref<NFOperator> = operator;
    assign_field!(
        operator.op = (match operator.op.clone() {
            Op::ADD => Op::SUB.clone(),
            Op::SUB => Op::ADD.clone(),
            Op::MUL => Op::DIV.clone(),
            Op::DIV => Op::MUL.clone(),
            Op::ADD_EW => Op::SUB_EW.clone(),
            Op::SUB_EW => Op::ADD_EW.clone(),
            Op::MUL_EW => Op::DIV_EW.clone(),
            Op::DIV_EW => Op::MUL_EW.clone(),
            Op::ADD_SCALAR_ARRAY => Op::SUB_SCALAR_ARRAY.clone(),
            Op::ADD_ARRAY_SCALAR { .. } => Op::SUB_ARRAY_SCALAR.clone(),
            Op::SUB_SCALAR_ARRAY { .. } => Op::ADD_SCALAR_ARRAY.clone(),
            Op::SUB_ARRAY_SCALAR => Op::ADD_ARRAY_SCALAR.clone(),
            Op::MUL_SCALAR_ARRAY => Op::DIV_SCALAR_ARRAY.clone(),
            Op::MUL_ARRAY_SCALAR { .. } => Op::DIV_ARRAY_SCALAR.clone(),
            Op::DIV_SCALAR_ARRAY { .. } => Op::MUL_SCALAR_ARRAY.clone(),
            Op::DIV_ARRAY_SCALAR { .. } => Op::MUL_ARRAY_SCALAR.clone(),
            Op::LESS => Op::GREATEREQ.clone(),
            Op::LESSEQ => Op::GREATER.clone(),
            Op::GREATER => Op::LESSEQ.clone(),
            Op::GREATEREQ => Op::LESS.clone(),
            Op::EQUAL => Op::EQUAL.clone(),
            Op::NEQUAL => Op::NEQUAL.clone(),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFOperator.invert"));
                        __mm_s.push_str(&*literal!("Failed! Don't know how to invert: "));
                        __mm_s.push_str(&*symbol(&operator, &(literal!(" ")))?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        })
    );
    Ok(operator)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum TypeRestriction {
    SCALAR = 1,
    VECTOR = 2,
    MATRIX = 3,
    ARRAY = 4,
    OTHER = 5,
}
impl PartialOrd for TypeRestriction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for TypeRestriction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for TypeRestriction {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub(crate) fn typeRestriction(mut ty: &metamodelica::Ref<Type::NFType>) -> Result<TypeRestriction> {
    let mut restriction: TypeRestriction;
    if Type::isScalar(ty) {
        restriction = TypeRestriction::SCALAR.clone();
    } else if Type::isVector(ty)? {
        restriction = TypeRestriction::VECTOR.clone();
    } else if Type::isMatrix(ty)? {
        restriction = TypeRestriction::MATRIX.clone();
    } else if Type::isArray(ty) {
        restriction = TypeRestriction::ARRAY.clone();
    } else {
        restriction = TypeRestriction::OTHER.clone();
    }
    Ok(restriction)
}

pub(crate) fn repairMultary(
    mut operator: metamodelica::Ref<NFOperator>,
    mut types: metamodelica::List<metamodelica::Ref<Type::NFType>>,
) -> Result<metamodelica::Ref<NFOperator>> {
    fn tplLt(
        mut tpl1: (TypeRestriction, metamodelica::Ref<Type::NFType>),
        mut tpl2: (TypeRestriction, metamodelica::Ref<Type::NFType>),
    ) -> bool {
        let mut b: bool = Util::tuple21(tpl1.clone()) < Util::tuple21(tpl2.clone());
        b
    }

    let mut operator: metamodelica::Ref<NFOperator> = operator;
    let mut mc: MathClassification = getMathClassification(&operator)?;
    let mut sc: SizeClassification;
    let mut lst: metamodelica::List<(TypeRestriction, metamodelica::Ref<Type::NFType>)>;
    let mut min_: (TypeRestriction, metamodelica::Ref<Type::NFType>);
    let mut max_: (TypeRestriction, metamodelica::Ref<Type::NFType>);
    let mut ty: metamodelica::Ref<Type::NFType>;
    lst = ({
        let mut __acc: metamodelica::List<(TypeRestriction, metamodelica::Ref<Type::NFType>)> = metamodelica::nil();
        for mut t in (types.clone()).into_iter().cloned() {
            let __x = (typeRestriction(&(t.clone()))?, t.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    min_ = List::minElement(
        &lst,
        &fnptr!(
            tplLt,
            (TypeRestriction, metamodelica::Ref<Type::NFType>),
            (TypeRestriction, metamodelica::Ref<Type::NFType>)
        ),
    )?;
    max_ = List::maxElement(
        &lst,
        &fnptr!(
            tplLt,
            (TypeRestriction, metamodelica::Ref<Type::NFType>),
            (TypeRestriction, metamodelica::Ref<Type::NFType>)
        ),
    )?;
    (sc, ty) = (::match_deref::match_deref! { match &((min_, max_)) {
        ((TypeRestriction::SCALAR, _), (TypeRestriction::SCALAR, __esc_ty)) => {
            ty = (*__esc_ty).clone();
            (SizeClassification::SCALAR.clone(), ty.clone())
        },
        ((TypeRestriction::SCALAR, _), (_, __esc_ty)) => {
            ty = (*__esc_ty).clone();
            (SizeClassification::SCALAR_ARRAY.clone(), ty.clone())
        },
        ((TypeRestriction::VECTOR { .. }, _), (TypeRestriction::VECTOR { .. }, __esc_ty)) => {
            ty = (*__esc_ty).clone();
            (SizeClassification::ELEMENT_WISE.clone(), ty.clone())
        },
        ((TypeRestriction::VECTOR { .. }, _), (TypeRestriction::MATRIX { .. }, __esc_ty)) => {
            ty = (*__esc_ty).clone();
            (SizeClassification::VECTOR_MATRIX.clone(), ty.clone())
        },
        ((TypeRestriction::MATRIX { .. }, _), (TypeRestriction::MATRIX { .. }, __esc_ty)) => {
            ty = (*__esc_ty).clone();
            (SizeClassification::ELEMENT_WISE.clone(), ty.clone())
        },
        ((TypeRestriction::ARRAY { .. }, _), (TypeRestriction::ARRAY { .. }, __esc_ty)) => {
            ty = (*__esc_ty).clone();
            (SizeClassification::ELEMENT_WISE.clone(), ty.clone())
        },
        _ => {
            Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFOperator.repairMultary")); __mm_s.push_str(&*literal!(" failed because the multary arguments have incompatible sizes: ")); __mm_s.push_str(&*List::toString(types, &move |__a0: metamodelica::Ref<Type::NFType>| Type::toString(&__a0), List::Style::FLAT_CURLY.clone())?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFOperator.mo")))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    operator = fromClassification((mc, sc), ty)?;
    Ok(operator)
}

pub(crate) fn repairBinary(
    mut operator: metamodelica::Ref<NFOperator>,
    mut ty1: metamodelica::Ref<Type::NFType>,
    mut ty2: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<NFOperator>> {
    let mut operator: metamodelica::Ref<NFOperator> = operator;
    let mut mc: MathClassification = getMathClassification(&operator)?;
    let mut sc: SizeClassification;
    let mut ty: metamodelica::Ref<Type::NFType>;
    (sc, ty) = (match (typeRestriction(&ty1)?, typeRestriction(&ty2)?) {
        (TypeRestriction::SCALAR, TypeRestriction::SCALAR) => (SizeClassification::SCALAR.clone(), ty1),
        (TypeRestriction::SCALAR, mut r2) if (r2 > TypeRestriction::SCALAR.clone()) => {
            (SizeClassification::SCALAR_ARRAY.clone(), ty2)
        }
        (mut r1, TypeRestriction::SCALAR) if (r1 > TypeRestriction::SCALAR.clone()) => {
            (SizeClassification::ARRAY_SCALAR.clone(), ty1)
        }
        (TypeRestriction::VECTOR { .. }, TypeRestriction::MATRIX { .. }) => {
            (SizeClassification::VECTOR_MATRIX.clone(), ty1)
        }
        (TypeRestriction::MATRIX { .. }, TypeRestriction::VECTOR { .. }) => {
            (SizeClassification::MATRIX_VECTOR.clone(), ty2)
        }
        (mut r1, mut r2) if (r1 == r2) => (getSizeClassification(&operator)?, ty1),
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFOperator.repairBinary"));
                    __mm_s.push_str(&*literal!(
                        " failed because the binary arguments have incompatible sizes: "
                    ));
                    __mm_s.push_str(&*Type::toString(&ty1)?);
                    __mm_s.push_str(&*literal!(", "));
                    __mm_s.push_str(&*Type::toString(&ty2)?);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFOperator.mo")),
            )?;
            return Err("fail");
        }
    });
    operator = fromClassification((mc, sc), ty)?;
    Ok(operator)
}

pub(crate) fn isLogical(mut operator: &metamodelica::Ref<NFOperator>) -> bool {
    let mut b: bool;
    b = (match operator.op.clone() {
        Op::AND => true,
        Op::OR => true,
        Op::NOT => true,
        _ => false,
    });
    b
}

pub(crate) fn isRelational(mut operator: &metamodelica::Ref<NFOperator>) -> bool {
    let mut b: bool;
    b = (match operator.op.clone() {
        Op::LESS => true,
        Op::LESSEQ => true,
        Op::GREATER => true,
        Op::GREATEREQ => true,
        Op::EQUAL => true,
        Op::NEQUAL => true,
        _ => false,
    });
    b
}

pub(crate) fn isScalarProduct(mut operator: &metamodelica::Ref<NFOperator>) -> bool {
    let mut b: bool;
    b = (match operator.op.clone() {
        Op::SCALAR_PRODUCT => true,
        _ => false,
    });
    b
}

pub(crate) fn fromAbsyn(mut inOperator: Absyn::Operator) -> metamodelica::Ref<NFOperator> {
    let mut outOperator: metamodelica::Ref<NFOperator>;
    let mut op: Op;
    op = (match inOperator {
        Absyn::Operator::ADD { .. } => Op::ADD.clone(),
        Absyn::Operator::SUB { .. } => Op::SUB.clone(),
        Absyn::Operator::MUL { .. } => Op::MUL.clone(),
        Absyn::Operator::DIV { .. } => Op::DIV.clone(),
        Absyn::Operator::POW { .. } => Op::POW.clone(),
        Absyn::Operator::ADD_EW { .. } => Op::ADD_EW.clone(),
        Absyn::Operator::SUB_EW { .. } => Op::SUB_EW.clone(),
        Absyn::Operator::MUL_EW { .. } => Op::MUL_EW.clone(),
        Absyn::Operator::DIV_EW { .. } => Op::DIV_EW.clone(),
        Absyn::Operator::POW_EW { .. } => Op::POW_EW.clone(),
        Absyn::Operator::UPLUS { .. } => Op::ADD.clone(),
        Absyn::Operator::UPLUS_EW { .. } => Op::ADD.clone(),
        Absyn::Operator::UMINUS { .. } => Op::UMINUS.clone(),
        Absyn::Operator::UMINUS_EW { .. } => Op::UMINUS.clone(),
        Absyn::Operator::AND { .. } => Op::AND.clone(),
        Absyn::Operator::OR { .. } => Op::OR.clone(),
        Absyn::Operator::NOT { .. } => Op::NOT.clone(),
        Absyn::Operator::LESS { .. } => Op::LESS.clone(),
        Absyn::Operator::LESSEQ { .. } => Op::LESSEQ.clone(),
        Absyn::Operator::GREATER { .. } => Op::GREATER.clone(),
        Absyn::Operator::GREATEREQ { .. } => Op::GREATEREQ.clone(),
        Absyn::Operator::EQUAL { .. } => Op::EQUAL.clone(),
        Absyn::Operator::NEQUAL { .. } => Op::NEQUAL.clone(),
    });
    outOperator = metamodelica::Ref::new(NFOperator {
        ty: crate::NFType::interned_UNKNOWN(),
        op: op,
    });
    outOperator
}

pub(crate) fn toAbsyn(mut op: &metamodelica::Ref<NFOperator>) -> Result<Absyn::Operator> {
    let mut aop: Absyn::Operator;
    aop = (match op.op.clone() {
        Op::ADD => {
            if (Type::isArray(&op.ty)) {
                openmodelica_ast::Absyn::Operator::ADD_EW
            } else {
                openmodelica_ast::Absyn::Operator::ADD
            }
        }
        Op::SUB => {
            if (Type::isArray(&op.ty)) {
                openmodelica_ast::Absyn::Operator::SUB_EW
            } else {
                openmodelica_ast::Absyn::Operator::SUB
            }
        }
        Op::MUL => {
            if (Type::isArray(&op.ty)) {
                openmodelica_ast::Absyn::Operator::MUL_EW
            } else {
                openmodelica_ast::Absyn::Operator::MUL
            }
        }
        Op::DIV => {
            if (Type::isArray(&op.ty)) {
                openmodelica_ast::Absyn::Operator::DIV_EW
            } else {
                openmodelica_ast::Absyn::Operator::DIV
            }
        }
        Op::POW => {
            if (Type::isArray(&op.ty)) {
                openmodelica_ast::Absyn::Operator::POW_EW
            } else {
                openmodelica_ast::Absyn::Operator::POW
            }
        }
        Op::ADD_EW => openmodelica_ast::Absyn::Operator::ADD_EW,
        Op::SUB_EW => openmodelica_ast::Absyn::Operator::SUB_EW,
        Op::MUL_EW => openmodelica_ast::Absyn::Operator::MUL_EW,
        Op::DIV_EW => openmodelica_ast::Absyn::Operator::DIV_EW,
        Op::POW_EW => openmodelica_ast::Absyn::Operator::POW_EW,
        Op::ADD_SCALAR_ARRAY => openmodelica_ast::Absyn::Operator::ADD,
        Op::ADD_ARRAY_SCALAR { .. } => openmodelica_ast::Absyn::Operator::ADD,
        Op::SUB_SCALAR_ARRAY { .. } => openmodelica_ast::Absyn::Operator::SUB,
        Op::SUB_ARRAY_SCALAR => openmodelica_ast::Absyn::Operator::SUB,
        Op::MUL_SCALAR_ARRAY => openmodelica_ast::Absyn::Operator::MUL,
        Op::MUL_ARRAY_SCALAR { .. } => openmodelica_ast::Absyn::Operator::MUL,
        Op::MUL_VECTOR_MATRIX => openmodelica_ast::Absyn::Operator::MUL,
        Op::MUL_MATRIX_VECTOR => openmodelica_ast::Absyn::Operator::MUL,
        Op::SCALAR_PRODUCT => openmodelica_ast::Absyn::Operator::MUL,
        Op::MATRIX_PRODUCT => openmodelica_ast::Absyn::Operator::MUL,
        Op::DIV_SCALAR_ARRAY { .. } => openmodelica_ast::Absyn::Operator::DIV,
        Op::DIV_ARRAY_SCALAR { .. } => openmodelica_ast::Absyn::Operator::DIV,
        Op::POW_SCALAR_ARRAY { .. } => openmodelica_ast::Absyn::Operator::POW,
        Op::POW_ARRAY_SCALAR { .. } => openmodelica_ast::Absyn::Operator::POW,
        Op::POW_MATRIX => openmodelica_ast::Absyn::Operator::POW,
        Op::UMINUS => {
            if (Type::isArray(&op.ty)) {
                openmodelica_ast::Absyn::Operator::UMINUS_EW
            } else {
                openmodelica_ast::Absyn::Operator::UMINUS
            }
        }
        Op::AND => openmodelica_ast::Absyn::Operator::AND,
        Op::OR => openmodelica_ast::Absyn::Operator::OR,
        Op::NOT => openmodelica_ast::Absyn::Operator::NOT,
        Op::LESS => openmodelica_ast::Absyn::Operator::LESS,
        Op::LESSEQ => openmodelica_ast::Absyn::Operator::LESSEQ,
        Op::GREATER => openmodelica_ast::Absyn::Operator::GREATER,
        Op::EQUAL => openmodelica_ast::Absyn::Operator::EQUAL,
        Op::NEQUAL => openmodelica_ast::Absyn::Operator::NEQUAL,
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFOperator.toAbsyn"));
                    __mm_s.push_str(&*literal!(" got unknown type."));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFOperator.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok(aop)
}

pub(crate) fn toDAE(mut op: &metamodelica::Ref<NFOperator>) -> Result<(DAE::Operator, bool, bool)> {
    let mut daeOp: DAE::Operator;
    let mut swapArguments: bool = false;
    let mut negate: bool = false;
    let mut ty: metamodelica::Ref<DAE::Type>;
    ty = Type::toDAE(&op.ty, true)?;
    daeOp = (match op.op.clone() {
        Op::ADD => {
            if (Type::isArray(&op.ty)) {
                DAE::Operator::ADD_ARR { ty: ty }
            } else {
                DAE::Operator::ADD { ty: ty }
            }
        }
        Op::SUB => {
            if (Type::isArray(&op.ty)) {
                DAE::Operator::SUB_ARR { ty: ty }
            } else {
                DAE::Operator::SUB { ty: ty }
            }
        }
        Op::MUL => {
            if (Type::isArray(&op.ty)) {
                DAE::Operator::MUL_ARR { ty: ty }
            } else {
                DAE::Operator::MUL { ty: ty }
            }
        }
        Op::DIV => {
            if (Type::isArray(&op.ty)) {
                DAE::Operator::DIV_ARR { ty: ty }
            } else {
                DAE::Operator::DIV { ty: ty }
            }
        }
        Op::POW => {
            if (Type::isArray(&op.ty)) {
                DAE::Operator::POW_ARR2 { ty: ty }
            } else {
                DAE::Operator::POW { ty: ty }
            }
        }
        Op::ADD_SCALAR_ARRAY => {
            swapArguments = true;
            DAE::Operator::ADD_ARRAY_SCALAR { ty: ty }
        }
        Op::ADD_ARRAY_SCALAR { .. } => DAE::Operator::ADD_ARRAY_SCALAR { ty: ty },
        Op::SUB_SCALAR_ARRAY { .. } => DAE::Operator::SUB_SCALAR_ARRAY { ty: ty },
        Op::SUB_ARRAY_SCALAR => {
            negate = true;
            DAE::Operator::ADD_ARRAY_SCALAR { ty: ty }
        }
        Op::MUL_SCALAR_ARRAY => {
            swapArguments = true;
            DAE::Operator::MUL_ARRAY_SCALAR { ty: ty }
        }
        Op::MUL_ARRAY_SCALAR { .. } => DAE::Operator::MUL_ARRAY_SCALAR { ty: ty },
        Op::MUL_VECTOR_MATRIX => DAE::Operator::MUL_MATRIX_PRODUCT { ty: ty },
        Op::MUL_MATRIX_VECTOR => DAE::Operator::MUL_MATRIX_PRODUCT { ty: ty },
        Op::SCALAR_PRODUCT => DAE::Operator::MUL_SCALAR_PRODUCT { ty: ty },
        Op::ADD_EW => DAE::Operator::ADD_ARR { ty: ty },
        Op::SUB_EW => DAE::Operator::SUB_ARR { ty: ty },
        Op::MUL_EW => DAE::Operator::MUL_ARR { ty: ty },
        Op::DIV_EW => DAE::Operator::DIV_ARR { ty: ty },
        Op::MATRIX_PRODUCT => DAE::Operator::MUL_MATRIX_PRODUCT { ty: ty },
        Op::DIV_SCALAR_ARRAY { .. } => DAE::Operator::DIV_SCALAR_ARRAY { ty: ty },
        Op::DIV_ARRAY_SCALAR { .. } => DAE::Operator::DIV_ARRAY_SCALAR { ty: ty },
        Op::POW_SCALAR_ARRAY { .. } => DAE::Operator::POW_SCALAR_ARRAY { ty: ty },
        Op::POW_ARRAY_SCALAR { .. } => DAE::Operator::POW_ARRAY_SCALAR { ty: ty },
        Op::POW_MATRIX => DAE::Operator::POW_ARR { ty: ty },
        Op::UMINUS => {
            if (Type::isArray(&op.ty)) {
                DAE::Operator::UMINUS_ARR { ty: ty }
            } else {
                DAE::Operator::UMINUS { ty: ty }
            }
        }
        Op::AND => DAE::Operator::AND { ty: ty },
        Op::OR => DAE::Operator::OR { ty: ty },
        Op::NOT => DAE::Operator::NOT { ty: ty },
        Op::LESS => DAE::Operator::LESS { ty: ty },
        Op::LESSEQ => DAE::Operator::LESSEQ { ty: ty },
        Op::GREATER => DAE::Operator::GREATER { ty: ty },
        Op::GREATEREQ => DAE::Operator::GREATEREQ { ty: ty },
        Op::EQUAL => DAE::Operator::EQUAL { ty: ty },
        Op::NEQUAL => DAE::Operator::NEQUAL { ty: ty },
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFOperator.toDAE"));
                    __mm_s.push_str(&*literal!(" got unknown type: "));
                    __mm_s.push_str(&*opToString(op.op.clone())?);
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFOperator.mo")),
            )?;
            return Err("fail");
        }
    });
    Ok((daeOp, swapArguments, negate))
}

pub fn typeOf(mut op: &metamodelica::Ref<NFOperator>) -> metamodelica::Ref<Type::NFType> {
    let mut ty: metamodelica::Ref<Type::NFType> = op.ty.clone();
    ty
}

pub(crate) fn setType(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut op: metamodelica::Ref<NFOperator>,
) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = op;
    assign_field!(op.ty = ty);
    op
}

pub(crate) fn scalarize(mut op: metamodelica::Ref<NFOperator>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = op;
    assign_field!(op.ty = Type::arrayElementType(&op.ty));
    op
}

pub(crate) fn unlift(mut op: metamodelica::Ref<NFOperator>) -> Result<metamodelica::Ref<NFOperator>> {
    let mut op: metamodelica::Ref<NFOperator> = op;
    assign_field!(op.ty = Type::unliftArray(op.ty.clone())?);
    Ok(op)
}

pub(crate) fn symbol(mut op: &metamodelica::Ref<NFOperator>, mut spacing: &ArcStr) -> Result<ArcStr> {
    let mut symbol: ArcStr;
    symbol = (match op.op.clone() {
        Op::ADD => literal!("+"),
        Op::SUB => literal!("-"),
        Op::MUL => literal!("*"),
        Op::DIV => literal!("/"),
        Op::POW => literal!("^"),
        Op::ADD_EW => literal!(".+"),
        Op::SUB_EW => literal!(".-"),
        Op::MUL_EW => literal!(".*"),
        Op::DIV_EW => literal!("./"),
        Op::POW_EW => literal!(".^"),
        Op::ADD_SCALAR_ARRAY => literal!(".+"),
        Op::ADD_ARRAY_SCALAR { .. } => literal!(".+"),
        Op::SUB_SCALAR_ARRAY { .. } => literal!(".-"),
        Op::SUB_ARRAY_SCALAR => literal!(".-"),
        Op::MUL_SCALAR_ARRAY => literal!("*"),
        Op::MUL_ARRAY_SCALAR { .. } => literal!(".*"),
        Op::MUL_VECTOR_MATRIX => literal!("*"),
        Op::MUL_MATRIX_VECTOR => literal!("*"),
        Op::SCALAR_PRODUCT => literal!("*"),
        Op::MATRIX_PRODUCT => literal!("*"),
        Op::DIV_SCALAR_ARRAY { .. } => literal!("./"),
        Op::DIV_ARRAY_SCALAR { .. } => literal!("/"),
        Op::POW_SCALAR_ARRAY { .. } => literal!(".^"),
        Op::POW_ARRAY_SCALAR { .. } => literal!(".^"),
        Op::POW_MATRIX => literal!("^"),
        Op::UMINUS => literal!("-"),
        Op::AND => literal!("and"),
        Op::OR => literal!("or"),
        Op::NOT => literal!("not"),
        Op::LESS => literal!("<"),
        Op::LESSEQ => literal!("<="),
        Op::GREATER => literal!(">"),
        Op::GREATEREQ => literal!(">="),
        Op::EQUAL => literal!("=="),
        Op::NEQUAL => literal!("<>"),
        _ => {
            Error::terminate(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFOperator.symbol"));
                    __mm_s.push_str(&*literal!(" got unknown type."));
                    ArcStr::from(__mm_s)
                },
                &(metamodelica::sourceInfo!("NFFrontEnd/NFOperator.mo")),
            )?;
            return Err("fail");
        }
    });
    symbol = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*spacing);
        __mm_s.push_str(&*symbol);
        __mm_s.push_str(&*spacing);
        ArcStr::from(__mm_s)
    };
    Ok(symbol)
}

pub(crate) fn toJSON(mut operator: &metamodelica::Ref<NFOperator>) -> Result<metamodelica::Ref<JSON::JSON>> {
    let mut json: metamodelica::Ref<JSON::JSON>;
    let symbols: metamodelica::Array<metamodelica::Ref<JSON::JSON>> = metamodelica::Dangerous::listArray(list![
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("+") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("-") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("*") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("/") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("^") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!(".+") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!(".-") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!(".*") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("./") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!(".^") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!(".+") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!(".+") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!(".-") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!(".-") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("*") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!(".*") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("*") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("*") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("*") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("*") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("./") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("/") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!(".^") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!(".^") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("^") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("-") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("and") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("or") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("not") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("<") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("<=") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!(">") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!(">=") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("==") }),
        metamodelica::Ref::new(JSON::JSON::STRING { r#str: literal!("<>") })
    ]);
    let mut op: Op = operator.op.clone();
    json = ({
        let __elt = (*metamodelica::index_checked(&symbols.borrow(), ((op) as i32))?).clone();
        __elt
    });
    Ok(json)
}

pub(crate) fn priority(mut op: &metamodelica::Ref<NFOperator>, mut lhs: bool) -> i32 {
    let mut priority: i32;
    priority = (match op.op.clone() {
        Op::ADD => {
            if (lhs) {
                5
            } else {
                6
            }
        }
        Op::SUB => 5,
        Op::MUL => 2,
        Op::DIV => 2,
        Op::POW => 1,
        Op::ADD_EW => {
            if (lhs) {
                5
            } else {
                6
            }
        }
        Op::SUB_EW => 5,
        Op::MUL_EW => {
            if (lhs) {
                2
            } else {
                3
            }
        }
        Op::DIV_EW => 2,
        Op::POW_EW => 1,
        Op::ADD_SCALAR_ARRAY => {
            if (lhs) {
                5
            } else {
                6
            }
        }
        Op::ADD_ARRAY_SCALAR { .. } => {
            if (lhs) {
                5
            } else {
                6
            }
        }
        Op::SUB_SCALAR_ARRAY { .. } => 5,
        Op::SUB_ARRAY_SCALAR => 5,
        Op::MUL_SCALAR_ARRAY => {
            if (lhs) {
                2
            } else {
                3
            }
        }
        Op::MUL_ARRAY_SCALAR { .. } => {
            if (lhs) {
                2
            } else {
                3
            }
        }
        Op::MUL_VECTOR_MATRIX => {
            if (lhs) {
                2
            } else {
                3
            }
        }
        Op::MUL_MATRIX_VECTOR => {
            if (lhs) {
                2
            } else {
                3
            }
        }
        Op::SCALAR_PRODUCT => {
            if (lhs) {
                2
            } else {
                3
            }
        }
        Op::MATRIX_PRODUCT => {
            if (lhs) {
                2
            } else {
                3
            }
        }
        Op::DIV_SCALAR_ARRAY { .. } => 2,
        Op::DIV_ARRAY_SCALAR { .. } => 2,
        Op::POW_SCALAR_ARRAY { .. } => 1,
        Op::POW_ARRAY_SCALAR { .. } => 1,
        Op::POW_MATRIX => 1,
        Op::AND => 8,
        Op::OR => 9,
        _ => 0,
    });
    priority
}

pub(crate) fn isAssociative(mut op: &metamodelica::Ref<NFOperator>) -> bool {
    let mut isAssociative: bool;
    isAssociative = (match op.op.clone() {
        Op::ADD => true,
        Op::ADD_EW => true,
        Op::MUL_EW => true,
        _ => false,
    });
    isAssociative
}

pub(crate) fn isNonAssociative(mut op: &metamodelica::Ref<NFOperator>) -> bool {
    let mut isNonAssociative: bool;
    isNonAssociative = (match op.op.clone() {
        Op::POW => true,
        Op::POW_EW => true,
        Op::POW_SCALAR_ARRAY { .. } => true,
        Op::POW_ARRAY_SCALAR { .. } => true,
        Op::POW_MATRIX => true,
        _ => false,
    });
    isNonAssociative
}

pub fn makeAdd(mut ty: metamodelica::Ref<Type::NFType>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = metamodelica::Ref::new(NFOperator {
        ty: ty.clone(),
        op: Op::ADD.clone(),
    });
    op
}

pub fn makeSub(mut ty: metamodelica::Ref<Type::NFType>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = metamodelica::Ref::new(NFOperator {
        ty: ty.clone(),
        op: Op::SUB.clone(),
    });
    op
}

pub fn makeMul(mut ty: metamodelica::Ref<Type::NFType>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = metamodelica::Ref::new(NFOperator {
        ty: ty.clone(),
        op: Op::MUL.clone(),
    });
    op
}

pub fn makeScalarProduct(mut ty: metamodelica::Ref<Type::NFType>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = metamodelica::Ref::new(NFOperator {
        ty: ty.clone(),
        op: Op::SCALAR_PRODUCT.clone(),
    });
    op
}

pub(crate) fn makeDiv(mut ty: metamodelica::Ref<Type::NFType>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = metamodelica::Ref::new(NFOperator {
        ty: ty.clone(),
        op: Op::DIV.clone(),
    });
    op
}

pub(crate) fn makePow(mut ty: metamodelica::Ref<Type::NFType>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = metamodelica::Ref::new(NFOperator {
        ty: ty.clone(),
        op: Op::POW.clone(),
    });
    op
}

pub(crate) fn makeAddEW(mut ty: metamodelica::Ref<Type::NFType>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = metamodelica::Ref::new(NFOperator {
        ty: ty.clone(),
        op: Op::ADD_EW.clone(),
    });
    op
}

pub(crate) fn makeSubEW(mut ty: metamodelica::Ref<Type::NFType>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = metamodelica::Ref::new(NFOperator {
        ty: ty.clone(),
        op: Op::SUB_EW.clone(),
    });
    op
}

pub(crate) fn makeMulEW(mut ty: metamodelica::Ref<Type::NFType>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = metamodelica::Ref::new(NFOperator {
        ty: ty.clone(),
        op: Op::MUL_EW.clone(),
    });
    op
}

pub(crate) fn makeDivEW(mut ty: metamodelica::Ref<Type::NFType>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = metamodelica::Ref::new(NFOperator {
        ty: ty.clone(),
        op: Op::DIV_EW.clone(),
    });
    op
}

pub(crate) fn makeUMinus(mut ty: metamodelica::Ref<Type::NFType>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = metamodelica::Ref::new(NFOperator {
        ty: ty.clone(),
        op: Op::UMINUS.clone(),
    });
    op
}

pub fn makeAnd(mut ty: metamodelica::Ref<Type::NFType>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = metamodelica::Ref::new(NFOperator {
        ty: ty.clone(),
        op: Op::AND.clone(),
    });
    op
}

pub(crate) fn makeOr(mut ty: metamodelica::Ref<Type::NFType>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = metamodelica::Ref::new(NFOperator {
        ty: ty.clone(),
        op: Op::OR.clone(),
    });
    op
}

pub(crate) fn makeNot(mut ty: metamodelica::Ref<Type::NFType>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = metamodelica::Ref::new(NFOperator {
        ty: ty.clone(),
        op: Op::NOT.clone(),
    });
    op
}

pub fn makeLess(mut ty: metamodelica::Ref<Type::NFType>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = metamodelica::Ref::new(NFOperator {
        ty: ty.clone(),
        op: Op::LESS.clone(),
    });
    op
}

pub(crate) fn makeLessEq(mut ty: metamodelica::Ref<Type::NFType>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = metamodelica::Ref::new(NFOperator {
        ty: ty.clone(),
        op: Op::LESSEQ.clone(),
    });
    op
}

pub fn makeGreater(mut ty: metamodelica::Ref<Type::NFType>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = metamodelica::Ref::new(NFOperator {
        ty: ty.clone(),
        op: Op::GREATER.clone(),
    });
    op
}

pub fn makeGreaterEq(mut ty: metamodelica::Ref<Type::NFType>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = metamodelica::Ref::new(NFOperator {
        ty: ty.clone(),
        op: Op::GREATEREQ.clone(),
    });
    op
}

pub(crate) fn makeEqual(mut ty: metamodelica::Ref<Type::NFType>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = metamodelica::Ref::new(NFOperator {
        ty: ty.clone(),
        op: Op::EQUAL.clone(),
    });
    op
}

pub fn makeNotEqual(mut ty: metamodelica::Ref<Type::NFType>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = metamodelica::Ref::new(NFOperator {
        ty: ty.clone(),
        op: Op::NEQUAL.clone(),
    });
    op
}

pub(crate) fn makeScalarArray(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut op: Op,
) -> Result<metamodelica::Ref<NFOperator>> {
    let mut outOp: metamodelica::Ref<NFOperator>;
    let mut o: Op;
    o = (match op {
        Op::ADD => Op::ADD_SCALAR_ARRAY.clone(),
        Op::SUB => Op::SUB_SCALAR_ARRAY.clone(),
        Op::MUL => Op::MUL_SCALAR_ARRAY.clone(),
        Op::DIV => Op::DIV_SCALAR_ARRAY.clone(),
        Op::POW => Op::POW_SCALAR_ARRAY.clone(),
        _ => return Err("match: no arm matched"),
    });
    outOp = metamodelica::Ref::new(NFOperator { ty: ty, op: o });
    Ok(outOp)
}

pub(crate) fn makeArrayScalar(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut op: Op,
) -> Result<metamodelica::Ref<NFOperator>> {
    let mut outOp: metamodelica::Ref<NFOperator>;
    let mut o: Op;
    o = (match op {
        Op::ADD => Op::ADD_ARRAY_SCALAR.clone(),
        Op::SUB => Op::SUB_ARRAY_SCALAR.clone(),
        Op::MUL => Op::MUL_ARRAY_SCALAR.clone(),
        Op::DIV => Op::DIV_ARRAY_SCALAR.clone(),
        Op::POW => Op::POW_ARRAY_SCALAR.clone(),
        _ => return Err("match: no arm matched"),
    });
    outOp = metamodelica::Ref::new(NFOperator { ty: ty, op: o });
    Ok(outOp)
}

pub(crate) fn makeEW(mut op: metamodelica::Ref<NFOperator>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = op;
    let () = (match op.op.clone() {
        Op::ADD => {
            assign_field!(op.op = Op::ADD_EW.clone());
            ()
        }
        Op::SUB => {
            assign_field!(op.op = Op::SUB_EW.clone());
            ()
        }
        Op::MUL => {
            assign_field!(op.op = Op::MUL_EW.clone());
            ()
        }
        Op::DIV => {
            assign_field!(op.op = Op::DIV_EW.clone());
            ()
        }
        Op::POW => {
            assign_field!(op.op = Op::POW_EW.clone());
            ()
        }
        _ => (),
    });
    op
}

pub(crate) fn stripEW(mut op: metamodelica::Ref<NFOperator>) -> metamodelica::Ref<NFOperator> {
    let mut op: metamodelica::Ref<NFOperator> = op;
    let () = (match op.op.clone() {
        Op::ADD_EW => {
            assign_field!(op.op = Op::ADD.clone());
            ()
        }
        Op::SUB_EW => {
            assign_field!(op.op = Op::SUB.clone());
            ()
        }
        Op::MUL_EW => {
            assign_field!(op.op = Op::MUL.clone());
            ()
        }
        Op::DIV_EW => {
            assign_field!(op.op = Op::DIV.clone());
            ()
        }
        Op::POW_EW => {
            assign_field!(op.op = Op::POW.clone());
            ()
        }
        _ => (),
    });
    op
}

pub(crate) fn isElementWise(mut op: &metamodelica::Ref<NFOperator>) -> bool {
    let mut ew: bool;
    ew = (match op.op.clone() {
        Op::ADD_EW => true,
        Op::SUB_EW => true,
        Op::MUL_EW => true,
        Op::DIV_EW => true,
        Op::POW_EW => true,
        _ => false,
    });
    ew
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum MathClassification {
    ADDITION = 1,
    SUBTRACTION = 2,
    MULTIPLICATION = 3,
    DIVISION = 4,
    POWER = 5,
    LOGICAL = 6,
    RELATION = 7,
}
impl PartialOrd for MathClassification {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for MathClassification {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for MathClassification {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum SizeClassification {
    SCALAR = 1,
    ELEMENT_WISE = 2,
    ARRAY_SCALAR = 3,
    SCALAR_ARRAY = 4,
    MATRIX = 5,
    VECTOR_MATRIX = 6,
    MATRIX_VECTOR = 7,
    LOGICAL = 8,
    RELATION = 9,
}
impl PartialOrd for SizeClassification {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SizeClassification {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for SizeClassification {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub type Classification = (MathClassification, SizeClassification);

pub(crate) fn mathSymbol(mut mcl: MathClassification) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match mcl {
        MathClassification::ADDITION => literal!("+"),
        MathClassification::SUBTRACTION => literal!("-"),
        MathClassification::MULTIPLICATION => literal!("*"),
        MathClassification::DIVISION { .. } => literal!("/"),
        MathClassification::POWER => literal!("^"),
        MathClassification::LOGICAL => literal!("L"),
        MathClassification::RELATION { .. } => literal!("R"),
        _ => return Err("fail"),
    });
    Ok(r#str)
}

pub(crate) fn classificationString(mut cla: Classification) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut mcl: MathClassification;
    let mut scl: SizeClassification;
    (mcl, scl) = cla;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*mathClassificationString(mcl)?);
        __mm_s.push_str(&*sizeClassificationString(scl)?);
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

pub(crate) fn mathClassificationString(mut mcl: MathClassification) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match mcl {
        MathClassification::ADDITION => literal!("[ADD]"),
        MathClassification::SUBTRACTION => literal!("[SUB]"),
        MathClassification::MULTIPLICATION => literal!("[MUL]"),
        MathClassification::DIVISION { .. } => literal!("[DIV]"),
        MathClassification::POWER => literal!("[POW]"),
        MathClassification::LOGICAL => literal!("[LOG]"),
        MathClassification::RELATION { .. } => literal!("[REL]"),
        _ => return Err("fail"),
    });
    Ok(r#str)
}

pub(crate) fn sizeClassificationString(mut scl: SizeClassification) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match scl {
        SizeClassification::SCALAR => literal!("[SCALAR]"),
        SizeClassification::ELEMENT_WISE => literal!("[ELMWIS]"),
        SizeClassification::ARRAY_SCALAR => literal!("[ARR-SC]"),
        SizeClassification::SCALAR_ARRAY => literal!("[SC-ARR]"),
        SizeClassification::MATRIX { .. } => literal!("[MATRIX]"),
        SizeClassification::VECTOR_MATRIX => literal!("[VEC-MA]"),
        SizeClassification::MATRIX_VECTOR => literal!("[MA-VEC]"),
        SizeClassification::LOGICAL => literal!("[LOGICL]"),
        SizeClassification::RELATION { .. } => literal!("[RELATN]"),
        _ => return Err("fail"),
    });
    Ok(r#str)
}

pub fn classify(mut op: &metamodelica::Ref<NFOperator>) -> Result<Classification> {
    let mut cl: Classification;
    cl = (match op.op.clone() {
        Op::ADD => (MathClassification::ADDITION.clone(), SizeClassification::SCALAR.clone()),
        Op::SUB => (
            MathClassification::SUBTRACTION.clone(),
            SizeClassification::SCALAR.clone(),
        ),
        Op::MUL => (
            MathClassification::MULTIPLICATION.clone(),
            SizeClassification::SCALAR.clone(),
        ),
        Op::DIV => (MathClassification::DIVISION.clone(), SizeClassification::SCALAR.clone()),
        Op::POW => (MathClassification::POWER.clone(), SizeClassification::SCALAR.clone()),
        Op::ADD_EW => (
            MathClassification::ADDITION.clone(),
            SizeClassification::ELEMENT_WISE.clone(),
        ),
        Op::SUB_EW => (
            MathClassification::SUBTRACTION.clone(),
            SizeClassification::ELEMENT_WISE.clone(),
        ),
        Op::MUL_EW => (
            MathClassification::MULTIPLICATION.clone(),
            SizeClassification::ELEMENT_WISE.clone(),
        ),
        Op::DIV_EW => (
            MathClassification::DIVISION.clone(),
            SizeClassification::ELEMENT_WISE.clone(),
        ),
        Op::POW_EW => (
            MathClassification::POWER.clone(),
            SizeClassification::ELEMENT_WISE.clone(),
        ),
        Op::MUL_ARRAY_SCALAR { .. } => (
            MathClassification::MULTIPLICATION.clone(),
            SizeClassification::ARRAY_SCALAR.clone(),
        ),
        Op::MUL_SCALAR_ARRAY => (
            MathClassification::MULTIPLICATION.clone(),
            SizeClassification::SCALAR_ARRAY.clone(),
        ),
        Op::ADD_ARRAY_SCALAR { .. } => (
            MathClassification::ADDITION.clone(),
            SizeClassification::ARRAY_SCALAR.clone(),
        ),
        Op::ADD_SCALAR_ARRAY => (
            MathClassification::ADDITION.clone(),
            SizeClassification::SCALAR_ARRAY.clone(),
        ),
        Op::SUB_ARRAY_SCALAR => (
            MathClassification::SUBTRACTION.clone(),
            SizeClassification::ARRAY_SCALAR.clone(),
        ),
        Op::SUB_SCALAR_ARRAY { .. } => (
            MathClassification::SUBTRACTION.clone(),
            SizeClassification::SCALAR_ARRAY.clone(),
        ),
        Op::SCALAR_PRODUCT => (
            MathClassification::MULTIPLICATION.clone(),
            SizeClassification::SCALAR.clone(),
        ),
        Op::MATRIX_PRODUCT => (
            MathClassification::MULTIPLICATION.clone(),
            SizeClassification::MATRIX.clone(),
        ),
        Op::MUL_VECTOR_MATRIX => (
            MathClassification::MULTIPLICATION.clone(),
            SizeClassification::VECTOR_MATRIX.clone(),
        ),
        Op::MUL_MATRIX_VECTOR => (
            MathClassification::MULTIPLICATION.clone(),
            SizeClassification::MATRIX_VECTOR.clone(),
        ),
        Op::DIV_ARRAY_SCALAR { .. } => (
            MathClassification::DIVISION.clone(),
            SizeClassification::ARRAY_SCALAR.clone(),
        ),
        Op::DIV_SCALAR_ARRAY { .. } => (
            MathClassification::DIVISION.clone(),
            SizeClassification::SCALAR_ARRAY.clone(),
        ),
        Op::POW_ARRAY_SCALAR { .. } => (
            MathClassification::POWER.clone(),
            SizeClassification::ARRAY_SCALAR.clone(),
        ),
        Op::POW_SCALAR_ARRAY { .. } => (
            MathClassification::POWER.clone(),
            SizeClassification::SCALAR_ARRAY.clone(),
        ),
        Op::POW_MATRIX => (MathClassification::POWER.clone(), SizeClassification::MATRIX.clone()),
        Op::AND => (MathClassification::LOGICAL.clone(), SizeClassification::LOGICAL.clone()),
        Op::OR => (MathClassification::LOGICAL.clone(), SizeClassification::LOGICAL.clone()),
        Op::NOT => (MathClassification::LOGICAL.clone(), SizeClassification::LOGICAL.clone()),
        Op::LESS => (
            MathClassification::RELATION.clone(),
            SizeClassification::RELATION.clone(),
        ),
        Op::LESSEQ => (
            MathClassification::RELATION.clone(),
            SizeClassification::RELATION.clone(),
        ),
        Op::GREATER => (
            MathClassification::RELATION.clone(),
            SizeClassification::RELATION.clone(),
        ),
        Op::GREATEREQ => (
            MathClassification::RELATION.clone(),
            SizeClassification::RELATION.clone(),
        ),
        Op::EQUAL => (
            MathClassification::RELATION.clone(),
            SizeClassification::RELATION.clone(),
        ),
        Op::NEQUAL => (
            MathClassification::RELATION.clone(),
            SizeClassification::RELATION.clone(),
        ),
        _ => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFOperator.classify"));
                    __mm_s.push_str(&*literal!(": Don't know how to handle "));
                    __mm_s.push_str(&*ArcStr::from(::std::format!("{:?}", op.op.clone())));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("NFFrontEnd/NFOperator.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(cl)
}

pub fn classifyAddition(mut op: &metamodelica::Ref<NFOperator>) -> SizeClassification {
    let mut sz: SizeClassification = if (Type::isScalar(&op.ty)) {
        SizeClassification::SCALAR.clone()
    } else {
        SizeClassification::ELEMENT_WISE.clone()
    };
    sz
}

pub fn fromClassification(
    mut cl: Classification,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> Result<metamodelica::Ref<NFOperator>> {
    let mut result: metamodelica::Ref<NFOperator>;
    let mut op: Op;
    op = (match cl {
        (MathClassification::ADDITION, SizeClassification::SCALAR) => Op::ADD.clone(),
        (MathClassification::SUBTRACTION, SizeClassification::SCALAR) => Op::SUB.clone(),
        (MathClassification::MULTIPLICATION, SizeClassification::SCALAR) => Op::MUL.clone(),
        (MathClassification::DIVISION { .. }, SizeClassification::SCALAR) => Op::DIV.clone(),
        (MathClassification::POWER, SizeClassification::SCALAR) => Op::POW.clone(),
        (MathClassification::ADDITION, SizeClassification::ELEMENT_WISE) => Op::ADD_EW.clone(),
        (MathClassification::SUBTRACTION, SizeClassification::ELEMENT_WISE) => Op::SUB_EW.clone(),
        (MathClassification::MULTIPLICATION, SizeClassification::ELEMENT_WISE) => Op::MUL_EW.clone(),
        (MathClassification::DIVISION { .. }, SizeClassification::ELEMENT_WISE) => Op::DIV_EW.clone(),
        (MathClassification::POWER, SizeClassification::ELEMENT_WISE) => Op::POW_EW.clone(),
        (MathClassification::ADDITION, SizeClassification::ARRAY_SCALAR) => Op::ADD_ARRAY_SCALAR.clone(),
        (MathClassification::SUBTRACTION, SizeClassification::ARRAY_SCALAR) => Op::SUB_ARRAY_SCALAR.clone(),
        (MathClassification::MULTIPLICATION, SizeClassification::ARRAY_SCALAR) => Op::MUL_ARRAY_SCALAR.clone(),
        (MathClassification::DIVISION { .. }, SizeClassification::ARRAY_SCALAR) => Op::DIV_ARRAY_SCALAR.clone(),
        (MathClassification::POWER, SizeClassification::ARRAY_SCALAR) => Op::POW_ARRAY_SCALAR.clone(),
        (MathClassification::ADDITION, SizeClassification::SCALAR_ARRAY) => Op::ADD_SCALAR_ARRAY.clone(),
        (MathClassification::SUBTRACTION, SizeClassification::SCALAR_ARRAY) => Op::SUB_SCALAR_ARRAY.clone(),
        (MathClassification::MULTIPLICATION, SizeClassification::SCALAR_ARRAY) => Op::MUL_SCALAR_ARRAY.clone(),
        (MathClassification::DIVISION { .. }, SizeClassification::SCALAR_ARRAY) => Op::DIV_SCALAR_ARRAY.clone(),
        (MathClassification::POWER, SizeClassification::SCALAR_ARRAY) => Op::POW_SCALAR_ARRAY.clone(),
        (MathClassification::ADDITION, SizeClassification::MATRIX { .. }) => Op::ADD_EW.clone(),
        (MathClassification::SUBTRACTION, SizeClassification::MATRIX { .. }) => Op::SUB_EW.clone(),
        (MathClassification::POWER, SizeClassification::MATRIX { .. }) => Op::POW_MATRIX.clone(),
        (MathClassification::MULTIPLICATION, SizeClassification::MATRIX { .. }) => Op::MATRIX_PRODUCT.clone(),
        (MathClassification::MULTIPLICATION, SizeClassification::VECTOR_MATRIX) => Op::MUL_VECTOR_MATRIX.clone(),
        (MathClassification::MULTIPLICATION, SizeClassification::MATRIX_VECTOR) => Op::MUL_MATRIX_VECTOR.clone(),
        _ => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFOperator.fromClassification"));
                    __mm_s.push_str(&*literal!(
                        ": Don't know how to handle math class and size class combination: "
                    ));
                    __mm_s.push_str(&*classificationString(cl)?);
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("NFFrontEnd/NFOperator.mo"),
            )?;
            return Err("fail");
        }
    });
    result = metamodelica::Ref::new(NFOperator { ty: ty, op: op });
    Ok(result)
}

pub fn getMathClassification(mut op: &metamodelica::Ref<NFOperator>) -> Result<MathClassification> {
    let mut mcl: MathClassification;
    (mcl, _) = classify(op)?;
    Ok(mcl)
}

pub fn getSizeClassification(mut op: &metamodelica::Ref<NFOperator>) -> Result<SizeClassification> {
    let mut scl: SizeClassification;
    (_, scl) = classify(op)?;
    Ok(scl)
}

pub(crate) fn combineSizeClassification(
    mut scl1: SizeClassification,
    mut scl2: SizeClassification,
) -> SizeClassification {
    let mut scl: SizeClassification;
    scl = (match (scl1, scl2) {
        (SizeClassification::ELEMENT_WISE, SizeClassification::SCALAR) => SizeClassification::ARRAY_SCALAR.clone(),
        (SizeClassification::SCALAR, SizeClassification::ELEMENT_WISE) => SizeClassification::SCALAR_ARRAY.clone(),
        _ => scl1,
    });
    scl
}

pub(crate) fn isDashClassification(mut mcl: MathClassification) -> bool {
    let mut b: bool;
    b = (match mcl {
        MathClassification::ADDITION => true,
        MathClassification::SUBTRACTION => true,
        _ => false,
    });
    b
}

pub(crate) fn isCommutative(mut operator: &metamodelica::Ref<NFOperator>) -> bool {
    let mut b: bool;
    b = (match &*(Type::arrayElementType(&operator.ty)) {
        Type::INTEGER => true,
        Type::REAL => true,
        Type::BOOLEAN => true,
        _ => false,
    });
    if !(b) {
        return b;
    }
    b = (match operator.op.clone() {
        Op::ADD => true,
        Op::MUL => true,
        Op::ADD_EW => true,
        Op::MUL_EW => true,
        Op::ADD_SCALAR_ARRAY => true,
        Op::ADD_ARRAY_SCALAR { .. } => true,
        Op::MUL_SCALAR_ARRAY => true,
        Op::MUL_ARRAY_SCALAR { .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isSoftCommutative(mut operator: &metamodelica::Ref<NFOperator>) -> bool {
    let mut b: bool;
    b = (match operator.op.clone() {
        Op::SUB => true,
        Op::DIV => true,
        Op::SUB_EW => true,
        Op::DIV_EW => true,
        Op::SUB_SCALAR_ARRAY { .. } => true,
        Op::SUB_ARRAY_SCALAR => true,
        Op::DIV_SCALAR_ARRAY { .. } => true,
        Op::DIV_ARRAY_SCALAR { .. } => true,
        _ => false,
    });
    b
}

pub fn repetition(mut operator: &metamodelica::Ref<NFOperator>) -> (bool, bool) {
    let mut b: (bool, bool);
    b = (match operator.op.clone() {
        Op::ADD_SCALAR_ARRAY => (true, false),
        Op::ADD_ARRAY_SCALAR { .. } => (false, true),
        Op::MUL_SCALAR_ARRAY => (true, false),
        Op::MUL_ARRAY_SCALAR { .. } => (false, true),
        Op::MUL_VECTOR_MATRIX => (true, true),
        Op::MUL_MATRIX_VECTOR => (true, true),
        Op::MATRIX_PRODUCT => (true, true),
        _ => (false, false),
    });
    b
}

pub fn reduction(mut operator: &metamodelica::Ref<NFOperator>) -> bool {
    let mut b: bool;
    b = (match operator.op.clone() {
        Op::MUL_MATRIX_VECTOR => true,
        Op::MUL_VECTOR_MATRIX => true,
        Op::MATRIX_PRODUCT => true,
        Op::SCALAR_PRODUCT => true,
        _ => false,
    });
    b
}

pub(crate) fn isCombineable(
    mut op1: &metamodelica::Ref<NFOperator>,
    mut op2: &metamodelica::Ref<NFOperator>,
) -> Result<bool> {
    let mut b: bool;
    let mut mcl1: MathClassification;
    let mut mcl2: MathClassification;
    let mut scl1: SizeClassification;
    let mut scl2: SizeClassification;
    (mcl1, scl1) = classify(op1)?;
    (mcl2, scl2) = classify(op2)?;
    b = isCombineableMath(mcl1, mcl2) && isCombineableSize(scl1, scl2);
    if b {
        b = !(isScalarProduct(op1) || isScalarProduct(op2));
    }
    Ok(b)
}

pub(crate) fn isCombineableMath(mut mcl1: MathClassification, mut mcl2: MathClassification) -> bool {
    let mut b: bool;
    b = mcl1 == mcl2 || isDashClassification(mcl1) && isDashClassification(mcl2);
    b
}

pub(crate) fn isCombineableSize(mut scl1: SizeClassification, mut scl2: SizeClassification) -> bool {
    let mut b: bool;
    b = scl1 == scl2;
    b
}

pub fn toDebugString(mut op: &metamodelica::Ref<NFOperator>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("OPERATOR("));
        __mm_s.push_str(&*Type::toString(&op.ty)?);
        __mm_s.push_str(&*literal!(", "));
        __mm_s.push_str(&*opToString(op.op.clone())?);
        __mm_s.push_str(&*literal!(")"));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

pub(crate) fn opToString(mut op: Op) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match op {
        Op::ADD => literal!("ADD"),
        Op::SUB => literal!("SUB"),
        Op::MUL => literal!("MUL"),
        Op::DIV => literal!("DIV"),
        Op::POW => literal!("POW"),
        Op::ADD_EW => literal!("ADD_EW"),
        Op::SUB_EW => literal!("SUB_EW"),
        Op::MUL_EW => literal!("MUL_EW"),
        Op::DIV_EW => literal!("DIV_EW"),
        Op::POW_EW => literal!("POW_EW"),
        Op::ADD_SCALAR_ARRAY => literal!("ADD_SCALAR_ARRAY"),
        Op::ADD_ARRAY_SCALAR { .. } => literal!("ADD_ARRAY_SCALAR"),
        Op::SUB_SCALAR_ARRAY { .. } => literal!("SUB_SCALAR_ARRAY"),
        Op::SUB_ARRAY_SCALAR => literal!("SUB_ARRAY_SCALAR"),
        Op::MUL_SCALAR_ARRAY => literal!("MUL_SCALAR_ARRAY"),
        Op::MUL_ARRAY_SCALAR { .. } => literal!("MUL_ARRAY_SCALAR"),
        Op::MUL_VECTOR_MATRIX => literal!("MUL_VECTOR_MATRIX"),
        Op::MUL_MATRIX_VECTOR => literal!("MUL_MATRIX_VECTOR"),
        Op::SCALAR_PRODUCT => literal!("SCALAR_PRODUCT"),
        Op::MATRIX_PRODUCT => literal!("MATRIX_PRODUCT"),
        Op::DIV_SCALAR_ARRAY { .. } => literal!("DIV_SCALAR_ARRAY"),
        Op::DIV_ARRAY_SCALAR { .. } => literal!("DIV_ARRAY_SCALAR"),
        Op::POW_SCALAR_ARRAY { .. } => literal!("POW_SCALAR_ARRAY"),
        Op::POW_ARRAY_SCALAR { .. } => literal!("POW_ARRAY_SCALAR"),
        Op::POW_MATRIX => literal!("POW_MATRIX"),
        Op::UMINUS => literal!("UMINUS"),
        Op::AND => literal!("AND"),
        Op::OR => literal!("OR"),
        Op::NOT => literal!("NOT"),
        Op::LESS => literal!("LESS"),
        Op::LESSEQ => literal!("LESSEQ"),
        Op::GREATER => literal!("GREATER"),
        Op::GREATEREQ => literal!("GREATEREQ"),
        Op::EQUAL => literal!("EQUAL"),
        Op::NEQUAL => literal!("NEQUAL"),
        Op::USERDEFINED { .. } => literal!("USERDEFINED"),
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NFOperator.opToString"));
                    __mm_s.push_str(&*literal!("failed. Unhanded enumeration."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(r#str)
}
