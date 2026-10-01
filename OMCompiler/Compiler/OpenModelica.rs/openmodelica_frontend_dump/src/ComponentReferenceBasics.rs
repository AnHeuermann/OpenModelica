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

use crate::ExpressionBasics;
use crate::TypesDump;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Config;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
pub fn crefDims(
    mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Dimension>>> {
    let mut outDimensionLst: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    outDimensionLst = (match &**inComponentRef {
        DAE::ComponentRef::CREF_IDENT { identType: idType, .. } => TypesDump::getDimensions(idType),
        DAE::ComponentRef::CREF_QUAL {
            componentRef: cr,
            identType: idType,
            ..
        } => {
            let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut res: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            dims = TypesDump::getDimensions(idType);
            res = crefDims(cr)?;
            res = listAppend(dims, res);
            res
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outDimensionLst)
}

pub fn crefSubs(
    mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Subscript>>> {
    let mut outSubscriptLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    outSubscriptLst = (match &**inComponentRef {
        DAE::ComponentRef::CREF_IDENT { subscriptLst: subs, .. } => subs.clone(),
        DAE::ComponentRef::CREF_QUAL {
            componentRef: cr,
            subscriptLst: subs,
            ..
        } => {
            let mut res: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            res = crefSubs(cr)?;
            res = listAppend(subs.clone(), res);
            res
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outSubscriptLst)
}

/* **************************************************/
/* Compare  */
/* **************************************************/
pub fn crefLastIdentEqual(
    mut cr1: &metamodelica::Ref<DAE::ComponentRef>,
    mut cr2: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut equal: bool;
    let mut id1: ArcStr;
    let mut id2: ArcStr;
    id1 = crefLastIdent(cr1)?;
    id2 = crefLastIdent(cr2)?;
    equal = stringEq(&id1, &id2);
    Ok(equal)
}

pub fn crefFirstCrefEqual(
    mut cr1: metamodelica::Ref<DAE::ComponentRef>,
    mut cr2: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut equal: bool;
    let mut pcr1: metamodelica::Ref<DAE::ComponentRef>;
    let mut pcr2: metamodelica::Ref<DAE::ComponentRef>;
    pcr1 = crefFirstCref(cr1)?;
    pcr2 = crefFirstCref(cr2)?;
    equal = crefEqual(&pcr1, &pcr2)?;
    Ok(equal)
}

pub fn crefFirstCrefLastCrefEqual(
    mut cr1: metamodelica::Ref<DAE::ComponentRef>,
    mut cr2: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut equal: bool;
    let mut pcr1: metamodelica::Ref<DAE::ComponentRef>;
    let mut pcr2: metamodelica::Ref<DAE::ComponentRef>;
    pcr1 = crefFirstCref(cr1)?;
    pcr2 = crefLastCref(cr2)?;
    equal = crefEqual(&pcr1, &pcr2)?;
    Ok(equal)
}

pub fn crefFirstCref(mut inCr: metamodelica::Ref<DAE::ComponentRef>) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCr: metamodelica::Ref<DAE::ComponentRef>;
    outCr = (match &*inCr {
        DAE::ComponentRef::CREF_QUAL {
            ident: id,
            identType: t2,
            subscriptLst: subs,
            componentRef: _,
        } => makeCrefIdent(id.clone(), t2.clone(), subs.clone()),
        DAE::ComponentRef::CREF_IDENT {
            ident: _,
            identType: _,
            subscriptLst: _,
        } => inCr,
        _ => return Err("match: no arm matched"),
    });
    Ok(outCr)
}

pub fn crefLastIdent<'__b>(mut inComponentRef: &'__b metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> {
    '__tco: loop {
        match &**inComponentRef {
            DAE::ComponentRef::CREF_IDENT { ident: id, .. } => return Ok(id.clone()),
            DAE::ComponentRef::CREF_QUAL { componentRef: cr, .. } => {
                let mut res: ArcStr;
                {
                    inComponentRef = cr;
                    continue '__tco;
                }
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

pub fn crefLastCref<'__b>(
    mut inComponentRef: &'__b metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    '__tco: loop {
        match &**inComponentRef {
            DAE::ComponentRef::CREF_IDENT { .. } => return Ok(inComponentRef.clone()),
            DAE::ComponentRef::CREF_QUAL { componentRef: cr, .. } => {
                let mut res: metamodelica::Ref<DAE::ComponentRef>;
                {
                    inComponentRef = cr;
                    continue '__tco;
                }
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

pub fn crefFirstIdentEqual(
    mut inCref1: &metamodelica::Ref<DAE::ComponentRef>,
    mut inCref2: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut outEqual: bool;
    let mut id1: ArcStr;
    let mut id2: ArcStr;
    id1 = crefFirstIdent(inCref1)?;
    id2 = crefFirstIdent(inCref2)?;
    outEqual = stringEq(&id1, &id2);
    Ok(outEqual)
}

pub fn crefFirstIdent(mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> {
    let mut outIdent: ArcStr;
    outIdent = (match &**inComponentRef {
        DAE::ComponentRef::CREF_IDENT { ident: id, .. } => id.clone(),
        DAE::ComponentRef::CREF_QUAL { ident: id, .. } => id.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(outIdent)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum CompareWithSubsType {
    WithoutSubscripts = 1,
    WithGenericSubscript = 2,
    WithGenericSubscriptNotAlphabetic = 3,
    WithIntSubscript = 4,
}
impl PartialOrd for CompareWithSubsType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CompareWithSubsType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for CompareWithSubsType {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub mod CompareWithGenericSubscript {
    use super::*;
    pub(crate) static compareSubscript: std::sync::LazyLock<CompareWithSubsType> =
        std::sync::LazyLock::new(|| CompareWithSubsType::WithGenericSubscript.clone());

    pub(crate) fn compare<'__b>(
        mut cr1: &'__b metamodelica::Ref<DAE::ComponentRef>,
        mut cr2: &'__b metamodelica::Ref<DAE::ComponentRef>,
    ) -> Result<i32> {
        let mut res: i32;
        res = (::match_deref::match_deref! { match (cr1, cr2) {
            (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => {
                res = stringCompare(&var_field!((**cr1).ident, DAE::ComponentRef::CREF_IDENT), &var_field!((**cr2).ident, DAE::ComponentRef::CREF_IDENT));
                if compareSubscript.clone() == CompareWithSubsType::WithoutSubscripts.clone() || res != 0 {
                    return Ok(res);
                }
                compareSubs(var_field!((**cr1).subscriptLst, DAE::ComponentRef::CREF_IDENT), var_field!((**cr2).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?
            },
            (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => {
                res = stringCompare(&var_field!((**cr1).ident, DAE::ComponentRef::CREF_QUAL), &var_field!((**cr2).ident, DAE::ComponentRef::CREF_QUAL));
                if res != 0 {
                    return Ok(res);
                }
                if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                    res = compareSubs(var_field!((**cr1).subscriptLst, DAE::ComponentRef::CREF_QUAL), var_field!((**cr2).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())?;
                    if res != 0 {
                        return Ok(res);
                    }
                }
                compare(var_field!((**cr1).componentRef, DAE::ComponentRef::CREF_QUAL), var_field!((**cr2).componentRef, DAE::ComponentRef::CREF_QUAL))?
            },
            (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => {
                res = stringCompare(&var_field!((**cr1).ident, DAE::ComponentRef::CREF_QUAL), &var_field!((**cr2).ident, DAE::ComponentRef::CREF_IDENT));
                if res != 0 {
                    return Ok(res);
                }
                if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                    res = compareSubs(var_field!((**cr1).subscriptLst, DAE::ComponentRef::CREF_QUAL), var_field!((**cr2).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?;
                }
                if res != 0 {
                    return Ok(res);
                }
                1
            },
            (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => {
                res = stringCompare(&var_field!((**cr1).ident, DAE::ComponentRef::CREF_IDENT), &var_field!((**cr2).ident, DAE::ComponentRef::CREF_QUAL));
                if res != 0 {
                    return Ok(res);
                }
                if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                    res = compareSubs(var_field!((**cr1).subscriptLst, DAE::ComponentRef::CREF_IDENT), var_field!((**cr2).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())?;
                }
                if res != 0 {
                    return Ok(res);
                }
                -1
            },
            _ => return Err("match: no arm matched"),
        } });
        Ok(res)
    }

    pub(crate) fn compareSubs(
        mut ss1: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
        mut ss2: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    ) -> Result<i32> {
        let mut res: i32 = 0;
        let mut ss: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = ss2;
        let mut s2: metamodelica::Ref<DAE::Subscript>;
        let mut i1: i32;
        let mut i2: i32;
        for mut s1 in &**ss1 {
            if (ss).is_empty() {
                res = -1;
                return Ok(res);
            }
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ss) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            s2 = metamodelica::Own::own(__pa0);
            ss = metamodelica::Own::own(__pa1);
            if compareSubscript.clone() == CompareWithSubsType::WithGenericSubscript.clone() {
                res = compareSubscriptStr(metamodelica::AsArg::as_arg(&s1), &s2)?;
            } else if compareSubscript.clone() == CompareWithSubsType::WithGenericSubscriptNotAlphabetic.clone() {
                res = ExpressionBasics::compareSubscripts(metamodelica::AsArg::as_arg(&s1), &s2)?;
            } else {
                i1 = ExpressionBasics::subscriptInt(metamodelica::AsArg::as_arg(&s1))?;
                i2 = ExpressionBasics::subscriptInt(&s2)?;
                res = if (i1 < i2) {
                    -1
                } else if (i1 > i2) {
                    1
                } else {
                    0
                };
            }
            if res != 0 {
                return Ok(res);
            }
        }
        if !((ss).is_empty()) {
            res = 1;
        }
        Ok(res)
    }

    pub(crate) fn compareSubscriptStr(
        mut s1: &metamodelica::Ref<DAE::Subscript>,
        mut s2: &metamodelica::Ref<DAE::Subscript>,
    ) -> Result<i32> {
        let mut res: i32;
        res = (::match_deref::match_deref! { match (s1, s2) {
            (Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i1 } }, Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i2 } }) => {
                if (i1.clone() == i2.clone()) {0} else {stringCompare(&(intString(i1.clone())), &(intString(i2.clone())))}
            },
            _ => {
                if (referenceEq(&*(&**s1),&*(&**s2))) {0} else {stringCompare(&(ExpressionBasics::printSubscriptStr(s1)?), &(ExpressionBasics::printSubscriptStr(s2)?))}
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(res)
    }
}

pub mod CompareWithGenericSubscriptNotAlphabetic {
    use super::*;
    pub(crate) fn compare<'__b>(
        mut cr1: &'__b metamodelica::Ref<DAE::ComponentRef>,
        mut cr2: &'__b metamodelica::Ref<DAE::ComponentRef>,
    ) -> Result<i32> {
        let mut res: i32;
        res = (::match_deref::match_deref! { match (cr1, cr2) {
            (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => {
                res = stringCompare(&var_field!((**cr1).ident, DAE::ComponentRef::CREF_IDENT), &var_field!((**cr2).ident, DAE::ComponentRef::CREF_IDENT));
                if compareSubscript.clone() == CompareWithSubsType::WithoutSubscripts.clone() || res != 0 {
                    return Ok(res);
                }
                compareSubs(var_field!((**cr1).subscriptLst, DAE::ComponentRef::CREF_IDENT), var_field!((**cr2).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?
            },
            (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => {
                res = stringCompare(&var_field!((**cr1).ident, DAE::ComponentRef::CREF_QUAL), &var_field!((**cr2).ident, DAE::ComponentRef::CREF_QUAL));
                if res != 0 {
                    return Ok(res);
                }
                if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                    res = compareSubs(var_field!((**cr1).subscriptLst, DAE::ComponentRef::CREF_QUAL), var_field!((**cr2).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())?;
                    if res != 0 {
                        return Ok(res);
                    }
                }
                compare(var_field!((**cr1).componentRef, DAE::ComponentRef::CREF_QUAL), var_field!((**cr2).componentRef, DAE::ComponentRef::CREF_QUAL))?
            },
            (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => {
                res = stringCompare(&var_field!((**cr1).ident, DAE::ComponentRef::CREF_QUAL), &var_field!((**cr2).ident, DAE::ComponentRef::CREF_IDENT));
                if res != 0 {
                    return Ok(res);
                }
                if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                    res = compareSubs(var_field!((**cr1).subscriptLst, DAE::ComponentRef::CREF_QUAL), var_field!((**cr2).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?;
                }
                if res != 0 {
                    return Ok(res);
                }
                1
            },
            (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => {
                res = stringCompare(&var_field!((**cr1).ident, DAE::ComponentRef::CREF_IDENT), &var_field!((**cr2).ident, DAE::ComponentRef::CREF_QUAL));
                if res != 0 {
                    return Ok(res);
                }
                if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                    res = compareSubs(var_field!((**cr1).subscriptLst, DAE::ComponentRef::CREF_IDENT), var_field!((**cr2).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())?;
                }
                if res != 0 {
                    return Ok(res);
                }
                -1
            },
            _ => return Err("match: no arm matched"),
        } });
        Ok(res)
    }

    pub(crate) fn compareSubs(
        mut ss1: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
        mut ss2: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    ) -> Result<i32> {
        let mut res: i32 = 0;
        let mut ss: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = ss2;
        let mut s2: metamodelica::Ref<DAE::Subscript>;
        let mut i1: i32;
        let mut i2: i32;
        for mut s1 in &**ss1 {
            if (ss).is_empty() {
                res = -1;
                return Ok(res);
            }
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ss) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            s2 = metamodelica::Own::own(__pa0);
            ss = metamodelica::Own::own(__pa1);
            if compareSubscript.clone() == CompareWithSubsType::WithGenericSubscript.clone() {
                res = compareSubscriptStr(metamodelica::AsArg::as_arg(&s1), &s2)?;
            } else if compareSubscript.clone() == CompareWithSubsType::WithGenericSubscriptNotAlphabetic.clone() {
                res = ExpressionBasics::compareSubscripts(metamodelica::AsArg::as_arg(&s1), &s2)?;
            } else {
                i1 = ExpressionBasics::subscriptInt(metamodelica::AsArg::as_arg(&s1))?;
                i2 = ExpressionBasics::subscriptInt(&s2)?;
                res = if (i1 < i2) {
                    -1
                } else if (i1 > i2) {
                    1
                } else {
                    0
                };
            }
            if res != 0 {
                return Ok(res);
            }
        }
        if !((ss).is_empty()) {
            res = 1;
        }
        Ok(res)
    }

    pub(crate) static compareSubscript: std::sync::LazyLock<CompareWithSubsType> =
        std::sync::LazyLock::new(|| CompareWithSubsType::WithGenericSubscriptNotAlphabetic.clone());

    pub(crate) fn compareSubscriptStr(
        mut s1: &metamodelica::Ref<DAE::Subscript>,
        mut s2: &metamodelica::Ref<DAE::Subscript>,
    ) -> Result<i32> {
        let mut res: i32;
        res = (::match_deref::match_deref! { match (s1, s2) {
            (Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i1 } }, Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i2 } }) => {
                if (i1.clone() == i2.clone()) {0} else {stringCompare(&(intString(i1.clone())), &(intString(i2.clone())))}
            },
            _ => {
                if (referenceEq(&*(&**s1),&*(&**s2))) {0} else {stringCompare(&(ExpressionBasics::printSubscriptStr(s1)?), &(ExpressionBasics::printSubscriptStr(s2)?))}
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(res)
    }
}

pub mod CompareWithoutSubscripts {
    use super::*;
    pub(crate) fn compare<'__b>(
        mut cr1: &'__b metamodelica::Ref<DAE::ComponentRef>,
        mut cr2: &'__b metamodelica::Ref<DAE::ComponentRef>,
    ) -> Result<i32> {
        let mut res: i32;
        res = (::match_deref::match_deref! { match (cr1, cr2) {
            (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => {
                res = stringCompare(&var_field!((**cr1).ident, DAE::ComponentRef::CREF_IDENT), &var_field!((**cr2).ident, DAE::ComponentRef::CREF_IDENT));
                if compareSubscript.clone() == CompareWithSubsType::WithoutSubscripts.clone() || res != 0 {
                    return Ok(res);
                }
                compareSubs(var_field!((**cr1).subscriptLst, DAE::ComponentRef::CREF_IDENT), var_field!((**cr2).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?
            },
            (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => {
                res = stringCompare(&var_field!((**cr1).ident, DAE::ComponentRef::CREF_QUAL), &var_field!((**cr2).ident, DAE::ComponentRef::CREF_QUAL));
                if res != 0 {
                    return Ok(res);
                }
                if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                    res = compareSubs(var_field!((**cr1).subscriptLst, DAE::ComponentRef::CREF_QUAL), var_field!((**cr2).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())?;
                    if res != 0 {
                        return Ok(res);
                    }
                }
                compare(var_field!((**cr1).componentRef, DAE::ComponentRef::CREF_QUAL), var_field!((**cr2).componentRef, DAE::ComponentRef::CREF_QUAL))?
            },
            (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => {
                res = stringCompare(&var_field!((**cr1).ident, DAE::ComponentRef::CREF_QUAL), &var_field!((**cr2).ident, DAE::ComponentRef::CREF_IDENT));
                if res != 0 {
                    return Ok(res);
                }
                if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                    res = compareSubs(var_field!((**cr1).subscriptLst, DAE::ComponentRef::CREF_QUAL), var_field!((**cr2).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?;
                }
                if res != 0 {
                    return Ok(res);
                }
                1
            },
            (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => {
                res = stringCompare(&var_field!((**cr1).ident, DAE::ComponentRef::CREF_IDENT), &var_field!((**cr2).ident, DAE::ComponentRef::CREF_QUAL));
                if res != 0 {
                    return Ok(res);
                }
                if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                    res = compareSubs(var_field!((**cr1).subscriptLst, DAE::ComponentRef::CREF_IDENT), var_field!((**cr2).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())?;
                }
                if res != 0 {
                    return Ok(res);
                }
                -1
            },
            _ => return Err("match: no arm matched"),
        } });
        Ok(res)
    }

    pub(crate) fn compareSubs(
        mut ss1: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
        mut ss2: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    ) -> Result<i32> {
        let mut res: i32 = 0;
        let mut ss: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = ss2;
        let mut s2: metamodelica::Ref<DAE::Subscript>;
        let mut i1: i32;
        let mut i2: i32;
        for mut s1 in &**ss1 {
            if (ss).is_empty() {
                res = -1;
                return Ok(res);
            }
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ss) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            s2 = metamodelica::Own::own(__pa0);
            ss = metamodelica::Own::own(__pa1);
            if compareSubscript.clone() == CompareWithSubsType::WithGenericSubscript.clone() {
                res = compareSubscriptStr(metamodelica::AsArg::as_arg(&s1), &s2)?;
            } else if compareSubscript.clone() == CompareWithSubsType::WithGenericSubscriptNotAlphabetic.clone() {
                res = ExpressionBasics::compareSubscripts(metamodelica::AsArg::as_arg(&s1), &s2)?;
            } else {
                i1 = ExpressionBasics::subscriptInt(metamodelica::AsArg::as_arg(&s1))?;
                i2 = ExpressionBasics::subscriptInt(&s2)?;
                res = if (i1 < i2) {
                    -1
                } else if (i1 > i2) {
                    1
                } else {
                    0
                };
            }
            if res != 0 {
                return Ok(res);
            }
        }
        if !((ss).is_empty()) {
            res = 1;
        }
        Ok(res)
    }

    pub(crate) static compareSubscript: std::sync::LazyLock<CompareWithSubsType> =
        std::sync::LazyLock::new(|| CompareWithSubsType::WithoutSubscripts.clone());

    pub(crate) fn compareSubscriptStr(
        mut s1: &metamodelica::Ref<DAE::Subscript>,
        mut s2: &metamodelica::Ref<DAE::Subscript>,
    ) -> Result<i32> {
        let mut res: i32;
        res = (::match_deref::match_deref! { match (s1, s2) {
            (Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i1 } }, Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i2 } }) => {
                if (i1.clone() == i2.clone()) {0} else {stringCompare(&(intString(i1.clone())), &(intString(i2.clone())))}
            },
            _ => {
                if (referenceEq(&*(&**s1),&*(&**s2))) {0} else {stringCompare(&(ExpressionBasics::printSubscriptStr(s1)?), &(ExpressionBasics::printSubscriptStr(s2)?))}
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(res)
    }
}

pub mod CompareWithIntSubscript {
    use super::*;
    pub(crate) fn compare<'__b>(
        mut cr1: &'__b metamodelica::Ref<DAE::ComponentRef>,
        mut cr2: &'__b metamodelica::Ref<DAE::ComponentRef>,
    ) -> Result<i32> {
        let mut res: i32;
        res = (::match_deref::match_deref! { match (cr1, cr2) {
            (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => {
                res = stringCompare(&var_field!((**cr1).ident, DAE::ComponentRef::CREF_IDENT), &var_field!((**cr2).ident, DAE::ComponentRef::CREF_IDENT));
                if compareSubscript.clone() == CompareWithSubsType::WithoutSubscripts.clone() || res != 0 {
                    return Ok(res);
                }
                compareSubs(var_field!((**cr1).subscriptLst, DAE::ComponentRef::CREF_IDENT), var_field!((**cr2).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?
            },
            (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => {
                res = stringCompare(&var_field!((**cr1).ident, DAE::ComponentRef::CREF_QUAL), &var_field!((**cr2).ident, DAE::ComponentRef::CREF_QUAL));
                if res != 0 {
                    return Ok(res);
                }
                if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                    res = compareSubs(var_field!((**cr1).subscriptLst, DAE::ComponentRef::CREF_QUAL), var_field!((**cr2).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())?;
                    if res != 0 {
                        return Ok(res);
                    }
                }
                compare(var_field!((**cr1).componentRef, DAE::ComponentRef::CREF_QUAL), var_field!((**cr2).componentRef, DAE::ComponentRef::CREF_QUAL))?
            },
            (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => {
                res = stringCompare(&var_field!((**cr1).ident, DAE::ComponentRef::CREF_QUAL), &var_field!((**cr2).ident, DAE::ComponentRef::CREF_IDENT));
                if res != 0 {
                    return Ok(res);
                }
                if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                    res = compareSubs(var_field!((**cr1).subscriptLst, DAE::ComponentRef::CREF_QUAL), var_field!((**cr2).subscriptLst, DAE::ComponentRef::CREF_IDENT).clone())?;
                }
                if res != 0 {
                    return Ok(res);
                }
                1
            },
            (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => {
                res = stringCompare(&var_field!((**cr1).ident, DAE::ComponentRef::CREF_IDENT), &var_field!((**cr2).ident, DAE::ComponentRef::CREF_QUAL));
                if res != 0 {
                    return Ok(res);
                }
                if compareSubscript.clone() != CompareWithSubsType::WithoutSubscripts.clone() {
                    res = compareSubs(var_field!((**cr1).subscriptLst, DAE::ComponentRef::CREF_IDENT), var_field!((**cr2).subscriptLst, DAE::ComponentRef::CREF_QUAL).clone())?;
                }
                if res != 0 {
                    return Ok(res);
                }
                -1
            },
            _ => return Err("match: no arm matched"),
        } });
        Ok(res)
    }

    pub(crate) fn compareSubs(
        mut ss1: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
        mut ss2: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    ) -> Result<i32> {
        let mut res: i32 = 0;
        let mut ss: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = ss2;
        let mut s2: metamodelica::Ref<DAE::Subscript>;
        let mut i1: i32;
        let mut i2: i32;
        for mut s1 in &**ss1 {
            if (ss).is_empty() {
                res = -1;
                return Ok(res);
            }
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ss) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            s2 = metamodelica::Own::own(__pa0);
            ss = metamodelica::Own::own(__pa1);
            if compareSubscript.clone() == CompareWithSubsType::WithGenericSubscript.clone() {
                res = compareSubscriptStr(metamodelica::AsArg::as_arg(&s1), &s2)?;
            } else if compareSubscript.clone() == CompareWithSubsType::WithGenericSubscriptNotAlphabetic.clone() {
                res = ExpressionBasics::compareSubscripts(metamodelica::AsArg::as_arg(&s1), &s2)?;
            } else {
                i1 = ExpressionBasics::subscriptInt(metamodelica::AsArg::as_arg(&s1))?;
                i2 = ExpressionBasics::subscriptInt(&s2)?;
                res = if (i1 < i2) {
                    -1
                } else if (i1 > i2) {
                    1
                } else {
                    0
                };
            }
            if res != 0 {
                return Ok(res);
            }
        }
        if !((ss).is_empty()) {
            res = 1;
        }
        Ok(res)
    }

    pub(crate) static compareSubscript: std::sync::LazyLock<CompareWithSubsType> =
        std::sync::LazyLock::new(|| CompareWithSubsType::WithIntSubscript.clone());

    pub(crate) fn compareSubscriptStr(
        mut s1: &metamodelica::Ref<DAE::Subscript>,
        mut s2: &metamodelica::Ref<DAE::Subscript>,
    ) -> Result<i32> {
        let mut res: i32;
        res = (::match_deref::match_deref! { match (s1, s2) {
            (Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i1 } }, Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i2 } }) => {
                if (i1.clone() == i2.clone()) {0} else {stringCompare(&(intString(i1.clone())), &(intString(i2.clone())))}
            },
            _ => {
                if (referenceEq(&*(&**s1),&*(&**s2))) {0} else {stringCompare(&(ExpressionBasics::printSubscriptStr(s1)?), &(ExpressionBasics::printSubscriptStr(s2)?))}
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(res)
    }
}

pub fn crefSortFunc(
    mut cr1: &metamodelica::Ref<DAE::ComponentRef>,
    mut cr2: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut greaterThan: bool;
    greaterThan = CompareWithGenericSubscript::compare(cr1, cr2)? > 0;
    Ok(greaterThan)
}

pub fn crefCompareGeneric(
    mut cr1: &metamodelica::Ref<DAE::ComponentRef>,
    mut cr2: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<i32> {
    let mut comp: i32;
    comp = CompareWithGenericSubscript::compare(cr1, cr2)?;
    Ok(comp)
}

pub fn crefCompareIntSubscript(
    mut cr1: &metamodelica::Ref<DAE::ComponentRef>,
    mut cr2: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<i32> {
    let mut comp: i32;
    comp = CompareWithIntSubscript::compare(cr1, cr2)?;
    Ok(comp)
}

pub(crate) fn crefCompareGenericNotAlphabetic(
    mut cr1: &metamodelica::Ref<DAE::ComponentRef>,
    mut cr2: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<i32> {
    let mut comp: i32;
    comp = CompareWithGenericSubscriptNotAlphabetic::compare(cr1, cr2)?;
    Ok(comp)
}

pub fn crefLexicalGreaterSubsAtEnd(
    mut cr1: &metamodelica::Ref<DAE::ComponentRef>,
    mut cr2: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut isGreater: bool;
    isGreater = crefLexicalCompareSubsAtEnd(cr1, cr2)? > 0;
    Ok(isGreater)
}

pub(crate) fn crefLexicalCompareSubsAtEnd(
    mut cr1: &metamodelica::Ref<DAE::ComponentRef>,
    mut cr2: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<i32> {
    let mut res: i32;
    let mut subs1: metamodelica::List<i32>;
    let mut subs2: metamodelica::List<i32>;
    res = CompareWithoutSubscripts::compare(cr1, cr2)?;
    if res != 0 {
        return Ok(res);
    }
    subs1 = ExpressionBasics::subscriptsInt(crefSubs(cr1)?)?;
    subs2 = ExpressionBasics::subscriptsInt(crefSubs(cr2)?)?;
    res = crefLexicalCompareSubsAtEnd2(&subs1, subs2)?;
    Ok(res)
}

fn crefLexicalCompareSubsAtEnd2(
    mut inSubs1: &metamodelica::List<i32>,
    mut inSubs2: metamodelica::List<i32>,
) -> Result<i32> {
    let mut res: i32 = 0;
    let mut rest: metamodelica::List<i32> = inSubs2;
    for mut i in &**inSubs1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        res = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        res = if (i.clone() > res) {
            1
        } else if (i.clone() < res) {
            -1
        } else {
            0
        };
        if res != 0 {
            return Ok(res);
        }
    }
    Ok(res)
}

pub(crate) fn crefContainedIn(
    mut containerCref: metamodelica::Ref<DAE::ComponentRef>,
    mut containedCref: metamodelica::Ref<DAE::ComponentRef>,
) -> bool {
    let mut outBoolean: bool;
    outBoolean = 'mc: {
        let __mc_input = (containerCref, containedCref);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (full, partOf) => {
                    let true = (crefEqualNoStringCompare(metamodelica::AsArg::as_arg(&full), metamodelica::AsArg::as_arg(&partOf))?) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (full @ Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: cr2, .. }, partOf) => {
                    let mut res: bool;
                    let false = (crefEqualNoStringCompare(metamodelica::AsArg::as_arg(&full), metamodelica::AsArg::as_arg(&partOf))?) else { return Err("pattern mismatch") };
                    res = crefContainedIn(cr2.clone(), partOf.clone());
                    Ok(res)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outBoolean
}

pub fn crefPrefixOf(
    mut prefixCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut fullCref: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut outPrefixOf: bool;
    outPrefixOf = (::match_deref::match_deref! { match (prefixCref, fullCref) {
        (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => metamodelica::stringEq(&var_field!((**prefixCref).ident, DAE::ComponentRef::CREF_QUAL), &var_field!((**fullCref).ident, DAE::ComponentRef::CREF_QUAL)) && ExpressionBasics::subscriptEqual(var_field!((**prefixCref).subscriptLst, DAE::ComponentRef::CREF_QUAL), var_field!((**fullCref).subscriptLst, DAE::ComponentRef::CREF_QUAL))? && crefPrefixOf(var_field!((**prefixCref).componentRef, DAE::ComponentRef::CREF_QUAL), var_field!((**fullCref).componentRef, DAE::ComponentRef::CREF_QUAL))?,
        (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::ListNode::Nil, .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => metamodelica::stringEq(&var_field!((**prefixCref).ident, DAE::ComponentRef::CREF_IDENT), &var_field!((**fullCref).ident, DAE::ComponentRef::CREF_QUAL)),
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => metamodelica::stringEq(&var_field!((**prefixCref).ident, DAE::ComponentRef::CREF_IDENT), &var_field!((**fullCref).ident, DAE::ComponentRef::CREF_QUAL)) && ExpressionBasics::subscriptEqual(var_field!((**prefixCref).subscriptLst, DAE::ComponentRef::CREF_IDENT), var_field!((**fullCref).subscriptLst, DAE::ComponentRef::CREF_QUAL))?,
        (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::ListNode::Nil, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => stringEq(&var_field!((**prefixCref).ident, DAE::ComponentRef::CREF_IDENT), &var_field!((**fullCref).ident, DAE::ComponentRef::CREF_IDENT)),
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => metamodelica::stringEq(&var_field!((**prefixCref).ident, DAE::ComponentRef::CREF_IDENT), &var_field!((**fullCref).ident, DAE::ComponentRef::CREF_IDENT)) && ExpressionBasics::subscriptEqual(var_field!((**prefixCref).subscriptLst, DAE::ComponentRef::CREF_IDENT), var_field!((**fullCref).subscriptLst, DAE::ComponentRef::CREF_IDENT))?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPrefixOf)
}

pub fn crefPrefixOfIgnoreSubscripts(
    mut prefixCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut fullCref: &metamodelica::Ref<DAE::ComponentRef>,
) -> bool {
    let mut outPrefixOf: bool;
    outPrefixOf = (::match_deref::match_deref! { match (prefixCref, fullCref) {
        (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => metamodelica::stringEq(&var_field!((**prefixCref).ident, DAE::ComponentRef::CREF_QUAL), &var_field!((**fullCref).ident, DAE::ComponentRef::CREF_QUAL)) && crefPrefixOfIgnoreSubscripts(var_field!((**prefixCref).componentRef, DAE::ComponentRef::CREF_QUAL), var_field!((**fullCref).componentRef, DAE::ComponentRef::CREF_QUAL)),
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => metamodelica::stringEq(&var_field!((**prefixCref).ident, DAE::ComponentRef::CREF_IDENT), &var_field!((**fullCref).ident, DAE::ComponentRef::CREF_QUAL)),
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => metamodelica::stringEq(&var_field!((**prefixCref).ident, DAE::ComponentRef::CREF_IDENT), &var_field!((**fullCref).ident, DAE::ComponentRef::CREF_IDENT)),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outPrefixOf
}

pub(crate) fn crefNotPrefixOf(
    mut cr1: &metamodelica::Ref<DAE::ComponentRef>,
    mut cr2: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match (cr1, cr2) {
        (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => true,
        _ => !(crefPrefixOf(cr1, cr2)?),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

pub fn crefEqual(
    mut inComponentRef1: &metamodelica::Ref<DAE::ComponentRef>,
    mut inComponentRef2: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = crefEqualNoStringCompare(inComponentRef1, inComponentRef2)?;
    Ok(outBoolean)
}

pub fn crefInLst(
    mut cref: metamodelica::Ref<DAE::ComponentRef>,
    mut lst: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<bool> {
    let mut b: bool;
    b = List::isMemberOnTrue(
        cref,
        lst,
        &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| {
            crefEqual(&__a0, &__a1)
        },
    )?;
    Ok(b)
}

pub fn crefNotInLst(
    mut cref: metamodelica::Ref<DAE::ComponentRef>,
    mut lst: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<bool> {
    let mut b: bool;
    b = !(List::isMemberOnTrue(
        cref,
        lst,
        &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| {
            crefEqual(&__a0, &__a1)
        },
    )?);
    Ok(b)
}

pub(crate) fn crefEqualVerySlowStringCompareDoNotUse(
    mut inComponentRef1: metamodelica::Ref<DAE::ComponentRef>,
    mut inComponentRef2: metamodelica::Ref<DAE::ComponentRef>,
) -> bool {
    let mut outBoolean: bool;
    outBoolean = 'mc: {
        let __mc_input = (inComponentRef1.clone(), inComponentRef2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (referenceEq(&*(&*inComponentRef1),&*(&*inComponentRef2))) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: n1, subscriptLst: Deref @ metamodelica::ListNode::Nil, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: n2, subscriptLst: Deref @ metamodelica::ListNode::Nil, .. }) => {
                    let true = (stringEq(&n1, &n2)) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: n1, subscriptLst: idx1 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: n2, subscriptLst: idx2 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }) => {
                    let true = (stringEq(&n1, &n2)) else { return Err("pattern mismatch") };
                    let true = (ExpressionBasics::subscriptEqual(metamodelica::AsArg::as_arg(&idx1), metamodelica::AsArg::as_arg(&idx2))?) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: n1, subscriptLst: Deref @ metamodelica::ListNode::Nil, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: n2, subscriptLst: idx2 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }) => {
                    let mut s1: ArcStr;
                    let 0 = (System::stringFind(n1.clone(), n2.clone())?) else { return Err("pattern mismatch") };
                    s1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*n2); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*ExpressionBasics::printListStr(idx2.clone(), &move |__a0: metamodelica::Ref<DAE::Subscript>| ExpressionBasics::printSubscriptStr(&__a0), literal!(","))?); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) };
                    let true = (stringEq(&s1, &n1)) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_IDENT { ident: n1, subscriptLst: idx2 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: n2, subscriptLst: Deref @ metamodelica::ListNode::Nil, .. }) => {
                    let mut s1: ArcStr;
                    let 0 = (System::stringFind(n2.clone(), n1.clone())?) else { return Err("pattern mismatch") };
                    s1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*n1); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*ExpressionBasics::printListStr(idx2.clone(), &move |__a0: metamodelica::Ref<DAE::Subscript>| ExpressionBasics::printSubscriptStr(&__a0), literal!(","))?); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) };
                    let true = (stringEq(&s1, &n2)) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::ComponentRef::CREF_QUAL { ident: n1, subscriptLst: idx1, componentRef: cr1, .. }, Deref @ DAE::ComponentRef::CREF_QUAL { ident: n2, subscriptLst: idx2, componentRef: cr2, .. }) => {
                    let true = (stringEq(&n1, &n2)) else { return Err("pattern mismatch") };
                    let true = (crefEqualVerySlowStringCompareDoNotUse(cr1.clone(), cr2.clone())) else { return Err("pattern mismatch") };
                    let true = (ExpressionBasics::subscriptEqual(metamodelica::AsArg::as_arg(&idx1), metamodelica::AsArg::as_arg(&idx2))?) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr1 @ Deref @ DAE::ComponentRef::CREF_QUAL { ident: n1, .. }, cr2 @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: n2, .. }) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let 0 = (System::stringFind(n2.clone(), n1.clone())?) else { return Err("pattern mismatch") };
                    s1 = printComponentRefStr(metamodelica::AsArg::as_arg(&cr1))?;
                    s2 = printComponentRefStr(metamodelica::AsArg::as_arg(&cr2))?;
                    let true = (stringEq(&s1, &s2)) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr1 @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: n1, .. }, cr2 @ Deref @ DAE::ComponentRef::CREF_QUAL { ident: n2, .. }) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let 0 = (System::stringFind(n1.clone(), n2.clone())?) else { return Err("pattern mismatch") };
                    s1 = printComponentRefStr(metamodelica::AsArg::as_arg(&cr1))?;
                    s2 = printComponentRefStr(metamodelica::AsArg::as_arg(&cr2))?;
                    let true = (stringEq(&s1, &s2)) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outBoolean
}

pub fn crefEqualNoStringCompare(
    mut inCref1: &metamodelica::Ref<DAE::ComponentRef>,
    mut inCref2: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut outEqual: bool;
    if referenceEq(&*(&**inCref1), &*(&**inCref2)) {
        outEqual = true;
        return Ok(outEqual);
    }
    outEqual = (::match_deref::match_deref! { match (inCref1, inCref2) {
        (Deref @ DAE::ComponentRef::CREF_IDENT { .. }, Deref @ DAE::ComponentRef::CREF_IDENT { .. }) => metamodelica::stringEq(&var_field!((**inCref1).ident, DAE::ComponentRef::CREF_IDENT), &var_field!((**inCref2).ident, DAE::ComponentRef::CREF_IDENT)) && ExpressionBasics::subscriptEqual(var_field!((**inCref1).subscriptLst, DAE::ComponentRef::CREF_IDENT), var_field!((**inCref2).subscriptLst, DAE::ComponentRef::CREF_IDENT))?,
        (Deref @ DAE::ComponentRef::CREF_QUAL { .. }, Deref @ DAE::ComponentRef::CREF_QUAL { .. }) => metamodelica::stringEq(&var_field!((**inCref1).ident, DAE::ComponentRef::CREF_QUAL), &var_field!((**inCref2).ident, DAE::ComponentRef::CREF_QUAL)) && crefEqualNoStringCompare(var_field!((**inCref1).componentRef, DAE::ComponentRef::CREF_QUAL), var_field!((**inCref2).componentRef, DAE::ComponentRef::CREF_QUAL))? && ExpressionBasics::subscriptEqual(var_field!((**inCref1).subscriptLst, DAE::ComponentRef::CREF_QUAL), var_field!((**inCref2).subscriptLst, DAE::ComponentRef::CREF_QUAL))?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqual)
}

pub(crate) fn crefEqualReturn(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut cr2: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut ocr: metamodelica::Ref<DAE::ComponentRef>;
    let true = (crefEqualNoStringCompare(&cr, cr2)?) else {
        return Err("pattern mismatch");
    };
    ocr = cr;
    Ok(ocr)
}

pub fn crefEqualWithoutLastSubs(
    mut cr1: &metamodelica::Ref<DAE::ComponentRef>,
    mut cr2: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut res: bool;
    res = crefEqualNoStringCompare(&(crefStripLastSubs(cr1)?), &(crefStripLastSubs(cr2)?))?;
    Ok(res)
}

pub fn crefEqualWithoutSubs(
    mut cr1: metamodelica::Ref<DAE::ComponentRef>,
    mut cr2: metamodelica::Ref<DAE::ComponentRef>,
) -> bool {
    let mut res: bool;
    res = crefEqualWithoutSubs2(referenceEq(&*(&*cr1), &*(&*cr2)), cr1, cr2);
    res
}

fn crefEqualWithoutSubs2(
    mut refEq: bool,
    mut icr1: metamodelica::Ref<DAE::ComponentRef>,
    mut icr2: metamodelica::Ref<DAE::ComponentRef>,
) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &((refEq, icr1, icr2)) {
            (true, _, _) => {
                return true
            },
            (_, Deref @ DAE::ComponentRef::CREF_IDENT { ident: n1, .. }, Deref @ DAE::ComponentRef::CREF_IDENT { ident: n2, .. }) => {
                return stringEq(&n1, &n2)
            },
            (_, Deref @ DAE::ComponentRef::CREF_QUAL { ident: n1, componentRef: cr1, .. }, Deref @ DAE::ComponentRef::CREF_QUAL { ident: n2, componentRef: cr2, .. }) => {
                let mut r: bool;
                r = stringEq(&n1, &n2);
                if (r) {{ (refEq, icr1, icr2) = (referenceEq(&*(cr1.clone()),&*(cr2.clone())), cr1.clone(), cr2.clone()); continue '__tco; }} else {return false}
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn crefStripLastSubs(
    mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    outComponentRef = (match &**inComponentRef {
        DAE::ComponentRef::CREF_IDENT {
            ident: id,
            identType: t2,
            ..
        } => makeCrefIdent(id.clone(), t2.clone(), metamodelica::nil()),
        DAE::ComponentRef::CREF_QUAL {
            ident: id,
            identType: t2,
            subscriptLst: s,
            componentRef: cr,
        } => {
            let mut cr_1: metamodelica::Ref<DAE::ComponentRef>;
            cr_1 = crefStripLastSubs(cr)?;
            makeCrefQual(id.clone(), t2.clone(), s.clone(), cr_1)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outComponentRef)
}

pub fn makeCrefIdent(
    mut ident: ArcStr,
    mut identType: metamodelica::Ref<DAE::Type>,
    mut subscriptLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outCrefIdent: metamodelica::Ref<DAE::ComponentRef>;
    outCrefIdent = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
        ident: ident,
        identType: identType,
        subscriptLst: subscriptLst,
    });
    outCrefIdent
}

pub fn makeCrefQual(
    mut ident: ArcStr,
    mut identType: metamodelica::Ref<DAE::Type>,
    mut subscriptLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut componentRef: metamodelica::Ref<DAE::ComponentRef>,
) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outCrefQual: metamodelica::Ref<DAE::ComponentRef>;
    outCrefQual = metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
        ident: ident,
        identType: identType,
        subscriptLst: subscriptLst,
        componentRef: componentRef,
    });
    outCrefQual
}

pub fn printComponentRefStr(mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inComponentRef {
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: s, subscriptLst: Deref @ metamodelica::ListNode::Nil, .. } => {
            s.clone()
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: s, subscriptLst: subs, .. } => {
            let mut r#str: ArcStr;
            r#str = printComponentRef2Str(s.clone(), subs.clone())?;
            r#str
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: s, subscriptLst: subs, componentRef: cr, .. } => {
            let mut r#str: ArcStr;
            let mut strrest: ArcStr;
            let mut strseb: ArcStr;
            let mut b: bool;
            b = Config::modelicaOutput()?;
            r#str = printComponentRef2Str(s.clone(), subs.clone())?;
            strrest = printComponentRefStr(cr)?;
            strseb = if (b) {literal!("__")} else {literal!(".")};
            r#str = stringAppendList(list![r#str, strseb, strrest]);
            r#str
        },
        Deref @ DAE::ComponentRef::WILD { .. } => {
            literal!("_")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

pub fn printComponentRef2Str(
    mut inIdent: ArcStr,
    mut inSubscriptLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inSubscriptLst) {
        Deref @ metamodelica::ListNode::Nil => {
            let mut s = inIdent;
            s
        },
        l => {
            let mut s = inIdent;
            let mut r#str: ArcStr;
            let mut strseba: ArcStr;
            let mut strsebb: ArcStr;
            let mut b: bool;
            b = Config::modelicaOutput()?;
            r#str = ExpressionBasics::printListStr(l.clone(), &move |__a0: metamodelica::Ref<DAE::Subscript>| ExpressionBasics::printSubscriptStr(&__a0), literal!(","))?;
            (strseba, strsebb) = if (b) {(literal!("_L"), literal!("_R"))} else {(literal!("["), literal!("]"))};
            r#str = stringAppendList(list![s, strseba, r#str, strsebb]);
            r#str
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

pub fn printComponentRefListStr(mut crs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>) -> Result<ArcStr> {
    let mut res: ArcStr;
    res = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("{"));
        __mm_s.push_str(&*stringDelimitList(
            List::map(crs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
                printComponentRefStr(&__a0)
            })?,
            literal!(","),
        ));
        __mm_s.push_str(&*literal!("}"));
        ArcStr::from(__mm_s)
    };
    Ok(res)
}

pub const crefHashSeed: i32 = 5381;

pub fn hashComponentRef(mut cr: &metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> {
    let mut hash: i32 = hashComponentRefFrom(cr, crefHashSeed.clone())?;
    Ok(hash)
}

fn hashComponentRefFrom<'__b>(mut cr: &'__b metamodelica::Ref<DAE::ComponentRef>, mut hash: i32) -> Result<i32> {
    '__tco: loop {
        match &**cr {
            DAE::ComponentRef::CREF_IDENT { .. } => {
                return Ok(crefHashSubscripts(
                    var_field!((**cr).subscriptLst, DAE::ComponentRef::CREF_IDENT),
                    crefHashIdent(var_field!((**cr).ident, DAE::ComponentRef::CREF_IDENT), hash),
                )?);
            }
            DAE::ComponentRef::CREF_QUAL { .. } => {
                (cr, hash) = (
                    var_field!((**cr).componentRef, DAE::ComponentRef::CREF_QUAL),
                    crefHashSubscripts(
                        var_field!((**cr).subscriptLst, DAE::ComponentRef::CREF_QUAL),
                        crefHashIdent(var_field!((**cr).ident, DAE::ComponentRef::CREF_QUAL), hash),
                    )?,
                );
                continue '__tco;
            }
            _ => return Ok(hash),
        }
    }
}

pub fn crefHashIdent(mut ident: &ArcStr, mut hash: i32) -> i32 {
    let mut outHash: i32 = stringHashDjb2Continue(&ident, stringHashDjb2Continue(&(literal!(".")), hash));
    outHash
}

fn crefHashSubscripts(mut subs: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>, mut hash: i32) -> Result<i32> {
    let mut hash: i32 = hash;
    for mut sub in &**subs {
        hash = crefHashSubscript(metamodelica::AsArg::as_arg(&sub), hash)?;
    }
    Ok(hash)
}

pub fn crefHashSubscript(mut sub: &metamodelica::Ref<DAE::Subscript>, mut hash: i32) -> Result<i32> {
    let mut outHash: i32 = intHashDjb2Continue(hashSubscript(sub)?, stringHashDjb2Continue(&(literal!("[")), hash));
    Ok(outHash)
}

pub(crate) fn hashSubscript(mut sub: &metamodelica::Ref<DAE::Subscript>) -> Result<i32> {
    let mut hash: i32;
    hash = (::match_deref::match_deref! { match sub {
        Deref @ DAE::Subscript::WHOLEDIM { .. } => {
            0
        },
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i } } => {
            i.clone()
        },
        Deref @ DAE::Subscript::SLICE { exp } => {
            ExpressionBasics::hashExp(exp)?
        },
        Deref @ DAE::Subscript::INDEX { exp } => {
            ExpressionBasics::hashExp(exp)?
        },
        Deref @ DAE::Subscript::WHOLE_NONEXP { exp } => {
            ExpressionBasics::hashExp(exp)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(hash)
}
