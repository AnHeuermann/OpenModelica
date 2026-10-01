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

use crate::FHashTableStringToUnit as HashTableStringToUnit;
use crate::FHashTableUnitToString as HashTableUnitToString;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::Util;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Unit {
    /// based on SI base units
    UNIT {
        /// prefix
        factor: metamodelica::Real,
        /// exponent
        mol: i32,
        /// exponent
        cd: i32,
        /// exponent
        m: i32,
        /// exponent
        s: i32,
        /// exponent
        A: i32,
        /// exponent
        K: i32,
        /// exponent
        g: i32,
    },
    /// unknown unit that belongs to all the variables from varList
    MASTER {
        varList: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    },
    /// unknown SI base unit decomposition
    UNKNOWN { unit: ArcStr },
}
impl metamodelica::gc::MMTrace for Unit {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Unit::UNIT {
                factor,
                mol,
                cd,
                m,
                s,
                A,
                K,
                g,
            } => {
                metamodelica::gc::MMTrace::mm_accept(factor, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(mol, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(cd, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(m, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(s, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(A, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(K, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(g, __mmv)?;
                Ok(())
            }
            Unit::MASTER { varList } => {
                metamodelica::gc::MMTrace::mm_accept(varList, __mmv)?;
                Ok(())
            }
            Unit::UNKNOWN { unit } => {
                metamodelica::gc::MMTrace::mm_accept(unit, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for Unit {
    fn default() -> Self {
        Self::MASTER {
            varList: Default::default(),
        }
    }
}
pub use self::Unit::{MASTER, UNIT, UNKNOWN};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Token {
    T_NUMBER { number: i32 },
    T_UNIT { unit: ArcStr },
    T_MUL,
    T_DIV,
    T_LPAREN,
    T_RPAREN,
}
impl metamodelica::gc::MMTrace for Token {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Token::T_NUMBER { number } => {
                metamodelica::gc::MMTrace::mm_accept(number, __mmv)?;
                Ok(())
            }
            Token::T_UNIT { unit } => {
                metamodelica::gc::MMTrace::mm_accept(unit, __mmv)?;
                Ok(())
            }
            Token::T_MUL => Ok(()),
            Token::T_DIV => Ok(()),
            Token::T_LPAREN => Ok(()),
            Token::T_RPAREN => Ok(()),
        }
    }
}
pub(crate) use self::Token::{T_DIV, T_LPAREN, T_MUL, T_NUMBER, T_RPAREN, T_UNIT};

thread_local! { static __UPDATECREF_TLS: metamodelica::Ref<DAE::ComponentRef> = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: literal!("jhagemann"), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }); }
pub(crate) fn UPDATECREF() -> metamodelica::Ref<DAE::ComponentRef> {
    __UPDATECREF_TLS.with(|__t| __t.clone())
}

thread_local! { static __LU_COMPLEXUNITS_TLS: metamodelica::List<(ArcStr, Unit)> = list![(literal!("mol"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 1, cd: 0, m: 0, s: 0, A: 0, K: 0, g: 0 }), (literal!("cd"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 1, m: 0, s: 0, A: 0, K: 0, g: 0 }), (literal!("m"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 1, s: 0, A: 0, K: 0, g: 0 }), (literal!("s"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 1, A: 0, K: 0, g: 0 }), (literal!("A"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 0, A: 1, K: 0, g: 0 }), (literal!("K"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 0, A: 0, K: 1, g: 0 }), (literal!("g"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 0, A: 0, K: 0, g: 1 }), (literal!("V"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e3_f64), mol: 0, cd: 0, m: 2, s: -3, A: -1, K: 0, g: 1 }), (literal!("W"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e3_f64), mol: 0, cd: 0, m: 2, s: -3, A: 0, K: 0, g: 1 }), (literal!("Hz"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: -1, A: 0, K: 0, g: 0 }), (literal!("Ohm"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e3_f64), mol: 0, cd: 0, m: 2, s: -3, A: -2, K: 0, g: 1 }), (literal!("F"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e-3_f64), mol: 0, cd: 0, m: -2, s: 4, A: 2, K: 0, g: -1 }), (literal!("H"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e3_f64), mol: 0, cd: 0, m: 2, s: -2, A: -2, K: 0, g: 1 }), (literal!("C"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 1, A: 1, K: 0, g: 0 }), (literal!("T"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e3_f64), mol: 0, cd: 0, m: 0, s: -2, A: -1, K: 0, g: 1 }), (literal!("S"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e-3_f64), mol: 0, cd: 0, m: -2, s: 3, A: 2, K: 0, g: -1 }), (literal!("Wb"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e3_f64), mol: 0, cd: 0, m: 2, s: -2, A: -1, K: 0, g: 1 }), (literal!("N"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e3_f64), mol: 0, cd: 0, m: 1, s: -2, A: 0, K: 0, g: 1 }), (literal!("Pa"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e3_f64), mol: 0, cd: 0, m: -1, s: -2, A: 0, K: 0, g: 1 }), (literal!("J"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e3_f64), mol: 0, cd: 0, m: 2, s: -2, A: 0, K: 0, g: 1 }), (literal!("min"), Unit::UNIT { factor: metamodelica::OrderedFloat(6e1_f64), mol: 0, cd: 0, m: 0, s: 1, A: 0, K: 0, g: 0 }), (literal!("h"), Unit::UNIT { factor: metamodelica::OrderedFloat(3.6e3_f64), mol: 0, cd: 0, m: 0, s: 1, A: 0, K: 0, g: 0 }), (literal!("d"), Unit::UNIT { factor: metamodelica::OrderedFloat(8.64e4_f64), mol: 0, cd: 0, m: 0, s: 1, A: 0, K: 0, g: 0 }), (literal!("l"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e-3_f64), mol: 0, cd: 0, m: 3, s: 0, A: 0, K: 0, g: 0 }), (literal!("kg"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e3_f64), mol: 0, cd: 0, m: 0, s: 0, A: 0, K: 0, g: 1 }), (literal!("kat"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 1, cd: 0, m: 0, s: -1, A: 0, K: 0, g: 0 }), (literal!("1"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 0, A: 0, K: 0, g: 0 }), (literal!("rad"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 0, A: 0, K: 0, g: 0 }), (literal!("degC"), Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 0, A: 0, K: 1, g: 0 }), (literal!("degF"), Unit::UNIT { factor: metamodelica::OrderedFloat(0.55555555555555555555555555555555555555_f64), mol: 0, cd: 0, m: 0, s: 0, A: 0, K: 1, g: 0 })]; }
pub(crate) fn LU_COMPLEXUNITS() -> metamodelica::List<(ArcStr, Unit)> {
    __LU_COMPLEXUNITS_TLS.with(|__t| __t.clone())
}

//°Fahrenheit
//("degF", UNIT(5.0 / 9.0, 0, 0, 0, 0, 0, 1, 0, 459.67)), //°Fahrenheit
//("degC",       UNIT(1e0, 0, 0, 0, 0, 0, 1, 0, 273.15))};//°Celsius
/*                 fac, mol, cd, m, s, A, K, g*/
pub fn getKnownUnits() -> Result<(
    metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
    (i32, i32, metamodelica::Array<Option<(ArcStr, Unit)>>),
    i32,
    (
        Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
        Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
        Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        Arc<dyn ::std::ops::Fn(Unit) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut outKnownUnits: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit)>>),
        i32,
        (
            HashTableStringToUnit::FuncHashKey,
            HashTableStringToUnit::FuncKeyEqual,
            HashTableStringToUnit::FuncKeyStr,
            HashTableStringToUnit::FuncValueStr,
        ),
    );
    outKnownUnits = HashTableStringToUnit::emptyHashTableSized(Util::nextPrime(4 * ((LU_COMPLEXUNITS()).len() as i32)));
    for mut unit in &*LU_COMPLEXUNITS().clone() {
        outKnownUnits = BaseHashTable::add(unit.clone(), outKnownUnits)?;
    }
    Ok(outKnownUnits)
}

pub fn getKnownUnitsInverse() -> Result<(
    metamodelica::Array<metamodelica::List<(Unit, i32)>>,
    (i32, i32, metamodelica::Array<Option<(Unit, ArcStr)>>),
    i32,
    (
        Arc<dyn ::std::ops::Fn(Unit) -> Result<i32> + 'static>,
        Arc<dyn ::std::ops::Fn(Unit, Unit) -> Result<bool> + 'static>,
        Arc<dyn ::std::ops::Fn(Unit) -> Result<ArcStr> + 'static>,
        Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut outKnownUnitsInverse: (
        metamodelica::Array<metamodelica::List<(Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit, ArcStr)>>),
        i32,
        (
            HashTableUnitToString::FuncHashKey,
            HashTableUnitToString::FuncKeyEqual,
            HashTableUnitToString::FuncKeyStr,
            HashTableUnitToString::FuncValueStr,
        ),
    );
    let mut s: ArcStr;
    let mut ut: Unit;
    outKnownUnitsInverse =
        HashTableUnitToString::emptyHashTableSized(Util::nextPrime(4 * ((LU_COMPLEXUNITS()).len() as i32)));
    for mut unit in &*LU_COMPLEXUNITS().clone() {
        (s, ut) = unit.clone();
        if !(BaseHashTable::hasKey(ut.clone(), &outKnownUnitsInverse)?) {
            outKnownUnitsInverse = BaseHashTable::add((ut, s), outKnownUnitsInverse)?;
        }
    }
    Ok(outKnownUnitsInverse)
}

pub(crate) fn isUnit(mut inUnit: &Unit) -> bool {
    let mut b: bool;
    b = (match inUnit.clone() {
        Unit::UNIT { .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn hashUnit(mut inKey: &Unit) -> Result<i32> {
    let mut outHash: i32;
    let mut r#str: ArcStr;
    r#str = unit2string(inKey)?;
    outHash = stringHashDjb2(&r#str);
    Ok(outHash)
}

pub(crate) fn unitEqual(mut inKey: &Unit, mut inKey2: &Unit) -> bool {
    let mut res: bool;
    res = 'mc: {
        let __mc_input = (inKey.clone(), inKey2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (
                Unit::UNIT {
                    factor: mut factor1,
                    mol: mut i1,
                    cd: mut i2,
                    m: mut i3,
                    s: mut i4,
                    A: mut i5,
                    K: mut i6,
                    g: mut i7,
                },
                Unit::UNIT {
                    factor: mut factor2,
                    mol: mut j1,
                    cd: mut j2,
                    m: mut j3,
                    s: mut j4,
                    A: mut j5,
                    K: mut j6,
                    g: mut j7,
                },
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let true = (realEq(factor1.clone(), factor2.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i1.clone(), j1.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i2.clone(), j2.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i3.clone(), j3.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i4.clone(), j4.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i5.clone(), j5.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i6.clone(), j6.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i7.clone(), j7.clone())) else {
                return Err("pattern mismatch");
            };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (
                Unit::UNIT {
                    factor: mut factor1,
                    mol: mut i1,
                    cd: mut i2,
                    m: mut i3,
                    s: mut i4,
                    A: mut i5,
                    K: mut i6,
                    g: mut i7,
                },
                Unit::UNIT {
                    factor: mut factor2,
                    mol: mut j1,
                    cd: mut j2,
                    m: mut j3,
                    s: mut j4,
                    A: mut j5,
                    K: mut j6,
                    g: mut j7,
                },
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut r: metamodelica::Real;
            r = realMax(realAbs(factor1.clone()), realAbs(factor2.clone()));
            let true = (realLe(
                realDiv(realAbs((factor1.clone()) - (factor2.clone())), r),
                metamodelica::OrderedFloat(1e-3_f64),
            )) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i1.clone(), j1.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i2.clone(), j2.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i3.clone(), j3.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i4.clone(), j4.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i5.clone(), j5.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i6.clone(), j6.clone())) else {
                return Err("pattern mismatch");
            };
            let true = (intEq(i7.clone(), j7.clone())) else {
                return Err("pattern mismatch");
            };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (Unit::MASTER { .. }, Unit::MASTER { .. }) = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(true)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (Unit::UNKNOWN { unit: mut s }, Unit::UNKNOWN { unit: mut s2 }) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (stringEqual(&s, &s2)) else {
                return Err("pattern mismatch");
            };
            Ok(true)
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
    res
}

pub(crate) fn unit2string(mut inUnit: &Unit) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inUnit.clone() {
        Unit::UNIT {
            factor: mut factor1,
            mol: mut i1,
            cd: mut i2,
            m: mut i3,
            s: mut i4,
            A: mut i5,
            K: mut i6,
            g: mut i7,
        } => {
            let mut s: ArcStr;
            let mut r#str: ArcStr;
            let mut b: bool;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*realString(factor1.clone()));
                __mm_s.push_str(&*literal!(" * "));
                ArcStr::from(__mm_s)
            };
            b = false;
            s = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("mol^("));
                __mm_s.push_str(&*intString(i1.clone()));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            };
            s = if (intEq(i1.clone(), 0)) { literal!("") } else { s };
            b = b || intNe(i1.clone(), 0);
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*s);
                ArcStr::from(__mm_s)
            };
            s = if (b && intNe(i2.clone(), 0)) {
                literal!(" * ")
            } else {
                literal!("")
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*s);
                ArcStr::from(__mm_s)
            };
            s = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("cd^("));
                __mm_s.push_str(&*intString(i2.clone()));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            };
            s = if (intEq(i2.clone(), 0)) { literal!("") } else { s };
            b = b || intNe(i2.clone(), 0);
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*s);
                ArcStr::from(__mm_s)
            };
            s = if (b && intNe(i3.clone(), 0)) {
                literal!(" * ")
            } else {
                literal!("")
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*s);
                ArcStr::from(__mm_s)
            };
            s = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("m^("));
                __mm_s.push_str(&*intString(i3.clone()));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            };
            s = if (intEq(i3.clone(), 0)) { literal!("") } else { s };
            b = b || intNe(i3.clone(), 0);
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*s);
                ArcStr::from(__mm_s)
            };
            s = if (b && intNe(i4.clone(), 0)) {
                literal!(" * ")
            } else {
                literal!("")
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*s);
                ArcStr::from(__mm_s)
            };
            s = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("s^("));
                __mm_s.push_str(&*intString(i4.clone()));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            };
            s = if (intEq(i4.clone(), 0)) { literal!("") } else { s };
            b = b || intNe(i4.clone(), 0);
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*s);
                ArcStr::from(__mm_s)
            };
            s = if (b && intNe(i5.clone(), 0)) {
                literal!(" * ")
            } else {
                literal!("")
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*s);
                ArcStr::from(__mm_s)
            };
            s = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("A^("));
                __mm_s.push_str(&*intString(i5.clone()));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            };
            s = if (intEq(i5.clone(), 0)) { literal!("") } else { s };
            b = b || intNe(i5.clone(), 0);
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*s);
                ArcStr::from(__mm_s)
            };
            s = if (b && intNe(i6.clone(), 0)) {
                literal!(" * ")
            } else {
                literal!("")
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*s);
                ArcStr::from(__mm_s)
            };
            s = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("K^("));
                __mm_s.push_str(&*intString(i6.clone()));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            };
            s = if (intEq(i6.clone(), 0)) { literal!("") } else { s };
            b = b || intNe(i6.clone(), 0);
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*s);
                ArcStr::from(__mm_s)
            };
            s = if (b && intNe(i7.clone(), 0)) {
                literal!(" * ")
            } else {
                literal!("")
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*s);
                ArcStr::from(__mm_s)
            };
            s = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("g^("));
                __mm_s.push_str(&*intString(i7.clone()));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            };
            s = if (intEq(i7.clone(), 0)) { literal!("") } else { s };
            b = b || intNe(i7.clone(), 0);
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*s);
                ArcStr::from(__mm_s)
            };
            s = if (b) { literal!("") } else { literal!("1") };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*s);
                ArcStr::from(__mm_s)
            };
            r#str
        }
        Unit::MASTER { varList: ref crefList } => {
            let mut r#str: ArcStr;
            r#str = literal!("MASTER(");
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*printListCr(metamodelica::AsArg::as_arg(&crefList))?);
                ArcStr::from(__mm_s)
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            };
            r#str
        }
        Unit::UNKNOWN { unit: mut s } => {
            let mut r#str: ArcStr;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("UNKOWN("));
                __mm_s.push_str(&*s);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            };
            r#str
        }
    });
    Ok(outString)
}

pub(crate) fn printListCr(mut inlCr: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>) -> Result<ArcStr> {
    let mut outS: ArcStr;
    outS = (::match_deref::match_deref! { match inlCr {
        Deref @ metamodelica::ListNode::Nil => {
            literal!("")
        },
        Deref @ metamodelica::ListNode::Cons { head: cr, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut s: ArcStr;
            s = ComponentReference::crefStr(metamodelica::AsArg::as_arg(&cr))?;
            s
        },
        Deref @ metamodelica::ListNode::Cons { head: cr, tail: lCr } => {
            let mut s: ArcStr;
            s = ComponentReference::crefStr(metamodelica::AsArg::as_arg(&cr))?;
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*printListCr(lCr)?); ArcStr::from(__mm_s) };
            s
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outS)
}

pub(crate) fn unitMul(mut inUnit1: Unit, mut inUnit2: Unit) -> Result<Unit> {
    let mut outUnit: Unit;
    let mut factor1: metamodelica::Real;
    let mut factor2: metamodelica::Real;
    let mut i1: i32;
    let mut i2: i32;
    let mut i3: i32;
    let mut i4: i32;
    let mut i5: i32;
    let mut i6: i32;
    let mut i7: i32;
    let mut j1: i32;
    let mut j2: i32;
    let mut j3: i32;
    let mut j4: i32;
    let mut j5: i32;
    let mut j6: i32;
    let mut j7: i32;
    let Unit::UNIT {
        factor: __pa0,
        mol: __pa1,
        cd: __pa2,
        m: __pa3,
        s: __pa4,
        A: __pa5,
        K: __pa6,
        g: __pa7,
    } = (inUnit1)
    else {
        return Err("pattern mismatch");
    };
    factor1 = metamodelica::Own::own(__pa0);
    i1 = metamodelica::Own::own(__pa1);
    i2 = metamodelica::Own::own(__pa2);
    i3 = metamodelica::Own::own(__pa3);
    i4 = metamodelica::Own::own(__pa4);
    i5 = metamodelica::Own::own(__pa5);
    i6 = metamodelica::Own::own(__pa6);
    i7 = metamodelica::Own::own(__pa7);
    let Unit::UNIT {
        factor: __pa8,
        mol: __pa9,
        cd: __pa10,
        m: __pa11,
        s: __pa12,
        A: __pa13,
        K: __pa14,
        g: __pa15,
    } = (inUnit2)
    else {
        return Err("pattern mismatch");
    };
    factor2 = metamodelica::Own::own(__pa8);
    j1 = metamodelica::Own::own(__pa9);
    j2 = metamodelica::Own::own(__pa10);
    j3 = metamodelica::Own::own(__pa11);
    j4 = metamodelica::Own::own(__pa12);
    j5 = metamodelica::Own::own(__pa13);
    j6 = metamodelica::Own::own(__pa14);
    j7 = metamodelica::Own::own(__pa15);
    factor1 = factor1 * factor2;
    i1 = i1 + j1;
    i2 = i2 + j2;
    i3 = i3 + j3;
    i4 = i4 + j4;
    i5 = i5 + j5;
    i6 = i6 + j6;
    i7 = i7 + j7;
    outUnit = Unit::UNIT {
        factor: factor1,
        mol: i1,
        cd: i2,
        m: i3,
        s: i4,
        A: i5,
        K: i6,
        g: i7,
    };
    Ok(outUnit)
}

pub(crate) fn unitDiv(mut inUnit1: Unit, mut inUnit2: Unit) -> Result<Unit> {
    let mut outUnit: Unit;
    let mut factor1: metamodelica::Real;
    let mut factor2: metamodelica::Real;
    let mut i1: i32;
    let mut i2: i32;
    let mut i3: i32;
    let mut i4: i32;
    let mut i5: i32;
    let mut i6: i32;
    let mut i7: i32;
    let mut j1: i32;
    let mut j2: i32;
    let mut j3: i32;
    let mut j4: i32;
    let mut j5: i32;
    let mut j6: i32;
    let mut j7: i32;
    let Unit::UNIT {
        factor: __pa0,
        mol: __pa1,
        cd: __pa2,
        m: __pa3,
        s: __pa4,
        A: __pa5,
        K: __pa6,
        g: __pa7,
    } = (inUnit1)
    else {
        return Err("pattern mismatch");
    };
    factor1 = metamodelica::Own::own(__pa0);
    i1 = metamodelica::Own::own(__pa1);
    i2 = metamodelica::Own::own(__pa2);
    i3 = metamodelica::Own::own(__pa3);
    i4 = metamodelica::Own::own(__pa4);
    i5 = metamodelica::Own::own(__pa5);
    i6 = metamodelica::Own::own(__pa6);
    i7 = metamodelica::Own::own(__pa7);
    let Unit::UNIT {
        factor: __pa8,
        mol: __pa9,
        cd: __pa10,
        m: __pa11,
        s: __pa12,
        A: __pa13,
        K: __pa14,
        g: __pa15,
    } = (inUnit2)
    else {
        return Err("pattern mismatch");
    };
    factor2 = metamodelica::Own::own(__pa8);
    j1 = metamodelica::Own::own(__pa9);
    j2 = metamodelica::Own::own(__pa10);
    j3 = metamodelica::Own::own(__pa11);
    j4 = metamodelica::Own::own(__pa12);
    j5 = metamodelica::Own::own(__pa13);
    j6 = metamodelica::Own::own(__pa14);
    j7 = metamodelica::Own::own(__pa15);
    factor1 = metamodelica::real_div_checked(factor1, factor2)?;
    i1 = i1 - j1;
    i2 = i2 - j2;
    i3 = i3 - j3;
    i4 = i4 - j4;
    i5 = i5 - j5;
    i6 = i6 - j6;
    i7 = i7 - j7;
    outUnit = Unit::UNIT {
        factor: factor1,
        mol: i1,
        cd: i2,
        m: i3,
        s: i4,
        A: i5,
        K: i6,
        g: i7,
    };
    Ok(outUnit)
}

pub(crate) fn unitPow(mut inUnit: Unit, mut inExp: i32) -> Result<Unit> {
    let mut outUnit: Unit;
    let mut factor: metamodelica::Real;
    let mut i1: i32;
    let mut i2: i32;
    let mut i3: i32;
    let mut i4: i32;
    let mut i5: i32;
    let mut i6: i32;
    let mut i7: i32;
    let Unit::UNIT {
        factor: __pa0,
        mol: __pa1,
        cd: __pa2,
        m: __pa3,
        s: __pa4,
        A: __pa5,
        K: __pa6,
        g: __pa7,
    } = (inUnit)
    else {
        return Err("pattern mismatch");
    };
    factor = metamodelica::Own::own(__pa0);
    i1 = metamodelica::Own::own(__pa1);
    i2 = metamodelica::Own::own(__pa2);
    i3 = metamodelica::Own::own(__pa3);
    i4 = metamodelica::Own::own(__pa4);
    i5 = metamodelica::Own::own(__pa5);
    i6 = metamodelica::Own::own(__pa6);
    i7 = metamodelica::Own::own(__pa7);
    factor = realPow(factor, intReal(inExp));
    i1 = i1 * inExp;
    i2 = i2 * inExp;
    i3 = i3 * inExp;
    i4 = i4 * inExp;
    i5 = i5 * inExp;
    i6 = i6 * inExp;
    i7 = i7 * inExp;
    outUnit = Unit::UNIT {
        factor: factor,
        mol: i1,
        cd: i2,
        m: i3,
        s: i4,
        A: i5,
        K: i6,
        g: i7,
    };
    Ok(outUnit)
}

pub(crate) fn unitMulReal(mut inUnit: Unit, mut inFactor: metamodelica::Real) -> Result<Unit> {
    let mut outUnit: Unit;
    outUnit = (match inUnit {
        mut unit @ Unit::UNIT { .. } => {
            let __owned_variant_factor_0 = var_field!(unit.factor, Unit::UNIT).clone() * inFactor;
            if let Unit::UNIT { factor, .. } = &mut unit {
                *factor = __owned_variant_factor_0;
            } else {
                panic!("owned-variant field-assign: value held a different variant than Unit::UNIT");
            }
            unit.clone()
        }
        _ => return Err("fail"),
    });
    Ok(outUnit)
}

pub(crate) fn unitRoot(mut inUnit: Unit, mut inExponent: metamodelica::Real) -> Result<Unit> {
    let mut outUnit: Unit;
    let mut r: metamodelica::Real;
    let mut factor: metamodelica::Real;
    let mut i: i32;
    let mut i1: i32;
    let mut i2: i32;
    let mut i3: i32;
    let mut i4: i32;
    let mut i5: i32;
    let mut i6: i32;
    let mut i7: i32;
    i = ((inExponent).0.floor() as i32);
    r = realDiv(metamodelica::OrderedFloat(1.0_f64), inExponent);
    let Unit::UNIT {
        factor: __pa0,
        mol: __pa1,
        cd: __pa2,
        m: __pa3,
        s: __pa4,
        A: __pa5,
        K: __pa6,
        g: __pa7,
    } = (inUnit)
    else {
        return Err("pattern mismatch");
    };
    factor = metamodelica::Own::own(__pa0);
    i1 = metamodelica::Own::own(__pa1);
    i2 = metamodelica::Own::own(__pa2);
    i3 = metamodelica::Own::own(__pa3);
    i4 = metamodelica::Own::own(__pa4);
    i5 = metamodelica::Own::own(__pa5);
    i6 = metamodelica::Own::own(__pa6);
    i7 = metamodelica::Own::own(__pa7);
    factor = realPow(factor, r);
    r = realDiv(intReal(i1), inExponent);
    i1 = intDiv(i1, i);
    let true = (realEq(r, intReal(i1))) else {
        return Err("pattern mismatch");
    };
    r = realDiv(intReal(i2), inExponent);
    i2 = intDiv(i2, i);
    let true = (realEq(r, intReal(i2))) else {
        return Err("pattern mismatch");
    };
    r = realDiv(intReal(i3), inExponent);
    i3 = intDiv(i3, i);
    let true = (realEq(r, intReal(i3))) else {
        return Err("pattern mismatch");
    };
    r = realDiv(intReal(i4), inExponent);
    i4 = intDiv(i4, i);
    let true = (realEq(r, intReal(i4))) else {
        return Err("pattern mismatch");
    };
    r = realDiv(intReal(i5), inExponent);
    i5 = intDiv(i5, i);
    let true = (realEq(r, intReal(i5))) else {
        return Err("pattern mismatch");
    };
    r = realDiv(intReal(i6), inExponent);
    i6 = intDiv(i6, i);
    let true = (realEq(r, intReal(i6))) else {
        return Err("pattern mismatch");
    };
    r = realDiv(intReal(i7), inExponent);
    i7 = intDiv(i7, i);
    let true = (realEq(r, intReal(i7))) else {
        return Err("pattern mismatch");
    };
    outUnit = Unit::UNIT {
        factor: factor,
        mol: i1,
        cd: i2,
        m: i3,
        s: i4,
        A: i5,
        K: i6,
        g: i7,
    };
    Ok(outUnit)
}

pub(crate) fn unitString(
    mut inUnit: Unit,
    mut inHtU2S: &(
        metamodelica::Array<metamodelica::List<(Unit, i32)>>,
        (i32, i32, metamodelica::Array<Option<(Unit, ArcStr)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(Unit) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit, Unit) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inUnit.clone() {
        _ if (BaseHashTable::hasKey(inUnit.clone(), inHtU2S)?) => {
            let mut s: ArcStr;
            s = BaseHashTable::get(inUnit.clone(), inHtU2S)?;
            s
        }
        mut unit @ Unit::UNIT { .. } => {
            let mut s: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut s4: ArcStr;
            let mut s5: ArcStr;
            let mut s6: ArcStr;
            let mut s7: ArcStr;
            let mut sExponent: ArcStr;
            let mut b: bool;
            s = prefix2String(var_field!(unit.factor, Unit::UNIT).clone());
            s = if (realEq(
                var_field!(unit.factor, Unit::UNIT).clone(),
                metamodelica::OrderedFloat(1.0_f64),
            )) {
                literal!("")
            } else {
                s
            };
            b = false;
            sExponent = if (intEq(var_field!(unit.mol, Unit::UNIT).clone(), 1)) {
                literal!("")
            } else {
                intString(var_field!(unit.mol, Unit::UNIT).clone())
            };
            s1 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("mol"));
                __mm_s.push_str(&*sExponent);
                ArcStr::from(__mm_s)
            };
            s1 = if (intEq(var_field!(unit.mol, Unit::UNIT).clone(), 0)) {
                literal!("")
            } else {
                s1
            };
            b = b || intNe(var_field!(unit.mol, Unit::UNIT).clone(), 0);
            s2 = if (b && intNe(var_field!(unit.cd, Unit::UNIT).clone(), 0)) {
                literal!(".")
            } else {
                literal!("")
            };
            sExponent = if (intEq(var_field!(unit.cd, Unit::UNIT).clone(), 1)) {
                literal!("")
            } else {
                intString(var_field!(unit.cd, Unit::UNIT).clone())
            };
            s2 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*s2);
                __mm_s.push_str(&*literal!("cd"));
                __mm_s.push_str(&*sExponent);
                ArcStr::from(__mm_s)
            };
            s2 = if (intEq(var_field!(unit.cd, Unit::UNIT).clone(), 0)) {
                literal!("")
            } else {
                s2
            };
            b = b || intNe(var_field!(unit.cd, Unit::UNIT).clone(), 0);
            s3 = if (b && intNe(var_field!(unit.m, Unit::UNIT).clone(), 0)) {
                literal!(".")
            } else {
                literal!("")
            };
            sExponent = if (intEq(var_field!(unit.m, Unit::UNIT).clone(), 1)) {
                literal!("")
            } else {
                intString(var_field!(unit.m, Unit::UNIT).clone())
            };
            s3 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*s3);
                __mm_s.push_str(&*literal!("m"));
                __mm_s.push_str(&*sExponent);
                ArcStr::from(__mm_s)
            };
            s3 = if (intEq(var_field!(unit.m, Unit::UNIT).clone(), 0)) {
                literal!("")
            } else {
                s3
            };
            b = b || intNe(var_field!(unit.m, Unit::UNIT).clone(), 0);
            s4 = if (b && intNe(var_field!(unit.s, Unit::UNIT).clone(), 0)) {
                literal!(".")
            } else {
                literal!("")
            };
            sExponent = if (intEq(var_field!(unit.s, Unit::UNIT).clone(), 1)) {
                literal!("")
            } else {
                intString(var_field!(unit.s, Unit::UNIT).clone())
            };
            s4 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*s4);
                __mm_s.push_str(&*literal!("s"));
                __mm_s.push_str(&*sExponent);
                ArcStr::from(__mm_s)
            };
            s4 = if (intEq(var_field!(unit.s, Unit::UNIT).clone(), 0)) {
                literal!("")
            } else {
                s4
            };
            b = b || intNe(var_field!(unit.s, Unit::UNIT).clone(), 0);
            s5 = if (b && intNe(var_field!(unit.A, Unit::UNIT).clone(), 0)) {
                literal!(".")
            } else {
                literal!("")
            };
            sExponent = if (intEq(var_field!(unit.A, Unit::UNIT).clone(), 1)) {
                literal!("")
            } else {
                intString(var_field!(unit.A, Unit::UNIT).clone())
            };
            s5 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*s5);
                __mm_s.push_str(&*literal!("A"));
                __mm_s.push_str(&*sExponent);
                ArcStr::from(__mm_s)
            };
            s5 = if (intEq(var_field!(unit.A, Unit::UNIT).clone(), 0)) {
                literal!("")
            } else {
                s5
            };
            b = b || intNe(var_field!(unit.A, Unit::UNIT).clone(), 0);
            s6 = if (b && intNe(var_field!(unit.K, Unit::UNIT).clone(), 0)) {
                literal!(".")
            } else {
                literal!("")
            };
            sExponent = if (intEq(var_field!(unit.K, Unit::UNIT).clone(), 1)) {
                literal!("")
            } else {
                intString(var_field!(unit.K, Unit::UNIT).clone())
            };
            s6 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*s6);
                __mm_s.push_str(&*literal!("K"));
                __mm_s.push_str(&*sExponent);
                ArcStr::from(__mm_s)
            };
            s6 = if (intEq(var_field!(unit.K, Unit::UNIT).clone(), 0)) {
                literal!("")
            } else {
                s6
            };
            b = b || intNe(var_field!(unit.K, Unit::UNIT).clone(), 0);
            s7 = if (b && intNe(var_field!(unit.g, Unit::UNIT).clone(), 0)) {
                literal!(".")
            } else {
                literal!("")
            };
            sExponent = if (intEq(var_field!(unit.g, Unit::UNIT).clone(), 1)) {
                literal!("")
            } else {
                intString(var_field!(unit.g, Unit::UNIT).clone())
            };
            s7 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*s7);
                __mm_s.push_str(&*literal!("g"));
                __mm_s.push_str(&*sExponent);
                ArcStr::from(__mm_s)
            };
            s7 = if (intEq(var_field!(unit.g, Unit::UNIT).clone(), 0)) {
                literal!("")
            } else {
                s7
            };
            b = b || intNe(var_field!(unit.g, Unit::UNIT).clone(), 0);
            s = if (b) {
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*s);
                    __mm_s.push_str(&*s1);
                    __mm_s.push_str(&*s2);
                    __mm_s.push_str(&*s3);
                    __mm_s.push_str(&*s4);
                    __mm_s.push_str(&*s5);
                    __mm_s.push_str(&*s6);
                    __mm_s.push_str(&*s7);
                    ArcStr::from(__mm_s)
                }
            } else {
                literal!("1")
            };
            s
        }
        _ => {
            Error::addCompilerWarning({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("function Unit.unitString failed for \""));
                __mm_s.push_str(&*unit2string(&inUnit)?);
                __mm_s.push_str(&*literal!("\"."));
                ArcStr::from(__mm_s)
            })?;
            return Err("fail");
        }
    });
    Ok(outString)
}

fn prefix2String(mut inReal: metamodelica::Real) -> ArcStr {
    let mut outPrefix: ArcStr;
    outPrefix = (match inReal {
        __rlit_0 if __rlit_0.eq(&metamodelica::OrderedFloat((1e-24) as f64)) => literal!("y"),
        __rlit_1 if __rlit_1.eq(&metamodelica::OrderedFloat((1e-21) as f64)) => literal!("z"),
        __rlit_2 if __rlit_2.eq(&metamodelica::OrderedFloat((1e-18) as f64)) => literal!("a"),
        __rlit_3 if __rlit_3.eq(&metamodelica::OrderedFloat((1e-15) as f64)) => literal!("f"),
        __rlit_4 if __rlit_4.eq(&metamodelica::OrderedFloat((1e-12) as f64)) => literal!("p"),
        __rlit_5 if __rlit_5.eq(&metamodelica::OrderedFloat((1e-6) as f64)) => literal!("u"),
        __rlit_6 if __rlit_6.eq(&metamodelica::OrderedFloat((1e-3) as f64)) => literal!("m"),
        __rlit_7 if __rlit_7.eq(&metamodelica::OrderedFloat((1e-2) as f64)) => literal!("c"),
        __rlit_8 if __rlit_8.eq(&metamodelica::OrderedFloat((1e-1) as f64)) => literal!("d"),
        __rlit_9 if __rlit_9.eq(&metamodelica::OrderedFloat((1e1) as f64)) => literal!("da"),
        __rlit_10 if __rlit_10.eq(&metamodelica::OrderedFloat((1e2) as f64)) => literal!("h"),
        __rlit_11 if __rlit_11.eq(&metamodelica::OrderedFloat((1e3) as f64)) => literal!("k"),
        __rlit_12 if __rlit_12.eq(&metamodelica::OrderedFloat((1e6) as f64)) => literal!("M"),
        __rlit_13 if __rlit_13.eq(&metamodelica::OrderedFloat((1e9) as f64)) => literal!("G"),
        __rlit_14 if __rlit_14.eq(&metamodelica::OrderedFloat((1e12) as f64)) => literal!("T"),
        __rlit_15 if __rlit_15.eq(&metamodelica::OrderedFloat((1e15) as f64)) => literal!("P"),
        __rlit_16 if __rlit_16.eq(&metamodelica::OrderedFloat((1e18) as f64)) => literal!("E"),
        __rlit_17 if __rlit_17.eq(&metamodelica::OrderedFloat((1e21) as f64)) => literal!("Z"),
        __rlit_18 if __rlit_18.eq(&metamodelica::OrderedFloat((1e24) as f64)) => literal!("Y"),
        _ => realString(inReal),
    });
    outPrefix
}

pub(crate) fn parseUnitString(
    mut inUnitString: ArcStr,
    mut inKnownUnits: &(
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<Unit> {
    let mut outUnit: Unit;
    let mut charList: metamodelica::List<ArcStr>;
    let mut tokenList: metamodelica::List<Token>;
    charList = stringListStringChar(inUnitString);
    if (charList).is_empty() {
        return Err("fail");
    }
    tokenList = lexer(charList)?;
    outUnit = parser3(
        &(list![true, true]),
        &tokenList,
        &(Unit::UNIT {
            factor: metamodelica::OrderedFloat(1e0_f64),
            mol: 0,
            cd: 0,
            m: 0,
            s: 0,
            A: 0,
            K: 0,
            g: 0,
        }),
        inKnownUnits,
    )?;
    if !(isUnit(&outUnit)) {
        return Err("fail");
    }
    Ok(outUnit)
}

fn parser3(
    mut inMul: &metamodelica::List<bool>,
    mut inTokenList: &metamodelica::List<Token>,
    mut inUnit: &Unit,
    mut inHtS2U: &(
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<Unit> {
    let mut outUnit: Unit;
    outUnit = 'mc: {
        let __mc_input = (&**inMul, &**inTokenList);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: true, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(inUnit.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: bMul, tail: bRest }, Deref @ metamodelica::ListNode::Cons { head: Token::T_NUMBER { number: 1 }, tail: tokens }) => {
                    let mut ut: Unit;
                    ut = Unit::UNIT { factor: metamodelica::OrderedFloat(1e0_f64), mol: 0, cd: 0, m: 0, s: 0, A: 0, K: 0, g: 0 };
                    ut = if (bMul.clone()) {unitMul(inUnit.clone(), ut.clone())?} else {unitDiv(inUnit.clone(), ut.clone())?};
                    ut = parser3(metamodelica::AsArg::as_arg(&bRest), metamodelica::AsArg::as_arg(&tokens), &ut, inHtS2U)?;
                    Ok(ut.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: bMul, tail: bRest }, Deref @ metamodelica::ListNode::Cons { head: Token::T_UNIT { unit: s }, tail: Deref @ metamodelica::ListNode::Cons { head: Token::T_NUMBER { number: exponent }, tail: tokens } }) => {
                    let mut ut: Unit;
                    ut = unitToken2unit(metamodelica::AsArg::as_arg(&s), inHtS2U)?;
                    ut = unitPow(ut.clone(), exponent.clone())?;
                    ut = if (bMul.clone()) {unitMul(inUnit.clone(), ut.clone())?} else {unitDiv(inUnit.clone(), ut.clone())?};
                    ut = parser3(metamodelica::AsArg::as_arg(&bRest), metamodelica::AsArg::as_arg(&tokens), &ut, inHtS2U)?;
                    Ok(ut.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: bMul, tail: bRest }, Deref @ metamodelica::ListNode::Cons { head: Token::T_UNIT { unit: s }, tail: tokens }) => {
                    let mut ut: Unit;
                    ut = unitToken2unit(metamodelica::AsArg::as_arg(&s), inHtS2U)?;
                    ut = if (bMul.clone()) {unitMul(inUnit.clone(), ut.clone())?} else {unitDiv(inUnit.clone(), ut.clone())?};
                    ut = parser3(metamodelica::AsArg::as_arg(&bRest), metamodelica::AsArg::as_arg(&tokens), &ut, inHtS2U)?;
                    Ok(ut.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: bMul, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: Token::T_MUL { .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Token::T_LPAREN { .. }, tail: tokens } }) => {
                    let mut ut: Unit;
                    ut = parser3(&(metamodelica::cons(bMul.clone(), metamodelica::cons(bMul.clone(), inMul.clone()))), metamodelica::AsArg::as_arg(&tokens), inUnit, inHtS2U)?;
                    Ok(ut.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: bMul, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: Token::T_DIV { .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Token::T_LPAREN { .. }, tail: tokens } }) => {
                    let mut ut: Unit;
                    let mut b: bool;
                    b = !(bMul.clone());
                    ut = parser3(&(metamodelica::cons(b, metamodelica::cons(b, inMul.clone()))), metamodelica::AsArg::as_arg(&tokens), inUnit, inHtS2U)?;
                    Ok(ut.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: bRest }, Deref @ metamodelica::ListNode::Cons { head: Token::T_RPAREN { .. }, tail: tokens }) => {
                    let mut ut: Unit;
                    ut = parser3(metamodelica::AsArg::as_arg(&bRest), metamodelica::AsArg::as_arg(&tokens), inUnit, inHtS2U)?;
                    Ok(ut.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: bMul, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: Token::T_MUL { .. }, tail: tokens }) => {
                    let mut ut: Unit;
                    ut = parser3(&(metamodelica::cons(bMul.clone(), inMul.clone())), metamodelica::AsArg::as_arg(&tokens), inUnit, inHtS2U)?;
                    Ok(ut.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: bMul, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: Token::T_DIV { .. }, tail: tokens }) => {
                    let mut ut: Unit;
                    let mut b: bool;
                    b = !(bMul.clone());
                    ut = parser3(&(metamodelica::cons(b, inMul.clone())), metamodelica::AsArg::as_arg(&tokens), inUnit, inHtS2U)?;
                    Ok(ut.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outUnit)
}

fn unitToken2unit(
    mut inS: &ArcStr,
    mut inHtS2U: &(
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (i32, i32, metamodelica::Array<Option<(ArcStr, Unit)>>),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(Unit) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<Unit> {
    let mut outUnit: Unit;
    outUnit = 'mc: {
        let __mc_input = inHtS2U.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut ut: Unit;
            ut = BaseHashTable::get(inS.clone(), inHtS2U)?;
            Ok(ut.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut s: ArcStr;
            let mut r: metamodelica::Real;
            let mut ut: Unit;
            s = stringGetStringChar(inS.clone(), 1)?;
            (r, s) = getPrefix(&s, inS.clone())?;
            ut = unitToken2unit(&s, inHtS2U)?;
            ut = unitMulReal(ut.clone(), r)?;
            Ok(ut.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outUnit)
}

fn getPrefix(mut inS: &ArcStr, mut inS2: ArcStr) -> Result<(metamodelica::Real, ArcStr)> {
    let mut outR: metamodelica::Real;
    let mut outUnit: ArcStr;
    (outR, outUnit) = 'mc: {
        let __mc_input = inS.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "y" => {
                    let mut strRest: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar(inS2.clone())) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    strRest = metamodelica::Own::own(__pa0);
                    s = stringCharListString(strRest.clone());
                    Ok((metamodelica::OrderedFloat(1e-24_f64), s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "z" => {
                    let mut strRest: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar(inS2.clone())) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    strRest = metamodelica::Own::own(__pa0);
                    s = stringCharListString(strRest.clone());
                    Ok((metamodelica::OrderedFloat(1e-21_f64), s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "a" => {
                    let mut strRest: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar(inS2.clone())) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    strRest = metamodelica::Own::own(__pa0);
                    s = stringCharListString(strRest.clone());
                    Ok((metamodelica::OrderedFloat(1e-18_f64), s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "f" => {
                    let mut strRest: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar(inS2.clone())) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    strRest = metamodelica::Own::own(__pa0);
                    s = stringCharListString(strRest.clone());
                    Ok((metamodelica::OrderedFloat(1e-15_f64), s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "p" => {
                    let mut strRest: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar(inS2.clone())) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    strRest = metamodelica::Own::own(__pa0);
                    s = stringCharListString(strRest.clone());
                    Ok((metamodelica::OrderedFloat(1e-12_f64), s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "u" => {
                    let mut strRest: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar(inS2.clone())) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    strRest = metamodelica::Own::own(__pa0);
                    s = stringCharListString(strRest.clone());
                    Ok((metamodelica::OrderedFloat(1e-6_f64), s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "m" => {
                    let mut strRest: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar(inS2.clone())) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    strRest = metamodelica::Own::own(__pa0);
                    s = stringCharListString(strRest.clone());
                    Ok((metamodelica::OrderedFloat(1e-3_f64), s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "c" => {
                    let mut strRest: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar(inS2.clone())) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    strRest = metamodelica::Own::own(__pa0);
                    s = stringCharListString(strRest.clone());
                    Ok((metamodelica::OrderedFloat(1e-2_f64), s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "d" => {
                    let mut strRest: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    strRest = stringListStringChar(inS2.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(strRest.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ "d", tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ "a", tail: __pa0 } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    strRest = metamodelica::Own::own(__pa0);
                    s = stringCharListString(strRest.clone());
                    Ok((metamodelica::OrderedFloat(1e1_f64), s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "d" => {
                    let mut strRest: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar(inS2.clone())) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    strRest = metamodelica::Own::own(__pa0);
                    s = stringCharListString(strRest.clone());
                    Ok((metamodelica::OrderedFloat(1e-1_f64), s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "h" => {
                    let mut strRest: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar(inS2.clone())) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    strRest = metamodelica::Own::own(__pa0);
                    s = stringCharListString(strRest.clone());
                    Ok((metamodelica::OrderedFloat(1e2_f64), s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "k" => {
                    let mut strRest: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar(inS2.clone())) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    strRest = metamodelica::Own::own(__pa0);
                    s = stringCharListString(strRest.clone());
                    Ok((metamodelica::OrderedFloat(1e3_f64), s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "M" => {
                    let mut strRest: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar(inS2.clone())) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    strRest = metamodelica::Own::own(__pa0);
                    s = stringCharListString(strRest.clone());
                    Ok((metamodelica::OrderedFloat(1e6_f64), s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "G" => {
                    let mut strRest: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar(inS2.clone())) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    strRest = metamodelica::Own::own(__pa0);
                    s = stringCharListString(strRest.clone());
                    Ok((metamodelica::OrderedFloat(1e9_f64), s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "T" => {
                    let mut strRest: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar(inS2.clone())) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    strRest = metamodelica::Own::own(__pa0);
                    s = stringCharListString(strRest.clone());
                    Ok((metamodelica::OrderedFloat(1e12_f64), s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "P" => {
                    let mut strRest: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar(inS2.clone())) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    strRest = metamodelica::Own::own(__pa0);
                    s = stringCharListString(strRest.clone());
                    Ok((metamodelica::OrderedFloat(1e15_f64), s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "E" => {
                    let mut strRest: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar(inS2.clone())) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    strRest = metamodelica::Own::own(__pa0);
                    s = stringCharListString(strRest.clone());
                    Ok((metamodelica::OrderedFloat(1e18_f64), s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "Z" => {
                    let mut strRest: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar(inS2.clone())) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    strRest = metamodelica::Own::own(__pa0);
                    s = stringCharListString(strRest.clone());
                    Ok((metamodelica::OrderedFloat(1e21_f64), s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "Y" => {
                    let mut strRest: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar(inS2.clone())) {
                        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    strRest = metamodelica::Own::own(__pa0);
                    s = stringCharListString(strRest.clone());
                    Ok((metamodelica::OrderedFloat(1e24_f64), s.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outR, outUnit))
}

fn lexer(mut inCharList: metamodelica::List<ArcStr>) -> Result<metamodelica::List<Token>> {
    let mut outTokenList: metamodelica::List<Token>;
    outTokenList = 'mc: {
        let __mc_input = inCharList;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ ".", tail: charList } => {
                    let mut tokenList: metamodelica::List<Token>;
                    tokenList = lexer(charList.clone())?;
                    Ok(metamodelica::cons(crate::FUnit::Token::T_MUL, tokenList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ "(", tail: charList } => {
                    let mut tokenList: metamodelica::List<Token>;
                    tokenList = lexer(charList.clone())?;
                    Ok(metamodelica::cons(crate::FUnit::Token::T_LPAREN, tokenList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ ")", tail: charList } => {
                    let mut tokenList: metamodelica::List<Token>;
                    tokenList = lexer(charList.clone())?;
                    Ok(metamodelica::cons(crate::FUnit::Token::T_RPAREN, tokenList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ "/", tail: charList } => {
                    let mut tokenList: metamodelica::List<Token>;
                    tokenList = lexer(charList.clone())?;
                    Ok(metamodelica::cons(crate::FUnit::Token::T_DIV, tokenList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ "+", tail: charList } => {
                    let mut number: ArcStr;
                    let mut tokenList: metamodelica::List<Token>;
                    let mut i: i32;
                    let mut charList = (*charList).clone();
                    (charList, number) = popNumber(metamodelica::AsArg::as_arg(&charList));
                    let false = (metamodelica::stringEq(&number, &(literal!("")))) else { return Err("pattern mismatch") };
                    tokenList = lexer(charList.clone())?;
                    i = stringInt(number.clone())?;
                    Ok(metamodelica::cons(Token::T_NUMBER { number: i }, tokenList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ "-", tail: charList } => {
                    let mut number: ArcStr;
                    let mut tokenList: metamodelica::List<Token>;
                    let mut i: i32;
                    let mut charList = (*charList).clone();
                    (charList, number) = popNumber(metamodelica::AsArg::as_arg(&charList));
                    let false = (metamodelica::stringEq(&number, &(literal!("")))) else { return Err("pattern mismatch") };
                    tokenList = lexer(charList.clone())?;
                    i = -(stringInt(number.clone())?);
                    Ok(metamodelica::cons(Token::T_NUMBER { number: i }, tokenList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                charList => {
                    let mut number: ArcStr;
                    let mut tokenList: metamodelica::List<Token>;
                    let mut i: i32;
                    let mut charList = (*charList).clone();
                    (charList, number) = popNumber(metamodelica::AsArg::as_arg(&charList));
                    let false = (metamodelica::stringEq(&number, &(literal!("")))) else { return Err("pattern mismatch") };
                    tokenList = lexer(charList.clone())?;
                    i = stringInt(number.clone())?;
                    Ok(metamodelica::cons(Token::T_NUMBER { number: i }, tokenList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                charList => {
                    let mut unit: ArcStr;
                    let mut tokenList: metamodelica::List<Token>;
                    let mut charList = (*charList).clone();
                    (charList, unit) = popUnit(metamodelica::AsArg::as_arg(&charList));
                    let false = (metamodelica::stringEq(&unit, &(literal!("")))) else { return Err("pattern mismatch") };
                    tokenList = lexer(charList.clone())?;
                    Ok(metamodelica::cons(Token::T_UNIT { unit: unit.clone() }, tokenList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function lexer failed"), metamodelica::sourceInfo!("FrontEnd/FUnit.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outTokenList)
}

fn popUnit(mut inCharList: &metamodelica::List<ArcStr>) -> (metamodelica::List<ArcStr>, ArcStr) {
    let mut outCharList: metamodelica::List<ArcStr>;
    let mut outUnit: ArcStr;
    (outCharList, outUnit) = (::match_deref::match_deref! { match inCharList {
        Deref @ metamodelica::ListNode::Nil => {
            (metamodelica::nil(), literal!(""))
        },
        Deref @ metamodelica::ListNode::Cons { head: s1, tail: strRest } if (stringCompare(&s1, &(literal!("a"))) >= 0 && stringCompare(&s1, &(literal!("z"))) <= 0) => {
            let mut s2: ArcStr;
            let mut strRest = (*strRest).clone();
            (strRest, s2) = popUnit(metamodelica::AsArg::as_arg(&strRest));
            (strRest.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*s1); __mm_s.push_str(&*s2); ArcStr::from(__mm_s) })
        },
        Deref @ metamodelica::ListNode::Cons { head: s1, tail: strRest } if (stringCompare(&s1, &(literal!("A"))) >= 0 && stringCompare(&s1, &(literal!("Z"))) <= 0) => {
            let mut s2: ArcStr;
            let mut strRest = (*strRest).clone();
            (strRest, s2) = popUnit(metamodelica::AsArg::as_arg(&strRest));
            (strRest.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*s1); __mm_s.push_str(&*s2); ArcStr::from(__mm_s) })
        },
        _ => {
            (inCharList.clone(), literal!(""))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outCharList, outUnit)
}

fn popNumber(mut inCharList: &metamodelica::List<ArcStr>) -> (metamodelica::List<ArcStr>, ArcStr) {
    let mut outCharList: metamodelica::List<ArcStr>;
    let mut outNumber: ArcStr;
    (outCharList, outNumber) = 'mc: {
        let __mc_input = &**inCharList;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((metamodelica::nil(), literal!("")))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: s1, tail: strRest } => {
                    let mut s2: ArcStr;
                    let mut i: i32;
                    let mut strRest = (*strRest).clone();
                    i = stringInt(s1.clone())?;
                    let true = (metamodelica::stringEq(&(intString(i)), &s1)) else { return Err("pattern mismatch") };
                    (strRest, s2) = popNumber(metamodelica::AsArg::as_arg(&strRest));
                    Ok((strRest.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*s1); __mm_s.push_str(&*s2); ArcStr::from(__mm_s) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inCharList.clone(), literal!("")))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outCharList, outNumber)
}
