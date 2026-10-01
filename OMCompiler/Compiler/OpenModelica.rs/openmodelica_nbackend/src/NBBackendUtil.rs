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

use crate::NBEquation as BEquation;
use crate::NBEquation::Equation;
use crate::NBEquation::Frame;
use crate::NBEquation::FrameLocation;
use crate::NBVariable as BVariable;
use openmodelica_nf_frontend::NFBackendExtension::BackendInfo;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::MMath;
use openmodelica_util::Rational;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::Pointer;

// NF imports
// backend imports
// Util imports
// old imports
pub(crate) fn convertRational(mut r: &metamodelica::Ref<Rational::Rational>) -> MMath::Rational {
    let mut oldR: MMath::Rational = MMath::Rational {
        nom: r.n.clone(),
        denom: r.d.clone(),
    };
    oldR
}

pub(crate) fn findTrueIndices(mut arr: metamodelica::Array<bool>) -> Result<metamodelica::List<i32>> {
    let mut indices: metamodelica::List<i32> = ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut i in ({
            let __s = metamodelica::arrayLength(arr.clone());
            let __e = 1;
            (0i32..)
                .map(move |__k| __s + __k * (-1))
                .take_while(move |&__v| __v >= __e)
        })
        .into_iter()
        {
            if !({
                let __elt = (*metamodelica::index_checked(&arr.borrow(), i.clone())?).clone();
                __elt
            }) {
                continue;
            }
            let __x = i.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(indices)
}

pub(crate) fn indexTplGt<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut tpl1: (i32, T),
    mut tpl2: (i32, T),
) -> bool {
    let mut gt: bool;
    let mut i1: i32;
    let mut i2: i32;
    (i1, _) = tpl1;
    (i2, _) = tpl2;
    gt = i1 > i2;
    gt
}

pub(crate) fn noNameHashEq(mut eq: &metamodelica::Ref<Equation::Equation>, mut r#mod: i32) -> Result<i32> {
    let mut hash: i32;
    hash = noNameHashExp(&(BEquation::Equation::getResidualExp(eq, true)?), r#mod)?;
    Ok(hash)
}

pub(crate) fn noNameHashExp(mut exp: &metamodelica::Ref<Expression::NFExpression>, mut r#mod: i32) -> Result<i32> {
    let mut hash: i32 = 0;
    hash = (match &**exp {
        Expression::INTEGER { value: __exp_value } => __exp_value.clone(),
        Expression::REAL { value: __exp_value } => ((__exp_value.clone()).0.floor() as i32),
        Expression::STRING { value: __exp_value } => stringHashDjb2Mod(&__exp_value, r#mod),
        Expression::BOOLEAN { value: __exp_value } => Util::boolInt(__exp_value.clone()),
        Expression::ENUM_LITERAL { index: __exp_index, .. } => __exp_index.clone(),
        Expression::CLKCONST { .. } => 0,
        Expression::CREF { cref: __exp_cref, .. } => {
            let mut var: metamodelica::Ref<Variable::NFVariable>;
            var = BVariable::getVar(
                metamodelica::AsArg::as_arg(&__exp_cref),
                metamodelica::sourceInfo!("NBackEnd/Util/NBBackendUtil.mo"),
            )?;
            stringHashDjb2Mod(&(BackendInfo::toString(&var.backendinfo)?), r#mod)
        }
        Expression::TYPENAME { .. } => 1,
        Expression::ARRAY {
            literal: __exp_literal, ..
        } => {
            let __range0 = var_field!((**exp).elements, Expression::NFExpression::ARRAY)
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            for mut elem in __range0 {
                hash = hash + noNameHashExp(&elem, r#mod)?;
            }
            hash = hash + Util::boolInt(__exp_literal.clone());
            hash
        }
        Expression::MATRIX {
            elements: __exp_elements,
        } => {
            for mut lst in &*__exp_elements.clone() {
                for mut elem in &*lst.clone() {
                    hash = hash + noNameHashExp(metamodelica::AsArg::as_arg(&elem), r#mod)?;
                }
            }
            hash
        }
        Expression::RANGE {
            start: __exp_start,
            step: __exp_step,
            stop: __exp_stop,
            ..
        } => {
            if (__exp_step).is_some() {
                hash = noNameHashExp(&(__exp_step.clone().ok_or("pattern mismatch")?), r#mod)?;
            }
            hash + noNameHashExp(metamodelica::AsArg::as_arg(&__exp_start), r#mod)?
                + noNameHashExp(metamodelica::AsArg::as_arg(&__exp_stop), r#mod)?
        }
        Expression::TUPLE {
            elements: __exp_elements,
            ..
        } => {
            for mut elem in &*__exp_elements.clone() {
                hash = hash + noNameHashExp(metamodelica::AsArg::as_arg(&elem), r#mod)?;
            }
            hash
        }
        Expression::RECORD {
            elements: __exp_elements,
            ..
        } => {
            for mut elem in &*__exp_elements.clone() {
                hash = hash + noNameHashExp(metamodelica::AsArg::as_arg(&elem), r#mod)?;
            }
            hash
        }
        Expression::CALL { .. } => 2,
        Expression::SIZE {
            dimIndex: __exp_dimIndex,
            exp: __exp_exp,
        } => {
            if (__exp_dimIndex).is_some() {
                hash = noNameHashExp(&(__exp_dimIndex.clone().ok_or("pattern mismatch")?), r#mod)?;
            }
            hash + noNameHashExp(metamodelica::AsArg::as_arg(&__exp_exp), r#mod)?
        }
        Expression::END => stringHashDjb2Mod(&(literal!("end")), r#mod),
        Expression::BINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            operator: __exp_operator,
        } => {
            let mut hash1: i32;
            let mut hash2: i32;
            hash1 = noNameHashExp(metamodelica::AsArg::as_arg(&__exp_exp1), r#mod)?;
            hash2 = noNameHashExp(metamodelica::AsArg::as_arg(&__exp_exp2), r#mod)?;
            hash = (match Operator::classify(metamodelica::AsArg::as_arg(&__exp_operator))? {
                (Operator::MathClassification::ADDITION, _) => hash1 + hash2,
                (Operator::MathClassification::SUBTRACTION, _) => hash1 - hash2,
                (Operator::MathClassification::MULTIPLICATION, _) => hash1 * hash2,
                (Operator::MathClassification::DIVISION, _) => {
                    ((metamodelica::real_div_checked(
                        metamodelica::OrderedFloat((hash1) as f64),
                        metamodelica::OrderedFloat((hash2) as f64),
                    )?)
                    .0
                    .floor() as i32)
                }
                (Operator::MathClassification::POWER, _) => {
                    (((metamodelica::OrderedFloat((hash1) as f64)).powf(metamodelica::OrderedFloat((hash2) as f64)))
                        .0
                        .floor() as i32)
                }
                (Operator::MathClassification::LOGICAL, _) => -(hash1 + hash2),
                (Operator::MathClassification::RELATION, _) => hash2 - hash1,
                _ => hash2 - hash1,
            });
            hash
        }
        Expression::UNARY { exp: __exp_exp, .. } => -(noNameHashExp(metamodelica::AsArg::as_arg(&__exp_exp), r#mod)?),
        Expression::LBINARY {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            operator: __exp_operator,
        } => {
            let mut hash1: i32;
            let mut hash2: i32;
            hash1 = noNameHashExp(metamodelica::AsArg::as_arg(&__exp_exp1), r#mod)?;
            hash2 = noNameHashExp(metamodelica::AsArg::as_arg(&__exp_exp2), r#mod)?;
            hash = (match __exp_operator.op.clone() {
                Operator::Op::AND => hash1 + hash2,
                Operator::Op::OR => hash1 - hash2,
                _ => hash2 - hash1,
            });
            hash
        }
        Expression::LUNARY { exp: __exp_exp, .. } => -(noNameHashExp(metamodelica::AsArg::as_arg(&__exp_exp), r#mod)?),
        Expression::RELATION {
            exp1: __exp_exp1,
            exp2: __exp_exp2,
            operator: __exp_operator,
            ..
        } => {
            let mut hash1: i32;
            let mut hash2: i32;
            hash1 = noNameHashExp(metamodelica::AsArg::as_arg(&__exp_exp1), r#mod)?;
            hash2 = noNameHashExp(metamodelica::AsArg::as_arg(&__exp_exp2), r#mod)?;
            hash = (match __exp_operator.op.clone() {
                Operator::Op::LESS => hash1 + hash2,
                Operator::Op::LESSEQ => -(hash1 + hash2),
                Operator::Op::GREATER => hash1 - hash2,
                Operator::Op::GREATEREQ => hash2 - hash1,
                Operator::Op::EQUAL => hash1 * hash2,
                Operator::Op::NEQUAL => {
                    (((metamodelica::OrderedFloat((hash1) as f64)).powf(metamodelica::OrderedFloat((hash2) as f64)))
                        .0
                        .floor() as i32)
                }
                _ => hash2 - hash1,
            });
            hash
        }
        Expression::IF {
            condition: __exp_condition,
            falseBranch: __exp_falseBranch,
            trueBranch: __exp_trueBranch,
            ..
        } => {
            noNameHashExp(metamodelica::AsArg::as_arg(&__exp_condition), r#mod)?
                + noNameHashExp(metamodelica::AsArg::as_arg(&__exp_trueBranch), r#mod)?
                + noNameHashExp(metamodelica::AsArg::as_arg(&__exp_falseBranch), r#mod)?
        }
        Expression::CAST { exp: __exp_exp, .. } => noNameHashExp(metamodelica::AsArg::as_arg(&__exp_exp), r#mod)?,
        Expression::BOX { exp: __exp_exp } => noNameHashExp(metamodelica::AsArg::as_arg(&__exp_exp), r#mod)?,
        Expression::UNBOX { exp: __exp_exp, .. } => noNameHashExp(metamodelica::AsArg::as_arg(&__exp_exp), r#mod)?,
        Expression::SUBSCRIPTED_EXP { exp: __exp_exp, .. } => {
            noNameHashExp(metamodelica::AsArg::as_arg(&__exp_exp), r#mod)?
        }
        Expression::TUPLE_ELEMENT {
            index: __exp_index,
            tupleExp: __exp_tupleExp,
            ..
        } => noNameHashExp(metamodelica::AsArg::as_arg(&__exp_tupleExp), r#mod)? + __exp_index.clone(),
        Expression::RECORD_ELEMENT {
            index: __exp_index,
            recordExp: __exp_recordExp,
            ..
        } => noNameHashExp(metamodelica::AsArg::as_arg(&__exp_recordExp), r#mod)? + __exp_index.clone(),
        Expression::MUTABLE { exp: __exp_exp } => noNameHashExp(&(Mutable::access(__exp_exp.clone())), r#mod)?,
        Expression::EMPTY { .. } => stringHashDjb2Mod(&(literal!("empty")), r#mod),
        Expression::PARTIAL_FUNCTION_APPLICATION { args: __exp_args, .. } => {
            for mut arg in &*__exp_args.clone() {
                hash = hash + noNameHashExp(metamodelica::AsArg::as_arg(&arg), r#mod)?;
            }
            hash
        }
        _ => 0,
    });
    hash = intMod(intAbs(hash), r#mod);
    Ok(hash)
}

pub(crate) fn isOnlyTimeDependent(mut exp: metamodelica::Ref<Expression::NFExpression>) -> Result<bool> {
    let mut b: bool;
    b = Expression::fold(
        exp,
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<Expression::NFExpression>, __a1: bool| {
            isOnlyTimeDependentFold(&__a0, __a1)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, bool) -> Result<bool> + 'static,
            >),
        true,
    )?;
    Ok(b)
}

pub(crate) fn isOnlyTimeDependentFold(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut b: bool,
) -> Result<bool> {
    let mut b: bool = b;
    if b {
        b = (match &**exp {
            Expression::CREF { cref: __exp_cref, .. } => {
                ComponentRef::isTime(metamodelica::AsArg::as_arg(&__exp_cref))?
                    || BVariable::checkCref(
                        metamodelica::AsArg::as_arg(&__exp_cref),
                        &fnptr!(
                            BVariable::isParamOrConst,
                            Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
                        ),
                        metamodelica::sourceInfo!("NBackEnd/Util/NBBackendUtil.mo"),
                    )?
            }
            _ => true,
        });
    }
    Ok(b)
}

pub(crate) fn isContinuous(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut staticAsContinuous: bool,
) -> Result<bool> {
    let mut b: bool;
    b = Expression::fold(
        exp,
        (std::sync::Arc::new({
            let __pe_b1 = staticAsContinuous;
            move |__pe_a0, __pe_a2| isContinuousFold(&__pe_a0, __pe_b1.clone(), __pe_a2)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, bool) -> Result<bool> + 'static,
            >),
        true,
    )?;
    Ok(b)
}

pub(crate) fn isContinuousFold(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut staticAsContinuous: bool,
    mut b: bool,
) -> Result<bool> {
    let mut b: bool = b;
    if b {
        b = (match &**exp {
            Expression::CREF { cref: __exp_cref, .. } => BVariable::checkCref(
                metamodelica::AsArg::as_arg(&__exp_cref),
                &({
                    let __pe_b1 = staticAsContinuous;
                    move |__pe_a0| BVariable::isContinuous(__pe_a0, __pe_b1.clone())
                }),
                metamodelica::sourceInfo!("NBackEnd/Util/NBBackendUtil.mo"),
            )?,
            _ => true,
        });
    }
    Ok(b)
}

pub(crate) fn containsContinuousVar(mut exp: metamodelica::Ref<Expression::NFExpression>) -> Result<bool> {
    let mut b: bool;
    b = Expression::fold(
        exp,
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<Expression::NFExpression>, __a1: bool| {
            containsContinuousVarFold(&__a0, __a1)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, bool) -> Result<bool> + 'static,
            >),
        false,
    )?;
    Ok(b)
}

pub(crate) fn containsContinuousVarFold(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut b: bool,
) -> Result<bool> {
    let mut b: bool = b;
    if !(b) {
        b = (match &**exp {
            Expression::CREF { cref: __exp_cref, .. } => BVariable::checkCref(
                metamodelica::AsArg::as_arg(&__exp_cref),
                &({
                    let __pe_b1 = false;
                    move |__pe_a0| BVariable::isContinuous(__pe_a0, __pe_b1.clone())
                }),
                metamodelica::sourceInfo!("NBackEnd/Util/NBBackendUtil.mo"),
            )?,
            _ => false,
        });
    }
    Ok(b)
}

pub(crate) fn makeFDerString(mut r#str: ArcStr, mut i_opt: Option<i32>) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    let mut i: ArcStr = if ((i_opt).is_some()) {
        intString(i_opt.clone().ok_or("pattern mismatch")?)
    } else {
        literal!("")
    };
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*arcstr::literal!(BVariable::FUNCTION_DERIVATIVE_STR));
        __mm_s.push_str(&*i);
        __mm_s.push_str(&*literal!("_"));
        __mm_s.push_str(&*r#str);
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}
