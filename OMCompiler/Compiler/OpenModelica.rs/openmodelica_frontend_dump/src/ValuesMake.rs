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

use openmodelica_ast::Absyn;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_util_datatypes_basic::List;

pub fn makeZero(mut ty: &metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut zero: metamodelica::Ref<Values::Value>;
    zero = (match &**ty {
        DAE::Type::T_REAL { .. } => metamodelica::Ref::new(Values::Value::REAL {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
        DAE::Type::T_INTEGER { .. } => metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 }),
        _ => return Err("match: no arm matched"),
    });
    Ok(zero)
}

pub fn makeBoolean(mut b: bool) -> metamodelica::Ref<Values::Value> {
    let mut v: metamodelica::Ref<Values::Value>;
    v = metamodelica::Ref::new(Values::Value::BOOL { boolean: b });
    v
}

pub fn makeReal(mut r: metamodelica::Real) -> metamodelica::Ref<Values::Value> {
    let mut v: metamodelica::Ref<Values::Value>;
    v = metamodelica::Ref::new(Values::Value::REAL { real: r });
    v
}

pub fn makeInteger(mut i: i32) -> metamodelica::Ref<Values::Value> {
    let mut v: metamodelica::Ref<Values::Value>;
    v = metamodelica::Ref::new(Values::Value::INTEGER { integer: i });
    v
}

pub fn makeString(mut s: ArcStr) -> metamodelica::Ref<Values::Value> {
    let mut v: metamodelica::Ref<Values::Value>;
    v = metamodelica::Ref::new(Values::Value::STRING { string: s });
    v
}

pub fn makeTuple(
    mut inValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> metamodelica::Ref<Values::Value> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = metamodelica::Ref::new(Values::Value::TUPLE { valueLst: inValueLst });
    outValue
}

pub fn makeList(
    mut inValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> metamodelica::Ref<Values::Value> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = metamodelica::Ref::new(Values::Value::LIST { valueLst: inValueLst });
    outValue
}

pub fn makeArray(
    mut inValueLst: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> metamodelica::Ref<Values::Value> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = (::match_deref::match_deref! { match &(inValueLst) {
        vlst @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { dimLst: il, .. }, tail: _ } => {
            let mut i1: i32;
            i1 = ((vlst).len() as i32);
            metamodelica::Ref::new(Values::Value::ARRAY { valueLst: vlst.clone(), dimLst: metamodelica::cons(i1, il.clone()) })
        },
        vlst => {
            let mut i1: i32;
            i1 = ((vlst).len() as i32);
            metamodelica::Ref::new(Values::Value::ARRAY { valueLst: vlst.clone(), dimLst: list![i1] })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outValue
}

pub fn makeEmptyArray() -> metamodelica::Ref<Values::Value> {
    let mut outValue: metamodelica::Ref<Values::Value> = metamodelica::Ref::new(Values::Value::ARRAY {
        valueLst: metamodelica::nil(),
        dimLst: list![0],
    });
    outValue
}

pub fn makeStringArray(mut inReals: metamodelica::List<ArcStr>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outArray: metamodelica::Ref<Values::Value>;
    outArray = makeArray(List::map(inReals, &fnptr!(makeString, ArcStr))?);
    Ok(outArray)
}

pub fn makeIntArray(mut inInts: metamodelica::List<i32>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outArray: metamodelica::Ref<Values::Value>;
    outArray = makeArray(List::map(inInts, &fnptr!(makeInteger, i32))?);
    Ok(outArray)
}

pub fn makeRealArray(mut inReals: metamodelica::List<metamodelica::Real>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outArray: metamodelica::Ref<Values::Value>;
    outArray = makeArray(List::map(inReals, &fnptr!(makeReal, metamodelica::Real))?);
    Ok(outArray)
}

pub fn makeRealMatrix(
    mut inReals: metamodelica::List<metamodelica::List<metamodelica::Real>>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outArray: metamodelica::Ref<Values::Value>;
    outArray = makeArray(List::map(inReals, &makeRealArray)?);
    Ok(outArray)
}

pub fn makeCodeTypeName(mut path: metamodelica::Ref<Absyn::Path>) -> metamodelica::Ref<Values::Value> {
    let mut val: metamodelica::Ref<Values::Value>;
    val = metamodelica::Ref::new(Values::Value::CODE {
        A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: path }),
    });
    val
}

pub fn makeCodeTypeNameStr(mut r#str: ArcStr) -> metamodelica::Ref<Values::Value> {
    let mut val: metamodelica::Ref<Values::Value>;
    val = metamodelica::Ref::new(Values::Value::CODE {
        A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
            path: metamodelica::Ref::new(Absyn::Path::IDENT { name: r#str }),
        }),
    });
    val
}

pub fn makeCodeTypeNameArray(
    mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
) -> metamodelica::Ref<Values::Value> {
    let mut val: metamodelica::Ref<Values::Value>;
    val = makeArray(
        ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
            for mut p in (paths).into_iter().cloned() {
                let __x = makeCodeTypeName(p.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    );
    val
}
