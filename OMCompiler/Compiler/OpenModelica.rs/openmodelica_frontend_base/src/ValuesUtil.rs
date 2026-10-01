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

use crate::Expression;
use crate::ExpressionSimplify;
use crate::Types;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_inst::ExpressionSimplifyTypes;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Print;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

pub fn typeConvert(
    mut inType1: metamodelica::Ref<DAE::Type>,
    mut inType2: metamodelica::Ref<DAE::Type>,
    mut inValueLst3: &metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut outValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>;
    outValueLst = (::match_deref::match_deref! { match &((inType1, inType2, &**inValueLst3)) {
        (_, _, Deref @ metamodelica::ListNode::Nil) => {
            metamodelica::nil()
        },
        (from @ Deref @ DAE::Type::T_INTEGER { .. }, to @ Deref @ DAE::Type::T_REAL { .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: vrest }) => {
            let mut vallst: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut rval: metamodelica::Real;
            vallst = typeConvert(from.clone(), to.clone(), metamodelica::AsArg::as_arg(&vrest))?;
            rval = intReal(i.clone());
            metamodelica::cons(metamodelica::Ref::new(Values::Value::REAL { real: rval }), vallst)
        },
        (from @ Deref @ DAE::Type::T_REAL { .. }, to @ Deref @ DAE::Type::T_INTEGER { .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: r }, tail: vrest }) => {
            let mut vallst: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut ival: i32;
            vallst = typeConvert(from.clone(), to.clone(), metamodelica::AsArg::as_arg(&vrest))?;
            ival = ((r.clone()).0.floor() as i32);
            metamodelica::cons(metamodelica::Ref::new(Values::Value::INTEGER { integer: ival }), vallst)
        },
        (from, to, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, dimLst: dims }, tail: vrest }) => {
            let mut vallst: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut vallst2: metamodelica::List<metamodelica::Ref<Values::Value>>;
            vallst = typeConvert(from.clone(), to.clone(), metamodelica::AsArg::as_arg(&vals))?;
            vallst2 = typeConvert(from.clone(), to.clone(), metamodelica::AsArg::as_arg(&vrest))?;
            metamodelica::cons(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: vallst, dimLst: dims.clone() }), vallst2)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outValueLst)
}

pub(crate) fn valueExpType(mut inValue: &metamodelica::Ref<Values::Value>) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut tp: metamodelica::Ref<DAE::Type>;
    tp = 'mc: {
        let __mc_input = &**inValue;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::INTEGER { integer: _ } => {
                    Ok(DAE::T_INTEGER_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::REAL { real: _ } => {
                    Ok(DAE::T_REAL_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::BOOL { boolean: _ } => {
                    Ok(DAE::T_BOOL_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::STRING { string: _ } => {
                    Ok(DAE::T_STRING_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ENUM_LITERAL { name: path, .. } => {
                    let mut path = (*path).clone();
                    path = AbsynUtil::pathPrefix(metamodelica::AsArg::as_arg(&path))?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: None, path: path.clone(), names: metamodelica::nil(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ARRAY { valueLst: valLst, dimLst: int_dims } => {
                    let mut eltTp: metamodelica::Ref<DAE::Type>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    eltTp = valueExpType(&((valLst).head().cloned()?))?;
                    dims = List::map(int_dims.clone(), &fnptr!(Expression::intDimension, i32))?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: eltTp.clone(), dims: dims.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::RECORD { record_: path, orderd: valLst, comp: nameLst, index: _ } => {
                    let mut eltTps: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut varLst: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    eltTps = List::map(valLst.clone(), &move |__a0: metamodelica::Ref<Values::Value>| valueExpType(&__a0))?;
                    varLst = List::threadMap(eltTps.clone(), nameLst.clone(), &fnptr!(valueExpTypeExpVar, metamodelica::Ref<DAE::Type>, ArcStr))?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: path.clone() }, varLst: varLst.clone(), equalityConstraint: None, usedExternally: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("valueExpType on ")); __mm_s.push_str(&*ValuesDump::valString(inValue)?); __mm_s.push_str(&*literal!(" not implemented yet\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(tp)
}

fn valueExpTypeExpVar(mut etp: metamodelica::Ref<DAE::Type>, mut name: ArcStr) -> metamodelica::Ref<DAE::Var> {
    let mut expVar: metamodelica::Ref<DAE::Var>;
    expVar = metamodelica::Ref::new(DAE::Var {
        name: name,
        attributes: DAE::dummyAttrVar().clone(),
        ty: etp,
        binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(),
        bind_from_outside: false,
        constOfForIteratorRange: None,
    });
    expVar
}

pub fn isZero(mut inValue: &metamodelica::Ref<Values::Value>) -> bool {
    let mut isZero: bool;
    isZero = (match &**inValue {
        Values::Value::REAL { real: rval } => realEq(rval.clone(), metamodelica::OrderedFloat(0.0_f64)),
        Values::Value::INTEGER { integer: ival } => intEq(ival.clone(), 0),
        _ => false,
    });
    isZero
}

pub fn isArray(mut inValue: &metamodelica::Ref<Values::Value>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inValue {
        Values::Value::ARRAY { .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isRecord(mut inValue: &metamodelica::Ref<Values::Value>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inValue {
        Values::Value::RECORD { .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn nthArrayelt(
    mut inValue: &metamodelica::Ref<Values::Value>,
    mut inInteger: i32,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    let mut vlst: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let __pa0 = ::match_deref::match_deref! { match &((*inValue)) {
        Deref @ Values::Value::ARRAY { valueLst: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    vlst = metamodelica::Own::own(__pa0);
    outValue = (vlst).get(inInteger)?;
    Ok(outValue)
}

pub fn safeIntRealOp(
    mut val1: &metamodelica::Ref<Values::Value>,
    mut val2: &metamodelica::Ref<Values::Value>,
    mut op: Values::IntRealOp,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outv: metamodelica::Ref<Values::Value> = metamodelica::Ref::new(Values::Value::META_FAIL);
    outv = 'mc: {
        let __mc_input = (&**val1, &**val2, op);
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::MULOP { .. }) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut outv: metamodelica::Ref<Values::Value> = outv.clone();
                    e = ExpressionSimplify::safeIntOp(iv1.clone(), iv2.clone(), openmodelica_frontend_inst::ExpressionSimplifyTypes::IntOp::MULOP);
                    outv = expValue(&e)?;
                    Ok((outv.clone(), outv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outv = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::MULOP { .. }) => {
                    let mut rv2: metamodelica::Real;
                    let mut rv3: metamodelica::Real;
                    rv2 = intReal(iv2.clone());
                    rv3 = rv1.clone() * rv2;
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: rv3 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::MULOP { .. }) => {
                    let mut rv1: metamodelica::Real;
                    let mut rv3: metamodelica::Real;
                    rv1 = intReal(iv1.clone());
                    rv3 = rv1 * rv2.clone();
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: rv3 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::MULOP { .. }) => {
                    let mut rv3: metamodelica::Real;
                    rv3 = rv1.clone() * rv2.clone();
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: rv3 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::DIVOP { .. }) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut outv: metamodelica::Ref<Values::Value> = outv.clone();
                    e = ExpressionSimplify::safeIntOp(iv1.clone(), iv2.clone(), openmodelica_frontend_inst::ExpressionSimplifyTypes::IntOp::DIVOP);
                    outv = expValue(&e)?;
                    Ok((outv.clone(), outv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outv = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::DIVOP { .. }) => {
                    let mut rv2: metamodelica::Real;
                    let mut rv3: metamodelica::Real;
                    rv2 = intReal(iv2.clone());
                    rv3 = metamodelica::real_div_checked(rv1.clone(), rv2)?;
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: rv3 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::DIVOP { .. }) => {
                    let mut rv1: metamodelica::Real;
                    let mut rv3: metamodelica::Real;
                    rv1 = intReal(iv1.clone());
                    rv3 = metamodelica::real_div_checked(rv1, rv2.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: rv3 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::DIVOP { .. }) => {
                    let mut rv3: metamodelica::Real;
                    rv3 = metamodelica::real_div_checked(rv1.clone(), rv2.clone())?;
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: rv3 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::POWOP { .. }) => {
                    let mut rv1: metamodelica::Real;
                    let mut rv2: metamodelica::Real;
                    let mut rv3: metamodelica::Real;
                    let true = (iv2.clone() < 0) else { return Err("pattern mismatch") };
                    rv1 = intReal(iv1.clone());
                    rv2 = intReal(iv2.clone());
                    rv3 = realPow(rv1, rv2);
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: rv3 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::POWOP { .. }) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut outv: metamodelica::Ref<Values::Value> = outv.clone();
                    e = ExpressionSimplify::safeIntOp(iv1.clone(), iv2.clone(), openmodelica_frontend_inst::ExpressionSimplifyTypes::IntOp::POWOP);
                    outv = expValue(&e)?;
                    Ok((outv.clone(), outv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outv = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::POWOP { .. }) => {
                    let mut rv2: metamodelica::Real;
                    let mut rv3: metamodelica::Real;
                    rv2 = intReal(iv2.clone());
                    rv3 = realPow(rv1.clone(), rv2);
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: rv3 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::POWOP { .. }) => {
                    let mut iv2: i32;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut outv: metamodelica::Ref<Values::Value> = outv.clone();
                    iv2 = ((rv2.clone()).0.floor() as i32);
                    e = ExpressionSimplify::safeIntOp(iv1.clone(), iv2, openmodelica_frontend_inst::ExpressionSimplifyTypes::IntOp::POWOP);
                    outv = expValue(&e)?;
                    Ok((outv.clone(), outv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outv = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::POWOP { .. }) => {
                    let mut rv1: metamodelica::Real;
                    let mut rv3: metamodelica::Real;
                    rv1 = intReal(iv1.clone());
                    rv3 = realPow(rv1, rv2.clone());
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: rv3 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::POWOP { .. }) => {
                    let mut rv3: metamodelica::Real;
                    rv3 = realPow(rv1.clone(), rv2.clone());
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: rv3 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::ADDOP { .. }) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut outv: metamodelica::Ref<Values::Value> = outv.clone();
                    e = ExpressionSimplify::safeIntOp(iv1.clone(), iv2.clone(), openmodelica_frontend_inst::ExpressionSimplifyTypes::IntOp::ADDOP);
                    outv = expValue(&e)?;
                    Ok((outv.clone(), outv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outv = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::ADDOP { .. }) => {
                    let mut rv2: metamodelica::Real;
                    let mut rv3: metamodelica::Real;
                    rv2 = intReal(iv2.clone());
                    rv3 = rv1.clone() + rv2;
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: rv3 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::ADDOP { .. }) => {
                    let mut rv1: metamodelica::Real;
                    let mut rv3: metamodelica::Real;
                    rv1 = intReal(iv1.clone());
                    rv3 = rv1 + rv2.clone();
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: rv3 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::ADDOP { .. }) => {
                    let mut rv3: metamodelica::Real;
                    rv3 = rv1.clone() + rv2.clone();
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: rv3 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::SUBOP { .. }) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut outv: metamodelica::Ref<Values::Value> = outv.clone();
                    e = ExpressionSimplify::safeIntOp(iv1.clone(), iv2.clone(), openmodelica_frontend_inst::ExpressionSimplifyTypes::IntOp::SUBOP);
                    outv = expValue(&e)?;
                    Ok((outv.clone(), outv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outv = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::INTEGER { integer: iv2 }, Values::IntRealOp::SUBOP { .. }) => {
                    let mut rv2: metamodelica::Real;
                    let mut rv3: metamodelica::Real;
                    rv2 = intReal(iv2.clone());
                    rv3 = rv1.clone() - rv2;
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: rv3 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: iv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::SUBOP { .. }) => {
                    let mut rv1: metamodelica::Real;
                    let mut rv3: metamodelica::Real;
                    rv1 = intReal(iv1.clone());
                    rv3 = rv1 - rv2.clone();
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: rv3 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: rv1 }, Deref @ Values::Value::REAL { real: rv2 }, Values::IntRealOp::SUBOP { .. }) => {
                    let mut rv3: metamodelica::Real;
                    rv3 = rv1.clone() - rv2.clone();
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: rv3 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outv)
}

pub fn safeLessEq(
    mut val1: &metamodelica::Ref<Values::Value>,
    mut val2: &metamodelica::Ref<Values::Value>,
) -> Result<bool> {
    let mut outv: bool;
    outv = (::match_deref::match_deref! { match (val1, val2) {
        (Deref @ Values::Value::REAL { real: r1 }, Deref @ Values::Value::REAL { real: r2 }) => {
            r1.clone() <= r2.clone()
        },
        (Deref @ Values::Value::REAL { real: r1 }, _) => {
            let mut r2: metamodelica::Real;
            r2 = intReal(valueInteger(val2)?);
            r1.clone() <= r2
        },
        (_, Deref @ Values::Value::REAL { real: r2 }) => {
            let mut r1: metamodelica::Real;
            r1 = intReal(valueInteger(val1)?);
            r1 <= r2.clone()
        },
        (_, _) => {
            let mut i1: i32;
            let mut i2: i32;
            i1 = valueInteger(val1)?;
            i2 = valueInteger(val2)?;
            i1 <= i2
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outv)
}

pub(crate) fn writeToFileAsArgs(
    mut vallst: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut filename: ArcStr,
) -> Result<()> {
    let mut r#str: ArcStr;
    r#str = ValuesDump::unparseValues(vallst)?;
    System::writeFile(filename, r#str)?;
    Ok(())
}

pub fn addElementwiseArrayelt(
    mut inValueLst1: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inValueLst2: &metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut outValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>;
    outValueLst = (::match_deref::match_deref! { match (inValueLst1, inValueLst2) {
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: v1lst, dimLst: dims }, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: v2lst, .. }, tail: rest2 }) => {
            let mut reslst: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut res2: metamodelica::List<metamodelica::Ref<Values::Value>>;
            reslst = addElementwiseArrayelt(metamodelica::AsArg::as_arg(&v1lst), metamodelica::AsArg::as_arg(&v2lst))?;
            res2 = addElementwiseArrayelt(rest1, rest2)?;
            metamodelica::cons(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: reslst, dimLst: dims.clone() }), res2)
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: v1 }, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: v2 }, tail: rest2 }) => {
            let mut res2: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut res: i32;
            res = v1.clone() + v2.clone();
            res2 = addElementwiseArrayelt(rest1, rest2)?;
            metamodelica::cons(metamodelica::Ref::new(Values::Value::INTEGER { integer: res }), res2)
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: r1 }, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: r2 }, tail: rest2 }) => {
            let mut res2: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut rres: metamodelica::Real;
            rres = r1.clone() + r2.clone();
            res2 = addElementwiseArrayelt(rest1, rest2)?;
            metamodelica::cons(metamodelica::Ref::new(Values::Value::REAL { real: rres }), res2)
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: s1 }, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: s2 }, tail: rest2 }) => {
            let mut res2: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut sres: ArcStr;
            sres = stringAppend(s1.clone(), s2.clone());
            res2 = addElementwiseArrayelt(rest1, rest2)?;
            metamodelica::cons(metamodelica::Ref::new(Values::Value::STRING { string: sres }), res2)
        },
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            metamodelica::nil()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outValueLst)
}

pub fn subElementwiseArrayelt(
    mut inValueLst1: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inValueLst2: &metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut outValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>;
    outValueLst = (::match_deref::match_deref! { match (inValueLst1, inValueLst2) {
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: v1lst, dimLst: dims }, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: v2lst, .. }, tail: rest2 }) => {
            let mut reslst: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut res2: metamodelica::List<metamodelica::Ref<Values::Value>>;
            reslst = subElementwiseArrayelt(metamodelica::AsArg::as_arg(&v1lst), metamodelica::AsArg::as_arg(&v2lst))?;
            res2 = subElementwiseArrayelt(rest1, rest2)?;
            metamodelica::cons(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: reslst, dimLst: dims.clone() }), res2)
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: v1 }, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: v2 }, tail: rest2 }) => {
            let mut res2: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut res: i32;
            res = v1.clone() - v2.clone();
            res2 = subElementwiseArrayelt(rest1, rest2)?;
            metamodelica::cons(metamodelica::Ref::new(Values::Value::INTEGER { integer: res }), res2)
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: r1 }, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: r2 }, tail: rest2 }) => {
            let mut res2: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut rres: metamodelica::Real;
            rres = r1.clone() - r2.clone();
            res2 = subElementwiseArrayelt(rest1, rest2)?;
            metamodelica::cons(metamodelica::Ref::new(Values::Value::REAL { real: rres }), res2)
        },
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            metamodelica::nil()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outValueLst)
}

pub fn mulElementwiseArrayelt(
    mut inValueLst1: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inValueLst2: &metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut outValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>;
    outValueLst = (::match_deref::match_deref! { match (inValueLst1, inValueLst2) {
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: v1lst, dimLst: dims }, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: v2lst, .. }, tail: rest2 }) => {
            let mut reslst: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut res2: metamodelica::List<metamodelica::Ref<Values::Value>>;
            reslst = mulElementwiseArrayelt(metamodelica::AsArg::as_arg(&v1lst), metamodelica::AsArg::as_arg(&v2lst))?;
            res2 = mulElementwiseArrayelt(rest1, rest2)?;
            metamodelica::cons(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: reslst, dimLst: dims.clone() }), res2)
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: v1 }, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: v2 }, tail: rest2 }) => {
            let mut res2: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut res: i32;
            res = v1.clone() * v2.clone();
            res2 = mulElementwiseArrayelt(rest1, rest2)?;
            metamodelica::cons(metamodelica::Ref::new(Values::Value::INTEGER { integer: res }), res2)
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: r1 }, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: r2 }, tail: rest2 }) => {
            let mut res2: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut rres: metamodelica::Real;
            rres = r1.clone() * r2.clone();
            res2 = mulElementwiseArrayelt(rest1, rest2)?;
            metamodelica::cons(metamodelica::Ref::new(Values::Value::REAL { real: rres }), res2)
        },
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            metamodelica::nil()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outValueLst)
}

pub fn divElementwiseArrayelt(
    mut inValueLst1: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inValueLst2: &metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut outValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>;
    outValueLst = (::match_deref::match_deref! { match (inValueLst1, inValueLst2) {
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: v1lst, dimLst: dims }, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: v2lst, .. }, tail: rest2 }) => {
            let mut reslst: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut res2: metamodelica::List<metamodelica::Ref<Values::Value>>;
            reslst = divElementwiseArrayelt(metamodelica::AsArg::as_arg(&v1lst), metamodelica::AsArg::as_arg(&v2lst))?;
            res2 = divElementwiseArrayelt(rest1, rest2)?;
            metamodelica::cons(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: reslst, dimLst: dims.clone() }), res2)
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i1 }, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i2 }, tail: rest2 }) => {
            let mut res2: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut res: metamodelica::Real;
            let mut r1: metamodelica::Real;
            let mut r2: metamodelica::Real;
            r1 = intReal(i1.clone());
            r2 = intReal(i2.clone());
            res = metamodelica::real_div_checked(r1, r2)?;
            res2 = divElementwiseArrayelt(rest1, rest2)?;
            metamodelica::cons(metamodelica::Ref::new(Values::Value::REAL { real: res }), res2)
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: r1 }, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: r2 }, tail: rest2 }) => {
            let mut res2: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut res: metamodelica::Real;
            res = metamodelica::real_div_checked(r1.clone(), r2.clone())?;
            res2 = divElementwiseArrayelt(rest1, rest2)?;
            metamodelica::cons(metamodelica::Ref::new(Values::Value::REAL { real: res }), res2)
        },
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            metamodelica::nil()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outValueLst)
}

pub fn powElementwiseArrayelt(
    mut inValueLst1: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inValueLst2: &metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut outValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>;
    outValueLst = (::match_deref::match_deref! { match (inValueLst1, inValueLst2) {
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: v1lst, dimLst: dims }, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: v2lst, .. }, tail: rest2 }) => {
            let mut reslst: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut res2: metamodelica::List<metamodelica::Ref<Values::Value>>;
            reslst = powElementwiseArrayelt(metamodelica::AsArg::as_arg(&v1lst), metamodelica::AsArg::as_arg(&v2lst))?;
            res2 = powElementwiseArrayelt(rest1, rest2)?;
            metamodelica::cons(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: reslst, dimLst: dims.clone() }), res2)
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i1 }, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i2 }, tail: rest2 }) => {
            let mut res2: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut res: metamodelica::Real;
            let mut r1: metamodelica::Real;
            let mut r2: metamodelica::Real;
            r1 = intReal(i1.clone());
            r2 = intReal(i2.clone());
            res = (r1).powf(r2);
            res2 = powElementwiseArrayelt(rest1, rest2)?;
            metamodelica::cons(metamodelica::Ref::new(Values::Value::REAL { real: res }), res2)
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: r1 }, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: r2 }, tail: rest2 }) => {
            let mut res2: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut res: metamodelica::Real;
            res = (r1.clone()).powf(r2.clone());
            res2 = powElementwiseArrayelt(rest1, rest2)?;
            metamodelica::cons(metamodelica::Ref::new(Values::Value::REAL { real: res }), res2)
        },
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            metamodelica::nil()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outValueLst)
}

pub fn absynExpValue(mut exp: &metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut value: metamodelica::Ref<Values::Value>;
    value = (match &**exp {
        Absyn::Exp::INTEGER { value: __exp_value } => metamodelica::Ref::new(Values::Value::INTEGER {
            integer: __exp_value.clone(),
        }),
        Absyn::Exp::REAL { value: __exp_value } => metamodelica::Ref::new(Values::Value::REAL {
            real: stringReal(__exp_value.clone())?,
        }),
        Absyn::Exp::CREF {
            componentRef: __exp_componentRef,
        } => metamodelica::Ref::new(Values::Value::CODE {
            A: metamodelica::Ref::new(Absyn::CodeNode::C_VARIABLENAME {
                componentRef: __exp_componentRef.clone(),
            }),
        }),
        Absyn::Exp::STRING { value: __exp_value } => metamodelica::Ref::new(Values::Value::STRING {
            string: __exp_value.clone(),
        }),
        Absyn::Exp::BOOL { value: __exp_value } => metamodelica::Ref::new(Values::Value::BOOL {
            boolean: __exp_value.clone(),
        }),
        Absyn::Exp::ARRAY {
            arrayExp: __exp_arrayExp,
        } => ValuesMake::makeArray(
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
                for mut e in (__exp_arrayExp.clone()).into_iter().cloned() {
                    let __x = absynExpValue(&(e.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        ),
        Absyn::Exp::TUPLE {
            expressions: __exp_expressions,
        } => ValuesMake::makeTuple(
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
                for mut e in (__exp_expressions.clone()).into_iter().cloned() {
                    let __x = absynExpValue(&(e.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        ),
        Absyn::Exp::CODE { code: __exp_code } => metamodelica::Ref::new(Values::Value::CODE { A: __exp_code.clone() }),
        _ => metamodelica::Ref::new(Values::Value::CODE {
            A: metamodelica::Ref::new(Absyn::CodeNode::C_EXPRESSION { exp: exp.clone() }),
        }),
    });
    Ok(value)
}

pub fn expValue(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = (match &**inExp {
        DAE::Exp::ICONST { integer: i } => metamodelica::Ref::new(Values::Value::INTEGER { integer: i.clone() }),
        DAE::Exp::RCONST { real: r } => metamodelica::Ref::new(Values::Value::REAL { real: r.clone() }),
        DAE::Exp::SCONST { string: s } => metamodelica::Ref::new(Values::Value::STRING { string: s.clone() }),
        DAE::Exp::BCONST { bool: b } => metamodelica::Ref::new(Values::Value::BOOL { boolean: b.clone() }),
        _ => return Err("match: no arm matched"),
    });
    Ok(outValue)
}

pub fn valueExp(
    mut inValue: metamodelica::Ref<Values::Value>,
    mut originalExp: Option<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inValue) {
        Deref @ Values::Value::INTEGER { integer: i } => {
            metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() })
        },
        Deref @ Values::Value::REAL { real: r } => {
            metamodelica::Ref::new(DAE::Exp::RCONST { real: r.clone() })
        },
        Deref @ Values::Value::STRING { string: s } => {
            metamodelica::Ref::new(DAE::Exp::SCONST { string: s.clone() })
        },
        Deref @ Values::Value::BOOL { boolean: b } => {
            metamodelica::Ref::new(DAE::Exp::BCONST { bool: b.clone() })
        },
        Deref @ Values::Value::ENUM_LITERAL { name: path, index: i } => {
            metamodelica::Ref::new(DAE::Exp::ENUM_LITERAL { name: path.clone(), index: i.clone() })
        },
        Deref @ Values::Value::ARRAY { valueLst: vallist, dimLst: int_dims } => {
            valueExpArray(vallist.clone(), int_dims.clone(), originalExp)?
        },
        Deref @ Values::Value::TUPLE { valueLst: vallist } => {
            let mut explist: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            explist = List::map(vallist.clone(), &({ let __pe_b1 = None; move |__pe_a0| valueExp(__pe_a0, __pe_b1.clone()) }))?;
            metamodelica::Ref::new(DAE::Exp::TUPLE { PR: explist })
        },
        Deref @ Values::Value::RECORD { record_: path, orderd: vallist, comp: namelst, index: (-1) } => {
            let mut t: metamodelica::Ref<DAE::Type>;
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut tpl: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut varlst: metamodelica::List<metamodelica::Ref<DAE::Var>>;
            expl = List::map(vallist.clone(), &({ let __pe_b1 = None; move |__pe_a0| valueExp(__pe_a0, __pe_b1.clone()) }))?;
            tpl = List::map(expl.clone(), &Expression::r#typeof)?;
            varlst = List::threadMap(namelst.clone(), tpl, &fnptr!(Expression::makeVar, ArcStr, metamodelica::Ref<DAE::Type>))?;
            t = metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: path.clone() }, varLst: varlst, equalityConstraint: None, usedExternally: false });
            metamodelica::Ref::new(DAE::Exp::RECORD { path: path.clone(), exps: expl, comp: namelst.clone(), ty: t })
        },
        Deref @ Values::Value::ENUM_LITERAL { name: path, index: ix } => {
            metamodelica::Ref::new(DAE::Exp::ENUM_LITERAL { name: path.clone(), index: ix.clone() })
        },
        Deref @ Values::Value::TUPLE { valueLst: vallist } => {
            let mut explist: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            explist = List::map(vallist.clone(), &({ let __pe_b1 = None; move |__pe_a0| valueExp(__pe_a0, __pe_b1.clone()) }))?;
            metamodelica::Ref::new(DAE::Exp::TUPLE { PR: explist })
        },
        Deref @ Values::Value::OPTION { some: Some(v) } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = valueExp(v.clone(), None)?;
            (e, _) = Types::matchType(e, Types::typeOfValue(v.clone())?, DAE::T_METABOXED_DEFAULT().clone(), true)?;
            metamodelica::Ref::new(DAE::Exp::META_OPTION { exp: Some(e) })
        },
        Deref @ Values::Value::OPTION { some: None } => {
            metamodelica::Ref::new(DAE::Exp::META_OPTION { exp: None })
        },
        Deref @ Values::Value::META_TUPLE { valueLst: vallist } => {
            let mut explist: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut typelist: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            explist = List::map(vallist.clone(), &({ let __pe_b1 = None; move |__pe_a0| valueExp(__pe_a0, __pe_b1.clone()) }))?;
            typelist = List::map(vallist.clone(), &Types::typeOfValue)?;
            (explist, _) = Types::matchTypeTuple(&explist, &(typelist.clone()), &(List::map(typelist, &fnptr!(Types::boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?), true)?;
            metamodelica::Ref::new(DAE::Exp::META_TUPLE { listExp: explist })
        },
        Deref @ Values::Value::LIST { valueLst: Deref @ metamodelica::ListNode::Nil } => {
            metamodelica::Ref::new(DAE::Exp::LIST { valList: metamodelica::nil() })
        },
        Deref @ Values::Value::LIST { valueLst: vallist } => {
            let mut explist: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut vt: metamodelica::Ref<DAE::Type>;
            let mut typelist: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            explist = List::map(vallist.clone(), &({ let __pe_b1 = None; move |__pe_a0| valueExp(__pe_a0, __pe_b1.clone()) }))?;
            typelist = List::map(vallist.clone(), &Types::typeOfValue)?;
            vt = Types::boxIfUnboxedType(List::reduce(&typelist, &Types::superType)?);
            (explist, _) = Types::matchTypes(explist, typelist, &vt, true)?;
            metamodelica::Ref::new(DAE::Exp::LIST { valList: explist })
        },
        Deref @ Values::Value::META_ARRAY { valueLst: vallist } => {
            let mut explist: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut vt: metamodelica::Ref<DAE::Type>;
            let mut typelist: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            explist = List::map(vallist.clone(), &({ let __pe_b1 = None; move |__pe_a0| valueExp(__pe_a0, __pe_b1.clone()) }))?;
            typelist = List::map(vallist.clone(), &Types::typeOfValue)?;
            vt = Types::boxIfUnboxedType(List::reduce(&typelist, &Types::superType)?);
            (explist, _) = Types::matchTypes(explist, typelist, &vt, true)?;
            Expression::makeBuiltinCall(literal!("listArrayLiteral"), list![metamodelica::Ref::new(DAE::Exp::LIST { valList: explist })], metamodelica::Ref::new(DAE::Type::T_METAARRAY { ty: vt }), false)
        },
        Deref @ Values::Value::RECORD { record_: path, orderd: vallist, comp: namelst, index: ix } => {
            let mut explist: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut typelist: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let true = (ix.clone() >= 0) else { return Err("pattern mismatch") };
            explist = List::map(vallist.clone(), &({ let __pe_b1 = None; move |__pe_a0| valueExp(__pe_a0, __pe_b1.clone()) }))?;
            typelist = List::map(vallist.clone(), &Types::typeOfValue)?;
            (explist, _) = Types::matchTypeTuple(&explist, &(typelist.clone()), &(List::map(typelist, &fnptr!(Types::boxIfUnboxedType, metamodelica::Ref<DAE::Type>))?), true)?;
            metamodelica::Ref::new(DAE::Exp::METARECORDCALL { path: path.clone(), args: explist, fieldNames: namelst.clone(), index: ix.clone(), typeVars: metamodelica::nil() })
        },
        Deref @ Values::Value::META_FAIL { .. } => {
            metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("fail") }), expLst: metamodelica::nil(), attr: DAE::callAttrBuiltinOther().clone() })
        },
        Deref @ Values::Value::META_BOX { value: v } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = valueExp(v.clone(), None)?;
            metamodelica::Ref::new(DAE::Exp::BOX { exp: e })
        },
        Deref @ Values::Value::CODE { A: code } => {
            metamodelica::Ref::new(DAE::Exp::CODE { code: code.clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone() })
        },
        Deref @ Values::Value::EMPTY { scope, name, tyStr, ty: valType } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ety: metamodelica::Ref<DAE::Type>;
            if (originalExp).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(originalExp) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                e = metamodelica::Own::own(__pa0);
            } else {
                ety = Types::simplifyType(Types::typeOfValue(valType.clone())?)?;
                e = metamodelica::Ref::new(DAE::Exp::EMPTY { scope: scope.clone(), name: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: name.clone(), identType: ety.clone(), subscriptLst: metamodelica::nil() }), ty: ety, tyStr: tyStr.clone() });
            }
            e
        },
        Deref @ Values::Value::NORETCALL { .. } => {
            metamodelica::Ref::new(DAE::Exp::TUPLE { PR: metamodelica::nil() })
        },
        v => {
            let mut s: ArcStr;
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ValuesUtil.valueExp failed for ")); __mm_s.push_str(&*ValuesDump::valString(metamodelica::AsArg::as_arg(&v))?); ArcStr::from(__mm_s) };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![s])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn valueExpNoOriginal(mut inValue: metamodelica::Ref<Values::Value>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = valueExp(inValue.clone(), None)?;
    Ok(outExp)
}

fn valueExpArray(
    mut values: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inDims: metamodelica::List<i32>,
    mut originalExp: Option<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (&*values, &*inDims, originalExp);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: DAE::T_UNKNOWN_DEFAULT().clone(), scalar: false, array: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    dims = List::map(inDims.clone(), &fnptr!(Expression::intDimension, i32))?;
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_UNKNOWN_DEFAULT().clone(), dims: dims.clone() }), scalar: false, array: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: v, tail: xs }, .. }, tail: xs2 }, Deref @ metamodelica::ListNode::Cons { head: dim, tail: int_dims }, _) => {
                    let mut explist: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut mexpl: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                    if '__try0: {
                        ::match_deref::match_deref! { match &(v.clone()) {
                            Deref @ Values::Value::ARRAY { .. } => (),
                            _ => break '__try0 Err::<_, _>("pattern mismatch"),
                        } };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    explist = List::map(metamodelica::cons(v.clone(), xs.clone()), &({ let __pe_b1 = None; move |__pe_a0| valueExp(__pe_a0, __pe_b1.clone()) }))?;
                    let (__pa1, __pa2) = ::match_deref::match_deref! { match &(valueExp(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: xs2.clone(), dimLst: int_dims.clone() }), None)?) {
                        Deref @ DAE::Exp::MATRIX { ty: __pa1, integer: _, matrix: __pa2 } => (__pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    t = metamodelica::Own::own(__pa1);
                    mexpl = metamodelica::Own::own(__pa2);
                    t = Expression::arrayDimensionSetFirst(&t, metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim.clone() }))?;
                    Ok(metamodelica::Ref::new(DAE::Exp::MATRIX { ty: t.clone(), integer: dim.clone(), matrix: metamodelica::cons(explist.clone(), mexpl.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: v, tail: xs }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, _, _) => {
                    let mut explist: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut vt: metamodelica::Ref<DAE::Type>;
                    let mut dim: i32;
                    if '__try0: {
                        ::match_deref::match_deref! { match &(v.clone()) {
                            Deref @ Values::Value::ARRAY { .. } => (),
                            _ => break '__try0 Err::<_, _>("pattern mismatch"),
                        } };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    dim = (((metamodelica::cons(v.clone(), xs.clone()))).len() as i32);
                    explist = List::map(metamodelica::cons(v.clone(), xs.clone()), &({ let __pe_b1 = None; move |__pe_a0| valueExp(__pe_a0, __pe_b1.clone()) }))?;
                    vt = Types::typeOfValue(v.clone())?;
                    t = Types::simplifyType(vt.clone())?;
                    dim = (((metamodelica::cons(v.clone(), xs.clone()))).len() as i32);
                    t = Expression::liftArrayR(t.clone(), metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim }));
                    t = Expression::liftArrayR(t.clone(), metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 1 }));
                    Ok(metamodelica::Ref::new(DAE::Exp::MATRIX { ty: t.clone(), integer: dim, matrix: list![explist.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ metamodelica::ListNode::Cons { head: v, tail: xs }, _, Some(Deref @ DAE::Exp::ARRAY { array: exps1, .. })) => {
                            let mut explist: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut t: metamodelica::Ref<DAE::Type>;
                            let mut vt: metamodelica::Ref<DAE::Type>;
                            let mut dim: i32;
                            let mut b: bool;
                            explist = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                let __thr_src0 = values.clone();
                let mut __thr_it0 = (&__thr_src0).into_iter();
                let __thr_src1 = exps1.clone();
                let mut __thr_it1 = (&__thr_src1).into_iter();
                loop {
                            match (__thr_it0.next(), __thr_it1.next()) {
                                (Some(e1), Some(e2)) => {
                                    let __x = valueExp(e1.clone(), Some(e2.clone()))?;
                                    __acc = cons(__x, __acc);
                                }
                                (None, None) => break,
                                _ => return Err("threaded for: ranges of unequal length"),
                            }
                }
                __acc.reverse()
            });
                            vt = Types::typeOfValue(v.clone())?;
                            t = Types::simplifyType(vt.clone())?;
                            dim = (((metamodelica::cons(v.clone(), xs.clone()))).len() as i32);
                            t = Expression::liftArrayR(t.clone(), metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim }));
                            b = Types::isArray(&vt);
                            b = boolNot(b);
                            Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: t.clone(), scalar: b, array: explist.clone() }))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: v, tail: xs }, _, _) => {
                    let mut explist: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut vt: metamodelica::Ref<DAE::Type>;
                    let mut dim: i32;
                    let mut b: bool;
                    explist = List::map(metamodelica::cons(v.clone(), xs.clone()), &({ let __pe_b1 = None; move |__pe_a0| valueExp(__pe_a0, __pe_b1.clone()) }))?;
                    vt = Types::typeOfValue(v.clone())?;
                    t = Types::simplifyType(vt.clone())?;
                    dim = (((metamodelica::cons(v.clone(), xs.clone()))).len() as i32);
                    t = Expression::liftArrayR(t.clone(), metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim }));
                    b = Types::isArray(&vt);
                    b = boolNot(b);
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: t.clone(), scalar: b, array: explist.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

pub fn valueReal(mut inValue: &metamodelica::Ref<Values::Value>) -> Result<metamodelica::Real> {
    let mut outReal: metamodelica::Real;
    outReal = (match &**inValue {
        Values::Value::REAL { real: __inValue_real } => __inValue_real.clone(),
        Values::Value::INTEGER {
            integer: __inValue_integer,
        } => intReal(__inValue_integer.clone()),
        _ => return Err("match: no arm matched"),
    });
    Ok(outReal)
}

pub fn valueBool(mut inValue: &metamodelica::Ref<Values::Value>) -> Result<bool> {
    let mut outBool: bool;
    let __pa0 = ::match_deref::match_deref! { match &((*inValue)) {
        Deref @ Values::Value::BOOL { boolean: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outBool = metamodelica::Own::own(__pa0);
    Ok(outBool)
}

pub fn valueReals<'__b>(
    mut inValue: &'__b metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> metamodelica::List<metamodelica::Real> {
    '__tco: loop {
        ::match_deref::match_deref! { match inValue {
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: r }, tail: rest } => {
                let mut res: metamodelica::List<metamodelica::Real>;
                res = valueReals(rest);
                return metamodelica::cons(r.clone(), res)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: rest } => {
                let mut r: metamodelica::Real;
                let mut res: metamodelica::List<metamodelica::Real>;
                r = intReal(i.clone());
                res = valueReals(rest);
                return metamodelica::cons(r, res)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                let mut res: metamodelica::List<metamodelica::Real>;
                { inValue = rest; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn valueString(mut value: &metamodelica::Ref<Values::Value>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &((*value)) {
        Deref @ Values::Value::STRING { string: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#str = metamodelica::Own::own(__pa0);
    Ok(r#str)
}

pub fn arrayValueInts(mut inValue: &metamodelica::Ref<Values::Value>) -> Result<metamodelica::List<i32>> {
    let mut outReal: metamodelica::List<i32>;
    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let __pa0 = ::match_deref::match_deref! { match &((*inValue)) {
        Deref @ Values::Value::ARRAY { valueLst: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    vals = metamodelica::Own::own(__pa0);
    outReal = List::map(vals, &move |__a0: metamodelica::Ref<Values::Value>| valueInteger(&__a0))?;
    Ok(outReal)
}

pub fn arrayValueReals(
    mut inValue: &metamodelica::Ref<Values::Value>,
) -> Result<metamodelica::List<metamodelica::Real>> {
    let mut outReal: metamodelica::List<metamodelica::Real>;
    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let __pa0 = ::match_deref::match_deref! { match &((*inValue)) {
        Deref @ Values::Value::ARRAY { valueLst: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    vals = metamodelica::Own::own(__pa0);
    outReal = valueReals(&vals);
    Ok(outReal)
}

pub fn matrixValueReals(
    mut inValue: &metamodelica::Ref<Values::Value>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Real>>> {
    let mut outReals: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    outReals = 'mc: {
        let __mc_input = &**inValue;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ARRAY { valueLst: vals, .. } => {
                    Ok(List::map(vals.clone(), &move |__a0: metamodelica::Ref<Values::Value>| arrayValueReals(&__a0))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ARRAY { valueLst: vals, .. } => {
                    let mut reals: metamodelica::List<metamodelica::Real>;
                    reals = valueReals(metamodelica::AsArg::as_arg(&vals));
                    Ok(List::map(reals.clone(), &fnptr!(List::create, _))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outReals)
}

pub fn arrayValueStrings(mut value: &metamodelica::Ref<Values::Value>) -> Result<metamodelica::List<ArcStr>> {
    let mut strings: metamodelica::List<ArcStr>;
    strings = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut v in (arrayValues(value)?).into_iter().cloned() {
            let __x = valueString(&(v.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(strings)
}

pub fn valueNeg(mut inValue: &metamodelica::Ref<Values::Value>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = (match &**inValue {
        Values::Value::REAL { real: r } => {
            let mut r_1: metamodelica::Real;
            r_1 = -(r.clone());
            metamodelica::Ref::new(Values::Value::REAL { real: r_1 })
        }
        Values::Value::INTEGER { integer: i } => {
            let mut i_1: i32;
            i_1 = -(i.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: i_1 })
        }
        Values::Value::ARRAY {
            valueLst: vlst,
            dimLst: dims,
        } => {
            let mut vlst_1: metamodelica::List<metamodelica::Ref<Values::Value>>;
            vlst_1 = List::map(vlst.clone(), &move |__a0: metamodelica::Ref<Values::Value>| {
                valueNeg(&__a0)
            })?;
            metamodelica::Ref::new(Values::Value::ARRAY {
                valueLst: vlst_1,
                dimLst: dims.clone(),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outValue)
}

pub(crate) fn valueSum(
    mut value1: &metamodelica::Ref<Values::Value>,
    mut value2: &metamodelica::Ref<Values::Value>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut result: metamodelica::Ref<Values::Value>;
    result = (::match_deref::match_deref! { match (value1, value2) {
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::INTEGER { .. }) => metamodelica::Ref::new(Values::Value::INTEGER { integer: var_field!((**value1).integer, Values::Value::INTEGER).clone() + var_field!((**value2).integer, Values::Value::INTEGER).clone() }),
        (Deref @ Values::Value::STRING { .. }, Deref @ Values::Value::STRING { .. }) => metamodelica::Ref::new(Values::Value::STRING { string: { let mut __mm_s = String::new(); __mm_s.push_str(&*var_field!((**value1).string, Values::Value::STRING)); __mm_s.push_str(&*var_field!((**value2).string, Values::Value::STRING)); ArcStr::from(__mm_s) } }),
        _ => metamodelica::Ref::new(Values::Value::REAL { real: valueReal(value1)? + valueReal(value2)? }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn valueSubtract(
    mut value1: &metamodelica::Ref<Values::Value>,
    mut value2: &metamodelica::Ref<Values::Value>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut result: metamodelica::Ref<Values::Value>;
    result = (::match_deref::match_deref! { match (value1, value2) {
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::INTEGER { .. }) => metamodelica::Ref::new(Values::Value::INTEGER { integer: var_field!((**value1).integer, Values::Value::INTEGER).clone() - var_field!((**value2).integer, Values::Value::INTEGER).clone() }),
        _ => metamodelica::Ref::new(Values::Value::REAL { real: valueReal(value1)? - valueReal(value2)? }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn valueMultiply(
    mut value1: &metamodelica::Ref<Values::Value>,
    mut value2: &metamodelica::Ref<Values::Value>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut result: metamodelica::Ref<Values::Value>;
    result = (::match_deref::match_deref! { match (value1, value2) {
        (Deref @ Values::Value::INTEGER { .. }, Deref @ Values::Value::INTEGER { .. }) => metamodelica::Ref::new(Values::Value::INTEGER { integer: var_field!((**value1).integer, Values::Value::INTEGER).clone() * var_field!((**value2).integer, Values::Value::INTEGER).clone() }),
        _ => metamodelica::Ref::new(Values::Value::REAL { real: valueReal(value1)? * valueReal(value2)? }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn valueDivide(
    mut value1: &metamodelica::Ref<Values::Value>,
    mut value2: &metamodelica::Ref<Values::Value>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut result: metamodelica::Ref<Values::Value>;
    result = (match &**value2 {
        Values::Value::INTEGER { integer: 0 } => {
            Error::addMessage(
                Error::DIVISION_BY_ZERO.clone(),
                list![
                    literal!("0"),
                    intString(var_field!((**value2).integer, Values::Value::INTEGER).clone())
                ],
            )?;
            return Err("fail");
        }
        Values::Value::REAL { real: __rlit_0 } if __rlit_0.eq(&metamodelica::OrderedFloat((0.0) as f64)) => {
            Error::addMessage(
                Error::DIVISION_BY_ZERO.clone(),
                list![
                    literal!("0"),
                    realString(var_field!((**value2).real, Values::Value::REAL).clone())
                ],
            )?;
            return Err("fail");
        }
        _ => metamodelica::Ref::new(Values::Value::REAL {
            real: metamodelica::real_div_checked(valueReal(value1)?, valueReal(value2)?)?,
        }),
    });
    Ok(result)
}

pub(crate) fn valuePow(
    mut value1: &metamodelica::Ref<Values::Value>,
    mut value2: &metamodelica::Ref<Values::Value>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut result: metamodelica::Ref<Values::Value>;
    result = metamodelica::Ref::new(Values::Value::REAL {
        real: (valueReal(value1)?).powf(valueReal(value2)?),
    });
    Ok(result)
}

pub(crate) fn sumArray(mut value: metamodelica::Ref<Values::Value>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut result: metamodelica::Ref<Values::Value>;
    result = (match &*value {
        Values::Value::ARRAY {
            valueLst: __value_valueLst,
            ..
        } => sumArrayelt(metamodelica::AsArg::as_arg(&__value_valueLst))?,
        _ => value,
    });
    Ok(result)
}

pub fn sumArrayelt(
    mut values: &metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut result: metamodelica::Ref<Values::Value>;
    result = sumArray((values).head().cloned()?)?;
    for mut v in &*(values).rest()? {
        result = valueSum(&(sumArray(v.clone())?), &result)?;
    }
    Ok(result)
}

pub fn multScalarArrayelt(
    mut scalarValue: &metamodelica::Ref<Values::Value>,
    mut arrayValues: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut result: metamodelica::List<metamodelica::Ref<Values::Value>>;
    result = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut v in (arrayValues).into_iter().cloned() {
            let __x = (match &*v.clone() {
                Values::Value::ARRAY {
                    dimLst: __v_dimLst,
                    valueLst: __v_valueLst,
                } => metamodelica::Ref::new(Values::Value::ARRAY {
                    valueLst: multScalarArrayelt(scalarValue, __v_valueLst.clone())?,
                    dimLst: __v_dimLst.clone(),
                }),
                _ => valueMultiply(scalarValue, &(v.clone()))?,
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(result)
}

pub fn addScalarArrayelt(
    mut scalarValue: &metamodelica::Ref<Values::Value>,
    mut arrayValues: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut result: metamodelica::List<metamodelica::Ref<Values::Value>>;
    result = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut v in (arrayValues).into_iter().cloned() {
            let __x = (match &*v.clone() {
                Values::Value::ARRAY {
                    dimLst: __v_dimLst,
                    valueLst: __v_valueLst,
                } => metamodelica::Ref::new(Values::Value::ARRAY {
                    valueLst: addScalarArrayelt(scalarValue, __v_valueLst.clone())?,
                    dimLst: __v_dimLst.clone(),
                }),
                _ => valueSum(scalarValue, &(v.clone()))?,
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(result)
}

pub fn subScalarArrayelt(
    mut scalarValue: &metamodelica::Ref<Values::Value>,
    mut arrayValues: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut result: metamodelica::List<metamodelica::Ref<Values::Value>>;
    result = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut v in (arrayValues).into_iter().cloned() {
            let __x = (match &*v.clone() {
                Values::Value::ARRAY {
                    dimLst: __v_dimLst,
                    valueLst: __v_valueLst,
                } => metamodelica::Ref::new(Values::Value::ARRAY {
                    valueLst: subScalarArrayelt(scalarValue, __v_valueLst.clone())?,
                    dimLst: __v_dimLst.clone(),
                }),
                _ => valueSubtract(scalarValue, &(v.clone()))?,
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(result)
}

pub(crate) fn subArrayeltScalar(
    mut scalarValue: &metamodelica::Ref<Values::Value>,
    mut arrayValues: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut result: metamodelica::List<metamodelica::Ref<Values::Value>>;
    result = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut v in (arrayValues).into_iter().cloned() {
            let __x = (match &*v.clone() {
                Values::Value::ARRAY {
                    dimLst: __v_dimLst,
                    valueLst: __v_valueLst,
                } => metamodelica::Ref::new(Values::Value::ARRAY {
                    valueLst: subArrayeltScalar(scalarValue, __v_valueLst.clone())?,
                    dimLst: __v_dimLst.clone(),
                }),
                _ => valueSubtract(&(v.clone()), scalarValue)?,
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(result)
}

pub fn divScalarArrayelt(
    mut scalarValue: &metamodelica::Ref<Values::Value>,
    mut arrayValues: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut result: metamodelica::List<metamodelica::Ref<Values::Value>>;
    result = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut v in (arrayValues).into_iter().cloned() {
            let __x = (match &*v.clone() {
                Values::Value::ARRAY {
                    dimLst: __v_dimLst,
                    valueLst: __v_valueLst,
                } => metamodelica::Ref::new(Values::Value::ARRAY {
                    valueLst: divScalarArrayelt(scalarValue, __v_valueLst.clone())?,
                    dimLst: __v_dimLst.clone(),
                }),
                _ => valueDivide(scalarValue, &(v.clone()))?,
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(result)
}

pub fn divArrayeltScalar(
    mut scalarValue: &metamodelica::Ref<Values::Value>,
    mut arrayValues: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut result: metamodelica::List<metamodelica::Ref<Values::Value>>;
    result = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut v in (arrayValues).into_iter().cloned() {
            let __x = (match &*v.clone() {
                Values::Value::ARRAY {
                    dimLst: __v_dimLst,
                    valueLst: __v_valueLst,
                } => metamodelica::Ref::new(Values::Value::ARRAY {
                    valueLst: divArrayeltScalar(scalarValue, __v_valueLst.clone())?,
                    dimLst: __v_dimLst.clone(),
                }),
                _ => valueDivide(&(v.clone()), scalarValue)?,
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(result)
}

pub fn powScalarArrayelt(
    mut scalarValue: &metamodelica::Ref<Values::Value>,
    mut arrayValues: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut result: metamodelica::List<metamodelica::Ref<Values::Value>>;
    result = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut v in (arrayValues).into_iter().cloned() {
            let __x = (match &*v.clone() {
                Values::Value::ARRAY {
                    dimLst: __v_dimLst,
                    valueLst: __v_valueLst,
                } => metamodelica::Ref::new(Values::Value::ARRAY {
                    valueLst: powScalarArrayelt(scalarValue, __v_valueLst.clone())?,
                    dimLst: __v_dimLst.clone(),
                }),
                _ => valuePow(scalarValue, &(v.clone()))?,
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(result)
}

pub fn powArrayeltScalar(
    mut scalarValue: &metamodelica::Ref<Values::Value>,
    mut arrayValues: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut result: metamodelica::List<metamodelica::Ref<Values::Value>>;
    result = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut v in (arrayValues).into_iter().cloned() {
            let __x = (match &*v.clone() {
                Values::Value::ARRAY {
                    dimLst: __v_dimLst,
                    valueLst: __v_valueLst,
                } => metamodelica::Ref::new(Values::Value::ARRAY {
                    valueLst: powArrayeltScalar(scalarValue, __v_valueLst.clone())?,
                    dimLst: __v_dimLst.clone(),
                }),
                _ => valuePow(scalarValue, &(v.clone()))?,
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(result)
}

pub fn multScalarProduct(
    mut inValueLst1: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inValueLst2: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = 'mc: {
        let __mc_input = (inValueLst1, inValueLst2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i1 }, tail: v1lst @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: i2 }, tail: v2lst @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }) => {
                    let mut res: i32;
                    let mut i1 = (*i1).clone();
                    let mut i2 = (*i2).clone();
                    i1 = i1.clone() * i2.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(multScalarProduct(v1lst.clone(), v2lst.clone())?) {
                        Deref @ Values::Value::INTEGER { integer: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    i2 = metamodelica::Own::own(__pa0);
                    res = i1.clone() + i2.clone();
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: res }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: v1 }, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: v2 }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut res: i32;
                    res = v1.clone() * v2.clone();
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: res }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: r1 }, tail: v1lst @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: r2 }, tail: v2lst @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }) => {
                    let mut rres: metamodelica::Real;
                    let mut r1 = (*r1).clone();
                    let mut r2 = (*r2).clone();
                    r1 = r1.clone() * r2.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(multScalarProduct(v1lst.clone(), v2lst.clone())?) {
                        Deref @ Values::Value::REAL { real: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    r2 = metamodelica::Own::own(__pa0);
                    rres = r1.clone() + r2.clone();
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: rres }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: r1 }, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: r2 }, tail: Deref @ metamodelica::ListNode::Nil }) => {
                    let mut rres: metamodelica::Real;
                    rres = r1.clone() * r2.clone();
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: rres }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: v2lst, .. }, tail: rest }, vlst @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { .. }, tail: _ }) => {
                    let mut dim: i32;
                    let mut vres: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut sres: metamodelica::Ref<Values::Value>;
                    let mut dims: metamodelica::List<i32>;
                    sres = multScalarProduct(v2lst.clone(), vlst.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(multScalarProduct(rest.clone(), vlst.clone())?) {
                        Deref @ Values::Value::ARRAY { valueLst: __pa0, dimLst: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    vres = metamodelica::Own::own(__pa0);
                    dim = metamodelica::Own::own(__pa1);
                    dims = metamodelica::Own::own(__pa2);
                    dim = dim + 1;
                    Ok(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: metamodelica::cons(sres.clone(), vres.clone()), dimLst: metamodelica::cons(dim, dims.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { .. }, tail: _ }) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: v2lst, .. }, tail: rest }, vlst @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { .. }, tail: _ }) => {
                    let mut dim: i32;
                    let mut vres: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut sres: metamodelica::Ref<Values::Value>;
                    let mut dims: metamodelica::List<i32>;
                    sres = multScalarProduct(v2lst.clone(), vlst.clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(multScalarProduct(rest.clone(), vlst.clone())?) {
                        Deref @ Values::Value::ARRAY { valueLst: __pa0, dimLst: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    vres = metamodelica::Own::own(__pa0);
                    dim = metamodelica::Own::own(__pa1);
                    dims = metamodelica::Own::own(__pa2);
                    dim = dim + 1;
                    Ok(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: metamodelica::cons(sres.clone(), vres.clone()), dimLst: metamodelica::cons(dim, dims.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { .. }, tail: _ }) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vlst @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { .. }, tail: _ }, mat @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, .. }, tail: _ }) => {
                    let mut dim: i32;
                    let mut col: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut mat_1: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut dims: metamodelica::List<i32>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(matrixStripFirstColumn(metamodelica::AsArg::as_arg(&mat))?) {
                        (Deref @ Values::Value::ARRAY { valueLst: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    col = metamodelica::Own::own(__pa0);
                    mat_1 = metamodelica::Own::own(__pa1);
                    v = multScalarProduct(vlst.clone(), col.clone())?;
                    let (__pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(multScalarProduct(vlst.clone(), mat_1.clone())?) {
                        Deref @ Values::Value::ARRAY { valueLst: __pa3, dimLst: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } } => (__pa3.clone(), __pa4.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    vals = metamodelica::Own::own(__pa3);
                    dim = metamodelica::Own::own(__pa4);
                    dims = metamodelica::Own::own(__pa5);
                    Ok(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: metamodelica::cons(v.clone(), vals.clone()), dimLst: metamodelica::cons(dim, dims.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vlst @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { .. }, tail: _ }, mat @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. }, tail: _ }) => {
                    let mut i1: i32;
                    let mut col: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let __pa0 = ::match_deref::match_deref! { match &(matrixStripFirstColumn(metamodelica::AsArg::as_arg(&mat))?) {
                        (Deref @ Values::Value::ARRAY { valueLst: __pa0, .. }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    col = metamodelica::Own::own(__pa0);
                    let __pa2 = ::match_deref::match_deref! { match &(multScalarProduct(vlst.clone(), col.clone())?) {
                        Deref @ Values::Value::INTEGER { integer: __pa2 } => __pa2.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    i1 = metamodelica::Own::own(__pa2);
                    Ok(ValuesMake::makeArray(list![metamodelica::Ref::new(Values::Value::INTEGER { integer: i1 })]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vlst @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { .. }, tail: _ }, mat @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, .. }, tail: _ }) => {
                    let mut dim: i32;
                    let mut col: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut mat_1: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut dims: metamodelica::List<i32>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(matrixStripFirstColumn(metamodelica::AsArg::as_arg(&mat))?) {
                        (Deref @ Values::Value::ARRAY { valueLst: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    col = metamodelica::Own::own(__pa0);
                    mat_1 = metamodelica::Own::own(__pa1);
                    v = multScalarProduct(vlst.clone(), col.clone())?;
                    let (__pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(multScalarProduct(vlst.clone(), mat_1.clone())?) {
                        Deref @ Values::Value::ARRAY { valueLst: __pa3, dimLst: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } } => (__pa3.clone(), __pa4.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    vals = metamodelica::Own::own(__pa3);
                    dim = metamodelica::Own::own(__pa4);
                    dims = metamodelica::Own::own(__pa5);
                    dim = dim + 1;
                    Ok(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: metamodelica::cons(v.clone(), vals.clone()), dimLst: metamodelica::cons(dim, dims.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vlst @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { .. }, tail: _ }, mat @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. }, tail: _ }) => {
                    let mut col: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut r1: metamodelica::Real;
                    let __pa0 = ::match_deref::match_deref! { match &(matrixStripFirstColumn(metamodelica::AsArg::as_arg(&mat))?) {
                        (Deref @ Values::Value::ARRAY { valueLst: __pa0, .. }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    col = metamodelica::Own::own(__pa0);
                    let __pa2 = ::match_deref::match_deref! { match &(multScalarProduct(vlst.clone(), col.clone())?) {
                        Deref @ Values::Value::REAL { real: __pa2 } => __pa2.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    r1 = metamodelica::Own::own(__pa2);
                    Ok(ValuesMake::makeArray(list![metamodelica::Ref::new(Values::Value::REAL { real: r1 })]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("Values.multScalarProduct failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outValue)
}

pub fn crossProduct(
    mut inValueLst1: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inValueLst2: &metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = (::match_deref::match_deref! { match (inValueLst1, inValueLst2) {
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: x1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: x2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: x3 }, tail: Deref @ metamodelica::ListNode::Nil } } }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: y1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: y2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: y3 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
            let mut z1: metamodelica::Real;
            let mut z2: metamodelica::Real;
            let mut z3: metamodelica::Real;
            z1 = ((x2.clone()) * (y3.clone())) - ((x3.clone()) * (y2.clone()));
            z2 = ((x3.clone()) * (y1.clone())) - ((x1.clone()) * (y3.clone()));
            z3 = ((x1.clone()) * (y2.clone())) - ((x2.clone()) * (y1.clone()));
            ValuesMake::makeArray(list![metamodelica::Ref::new(Values::Value::REAL { real: z1 }), metamodelica::Ref::new(Values::Value::REAL { real: z2 }), metamodelica::Ref::new(Values::Value::REAL { real: z3 })])
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: ix1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: ix2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: ix3 }, tail: Deref @ metamodelica::ListNode::Nil } } }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: iy1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: iy2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: iy3 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
            let mut iz1: i32;
            let mut iz2: i32;
            let mut iz3: i32;
            iz1 = intSub(intMul(ix2.clone(), iy3.clone()), intMul(ix3.clone(), iy2.clone()));
            iz2 = intSub(intMul(ix3.clone(), iy1.clone()), intMul(ix1.clone(), iy3.clone()));
            iz3 = intSub(intMul(ix1.clone(), iy2.clone()), intMul(ix2.clone(), iy1.clone()));
            ValuesMake::makeArray(list![metamodelica::Ref::new(Values::Value::INTEGER { integer: iz1 }), metamodelica::Ref::new(Values::Value::INTEGER { integer: iz2 }), metamodelica::Ref::new(Values::Value::INTEGER { integer: iz3 })])
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("ValuesUtil.crossProduct failed")])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outValue)
}

pub fn multMatrix(
    mut inValueLst1: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inValueLst2: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut outValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>;
    outValueLst = (::match_deref::match_deref! { match &((&**inValueLst1, inValueLst2)) {
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: v1lst, .. }, tail: rest1 }, m2 @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { .. }, tail: _ }) => {
            let mut res1: metamodelica::Ref<Values::Value>;
            let mut res2: metamodelica::List<metamodelica::Ref<Values::Value>>;
            res1 = multScalarProduct(v1lst.clone(), m2.clone())?;
            res2 = multMatrix(metamodelica::AsArg::as_arg(&rest1), m2.clone())?;
            metamodelica::cons(res1, res2)
        },
        (Deref @ metamodelica::ListNode::Nil, _) => {
            metamodelica::nil()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outValueLst)
}

fn matrixStripFirstColumn(
    mut inValueLst: &metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<(
    metamodelica::Ref<Values::Value>,
    metamodelica::List<metamodelica::Ref<Values::Value>>,
)> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    let mut outValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>;
    (outValue, outValueLst) = (::match_deref::match_deref! { match inValueLst {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: v1, tail: vrest }, dimLst: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil } }, tail: rest } => {
            let mut resl: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut resl2: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut i: i32;
            let mut dim = (*dim).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(matrixStripFirstColumn(rest)?) {
                (Deref @ Values::Value::ARRAY { valueLst: __pa0, dimLst: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            resl = metamodelica::Own::own(__pa0);
            i = metamodelica::Own::own(__pa1);
            resl2 = metamodelica::Own::own(__pa2);
            i = i + 1;
            dim = dim.clone() - 1;
            (metamodelica::Ref::new(Values::Value::ARRAY { valueLst: metamodelica::cons(v1.clone(), resl), dimLst: list![i] }), metamodelica::cons(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: vrest.clone(), dimLst: list![dim.clone()] }), resl2))
        },
        Deref @ metamodelica::ListNode::Nil => {
            (metamodelica::Ref::new(Values::Value::ARRAY { valueLst: metamodelica::nil(), dimLst: list![0] }), metamodelica::nil())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outValue, outValueLst))
}

pub fn intlistToValue(mut inIntegerLst: &metamodelica::List<i32>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = (::match_deref::match_deref! { match inIntegerLst {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::Ref::new(Values::Value::ARRAY { valueLst: metamodelica::nil(), dimLst: list![0] })
        },
        Deref @ metamodelica::ListNode::Cons { head: i, tail: lst } => {
            let mut res: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut len: i32;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(intlistToValue(lst)?) {
                Deref @ Values::Value::ARRAY { valueLst: __pa0, dimLst: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            res = metamodelica::Own::own(__pa0);
            len = metamodelica::Own::own(__pa1);
            len = len + 1;
            metamodelica::Ref::new(Values::Value::ARRAY { valueLst: metamodelica::cons(metamodelica::Ref::new(Values::Value::INTEGER { integer: i.clone() }), res), dimLst: list![len] })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outValue)
}

pub fn arrayValues(
    mut inValue: &metamodelica::Ref<Values::Value>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut outValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>;
    outValueLst = (match &**inValue {
        Values::Value::ARRAY { valueLst: v_lst, .. } => v_lst.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(outValueLst)
}

pub fn arrayScalar(mut inValue: &metamodelica::Ref<Values::Value>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    let __pa0 = ::match_deref::match_deref! { match &((*inValue)) {
        Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outValue = metamodelica::Own::own(__pa0);
    Ok(outValue)
}

pub(crate) fn writePtolemyplotDataset(
    mut inString1: ArcStr,
    mut inValue2: &metamodelica::Ref<Values::Value>,
    mut inStringLst3: &metamodelica::List<ArcStr>,
    mut inString4: ArcStr,
) -> Result<i32> {
    let mut outInteger: i32;
    outInteger = (::match_deref::match_deref! { match (inValue2, inStringLst3) {
        (Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: t, tail: rest }, .. }, Deref @ metamodelica::ListNode::Cons { head: _, tail: varnames }) => {
            let mut filename = inString1;
            let mut message = inString4;
            let mut r#str: ArcStr;
            let mut handle: i32;
            handle = Print::saveAndClearBuf()?;
            Print::printBuf(literal!("#Ptolemy Plot generated by OpenModelica\nTitleText: "))?;
            Print::printBuf(message)?;
            Print::printBuf(literal!("\n"))?;
            unparsePtolemyValues(t.clone(), metamodelica::AsArg::as_arg(&rest), varnames)?;
            r#str = Print::getString()?;
            Print::restoreBuf(handle)?;
            System::writeFile(filename, r#str)?;
            0
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outInteger)
}

fn unparsePtolemyValues(
    mut inValue: metamodelica::Ref<Values::Value>,
    mut inValueLst: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inStringLst: &metamodelica::List<ArcStr>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match (inValueLst, inStringLst) {
        (Deref @ metamodelica::ListNode::Nil, _) => {
            ()
        },
        (Deref @ metamodelica::ListNode::Cons { head: s1, tail: xs }, Deref @ metamodelica::ListNode::Cons { head: v1, tail: vs }) => {
            let mut t = inValue;
            unparsePtolemySet(t.clone(), metamodelica::AsArg::as_arg(&s1), v1.clone())?;
            unparsePtolemyValues(t, xs, vs)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn unparsePtolemySet(
    mut v1: metamodelica::Ref<Values::Value>,
    mut v2: &metamodelica::Ref<Values::Value>,
    mut varname: ArcStr,
) -> Result<()> {
    Print::printBuf(stringAppendList(list![literal!("DataSet: "), varname, literal!("\n")]))?;
    unparsePtolemySet2(v1, v2)?;
    Ok(())
}

fn unparsePtolemySet2(
    mut inValue1: metamodelica::Ref<Values::Value>,
    mut inValue2: &metamodelica::Ref<Values::Value>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inValue1, &**inValue2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Nil, .. }, Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Nil, .. }) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: v1, tail: v1s }, .. }, Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: v2, tail: v2s }, .. }) => {
                    ValuesDump::valString2(metamodelica::AsArg::as_arg(&v1))?;
                    Print::printBuf(literal!(","))?;
                    ValuesDump::valString2(metamodelica::AsArg::as_arg(&v2))?;
                    Print::printBuf(literal!("\n"))?;
                    unparsePtolemySet2(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: v1s.clone(), dimLst: metamodelica::nil() }), &(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: v2s.clone(), dimLst: metamodelica::nil() })))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v1, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- ValuesUtil.unparsePtolemySet2 failed on v1: ")); __mm_s.push_str(&*ValuesDump::printValStr(metamodelica::AsArg::as_arg(&v1))?); __mm_s.push_str(&*literal!(" and v2: ")); __mm_s.push_str(&*ValuesDump::printValStr(metamodelica::AsArg::as_arg(&v1))?); ArcStr::from(__mm_s) })?;
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

pub(crate) fn reverseMatrix(mut inValue: metamodelica::Ref<Values::Value>) -> metamodelica::Ref<Values::Value> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = 'mc: {
        let __mc_input = inValue;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ARRAY { valueLst: lst, dimLst: dims } => {
                    let mut lst_1: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut lst_2: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    lst_1 = List::map(lst.clone(), &fnptr!(reverseMatrix, metamodelica::Ref<Values::Value>))?;
                    lst_2 = lst_1.clone().reverse();
                    Ok(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: lst_2.clone(), dimLst: dims.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                value => {
                    Ok(value.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outValue
}

pub fn nthnthArrayelt(
    mut inLst: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inValue: metamodelica::Ref<Values::Value>,
    mut lastValue: metamodelica::Ref<Values::Value>,
) -> Result<metamodelica::Ref<Values::Value>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inLst, inValue, lastValue)) {
            (Deref @ metamodelica::ListNode::Nil, _, preRes) => {
                return Ok(preRes.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: vlst2 }, Deref @ Values::Value::ARRAY { valueLst: vlst, .. }, _) => {
                let mut res: metamodelica::Ref<Values::Value>;
                res = (vlst).get(n.clone())?;
                { (inLst, inValue, lastValue) = (vlst2.clone(), res.clone(), res); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ENUM_LITERAL { index: n, .. }, tail: vlst2 }, Deref @ Values::Value::ARRAY { valueLst: vlst, .. }, _) => {
                let mut res: metamodelica::Ref<Values::Value>;
                res = (vlst).get(n.clone())?;
                { (inLst, inValue, lastValue) = (vlst2.clone(), res.clone(), res); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: vlst2 }, Deref @ Values::Value::ARRAY { valueLst: vlst, .. }, _) => {
                let mut res: metamodelica::Ref<Values::Value>;
                res = (vlst).get(if (b.clone()) {2} else {1})?;
                { (inLst, inValue, lastValue) = (vlst2.clone(), res.clone(), res); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn valueInteger(mut inValue: &metamodelica::Ref<Values::Value>) -> Result<i32> {
    let mut outInteger: i32;
    outInteger = (match &**inValue {
        Values::Value::INTEGER { integer: i } => i.clone(),
        Values::Value::ENUM_LITERAL { index: i, .. } => i.clone(),
        Values::Value::BOOL { boolean: true } => 1,
        Values::Value::BOOL { boolean: false } => 0,
        _ => return Err("match: no arm matched"),
    });
    Ok(outInteger)
}

pub fn valueDimensions(mut inValue: &metamodelica::Ref<Values::Value>) -> metamodelica::List<i32> {
    let mut outDimensions: metamodelica::List<i32>;
    outDimensions = (match &**inValue {
        Values::Value::ARRAY { dimLst: dims, .. } => dims.clone(),
        _ => metamodelica::nil(),
    });
    outDimensions
}

pub fn extractValueString(mut val: &metamodelica::Ref<Values::Value>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &((*val)) {
        Deref @ Values::Value::STRING { string: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    r#str = metamodelica::Own::own(__pa0);
    Ok(r#str)
}

pub(crate) fn getCode(mut val: &metamodelica::Ref<Values::Value>) -> Result<metamodelica::Ref<Absyn::CodeNode>> {
    let mut code: metamodelica::Ref<Absyn::CodeNode>;
    let __pa0 = ::match_deref::match_deref! { match &((*val)) {
        Deref @ Values::Value::CODE { A: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    code = metamodelica::Own::own(__pa0);
    Ok(code)
}

pub fn getPath(mut val: &metamodelica::Ref<Values::Value>) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut code: metamodelica::Ref<Absyn::CodeNode>;
    let __pa0 = ::match_deref::match_deref! { match &((*val)) {
        Deref @ Values::Value::CODE { A: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    code = metamodelica::Own::own(__pa0);
    let __pa1 = ::match_deref::match_deref! { match &(code) {
        Deref @ Absyn::CodeNode::C_TYPENAME { path: __pa1 } => __pa1.clone(),
        _ => return Err("pattern mismatch"),
    } };
    path = metamodelica::Own::own(__pa1);
    Ok(path)
}

pub fn printCodeVariableName(mut val: &metamodelica::Ref<Values::Value>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (::match_deref::match_deref! { match val {
        Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp } } => {
            Dump::printExpStr(exp.clone())?
        },
        Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr } } => {
            Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(r#str)
}

pub fn boxIfUnboxedVal(mut v: metamodelica::Ref<Values::Value>) -> metamodelica::Ref<Values::Value> {
    let mut ov: metamodelica::Ref<Values::Value>;
    ov = (match &*v {
        Values::Value::INTEGER { integer: _ } => metamodelica::Ref::new(Values::Value::META_BOX { value: v }),
        Values::Value::REAL { real: _ } => metamodelica::Ref::new(Values::Value::META_BOX { value: v }),
        Values::Value::BOOL { boolean: _ } => metamodelica::Ref::new(Values::Value::META_BOX { value: v }),
        _ => v,
    });
    ov
}

pub fn unboxIfBoxedVal(mut iv: metamodelica::Ref<Values::Value>) -> metamodelica::Ref<Values::Value> {
    let mut ov: metamodelica::Ref<Values::Value>;
    ov = (match &*iv {
        Values::Value::META_BOX { value: v } => v.clone(),
        _ => iv,
    });
    ov
}

pub fn arrayOrListVals(
    mut v: &metamodelica::Ref<Values::Value>,
    mut boxIfUnboxed: bool,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
    vals = (::match_deref::match_deref! { match &((&**v, boxIfUnboxed)) {
        (Deref @ Values::Value::ARRAY { valueLst: __esc_vals, .. }, _) => {
            vals = (*__esc_vals).clone();
            vals.clone()
        },
        (Deref @ Values::Value::LIST { valueLst: __esc_vals }, true) => {
            vals = (*__esc_vals).clone();
            List::map(vals.clone(), &fnptr!(boxIfUnboxedVal, metamodelica::Ref<Values::Value>))?
        },
        (Deref @ Values::Value::LIST { valueLst: __esc_vals }, _) => {
            vals = (*__esc_vals).clone();
            vals.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(vals)
}

pub fn containsEmpty(mut inValue: metamodelica::Ref<Values::Value>) -> Option<metamodelica::Ref<Values::Value>> {
    let mut outEmptyVal: Option<metamodelica::Ref<Values::Value>>;
    outEmptyVal = (match &*inValue {
        Values::Value::EMPTY { .. } => Some(inValue),
        Values::Value::ARRAY {
            valueLst: __inValue_valueLst,
            ..
        } => arrayContainsEmpty(metamodelica::AsArg::as_arg(&__inValue_valueLst)),
        Values::Value::RECORD {
            orderd: __inValue_orderd,
            ..
        } => arrayContainsEmpty(metamodelica::AsArg::as_arg(&__inValue_orderd)),
        Values::Value::TUPLE {
            valueLst: __inValue_valueLst,
        } => arrayContainsEmpty(metamodelica::AsArg::as_arg(&__inValue_valueLst)),
        _ => None,
    });
    outEmptyVal
}

pub(crate) fn arrayContainsEmpty(
    mut inValues: &metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Option<metamodelica::Ref<Values::Value>> {
    let mut outOptValue: Option<metamodelica::Ref<Values::Value>> = None;
    for mut val in &**inValues {
        outOptValue = containsEmpty(val.clone());
        if (outOptValue).is_some() {
            break;
        }
    }
    outOptValue
}

pub fn liftValueList(
    mut inValue: metamodelica::Ref<Values::Value>,
    mut inDimensions: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value> = inValue;
    for mut dim in &*inDimensions.reverse() {
        outValue = ValuesMake::makeArray(List::fill(
            outValue,
            Expression::dimensionSize(metamodelica::AsArg::as_arg(&dim))?,
        ));
    }
    Ok(outValue)
}

pub fn isEmpty(mut inValue: &metamodelica::Ref<Values::Value>) -> bool {
    let mut outIsEmpty: bool;
    outIsEmpty = (match &**inValue {
        Values::Value::EMPTY { .. } => true,
        _ => false,
    });
    outIsEmpty
}

pub fn typeConvertRecord(
    mut inValue: metamodelica::Ref<Values::Value>,
    mut inType: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value> = inValue;
    outValue = (::match_deref::match_deref! { match &((outValue.clone(), inType.clone())) {
        (Deref @ Values::Value::RECORD { .. }, Deref @ DAE::Type::T_COMPLEX { .. }) => {
            assign_variant_field!(outValue => Values::Value::RECORD; orderd = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        let __thr_src0 = var_field!((*outValue).orderd, Values::Value::RECORD).clone();
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = var_field!((**inType).varLst, DAE::Type::T_COMPLEX).clone();
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(val), Some(var)) => {
                    let __x = typeConvertRecord(val.clone(), &(Types::getVarType(&(var.clone()))?))?;
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    }));
            outValue
        },
        (Deref @ Values::Value::INTEGER { .. }, Deref @ DAE::Type::T_REAL { .. }) => {
            metamodelica::Ref::new(Values::Value::REAL { real: intReal(var_field!((*outValue).integer, Values::Value::INTEGER).clone()) })
        },
        (Deref @ Values::Value::ARRAY { .. }, Deref @ DAE::Type::T_ARRAY { .. }) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            ty = Expression::unliftArray(inType)?;
            assign_variant_field!(outValue => Values::Value::ARRAY; valueLst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut v in (var_field!((*outValue).valueLst, Values::Value::ARRAY).clone()).into_iter().cloned() {
            let __x = typeConvertRecord(v.clone(), &ty)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            outValue
        },
        _ => {
            outValue
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outValue)
}

pub fn fixZeroSizeArray(
    mut e: metamodelica::Ref<DAE::Exp>,
    mut ty: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut e: metamodelica::Ref<DAE::Exp> = e;
    e = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_UNKNOWN { .. }, .. }, scalar: false, array: Deref @ metamodelica::ListNode::Nil } => metamodelica::Ref::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: !(Types::isArray(&(Types::unliftArray(&ty)?))), array: metamodelica::nil() }),
        _ => e,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(e)
}

pub fn arraySize(mut value: &metamodelica::Ref<Values::Value>) -> Result<i32> {
    let mut size: i32;
    size = (match &**value {
        Values::Value::ARRAY {
            dimLst: __value_dimLst, ..
        } => (__value_dimLst).head().cloned()?,
        Values::Value::META_ARRAY {
            valueLst: __value_valueLst,
        } => ((__value_valueLst).len() as i32),
        _ => 0,
    });
    Ok(size)
}
