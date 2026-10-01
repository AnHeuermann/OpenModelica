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

use crate::AbsynUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub static dummyInfo: SourceInfo = SourceInfo {
    fileName: literal!(""),
    isReadOnly: false,
    lineNumberStart: 0,
    columnNumberStart: 0,
    lineNumberEnd: 0,
    columnNumberEnd: 0,
    lastModification: metamodelica::OrderedFloat(0.0_f64),
};

pub fn stripSubmod(mut r#mod: metamodelica::Ref<SCode::Mod>) -> metamodelica::Ref<SCode::Mod> {
    let mut r#mod: metamodelica::Ref<SCode::Mod> = r#mod;
    let () = (match &*r#mod {
        SCode::Mod::MOD { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = metamodelica::nil());
            ()
        }
        _ => (),
    });
    r#mod
}

pub fn filterSubMods(
    mut r#mod: metamodelica::Ref<SCode::Mod>,
    mut filter: &dyn ::std::ops::Fn(metamodelica::Ref<SCode::SubMod>) -> Result<bool>,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    pub type FilterFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::SubMod>) -> Result<bool> + 'static>;

    let mut r#mod: metamodelica::Ref<SCode::Mod> = r#mod;
    r#mod = (match &*r#mod {
        SCode::Mod::MOD {
            subModLst: __mod_subModLst,
            ..
        } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::SubMod>> = metamodelica::nil();
                for mut m in (__mod_subModLst.clone()).into_iter().cloned() {
                    if !(filter(m.clone())?) { continue; }
                    let __x = m.clone();
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            (::match_deref::match_deref! { match &(r#mod.clone()) {
                Deref @ SCode::Mod::MOD { subModLst: Deref @ metamodelica::ListNode::Nil, binding: None, .. } => openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
                _ => r#mod,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        }
        _ => r#mod,
    });
    Ok(r#mod)
}

pub fn filterGivenSubModNames(
    mut submod: &metamodelica::Ref<SCode::SubMod>,
    mut namesToKeep: metamodelica::List<ArcStr>,
) -> bool {
    let mut keep: bool;
    keep = listMember(submod.ident.clone(), namesToKeep);
    keep
}

pub fn removeGivenSubModNames(
    mut submod: &metamodelica::Ref<SCode::SubMod>,
    mut namesToRemove: metamodelica::List<ArcStr>,
) -> bool {
    let mut keep: bool;
    keep = !(listMember(submod.ident.clone(), namesToRemove));
    keep
}

pub fn getElementNamed(
    mut inIdent: ArcStr,
    mut inClass: &metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut outElement: metamodelica::Ref<SCode::Element>;
    outElement = (::match_deref::match_deref! { match inClass {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. } => {
            let mut id = inIdent;
            let mut elt: metamodelica::Ref<SCode::Element>;
            elt = getElementNamedFromElts(id, metamodelica::AsArg::as_arg(&elts))?;
            elt
        },
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { composition: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. }, .. } => {
            let mut id = inIdent;
            let mut elt: metamodelica::Ref<SCode::Element>;
            elt = getElementNamedFromElts(id, metamodelica::AsArg::as_arg(&elts))?;
            elt
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outElement)
}

pub(crate) fn getElementNamedFromElts(
    mut inIdent: ArcStr,
    mut inElementLst: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut outElement: metamodelica::Ref<SCode::Element>;
    outElement = 'mc: {
        let __mc_input = (inIdent, &**inElementLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id2, Deref @ metamodelica::ListNode::Cons { head: comp @ Deref @ SCode::Element::COMPONENT { name: id1, .. }, tail: _ }) => {
                    let true = (stringEq(&id1, &id2)) else { return Err("pattern mismatch") };
                    Ok(comp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id2, Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::COMPONENT { name: id1, .. }, tail: xs }) => {
                    let mut elt: metamodelica::Ref<SCode::Element>;
                    let false = (stringEq(&id1, &id2)) else { return Err("pattern mismatch") };
                    elt = getElementNamedFromElts(id2.clone(), metamodelica::AsArg::as_arg(&xs))?;
                    Ok(elt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id2, Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::CLASS { name: id1, .. }, tail: xs }) => {
                    let mut elt: metamodelica::Ref<SCode::Element>;
                    let false = (stringEq(&id1, &id2)) else { return Err("pattern mismatch") };
                    elt = getElementNamedFromElts(id2.clone(), metamodelica::AsArg::as_arg(&xs))?;
                    Ok(elt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id2, Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::EXTENDS { .. }, tail: xs }) => {
                    let mut elt: metamodelica::Ref<SCode::Element>;
                    elt = getElementNamedFromElts(id2.clone(), metamodelica::AsArg::as_arg(&xs))?;
                    Ok(elt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id2, Deref @ metamodelica::ListNode::Cons { head: cdef @ Deref @ SCode::Element::CLASS { name: id1, .. }, tail: _ }) => {
                    let true = (stringEq(&id1, &id2)) else { return Err("pattern mismatch") };
                    Ok(cdef.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (id2, Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }) => {
                    let mut elt: metamodelica::Ref<SCode::Element>;
                    elt = getElementNamedFromElts(id2.clone(), metamodelica::AsArg::as_arg(&xs))?;
                    Ok(elt.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outElement)
}

pub(crate) fn isElementExtends(mut ele: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut isExtend: bool;
    isExtend = (match &**ele {
        SCode::Element::EXTENDS { .. } => true,
        _ => false,
    });
    isExtend
}

pub fn isElementExtendsOrClassExtends(mut ele: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut isExtend: bool;
    isExtend = (match &**ele {
        SCode::Element::EXTENDS { .. } => true,
        _ => false,
    });
    isExtend
}

pub(crate) fn isNotElementClassExtends(mut ele: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut isExtend: bool;
    isExtend = (::match_deref::match_deref! { match ele {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isExtend
}

pub fn isParameterOrConst(mut inVariability: SCode::Variability) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match inVariability {
        SCode::Variability::PARAM { .. } => true,
        SCode::Variability::CONST { .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isConstant(mut inVariability: SCode::Variability) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match inVariability {
        SCode::Variability::CONST { .. } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn countParts(mut inClass: &metamodelica::Ref<SCode::Element>) -> i32 {
    let mut outInteger: i32;
    outInteger = (::match_deref::match_deref! { match inClass {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. } => {
            let mut res: i32;
            res = ((elts).len() as i32);
            res
        },
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { composition: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. }, .. } => {
            let mut res: i32;
            res = ((elts).len() as i32);
            res
        },
        _ => {
            0
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outInteger
}

pub fn componentNames(mut inClass: &metamodelica::Ref<SCode::Element>) -> metamodelica::List<ArcStr> {
    let mut outStringLst: metamodelica::List<ArcStr>;
    outStringLst = (::match_deref::match_deref! { match inClass {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. } => {
            let mut res: metamodelica::List<ArcStr>;
            res = componentNamesFromElts(metamodelica::AsArg::as_arg(&elts));
            res
        },
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { composition: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. }, .. } => {
            let mut res: metamodelica::List<ArcStr>;
            res = componentNamesFromElts(metamodelica::AsArg::as_arg(&elts));
            res
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outStringLst
}

pub fn componentNamesFromElts(
    mut inElements: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> metamodelica::List<ArcStr> {
    let mut outComponentNames: metamodelica::List<ArcStr>;
    outComponentNames = List::filterMap(inElements, &move |__a0: metamodelica::Ref<SCode::Element>| {
        componentName(&__a0)
    });
    outComponentNames
}

pub(crate) fn componentName(mut inComponent: &metamodelica::Ref<SCode::Element>) -> Result<ArcStr> {
    let mut outName: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &((*inComponent)) {
        Deref @ SCode::Element::COMPONENT { name: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outName = metamodelica::Own::own(__pa0);
    Ok(outName)
}

pub fn elementInfo(mut e: &metamodelica::Ref<SCode::Element>) -> SourceInfo {
    let mut info: SourceInfo;
    info = (match &**e {
        SCode::Element::COMPONENT { info: i, .. } => i.clone(),
        SCode::Element::CLASS { info: i, .. } => i.clone(),
        SCode::Element::EXTENDS { info: i, .. } => i.clone(),
        SCode::Element::IMPORT { info: i, .. } => i.clone(),
        _ => Absyn::dummyInfo.clone(),
    });
    info
}

pub fn setElementName(mut e: metamodelica::Ref<SCode::Element>, mut name: ArcStr) -> metamodelica::Ref<SCode::Element> {
    let mut e: metamodelica::Ref<SCode::Element> = e;
    let () = (match &*e {
        SCode::Element::CLASS { .. } => {
            assign_variant_field!(e => SCode::Element::CLASS; name = name);
            ()
        }
        SCode::Element::COMPONENT { .. } => {
            assign_variant_field!(e => SCode::Element::COMPONENT; name = name);
            ()
        }
        SCode::Element::DEFINEUNIT { .. } => {
            assign_variant_field!(e => SCode::Element::DEFINEUNIT; name = name);
            ()
        }
        _ => (),
    });
    e
}

pub fn elementName(mut e: &metamodelica::Ref<SCode::Element>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = (match &**e {
        SCode::Element::COMPONENT { name: __esc_s, .. } => {
            s = (*__esc_s).clone();
            s.clone()
        }
        SCode::Element::CLASS { name: __esc_s, .. } => {
            s = (*__esc_s).clone();
            s.clone()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(s)
}

pub fn elementNameInfo(mut element: &metamodelica::Ref<SCode::Element>) -> Result<(ArcStr, SourceInfo)> {
    let mut name: ArcStr;
    let mut info: SourceInfo;
    (name, info) = (match &**element {
        SCode::Element::COMPONENT {
            name: __esc_name,
            info: __esc_info,
            ..
        } => {
            name = (*__esc_name).clone();
            info = (*__esc_info).clone();
            (name.clone(), info.clone())
        }
        SCode::Element::CLASS {
            name: __esc_name,
            info: __esc_info,
            ..
        } => {
            name = (*__esc_name).clone();
            info = (*__esc_info).clone();
            (name.clone(), info.clone())
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((name, info))
}

pub fn elementNames(
    mut elts: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut names: metamodelica::List<ArcStr>;
    names = List::fold(
        elts,
        &move |__a0: metamodelica::Ref<SCode::Element>, __a1: metamodelica::List<ArcStr>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(elementNamesWork(&__a0, __a1))
        },
        metamodelica::nil(),
    )?;
    Ok(names)
}

fn elementNamesWork(
    mut e: &metamodelica::Ref<SCode::Element>,
    mut acc: metamodelica::List<ArcStr>,
) -> metamodelica::List<ArcStr> {
    let mut out: metamodelica::List<ArcStr>;
    out = (match &**e {
        SCode::Element::COMPONENT { name: s, .. } => metamodelica::cons(s.clone(), acc),
        SCode::Element::CLASS { name: s, .. } => metamodelica::cons(s.clone(), acc),
        _ => acc,
    });
    out
}

pub(crate) fn renameElement(
    mut element: metamodelica::Ref<SCode::Element>,
    mut name: ArcStr,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut element: metamodelica::Ref<SCode::Element> = element;
    let () = (match &*element {
        SCode::Element::CLASS { .. } => {
            assign_variant_field!(element => SCode::Element::CLASS; name = name);
            ()
        }
        SCode::Element::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; name = name);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(element)
}

pub fn elementNameEqual(
    mut inElement1: &metamodelica::Ref<SCode::Element>,
    mut inElement2: &metamodelica::Ref<SCode::Element>,
) -> bool {
    let mut outEqual: bool;
    outEqual = (::match_deref::match_deref! { match (inElement1, inElement2) {
        (Deref @ SCode::Element::CLASS { .. }, Deref @ SCode::Element::CLASS { .. }) => metamodelica::stringEq(&var_field!((**inElement1).name, SCode::Element::CLASS), &var_field!((**inElement2).name, SCode::Element::CLASS)),
        (Deref @ SCode::Element::COMPONENT { .. }, Deref @ SCode::Element::COMPONENT { .. }) => metamodelica::stringEq(&var_field!((**inElement1).name, SCode::Element::COMPONENT), &var_field!((**inElement2).name, SCode::Element::COMPONENT)),
        (Deref @ SCode::Element::DEFINEUNIT { .. }, Deref @ SCode::Element::DEFINEUNIT { .. }) => metamodelica::stringEq(&var_field!((**inElement1).name, SCode::Element::DEFINEUNIT), &var_field!((**inElement2).name, SCode::Element::DEFINEUNIT)),
        (Deref @ SCode::Element::EXTENDS { .. }, Deref @ SCode::Element::EXTENDS { .. }) => AbsynUtil::pathEqual(var_field!((**inElement1).baseClassPath, SCode::Element::EXTENDS), var_field!((**inElement2).baseClassPath, SCode::Element::EXTENDS)),
        (Deref @ SCode::Element::IMPORT { .. }, Deref @ SCode::Element::IMPORT { .. }) => AbsynUtil::importEqual(var_field!((**inElement1).imp, SCode::Element::IMPORT), var_field!((**inElement2).imp, SCode::Element::IMPORT)),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outEqual
}

pub fn enumName(mut e: &metamodelica::Ref<SCode::Enum>) -> ArcStr {
    let mut s: ArcStr;
    s = (match &**e {
        SCode::Enum { literal: __esc_s, .. } => {
            s = (*__esc_s).clone();
            s.clone()
        }
    });
    s
}

pub fn isRecord(mut inClass: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inClass {
        SCode::Element::CLASS {
            restriction: SCode::Restriction::R_RECORD { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub fn isTypeVar(mut inClass: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inClass {
        SCode::Element::CLASS {
            restriction: SCode::Restriction::R_TYPE { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub fn isPolymorphicTypeVar(mut cls: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match cls {
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_TYPE { .. }, classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TCOMPLEX { path: Deref @ Absyn::Path::IDENT { name: Deref @ "polymorphic" }, .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isOperatorRecord(mut inClass: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inClass {
        SCode::Element::CLASS {
            restriction: SCode::Restriction::R_RECORD { isOperator: true },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub fn isFunction(mut inClass: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inClass {
        SCode::Element::CLASS {
            restriction: SCode::Restriction::R_FUNCTION { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub fn isUniontype(mut inClass: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inClass {
        SCode::Element::CLASS {
            restriction: SCode::Restriction::R_UNIONTYPE { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub fn isFunctionRestriction(mut inRestriction: &SCode::Restriction) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match inRestriction.clone() {
        SCode::Restriction::R_FUNCTION { .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isFunctionOrExtFunctionRestriction(mut r: &SCode::Restriction) -> bool {
    let mut res: bool;
    res = (match r.clone() {
        SCode::Restriction::R_FUNCTION {
            functionRestriction: SCode::FunctionRestriction::FR_NORMAL_FUNCTION { .. },
        } => true,
        SCode::Restriction::R_FUNCTION {
            functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { .. },
        } => true,
        _ => false,
    });
    res
}

pub fn isOperator(mut el: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut res: bool;
    res = (match &**el {
        SCode::Element::CLASS {
            restriction: SCode::Restriction::R_OPERATOR { .. },
            ..
        } => true,
        SCode::Element::CLASS {
            restriction:
                SCode::Restriction::R_FUNCTION {
                    functionRestriction: SCode::FunctionRestriction::FR_OPERATOR_FUNCTION { .. },
                },
            ..
        } => true,
        _ => false,
    });
    res
}

pub(crate) fn isEnumeration(mut el: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut res: bool;
    res = (match &**el {
        SCode::Element::CLASS {
            restriction: SCode::Restriction::R_ENUMERATION { .. },
            ..
        } => true,
        _ => false,
    });
    res
}

pub fn className(mut inClass: &metamodelica::Ref<SCode::Element>) -> Result<ArcStr> {
    let mut outName: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &((*inClass)) {
        Deref @ SCode::Element::CLASS { name: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outName = metamodelica::Own::own(__pa0);
    Ok(outName)
}

pub fn classSetPartial(
    mut cls: metamodelica::Ref<SCode::Element>,
    mut inPartial: SCode::Partial,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut cls: metamodelica::Ref<SCode::Element> = cls;
    let () = (match &*cls {
        SCode::Element::CLASS { .. } => {
            assign_variant_field!(cls => SCode::Element::CLASS; partialPrefix = inPartial);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(cls)
}

pub fn elementEqual(
    mut element1: &metamodelica::Ref<SCode::Element>,
    mut element2: &metamodelica::Ref<SCode::Element>,
) -> bool {
    let mut equal: bool;
    equal = 'mc: {
        let __mc_input = (&**element1, &**element2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { .. }, Deref @ SCode::Element::CLASS { .. }) => {
                    Ok(stringEq(&var_field!((**element1).name, SCode::Element::CLASS), &var_field!((**element2).name, SCode::Element::CLASS)) && prefixesEqual(var_field!((**element1).prefixes, SCode::Element::CLASS), var_field!((**element2).prefixes, SCode::Element::CLASS)) && var_field!((**element1).encapsulatedPrefix, SCode::Element::CLASS).clone() == var_field!((**element2).encapsulatedPrefix, SCode::Element::CLASS).clone() && var_field!((**element1).partialPrefix, SCode::Element::CLASS).clone() == var_field!((**element2).partialPrefix, SCode::Element::CLASS).clone() && restrictionEqual(var_field!((**element1).restriction, SCode::Element::CLASS), var_field!((**element2).restriction, SCode::Element::CLASS)) && classDefEqual(var_field!((**element1).classDef, SCode::Element::CLASS), var_field!((**element2).classDef, SCode::Element::CLASS))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::COMPONENT { .. }, Deref @ SCode::Element::COMPONENT { .. }) => {
                    Ok(stringEq(&var_field!((**element1).name, SCode::Element::COMPONENT), &var_field!((**element2).name, SCode::Element::COMPONENT)) && prefixesEqual(var_field!((**element1).prefixes, SCode::Element::COMPONENT), var_field!((**element2).prefixes, SCode::Element::COMPONENT)) && attributesEqual(var_field!((**element1).attributes, SCode::Element::COMPONENT), var_field!((**element2).attributes, SCode::Element::COMPONENT)) && modEqual(var_field!((**element1).modifications, SCode::Element::COMPONENT), var_field!((**element2).modifications, SCode::Element::COMPONENT)) && AbsynUtil::typeSpecEqual(var_field!((**element1).typeSpec, SCode::Element::COMPONENT), var_field!((**element2).typeSpec, SCode::Element::COMPONENT))? && var_field!((**element1).condition, SCode::Element::COMPONENT).clone() == var_field!((**element2).condition, SCode::Element::COMPONENT).clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::EXTENDS { .. }, Deref @ SCode::Element::EXTENDS { .. }) => {
                    Ok(AbsynUtil::pathEqual(var_field!((**element1).baseClassPath, SCode::Element::EXTENDS), var_field!((**element2).baseClassPath, SCode::Element::EXTENDS)) && modEqual(var_field!((**element1).modifications, SCode::Element::EXTENDS), var_field!((**element2).modifications, SCode::Element::EXTENDS)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::IMPORT { .. }, Deref @ SCode::Element::IMPORT { .. }) => {
                    Ok(AbsynUtil::importEqual(var_field!((**element1).imp, SCode::Element::IMPORT), var_field!((**element2).imp, SCode::Element::IMPORT)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::DEFINEUNIT { .. }, Deref @ SCode::Element::DEFINEUNIT { .. }) => {
                    Ok(stringEq(&var_field!((**element1).name, SCode::Element::DEFINEUNIT), &var_field!((**element2).name, SCode::Element::DEFINEUNIT)) && var_field!((**element1).exp, SCode::Element::DEFINEUNIT).clone() == var_field!((**element2).exp, SCode::Element::DEFINEUNIT).clone() && var_field!((**element1).weight, SCode::Element::DEFINEUNIT).clone() == var_field!((**element2).weight, SCode::Element::DEFINEUNIT).clone())
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
    equal
}

// stefan
pub(crate) fn annotationEqual(
    mut annotation1: &metamodelica::Ref<SCode::Annotation>,
    mut annotation2: &metamodelica::Ref<SCode::Annotation>,
) -> bool {
    let mut equal: bool = modEqual(&annotation1.modification, &annotation2.modification);
    equal
}

pub fn restrictionEqual(mut restr1: &SCode::Restriction, mut restr2: &SCode::Restriction) -> bool {
    let mut equal: bool;
    equal = (match (restr1.clone(), restr2.clone()) {
        (SCode::Restriction::R_CLASS { .. }, SCode::Restriction::R_CLASS { .. }) => true,
        (SCode::Restriction::R_OPTIMIZATION { .. }, SCode::Restriction::R_OPTIMIZATION { .. }) => true,
        (SCode::Restriction::R_MODEL { .. }, SCode::Restriction::R_MODEL { .. }) => true,
        (SCode::Restriction::R_RECORD { isOperator: true }, SCode::Restriction::R_RECORD { isOperator: true }) => true,
        (SCode::Restriction::R_RECORD { isOperator: false }, SCode::Restriction::R_RECORD { isOperator: false }) => {
            true
        }
        (SCode::Restriction::R_BLOCK { .. }, SCode::Restriction::R_BLOCK { .. }) => true,
        (
            SCode::Restriction::R_CONNECTOR { isExpandable: true },
            SCode::Restriction::R_CONNECTOR { isExpandable: true },
        ) => true,
        (
            SCode::Restriction::R_CONNECTOR { isExpandable: false },
            SCode::Restriction::R_CONNECTOR { isExpandable: false },
        ) => true,
        (SCode::Restriction::R_OPERATOR { .. }, SCode::Restriction::R_OPERATOR { .. }) => true,
        (SCode::Restriction::R_TYPE { .. }, SCode::Restriction::R_TYPE { .. }) => true,
        (SCode::Restriction::R_PACKAGE { .. }, SCode::Restriction::R_PACKAGE { .. }) => true,
        (
            SCode::Restriction::R_FUNCTION {
                functionRestriction: mut funcRest1,
            },
            SCode::Restriction::R_FUNCTION {
                functionRestriction: mut funcRest2,
            },
        ) => funcRestrictionEqual(funcRest1.clone(), funcRest2.clone()),
        (SCode::Restriction::R_ENUMERATION { .. }, SCode::Restriction::R_ENUMERATION { .. }) => true,
        (SCode::Restriction::R_PREDEFINED_INTEGER { .. }, SCode::Restriction::R_PREDEFINED_INTEGER { .. }) => true,
        (SCode::Restriction::R_PREDEFINED_REAL { .. }, SCode::Restriction::R_PREDEFINED_REAL { .. }) => true,
        (SCode::Restriction::R_PREDEFINED_STRING { .. }, SCode::Restriction::R_PREDEFINED_STRING { .. }) => true,
        (SCode::Restriction::R_PREDEFINED_BOOLEAN { .. }, SCode::Restriction::R_PREDEFINED_BOOLEAN { .. }) => true,
        (SCode::Restriction::R_PREDEFINED_CLOCK { .. }, SCode::Restriction::R_PREDEFINED_CLOCK { .. }) => true,
        (SCode::Restriction::R_PREDEFINED_ENUMERATION { .. }, SCode::Restriction::R_PREDEFINED_ENUMERATION { .. }) => {
            true
        }
        (SCode::Restriction::R_UNIONTYPE { .. }, SCode::Restriction::R_UNIONTYPE { .. }) => {
            ({
                let mut __acc: Option<bool> = None;
                let __thr_src0 = var_field!(restr1.typeVars, SCode::Restriction::R_UNIONTYPE).clone();
                let mut __thr_it0 = (&__thr_src0).into_iter();
                let __thr_src1 = var_field!(restr2.typeVars, SCode::Restriction::R_UNIONTYPE).clone();
                let mut __thr_it1 = (&__thr_src1).into_iter();
                loop {
                    match (__thr_it0.next(), __thr_it1.next()) {
                        (Some(t1), Some(t2)) => {
                            let __x = metamodelica::stringEq(&t1, &t2);
                            __acc = Some(match __acc {
                                None => __x,
                                Some(__cur) => {
                                    if __x < __cur {
                                        __x
                                    } else {
                                        __cur
                                    }
                                }
                            });
                        }
                        (None, None) => break,
                        _ => panic!("threaded for: ranges of unequal length"),
                    }
                }
                __acc.unwrap_or(true)
            })
        }
        _ => false,
    });
    equal
}

pub(crate) fn funcRestrictionEqual(
    mut funcRestr1: SCode::FunctionRestriction,
    mut funcRestr2: SCode::FunctionRestriction,
) -> bool {
    let mut equal: bool;
    equal = (match (funcRestr1, funcRestr2) {
        (
            SCode::FunctionRestriction::FR_NORMAL_FUNCTION { .. },
            SCode::FunctionRestriction::FR_NORMAL_FUNCTION { .. },
        ) => AbsynUtil::purityEqual(
            var_field!(funcRestr1.purity, SCode::FunctionRestriction::FR_NORMAL_FUNCTION).clone(),
            var_field!(funcRestr2.purity, SCode::FunctionRestriction::FR_NORMAL_FUNCTION).clone(),
            false,
        ),
        (
            SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { .. },
            SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { .. },
        ) => AbsynUtil::purityEqual(
            var_field!(funcRestr1.purity, SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION).clone(),
            var_field!(funcRestr2.purity, SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION).clone(),
            false,
        ),
        (
            SCode::FunctionRestriction::FR_OPERATOR_FUNCTION { .. },
            SCode::FunctionRestriction::FR_OPERATOR_FUNCTION { .. },
        ) => true,
        (
            SCode::FunctionRestriction::FR_RECORD_CONSTRUCTOR { .. },
            SCode::FunctionRestriction::FR_RECORD_CONSTRUCTOR { .. },
        ) => true,
        (
            SCode::FunctionRestriction::FR_PARALLEL_FUNCTION { .. },
            SCode::FunctionRestriction::FR_PARALLEL_FUNCTION { .. },
        ) => true,
        (
            SCode::FunctionRestriction::FR_KERNEL_FUNCTION { .. },
            SCode::FunctionRestriction::FR_KERNEL_FUNCTION { .. },
        ) => true,
        _ => false,
    });
    equal
}

pub(crate) fn enumEqual(mut e1: &metamodelica::Ref<SCode::Enum>, mut e2: &metamodelica::Ref<SCode::Enum>) -> bool {
    let mut isEqual: bool = metamodelica::stringEq(&e1.literal, &e2.literal);
    isEqual
}

fn classDefEqual(
    mut cdef1: &metamodelica::Ref<SCode::ClassDef>,
    mut cdef2: &metamodelica::Ref<SCode::ClassDef>,
) -> Result<bool> {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match (cdef1, cdef2) {
        (Deref @ SCode::ClassDef::PARTS { .. }, Deref @ SCode::ClassDef::PARTS { .. }) => List::isEqualOnTrue(var_field!((**cdef1).elementLst, SCode::ClassDef::PARTS).clone(), var_field!((**cdef2).elementLst, SCode::ClassDef::PARTS).clone(), &move |__a0: metamodelica::Ref<SCode::Element>, __a1: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(elementEqual(&__a0, &__a1)) })? && List::isEqualOnTrue(var_field!((**cdef1).normalEquationLst, SCode::ClassDef::PARTS).clone(), var_field!((**cdef2).normalEquationLst, SCode::ClassDef::PARTS).clone(), &move |__a0: metamodelica::Ref<SCode::Equation>, __a1: metamodelica::Ref<SCode::Equation>| -> metamodelica::Result<_> { ::std::result::Result::Ok(equationEqual(&__a0, &__a1)) })? && List::isEqualOnTrue(var_field!((**cdef1).initialEquationLst, SCode::ClassDef::PARTS).clone(), var_field!((**cdef2).initialEquationLst, SCode::ClassDef::PARTS).clone(), &move |__a0: metamodelica::Ref<SCode::Equation>, __a1: metamodelica::Ref<SCode::Equation>| -> metamodelica::Result<_> { ::std::result::Result::Ok(equationEqual(&__a0, &__a1)) })? && List::isEqualOnTrue(var_field!((**cdef1).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), var_field!((**cdef2).normalAlgorithmLst, SCode::ClassDef::PARTS).clone(), &move |__a0: metamodelica::Ref<SCode::AlgorithmSection>, __a1: metamodelica::Ref<SCode::AlgorithmSection>| algorithmEqual(&__a0, &__a1))? && List::isEqualOnTrue(var_field!((**cdef1).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), var_field!((**cdef2).initialAlgorithmLst, SCode::ClassDef::PARTS).clone(), &move |__a0: metamodelica::Ref<SCode::AlgorithmSection>, __a1: metamodelica::Ref<SCode::AlgorithmSection>| algorithmEqual(&__a0, &__a1))?,
        (Deref @ SCode::ClassDef::DERIVED { .. }, Deref @ SCode::ClassDef::DERIVED { .. }) => AbsynUtil::typeSpecEqual(var_field!((**cdef1).typeSpec, SCode::ClassDef::DERIVED), var_field!((**cdef2).typeSpec, SCode::ClassDef::DERIVED))? && modEqual(var_field!((**cdef1).modifications, SCode::ClassDef::DERIVED), var_field!((**cdef2).modifications, SCode::ClassDef::DERIVED)) && attributesEqual(var_field!((**cdef1).attributes, SCode::ClassDef::DERIVED), var_field!((**cdef2).attributes, SCode::ClassDef::DERIVED)),
        (Deref @ SCode::ClassDef::ENUMERATION { .. }, Deref @ SCode::ClassDef::ENUMERATION { .. }) => List::isEqualOnTrue(var_field!((**cdef1).enumLst, SCode::ClassDef::ENUMERATION).clone(), var_field!((**cdef2).enumLst, SCode::ClassDef::ENUMERATION).clone(), &move |__a0: metamodelica::Ref<SCode::Enum>, __a1: metamodelica::Ref<SCode::Enum>| -> metamodelica::Result<_> { ::std::result::Result::Ok(enumEqual(&__a0, &__a1)) })?,
        (Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }) => modEqual(var_field!((**cdef1).modifications, SCode::ClassDef::CLASS_EXTENDS), var_field!((**cdef2).modifications, SCode::ClassDef::CLASS_EXTENDS)) && classDefEqual(var_field!((**cdef1).composition, SCode::ClassDef::CLASS_EXTENDS), var_field!((**cdef2).composition, SCode::ClassDef::CLASS_EXTENDS))?,
        (Deref @ SCode::ClassDef::PDER { .. }, Deref @ SCode::ClassDef::PDER { .. }) => List::isEqualOnTrue(var_field!((**cdef1).derivedVariables, SCode::ClassDef::PDER).clone(), var_field!((**cdef2).derivedVariables, SCode::ClassDef::PDER).clone(), &fnptr!(stringEq, ArcStr, ArcStr))?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

fn arraydimOptEqual(
    mut adopt1: Option<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>,
    mut adopt2: Option<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>,
) -> Result<bool> {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match &((adopt1, adopt2)) {
        (None, None) => {
            true
        },
        (Some(lst1), Some(lst2)) => {
            List::isEqualOnTrue(lst1.clone(), lst2.clone(), &move |__a0: metamodelica::Ref<Absyn::Subscript>, __a1: metamodelica::Ref<Absyn::Subscript>| subscriptEqual(&__a0, &__a1))?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

fn subscriptEqual(
    mut sub1: &metamodelica::Ref<Absyn::Subscript>,
    mut sub2: &metamodelica::Ref<Absyn::Subscript>,
) -> Result<bool> {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match (sub1, sub2) {
        (Deref @ Absyn::Subscript::NOSUB { .. }, Deref @ Absyn::Subscript::NOSUB { .. }) => {
            true
        },
        (Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e1 }, Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e2 }) => {
            AbsynUtil::expEqual(e1.clone(), e2.clone())?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(equal)
}

fn algorithmEqual(
    mut alg1: &metamodelica::Ref<SCode::AlgorithmSection>,
    mut alg2: &metamodelica::Ref<SCode::AlgorithmSection>,
) -> Result<bool> {
    let mut equal: bool;
    equal = List::isEqualOnTrue(
        alg1.statements.clone(),
        alg2.statements.clone(),
        &fnptr!(
            statementEqual,
            metamodelica::Ref<SCode::Statement>,
            metamodelica::Ref<SCode::Statement>
        ),
    )?;
    Ok(equal)
}

fn statementEqual(mut ai1: metamodelica::Ref<SCode::Statement>, mut ai2: metamodelica::Ref<SCode::Statement>) -> bool {
    let mut equal: bool = false;
    equal = 'mc: {
        let __mc_input = (ai1, ai2);
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Statement::ALG_ASSIGN { assignComponent: Deref @ Absyn::Exp::CREF { componentRef: cr1 }, value: e1, .. }, Deref @ SCode::Statement::ALG_ASSIGN { assignComponent: Deref @ Absyn::Exp::CREF { componentRef: cr2 }, value: e2, .. }) => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut equal: bool = equal.clone();
                    b1 = AbsynUtil::crefEqual(metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&cr2))?;
                    b2 = AbsynUtil::expEqual(e1.clone(), e2.clone())?;
                    equal = boolAnd(b1, b2);
                    Ok((equal, equal.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            equal = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Statement::ALG_ASSIGN { assignComponent: e11 @ Deref @ Absyn::Exp::TUPLE { expressions: _ }, value: e12, .. }, Deref @ SCode::Statement::ALG_ASSIGN { assignComponent: e21 @ Deref @ Absyn::Exp::TUPLE { expressions: _ }, value: e22, .. }) => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut equal: bool = equal.clone();
                    b1 = AbsynUtil::expEqual(e11.clone(), e21.clone())?;
                    b2 = AbsynUtil::expEqual(e12.clone(), e22.clone())?;
                    equal = boolAnd(b1, b2);
                    Ok((equal, equal.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            equal = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (a1, a2) => {
                    let mut alg1: metamodelica::Ref<Absyn::Algorithm>;
                    let mut alg2: metamodelica::Ref<Absyn::Algorithm>;
                    let __pa0 = ::match_deref::match_deref! { match &(statementToAlgorithmItem(metamodelica::AsArg::as_arg(&a1))?) {
                        Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    alg1 = metamodelica::Own::own(__pa0);
                    let __pa1 = ::match_deref::match_deref! { match &(statementToAlgorithmItem(metamodelica::AsArg::as_arg(&a2))?) {
                        Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: __pa1, .. } => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    alg2 = metamodelica::Own::own(__pa1);
                    Ok(alg1.clone() == alg2.clone())
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
    equal
}

fn equationEqual(mut eq1: &metamodelica::Ref<SCode::Equation>, mut eq2: &metamodelica::Ref<SCode::Equation>) -> bool {
    let mut equal: bool;
    equal = 'mc: {
        let __mc_input = (&**eq1, &**eq2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Equation::EQ_IF { condition: ifcond1, thenBranch: tb1, elseBranch: fb1, .. }, Deref @ SCode::Equation::EQ_IF { condition: ifcond2, thenBranch: tb2, elseBranch: fb2, .. }) => {
                    let true = (equationEqual2(metamodelica::AsArg::as_arg(&tb1), metamodelica::AsArg::as_arg(&tb2))?) else { return Err("pattern mismatch") };
                    let true = (List::isEqualOnTrue(fb1.clone(), fb2.clone(), &move |__a0: metamodelica::Ref<SCode::Equation>, __a1: metamodelica::Ref<SCode::Equation>| -> metamodelica::Result<_> { ::std::result::Result::Ok(equationEqual(&__a0, &__a1)) })?) else { return Err("pattern mismatch") };
                    let true = (List::isEqualOnTrue(ifcond1.clone(), ifcond2.clone(), &AbsynUtil::expEqual)?) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Equation::EQ_EQUALS { expLeft: e11, expRight: e12, .. }, Deref @ SCode::Equation::EQ_EQUALS { expLeft: e21, expRight: e22, .. }) => {
                    let true = (AbsynUtil::expEqual(e11.clone(), e21.clone())?) else { return Err("pattern mismatch") };
                    let true = (AbsynUtil::expEqual(e12.clone(), e22.clone())?) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Equation::EQ_PDE { expLeft: e11, expRight: e12, domain: cr1, .. }, Deref @ SCode::Equation::EQ_PDE { expLeft: e21, expRight: e22, domain: cr2, .. }) => {
                    let true = (AbsynUtil::expEqual(e11.clone(), e21.clone())?) else { return Err("pattern mismatch") };
                    let true = (AbsynUtil::expEqual(e12.clone(), e22.clone())?) else { return Err("pattern mismatch") };
                    let true = (AbsynUtil::crefEqual(metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&cr2))?) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Equation::EQ_CONNECT { crefLeft: cr11, crefRight: cr12, .. }, Deref @ SCode::Equation::EQ_CONNECT { crefLeft: cr21, crefRight: cr22, .. }) => {
                    let true = (AbsynUtil::crefEqual(metamodelica::AsArg::as_arg(&cr11), metamodelica::AsArg::as_arg(&cr21))?) else { return Err("pattern mismatch") };
                    let true = (AbsynUtil::crefEqual(metamodelica::AsArg::as_arg(&cr12), metamodelica::AsArg::as_arg(&cr22))?) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Equation::EQ_FOR { index: id1, range: Some(exp1), eEquationLst: eql1, .. }, Deref @ SCode::Equation::EQ_FOR { index: id2, range: Some(exp2), eEquationLst: eql2, .. }) => {
                    let true = (List::isEqualOnTrue(eql1.clone(), eql2.clone(), &move |__a0: metamodelica::Ref<SCode::Equation>, __a1: metamodelica::Ref<SCode::Equation>| -> metamodelica::Result<_> { ::std::result::Result::Ok(equationEqual(&__a0, &__a1)) })?) else { return Err("pattern mismatch") };
                    let true = (AbsynUtil::expEqual(exp1.clone(), exp2.clone())?) else { return Err("pattern mismatch") };
                    let true = (stringEq(&id1, &id2)) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Equation::EQ_FOR { index: id1, range: None, eEquationLst: eql1, .. }, Deref @ SCode::Equation::EQ_FOR { index: id2, range: None, eEquationLst: eql2, .. }) => {
                    let true = (List::isEqualOnTrue(eql1.clone(), eql2.clone(), &move |__a0: metamodelica::Ref<SCode::Equation>, __a1: metamodelica::Ref<SCode::Equation>| -> metamodelica::Result<_> { ::std::result::Result::Ok(equationEqual(&__a0, &__a1)) })?) else { return Err("pattern mismatch") };
                    let true = (stringEq(&id1, &id2)) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Equation::EQ_WHEN { condition: cond1, eEquationLst: elst1, .. }, Deref @ SCode::Equation::EQ_WHEN { condition: cond2, eEquationLst: elst2, .. }) => {
                    let true = (List::isEqualOnTrue(elst1.clone(), elst2.clone(), &move |__a0: metamodelica::Ref<SCode::Equation>, __a1: metamodelica::Ref<SCode::Equation>| -> metamodelica::Result<_> { ::std::result::Result::Ok(equationEqual(&__a0, &__a1)) })?) else { return Err("pattern mismatch") };
                    let true = (AbsynUtil::expEqual(cond1.clone(), cond2.clone())?) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Equation::EQ_ASSERT { condition: c1, message: m1, .. }, Deref @ SCode::Equation::EQ_ASSERT { condition: c2, message: m2, .. }) => {
                    let true = (AbsynUtil::expEqual(c1.clone(), c2.clone())?) else { return Err("pattern mismatch") };
                    let true = (AbsynUtil::expEqual(m1.clone(), m2.clone())?) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Equation::EQ_REINIT { .. }, Deref @ SCode::Equation::EQ_REINIT { .. }) => {
                    let true = (AbsynUtil::expEqual(var_field!((**eq1).cref, SCode::Equation::EQ_REINIT).clone(), var_field!((**eq2).cref, SCode::Equation::EQ_REINIT).clone())?) else { return Err("pattern mismatch") };
                    let true = (AbsynUtil::expEqual(var_field!((**eq1).expReinit, SCode::Equation::EQ_REINIT).clone(), var_field!((**eq2).expReinit, SCode::Equation::EQ_REINIT).clone())?) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Equation::EQ_NORETCALL { exp: e1, .. }, Deref @ SCode::Equation::EQ_NORETCALL { exp: e2, .. }) => {
                    let true = (AbsynUtil::expEqual(e1.clone(), e2.clone())?) else { return Err("pattern mismatch") };
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
    equal
}

fn equationEqual2(
    mut inTb1: &metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Equation>>>,
    mut inTb2: &metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Equation>>>,
) -> Result<bool> {
    let mut bOut: bool;
    bOut = 'mc: {
        let __mc_input = (&**inTb1, &**inTb2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: tb_1, tail: tb1 }, Deref @ metamodelica::ListNode::Cons { head: tb_2, tail: tb2 }) => {
                    let true = (List::isEqualOnTrue(tb_1.clone(), tb_2.clone(), &move |__a0: metamodelica::Ref<SCode::Equation>, __a1: metamodelica::Ref<SCode::Equation>| -> metamodelica::Result<_> { ::std::result::Result::Ok(equationEqual(&__a0, &__a1)) })?) else { return Err("pattern mismatch") };
                    let true = (equationEqual2(metamodelica::AsArg::as_arg(&tb1), metamodelica::AsArg::as_arg(&tb2))?) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(bOut)
}

pub fn modEqual(mut mod1: &metamodelica::Ref<SCode::Mod>, mut mod2: &metamodelica::Ref<SCode::Mod>) -> bool {
    let mut equal: bool;
    equal = 'mc: {
        let __mc_input = (&**mod1, &**mod2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Mod::MOD { finalPrefix: f1, eachPrefix: each1, subModLst: submodlst1, binding: Some(e1), comment: _, .. }, Deref @ SCode::Mod::MOD { finalPrefix: f2, eachPrefix: each2, subModLst: submodlst2, binding: Some(e2), comment: _, .. }) => {
                    let true = (f1.clone() == f2.clone()) else { return Err("pattern mismatch") };
                    let true = (eachEqual(each1.clone(), each2.clone())) else { return Err("pattern mismatch") };
                    let true = (subModsEqual(metamodelica::AsArg::as_arg(&submodlst1), metamodelica::AsArg::as_arg(&submodlst2))) else { return Err("pattern mismatch") };
                    let true = (AbsynUtil::expEqual(e1.clone(), e2.clone())?) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Mod::MOD { finalPrefix: f1, eachPrefix: each1, subModLst: submodlst1, binding: None, comment: _, .. }, Deref @ SCode::Mod::MOD { finalPrefix: f2, eachPrefix: each2, subModLst: submodlst2, binding: None, comment: _, .. }) => {
                    let true = (f1.clone() == f2.clone()) else { return Err("pattern mismatch") };
                    let true = (eachEqual(each1.clone(), each2.clone())) else { return Err("pattern mismatch") };
                    let true = (subModsEqual(metamodelica::AsArg::as_arg(&submodlst1), metamodelica::AsArg::as_arg(&submodlst2))) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Mod::NOMOD { .. }, Deref @ SCode::Mod::NOMOD { .. }) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Mod::REDECL { finalPrefix: f1, eachPrefix: each1, element: elt1 }, Deref @ SCode::Mod::REDECL { finalPrefix: f2, eachPrefix: each2, element: elt2 }) => {
                    let true = (f1.clone() == f2.clone()) else { return Err("pattern mismatch") };
                    let true = (eachEqual(each1.clone(), each2.clone())) else { return Err("pattern mismatch") };
                    let true = (elementEqual(metamodelica::AsArg::as_arg(&elt1), metamodelica::AsArg::as_arg(&elt2))) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Mod::BREAK_COMPONENT { .. }, Deref @ SCode::Mod::BREAK_COMPONENT { .. }) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Mod::BREAK_CONNECT { .. }, Deref @ SCode::Mod::BREAK_CONNECT { .. }) => {
                    Ok(AbsynUtil::crefEqual(var_field!((**mod1).lhs, SCode::Mod::BREAK_CONNECT), var_field!((**mod2).lhs, SCode::Mod::BREAK_CONNECT))? && AbsynUtil::crefEqual(var_field!((**mod1).rhs, SCode::Mod::BREAK_CONNECT), var_field!((**mod2).lhs, SCode::Mod::BREAK_CONNECT))?)
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
    equal
}

fn subModsEqual(
    mut inSubModLst1: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    mut inSubModLst2: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
) -> bool {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match (inSubModLst1, inSubModLst2) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            true
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::SubMod { ident: id1, r#mod: mod1 }, tail: subModLst1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::SubMod { ident: id2, r#mod: mod2 }, tail: subModLst2 }) if (stringEq(&id1, &id2) && modEqual(metamodelica::AsArg::as_arg(&mod1), metamodelica::AsArg::as_arg(&mod2)) && subModsEqual(subModLst1, subModLst2)) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    equal
}

fn subscriptsEqual(
    mut inSs1: &metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut inSs2: &metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
) -> bool {
    let mut equal: bool;
    equal = 'mc: {
        let __mc_input = (&**inSs1, &**inSs2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Subscript::NOSUB { .. }, tail: ss1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Subscript::NOSUB { .. }, tail: ss2 }) => {
                    Ok(subscriptsEqual(metamodelica::AsArg::as_arg(&ss1), metamodelica::AsArg::as_arg(&ss2)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e1 }, tail: ss1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e2 }, tail: ss2 }) => {
                    let true = (AbsynUtil::expEqual(e1.clone(), e2.clone())?) else { return Err("pattern mismatch") };
                    let true = (subscriptsEqual(metamodelica::AsArg::as_arg(&ss1), metamodelica::AsArg::as_arg(&ss2))) else { return Err("pattern mismatch") };
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
    equal
}

pub fn attributesEqual(mut attr1: &SCode::Attributes, mut attr2: &SCode::Attributes) -> bool {
    let mut equal: bool;
    equal = arrayDimEqual(&attr1.arrayDims, &attr2.arrayDims)
        && attr1.connectorType.clone() == attr2.connectorType.clone()
        && parallelismEqual(attr1.parallelism.clone(), attr2.parallelism.clone())
        && variabilityEqual(attr1.variability.clone(), attr2.variability.clone())
        && AbsynUtil::directionEqual(attr1.direction.clone(), attr2.direction.clone())
        && AbsynUtil::isFieldEqual(attr1.isField.clone(), attr2.isField.clone());
    equal
}

pub fn parallelismEqual(mut prl1: SCode::Parallelism, mut prl2: SCode::Parallelism) -> bool {
    let mut equal: bool;
    equal = (match (prl1, prl2) {
        (SCode::Parallelism::PARGLOBAL { .. }, SCode::Parallelism::PARGLOBAL { .. }) => true,
        (SCode::Parallelism::PARLOCAL { .. }, SCode::Parallelism::PARLOCAL { .. }) => true,
        (SCode::Parallelism::NON_PARALLEL { .. }, SCode::Parallelism::NON_PARALLEL { .. }) => true,
        _ => false,
    });
    equal
}

pub(crate) fn variabilityEqual(mut var1: SCode::Variability, mut var2: SCode::Variability) -> bool {
    let mut equal: bool;
    equal = (match (var1, var2) {
        (SCode::Variability::VAR { .. }, SCode::Variability::VAR { .. }) => true,
        (SCode::Variability::DISCRETE { .. }, SCode::Variability::DISCRETE { .. }) => true,
        (SCode::Variability::PARAM { .. }, SCode::Variability::PARAM { .. }) => true,
        (SCode::Variability::CONST { .. }, SCode::Variability::CONST { .. }) => true,
        _ => false,
    });
    equal
}

fn arrayDimEqual(
    mut iad1: &metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut iad2: &metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
) -> bool {
    let mut equal: bool;
    equal = 'mc: {
        let __mc_input = (&**iad1, &**iad2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Subscript::NOSUB { .. }, tail: ad1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Subscript::NOSUB { .. }, tail: ad2 }) => {
                    let true = (arrayDimEqual(metamodelica::AsArg::as_arg(&ad1), metamodelica::AsArg::as_arg(&ad2))) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e1 }, tail: ad1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e2 }, tail: ad2 }) => {
                    let true = (AbsynUtil::expEqual(e1.clone(), e2.clone())?) else { return Err("pattern mismatch") };
                    let true = (arrayDimEqual(metamodelica::AsArg::as_arg(&ad1), metamodelica::AsArg::as_arg(&ad2))) else { return Err("pattern mismatch") };
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
    equal
}

pub(crate) fn setClassRestriction(
    mut r: SCode::Restriction,
    mut cl: metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut cl: metamodelica::Ref<SCode::Element> = cl;
    let () = (match &*cl {
        SCode::Element::CLASS { .. } => {
            assign_variant_field!(cl => SCode::Element::CLASS; restriction = r);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(cl)
}

pub fn setClassName(
    mut name: ArcStr,
    mut cl: metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut cl: metamodelica::Ref<SCode::Element> = cl;
    let () = (match &*cl {
        SCode::Element::CLASS { .. } => {
            if !metamodelica::stringEq(&name, &var_field!((*cl).name, SCode::Element::CLASS)) {
                assign_variant_field!(cl => SCode::Element::CLASS; name = name);
            }
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(cl)
}

pub fn makeClassPartial(mut inClass: metamodelica::Ref<SCode::Element>) -> metamodelica::Ref<SCode::Element> {
    let mut outClass: metamodelica::Ref<SCode::Element> = inClass;
    outClass = (match &*outClass {
        SCode::Element::CLASS {
            partialPrefix: SCode::Partial::NOT_PARTIAL { .. },
            ..
        } => {
            assign_variant_field!(outClass => SCode::Element::CLASS; partialPrefix = openmodelica_frontend_types::SCode::Partial::PARTIAL);
            outClass
        }
        _ => outClass,
    });
    outClass
}

pub fn setClassPartialPrefix(
    mut partialPrefix: SCode::Partial,
    mut cl: metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut cl: metamodelica::Ref<SCode::Element> = cl;
    let () = (match &*cl {
        SCode::Element::CLASS { .. } => {
            if !(partialPrefix == var_field!((*cl).partialPrefix, SCode::Element::CLASS).clone()) {
                assign_variant_field!(cl => SCode::Element::CLASS; partialPrefix = partialPrefix);
            }
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(cl)
}

pub fn findIteratorIndexedCrefsInEquations(
    mut inEqs: &metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    mut inIterator: ArcStr,
    mut inCrefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>,
) -> Result<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>> {
    let mut outCrefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>;
    outCrefs = List::fold1(
        inEqs,
        &move |__a0: metamodelica::Ref<SCode::Equation>,
               __a1: ArcStr,
               __a2: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>| {
            findIteratorIndexedCrefsInEquation(&__a0, &__a1, __a2)
        },
        inIterator,
        inCrefs,
    )?;
    Ok(outCrefs)
}

pub(crate) fn findIteratorIndexedCrefsInEquation(
    mut inEq: &metamodelica::Ref<SCode::Equation>,
    mut inIterator: &ArcStr,
    mut inCrefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>,
) -> Result<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>> {
    let mut outCrefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>;
    outCrefs = foldEquationsExps(
        inEq,
        (std::sync::Arc::new({
            let __pe_b1 = inIterator.clone();
            move |__pe_a0, __pe_a2| AbsynUtil::findIteratorIndexedCrefs(__pe_a0, &__pe_b1, __pe_a2)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>,
                    )
                        -> Result<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>
                    + 'static,
            >),
        inCrefs,
    )?;
    Ok(outCrefs)
}

pub fn findIteratorIndexedCrefsInStatements(
    mut inStatements: &metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    mut inIterator: ArcStr,
    mut inCrefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>,
) -> Result<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>> {
    let mut outCrefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>;
    outCrefs = List::fold1(
        inStatements,
        &move |__a0: metamodelica::Ref<SCode::Statement>,
               __a1: ArcStr,
               __a2: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>| {
            findIteratorIndexedCrefsInStatement(&__a0, &__a1, __a2)
        },
        inIterator,
        inCrefs,
    )?;
    Ok(outCrefs)
}

pub(crate) fn findIteratorIndexedCrefsInStatement(
    mut inStatement: &metamodelica::Ref<SCode::Statement>,
    mut inIterator: &ArcStr,
    mut inCrefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>,
) -> Result<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>> {
    let mut outCrefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>;
    outCrefs = foldStatementsExps(
        inStatement,
        (std::sync::Arc::new({
            let __pe_b1 = inIterator.clone();
            move |__pe_a0, __pe_a2| AbsynUtil::findIteratorIndexedCrefs(__pe_a0, &__pe_b1, __pe_a2)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>,
                    )
                        -> Result<metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>>
                    + 'static,
            >),
        inCrefs,
    )?;
    Ok(outCrefs)
}

fn filterComponents(
    mut inElements: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::List<ArcStr>,
)> {
    let mut outComponents: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut outComponentNames: metamodelica::List<ArcStr>;
    (outComponents, outComponentNames) = List::map_2(inElements, &filterComponents2)?;
    Ok((outComponents, outComponentNames))
}

fn filterComponents2(
    mut inElement: metamodelica::Ref<SCode::Element>,
) -> Result<(metamodelica::Ref<SCode::Element>, ArcStr)> {
    let mut outComponent: metamodelica::Ref<SCode::Element>;
    let mut outName: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::COMPONENT { name: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outName = metamodelica::Own::own(__pa0);
    outComponent = inElement;
    Ok((outComponent, outName))
}

pub fn getClassComponents(
    mut cl: &metamodelica::Ref<SCode::Element>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SCode::Element>>,
    metamodelica::List<ArcStr>,
)> {
    let mut compElts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut compNames: metamodelica::List<ArcStr>;
    (compElts, compNames) = (::match_deref::match_deref! { match cl {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. } => {
            let mut comps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut names: metamodelica::List<ArcStr>;
            (comps, names) = filterComponents(metamodelica::AsArg::as_arg(&elts))?;
            (comps, names)
        },
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { composition: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. }, .. } => {
            let mut comps: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut names: metamodelica::List<ArcStr>;
            (comps, names) = filterComponents(metamodelica::AsArg::as_arg(&elts))?;
            (comps, names)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((compElts, compNames))
}

pub fn getClassElements(
    mut cl: &metamodelica::Ref<SCode::Element>,
) -> metamodelica::List<metamodelica::Ref<SCode::Element>> {
    let mut elts: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    elts = (::match_deref::match_deref! { match cl {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: __esc_elts, .. }, .. } => {
            elts = (*__esc_elts).clone();
            elts.clone()
        },
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { composition: Deref @ SCode::ClassDef::PARTS { elementLst: __esc_elts, .. }, .. }, .. } => {
            elts = (*__esc_elts).clone();
            elts.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    elts
}

pub fn makeEnumType(
    mut inEnum: &metamodelica::Ref<SCode::Enum>,
    mut inInfo: SourceInfo,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut outEnumType: metamodelica::Ref<SCode::Element>;
    let mut literal: ArcStr;
    let mut comment: metamodelica::Ref<SCode::Comment>;
    let __arc2 = &(*inEnum);
    let SCode::ENUM {
        literal: __pa0,
        comment: __pa1,
    } = &**__arc2;
    literal = metamodelica::Own::own(__pa0);
    comment = metamodelica::Own::own(__pa1);
    checkValidEnumLiteral(literal.clone(), &inInfo)?;
    outEnumType = metamodelica::Ref::new(SCode::Element::COMPONENT {
        name: literal,
        prefixes: SCode::defaultPrefixes.clone(),
        attributes: SCode::defaultConstAttr.clone(),
        typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("EnumType"),
            }),
            arrayDim: None,
        }),
        modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
        comment: comment,
        condition: None,
        info: inInfo,
    });
    Ok(outEnumType)
}

pub fn variabilityOr(mut inConst1: SCode::Variability, mut inConst2: SCode::Variability) -> SCode::Variability {
    let mut outConst: SCode::Variability;
    outConst = (match (inConst1, inConst2) {
        (SCode::Variability::CONST { .. }, _) => openmodelica_frontend_types::SCode::Variability::CONST,
        (_, SCode::Variability::CONST { .. }) => openmodelica_frontend_types::SCode::Variability::CONST,
        (SCode::Variability::PARAM { .. }, _) => openmodelica_frontend_types::SCode::Variability::PARAM,
        (_, SCode::Variability::PARAM { .. }) => openmodelica_frontend_types::SCode::Variability::PARAM,
        (SCode::Variability::DISCRETE { .. }, _) => openmodelica_frontend_types::SCode::Variability::DISCRETE,
        (_, SCode::Variability::DISCRETE { .. }) => openmodelica_frontend_types::SCode::Variability::DISCRETE,
        _ => openmodelica_frontend_types::SCode::Variability::VAR,
    });
    outConst
}

pub fn statementToAlgorithmItem(
    mut stmt: &metamodelica::Ref<SCode::Statement>,
) -> Result<metamodelica::Ref<Absyn::AlgorithmItem>> {
    let mut algi: metamodelica::Ref<Absyn::AlgorithmItem>;
    algi = (::match_deref::match_deref! { match stmt {
        Deref @ SCode::Statement::ALG_ASSIGN { assignComponent, value, comment: _, info } => {
            metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: metamodelica::Ref::new(Absyn::Algorithm::ALG_ASSIGN { assignComponent: assignComponent.clone(), value: value.clone() }), comment: None, info: info.clone() })
        },
        Deref @ SCode::Statement::ALG_IF { boolExpr, trueBranch, elseIfBranch: branches, elseBranch, comment: _, info } => {
            let mut conditions: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut stmtsList: metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Statement>>>;
            let mut algs1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
            let mut algs2: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
            let mut algsLst: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>>;
            let mut abranches: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>)>;
            algs1 = List::map(trueBranch.clone(), &move |__a0: metamodelica::Ref<SCode::Statement>| statementToAlgorithmItem(&__a0))?;
            conditions = List::map(branches.clone(), &fnptr!(Util::tuple21, _))?;
            stmtsList = List::map(branches.clone(), &fnptr!(Util::tuple22, _))?;
            algsLst = List::mapList(stmtsList, &move |__a0: metamodelica::Ref<SCode::Statement>| statementToAlgorithmItem(&__a0))?;
            abranches = List::zip(conditions, algsLst);
            algs2 = List::map(elseBranch.clone(), &move |__a0: metamodelica::Ref<SCode::Statement>| statementToAlgorithmItem(&__a0))?;
            metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: metamodelica::Ref::new(Absyn::Algorithm::ALG_IF { ifExp: boolExpr.clone(), trueBranch: algs1, elseIfAlgorithmBranch: abranches, elseBranch: algs2 }), comment: None, info: info.clone() })
        },
        Deref @ SCode::Statement::ALG_FOR { index: iterator, range, forBody: body, comment: _, info } => {
            let mut algs1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
            algs1 = List::map(body.clone(), &move |__a0: metamodelica::Ref<SCode::Statement>| statementToAlgorithmItem(&__a0))?;
            metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: metamodelica::Ref::new(Absyn::Algorithm::ALG_FOR { iterators: list![metamodelica::Ref::new(Absyn::ForIterator { name: iterator.clone(), guardExp: None, range: range.clone() })], forBody: algs1 }), comment: None, info: info.clone() })
        },
        Deref @ SCode::Statement::ALG_PARFOR { index: iterator, range, parforBody: body, comment: _, info } => {
            let mut algs1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
            algs1 = List::map(body.clone(), &move |__a0: metamodelica::Ref<SCode::Statement>| statementToAlgorithmItem(&__a0))?;
            metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: metamodelica::Ref::new(Absyn::Algorithm::ALG_PARFOR { iterators: list![metamodelica::Ref::new(Absyn::ForIterator { name: iterator.clone(), guardExp: None, range: range.clone() })], parforBody: algs1 }), comment: None, info: info.clone() })
        },
        Deref @ SCode::Statement::ALG_WHILE { boolExpr, whileBody: body, comment: _, info } => {
            let mut algs1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
            algs1 = List::map(body.clone(), &move |__a0: metamodelica::Ref<SCode::Statement>| statementToAlgorithmItem(&__a0))?;
            metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: metamodelica::Ref::new(Absyn::Algorithm::ALG_WHILE { boolExpr: boolExpr.clone(), whileBody: algs1 }), comment: None, info: info.clone() })
        },
        Deref @ SCode::Statement::ALG_WHEN_A { branches, comment: _, info } => {
            let mut boolExpr: metamodelica::Ref<Absyn::Exp>;
            let mut conditions: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut stmtsList: metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Statement>>>;
            let mut algs1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
            let mut algsLst: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>>;
            let mut abranches: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>)>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::map(branches.clone(), &fnptr!(Util::tuple21, _))?) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            boolExpr = metamodelica::Own::own(__pa0);
            conditions = metamodelica::Own::own(__pa1);
            stmtsList = List::map(branches.clone(), &fnptr!(Util::tuple22, _))?;
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(List::mapList(stmtsList, &move |__a0: metamodelica::Ref<SCode::Statement>| statementToAlgorithmItem(&__a0))?) {
                Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            algs1 = metamodelica::Own::own(__pa2);
            algsLst = metamodelica::Own::own(__pa3);
            abranches = List::zip(conditions, algsLst);
            metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: metamodelica::Ref::new(Absyn::Algorithm::ALG_WHEN_A { boolExpr: boolExpr, whenBody: algs1, elseWhenAlgorithmBranch: abranches }), comment: None, info: info.clone() })
        },
        Deref @ SCode::Statement::ALG_ASSERT { condition: __stmt_condition, info: __stmt_info, level: __stmt_level, message: __stmt_message, .. } => {
            metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: metamodelica::Ref::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("assert"), subscripts: metamodelica::nil() }), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![__stmt_condition.clone(), __stmt_message.clone(), __stmt_level.clone()], argNames: metamodelica::nil() }) }), comment: None, info: __stmt_info.clone() })
        },
        Deref @ SCode::Statement::ALG_TERMINATE { info: __stmt_info, message: __stmt_message, .. } => {
            metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: metamodelica::Ref::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("terminate"), subscripts: metamodelica::nil() }), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![__stmt_message.clone()], argNames: metamodelica::nil() }) }), comment: None, info: __stmt_info.clone() })
        },
        Deref @ SCode::Statement::ALG_REINIT { cref: __stmt_cref, info: __stmt_info, newValue: __stmt_newValue, .. } => {
            metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: metamodelica::Ref::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("reinit"), subscripts: metamodelica::nil() }), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![__stmt_cref.clone(), __stmt_newValue.clone()], argNames: metamodelica::nil() }) }), comment: None, info: __stmt_info.clone() })
        },
        Deref @ SCode::Statement::ALG_NORETCALL { exp: Deref @ Absyn::Exp::CALL { function_: functionCall, functionArgs, .. }, comment: _, info } => {
            metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: metamodelica::Ref::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: functionCall.clone(), functionArgs: functionArgs.clone() }), comment: None, info: info.clone() })
        },
        Deref @ SCode::Statement::ALG_RETURN { comment: _, info } => {
            metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: openmodelica_ast::Absyn::Algorithm::interned_ALG_RETURN(), comment: None, info: info.clone() })
        },
        Deref @ SCode::Statement::ALG_BREAK { comment: _, info } => {
            metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: openmodelica_ast::Absyn::Algorithm::interned_ALG_BREAK(), comment: None, info: info.clone() })
        },
        Deref @ SCode::Statement::ALG_CONTINUE { comment: _, info } => {
            metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: openmodelica_ast::Absyn::Algorithm::interned_ALG_CONTINUE(), comment: None, info: info.clone() })
        },
        Deref @ SCode::Statement::ALG_FAILURE { stmts: body, comment: _, info } => {
            let mut algs1: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
            algs1 = List::map(body.clone(), &move |__a0: metamodelica::Ref<SCode::Statement>| statementToAlgorithmItem(&__a0))?;
            metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: metamodelica::Ref::new(Absyn::Algorithm::ALG_FAILURE { equ: algs1 }), comment: None, info: info.clone() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(algi)
}

pub fn emptyModOrEquality(mut r#mod: &metamodelica::Ref<SCode::Mod>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match r#mod {
        Deref @ SCode::Mod::NOMOD { .. } => true,
        Deref @ SCode::Mod::MOD { subModLst: Deref @ metamodelica::ListNode::Nil, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isComponentWithDirection(mut elt: &metamodelica::Ref<SCode::Element>, mut dir1: Absyn::Direction) -> bool {
    let mut b: bool;
    b = (match &**elt {
        SCode::Element::COMPONENT {
            attributes: SCode::Attributes { direction: dir2, .. },
            ..
        } => AbsynUtil::directionEqual(dir1, dir2.clone()),
        _ => false,
    });
    b
}

pub fn isComponent(mut elt: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut b: bool;
    b = (match &**elt {
        SCode::Element::COMPONENT { .. } => true,
        _ => false,
    });
    b
}

pub fn isNotComponent(mut elt: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut b: bool;
    b = (match &**elt {
        SCode::Element::COMPONENT { .. } => false,
        _ => true,
    });
    b
}

pub(crate) fn isClassOrComponent(mut inElement: &metamodelica::Ref<SCode::Element>) -> Result<bool> {
    let mut outIsClassOrComponent: bool;
    outIsClassOrComponent = (match &**inElement {
        SCode::Element::CLASS { .. } => true,
        SCode::Element::COMPONENT { .. } => true,
        _ => return Err("match: no arm matched"),
    });
    Ok(outIsClassOrComponent)
}

pub fn isClass(mut inElement: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut outIsClass: bool;
    outIsClass = (match &**inElement {
        SCode::Element::CLASS { .. } => true,
        _ => false,
    });
    outIsClass
}

pub fn isImport(mut element: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut isImport: bool;
    isImport = (match &**element {
        SCode::Element::IMPORT { .. } => true,
        _ => false,
    });
    isImport
}

pub(crate) fn foldEquations<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inEquation: metamodelica::Ref<SCode::Equation>,
    mut inFunc: Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Equation>, ArgT) -> Result<ArgT> + 'static>,
    mut inArg: ArgT,
) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Equation>, ArgT) -> Result<ArgT> + 'static>;

    let mut outArg: ArgT;
    outArg = inFunc(inEquation.clone(), inArg)?;
    outArg = (match &*inEquation {
        SCode::Equation::EQ_IF {
            elseBranch: __inEquation_elseBranch,
            thenBranch: __inEquation_thenBranch,
            ..
        } => {
            outArg = List::foldList(
                metamodelica::AsArg::as_arg(&__inEquation_thenBranch),
                &({
                    let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Equation>, _) -> Result<_> + 'static> =
                        inFunc.clone();
                    move |__pe_a0, __pe_a2| foldEquations(__pe_a0, __pe_b1.clone(), __pe_a2)
                }),
                outArg,
            )?;
            List::fold1(
                metamodelica::AsArg::as_arg(&__inEquation_elseBranch),
                &foldEquations,
                inFunc.clone(),
                outArg,
            )?
        }
        SCode::Equation::EQ_FOR {
            eEquationLst: __inEquation_eEquationLst,
            ..
        } => List::fold1(
            metamodelica::AsArg::as_arg(&__inEquation_eEquationLst),
            &foldEquations,
            inFunc.clone(),
            outArg,
        )?,
        SCode::Equation::EQ_WHEN {
            eEquationLst: __inEquation_eEquationLst,
            elseBranches: __inEquation_elseBranches,
            ..
        } => {
            let mut eql: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
            outArg = List::fold1(
                metamodelica::AsArg::as_arg(&__inEquation_eEquationLst),
                &foldEquations,
                inFunc.clone(),
                outArg,
            )?;
            for mut branch in &*__inEquation_elseBranches.clone() {
                (_, eql) = branch.clone();
                outArg = List::fold1(&eql, &foldEquations, inFunc.clone(), outArg)?;
            }
            outArg
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outArg)
}

pub fn foldEquationsExps<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inEquation: &metamodelica::Ref<SCode::Equation>,
    mut inFunc: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<ArgT> + 'static>,
    mut inArg: ArgT,
) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<ArgT> + 'static>;

    let mut outArg: ArgT = inArg;
    outArg = (match &**inEquation {
        SCode::Equation::EQ_IF {
            condition: __inEquation_condition,
            elseBranch: __inEquation_elseBranch,
            thenBranch: __inEquation_thenBranch,
            ..
        } => {
            outArg = List::fold(metamodelica::AsArg::as_arg(&__inEquation_condition), &*inFunc, outArg)?;
            outArg = List::foldList(
                metamodelica::AsArg::as_arg(&__inEquation_thenBranch),
                &({
                    let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, _) -> Result<_> + 'static> =
                        inFunc.clone();
                    move |__pe_a0, __pe_a2| foldEquationsExps(&__pe_a0, __pe_b1.clone(), __pe_a2)
                }),
                outArg,
            )?;
            List::fold1(
                metamodelica::AsArg::as_arg(&__inEquation_elseBranch),
                &move |__a0: metamodelica::Ref<SCode::Equation>, __a1: _, __a2: _| foldEquationsExps(&__a0, __a1, __a2),
                inFunc.clone(),
                outArg,
            )?
        }
        SCode::Equation::EQ_EQUALS {
            expLeft: __inEquation_expLeft,
            expRight: __inEquation_expRight,
            ..
        } => {
            outArg = inFunc(__inEquation_expLeft.clone(), outArg)?;
            outArg = inFunc(__inEquation_expRight.clone(), outArg)?;
            outArg
        }
        SCode::Equation::EQ_PDE {
            expLeft: __inEquation_expLeft,
            expRight: __inEquation_expRight,
            ..
        } => {
            outArg = inFunc(__inEquation_expLeft.clone(), outArg)?;
            outArg = inFunc(__inEquation_expRight.clone(), outArg)?;
            outArg
        }
        SCode::Equation::EQ_CONNECT {
            crefLeft: __inEquation_crefLeft,
            crefRight: __inEquation_crefRight,
            ..
        } => {
            outArg = inFunc(
                metamodelica::Ref::new(Absyn::Exp::CREF {
                    componentRef: __inEquation_crefLeft.clone(),
                }),
                outArg,
            )?;
            outArg = inFunc(
                metamodelica::Ref::new(Absyn::Exp::CREF {
                    componentRef: __inEquation_crefRight.clone(),
                }),
                outArg,
            )?;
            outArg
        }
        SCode::Equation::EQ_FOR {
            eEquationLst: __inEquation_eEquationLst,
            range: __inEquation_range,
            ..
        } => {
            let mut exp: metamodelica::Ref<Absyn::Exp>;
            if (__inEquation_range).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(__inEquation_range.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                exp = metamodelica::Own::own(__pa0);
                outArg = inFunc(exp, outArg)?;
            }
            List::fold1(
                metamodelica::AsArg::as_arg(&__inEquation_eEquationLst),
                &move |__a0: metamodelica::Ref<SCode::Equation>, __a1: _, __a2: _| foldEquationsExps(&__a0, __a1, __a2),
                inFunc.clone(),
                outArg,
            )?
        }
        SCode::Equation::EQ_WHEN {
            condition: __inEquation_condition,
            eEquationLst: __inEquation_eEquationLst,
            elseBranches: __inEquation_elseBranches,
            ..
        } => {
            let mut exp: metamodelica::Ref<Absyn::Exp>;
            let mut eql: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
            outArg = inFunc(__inEquation_condition.clone(), outArg)?;
            outArg = List::fold1(
                metamodelica::AsArg::as_arg(&__inEquation_eEquationLst),
                &move |__a0: metamodelica::Ref<SCode::Equation>, __a1: _, __a2: _| foldEquationsExps(&__a0, __a1, __a2),
                inFunc.clone(),
                outArg,
            )?;
            for mut branch in &*__inEquation_elseBranches.clone() {
                (exp, eql) = branch.clone();
                outArg = inFunc(exp, outArg)?;
                outArg = List::fold1(
                    &eql,
                    &move |__a0: metamodelica::Ref<SCode::Equation>, __a1: _, __a2: _| {
                        foldEquationsExps(&__a0, __a1, __a2)
                    },
                    inFunc.clone(),
                    outArg,
                )?;
            }
            outArg
        }
        SCode::Equation::EQ_ASSERT {
            condition: __inEquation_condition,
            level: __inEquation_level,
            message: __inEquation_message,
            ..
        } => {
            outArg = inFunc(__inEquation_condition.clone(), outArg)?;
            outArg = inFunc(__inEquation_message.clone(), outArg)?;
            outArg = inFunc(__inEquation_level.clone(), outArg)?;
            outArg
        }
        SCode::Equation::EQ_TERMINATE {
            message: __inEquation_message,
            ..
        } => inFunc(__inEquation_message.clone(), outArg)?,
        SCode::Equation::EQ_REINIT {
            cref: __inEquation_cref,
            expReinit: __inEquation_expReinit,
            ..
        } => {
            outArg = inFunc(__inEquation_cref.clone(), outArg)?;
            outArg = inFunc(__inEquation_expReinit.clone(), outArg)?;
            outArg
        }
        SCode::Equation::EQ_NORETCALL {
            exp: __inEquation_exp, ..
        } => inFunc(__inEquation_exp.clone(), outArg)?,
    });
    Ok(outArg)
}

pub fn foldStatementsExps<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inStatement: &metamodelica::Ref<SCode::Statement>,
    mut inFunc: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<ArgT> + 'static>,
    mut inArg: ArgT,
) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<ArgT> + 'static>;

    let mut outArg: ArgT = inArg;
    outArg = (match &**inStatement {
        SCode::Statement::ALG_ASSIGN {
            assignComponent: __inStatement_assignComponent,
            value: __inStatement_value,
            ..
        } => {
            outArg = inFunc(__inStatement_assignComponent.clone(), outArg)?;
            outArg = inFunc(__inStatement_value.clone(), outArg)?;
            outArg
        }
        SCode::Statement::ALG_IF {
            boolExpr: __inStatement_boolExpr,
            elseBranch: __inStatement_elseBranch,
            elseIfBranch: __inStatement_elseIfBranch,
            trueBranch: __inStatement_trueBranch,
            ..
        } => {
            let mut exp: metamodelica::Ref<Absyn::Exp>;
            let mut stmts: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
            outArg = inFunc(__inStatement_boolExpr.clone(), outArg)?;
            outArg = List::fold1(
                metamodelica::AsArg::as_arg(&__inStatement_trueBranch),
                &move |__a0: metamodelica::Ref<SCode::Statement>, __a1: _, __a2: _| {
                    foldStatementsExps(&__a0, __a1, __a2)
                },
                inFunc.clone(),
                outArg,
            )?;
            for mut branch in &*__inStatement_elseIfBranch.clone() {
                (exp, stmts) = branch.clone();
                outArg = inFunc(exp, outArg)?;
                outArg = List::fold1(
                    &stmts,
                    &move |__a0: metamodelica::Ref<SCode::Statement>, __a1: _, __a2: _| {
                        foldStatementsExps(&__a0, __a1, __a2)
                    },
                    inFunc.clone(),
                    outArg,
                )?;
            }
            outArg = List::fold1(
                metamodelica::AsArg::as_arg(&__inStatement_elseBranch),
                &move |__a0: metamodelica::Ref<SCode::Statement>, __a1: _, __a2: _| {
                    foldStatementsExps(&__a0, __a1, __a2)
                },
                inFunc.clone(),
                outArg,
            )?;
            outArg
        }
        SCode::Statement::ALG_FOR {
            forBody: __inStatement_forBody,
            range: __inStatement_range,
            ..
        } => {
            let mut exp: metamodelica::Ref<Absyn::Exp>;
            if (__inStatement_range).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(__inStatement_range.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                exp = metamodelica::Own::own(__pa0);
                outArg = inFunc(exp, outArg)?;
            }
            List::fold1(
                metamodelica::AsArg::as_arg(&__inStatement_forBody),
                &move |__a0: metamodelica::Ref<SCode::Statement>, __a1: _, __a2: _| {
                    foldStatementsExps(&__a0, __a1, __a2)
                },
                inFunc.clone(),
                outArg,
            )?
        }
        SCode::Statement::ALG_PARFOR {
            parforBody: __inStatement_parforBody,
            range: __inStatement_range,
            ..
        } => {
            let mut exp: metamodelica::Ref<Absyn::Exp>;
            if (__inStatement_range).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(__inStatement_range.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                exp = metamodelica::Own::own(__pa0);
                outArg = inFunc(exp, outArg)?;
            }
            List::fold1(
                metamodelica::AsArg::as_arg(&__inStatement_parforBody),
                &move |__a0: metamodelica::Ref<SCode::Statement>, __a1: _, __a2: _| {
                    foldStatementsExps(&__a0, __a1, __a2)
                },
                inFunc.clone(),
                outArg,
            )?
        }
        SCode::Statement::ALG_WHILE {
            boolExpr: __inStatement_boolExpr,
            whileBody: __inStatement_whileBody,
            ..
        } => {
            outArg = inFunc(__inStatement_boolExpr.clone(), outArg)?;
            List::fold1(
                metamodelica::AsArg::as_arg(&__inStatement_whileBody),
                &move |__a0: metamodelica::Ref<SCode::Statement>, __a1: _, __a2: _| {
                    foldStatementsExps(&__a0, __a1, __a2)
                },
                inFunc.clone(),
                outArg,
            )?
        }
        SCode::Statement::ALG_WHEN_A {
            branches: __inStatement_branches,
            ..
        } => {
            let mut exp: metamodelica::Ref<Absyn::Exp>;
            let mut stmts: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
            for mut branch in &*__inStatement_branches.clone() {
                (exp, stmts) = branch.clone();
                outArg = inFunc(exp, outArg)?;
                outArg = List::fold1(
                    &stmts,
                    &move |__a0: metamodelica::Ref<SCode::Statement>, __a1: _, __a2: _| {
                        foldStatementsExps(&__a0, __a1, __a2)
                    },
                    inFunc.clone(),
                    outArg,
                )?;
            }
            outArg
        }
        SCode::Statement::ALG_ASSERT {
            condition: __inStatement_condition,
            level: __inStatement_level,
            message: __inStatement_message,
            ..
        } => {
            outArg = inFunc(__inStatement_condition.clone(), outArg)?;
            outArg = inFunc(__inStatement_message.clone(), outArg)?;
            outArg = inFunc(__inStatement_level.clone(), outArg)?;
            outArg
        }
        SCode::Statement::ALG_TERMINATE {
            message: __inStatement_message,
            ..
        } => inFunc(__inStatement_message.clone(), outArg)?,
        SCode::Statement::ALG_REINIT {
            cref: __inStatement_cref,
            newValue: __inStatement_newValue,
            ..
        } => {
            outArg = inFunc(__inStatement_cref.clone(), outArg)?;
            inFunc(__inStatement_newValue.clone(), outArg)?
        }
        SCode::Statement::ALG_NORETCALL {
            exp: __inStatement_exp, ..
        } => inFunc(__inStatement_exp.clone(), outArg)?,
        SCode::Statement::ALG_FAILURE {
            stmts: __inStatement_stmts,
            ..
        } => List::fold1(
            metamodelica::AsArg::as_arg(&__inStatement_stmts),
            &move |__a0: metamodelica::Ref<SCode::Statement>, __a1: _, __a2: _| foldStatementsExps(&__a0, __a1, __a2),
            inFunc.clone(),
            outArg,
        )?,
        SCode::Statement::ALG_TRY {
            body: __inStatement_body,
            elseBody: __inStatement_elseBody,
            ..
        } => {
            outArg = List::fold1(
                metamodelica::AsArg::as_arg(&__inStatement_body),
                &move |__a0: metamodelica::Ref<SCode::Statement>, __a1: _, __a2: _| {
                    foldStatementsExps(&__a0, __a1, __a2)
                },
                inFunc.clone(),
                outArg,
            )?;
            List::fold1(
                metamodelica::AsArg::as_arg(&__inStatement_elseBody),
                &move |__a0: metamodelica::Ref<SCode::Statement>, __a1: _, __a2: _| {
                    foldStatementsExps(&__a0, __a1, __a2)
                },
                inFunc.clone(),
                outArg,
            )?
        }
        SCode::Statement::ALG_RETURN { .. } => outArg,
        SCode::Statement::ALG_BREAK { .. } => outArg,
        SCode::Statement::ALG_CONTINUE { .. } => outArg,
    });
    Ok(outArg)
}

pub(crate) fn mapFoldEquationsList<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut eql: metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    mut traverser: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<SCode::Equation>,
                ArgT,
            ) -> Result<(metamodelica::Ref<SCode::Equation>, ArgT)>
            + 'static,
    >,
    mut arg: ArgT,
) -> Result<(metamodelica::List<metamodelica::Ref<SCode::Equation>>, ArgT)> {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<SCode::Equation>,
                ArgT,
            ) -> Result<(metamodelica::Ref<SCode::Equation>, ArgT)>
            + 'static,
    >;

    let mut eql: metamodelica::List<metamodelica::Ref<SCode::Equation>> = eql;
    let mut arg: ArgT = arg;
    (eql, arg) = List::mapFold(
        &eql,
        &({
            let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Equation>, _) -> Result<_> + 'static> =
                traverser.clone();
            move |__pe_a0, __pe_a2| mapFoldEquations(__pe_a0, __pe_b1.clone(), __pe_a2)
        }),
        arg,
    )?;
    Ok((eql, arg))
}

pub fn mapFoldEquations<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut eq: metamodelica::Ref<SCode::Equation>,
    mut traverser: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<SCode::Equation>,
                ArgT,
            ) -> Result<(metamodelica::Ref<SCode::Equation>, ArgT)>
            + 'static,
    >,
    mut arg: ArgT,
) -> Result<(metamodelica::Ref<SCode::Equation>, ArgT)> {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<SCode::Equation>,
                ArgT,
            ) -> Result<(metamodelica::Ref<SCode::Equation>, ArgT)>
            + 'static,
    >;

    let mut eq: metamodelica::Ref<SCode::Equation> = eq;
    let mut arg: ArgT = arg;
    (eq, arg) = traverser(eq, arg)?;
    (eq, arg) = (match &*eq {
        SCode::Equation::EQ_IF {
            condition: expl1,
            thenBranch: then_branch,
            elseBranch: else_branch,
            comment,
            info,
        } => {
            let mut then_branch = (*then_branch).clone();
            let mut else_branch = (*else_branch).clone();
            (then_branch, arg) = List::mapFold(
                metamodelica::AsArg::as_arg(&then_branch),
                &({
                    let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Equation>, _) -> Result<_> + 'static> =
                        traverser.clone();
                    move |__pe_a0, __pe_a2| mapFoldEquationsList(__pe_a0, __pe_b1.clone(), __pe_a2)
                }),
                arg,
            )?;
            (else_branch, arg) = mapFoldEquationsList(else_branch.clone(), traverser.clone(), arg)?;
            (
                metamodelica::Ref::new(SCode::Equation::EQ_IF {
                    condition: expl1.clone(),
                    thenBranch: then_branch.clone(),
                    elseBranch: else_branch.clone(),
                    comment: comment.clone(),
                    info: info.clone(),
                }),
                arg,
            )
        }
        SCode::Equation::EQ_FOR {
            eEquationLst: __eq_eEquationLst,
            ..
        } => {
            let mut eql: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
            (eql, arg) = mapFoldEquationsList(__eq_eEquationLst.clone(), traverser.clone(), arg)?;
            assign_variant_field!(eq => SCode::Equation::EQ_FOR; eEquationLst = eql);
            (eq, arg)
        }
        SCode::Equation::EQ_WHEN {
            condition: e1,
            eEquationLst: eql,
            elseBranches: else_when,
            comment,
            info,
        } => {
            let mut eql = (*eql).clone();
            let mut else_when = (*else_when).clone();
            (eql, arg) = mapFoldEquationsList(eql.clone(), traverser.clone(), arg)?;
            (else_when, arg) = List::mapFold(
                metamodelica::AsArg::as_arg(&else_when),
                &({
                    let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Equation>, _) -> Result<_> + 'static> =
                        traverser.clone();
                    move |__pe_a0, __pe_a2| mapFoldElseWhenEquations(__pe_a0, __pe_b1.clone(), __pe_a2)
                }),
                arg,
            )?;
            (
                metamodelica::Ref::new(SCode::Equation::EQ_WHEN {
                    condition: e1.clone(),
                    eEquationLst: eql.clone(),
                    elseBranches: else_when.clone(),
                    comment: comment.clone(),
                    info: info.clone(),
                }),
                arg,
            )
        }
        _ => (eq, arg),
    });
    Ok((eq, arg))
}

fn mapFoldElseWhenEquations<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut elseWhen: (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    ),
    mut traverser: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<SCode::Equation>,
                ArgT,
            ) -> Result<(metamodelica::Ref<SCode::Equation>, ArgT)>
            + 'static,
    >,
    mut arg: ArgT,
) -> Result<(
    (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    ),
    ArgT,
)> {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<SCode::Equation>,
                ArgT,
            ) -> Result<(metamodelica::Ref<SCode::Equation>, ArgT)>
            + 'static,
    >;

    let mut elseWhen: (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    ) = elseWhen;
    let mut arg: ArgT = arg;
    let mut exp: metamodelica::Ref<Absyn::Exp>;
    let mut eql: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    (exp, eql) = elseWhen;
    (eql, arg) = mapFoldEquationsList(eql, traverser.clone(), arg)?;
    elseWhen = (exp, eql);
    Ok((elseWhen, arg))
}

pub(crate) fn mapFoldEquationListExps<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inEquations: &metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    mut traverser: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<(metamodelica::Ref<Absyn::Exp>, ArgT)>
            + 'static,
    >,
    mut inArg: ArgT,
) -> Result<(metamodelica::List<metamodelica::Ref<SCode::Equation>>, ArgT)> {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<(metamodelica::Ref<Absyn::Exp>, ArgT)>
            + 'static,
    >;

    let mut outEquations: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    let mut outArg: ArgT;
    (outEquations, outArg) = List::map1Fold(inEquations, &mapFoldEquationExps, traverser.clone(), inArg)?;
    Ok((outEquations, outArg))
}

pub fn mapFoldEquationExps<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut eq: metamodelica::Ref<SCode::Equation>,
    mut traverser: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<(metamodelica::Ref<Absyn::Exp>, ArgT)>
            + 'static,
    >,
    mut arg: ArgT,
) -> Result<(metamodelica::Ref<SCode::Equation>, ArgT)> {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<(metamodelica::Ref<Absyn::Exp>, ArgT)>
            + 'static,
    >;

    let mut eq: metamodelica::Ref<SCode::Equation> = eq;
    let mut arg: ArgT = arg;
    (eq, arg) = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ SCode::Equation::EQ_IF { condition: expl1, thenBranch: then_branch, elseBranch: else_branch, comment, info } => {
            let mut expl1 = (*expl1).clone();
            (expl1, arg) = AbsynUtil::traverseExpList(expl1.clone(), traverser.clone(), arg)?;
            (metamodelica::Ref::new(SCode::Equation::EQ_IF { condition: expl1.clone(), thenBranch: then_branch.clone(), elseBranch: else_branch.clone(), comment: comment.clone(), info: info.clone() }), arg)
        },
        Deref @ SCode::Equation::EQ_EQUALS { expLeft: e1, expRight: e2, comment, info } => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (e1, arg) = traverser(e1.clone(), arg)?;
            (e2, arg) = traverser(e2.clone(), arg)?;
            (metamodelica::Ref::new(SCode::Equation::EQ_EQUALS { expLeft: e1.clone(), expRight: e2.clone(), comment: comment.clone(), info: info.clone() }), arg)
        },
        Deref @ SCode::Equation::EQ_PDE { expLeft: e1, expRight: e2, domain, comment, info } => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (e1, arg) = traverser(e1.clone(), arg)?;
            (e2, arg) = traverser(e2.clone(), arg)?;
            (metamodelica::Ref::new(SCode::Equation::EQ_PDE { expLeft: e1.clone(), expRight: e2.clone(), domain: domain.clone(), comment: comment.clone(), info: info.clone() }), arg)
        },
        Deref @ SCode::Equation::EQ_CONNECT { crefLeft: cr1, crefRight: cr2, comment, info } => {
            let mut cr1 = (*cr1).clone();
            let mut cr2 = (*cr2).clone();
            (cr1, arg) = mapFoldComponentRefExps(metamodelica::AsArg::as_arg(&cr1), traverser.clone(), arg)?;
            (cr2, arg) = mapFoldComponentRefExps(metamodelica::AsArg::as_arg(&cr2), traverser.clone(), arg)?;
            (metamodelica::Ref::new(SCode::Equation::EQ_CONNECT { crefLeft: cr1.clone(), crefRight: cr2.clone(), comment: comment.clone(), info: info.clone() }), arg)
        },
        Deref @ SCode::Equation::EQ_FOR { index, range: Some(e1), eEquationLst: eql, comment, info } => {
            let mut e1 = (*e1).clone();
            (e1, arg) = traverser(e1.clone(), arg)?;
            (metamodelica::Ref::new(SCode::Equation::EQ_FOR { index: index.clone(), range: Some(e1.clone()), eEquationLst: eql.clone(), comment: comment.clone(), info: info.clone() }), arg)
        },
        Deref @ SCode::Equation::EQ_WHEN { condition: e1, eEquationLst: eql, elseBranches: else_when, comment, info } => {
            let mut e1 = (*e1).clone();
            let mut else_when = (*else_when).clone();
            (e1, arg) = traverser(e1.clone(), arg)?;
            (else_when, arg) = List::map1Fold(metamodelica::AsArg::as_arg(&else_when), &move |__a0: (metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<SCode::Equation>>), __a1: _, __a2: _| mapFoldElseWhenExps(&__a0, metamodelica::arc_ref(&__a1), __a2), traverser.clone(), arg)?;
            (metamodelica::Ref::new(SCode::Equation::EQ_WHEN { condition: e1.clone(), eEquationLst: eql.clone(), elseBranches: else_when.clone(), comment: comment.clone(), info: info.clone() }), arg)
        },
        Deref @ SCode::Equation::EQ_ASSERT { condition: e1, message: e2, level: e3, comment, info } => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            let mut e3 = (*e3).clone();
            (e1, arg) = traverser(e1.clone(), arg)?;
            (e2, arg) = traverser(e2.clone(), arg)?;
            (e3, arg) = traverser(e3.clone(), arg)?;
            (metamodelica::Ref::new(SCode::Equation::EQ_ASSERT { condition: e1.clone(), message: e2.clone(), level: e3.clone(), comment: comment.clone(), info: info.clone() }), arg)
        },
        Deref @ SCode::Equation::EQ_TERMINATE { message: e1, comment, info } => {
            let mut e1 = (*e1).clone();
            (e1, arg) = traverser(e1.clone(), arg)?;
            (metamodelica::Ref::new(SCode::Equation::EQ_TERMINATE { message: e1.clone(), comment: comment.clone(), info: info.clone() }), arg)
        },
        Deref @ SCode::Equation::EQ_REINIT { cref: e1, expReinit: e2, comment, info } => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (e1, arg) = traverser(e1.clone(), arg)?;
            (e2, arg) = traverser(e2.clone(), arg)?;
            (metamodelica::Ref::new(SCode::Equation::EQ_REINIT { cref: e1.clone(), expReinit: e2.clone(), comment: comment.clone(), info: info.clone() }), arg)
        },
        Deref @ SCode::Equation::EQ_NORETCALL { exp: e1, comment, info } => {
            let mut e1 = (*e1).clone();
            (e1, arg) = traverser(e1.clone(), arg)?;
            (metamodelica::Ref::new(SCode::Equation::EQ_NORETCALL { exp: e1.clone(), comment: comment.clone(), info: info.clone() }), arg)
        },
        _ => {
            (eq, arg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eq, arg))
}

fn mapFoldComponentRefExps<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inCref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<(metamodelica::Ref<Absyn::Exp>, ArgT)>
            + 'static,
    >,
    mut inArg: ArgT,
) -> Result<(metamodelica::Ref<Absyn::ComponentRef>, ArgT)> {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<(metamodelica::Ref<Absyn::Exp>, ArgT)>
            + 'static,
    >;

    let mut outCref: metamodelica::Ref<Absyn::ComponentRef>;
    let mut outArg: ArgT;
    (outCref, outArg) = (match &**inCref {
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cr } => {
            let mut arg: ArgT;
            let mut cr = (*cr).clone();
            (cr, arg) = mapFoldComponentRefExps(metamodelica::AsArg::as_arg(&cr), inFunc.clone(), inArg)?;
            (AbsynUtil::crefMakeFullyQualified(cr.clone()), arg)
        }
        Absyn::ComponentRef::CREF_QUAL {
            name,
            subscripts: subs,
            componentRef: cr,
        } => {
            let mut arg: ArgT;
            let mut subs = (*subs).clone();
            let mut cr = (*cr).clone();
            (cr, arg) = mapFoldComponentRefExps(metamodelica::AsArg::as_arg(&cr), inFunc.clone(), inArg)?;
            (subs, arg) = List::map1Fold(
                metamodelica::AsArg::as_arg(&subs),
                &mapFoldSubscriptExps,
                inFunc.clone(),
                arg,
            )?;
            (
                metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL {
                    name: name.clone(),
                    subscripts: subs.clone(),
                    componentRef: cr.clone(),
                }),
                arg,
            )
        }
        Absyn::ComponentRef::CREF_IDENT { name, subscripts: subs } => {
            let mut arg: ArgT;
            let mut subs = (*subs).clone();
            (subs, arg) = List::map1Fold(
                metamodelica::AsArg::as_arg(&subs),
                &mapFoldSubscriptExps,
                inFunc.clone(),
                inArg,
            )?;
            (
                metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                    name: name.clone(),
                    subscripts: subs.clone(),
                }),
                arg,
            )
        }
        Absyn::ComponentRef::WILD { .. } => (inCref.clone(), inArg),
        _ => return Err("match: no arm matched"),
    });
    Ok((outCref, outArg))
}

fn mapFoldSubscriptExps<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inSubscript: metamodelica::Ref<Absyn::Subscript>,
    mut inFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<(metamodelica::Ref<Absyn::Exp>, ArgT)>
            + 'static,
    >,
    mut inArg: ArgT,
) -> Result<(metamodelica::Ref<Absyn::Subscript>, ArgT)> {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<(metamodelica::Ref<Absyn::Exp>, ArgT)>
            + 'static,
    >;

    let mut outSubscript: metamodelica::Ref<Absyn::Subscript>;
    let mut outArg: ArgT;
    (outSubscript, outArg) = (match &*inSubscript {
        Absyn::Subscript::SUBSCRIPT { subscript: sub_exp } => {
            let mut traverser = inFunc.clone();
            let mut arg = inArg.clone();
            let mut sub_exp = (*sub_exp).clone();
            (sub_exp, arg) = traverser(sub_exp.clone(), arg)?;
            (
                metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT {
                    subscript: sub_exp.clone(),
                }),
                arg,
            )
        }
        Absyn::Subscript::NOSUB { .. } => (inSubscript, inArg),
    });
    Ok((outSubscript, outArg))
}

fn mapFoldElseWhenExps<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inElseWhen: &(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    ),
    mut traverser: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<(metamodelica::Ref<Absyn::Exp>, ArgT)>,
    mut inArg: ArgT,
) -> Result<(
    (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    ),
    ArgT,
)> {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<(metamodelica::Ref<Absyn::Exp>, ArgT)>
            + 'static,
    >;

    let mut outElseWhen: (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    );
    let mut outArg: ArgT;
    let mut exp: metamodelica::Ref<Absyn::Exp>;
    let mut eql: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    (exp, eql) = inElseWhen.clone();
    (exp, outArg) = traverser(exp, inArg)?;
    outElseWhen = (exp, eql);
    Ok((outElseWhen, outArg))
}

fn mapFoldForIteratorExps<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inIterator: &metamodelica::Ref<Absyn::ForIterator>,
    mut inFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<(metamodelica::Ref<Absyn::Exp>, ArgT)>
            + 'static,
    >,
    mut inArg: ArgT,
) -> Result<(metamodelica::Ref<Absyn::ForIterator>, ArgT)> {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<(metamodelica::Ref<Absyn::Exp>, ArgT)>
            + 'static,
    >;

    let mut outIterator: metamodelica::Ref<Absyn::ForIterator>;
    let mut outArg: ArgT;
    (outIterator, outArg) = (::match_deref::match_deref! { match inIterator {
        Deref @ Absyn::ForIterator { name: ident, guardExp: None, range: None } => {
            let mut arg = inArg;
            (metamodelica::Ref::new(Absyn::ForIterator { name: ident.clone(), guardExp: None, range: None }), arg)
        },
        Deref @ Absyn::ForIterator { name: ident, guardExp: None, range: Some(range) } => {
            let mut traverser = inFunc.clone();
            let mut arg = inArg;
            let mut range = (*range).clone();
            (range, arg) = traverser(range.clone(), arg)?;
            (metamodelica::Ref::new(Absyn::ForIterator { name: ident.clone(), guardExp: None, range: Some(range.clone()) }), arg)
        },
        Deref @ Absyn::ForIterator { name: ident, guardExp: Some(guardExp), range: Some(range) } => {
            let mut traverser = inFunc.clone();
            let mut arg = inArg;
            let mut guardExp = (*guardExp).clone();
            let mut range = (*range).clone();
            (guardExp, arg) = traverser(guardExp.clone(), arg)?;
            (range, arg) = traverser(range.clone(), arg)?;
            (metamodelica::Ref::new(Absyn::ForIterator { name: ident.clone(), guardExp: Some(guardExp.clone()), range: Some(range.clone()) }), arg)
        },
        Deref @ Absyn::ForIterator { name: ident, guardExp: Some(guardExp), range: None } => {
            let mut traverser = inFunc.clone();
            let mut arg = inArg;
            let mut guardExp = (*guardExp).clone();
            (guardExp, arg) = traverser(guardExp.clone(), arg)?;
            (metamodelica::Ref::new(Absyn::ForIterator { name: ident.clone(), guardExp: Some(guardExp.clone()), range: None }), arg)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outIterator, outArg))
}

pub(crate) fn mapFoldStatementsList<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut statements: metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    mut traverser: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<SCode::Statement>,
                ArgT,
            ) -> Result<(metamodelica::Ref<SCode::Statement>, ArgT)>
            + 'static,
    >,
    mut arg: ArgT,
) -> Result<(metamodelica::List<metamodelica::Ref<SCode::Statement>>, ArgT)> {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<SCode::Statement>,
                ArgT,
            ) -> Result<(metamodelica::Ref<SCode::Statement>, ArgT)>
            + 'static,
    >;

    let mut statements: metamodelica::List<metamodelica::Ref<SCode::Statement>> = statements;
    let mut arg: ArgT = arg;
    (statements, arg) = List::mapFold(
        &statements,
        &({
            let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Statement>, _) -> Result<_> + 'static> =
                traverser.clone();
            move |__pe_a0, __pe_a2| mapFoldStatements(__pe_a0, __pe_b1.clone(), __pe_a2)
        }),
        arg,
    )?;
    Ok((statements, arg))
}

pub fn mapFoldStatements<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut stmt: metamodelica::Ref<SCode::Statement>,
    mut traverser: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<SCode::Statement>,
                ArgT,
            ) -> Result<(metamodelica::Ref<SCode::Statement>, ArgT)>
            + 'static,
    >,
    mut arg: ArgT,
) -> Result<(metamodelica::Ref<SCode::Statement>, ArgT)> {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<SCode::Statement>,
                ArgT,
            ) -> Result<(metamodelica::Ref<SCode::Statement>, ArgT)>
            + 'static,
    >;

    let mut stmt: metamodelica::Ref<SCode::Statement> = stmt;
    let mut arg: ArgT = arg;
    (stmt, arg) = traverser(stmt, arg)?;
    (stmt, arg) = (match &*stmt {
        SCode::Statement::ALG_IF {
            boolExpr: e,
            trueBranch: stmts1,
            elseIfBranch: branches,
            elseBranch: stmts2,
            comment,
            info,
        } => {
            let mut stmts1 = (*stmts1).clone();
            let mut branches = (*branches).clone();
            let mut stmts2 = (*stmts2).clone();
            (stmts1, arg) = mapFoldStatementsList(stmts1.clone(), traverser.clone(), arg)?;
            (branches, arg) = List::mapFold(
                metamodelica::AsArg::as_arg(&branches),
                &({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<SCode::Statement>, _) -> Result<_> + 'static,
                    > = traverser.clone();
                    move |__pe_a0, __pe_a2| mapFoldBranchStatements(__pe_a0, __pe_b1.clone(), __pe_a2)
                }),
                arg,
            )?;
            (stmts2, arg) = mapFoldStatementsList(stmts2.clone(), traverser.clone(), arg)?;
            (
                metamodelica::Ref::new(SCode::Statement::ALG_IF {
                    boolExpr: e.clone(),
                    trueBranch: stmts1.clone(),
                    elseIfBranch: branches.clone(),
                    elseBranch: stmts2.clone(),
                    comment: comment.clone(),
                    info: info.clone(),
                }),
                arg,
            )
        }
        SCode::Statement::ALG_FOR {
            index: iter,
            range,
            forBody: stmts1,
            comment,
            info,
        } => {
            let mut stmts1 = (*stmts1).clone();
            (stmts1, arg) = mapFoldStatementsList(stmts1.clone(), traverser.clone(), arg)?;
            (
                metamodelica::Ref::new(SCode::Statement::ALG_FOR {
                    index: iter.clone(),
                    range: range.clone(),
                    forBody: stmts1.clone(),
                    comment: comment.clone(),
                    info: info.clone(),
                }),
                arg,
            )
        }
        SCode::Statement::ALG_PARFOR {
            index: iter,
            range,
            parforBody: stmts1,
            comment,
            info,
        } => {
            let mut stmts1 = (*stmts1).clone();
            (stmts1, arg) = mapFoldStatementsList(stmts1.clone(), traverser.clone(), arg)?;
            (
                metamodelica::Ref::new(SCode::Statement::ALG_PARFOR {
                    index: iter.clone(),
                    range: range.clone(),
                    parforBody: stmts1.clone(),
                    comment: comment.clone(),
                    info: info.clone(),
                }),
                arg,
            )
        }
        SCode::Statement::ALG_WHILE {
            boolExpr: e,
            whileBody: stmts1,
            comment,
            info,
        } => {
            let mut stmts1 = (*stmts1).clone();
            (stmts1, arg) = mapFoldStatementsList(stmts1.clone(), traverser.clone(), arg)?;
            (
                metamodelica::Ref::new(SCode::Statement::ALG_WHILE {
                    boolExpr: e.clone(),
                    whileBody: stmts1.clone(),
                    comment: comment.clone(),
                    info: info.clone(),
                }),
                arg,
            )
        }
        SCode::Statement::ALG_WHEN_A {
            branches,
            comment,
            info,
        } => {
            let mut branches = (*branches).clone();
            (branches, arg) = List::mapFold(
                metamodelica::AsArg::as_arg(&branches),
                &({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<SCode::Statement>, _) -> Result<_> + 'static,
                    > = traverser.clone();
                    move |__pe_a0, __pe_a2| mapFoldBranchStatements(__pe_a0, __pe_b1.clone(), __pe_a2)
                }),
                arg,
            )?;
            (
                metamodelica::Ref::new(SCode::Statement::ALG_WHEN_A {
                    branches: branches.clone(),
                    comment: comment.clone(),
                    info: info.clone(),
                }),
                arg,
            )
        }
        SCode::Statement::ALG_FAILURE {
            stmts: stmts1,
            comment,
            info,
        } => {
            let mut stmts1 = (*stmts1).clone();
            (stmts1, arg) = mapFoldStatementsList(stmts1.clone(), traverser.clone(), arg)?;
            (
                metamodelica::Ref::new(SCode::Statement::ALG_FAILURE {
                    stmts: stmts1.clone(),
                    comment: comment.clone(),
                    info: info.clone(),
                }),
                arg,
            )
        }
        _ => (stmt, arg),
    });
    Ok((stmt, arg))
}

fn mapFoldBranchStatements<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut branch: (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    ),
    mut traverser: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<SCode::Statement>,
                ArgT,
            ) -> Result<(metamodelica::Ref<SCode::Statement>, ArgT)>
            + 'static,
    >,
    mut arg: ArgT,
) -> Result<(
    (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    ),
    ArgT,
)> {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<SCode::Statement>,
                ArgT,
            ) -> Result<(metamodelica::Ref<SCode::Statement>, ArgT)>
            + 'static,
    >;

    let mut branch: (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    ) = branch;
    let mut arg: ArgT = arg;
    let mut exp: metamodelica::Ref<Absyn::Exp>;
    let mut stmts: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
    (exp, stmts) = branch;
    (stmts, arg) = mapFoldStatementsList(stmts, traverser.clone(), arg)?;
    branch = (exp, stmts);
    Ok((branch, arg))
}

pub(crate) fn mapFoldStatementListExps<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inStatements: &metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    mut inFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<(metamodelica::Ref<Absyn::Exp>, ArgT)>
            + 'static,
    >,
    mut inArg: ArgT,
) -> Result<(metamodelica::List<metamodelica::Ref<SCode::Statement>>, ArgT)> {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<(metamodelica::Ref<Absyn::Exp>, ArgT)>
            + 'static,
    >;

    let mut outStatements: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
    let mut outArg: ArgT;
    (outStatements, outArg) = List::map1Fold(inStatements, &mapFoldStatementExps, inFunc.clone(), inArg)?;
    Ok((outStatements, outArg))
}

pub fn mapFoldStatementExps<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inStatement: metamodelica::Ref<SCode::Statement>,
    mut inFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<(metamodelica::Ref<Absyn::Exp>, ArgT)>
            + 'static,
    >,
    mut inArg: ArgT,
) -> Result<(metamodelica::Ref<SCode::Statement>, ArgT)> {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<(metamodelica::Ref<Absyn::Exp>, ArgT)>
            + 'static,
    >;

    let mut outStatement: metamodelica::Ref<SCode::Statement>;
    let mut outArg: ArgT;
    (outStatement, outArg) = (::match_deref::match_deref! { match &(inStatement.clone()) {
        Deref @ SCode::Statement::ALG_ASSIGN { assignComponent: e1, value: e2, comment, info } => {
            let mut traverser = inFunc.clone();
            let mut arg = inArg.clone();
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (e1, arg) = traverser(e1.clone(), arg)?;
            (e2, arg) = traverser(e2.clone(), arg)?;
            (metamodelica::Ref::new(SCode::Statement::ALG_ASSIGN { assignComponent: e1.clone(), value: e2.clone(), comment: comment.clone(), info: info.clone() }), arg)
        },
        Deref @ SCode::Statement::ALG_IF { boolExpr: e1, trueBranch: stmts1, elseIfBranch: branches, elseBranch: stmts2, comment, info } => {
            let mut traverser = inFunc.clone();
            let mut arg = inArg.clone();
            let mut e1 = (*e1).clone();
            let mut branches = (*branches).clone();
            (e1, arg) = traverser(e1.clone(), arg)?;
            (branches, arg) = List::map1Fold(metamodelica::AsArg::as_arg(&branches), &move |__a0: (metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<SCode::Statement>>), __a1: _, __a2: _| mapFoldBranchExps(&__a0, metamodelica::arc_ref(&__a1), __a2), traverser.clone(), arg)?;
            (metamodelica::Ref::new(SCode::Statement::ALG_IF { boolExpr: e1.clone(), trueBranch: stmts1.clone(), elseIfBranch: branches.clone(), elseBranch: stmts2.clone(), comment: comment.clone(), info: info.clone() }), arg)
        },
        Deref @ SCode::Statement::ALG_FOR { index: iterator, range: Some(e1), forBody: stmts1, comment, info } => {
            let mut traverser = inFunc.clone();
            let mut arg = inArg.clone();
            let mut e1 = (*e1).clone();
            (e1, arg) = traverser(e1.clone(), arg)?;
            (metamodelica::Ref::new(SCode::Statement::ALG_FOR { index: iterator.clone(), range: Some(e1.clone()), forBody: stmts1.clone(), comment: comment.clone(), info: info.clone() }), arg)
        },
        Deref @ SCode::Statement::ALG_PARFOR { index: iterator, range: Some(e1), parforBody: stmts1, comment, info } => {
            let mut traverser = inFunc.clone();
            let mut arg = inArg.clone();
            let mut e1 = (*e1).clone();
            (e1, arg) = traverser(e1.clone(), arg)?;
            (metamodelica::Ref::new(SCode::Statement::ALG_PARFOR { index: iterator.clone(), range: Some(e1.clone()), parforBody: stmts1.clone(), comment: comment.clone(), info: info.clone() }), arg)
        },
        Deref @ SCode::Statement::ALG_WHILE { boolExpr: e1, whileBody: stmts1, comment, info } => {
            let mut traverser = inFunc.clone();
            let mut arg = inArg.clone();
            let mut e1 = (*e1).clone();
            (e1, arg) = traverser(e1.clone(), arg)?;
            (metamodelica::Ref::new(SCode::Statement::ALG_WHILE { boolExpr: e1.clone(), whileBody: stmts1.clone(), comment: comment.clone(), info: info.clone() }), arg)
        },
        Deref @ SCode::Statement::ALG_WHEN_A { branches, comment, info } => {
            let mut traverser = inFunc.clone();
            let mut arg = inArg.clone();
            let mut branches = (*branches).clone();
            (branches, arg) = List::map1Fold(metamodelica::AsArg::as_arg(&branches), &move |__a0: (metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<SCode::Statement>>), __a1: _, __a2: _| mapFoldBranchExps(&__a0, metamodelica::arc_ref(&__a1), __a2), traverser.clone(), arg)?;
            (metamodelica::Ref::new(SCode::Statement::ALG_WHEN_A { branches: branches.clone(), comment: comment.clone(), info: info.clone() }), arg)
        },
        Deref @ SCode::Statement::ALG_ASSERT { .. } => {
            let mut traverser = inFunc.clone();
            let mut arg = inArg.clone();
            let mut e1: metamodelica::Ref<Absyn::Exp>;
            let mut e2: metamodelica::Ref<Absyn::Exp>;
            let mut e3: metamodelica::Ref<Absyn::Exp>;
            (e1, arg) = traverser(var_field!((*inStatement).condition, SCode::Statement::ALG_ASSERT).clone(), arg)?;
            (e2, arg) = traverser(var_field!((*inStatement).message, SCode::Statement::ALG_ASSERT).clone(), arg)?;
            (e3, arg) = traverser(var_field!((*inStatement).level, SCode::Statement::ALG_ASSERT).clone(), arg)?;
            (metamodelica::Ref::new(SCode::Statement::ALG_ASSERT { condition: e1, message: e2, level: e3, comment: var_field!((*inStatement).comment, SCode::Statement::ALG_ASSERT).clone(), info: var_field!((*inStatement).info, SCode::Statement::ALG_ASSERT).clone() }), arg)
        },
        Deref @ SCode::Statement::ALG_TERMINATE { .. } => {
            let mut traverser = inFunc.clone();
            let mut arg = inArg.clone();
            let mut e1: metamodelica::Ref<Absyn::Exp>;
            (e1, arg) = traverser(var_field!((*inStatement).message, SCode::Statement::ALG_TERMINATE).clone(), arg)?;
            (metamodelica::Ref::new(SCode::Statement::ALG_TERMINATE { message: e1, comment: var_field!((*inStatement).comment, SCode::Statement::ALG_TERMINATE).clone(), info: var_field!((*inStatement).info, SCode::Statement::ALG_TERMINATE).clone() }), arg)
        },
        Deref @ SCode::Statement::ALG_REINIT { .. } => {
            let mut traverser = inFunc.clone();
            let mut arg = inArg.clone();
            let mut e1: metamodelica::Ref<Absyn::Exp>;
            let mut e2: metamodelica::Ref<Absyn::Exp>;
            (e1, arg) = traverser(var_field!((*inStatement).cref, SCode::Statement::ALG_REINIT).clone(), arg)?;
            (e2, arg) = traverser(var_field!((*inStatement).newValue, SCode::Statement::ALG_REINIT).clone(), arg)?;
            (metamodelica::Ref::new(SCode::Statement::ALG_REINIT { cref: e1, newValue: e2, comment: var_field!((*inStatement).comment, SCode::Statement::ALG_REINIT).clone(), info: var_field!((*inStatement).info, SCode::Statement::ALG_REINIT).clone() }), arg)
        },
        Deref @ SCode::Statement::ALG_NORETCALL { exp: e1, comment, info } => {
            let mut traverser = inFunc.clone();
            let mut arg = inArg.clone();
            let mut e1 = (*e1).clone();
            (e1, arg) = traverser(e1.clone(), arg)?;
            (metamodelica::Ref::new(SCode::Statement::ALG_NORETCALL { exp: e1.clone(), comment: comment.clone(), info: info.clone() }), arg)
        },
        _ => {
            (inStatement, inArg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outStatement, outArg))
}

fn mapFoldBranchExps<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inBranch: &(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    ),
    mut traverser: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<(metamodelica::Ref<Absyn::Exp>, ArgT)>,
    mut inArg: ArgT,
) -> Result<(
    (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    ),
    ArgT,
)> {
    pub type TraverseFunc<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, ArgT) -> Result<(metamodelica::Ref<Absyn::Exp>, ArgT)>
            + 'static,
    >;

    let mut outBranch: (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    );
    let mut outArg: ArgT;
    let mut arg: ArgT;
    let mut exp: metamodelica::Ref<Absyn::Exp>;
    let mut stmts: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
    (exp, stmts) = inBranch.clone();
    (exp, outArg) = traverser(exp, inArg)?;
    outBranch = (exp, stmts);
    Ok((outBranch, outArg))
}

pub fn elementIsClass(mut el: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut b: bool;
    b = (match &**el {
        SCode::Element::CLASS { .. } => true,
        _ => false,
    });
    b
}

pub fn elementIsImport(mut inElement: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut outIsImport: bool;
    outIsImport = (match &**inElement {
        SCode::Element::IMPORT { .. } => true,
        _ => false,
    });
    outIsImport
}

pub fn elementIsPublicImport(mut el: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut b: bool;
    b = (match &**el {
        SCode::Element::IMPORT {
            visibility: SCode::Visibility::PUBLIC { .. },
            ..
        } => true,
        _ => false,
    });
    b
}

pub fn elementIsProtectedImport(mut el: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut b: bool;
    b = (match &**el {
        SCode::Element::IMPORT {
            visibility: SCode::Visibility::PROTECTED { .. },
            ..
        } => true,
        _ => false,
    });
    b
}

pub(crate) fn getElementClass(mut el: metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut cl: metamodelica::Ref<SCode::Element>;
    cl = (match &*el {
        SCode::Element::CLASS { .. } => el,
        _ => return Err("fail"),
    });
    Ok(cl)
}

pub static knownExternalCFunctions: std::sync::LazyLock<metamodelica::List<ArcStr>> = std::sync::LazyLock::new(|| {
    list![
        literal!("sin"),
        literal!("cos"),
        literal!("tan"),
        literal!("asin"),
        literal!("acos"),
        literal!("atan"),
        literal!("atan2"),
        literal!("sinh"),
        literal!("cosh"),
        literal!("tanh"),
        literal!("exp"),
        literal!("log"),
        literal!("log10"),
        literal!("sqrt")
    ]
});

pub fn isBuiltinFunction(
    mut cl: &metamodelica::Ref<SCode::Element>,
    mut inVars: metamodelica::List<ArcStr>,
    mut outVars: &metamodelica::List<ArcStr>,
) -> Result<ArcStr> {
    let mut name: ArcStr;
    name = (::match_deref::match_deref! { match (cl, outVars) {
        (Deref @ SCode::Element::CLASS { name: __esc_name, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { .. } }, classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { funcName: None, lang: Some(Deref @ "builtin"), .. }), .. }, .. }, _) => {
            name = (*__esc_name).clone();
            name.clone()
        },
        (Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { .. } }, classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { funcName: Some(__esc_name), lang: Some(Deref @ "builtin"), .. }), .. }, .. }, _) => {
            name = (*__esc_name).clone();
            name.clone()
        },
        (Deref @ SCode::Element::CLASS { name: __esc_name, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_PARALLEL_FUNCTION { .. } }, classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { funcName: None, lang: Some(Deref @ "builtin"), .. }), .. }, .. }, _) => {
            name = (*__esc_name).clone();
            name.clone()
        },
        (Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_PARALLEL_FUNCTION { .. } }, classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { funcName: Some(__esc_name), lang: Some(Deref @ "builtin"), .. }), .. }, .. }, _) => {
            name = (*__esc_name).clone();
            name.clone()
        },
        (Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { .. } }, classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { funcName: Some(__esc_name), lang: Some(Deref @ "C"), output_: Some(Deref @ Absyn::ComponentRef::CREF_IDENT { name: outVar2, subscripts: Deref @ metamodelica::ListNode::Nil }), args, .. }), .. }, .. }, Deref @ metamodelica::ListNode::Cons { head: outVar1, tail: Deref @ metamodelica::ListNode::Nil }) => {
            name = (*__esc_name).clone();
            let mut argsStr: metamodelica::List<ArcStr>;
            let true = (listMember(name.clone(), knownExternalCFunctions.clone())) else { return Err("pattern mismatch") };
            let true = (metamodelica::stringEq(&outVar2, &outVar1)) else { return Err("pattern mismatch") };
            argsStr = List::mapMap(args.clone(), &move |__a0: metamodelica::Ref<Absyn::Exp>| AbsynUtil::expCref(&__a0), &move |__a0: metamodelica::Ref<Absyn::ComponentRef>| AbsynUtil::crefIdent(&__a0))?;
            let true = (argsStr == inVars) else { return Err("pattern mismatch") };
            name.clone()
        },
        (Deref @ SCode::Element::CLASS { name: __esc_name, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { .. } }, classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { funcName: None, lang: Some(Deref @ "C"), .. }), .. }, .. }, _) => {
            name = (*__esc_name).clone();
            let true = (listMember(name.clone(), knownExternalCFunctions.clone())) else { return Err("pattern mismatch") };
            name.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(name)
}

pub fn getEquationInfo(mut inEquation: &metamodelica::Ref<SCode::Equation>) -> SourceInfo {
    let mut info: SourceInfo;
    info = (match &**inEquation {
        SCode::Equation::EQ_IF {
            info: __inEquation_info,
            ..
        } => __inEquation_info.clone(),
        SCode::Equation::EQ_EQUALS {
            info: __inEquation_info,
            ..
        } => __inEquation_info.clone(),
        SCode::Equation::EQ_PDE {
            info: __inEquation_info,
            ..
        } => __inEquation_info.clone(),
        SCode::Equation::EQ_CONNECT {
            info: __inEquation_info,
            ..
        } => __inEquation_info.clone(),
        SCode::Equation::EQ_FOR {
            info: __inEquation_info,
            ..
        } => __inEquation_info.clone(),
        SCode::Equation::EQ_WHEN {
            info: __inEquation_info,
            ..
        } => __inEquation_info.clone(),
        SCode::Equation::EQ_ASSERT {
            info: __inEquation_info,
            ..
        } => __inEquation_info.clone(),
        SCode::Equation::EQ_TERMINATE {
            info: __inEquation_info,
            ..
        } => __inEquation_info.clone(),
        SCode::Equation::EQ_REINIT {
            info: __inEquation_info,
            ..
        } => __inEquation_info.clone(),
        SCode::Equation::EQ_NORETCALL {
            info: __inEquation_info,
            ..
        } => __inEquation_info.clone(),
    });
    info
}

pub fn getStatementInfo(mut inStatement: &metamodelica::Ref<SCode::Statement>) -> Result<SourceInfo> {
    let mut outInfo: SourceInfo;
    outInfo = (match &**inStatement {
        SCode::Statement::ALG_ASSIGN {
            info: __inStatement_info,
            ..
        } => __inStatement_info.clone(),
        SCode::Statement::ALG_IF {
            info: __inStatement_info,
            ..
        } => __inStatement_info.clone(),
        SCode::Statement::ALG_FOR {
            info: __inStatement_info,
            ..
        } => __inStatement_info.clone(),
        SCode::Statement::ALG_PARFOR {
            info: __inStatement_info,
            ..
        } => __inStatement_info.clone(),
        SCode::Statement::ALG_WHILE {
            info: __inStatement_info,
            ..
        } => __inStatement_info.clone(),
        SCode::Statement::ALG_WHEN_A {
            info: __inStatement_info,
            ..
        } => __inStatement_info.clone(),
        SCode::Statement::ALG_ASSERT {
            info: __inStatement_info,
            ..
        } => __inStatement_info.clone(),
        SCode::Statement::ALG_TERMINATE {
            info: __inStatement_info,
            ..
        } => __inStatement_info.clone(),
        SCode::Statement::ALG_REINIT {
            info: __inStatement_info,
            ..
        } => __inStatement_info.clone(),
        SCode::Statement::ALG_NORETCALL {
            info: __inStatement_info,
            ..
        } => __inStatement_info.clone(),
        SCode::Statement::ALG_RETURN {
            info: __inStatement_info,
            ..
        } => __inStatement_info.clone(),
        SCode::Statement::ALG_BREAK {
            info: __inStatement_info,
            ..
        } => __inStatement_info.clone(),
        SCode::Statement::ALG_FAILURE {
            info: __inStatement_info,
            ..
        } => __inStatement_info.clone(),
        SCode::Statement::ALG_TRY {
            info: __inStatement_info,
            ..
        } => __inStatement_info.clone(),
        SCode::Statement::ALG_CONTINUE {
            info: __inStatement_info,
            ..
        } => __inStatement_info.clone(),
        _ => {
            Error::addInternalError(
                literal!("SCodeUtil.getStatementInfo failed"),
                metamodelica::sourceInfo!("FrontEnd/SCodeUtil.mo"),
            )?;
            Absyn::dummyInfo.clone()
        }
    });
    Ok(outInfo)
}

pub fn prependSubModToMod(
    mut subMod: metamodelica::Ref<SCode::SubMod>,
    mut r#mod: metamodelica::Ref<SCode::Mod>,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut r#mod: metamodelica::Ref<SCode::Mod> = r#mod;
    r#mod = (match &*r#mod {
        SCode::Mod::NOMOD { .. } => metamodelica::Ref::new(SCode::Mod::MOD {
            finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
            eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
            subModLst: list![subMod],
            binding: None,
            comment: None,
            info: Error::dummyInfo.clone(),
        }),
        SCode::Mod::MOD {
            subModLst: __mod_subModLst,
            ..
        } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = metamodelica::cons(subMod, __mod_subModLst.clone()));
            r#mod
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(r#mod)
}

pub fn addElementToClass(
    mut inElement: metamodelica::Ref<SCode::Element>,
    mut inClassDef: metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut outClassDef: metamodelica::Ref<SCode::Element>;
    let mut cdef: metamodelica::Ref<SCode::ClassDef>;
    let __pa0 = ::match_deref::match_deref! { match &(inClassDef.clone()) {
        Deref @ SCode::Element::CLASS { classDef: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cdef = metamodelica::Own::own(__pa0);
    cdef = addElementToCompositeClassDef(inElement, cdef)?;
    outClassDef = setClassDef(cdef, inClassDef)?;
    Ok(outClassDef)
}

pub(crate) fn addElementToCompositeClassDef(
    mut element: metamodelica::Ref<SCode::Element>,
    mut classDef: metamodelica::Ref<SCode::ClassDef>,
) -> Result<metamodelica::Ref<SCode::ClassDef>> {
    let mut classDef: metamodelica::Ref<SCode::ClassDef> = classDef;
    let () = (match &*classDef {
        SCode::ClassDef::PARTS {
            elementLst: __classDef_elementLst,
            ..
        } => {
            assign_variant_field!(classDef => SCode::ClassDef::PARTS; elementLst = metamodelica::cons(element, __classDef_elementLst.clone()));
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(classDef)
}

pub fn visibilityBool(mut inVisibility: SCode::Visibility) -> bool {
    let mut bVisibility: bool;
    bVisibility = (match inVisibility {
        SCode::Visibility::PUBLIC { .. } => true,
        SCode::Visibility::PROTECTED { .. } => false,
    });
    bVisibility
}

pub(crate) fn boolVisibility(mut inBoolVisibility: bool) -> SCode::Visibility {
    let mut outVisibility: SCode::Visibility;
    outVisibility = (match inBoolVisibility {
        true => openmodelica_frontend_types::SCode::Visibility::PUBLIC,
        false => openmodelica_frontend_types::SCode::Visibility::PROTECTED,
    });
    outVisibility
}

pub(crate) fn visibilityEqual(mut inVisibility1: SCode::Visibility, mut inVisibility2: SCode::Visibility) -> bool {
    let mut outEqual: bool;
    outEqual = (match (inVisibility1, inVisibility2) {
        (SCode::Visibility::PUBLIC { .. }, SCode::Visibility::PUBLIC { .. }) => true,
        (SCode::Visibility::PROTECTED { .. }, SCode::Visibility::PROTECTED { .. }) => true,
        _ => false,
    });
    outEqual
}

pub fn eachBool(mut inEach: SCode::Each) -> bool {
    let mut bEach: bool;
    bEach = (match inEach {
        SCode::Each::EACH { .. } => true,
        SCode::Each::NOT_EACH { .. } => false,
    });
    bEach
}

pub(crate) fn boolEach(mut inBoolEach: bool) -> SCode::Each {
    let mut outEach: SCode::Each;
    outEach = (match inBoolEach {
        true => openmodelica_frontend_types::SCode::Each::EACH,
        false => openmodelica_frontend_types::SCode::Each::NOT_EACH,
    });
    outEach
}

pub(crate) fn prefixesRedeclare(mut inPrefixes: &metamodelica::Ref<SCode::Prefixes>) -> SCode::Redeclare {
    let mut outRedeclare: SCode::Redeclare;
    let __arc1 = &(*inPrefixes);
    let SCode::PREFIXES {
        redeclarePrefix: __pa0, ..
    } = &**__arc1;
    outRedeclare = metamodelica::Own::own(__pa0);
    outRedeclare
}

pub(crate) fn prefixesSetRedeclare(
    mut prefixes: metamodelica::Ref<SCode::Prefixes>,
    mut inRedeclare: SCode::Redeclare,
) -> metamodelica::Ref<SCode::Prefixes> {
    let mut prefixes: metamodelica::Ref<SCode::Prefixes> = prefixes;
    assign_field!(prefixes.redeclarePrefix = inRedeclare);
    prefixes
}

pub(crate) fn prefixesSetReplaceable(
    mut prefixes: metamodelica::Ref<SCode::Prefixes>,
    mut inReplaceable: metamodelica::Ref<SCode::Replaceable>,
) -> metamodelica::Ref<SCode::Prefixes> {
    let mut prefixes: metamodelica::Ref<SCode::Prefixes> = prefixes;
    assign_field!(prefixes.replaceablePrefix = inReplaceable);
    prefixes
}

pub fn redeclareBool(mut inRedeclare: SCode::Redeclare) -> bool {
    let mut bRedeclare: bool;
    bRedeclare = (match inRedeclare {
        SCode::Redeclare::REDECLARE { .. } => true,
        SCode::Redeclare::NOT_REDECLARE { .. } => false,
    });
    bRedeclare
}

pub(crate) fn boolRedeclare(mut inBoolRedeclare: bool) -> SCode::Redeclare {
    let mut outRedeclare: SCode::Redeclare;
    outRedeclare = (match inBoolRedeclare {
        true => openmodelica_frontend_types::SCode::Redeclare::REDECLARE,
        false => openmodelica_frontend_types::SCode::Redeclare::NOT_REDECLARE,
    });
    outRedeclare
}

pub(crate) fn replaceableBool(mut inReplaceable: &metamodelica::Ref<SCode::Replaceable>) -> bool {
    let mut bReplaceable: bool;
    bReplaceable = (match &**inReplaceable {
        SCode::Replaceable::REPLACEABLE { .. } => true,
        SCode::Replaceable::NOT_REPLACEABLE { .. } => false,
    });
    bReplaceable
}

pub fn replaceableOptConstraint(
    mut inReplaceable: &metamodelica::Ref<SCode::Replaceable>,
) -> Option<metamodelica::Ref<SCode::ConstrainClass>> {
    let mut outOptConstrainClass: Option<metamodelica::Ref<SCode::ConstrainClass>>;
    outOptConstrainClass = (match &**inReplaceable {
        SCode::Replaceable::REPLACEABLE { cc } => cc.clone(),
        SCode::Replaceable::NOT_REPLACEABLE { .. } => None,
    });
    outOptConstrainClass
}

pub(crate) fn boolReplaceable(
    mut inBoolReplaceable: bool,
    mut inOptConstrainClass: Option<metamodelica::Ref<SCode::ConstrainClass>>,
) -> Result<metamodelica::Ref<SCode::Replaceable>> {
    let mut outReplaceable: metamodelica::Ref<SCode::Replaceable>;
    outReplaceable = (::match_deref::match_deref! { match &((inBoolReplaceable, inOptConstrainClass.clone())) {
        (true, _) => metamodelica::Ref::new(SCode::Replaceable::REPLACEABLE { cc: inOptConstrainClass }),
        (false, Some(_)) => {
            metamodelica::print(literal!("Ignoring constraint class because replaceable prefix is not present!\n"));
            openmodelica_frontend_types::SCode::Replaceable::interned_NOT_REPLACEABLE()
        },
        (false, _) => openmodelica_frontend_types::SCode::Replaceable::interned_NOT_REPLACEABLE(),
        _ => return Err("match: no arm matched"),
    } });
    Ok(outReplaceable)
}

pub fn encapsulatedBool(mut inEncapsulated: SCode::Encapsulated) -> bool {
    let mut bEncapsulated: bool;
    bEncapsulated = (match inEncapsulated {
        SCode::Encapsulated::ENCAPSULATED { .. } => true,
        SCode::Encapsulated::NOT_ENCAPSULATED { .. } => false,
    });
    bEncapsulated
}

pub(crate) fn boolEncapsulated(mut inBoolEncapsulated: bool) -> SCode::Encapsulated {
    let mut outEncapsulated: SCode::Encapsulated;
    outEncapsulated = (match inBoolEncapsulated {
        true => openmodelica_frontend_types::SCode::Encapsulated::ENCAPSULATED,
        false => openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
    });
    outEncapsulated
}

pub fn partialBool(mut inPartial: SCode::Partial) -> bool {
    let mut bPartial: bool;
    bPartial = (match inPartial {
        SCode::Partial::PARTIAL { .. } => true,
        SCode::Partial::NOT_PARTIAL { .. } => false,
    });
    bPartial
}

pub(crate) fn boolPartial(mut inBoolPartial: bool) -> SCode::Partial {
    let mut outPartial: SCode::Partial;
    outPartial = (match inBoolPartial {
        true => openmodelica_frontend_types::SCode::Partial::PARTIAL,
        false => openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
    });
    outPartial
}

pub fn prefixesFinal(mut inPrefixes: &metamodelica::Ref<SCode::Prefixes>) -> SCode::Final {
    let mut outFinal: SCode::Final;
    let __arc1 = &(*inPrefixes);
    let SCode::PREFIXES { finalPrefix: __pa0, .. } = &**__arc1;
    outFinal = metamodelica::Own::own(__pa0);
    outFinal
}

pub fn finalBool(mut inFinal: SCode::Final) -> bool {
    let mut bFinal: bool;
    bFinal = (match inFinal {
        SCode::Final::FINAL { .. } => true,
        SCode::Final::NOT_FINAL { .. } => false,
    });
    bFinal
}

pub fn finalEqual(mut inFinal1: SCode::Final, mut inFinal2: SCode::Final) -> bool {
    let mut bFinal: bool;
    bFinal = (match (inFinal1, inFinal2) {
        (SCode::Final::FINAL { .. }, SCode::Final::FINAL { .. }) => true,
        (SCode::Final::NOT_FINAL { .. }, SCode::Final::NOT_FINAL { .. }) => true,
        _ => false,
    });
    bFinal
}

pub(crate) fn boolFinal(mut inBoolFinal: bool) -> SCode::Final {
    let mut outFinal: SCode::Final;
    outFinal = if (inBoolFinal) {
        openmodelica_frontend_types::SCode::Final::FINAL
    } else {
        openmodelica_frontend_types::SCode::Final::NOT_FINAL
    };
    outFinal
}

pub(crate) fn connectorTypeEqual(
    mut inConnectorType1: SCode::ConnectorType,
    mut inConnectorType2: SCode::ConnectorType,
) -> Result<bool> {
    let mut outEqual: bool;
    outEqual = (match (inConnectorType1, inConnectorType2) {
        (SCode::ConnectorType::POTENTIAL { .. }, SCode::ConnectorType::POTENTIAL { .. }) => true,
        (SCode::ConnectorType::FLOW { .. }, SCode::ConnectorType::FLOW { .. }) => true,
        (SCode::ConnectorType::STREAM { .. }, SCode::ConnectorType::STREAM { .. }) => true,
        _ => return Err("match: no arm matched"),
    });
    Ok(outEqual)
}

pub(crate) fn potentialBool(mut inConnectorType: SCode::ConnectorType) -> bool {
    let mut outPotential: bool;
    outPotential = (match inConnectorType {
        SCode::ConnectorType::POTENTIAL { .. } => true,
        _ => false,
    });
    outPotential
}

pub fn flowBool(mut inConnectorType: SCode::ConnectorType) -> bool {
    let mut outFlow: bool;
    outFlow = (match inConnectorType {
        SCode::ConnectorType::FLOW { .. } => true,
        _ => false,
    });
    outFlow
}

pub(crate) fn boolFlow(mut inBoolFlow: bool) -> SCode::ConnectorType {
    let mut outFlow: SCode::ConnectorType;
    outFlow = (match inBoolFlow {
        true => openmodelica_frontend_types::SCode::ConnectorType::FLOW,
        _ => openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
    });
    outFlow
}

pub fn streamBool(mut inStream: SCode::ConnectorType) -> bool {
    let mut bStream: bool;
    bStream = (match inStream {
        SCode::ConnectorType::STREAM { .. } => true,
        _ => false,
    });
    bStream
}

pub(crate) fn boolStream(mut inBoolStream: bool) -> SCode::ConnectorType {
    let mut outStream: SCode::ConnectorType;
    outStream = (match inBoolStream {
        true => openmodelica_frontend_types::SCode::ConnectorType::STREAM,
        _ => openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
    });
    outStream
}

pub fn mergeAttributesFromClass(
    mut inAttributes: SCode::Attributes,
    mut inClass: &metamodelica::Ref<SCode::Element>,
) -> Result<SCode::Attributes> {
    let mut outAttributes: SCode::Attributes;
    outAttributes = (::match_deref::match_deref! { match inClass {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { attributes: cls_attr, .. }, .. } => {
            let mut attr: SCode::Attributes;
            let __pa0 = ::match_deref::match_deref! { match &(mergeAttributes(inAttributes, Some(cls_attr.clone()))?) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            attr = metamodelica::Own::own(__pa0);
            attr
        },
        _ => {
            inAttributes
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAttributes)
}

pub fn mergeAttributes(
    mut ele: SCode::Attributes,
    mut oEle: Option<SCode::Attributes>,
) -> Result<Option<SCode::Attributes>> {
    let mut outoEle: Option<SCode::Attributes>;
    outoEle = (match (ele.clone(), oEle) {
        (_, None) => Some(ele),
        (
            SCode::Attributes {
                arrayDims: ref ad1,
                connectorType: mut ct1,
                parallelism: mut p1,
                variability: mut v1,
                direction: mut d1,
                isField: mut isf1,
            },
            Some(SCode::Attributes {
                arrayDims: _,
                connectorType: mut ct2,
                parallelism: mut p2,
                variability: mut v2,
                direction: mut d2,
                isField: mut isf2,
            }),
        ) => {
            let mut p: SCode::Parallelism;
            let mut v: SCode::Variability;
            let mut d: Absyn::Direction;
            let mut isf: Absyn::IsField;
            let mut ad: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
            let mut ct: SCode::ConnectorType;
            ct = propagateConnectorType(ct1.clone(), ct2.clone());
            p = propagateParallelism(p1.clone(), p2.clone());
            v = propagateVariability(v1.clone(), v2.clone());
            d = propagateDirection(d1.clone(), d2.clone());
            isf = propagateIsField(isf1.clone(), isf2.clone());
            ad = ad1.clone();
            Some(SCode::Attributes {
                arrayDims: ad,
                connectorType: ct,
                parallelism: p,
                variability: v,
                direction: d,
                isField: isf,
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outoEle)
}

pub fn prefixesVisibility(mut inPrefixes: &metamodelica::Ref<SCode::Prefixes>) -> SCode::Visibility {
    let mut outVisibility: SCode::Visibility;
    let __arc1 = &(*inPrefixes);
    let SCode::PREFIXES { visibility: __pa0, .. } = &**__arc1;
    outVisibility = metamodelica::Own::own(__pa0);
    outVisibility
}

pub(crate) fn prefixesSetVisibility(
    mut prefixes: metamodelica::Ref<SCode::Prefixes>,
    mut inVisibility: SCode::Visibility,
) -> metamodelica::Ref<SCode::Prefixes> {
    let mut prefixes: metamodelica::Ref<SCode::Prefixes> = prefixes;
    assign_field!(prefixes.visibility = inVisibility);
    prefixes
}

pub fn eachEqual(mut each1: SCode::Each, mut each2: SCode::Each) -> bool {
    let mut equal: bool;
    equal = (match (each1, each2) {
        (SCode::Each::NOT_EACH { .. }, SCode::Each::NOT_EACH { .. }) => true,
        (SCode::Each::EACH { .. }, SCode::Each::EACH { .. }) => true,
        _ => false,
    });
    equal
}

pub(crate) fn replaceableEqual(
    mut r1: &metamodelica::Ref<SCode::Replaceable>,
    mut r2: &metamodelica::Ref<SCode::Replaceable>,
) -> bool {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match (r1, r2) {
        (Deref @ SCode::Replaceable::NOT_REPLACEABLE { .. }, Deref @ SCode::Replaceable::NOT_REPLACEABLE { .. }) => {
            true
        },
        (Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { constrainingClass: p1, modifier: m1, .. }) }, Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { constrainingClass: p2, modifier: m2, .. }) }) if (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&p1), metamodelica::AsArg::as_arg(&p2)) && modEqual(metamodelica::AsArg::as_arg(&m1), metamodelica::AsArg::as_arg(&m2))) => {
            true
        },
        (Deref @ SCode::Replaceable::REPLACEABLE { cc: None }, Deref @ SCode::Replaceable::REPLACEABLE { cc: None }) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    equal
}

pub fn prefixesEqual(
    mut prefixes1: &metamodelica::Ref<SCode::Prefixes>,
    mut prefixes2: &metamodelica::Ref<SCode::Prefixes>,
) -> bool {
    let mut equal: bool;
    equal = prefixes1.visibility.clone() == prefixes2.visibility.clone()
        && prefixes1.redeclarePrefix.clone() == prefixes2.redeclarePrefix.clone()
        && prefixes1.finalPrefix.clone() == prefixes2.finalPrefix.clone()
        && AbsynUtil::innerOuterEqual(prefixes1.innerOuter.clone(), prefixes2.innerOuter.clone())
        && replaceableEqual(&prefixes1.replaceablePrefix, &prefixes2.replaceablePrefix);
    equal
}

pub fn prefixesReplaceable(mut prefixes: &metamodelica::Ref<SCode::Prefixes>) -> metamodelica::Ref<SCode::Replaceable> {
    let mut repl: metamodelica::Ref<SCode::Replaceable>;
    let __arc1 = &(*prefixes);
    let SCode::PREFIXES {
        replaceablePrefix: __pa0,
        ..
    } = &**__arc1;
    repl = metamodelica::Own::own(__pa0);
    repl
}

pub fn elementPrefixes(
    mut inElement: &metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<SCode::Prefixes>> {
    let mut outPrefixes: metamodelica::Ref<SCode::Prefixes>;
    outPrefixes = (match &**inElement {
        SCode::Element::CLASS {
            prefixes: __inElement_prefixes,
            ..
        } => __inElement_prefixes.clone(),
        SCode::Element::COMPONENT {
            prefixes: __inElement_prefixes,
            ..
        } => __inElement_prefixes.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(outPrefixes)
}

pub fn setElementPrefixes(
    mut prefixes: metamodelica::Ref<SCode::Prefixes>,
    mut element: metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut element: metamodelica::Ref<SCode::Element> = element;
    let () = (match &*element {
        SCode::Element::CLASS { .. } => {
            assign_variant_field!(element => SCode::Element::CLASS; prefixes = prefixes);
            ()
        }
        SCode::Element::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; prefixes = prefixes);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(element)
}

pub fn isElementReplaceable(mut inElement: &metamodelica::Ref<SCode::Element>) -> Result<bool> {
    let mut isReplaceable: bool;
    let mut pf: metamodelica::Ref<SCode::Prefixes>;
    pf = elementPrefixes(inElement)?;
    isReplaceable = replaceableBool(&(prefixesReplaceable(&pf)));
    Ok(isReplaceable)
}

pub fn isElementRedeclare(mut inElement: &metamodelica::Ref<SCode::Element>) -> Result<bool> {
    let mut isRedeclare: bool;
    let mut pf: metamodelica::Ref<SCode::Prefixes>;
    pf = elementPrefixes(inElement)?;
    isRedeclare = redeclareBool(prefixesRedeclare(&pf));
    Ok(isRedeclare)
}

pub fn prefixesInnerOuter(mut inPrefixes: &metamodelica::Ref<SCode::Prefixes>) -> Absyn::InnerOuter {
    let mut outInnerOuter: Absyn::InnerOuter;
    let __arc1 = &(*inPrefixes);
    let SCode::PREFIXES { innerOuter: __pa0, .. } = &**__arc1;
    outInnerOuter = metamodelica::Own::own(__pa0);
    outInnerOuter
}

pub fn prefixesSetInnerOuter(
    mut prefixes: metamodelica::Ref<SCode::Prefixes>,
    mut innerOuter: Absyn::InnerOuter,
) -> metamodelica::Ref<SCode::Prefixes> {
    let mut prefixes: metamodelica::Ref<SCode::Prefixes> = prefixes;
    assign_field!(prefixes.innerOuter = innerOuter);
    prefixes
}

pub fn removeAttributeDimensions(mut attributes: SCode::Attributes) -> SCode::Attributes {
    let mut attributes: SCode::Attributes = attributes;
    attributes.arrayDims = metamodelica::nil();
    attributes
}

pub fn setAttributesDirection(mut attributes: SCode::Attributes, mut direction: Absyn::Direction) -> SCode::Attributes {
    let mut attributes: SCode::Attributes = attributes;
    attributes.direction = direction;
    attributes
}

pub fn attrVariability(mut attr: &SCode::Attributes) -> SCode::Variability {
    let mut var: SCode::Variability;
    var = (match attr.clone() {
        SCode::Attributes { variability: mut v, .. } => v.clone(),
    });
    var
}

pub(crate) fn setAttributesVariability(
    mut attributes: SCode::Attributes,
    mut variability: SCode::Variability,
) -> SCode::Attributes {
    let mut attributes: SCode::Attributes = attributes;
    attributes.variability = variability;
    attributes
}

pub(crate) fn isDerivedClassDef(mut inClassDef: &metamodelica::Ref<SCode::ClassDef>) -> bool {
    let mut isDerived: bool;
    isDerived = (match &**inClassDef {
        SCode::ClassDef::DERIVED { .. } => true,
        _ => false,
    });
    isDerived
}

pub fn isConnector(mut inRestriction: &SCode::Restriction) -> bool {
    let mut isConnector: bool;
    isConnector = (match inRestriction.clone() {
        SCode::Restriction::R_CONNECTOR { .. } => true,
        _ => false,
    });
    isConnector
}

pub fn removeBuiltinsFromTopScope(
    mut inProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut outProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    outProgram = List::filterOnTrue(
        inProgram,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<SCode::Element>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isNotBuiltinClass(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<bool> + 'static>),
    )?;
    Ok(outProgram)
}

fn isNotBuiltinClass(mut inClass: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inClass {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { lang: Some(Deref @ "builtin"), .. }), .. }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn getElementAnnotation(
    mut element: &metamodelica::Ref<SCode::Element>,
    mut name: &ArcStr,
) -> Option<metamodelica::Ref<SCode::Annotation>> {
    let mut outAnnotation: Option<metamodelica::Ref<SCode::Annotation>>;
    outAnnotation = (match &**element {
        SCode::Element::EXTENDS { ann: __element_ann, .. } => __element_ann.clone(),
        SCode::Element::CLASS { cmt: __element_cmt, .. } => __element_cmt.annotation_.clone(),
        SCode::Element::COMPONENT {
            comment: __element_comment,
            ..
        } => __element_comment.annotation_.clone(),
        _ => None,
    });
    outAnnotation
}

pub fn lookupAnnotation(
    mut ann: &metamodelica::Ref<SCode::Annotation>,
    mut name: &ArcStr,
) -> metamodelica::Ref<SCode::Mod> {
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    let mut submods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    let mut id: ArcStr;
    r#mod = (::match_deref::match_deref! { match ann {
        Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst: __esc_submods, .. } } => {
            submods = (*__esc_submods).clone();
            for mut sm in &*submods.clone() {
                let __arc2 = sm.clone();
                let SCode::NAMEMOD { ident: __pa0, r#mod: __pa1 } = &*__arc2;
                id = metamodelica::Own::own(__pa0);
                r#mod = metamodelica::Own::own(__pa1);
                if metamodelica::stringEq(&id, &name) {
                    return r#mod;
                }
            }
            openmodelica_frontend_types::SCode::Mod::interned_NOMOD()
        },
        _ => openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    r#mod
}

pub fn lookupAnnotationBinding(
    mut ann: &metamodelica::Ref<SCode::Annotation>,
    mut name: &ArcStr,
) -> Option<metamodelica::Ref<Absyn::Exp>> {
    let mut binding: Option<metamodelica::Ref<Absyn::Exp>>;
    binding = getModifierBinding(&(lookupAnnotation(ann, name)));
    binding
}

pub(crate) fn lookupBooleanAnnotation(
    mut ann: &metamodelica::Ref<SCode::Annotation>,
    mut name: &ArcStr,
) -> Option<bool> {
    let mut value: Option<bool>;
    let mut binding: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut bval: bool;
    binding = lookupAnnotationBinding(ann, name);
    value = (::match_deref::match_deref! { match &(binding) {
        Some(Deref @ Absyn::Exp::BOOL { value: __esc_bval }) => {
            bval = (*__esc_bval).clone();
            Some(bval.clone())
        },
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    value
}

pub fn lookupBooleanAnnotationMod(mut r#mod: &metamodelica::Ref<SCode::Mod>) -> Option<bool> {
    let mut value: Option<bool>;
    let mut binding: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut bval: bool;
    binding = getModifierBinding(r#mod);
    value = (::match_deref::match_deref! { match &(binding) {
        Some(Deref @ Absyn::Exp::BOOL { value: __esc_bval }) => {
            bval = (*__esc_bval).clone();
            Some(bval.clone())
        },
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    value
}

pub fn lookupAnnotations(
    mut ann: &metamodelica::Ref<SCode::Annotation>,
    mut name: &ArcStr,
) -> metamodelica::List<metamodelica::Ref<SCode::Mod>> {
    let mut mods: metamodelica::List<metamodelica::Ref<SCode::Mod>> = metamodelica::nil();
    let mut submods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    let mut id: ArcStr;
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    mods = (::match_deref::match_deref! { match ann {
        Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst: __esc_submods, .. } } => {
            submods = (*__esc_submods).clone();
            for mut sm in &*submods.clone() {
                let __arc2 = sm.clone();
                let SCode::NAMEMOD { ident: __pa0, r#mod: __pa1 } = &*__arc2;
                id = metamodelica::Own::own(__pa0);
                r#mod = metamodelica::Own::own(__pa1);
                if metamodelica::stringEq(&id, &name) {
                    mods = metamodelica::cons(r#mod, mods);
                }
            }
            mods
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    mods
}

pub fn lookupElementAnnotation(
    mut element: &metamodelica::Ref<SCode::Element>,
    mut name: &ArcStr,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    let mut ann: Option<metamodelica::Ref<SCode::Annotation>>;
    ann = getElementAnnotation(element, name);
    r#mod = if ((ann).is_some()) {
        lookupAnnotation(&(Util::getOption(ann)?), name)
    } else {
        openmodelica_frontend_types::SCode::Mod::interned_NOMOD()
    };
    Ok(r#mod)
}

pub fn lookupElementAnnotationBinding(
    mut element: &metamodelica::Ref<SCode::Element>,
    mut name: &ArcStr,
) -> Result<Option<metamodelica::Ref<Absyn::Exp>>> {
    let mut binding: Option<metamodelica::Ref<Absyn::Exp>>;
    binding = getModifierBinding(&(lookupElementAnnotation(element, name)?));
    Ok(binding)
}

pub fn hasBooleanNamedAnnotationInClass(
    mut inClass: &metamodelica::Ref<SCode::Element>,
    mut namedAnnotation: &ArcStr,
) -> bool {
    let mut hasAnn: bool;
    hasAnn = (::match_deref::match_deref! { match inClass {
        Deref @ SCode::Element::CLASS { cmt: Deref @ SCode::Comment { annotation_: Some(ann), .. }, .. } => {
            hasBooleanNamedAnnotation(metamodelica::AsArg::as_arg(&ann), namedAnnotation)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasAnn
}

pub fn hasBooleanNamedAnnotationInComponent(
    mut inComponent: &metamodelica::Ref<SCode::Element>,
    mut namedAnnotation: &ArcStr,
) -> bool {
    let mut hasAnn: bool;
    hasAnn = (::match_deref::match_deref! { match inComponent {
        Deref @ SCode::Element::COMPONENT { comment: Deref @ SCode::Comment { annotation_: Some(ann), .. }, .. } => {
            hasBooleanNamedAnnotation(metamodelica::AsArg::as_arg(&ann), namedAnnotation)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasAnn
}

pub fn commentAnnotation(mut cmt: &metamodelica::Ref<SCode::Comment>) -> Option<metamodelica::Ref<SCode::Annotation>> {
    let mut ann: Option<metamodelica::Ref<SCode::Annotation>> = cmt.annotation_.clone();
    ann
}

pub fn optCommentAnnotation(
    mut cmt: Option<metamodelica::Ref<SCode::Comment>>,
) -> Option<metamodelica::Ref<SCode::Annotation>> {
    let mut ann: Option<metamodelica::Ref<SCode::Annotation>>;
    ann = (::match_deref::match_deref! { match &(cmt) {
        Some(Deref @ SCode::Comment { annotation_: __esc_ann, .. }) => {
            ann = (*__esc_ann).clone();
            ann.clone()
        },
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ann
}

pub fn optCommentHasBooleanNamedAnnotation(
    mut comm: Option<metamodelica::Ref<SCode::Comment>>,
    mut annotationName: &ArcStr,
) -> bool {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match &(comm) {
        Some(Deref @ SCode::Comment { annotation_: Some(ann), .. }) => {
            hasBooleanNamedAnnotation(metamodelica::AsArg::as_arg(&ann), annotationName)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

pub fn commentHasBooleanNamedAnnotation(
    mut comm: &metamodelica::Ref<SCode::Comment>,
    mut annotationName: &ArcStr,
) -> bool {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match comm {
        Deref @ SCode::Comment { annotation_: Some(ann), .. } => {
            hasBooleanNamedAnnotation(metamodelica::AsArg::as_arg(&ann), annotationName)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

pub fn hasBooleanNamedAnnotation(mut inAnnotation: &metamodelica::Ref<SCode::Annotation>, mut inName: &ArcStr) -> bool {
    let mut outHasEntry: bool;
    let mut binding: Option<metamodelica::Ref<Absyn::Exp>>;
    binding = lookupAnnotationBinding(inAnnotation, inName);
    outHasEntry = (::match_deref::match_deref! { match &(binding) {
        Some(Deref @ Absyn::Exp::BOOL { value: true }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outHasEntry
}

pub fn optCommentHasBooleanNamedAnnotationFalse(
    mut comm: Option<metamodelica::Ref<SCode::Comment>>,
    mut annotationName: &ArcStr,
) -> bool {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match &(comm) {
        Some(Deref @ SCode::Comment { annotation_: Some(ann), .. }) => {
            hasBooleanNamedAnnotationFalse(metamodelica::AsArg::as_arg(&ann), annotationName)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

pub(crate) fn hasBooleanNamedAnnotationFalse(
    mut inAnnotation: &metamodelica::Ref<SCode::Annotation>,
    mut inName: &ArcStr,
) -> bool {
    let mut outHasEntry: bool;
    let mut binding: Option<metamodelica::Ref<Absyn::Exp>>;
    binding = lookupAnnotationBinding(inAnnotation, inName);
    outHasEntry = (::match_deref::match_deref! { match &(binding) {
        Some(Deref @ Absyn::Exp::BOOL { value: false }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outHasEntry
}

pub fn getEvaluateAnnotation(mut cmt: &metamodelica::Ref<SCode::Comment>) -> Option<bool> {
    let mut value: Option<bool>;
    let mut ann: metamodelica::Ref<SCode::Annotation>;
    value = (::match_deref::match_deref! { match cmt {
        Deref @ SCode::Comment { annotation_: Some(__esc_ann), .. } => {
            ann = (*__esc_ann).clone();
            lookupBooleanAnnotation(metamodelica::AsArg::as_arg(&ann), &(literal!("Evaluate")))
        },
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    value
}

pub(crate) fn appendAnnotationToCommentOption(
    mut inAnnotation: metamodelica::Ref<SCode::Annotation>,
    mut inComment: Option<metamodelica::Ref<SCode::Comment>>,
    mut check_replace: bool,
) -> Result<Option<metamodelica::Ref<SCode::Comment>>> {
    let mut outComment: Option<metamodelica::Ref<SCode::Comment>>;
    outComment = (::match_deref::match_deref! { match &(inComment) {
        Some(comment) => {
            Some(appendAnnotationToComment(inAnnotation, metamodelica::AsArg::as_arg(&comment), check_replace)?)
        },
        _ => {
            Some(metamodelica::Ref::new(SCode::Comment { annotation_: Some(inAnnotation), comment: None }))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outComment)
}

pub fn appendAnnotationToComment(
    mut inAnnotation: metamodelica::Ref<SCode::Annotation>,
    mut inComment: &metamodelica::Ref<SCode::Comment>,
    mut check_replace: bool,
) -> Result<metamodelica::Ref<SCode::Comment>> {
    fn isNotElem(
        mut r#mod: &metamodelica::Ref<SCode::SubMod>,
        mut mods: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    ) -> bool {
        let mut b: bool = true;
        for mut m in &**mods {
            if metamodelica::stringEq(&r#mod.ident, &m.ident) {
                b = false;
                return b;
            }
        }
        b
    }

    let mut outComment: metamodelica::Ref<SCode::Comment>;
    outComment = (::match_deref::match_deref! { match &((inAnnotation.clone(), inComment.clone())) {
        (_, Deref @ SCode::Comment { annotation_: None, comment: cmt }) => {
            metamodelica::Ref::new(SCode::Comment { annotation_: Some(inAnnotation), comment: cmt.clone() })
        },
        (Deref @ SCode::Annotation { modification: Deref @ SCode::Mod::MOD { subModLst: mods1, .. } }, Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: r#mod @ Deref @ SCode::Mod::MOD { .. } }), comment: cmt }) => {
            let mut r#mod = (*r#mod).clone();
            if !(check_replace) {
                assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = listAppend(mods1.clone(), var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone()));
            } else {
                assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = listAppend(mods1.clone(), List::filterOnTrue(var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone(), (std::sync::Arc::new({ let __pe_b1 = mods1.clone(); move |__pe_a0| Ok(isNotElem(&__pe_a0, &__pe_b1)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SCode::SubMod>) -> Result<bool> + 'static>))?));
            }
            metamodelica::Ref::new(SCode::Comment { annotation_: Some(metamodelica::Ref::new(SCode::Annotation { modification: r#mod.clone() })), comment: cmt.clone() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outComment)
}

pub fn getModifierInfo(mut inMod: &metamodelica::Ref<SCode::Mod>) -> SourceInfo {
    let mut outInfo: SourceInfo;
    outInfo = (match &**inMod {
        SCode::Mod::MOD { info, .. } => info.clone(),
        SCode::Mod::REDECL { element: el, .. } => elementInfo(el),
        SCode::Mod::BREAK_COMPONENT { info: __inMod_info } => __inMod_info.clone(),
        SCode::Mod::BREAK_CONNECT { info: __inMod_info, .. } => __inMod_info.clone(),
        _ => Absyn::dummyInfo.clone(),
    });
    outInfo
}

pub fn getModifierBinding(mut inMod: &metamodelica::Ref<SCode::Mod>) -> Option<metamodelica::Ref<Absyn::Exp>> {
    let mut outBinding: Option<metamodelica::Ref<Absyn::Exp>>;
    outBinding = (match &**inMod {
        SCode::Mod::MOD {
            binding: __inMod_binding,
            ..
        } => __inMod_binding.clone(),
        _ => None,
    });
    outBinding
}

pub(crate) fn setModifierBinding(
    mut binding: Option<metamodelica::Ref<Absyn::Exp>>,
    mut r#mod: metamodelica::Ref<SCode::Mod>,
) -> metamodelica::Ref<SCode::Mod> {
    let mut r#mod: metamodelica::Ref<SCode::Mod> = r#mod;
    let () = (match &*r#mod {
        SCode::Mod::MOD { .. } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD; binding = binding);
            ()
        }
        _ => (),
    });
    r#mod
}

pub(crate) fn getComponentCondition(
    mut element: &metamodelica::Ref<SCode::Element>,
) -> Option<metamodelica::Ref<Absyn::Exp>> {
    let mut condition: Option<metamodelica::Ref<Absyn::Exp>>;
    condition = (match &**element {
        SCode::Element::COMPONENT {
            condition: __element_condition,
            ..
        } => __element_condition.clone(),
        _ => None,
    });
    condition
}

pub(crate) fn removeComponentCondition(
    mut element: metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut element: metamodelica::Ref<SCode::Element> = element;
    let () = (match &*element {
        SCode::Element::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; condition = None);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(element)
}

pub(crate) fn isInnerComponent(mut inElement: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut outIsInner: bool;
    outIsInner = (::match_deref::match_deref! { match inElement {
        Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { innerOuter: io, .. }, .. } => {
            AbsynUtil::isInner(io.clone())
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsInner
}

pub fn makeElementProtected(mut element: metamodelica::Ref<SCode::Element>) -> metamodelica::Ref<SCode::Element> {
    let mut element: metamodelica::Ref<SCode::Element> = element;
    let mut prefixes: metamodelica::Ref<SCode::Prefixes>;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ SCode::Element::COMPONENT { prefixes: __esc_prefixes @ Deref @ SCode::Prefixes { visibility: SCode::Visibility::PUBLIC { .. }, .. }, .. } => {
            prefixes = (*__esc_prefixes).clone();
            assign_field!(prefixes.visibility = openmodelica_frontend_types::SCode::Visibility::PROTECTED);
            assign_variant_field!(element => SCode::Element::COMPONENT; prefixes = prefixes.clone());
            ()
        },
        Deref @ SCode::Element::EXTENDS { visibility: SCode::Visibility::PUBLIC { .. }, .. } => {
            assign_variant_field!(element => SCode::Element::EXTENDS; visibility = openmodelica_frontend_types::SCode::Visibility::PROTECTED);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    element
}

pub(crate) fn isElementPublic(mut inElement: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut outIsPublic: bool;
    outIsPublic = visibilityBool(elementVisibility(inElement));
    outIsPublic
}

pub fn isElementProtected(mut inElement: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut outIsProtected: bool;
    outIsProtected = !(visibilityBool(elementVisibility(inElement)));
    outIsProtected
}

pub fn isElementEncapsulated(mut inElement: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut outIsEncapsulated: bool;
    outIsEncapsulated = (match &**inElement {
        SCode::Element::CLASS {
            encapsulatedPrefix: SCode::Encapsulated::ENCAPSULATED { .. },
            ..
        } => true,
        _ => false,
    });
    outIsEncapsulated
}

pub(crate) fn getElementsFromElement<'__b>(
    mut inProgram: &'__b metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inElement: metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inElement) {
            Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: els, .. }, .. } => {
                return Ok(els.clone())
            },
            Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { composition: Deref @ SCode::ClassDef::PARTS { elementLst: els, .. }, .. }, .. } => {
                return Ok(els.clone())
            },
            Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: p, .. }, .. }, .. } => {
                let mut els: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                let mut e: metamodelica::Ref<SCode::Element>;
                e = getElementWithPath(inProgram.clone(), metamodelica::AsArg::as_arg(&p))?;
                { (inProgram, inElement) = (inProgram, e); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getElementWithId(
    mut inProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inId: ArcStr,
) -> Result<metamodelica::Ref<SCode::Element>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inProgram, inId)) {
            (Deref @ metamodelica::ListNode::Cons { head: e @ Deref @ SCode::Element::CLASS { name: n, .. }, tail: _ }, i) if (stringEq(&n, &i)) => {
                return Ok(e.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: e @ Deref @ SCode::Element::COMPONENT { name: n, .. }, tail: _ }, i) if (stringEq(&n, &i)) => {
                return Ok(e.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: e @ Deref @ SCode::Element::EXTENDS { baseClassPath: p, .. }, tail: _ }, i) if (stringEq(&(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?), &i)) => {
                return Ok(e.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, i) => {
                { (inProgram, inId) = (rest.clone(), i.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn getElementWithPath<'__b>(
    mut inProgram: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut inPath: &'__b metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    '__tco: loop {
        match &**inPath {
            Absyn::Path::FULLYQUALIFIED { path: p } => {
                (inProgram, inPath) = (inProgram, p);
                continue '__tco;
            }
            Absyn::Path::IDENT { name: i } => {
                let mut e: metamodelica::Ref<SCode::Element>;
                return Ok(getElementWithId(inProgram, i.clone())?);
            }
            Absyn::Path::QUALIFIED { name: i, path: p } => {
                let mut sp: metamodelica::List<metamodelica::Ref<SCode::Element>>;
                let mut e: metamodelica::Ref<SCode::Element>;
                e = getElementWithId(inProgram.clone(), i.clone())?;
                sp = getElementsFromElement(&inProgram, e)?;
                {
                    (inProgram, inPath) = (sp, p);
                    continue '__tco;
                }
            }
        }
    }
}

pub fn getElementName(mut e: &metamodelica::Ref<SCode::Element>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = (match &**e {
        SCode::Element::COMPONENT { name: __esc_s, .. } => {
            s = (*__esc_s).clone();
            s.clone()
        }
        SCode::Element::CLASS { name: __esc_s, .. } => {
            s = (*__esc_s).clone();
            s.clone()
        }
        SCode::Element::EXTENDS { baseClassPath: p, .. } => {
            AbsynUtil::pathString(p.clone(), literal!("."), true, false)?
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(s)
}

pub fn getElementTypePath(mut element: &metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut path: metamodelica::Ref<Absyn::Path>;
    path = (match &**element {
        SCode::Element::COMPONENT {
            typeSpec: __element_typeSpec,
            ..
        } => AbsynUtil::typeSpecPath(metamodelica::AsArg::as_arg(&__element_typeSpec)),
        SCode::Element::EXTENDS {
            baseClassPath: __element_baseClassPath,
            ..
        } => __element_baseClassPath.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(path)
}

pub(crate) fn setBaseClassPath(
    mut element: metamodelica::Ref<SCode::Element>,
    mut inBcPath: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut element: metamodelica::Ref<SCode::Element> = element;
    let () = (match &*element {
        SCode::Element::EXTENDS { .. } => {
            assign_variant_field!(element => SCode::Element::EXTENDS; baseClassPath = inBcPath);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(element)
}

pub fn getBaseClassPath(mut inE: &metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outBcPath: metamodelica::Ref<Absyn::Path>;
    let __pa0 = ::match_deref::match_deref! { match &((*inE)) {
        Deref @ SCode::Element::EXTENDS { baseClassPath: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outBcPath = metamodelica::Own::own(__pa0);
    Ok(outBcPath)
}

pub(crate) fn setComponentTypeSpec(
    mut element: metamodelica::Ref<SCode::Element>,
    mut typeSpec: metamodelica::Ref<Absyn::TypeSpec>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut element: metamodelica::Ref<SCode::Element> = element;
    let () = (match &*element {
        SCode::Element::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; typeSpec = typeSpec);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(element)
}

pub fn getComponentTypeSpec(mut inE: &metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<Absyn::TypeSpec>> {
    let mut outTypeSpec: metamodelica::Ref<Absyn::TypeSpec>;
    let __pa0 = ::match_deref::match_deref! { match &((*inE)) {
        Deref @ SCode::Element::COMPONENT { typeSpec: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outTypeSpec = metamodelica::Own::own(__pa0);
    Ok(outTypeSpec)
}

pub fn setComponentMod(
    mut element: metamodelica::Ref<SCode::Element>,
    mut r#mod: metamodelica::Ref<SCode::Mod>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut element: metamodelica::Ref<SCode::Element> = element;
    let () = (match &*element {
        SCode::Element::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; modifications = r#mod);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(element)
}

pub(crate) fn getComponentMod(mut inE: &metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut outMod: metamodelica::Ref<SCode::Mod>;
    let __pa0 = ::match_deref::match_deref! { match &((*inE)) {
        Deref @ SCode::Element::COMPONENT { modifications: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outMod = metamodelica::Own::own(__pa0);
    Ok(outMod)
}

pub fn isDerivedClass(mut inClass: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut isDerived: bool;
    isDerived = (::match_deref::match_deref! { match inClass {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isDerived
}

pub fn isClassExtends(mut cls: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut isCE: bool;
    isCE = (::match_deref::match_deref! { match cls {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isCE
}

pub(crate) fn getDerivedTypeSpec(
    mut inE: &metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<Absyn::TypeSpec>> {
    let mut outTypeSpec: metamodelica::Ref<Absyn::TypeSpec>;
    let __pa0 = ::match_deref::match_deref! { match &((*inE)) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { typeSpec: __pa0, .. }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outTypeSpec = metamodelica::Own::own(__pa0);
    Ok(outTypeSpec)
}

pub fn getDerivedMod(mut inE: &metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut outMod: metamodelica::Ref<SCode::Mod>;
    let __pa0 = ::match_deref::match_deref! { match &((*inE)) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { modifications: __pa0, .. }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outMod = metamodelica::Own::own(__pa0);
    Ok(outMod)
}

pub(crate) fn setClassPrefixes(
    mut prefixes: metamodelica::Ref<SCode::Prefixes>,
    mut cl: metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut cl: metamodelica::Ref<SCode::Element> = cl;
    let () = (match &*cl {
        SCode::Element::CLASS { .. } => {
            assign_variant_field!(cl => SCode::Element::CLASS; prefixes = prefixes);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(cl)
}

pub fn getClassDef(mut inClass: &metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<SCode::ClassDef>> {
    let mut outCdef: metamodelica::Ref<SCode::ClassDef>;
    outCdef = (match &**inClass {
        SCode::Element::CLASS {
            classDef: __esc_outCdef,
            ..
        } => {
            outCdef = (*__esc_outCdef).clone();
            outCdef.clone()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outCdef)
}

pub fn setClassDef(
    mut classDef: metamodelica::Ref<SCode::ClassDef>,
    mut cls: metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut cls: metamodelica::Ref<SCode::Element> = cls;
    let () = (match &*cls {
        SCode::Element::CLASS { .. } => {
            assign_variant_field!(cls => SCode::Element::CLASS; classDef = classDef);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(cls)
}

pub fn getClassBody(mut inClass: &metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<SCode::ClassDef>> {
    let mut outCdef: metamodelica::Ref<SCode::ClassDef>;
    outCdef = getClassDef(inClass)?;
    outCdef = (match &*outCdef {
        SCode::ClassDef::CLASS_EXTENDS {
            composition: __outCdef_composition,
            ..
        } => __outCdef_composition.clone(),
        _ => outCdef,
    });
    Ok(outCdef)
}

pub fn equationsContainReinit(mut inEqs: &metamodelica::List<metamodelica::Ref<SCode::Equation>>) -> Result<bool> {
    let mut hasReinit: bool;
    hasReinit = (::match_deref::match_deref! { match inEqs {
        _ => {
            let mut b: bool;
            b = List::applyAndFold(inEqs, &fnptr!(boolOr, bool, bool), &move |__a0: metamodelica::Ref<SCode::Equation>| equationContainReinit(&__a0), false)?;
            b
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(hasReinit)
}

pub(crate) fn equationContainReinit(mut inEq: &metamodelica::Ref<SCode::Equation>) -> Result<bool> {
    let mut hasReinit: bool;
    hasReinit = (match &**inEq {
        SCode::Equation::EQ_REINIT { .. } => true,
        SCode::Equation::EQ_WHEN {
            eEquationLst: eqs,
            elseBranches: tpl_el,
            ..
        } => {
            let mut b: bool;
            let mut eqs_lst: metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Equation>>>;
            b = equationsContainReinit(eqs)?;
            eqs_lst = List::map(tpl_el.clone(), &fnptr!(Util::tuple22, _))?;
            b = List::applyAndFold(
                &eqs_lst,
                &fnptr!(boolOr, bool, bool),
                &move |__a0: metamodelica::List<metamodelica::Ref<SCode::Equation>>| equationsContainReinit(&__a0),
                b,
            )?;
            b
        }
        SCode::Equation::EQ_IF {
            thenBranch: eqs_lst,
            elseBranch: eqs,
            ..
        } => {
            let mut b: bool;
            b = equationsContainReinit(eqs)?;
            b = List::applyAndFold(
                eqs_lst,
                &fnptr!(boolOr, bool, bool),
                &move |__a0: metamodelica::List<metamodelica::Ref<SCode::Equation>>| equationsContainReinit(&__a0),
                b,
            )?;
            b
        }
        SCode::Equation::EQ_FOR { eEquationLst: eqs, .. } => {
            let mut b: bool;
            b = equationsContainReinit(eqs)?;
            b
        }
        _ => false,
    });
    Ok(hasReinit)
}

pub fn algorithmsContainReinit(mut inAlgs: &metamodelica::List<metamodelica::Ref<SCode::Statement>>) -> Result<bool> {
    let mut hasReinit: bool;
    hasReinit = (::match_deref::match_deref! { match inAlgs {
        _ => {
            let mut b: bool;
            b = List::applyAndFold(inAlgs, &fnptr!(boolOr, bool, bool), &move |__a0: metamodelica::Ref<SCode::Statement>| algorithmContainReinit(&__a0), false)?;
            b
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(hasReinit)
}

pub(crate) fn algorithmContainReinit(mut inAlg: &metamodelica::Ref<SCode::Statement>) -> Result<bool> {
    let mut hasReinit: bool;
    hasReinit = (match &**inAlg {
        SCode::Statement::ALG_REINIT { .. } => true,
        SCode::Statement::ALG_WHEN_A { branches: tpl_alg, .. } => {
            let mut b: bool;
            let mut algs_lst: metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Statement>>>;
            algs_lst = List::map(tpl_alg.clone(), &fnptr!(Util::tuple22, _))?;
            b = List::applyAndFold(
                &algs_lst,
                &fnptr!(boolOr, bool, bool),
                &move |__a0: metamodelica::List<metamodelica::Ref<SCode::Statement>>| algorithmsContainReinit(&__a0),
                false,
            )?;
            b
        }
        SCode::Statement::ALG_IF {
            trueBranch: algs1,
            elseIfBranch: tpl_alg,
            elseBranch: algs2,
            ..
        } => {
            let mut b: bool;
            let mut b1: bool;
            let mut b2: bool;
            let mut b3: bool;
            let mut algs_lst: metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Statement>>>;
            b1 = algorithmsContainReinit(algs1)?;
            algs_lst = List::map(tpl_alg.clone(), &fnptr!(Util::tuple22, _))?;
            b2 = List::applyAndFold(
                &algs_lst,
                &fnptr!(boolOr, bool, bool),
                &move |__a0: metamodelica::List<metamodelica::Ref<SCode::Statement>>| algorithmsContainReinit(&__a0),
                b1,
            )?;
            b3 = algorithmsContainReinit(algs2)?;
            b = boolOr(b1, boolOr(b2, b3));
            b
        }
        SCode::Statement::ALG_FOR { forBody: algs, .. } => {
            let mut b: bool;
            b = algorithmsContainReinit(algs)?;
            b
        }
        SCode::Statement::ALG_WHILE { whileBody: algs, .. } => {
            let mut b: bool;
            b = algorithmsContainReinit(algs)?;
            b
        }
        _ => false,
    });
    Ok(hasReinit)
}

pub fn getClassPartialPrefix(mut inElement: &metamodelica::Ref<SCode::Element>) -> Result<SCode::Partial> {
    let mut outPartial: SCode::Partial;
    let __pa0 = ::match_deref::match_deref! { match &((*inElement)) {
        Deref @ SCode::Element::CLASS { partialPrefix: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outPartial = metamodelica::Own::own(__pa0);
    Ok(outPartial)
}

pub fn getClassRestriction(mut inElement: &metamodelica::Ref<SCode::Element>) -> Result<SCode::Restriction> {
    let mut outRestriction: SCode::Restriction;
    let __pa0 = ::match_deref::match_deref! { match &((*inElement)) {
        Deref @ SCode::Element::CLASS { restriction: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outRestriction = metamodelica::Own::own(__pa0);
    Ok(outRestriction)
}

pub(crate) fn isRedeclareSubMod(mut inSubMod: &metamodelica::Ref<SCode::SubMod>) -> bool {
    let mut outIsRedeclare: bool;
    outIsRedeclare = (::match_deref::match_deref! { match inSubMod {
        Deref @ SCode::SubMod { r#mod: Deref @ SCode::Mod::REDECL { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsRedeclare
}

pub fn isBreakSubMod(mut subMod: &metamodelica::Ref<SCode::SubMod>) -> bool {
    let mut isBreak: bool;
    isBreak = (match &*subMod.r#mod.clone() {
        SCode::Mod::BREAK_COMPONENT { .. } => true,
        SCode::Mod::BREAK_CONNECT { .. } => true,
        _ => false,
    });
    isBreak
}

pub fn isBreakComponentSubMod(mut subMod: &metamodelica::Ref<SCode::SubMod>) -> bool {
    let mut isBreak: bool;
    isBreak = (::match_deref::match_deref! { match subMod {
        Deref @ SCode::SubMod { r#mod: Deref @ SCode::Mod::BREAK_COMPONENT { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBreak
}

pub(crate) fn isBreakConnectSubMod(mut subMod: &metamodelica::Ref<SCode::SubMod>) -> bool {
    let mut isBreak: bool;
    isBreak = (::match_deref::match_deref! { match subMod {
        Deref @ SCode::SubMod { r#mod: Deref @ SCode::Mod::BREAK_CONNECT { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBreak
}

pub fn componentMod(mut inElement: &metamodelica::Ref<SCode::Element>) -> metamodelica::Ref<SCode::Mod> {
    let mut outMod: metamodelica::Ref<SCode::Mod>;
    outMod = (match &**inElement {
        SCode::Element::COMPONENT {
            modifications: r#mod, ..
        } => r#mod.clone(),
        _ => openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
    });
    outMod
}

pub fn elementMod(mut inElement: &metamodelica::Ref<SCode::Element>) -> metamodelica::Ref<SCode::Mod> {
    let mut outMod: metamodelica::Ref<SCode::Mod>;
    outMod = (::match_deref::match_deref! { match inElement {
        Deref @ SCode::Element::COMPONENT { modifications: r#mod, .. } => {
            r#mod.clone()
        },
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { modifications: r#mod, .. }, .. } => {
            r#mod.clone()
        },
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { modifications: r#mod, .. }, .. } => {
            r#mod.clone()
        },
        Deref @ SCode::Element::EXTENDS { modifications: r#mod, .. } => {
            r#mod.clone()
        },
        _ => {
            openmodelica_frontend_types::SCode::Mod::interned_NOMOD()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod
}

pub(crate) fn setElementMod(
    mut element: metamodelica::Ref<SCode::Element>,
    mut r#mod: metamodelica::Ref<SCode::Mod>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut element: metamodelica::Ref<SCode::Element> = element;
    let () = (match &*element {
        SCode::Element::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; modifications = r#mod);
            ()
        }
        SCode::Element::CLASS {
            classDef: __element_classDef,
            ..
        } => {
            assign_variant_field!(element => SCode::Element::CLASS; classDef = setClassDefMod(__element_classDef.clone(), r#mod));
            ()
        }
        SCode::Element::EXTENDS { .. } => {
            assign_variant_field!(element => SCode::Element::EXTENDS; modifications = r#mod);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(element)
}

fn setClassDefMod(
    mut classDef: metamodelica::Ref<SCode::ClassDef>,
    mut inMod: metamodelica::Ref<SCode::Mod>,
) -> metamodelica::Ref<SCode::ClassDef> {
    let mut classDef: metamodelica::Ref<SCode::ClassDef> = classDef;
    let () = (match &*classDef {
        SCode::ClassDef::DERIVED { .. } => {
            assign_variant_field!(classDef => SCode::ClassDef::DERIVED; modifications = inMod);
            ()
        }
        SCode::ClassDef::CLASS_EXTENDS { .. } => {
            assign_variant_field!(classDef => SCode::ClassDef::CLASS_EXTENDS; modifications = inMod);
            ()
        }
        _ => (),
    });
    classDef
}

pub(crate) fn isBuiltinElement(mut inElement: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut outIsBuiltin: bool;
    outIsBuiltin = (::match_deref::match_deref! { match inElement {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { lang: Some(Deref @ "builtin"), .. }), .. }, .. } => {
            true
        },
        Deref @ SCode::Element::CLASS { cmt: Deref @ SCode::Comment { annotation_: Some(ann), .. }, .. } => {
            hasBooleanNamedAnnotation(metamodelica::AsArg::as_arg(&ann), &(literal!("__OpenModelica_builtin")))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsBuiltin
}

pub fn isExternalFunctionRestriction(mut inRestr: SCode::FunctionRestriction) -> bool {
    let mut isExternal: bool;
    isExternal = (match inRestr {
        SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { .. } => true,
        _ => false,
    });
    isExternal
}

pub fn isImpureFunctionRestriction(mut inRestr: SCode::FunctionRestriction) -> bool {
    let mut isExternal: bool;
    isExternal = (match inRestr {
        SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION {
            purity: Absyn::FunctionPurity::IMPURE { .. },
        } => true,
        SCode::FunctionRestriction::FR_NORMAL_FUNCTION {
            purity: Absyn::FunctionPurity::IMPURE { .. },
        } => true,
        _ => false,
    });
    isExternal
}

pub(crate) fn isRestrictionImpure(mut inRestr: &SCode::Restriction, mut hasZeroOutputPreMSL3_2: bool) -> bool {
    let mut isImpure: bool;
    isImpure = (match inRestr.clone() {
        SCode::Restriction::R_FUNCTION {
            functionRestriction:
                SCode::FunctionRestriction::FR_NORMAL_FUNCTION {
                    purity: Absyn::FunctionPurity::IMPURE { .. },
                },
        } => true,
        SCode::Restriction::R_FUNCTION {
            functionRestriction:
                SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION {
                    purity: Absyn::FunctionPurity::IMPURE { .. },
                },
        } => true,
        SCode::Restriction::R_FUNCTION {
            functionRestriction:
                SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION {
                    purity: Absyn::FunctionPurity::NO_PURITY { .. },
                },
        } => !(hasZeroOutputPreMSL3_2),
        _ => false,
    });
    isImpure
}

pub fn getFunctionRestrictionPurity(mut restr: SCode::FunctionRestriction) -> Absyn::FunctionPurity {
    let mut purity: Absyn::FunctionPurity;
    purity = (match restr {
        SCode::FunctionRestriction::FR_NORMAL_FUNCTION {
            purity: mut __esc_purity,
        } => {
            purity = __esc_purity.clone();
            purity
        }
        SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION {
            purity: mut __esc_purity,
        } => {
            purity = __esc_purity.clone();
            purity
        }
        _ => openmodelica_ast::Absyn::FunctionPurity::NO_PURITY,
    });
    purity
}

pub(crate) fn elementInnerOuter(mut element: &metamodelica::Ref<SCode::Element>) -> Absyn::InnerOuter {
    let mut io: Absyn::InnerOuter;
    io = (match &**element {
        SCode::Element::CLASS {
            prefixes: __element_prefixes,
            ..
        } => prefixesInnerOuter(metamodelica::AsArg::as_arg(&__element_prefixes)),
        SCode::Element::COMPONENT {
            prefixes: __element_prefixes,
            ..
        } => prefixesInnerOuter(metamodelica::AsArg::as_arg(&__element_prefixes)),
        _ => openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
    });
    io
}

pub(crate) fn elementVisibility(mut element: &metamodelica::Ref<SCode::Element>) -> SCode::Visibility {
    let mut visibility: SCode::Visibility;
    visibility = (match &**element {
        SCode::Element::IMPORT {
            visibility: __element_visibility,
            ..
        } => __element_visibility.clone(),
        SCode::Element::EXTENDS {
            visibility: __element_visibility,
            ..
        } => __element_visibility.clone(),
        SCode::Element::CLASS {
            prefixes: __element_prefixes,
            ..
        } => prefixesVisibility(metamodelica::AsArg::as_arg(&__element_prefixes)),
        SCode::Element::COMPONENT {
            prefixes: __element_prefixes,
            ..
        } => prefixesVisibility(metamodelica::AsArg::as_arg(&__element_prefixes)),
        SCode::Element::DEFINEUNIT {
            visibility: __element_visibility,
            ..
        } => __element_visibility.clone(),
    });
    visibility
}

pub fn isClassNamed(mut inName: &ArcStr, mut inClass: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut outIsNamed: bool;
    outIsNamed = (match &**inClass {
        SCode::Element::CLASS { name, .. } => stringEq(&inName, &name),
        _ => false,
    });
    outIsNamed
}

pub fn isElementNamed(mut name: &ArcStr, mut element: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut res: bool;
    res = (match &**element {
        SCode::Element::CLASS {
            name: __element_name, ..
        } => metamodelica::stringEq(&__element_name, &name),
        SCode::Element::COMPONENT {
            name: __element_name, ..
        } => metamodelica::stringEq(&__element_name, &name),
        _ => false,
    });
    res
}

pub fn getElementComment(
    mut inElement: &metamodelica::Ref<SCode::Element>,
) -> Option<metamodelica::Ref<SCode::Comment>> {
    let mut outComment: Option<metamodelica::Ref<SCode::Comment>>;
    outComment = (match &**inElement {
        SCode::Element::COMPONENT { comment: cmt, .. } => Some(cmt.clone()),
        SCode::Element::CLASS { cmt, .. } => Some(cmt.clone()),
        SCode::Element::EXTENDS {
            ann: __inElement_ann, ..
        } => Some(metamodelica::Ref::new(SCode::Comment {
            annotation_: __inElement_ann.clone(),
            comment: None,
        })),
        _ => None,
    });
    outComment
}

pub(crate) fn stripAnnotationFromComment(
    mut inComment: Option<metamodelica::Ref<SCode::Comment>>,
) -> Option<metamodelica::Ref<SCode::Comment>> {
    let mut outComment: Option<metamodelica::Ref<SCode::Comment>>;
    outComment = (::match_deref::match_deref! { match &(inComment) {
        Some(Deref @ SCode::Comment { annotation_: _, comment: r#str }) => {
            Some(metamodelica::Ref::new(SCode::Comment { annotation_: None, comment: r#str.clone() }))
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outComment
}

pub(crate) fn isOverloadedFunction(mut inElement: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut isOverloaded: bool;
    isOverloaded = (::match_deref::match_deref! { match inElement {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::OVERLOAD { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isOverloaded
}

pub fn mergeWithOriginal(
    mut newClass: metamodelica::Ref<SCode::Element>,
    mut oldClass: &metamodelica::Ref<SCode::Element>,
) -> metamodelica::Ref<SCode::Element> {
    let mut newClass: metamodelica::Ref<SCode::Element> = newClass;
    let () = 'mc: {
        let __mc_input = (newClass.clone(), oldClass.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (isFunction(&newClass)) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { prefixes: prefixes1, classDef: cd1, .. }, Deref @ SCode::Element::CLASS { prefixes: prefixes2, classDef: cd2, .. }) => {
                    let mut mCCNew: metamodelica::Ref<SCode::Mod>;
                    let mut mCCOld: metamodelica::Ref<SCode::Mod>;
                    let mut newClass: metamodelica::Ref<SCode::Element> = newClass.clone();
                    mCCNew = getConstrainedByModifiers(metamodelica::AsArg::as_arg(&prefixes1));
                    mCCOld = getConstrainedByModifiers(metamodelica::AsArg::as_arg(&prefixes2));
                    assign_variant_field!(newClass => SCode::Element::CLASS;
                        classDef = mergeClassDef(metamodelica::AsArg::as_arg(&cd1), metamodelica::AsArg::as_arg(&cd2), mCCNew.clone(), mCCOld.clone())?,
                        prefixes = propagatePrefixes(metamodelica::AsArg::as_arg(&prefixes1), prefixes2.clone())?
                    );
                    Ok(((), newClass.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            newClass = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    newClass
}

pub fn getConstrainedByModifiers(mut inPrefixes: &metamodelica::Ref<SCode::Prefixes>) -> metamodelica::Ref<SCode::Mod> {
    let mut outMod: metamodelica::Ref<SCode::Mod>;
    outMod = (::match_deref::match_deref! { match inPrefixes {
        Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { modifier: m, .. }) }, .. } => {
            m.clone()
        },
        _ => {
            openmodelica_frontend_types::SCode::Mod::interned_NOMOD()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod
}

pub(crate) fn mergeClassDef(
    mut inNew: &metamodelica::Ref<SCode::ClassDef>,
    mut inOld: &metamodelica::Ref<SCode::ClassDef>,
    mut inCCModNew: metamodelica::Ref<SCode::Mod>,
    mut inCCModOld: metamodelica::Ref<SCode::Mod>,
) -> Result<metamodelica::Ref<SCode::ClassDef>> {
    let mut outNew: metamodelica::Ref<SCode::ClassDef>;
    outNew = (::match_deref::match_deref! { match (inNew, inOld) {
        (Deref @ SCode::ClassDef::DERIVED { typeSpec: ts1, modifications: m1, attributes: a1 }, Deref @ SCode::ClassDef::DERIVED { typeSpec: _, modifications: m2, attributes: a2 }) => {
            let mut n: metamodelica::Ref<SCode::ClassDef>;
            let mut m1 = (*m1).clone();
            let mut m2 = (*m2).clone();
            let mut a2 = (*a2).clone();
            m2 = mergeModifiers(m2.clone(), inCCModOld);
            m1 = mergeModifiers(m1.clone(), inCCModNew);
            m2 = mergeModifiers(m1.clone(), m2.clone());
            a2 = propagateAttributes(a2.clone(), a1.clone(), false);
            n = metamodelica::Ref::new(SCode::ClassDef::DERIVED { typeSpec: ts1.clone(), modifications: m2.clone(), attributes: a2.clone() });
            n
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outNew)
}

pub fn mergeModifiers(
    mut inNewMod: metamodelica::Ref<SCode::Mod>,
    mut inOldMod: metamodelica::Ref<SCode::Mod>,
) -> metamodelica::Ref<SCode::Mod> {
    let mut outMod: metamodelica::Ref<SCode::Mod>;
    outMod = (::match_deref::match_deref! { match &((inNewMod.clone(), inOldMod.clone())) {
        (_, Deref @ SCode::Mod::NOMOD { .. }) => {
            inNewMod
        },
        (Deref @ SCode::Mod::NOMOD { .. }, _) => {
            inOldMod
        },
        (Deref @ SCode::Mod::REDECL { .. }, _) => {
            inNewMod
        },
        (Deref @ SCode::Mod::MOD { finalPrefix: f1, eachPrefix: e1, subModLst: sl1, binding: b1, comment: cmt, info: i1 }, Deref @ SCode::Mod::MOD { finalPrefix: f2, eachPrefix: e2, subModLst: sl2, binding: b2, comment: _, .. }) => {
            let mut sl: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
            let mut b: Option<metamodelica::Ref<Absyn::Exp>>;
            let mut m: metamodelica::Ref<SCode::Mod>;
            b = if ((b1).is_some()) {b1.clone()} else {b2.clone()};
            sl = mergeSubMods(metamodelica::AsArg::as_arg(&sl1), metamodelica::AsArg::as_arg(&sl2));
            if (match (&(b), &(b1)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }) && metamodelica::ReferenceEq::reference_eq(&(sl), &(sl1.clone())) {
                m = inNewMod;
            } else if (match (&(b), &(b2)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }) && metamodelica::ReferenceEq::reference_eq(&(sl), &(sl2.clone())) && f1.clone() == f2.clone() && e1.clone() == e2.clone() {
                m = inOldMod;
            } else {
                m = metamodelica::Ref::new(SCode::Mod::MOD { finalPrefix: f1.clone(), eachPrefix: e1.clone(), subModLst: sl, binding: b, comment: cmt.clone(), info: i1.clone() });
            }
            m
        },
        _ => {
            inNewMod
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outMod
}

fn mergeSubMods(
    mut inNew: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    mut inOld: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
) -> metamodelica::List<metamodelica::Ref<SCode::SubMod>> {
    let mut outSubs: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    outSubs = 'mc: {
        let __mc_input = &**inNew;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(inOld.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: s, tail: rest } => {
                    let mut sl: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
                    let mut old: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
                    old = removeSub(metamodelica::AsArg::as_arg(&s), inOld)?;
                    sl = mergeSubMods(metamodelica::AsArg::as_arg(&rest), &old);
                    Ok(metamodelica::cons(s.clone(), sl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inNew.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outSubs
}

fn removeSub(
    mut inSub: &metamodelica::Ref<SCode::SubMod>,
    mut inOld: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::SubMod>>> {
    let mut outSubs: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    outSubs = (::match_deref::match_deref! { match (inSub, inOld) {
        (_, Deref @ metamodelica::ListNode::Nil) => {
            inOld.clone()
        },
        (Deref @ SCode::SubMod { ident: id1, .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::SubMod { ident: id2, .. }, tail: rest }) if (stringEqual(&id1, &id2)) => {
            rest.clone()
        },
        (_, Deref @ metamodelica::ListNode::Cons { head: s, tail: rest }) => {
            let mut rest = (*rest).clone();
            rest = removeSub(inSub, metamodelica::AsArg::as_arg(&rest))?;
            metamodelica::cons(s.clone(), rest.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outSubs)
}

pub(crate) fn mergeComponentModifiers(
    mut newComp: metamodelica::Ref<SCode::Element>,
    mut oldComp: &metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut newComp: metamodelica::Ref<SCode::Element> = newComp;
    let () = (::match_deref::match_deref! { match &((newComp.clone(), oldComp.clone())) {
        (Deref @ SCode::Element::COMPONENT { .. }, Deref @ SCode::Element::COMPONENT { .. }) => {
            assign_variant_field!(newComp => SCode::Element::COMPONENT; modifications = mergeModifiers(var_field!((*newComp).modifications, SCode::Element::COMPONENT).clone(), var_field!((**oldComp).modifications, SCode::Element::COMPONENT).clone()));
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(newComp)
}

pub fn propagateAttributes(
    mut inOriginalAttributes: SCode::Attributes,
    mut inNewAttributes: SCode::Attributes,
    mut inNewTypeIsArray: bool,
) -> SCode::Attributes {
    let mut outNewAttributes: SCode::Attributes;
    let mut dims1: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    let mut dims2: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    let mut ct1: SCode::ConnectorType;
    let mut ct2: SCode::ConnectorType;
    let mut prl1: SCode::Parallelism;
    let mut prl2: SCode::Parallelism;
    let mut var1: SCode::Variability;
    let mut var2: SCode::Variability;
    let mut dir1: Absyn::Direction;
    let mut dir2: Absyn::Direction;
    let mut if1: Absyn::IsField;
    let mut if2: Absyn::IsField;
    let SCode::ATTR {
        arrayDims: __pa0,
        connectorType: __pa1,
        parallelism: __pa2,
        variability: __pa3,
        direction: __pa4,
        isField: __pa5,
    } = inOriginalAttributes;
    dims1 = metamodelica::Own::own(__pa0);
    ct1 = metamodelica::Own::own(__pa1);
    prl1 = metamodelica::Own::own(__pa2);
    var1 = metamodelica::Own::own(__pa3);
    dir1 = metamodelica::Own::own(__pa4);
    if1 = metamodelica::Own::own(__pa5);
    let SCode::ATTR {
        arrayDims: __pa6,
        connectorType: __pa7,
        parallelism: __pa8,
        variability: __pa9,
        direction: __pa10,
        isField: __pa11,
    } = inNewAttributes;
    dims2 = metamodelica::Own::own(__pa6);
    ct2 = metamodelica::Own::own(__pa7);
    prl2 = metamodelica::Own::own(__pa8);
    var2 = metamodelica::Own::own(__pa9);
    dir2 = metamodelica::Own::own(__pa10);
    if2 = metamodelica::Own::own(__pa11);
    if !(inNewTypeIsArray) {
        dims2 = propagateArrayDimensions(dims1, dims2);
    }
    ct2 = propagateConnectorType(ct1, ct2);
    prl2 = propagateParallelism(prl1, prl2);
    var2 = propagateVariability(var1, var2);
    dir2 = propagateDirection(dir1, dir2);
    if2 = propagateIsField(if1, if2);
    outNewAttributes = SCode::Attributes {
        arrayDims: dims2,
        connectorType: ct2,
        parallelism: prl2,
        variability: var2,
        direction: dir2,
        isField: if2,
    };
    outNewAttributes
}

pub(crate) fn propagateArrayDimensions(
    mut inOriginalDims: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut inNewDims: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Subscript>> {
    let mut outNewDims: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
    outNewDims = (::match_deref::match_deref! { match &(inNewDims.clone()) {
        Deref @ metamodelica::ListNode::Nil => inOriginalDims,
        _ => inNewDims,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outNewDims
}

pub(crate) fn propagateConnectorType(
    mut inOriginalConnectorType: SCode::ConnectorType,
    mut inNewConnectorType: SCode::ConnectorType,
) -> SCode::ConnectorType {
    let mut outNewConnectorType: SCode::ConnectorType;
    outNewConnectorType = (match inNewConnectorType {
        SCode::ConnectorType::POTENTIAL { .. } => inOriginalConnectorType,
        _ => inNewConnectorType,
    });
    outNewConnectorType
}

pub(crate) fn propagateParallelism(
    mut inOriginalParallelism: SCode::Parallelism,
    mut inNewParallelism: SCode::Parallelism,
) -> SCode::Parallelism {
    let mut outNewParallelism: SCode::Parallelism;
    outNewParallelism = (match inNewParallelism {
        SCode::Parallelism::NON_PARALLEL { .. } => inOriginalParallelism,
        _ => inNewParallelism,
    });
    outNewParallelism
}

pub(crate) fn propagateVariability(
    mut inOriginalVariability: SCode::Variability,
    mut inNewVariability: SCode::Variability,
) -> SCode::Variability {
    let mut outNewVariability: SCode::Variability;
    outNewVariability = (match inNewVariability {
        SCode::Variability::VAR { .. } => inOriginalVariability,
        _ => inNewVariability,
    });
    outNewVariability
}

pub(crate) fn propagateDirection(
    mut inOriginalDirection: Absyn::Direction,
    mut inNewDirection: Absyn::Direction,
) -> Absyn::Direction {
    let mut outNewDirection: Absyn::Direction;
    outNewDirection = (match inNewDirection {
        Absyn::Direction::BIDIR { .. } => inOriginalDirection,
        _ => inNewDirection,
    });
    outNewDirection
}

pub(crate) fn propagateIsField(
    mut inOriginalIsField: Absyn::IsField,
    mut inNewIsField: Absyn::IsField,
) -> Absyn::IsField {
    let mut outNewIsField: Absyn::IsField;
    outNewIsField = (match inNewIsField {
        Absyn::IsField::NONFIELD { .. } => inOriginalIsField,
        _ => inNewIsField,
    });
    outNewIsField
}

pub fn propagateAttributesVar(
    mut originalVar: &metamodelica::Ref<SCode::Element>,
    mut newVar: metamodelica::Ref<SCode::Element>,
    mut isNewTypeArray: bool,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut newVar: metamodelica::Ref<SCode::Element> = newVar;
    let () = (::match_deref::match_deref! { match &((originalVar.clone(), newVar.clone())) {
        (Deref @ SCode::Element::COMPONENT { .. }, Deref @ SCode::Element::COMPONENT { .. }) => {
            assign_variant_field!(newVar => SCode::Element::COMPONENT;
                prefixes = propagatePrefixes(var_field!((**originalVar).prefixes, SCode::Element::COMPONENT), var_field!((*newVar).prefixes, SCode::Element::COMPONENT).clone())?,
                attributes = propagateAttributes(var_field!((**originalVar).attributes, SCode::Element::COMPONENT).clone(), var_field!((*newVar).attributes, SCode::Element::COMPONENT).clone(), isNewTypeArray)
            );
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(newVar)
}

pub(crate) fn propagateAttributesClass(
    mut originalClass: &metamodelica::Ref<SCode::Element>,
    mut newClass: metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut newClass: metamodelica::Ref<SCode::Element> = newClass;
    let () = (::match_deref::match_deref! { match &((originalClass.clone(), newClass.clone())) {
        (Deref @ SCode::Element::CLASS { .. }, Deref @ SCode::Element::CLASS { .. }) => {
            assign_variant_field!(newClass => SCode::Element::CLASS; prefixes = propagatePrefixes(var_field!((**originalClass).prefixes, SCode::Element::CLASS), var_field!((*newClass).prefixes, SCode::Element::CLASS).clone())?);
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(newClass)
}

pub fn propagatePrefixes(
    mut originalPrefixes: &metamodelica::Ref<SCode::Prefixes>,
    mut newPrefixes: metamodelica::Ref<SCode::Prefixes>,
) -> Result<metamodelica::Ref<SCode::Prefixes>> {
    let mut newPrefixes: metamodelica::Ref<SCode::Prefixes> = newPrefixes;
    let () = (::match_deref::match_deref! { match &((originalPrefixes.clone(), newPrefixes.clone())) {
        (Deref @ SCode::Prefixes { .. }, Deref @ SCode::Prefixes { .. }) => {
            assign_field!(newPrefixes.innerOuter = propagatePrefixInnerOuter(originalPrefixes.innerOuter.clone(), newPrefixes.innerOuter.clone()));
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(newPrefixes)
}

pub(crate) fn propagatePrefixInnerOuter(
    mut inOriginalIO: Absyn::InnerOuter,
    mut inIO: Absyn::InnerOuter,
) -> Absyn::InnerOuter {
    let mut outIO: Absyn::InnerOuter;
    outIO = (match inIO {
        Absyn::InnerOuter::NOT_INNER_OUTER { .. } => inOriginalIO,
        _ => inIO,
    });
    outIO
}

pub fn isPackage(mut inClass: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inClass {
        SCode::Element::CLASS {
            restriction: SCode::Restriction::R_PACKAGE { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub fn isPartial(mut inClass: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inClass {
        SCode::Element::CLASS {
            partialPrefix: SCode::Partial::PARTIAL { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isValidPackageElement(mut inElement: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut outIsValid: bool;
    outIsValid = (match &**inElement {
        SCode::Element::COMPONENT {
            attributes:
                SCode::Attributes {
                    variability: SCode::Variability::CONST { .. },
                    ..
                },
            ..
        } => true,
        SCode::Element::COMPONENT { .. } => false,
        _ => true,
    });
    outIsValid
}

pub fn classIsExternalObject(mut cl: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match cl {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: els, .. }, .. } => {
            isExternalObject(metamodelica::AsArg::as_arg(&els))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isExternalObject(mut els: &metamodelica::List<metamodelica::Ref<SCode::Element>>) -> bool {
    let mut res: bool;
    res = if (((els).len() as i32) == 3) {
        hasExtendsOfExternalObject(els) && hasExternalObjectDestructor(els) && hasExternalObjectConstructor(els)
    } else {
        false
    };
    res
}

fn hasExtendsOfExternalObject<'__b>(mut inEls: &'__b metamodelica::List<metamodelica::Ref<SCode::Element>>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match inEls {
            Deref @ metamodelica::ListNode::Nil => {
                return false
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: path, .. }, tail: _ } if (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&path), &(metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("ExternalObject") })))) => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: els } => {
                { inEls = els; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn hasExternalObjectDestructor<'__b>(mut inEls: &'__b metamodelica::List<metamodelica::Ref<SCode::Element>>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match inEls {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::CLASS { name: Deref @ "destructor", .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: els } => {
                { inEls = els; continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn hasExternalObjectConstructor<'__b>(mut inEls: &'__b metamodelica::List<metamodelica::Ref<SCode::Element>>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match inEls {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::CLASS { name: Deref @ "constructor", .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: els } => {
                { inEls = els; continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn getExternalObjectDestructor<'__b>(
    mut inEls: &'__b metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut cl: metamodelica::Ref<SCode::Element>;
    cl = (::match_deref::match_deref! { match inEls {
        Deref @ metamodelica::ListNode::Cons { head: __esc_cl @ Deref @ SCode::Element::CLASS { name: Deref @ "destructor", .. }, tail: _ } => {
            cl = (*__esc_cl).clone();
            cl.clone()
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: els } => {
            getExternalObjectDestructor(els)?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(cl)
}

pub fn getExternalObjectConstructor<'__b>(
    mut inEls: &'__b metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut cl: metamodelica::Ref<SCode::Element>;
    cl = (::match_deref::match_deref! { match inEls {
        Deref @ metamodelica::ListNode::Cons { head: __esc_cl @ Deref @ SCode::Element::CLASS { name: Deref @ "constructor", .. }, tail: _ } => {
            cl = (*__esc_cl).clone();
            cl.clone()
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: els } => {
            getExternalObjectConstructor(els)?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(cl)
}

pub fn isInstantiableClassRestriction(mut inRestriction: &SCode::Restriction) -> bool {
    let mut outIsInstantiable: bool;
    outIsInstantiable = (match inRestriction.clone() {
        SCode::Restriction::R_CLASS { .. } => true,
        SCode::Restriction::R_MODEL { .. } => true,
        SCode::Restriction::R_RECORD { .. } => true,
        SCode::Restriction::R_BLOCK { .. } => true,
        SCode::Restriction::R_CONNECTOR { .. } => true,
        SCode::Restriction::R_TYPE { .. } => true,
        SCode::Restriction::R_ENUMERATION { .. } => true,
        _ => false,
    });
    outIsInstantiable
}

pub fn isInitial(mut inInitial: SCode::Initial) -> bool {
    let mut isIn: bool;
    isIn = (match inInitial {
        SCode::Initial::INITIAL { .. } => true,
        _ => false,
    });
    isIn
}

pub fn checkSameRestriction(
    mut inResNew: SCode::Restriction,
    mut inResOrig: &SCode::Restriction,
    mut inInfoNew: SourceInfo,
    mut inInfoOrig: &SourceInfo,
) -> (SCode::Restriction, SourceInfo) {
    let mut outRes: SCode::Restriction;
    let mut outInfo: SourceInfo;
    (outRes, outInfo) = (match inInfoOrig.clone() {
        _ => (inResNew, inInfoNew),
    });
    (outRes, outInfo)
}

pub fn setComponentName(
    mut element: metamodelica::Ref<SCode::Element>,
    mut name: ArcStr,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut element: metamodelica::Ref<SCode::Element> = element;
    let () = (match &*element {
        SCode::Element::COMPONENT { .. } => {
            assign_variant_field!(element => SCode::Element::COMPONENT; name = name);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(element)
}

pub fn isArrayComponent(mut inElement: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut outIsArray: bool;
    outIsArray = (::match_deref::match_deref! { match inElement {
        Deref @ SCode::Element::COMPONENT { attributes: SCode::Attributes { arrayDims: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsArray
}

pub fn isEmptyMod(mut r#mod: &metamodelica::Ref<SCode::Mod>) -> bool {
    let mut isEmpty: bool;
    isEmpty = (match &**r#mod {
        SCode::Mod::NOMOD { .. } => true,
        _ => false,
    });
    isEmpty
}

pub fn getConstrainingMod(mut element: &metamodelica::Ref<SCode::Element>) -> metamodelica::Ref<SCode::Mod> {
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    r#mod = (::match_deref::match_deref! { match element {
        Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { modifier: __esc_mod, .. }) }, .. }, .. } => {
            r#mod = (*__esc_mod).clone();
            r#mod.clone()
        },
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { modifications: __esc_mod, .. }, .. } => {
            r#mod = (*__esc_mod).clone();
            r#mod.clone()
        },
        Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { modifier: __esc_mod, .. }) }, .. }, .. } => {
            r#mod = (*__esc_mod).clone();
            r#mod.clone()
        },
        Deref @ SCode::Element::COMPONENT { modifications: __esc_mod, .. } => {
            r#mod = (*__esc_mod).clone();
            r#mod.clone()
        },
        _ => openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    r#mod
}

pub fn isEmptyClassDef<'__b>(mut cdef: &'__b metamodelica::Ref<SCode::ClassDef>) -> bool {
    '__tco: loop {
        match &**cdef {
            SCode::ClassDef::PARTS { .. } => {
                return (var_field!((**cdef).elementLst, SCode::ClassDef::PARTS)).is_empty()
                    && (var_field!((**cdef).normalEquationLst, SCode::ClassDef::PARTS)).is_empty()
                    && (var_field!((**cdef).initialEquationLst, SCode::ClassDef::PARTS)).is_empty()
                    && (var_field!((**cdef).normalAlgorithmLst, SCode::ClassDef::PARTS)).is_empty()
                    && (var_field!((**cdef).initialAlgorithmLst, SCode::ClassDef::PARTS)).is_empty()
                    && (var_field!((**cdef).externalDecl, SCode::ClassDef::PARTS)).is_none();
            }
            SCode::ClassDef::CLASS_EXTENDS { .. } => {
                cdef = var_field!((**cdef).composition, SCode::ClassDef::CLASS_EXTENDS);
                continue '__tco;
            }
            SCode::ClassDef::ENUMERATION { .. } => {
                return (var_field!((**cdef).enumLst, SCode::ClassDef::ENUMERATION)).is_empty();
            }
            _ => return true,
        }
    }
}

pub fn stripCommentsFromProgram(
    mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut stripAnnotations: bool,
    mut stripComments: bool,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut program: metamodelica::List<metamodelica::Ref<SCode::Element>> = program;
    program = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
        for mut e in (program).into_iter().cloned() {
            let __x = stripCommentsFromElement(e.clone(), stripAnnotations, stripComments)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(program)
}

pub(crate) fn stripCommentsFromElement(
    mut element: metamodelica::Ref<SCode::Element>,
    mut stripAnn: bool,
    mut stripCmt: bool,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut element: metamodelica::Ref<SCode::Element> = element;
    let () = (match &*element {
        SCode::Element::EXTENDS { .. } => {
            if stripAnn {
                assign_variant_field!(element => SCode::Element::EXTENDS; ann = None);
            }
            assign_variant_field!(element => SCode::Element::EXTENDS; modifications = stripCommentsFromMod(var_field!((*element).modifications, SCode::Element::EXTENDS).clone(), stripAnn, stripCmt)?);
            ()
        }
        SCode::Element::CLASS {
            classDef: __element_classDef,
            ..
        } => {
            assign_variant_field!(element => SCode::Element::CLASS;
                classDef = stripCommentsFromClassDef(__element_classDef.clone(), stripAnn, stripCmt)?,
                cmt = stripCommentsFromComment(var_field!((*element).cmt, SCode::Element::CLASS).clone(), stripAnn, stripCmt)
            );
            ()
        }
        SCode::Element::COMPONENT {
            modifications: __element_modifications,
            ..
        } => {
            assign_variant_field!(element => SCode::Element::COMPONENT;
                modifications = stripCommentsFromMod(__element_modifications.clone(), stripAnn, stripCmt)?,
                comment = stripCommentsFromComment(var_field!((*element).comment, SCode::Element::COMPONENT).clone(), stripAnn, stripCmt)
            );
            ()
        }
        _ => (),
    });
    Ok(element)
}

pub(crate) fn stripCommentsFromMod(
    mut r#mod: metamodelica::Ref<SCode::Mod>,
    mut stripAnn: bool,
    mut stripCmt: bool,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut r#mod: metamodelica::Ref<SCode::Mod> = r#mod;
    let () = (match &*r#mod {
        SCode::Mod::MOD {
            subModLst: __mod_subModLst,
            ..
        } => {
            assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::SubMod>> = metamodelica::nil();
                for mut m in (__mod_subModLst.clone()).into_iter().cloned() {
                    let __x = stripCommentsFromSubMod(m.clone(), stripAnn, stripCmt)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        SCode::Mod::REDECL {
            element: __mod_element, ..
        } => {
            assign_variant_field!(r#mod => SCode::Mod::REDECL; element = stripCommentsFromElement(__mod_element.clone(), stripAnn, stripCmt)?);
            ()
        }
        _ => (),
    });
    Ok(r#mod)
}

pub(crate) fn stripCommentsFromSubMod(
    mut submod: metamodelica::Ref<SCode::SubMod>,
    mut stripAnn: bool,
    mut stripCmt: bool,
) -> Result<metamodelica::Ref<SCode::SubMod>> {
    let mut submod: metamodelica::Ref<SCode::SubMod> = submod;
    assign_field!(submod.r#mod = stripCommentsFromMod(submod.r#mod.clone(), stripAnn, stripCmt)?);
    Ok(submod)
}

pub(crate) fn stripCommentsFromClassDef(
    mut cdef: metamodelica::Ref<SCode::ClassDef>,
    mut stripAnn: bool,
    mut stripCmt: bool,
) -> Result<metamodelica::Ref<SCode::ClassDef>> {
    let mut cdef: metamodelica::Ref<SCode::ClassDef> = cdef;
    cdef = (match &*cdef {
        SCode::ClassDef::PARTS {
            clsattrs: __cdef_clsattrs,
            constraintLst: __cdef_constraintLst,
            elementLst: __cdef_elementLst,
            externalDecl: __cdef_externalDecl,
            initialAlgorithmLst: __cdef_initialAlgorithmLst,
            initialEquationLst: __cdef_initialEquationLst,
            normalAlgorithmLst: __cdef_normalAlgorithmLst,
            normalEquationLst: __cdef_normalEquationLst,
        } => {
            let mut el: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut eql: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
            let mut ieql: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
            let mut alg: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
            let mut ialg: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>>;
            let mut ext: Option<metamodelica::Ref<SCode::ExternalDecl>>;
            el = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
                for mut e in (__cdef_elementLst.clone()).into_iter().cloned() {
                    let __x = stripCommentsFromElement(e.clone(), stripAnn, stripCmt)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            eql = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Equation>> = metamodelica::nil();
                for mut eq in (__cdef_normalEquationLst.clone()).into_iter().cloned() {
                    let __x = stripCommentsFromEquation(eq.clone(), stripAnn, stripCmt);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            ieql = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Equation>> = metamodelica::nil();
                for mut ieq in (__cdef_initialEquationLst.clone()).into_iter().cloned() {
                    let __x = stripCommentsFromEquation(ieq.clone(), stripAnn, stripCmt);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            alg = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>> = metamodelica::nil();
                for mut a in (__cdef_normalAlgorithmLst.clone()).into_iter().cloned() {
                    let __x = stripCommentsFromAlgorithm(a.clone(), stripAnn, stripCmt);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            ialg = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::AlgorithmSection>> = metamodelica::nil();
                for mut ia in (__cdef_initialAlgorithmLst.clone()).into_iter().cloned() {
                    let __x = stripCommentsFromAlgorithm(ia.clone(), stripAnn, stripCmt);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            ext = stripCommentsFromExternalDecl(__cdef_externalDecl.clone(), stripAnn, stripCmt)?;
            metamodelica::Ref::new(SCode::ClassDef::PARTS {
                elementLst: el,
                normalEquationLst: eql,
                initialEquationLst: ieql,
                normalAlgorithmLst: alg,
                initialAlgorithmLst: ialg,
                constraintLst: __cdef_constraintLst.clone(),
                clsattrs: __cdef_clsattrs.clone(),
                externalDecl: ext,
            })
        }
        SCode::ClassDef::CLASS_EXTENDS {
            modifications: __cdef_modifications,
            ..
        } => {
            assign_variant_field!(cdef => SCode::ClassDef::CLASS_EXTENDS;
                modifications = stripCommentsFromMod(__cdef_modifications.clone(), stripAnn, stripCmt)?,
                composition = stripCommentsFromClassDef(var_field!((*cdef).composition, SCode::ClassDef::CLASS_EXTENDS).clone(), stripAnn, stripCmt)?
            );
            cdef
        }
        SCode::ClassDef::DERIVED {
            modifications: __cdef_modifications,
            ..
        } => {
            assign_variant_field!(cdef => SCode::ClassDef::DERIVED; modifications = stripCommentsFromMod(__cdef_modifications.clone(), stripAnn, stripCmt)?);
            cdef
        }
        SCode::ClassDef::ENUMERATION {
            enumLst: __cdef_enumLst,
        } => {
            assign_variant_field!(cdef => SCode::ClassDef::ENUMERATION; enumLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Enum>> = metamodelica::nil();
                for mut e in (__cdef_enumLst.clone()).into_iter().cloned() {
                    let __x = stripCommentsFromEnum(e.clone(), stripAnn, stripCmt);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            cdef
        }
        _ => cdef,
    });
    Ok(cdef)
}

pub(crate) fn stripCommentsFromEnum(
    mut r#enum: metamodelica::Ref<SCode::Enum>,
    mut stripAnn: bool,
    mut stripCmt: bool,
) -> metamodelica::Ref<SCode::Enum> {
    let mut r#enum: metamodelica::Ref<SCode::Enum> = r#enum;
    assign_field!(r#enum.comment = stripCommentsFromComment(r#enum.comment.clone(), stripAnn, stripCmt));
    r#enum
}

pub(crate) fn stripCommentsFromComment(
    mut cmt: metamodelica::Ref<SCode::Comment>,
    mut stripAnn: bool,
    mut stripCmt: bool,
) -> metamodelica::Ref<SCode::Comment> {
    let mut cmt: metamodelica::Ref<SCode::Comment> = cmt;
    if stripAnn {
        assign_field!(cmt.annotation_ = None);
    }
    if stripCmt {
        assign_field!(cmt.comment = None);
    }
    cmt
}

pub(crate) fn stripCommentsFromExternalDecl(
    mut extDecl: Option<metamodelica::Ref<SCode::ExternalDecl>>,
    mut stripAnn: bool,
    mut stripCmt: bool,
) -> Result<Option<metamodelica::Ref<SCode::ExternalDecl>>> {
    let mut extDecl: Option<metamodelica::Ref<SCode::ExternalDecl>> = extDecl;
    let mut ext_decl: metamodelica::Ref<SCode::ExternalDecl>;
    if (extDecl).is_some() && stripAnn {
        let __pa0 = ::match_deref::match_deref! { match &(extDecl) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        ext_decl = metamodelica::Own::own(__pa0);
        assign_field!(ext_decl.annotation_ = None);
        extDecl = Some(ext_decl);
    }
    Ok(extDecl)
}

pub(crate) fn stripCommentsFromEquation(
    mut eq: metamodelica::Ref<SCode::Equation>,
    mut stripAnn: bool,
    mut stripCmt: bool,
) -> metamodelica::Ref<SCode::Equation> {
    let mut eq: metamodelica::Ref<SCode::Equation> = eq;
    let () = (match &*eq {
        SCode::Equation::EQ_IF {
            thenBranch: __eq_thenBranch,
            ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_IF;
                        thenBranch = ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Equation>>> = metamodelica::nil();
                for mut branch in (__eq_thenBranch.clone()).into_iter().cloned() {
                    let __x = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Equation>> = metamodelica::nil();
                for mut e in (branch.clone()).into_iter().cloned() {
                    let __x = stripCommentsFromEquation(e.clone(), stripAnn, stripCmt);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        elseBranch = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Equation>> = metamodelica::nil();
                for mut e in (var_field!((*eq).elseBranch, SCode::Equation::EQ_IF).clone()).into_iter().cloned() {
                    let __x = stripCommentsFromEquation(e.clone(), stripAnn, stripCmt);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_IF).clone(), stripAnn, stripCmt)
                    );
            ()
        }
        SCode::Equation::EQ_EQUALS {
            comment: __eq_comment, ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_EQUALS; comment = stripCommentsFromComment(__eq_comment.clone(), stripAnn, stripCmt));
            ()
        }
        SCode::Equation::EQ_PDE {
            comment: __eq_comment, ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_PDE; comment = stripCommentsFromComment(__eq_comment.clone(), stripAnn, stripCmt));
            ()
        }
        SCode::Equation::EQ_CONNECT {
            comment: __eq_comment, ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_CONNECT; comment = stripCommentsFromComment(__eq_comment.clone(), stripAnn, stripCmt));
            ()
        }
        SCode::Equation::EQ_FOR {
            eEquationLst: __eq_eEquationLst,
            ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_FOR;
                        eEquationLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Equation>> = metamodelica::nil();
                for mut e in (__eq_eEquationLst.clone()).into_iter().cloned() {
                    let __x = stripCommentsFromEquation(e.clone(), stripAnn, stripCmt);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_FOR).clone(), stripAnn, stripCmt)
                    );
            ()
        }
        SCode::Equation::EQ_WHEN {
            eEquationLst: __eq_eEquationLst,
            ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_WHEN;
                        eEquationLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Equation>> = metamodelica::nil();
                for mut e in (__eq_eEquationLst.clone()).into_iter().cloned() {
                    let __x = stripCommentsFromEquation(e.clone(), stripAnn, stripCmt);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        elseBranches = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<SCode::Equation>>)> = metamodelica::nil();
                for mut b in (var_field!((*eq).elseBranches, SCode::Equation::EQ_WHEN).clone()).into_iter().cloned() {
                    let __x = stripCommentsFromWhenEqBranch(b.clone(), stripAnn, stripCmt);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        comment = stripCommentsFromComment(var_field!((*eq).comment, SCode::Equation::EQ_WHEN).clone(), stripAnn, stripCmt)
                    );
            ()
        }
        SCode::Equation::EQ_ASSERT {
            comment: __eq_comment, ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_ASSERT; comment = stripCommentsFromComment(__eq_comment.clone(), stripAnn, stripCmt));
            ()
        }
        SCode::Equation::EQ_TERMINATE {
            comment: __eq_comment, ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_TERMINATE; comment = stripCommentsFromComment(__eq_comment.clone(), stripAnn, stripCmt));
            ()
        }
        SCode::Equation::EQ_REINIT {
            comment: __eq_comment, ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_REINIT; comment = stripCommentsFromComment(__eq_comment.clone(), stripAnn, stripCmt));
            ()
        }
        SCode::Equation::EQ_NORETCALL {
            comment: __eq_comment, ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_NORETCALL; comment = stripCommentsFromComment(__eq_comment.clone(), stripAnn, stripCmt));
            ()
        }
    });
    eq
}

pub(crate) fn stripCommentsFromWhenEqBranch(
    mut branch: (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    ),
    mut stripAnn: bool,
    mut stripCmt: bool,
) -> (
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::List<metamodelica::Ref<SCode::Equation>>,
) {
    let mut branch: (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    ) = branch;
    let mut cond: metamodelica::Ref<Absyn::Exp>;
    let mut body: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    (cond, body) = branch;
    body = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Equation>> = metamodelica::nil();
        for mut e in (body).into_iter().cloned() {
            let __x = stripCommentsFromEquation(e.clone(), stripAnn, stripCmt);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    branch = (cond, body);
    branch
}

pub(crate) fn stripCommentsFromAlgorithm(
    mut alg: metamodelica::Ref<SCode::AlgorithmSection>,
    mut stripAnn: bool,
    mut stripCmt: bool,
) -> metamodelica::Ref<SCode::AlgorithmSection> {
    let mut alg: metamodelica::Ref<SCode::AlgorithmSection> = alg;
    assign_field!(
        alg.statements = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Statement>> = metamodelica::nil();
            for mut s in (alg.statements.clone()).into_iter().cloned() {
                let __x = stripCommentsFromStatement(s.clone(), stripAnn, stripCmt);
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    alg
}

pub(crate) fn stripCommentsFromStatement(
    mut stmt: metamodelica::Ref<SCode::Statement>,
    mut stripAnn: bool,
    mut stripCmt: bool,
) -> metamodelica::Ref<SCode::Statement> {
    let mut stmt: metamodelica::Ref<SCode::Statement> = stmt;
    let () = (match &*stmt {
        SCode::Statement::ALG_ASSIGN {
            comment: __stmt_comment,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_ASSIGN; comment = stripCommentsFromComment(__stmt_comment.clone(), stripAnn, stripCmt));
            ()
        }
        SCode::Statement::ALG_IF {
            trueBranch: __stmt_trueBranch,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_IF;
                        trueBranch = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Statement>> = metamodelica::nil();
                for mut s in (__stmt_trueBranch.clone()).into_iter().cloned() {
                    let __x = stripCommentsFromStatement(s.clone(), stripAnn, stripCmt);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        elseIfBranch = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<SCode::Statement>>)> = metamodelica::nil();
                for mut b in (var_field!((*stmt).elseIfBranch, SCode::Statement::ALG_IF).clone()).into_iter().cloned() {
                    let __x = stripCommentsFromStatementBranch(b.clone(), stripAnn, stripCmt);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        elseBranch = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Statement>> = metamodelica::nil();
                for mut s in (var_field!((*stmt).elseBranch, SCode::Statement::ALG_IF).clone()).into_iter().cloned() {
                    let __x = stripCommentsFromStatement(s.clone(), stripAnn, stripCmt);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_IF).clone(), stripAnn, stripCmt)
                    );
            ()
        }
        SCode::Statement::ALG_FOR {
            forBody: __stmt_forBody,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_FOR;
                        forBody = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Statement>> = metamodelica::nil();
                for mut s in (__stmt_forBody.clone()).into_iter().cloned() {
                    let __x = stripCommentsFromStatement(s.clone(), stripAnn, stripCmt);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_FOR).clone(), stripAnn, stripCmt)
                    );
            ()
        }
        SCode::Statement::ALG_PARFOR {
            parforBody: __stmt_parforBody,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_PARFOR;
                        parforBody = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Statement>> = metamodelica::nil();
                for mut s in (__stmt_parforBody.clone()).into_iter().cloned() {
                    let __x = stripCommentsFromStatement(s.clone(), stripAnn, stripCmt);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_PARFOR).clone(), stripAnn, stripCmt)
                    );
            ()
        }
        SCode::Statement::ALG_WHILE {
            whileBody: __stmt_whileBody,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHILE;
                        whileBody = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Statement>> = metamodelica::nil();
                for mut s in (__stmt_whileBody.clone()).into_iter().cloned() {
                    let __x = stripCommentsFromStatement(s.clone(), stripAnn, stripCmt);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_WHILE).clone(), stripAnn, stripCmt)
                    );
            ()
        }
        SCode::Statement::ALG_WHEN_A {
            branches: __stmt_branches,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHEN_A;
                        branches = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<SCode::Statement>>)> = metamodelica::nil();
                for mut b in (__stmt_branches.clone()).into_iter().cloned() {
                    let __x = stripCommentsFromStatementBranch(b.clone(), stripAnn, stripCmt);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_WHEN_A).clone(), stripAnn, stripCmt)
                    );
            ()
        }
        SCode::Statement::ALG_ASSERT {
            comment: __stmt_comment,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_ASSERT; comment = stripCommentsFromComment(__stmt_comment.clone(), stripAnn, stripCmt));
            ()
        }
        SCode::Statement::ALG_TERMINATE {
            comment: __stmt_comment,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_TERMINATE; comment = stripCommentsFromComment(__stmt_comment.clone(), stripAnn, stripCmt));
            ()
        }
        SCode::Statement::ALG_REINIT {
            comment: __stmt_comment,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_REINIT; comment = stripCommentsFromComment(__stmt_comment.clone(), stripAnn, stripCmt));
            ()
        }
        SCode::Statement::ALG_NORETCALL {
            comment: __stmt_comment,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_NORETCALL; comment = stripCommentsFromComment(__stmt_comment.clone(), stripAnn, stripCmt));
            ()
        }
        SCode::Statement::ALG_RETURN {
            comment: __stmt_comment,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_RETURN; comment = stripCommentsFromComment(__stmt_comment.clone(), stripAnn, stripCmt));
            ()
        }
        SCode::Statement::ALG_BREAK {
            comment: __stmt_comment,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_BREAK; comment = stripCommentsFromComment(__stmt_comment.clone(), stripAnn, stripCmt));
            ()
        }
        SCode::Statement::ALG_FAILURE {
            comment: __stmt_comment,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_FAILURE; comment = stripCommentsFromComment(__stmt_comment.clone(), stripAnn, stripCmt));
            ()
        }
        SCode::Statement::ALG_TRY { body: __stmt_body, .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_TRY;
                        body = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Statement>> = metamodelica::nil();
                for mut s in (__stmt_body.clone()).into_iter().cloned() {
                    let __x = stripCommentsFromStatement(s.clone(), stripAnn, stripCmt);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        elseBody = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Statement>> = metamodelica::nil();
                for mut s in (var_field!((*stmt).elseBody, SCode::Statement::ALG_TRY).clone()).into_iter().cloned() {
                    let __x = stripCommentsFromStatement(s.clone(), stripAnn, stripCmt);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        comment = stripCommentsFromComment(var_field!((*stmt).comment, SCode::Statement::ALG_TRY).clone(), stripAnn, stripCmt)
                    );
            ()
        }
        SCode::Statement::ALG_CONTINUE {
            comment: __stmt_comment,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_CONTINUE; comment = stripCommentsFromComment(__stmt_comment.clone(), stripAnn, stripCmt));
            ()
        }
    });
    stmt
}

pub(crate) fn stripCommentsFromStatementBranch(
    mut branch: (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    ),
    mut stripAnn: bool,
    mut stripCmt: bool,
) -> (
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::List<metamodelica::Ref<SCode::Statement>>,
) {
    let mut branch: (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    ) = branch;
    let mut cond: metamodelica::Ref<Absyn::Exp>;
    let mut body: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
    (cond, body) = branch;
    body = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Statement>> = metamodelica::nil();
        for mut s in (body).into_iter().cloned() {
            let __x = stripCommentsFromStatement(s.clone(), stripAnn, stripCmt);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    branch = (cond, body);
    branch
}

pub(crate) fn checkValidEnumLiteral(mut inLiteral: ArcStr, mut inInfo: &SourceInfo) -> Result<()> {
    if listMember(
        inLiteral.clone(),
        list![
            literal!("quantity"),
            literal!("min"),
            literal!("max"),
            literal!("start"),
            literal!("fixed")
        ],
    ) {
        Error::addSourceMessage(&(Error::INVALID_ENUM_LITERAL.clone()), list![inLiteral], inInfo)?;
        return Err("fail");
    }
    Ok(())
}

pub fn isRedeclareElement(mut element: &metamodelica::Ref<SCode::Element>) -> bool {
    let mut isElement: bool;
    isElement = (::match_deref::match_deref! { match element {
        Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { redeclarePrefix: SCode::Redeclare::REDECLARE { .. }, .. }, .. } => true,
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. } => false,
        Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { redeclarePrefix: SCode::Redeclare::REDECLARE { .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isElement
}

pub(crate) fn mergeSCodeOptAnn(
    mut inModOuter: Option<metamodelica::Ref<SCode::Annotation>>,
    mut inModInner: Option<metamodelica::Ref<SCode::Annotation>>,
) -> Result<Option<metamodelica::Ref<SCode::Annotation>>> {
    let mut outMod: Option<metamodelica::Ref<SCode::Annotation>>;
    outMod = (::match_deref::match_deref! { match &((inModOuter.clone(), inModInner.clone())) {
        (None, _) => {
            inModInner
        },
        (_, None) => {
            inModOuter
        },
        (Some(Deref @ SCode::Annotation { modification: mod1 }), Some(Deref @ SCode::Annotation { modification: mod2 })) => {
            let mut r#mod: metamodelica::Ref<SCode::Mod>;
            r#mod = mergeSCodeMods(mod1.clone(), mod2.clone())?;
            Some(metamodelica::Ref::new(SCode::Annotation { modification: r#mod }))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outMod)
}

pub fn mergeSCodeMods(
    mut inModOuter: metamodelica::Ref<SCode::Mod>,
    mut inModInner: metamodelica::Ref<SCode::Mod>,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut outMod: metamodelica::Ref<SCode::Mod>;
    outMod = (::match_deref::match_deref! { match &((inModOuter.clone(), inModInner.clone())) {
        (Deref @ SCode::Mod::NOMOD { .. }, _) => {
            inModInner
        },
        (_, Deref @ SCode::Mod::NOMOD { .. }) => {
            inModOuter
        },
        (Deref @ SCode::Mod::MOD { .. }, Deref @ SCode::Mod::MOD { .. }) => {
            let mut subMods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
            let mut binding: Option<metamodelica::Ref<Absyn::Exp>>;
            subMods = listAppend(var_field!((*inModOuter).subModLst, SCode::Mod::MOD).clone(), var_field!((*inModInner).subModLst, SCode::Mod::MOD).clone());
            binding = if ((var_field!((*inModOuter).binding, SCode::Mod::MOD)).is_some()) {var_field!((*inModOuter).binding, SCode::Mod::MOD).clone()} else {var_field!((*inModInner).binding, SCode::Mod::MOD).clone()};
            metamodelica::Ref::new(SCode::Mod::MOD { finalPrefix: var_field!((*inModOuter).finalPrefix, SCode::Mod::MOD).clone(), eachPrefix: var_field!((*inModOuter).eachPrefix, SCode::Mod::MOD).clone(), subModLst: subMods, binding: binding, comment: var_field!((*inModOuter).comment, SCode::Mod::MOD).clone(), info: var_field!((*inModOuter).info, SCode::Mod::MOD).clone() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outMod)
}

pub fn hasNamedExternalCall<'__b>(mut name: &'__b ArcStr, mut def: &'__b metamodelica::Ref<SCode::ClassDef>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match def {
            Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { funcName: Some(fn_name), .. }), .. } => {
                return metamodelica::stringEq(&fn_name, &name)
            },
            Deref @ SCode::ClassDef::CLASS_EXTENDS { .. } => {
                { (name, def) = (name, var_field!((**def).composition, SCode::ClassDef::CLASS_EXTENDS)); continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn classDefHasSections<'__b>(mut cdef: &'__b metamodelica::Ref<SCode::ClassDef>, mut checkExternal: bool) -> bool {
    '__tco: loop {
        match &**cdef {
            SCode::ClassDef::PARTS { .. } => {
                return !((var_field!((**cdef).normalEquationLst, SCode::ClassDef::PARTS)).is_empty()
                    && (var_field!((**cdef).initialEquationLst, SCode::ClassDef::PARTS)).is_empty()
                    && (var_field!((**cdef).normalAlgorithmLst, SCode::ClassDef::PARTS)).is_empty()
                    && (var_field!((**cdef).initialAlgorithmLst, SCode::ClassDef::PARTS)).is_empty()
                    && if (checkExternal) {
                        (var_field!((**cdef).externalDecl, SCode::ClassDef::PARTS)).is_none()
                    } else {
                        true
                    });
            }
            SCode::ClassDef::CLASS_EXTENDS { .. } => {
                (cdef, checkExternal) = (
                    var_field!((**cdef).composition, SCode::ClassDef::CLASS_EXTENDS),
                    checkExternal,
                );
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub(crate) fn mapElements(
    mut elements: metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<SCode::Element>> + 'static,
    >;

    let mut elements: metamodelica::List<metamodelica::Ref<SCode::Element>> = elements;
    elements = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
        for mut e in (elements).into_iter().cloned() {
            let __x = mapElement(e.clone(), func)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(elements)
}

pub(crate) fn mapElement(
    mut element: metamodelica::Ref<SCode::Element>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<SCode::Element>> + 'static,
    >;

    let mut element: metamodelica::Ref<SCode::Element> = element;
    let mut def: metamodelica::Ref<SCode::ClassDef>;
    let () = (match &*element {
        SCode::Element::CLASS {
            classDef: __element_classDef,
            ..
        } => {
            def = mapElementsClassDef(__element_classDef.clone(), func)?;
            if !(referenceEq(
                &*(&*def),
                &*(var_field!((*element).classDef, SCode::Element::CLASS).clone()),
            )) {
                assign_variant_field!(element => SCode::Element::CLASS; classDef = def);
            }
            ()
        }
        _ => (),
    });
    element = func(element)?;
    Ok(element)
}

pub(crate) fn mapElementsClassDef(
    mut classDef: metamodelica::Ref<SCode::ClassDef>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::Ref<SCode::ClassDef>> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<SCode::Element>> + 'static,
    >;

    let mut classDef: metamodelica::Ref<SCode::ClassDef> = classDef;
    let mut def: metamodelica::Ref<SCode::ClassDef>;
    let () = (match &*classDef {
        SCode::ClassDef::PARTS {
            elementLst: __classDef_elementLst,
            ..
        } => {
            assign_variant_field!(classDef => SCode::ClassDef::PARTS; elementLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
                for mut e in (__classDef_elementLst.clone()).into_iter().cloned() {
                    let __x = mapElement(e.clone(), func)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        SCode::ClassDef::CLASS_EXTENDS {
            composition: __classDef_composition,
            ..
        } => {
            def = mapElementsClassDef(__classDef_composition.clone(), func)?;
            if !(referenceEq(
                &*(&*def),
                &*(var_field!((*classDef).composition, SCode::ClassDef::CLASS_EXTENDS).clone()),
            )) {
                assign_variant_field!(classDef => SCode::ClassDef::CLASS_EXTENDS; composition = def);
            }
            ()
        }
        _ => (),
    });
    Ok(classDef)
}

pub fn mapEquationsList(
    mut eql: metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<SCode::Equation>) -> Result<metamodelica::Ref<SCode::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Equation>>> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<SCode::Equation>) -> Result<metamodelica::Ref<SCode::Equation>> + 'static,
    >;

    let mut eql: metamodelica::List<metamodelica::Ref<SCode::Equation>> = eql;
    eql = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Equation>> = metamodelica::nil();
        for mut e in (eql).into_iter().cloned() {
            let __x = mapEquations(e.clone(), func)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(eql)
}

pub(crate) fn mapEquations(
    mut eq: metamodelica::Ref<SCode::Equation>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<SCode::Equation>) -> Result<metamodelica::Ref<SCode::Equation>>,
) -> Result<metamodelica::Ref<SCode::Equation>> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<SCode::Equation>) -> Result<metamodelica::Ref<SCode::Equation>> + 'static,
    >;

    let mut eq: metamodelica::Ref<SCode::Equation> = eq;
    let () = (match &*eq {
        SCode::Equation::EQ_IF {
            thenBranch: __eq_thenBranch,
            ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_IF;
                        thenBranch = ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Equation>>> = metamodelica::nil();
                for mut b in (__eq_thenBranch.clone()).into_iter().cloned() {
                    let __x = mapEquationsList(b.clone(), func)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        elseBranch = mapEquationsList(var_field!((*eq).elseBranch, SCode::Equation::EQ_IF).clone(), func)?
                    );
            ()
        }
        SCode::Equation::EQ_FOR {
            eEquationLst: __eq_eEquationLst,
            ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_FOR; eEquationLst = mapEquationsList(__eq_eEquationLst.clone(), func)?);
            ()
        }
        SCode::Equation::EQ_WHEN {
            eEquationLst: __eq_eEquationLst,
            ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_WHEN;
                        eEquationLst = mapEquationsList(__eq_eEquationLst.clone(), func)?,
                        elseBranches = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<SCode::Equation>>)> = metamodelica::nil();
                for mut b in (var_field!((*eq).elseBranches, SCode::Equation::EQ_WHEN).clone()).into_iter().cloned() {
                    let __x = (Util::tuple21(b.clone()), mapEquationsList(Util::tuple22(b.clone()), func)?);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            ()
        }
        _ => (),
    });
    eq = func(eq)?;
    Ok(eq)
}

pub fn mapEquationExps(
    mut eq: metamodelica::Ref<SCode::Equation>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Absyn::Exp>>,
) -> Result<metamodelica::Ref<SCode::Equation>> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Absyn::Exp>> + 'static,
    >;

    let mut eq: metamodelica::Ref<SCode::Equation> = eq;
    let () = (match &*eq {
        SCode::Equation::EQ_IF {
            condition: __eq_condition,
            ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_IF; condition = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
                for mut e in (__eq_condition.clone()).into_iter().cloned() {
                    let __x = func(e.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        SCode::Equation::EQ_EQUALS {
            expLeft: __eq_expLeft, ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_EQUALS;
                expLeft = func(__eq_expLeft.clone())?,
                expRight = func(var_field!((*eq).expRight, SCode::Equation::EQ_EQUALS).clone())?
            );
            ()
        }
        SCode::Equation::EQ_PDE {
            expLeft: __eq_expLeft, ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_PDE;
                expLeft = func(__eq_expLeft.clone())?,
                expRight = func(var_field!((*eq).expRight, SCode::Equation::EQ_PDE).clone())?,
                domain = AbsynUtil::mapCrefExps(var_field!((*eq).domain, SCode::Equation::EQ_PDE).clone(), func)?
            );
            ()
        }
        SCode::Equation::EQ_CONNECT {
            crefLeft: __eq_crefLeft,
            ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_CONNECT;
                crefLeft = AbsynUtil::mapCrefExps(__eq_crefLeft.clone(), func)?,
                crefRight = AbsynUtil::mapCrefExps(var_field!((*eq).crefRight, SCode::Equation::EQ_CONNECT).clone(), func)?
            );
            ()
        }
        SCode::Equation::EQ_FOR { .. } => {
            if (var_field!((*eq).range, SCode::Equation::EQ_FOR)).is_some() {
                assign_variant_field!(eq => SCode::Equation::EQ_FOR; range = Some(func(Util::getOption(var_field!((*eq).range, SCode::Equation::EQ_FOR).clone())?)?));
            }
            ()
        }
        SCode::Equation::EQ_WHEN {
            condition: __eq_condition,
            ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_WHEN;
                        condition = func(__eq_condition.clone())?,
                        elseBranches = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<SCode::Equation>>)> = metamodelica::nil();
                for mut b in (var_field!((*eq).elseBranches, SCode::Equation::EQ_WHEN).clone()).into_iter().cloned() {
                    let __x = Util::applyTuple21(b.clone(), func)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            ()
        }
        SCode::Equation::EQ_ASSERT {
            condition: __eq_condition,
            ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_ASSERT;
                condition = func(__eq_condition.clone())?,
                message = func(var_field!((*eq).message, SCode::Equation::EQ_ASSERT).clone())?,
                level = func(var_field!((*eq).level, SCode::Equation::EQ_ASSERT).clone())?
            );
            ()
        }
        SCode::Equation::EQ_TERMINATE {
            message: __eq_message, ..
        } => {
            assign_variant_field!(eq => SCode::Equation::EQ_TERMINATE; message = func(__eq_message.clone())?);
            ()
        }
        SCode::Equation::EQ_REINIT { cref: __eq_cref, .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_REINIT;
                cref = func(__eq_cref.clone())?,
                expReinit = func(var_field!((*eq).expReinit, SCode::Equation::EQ_REINIT).clone())?
            );
            ()
        }
        SCode::Equation::EQ_NORETCALL { exp: __eq_exp, .. } => {
            assign_variant_field!(eq => SCode::Equation::EQ_NORETCALL; exp = func(__eq_exp.clone())?);
            ()
        }
    });
    Ok(eq)
}

pub fn mapAlgorithmStatements(
    mut alg: metamodelica::Ref<SCode::AlgorithmSection>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<SCode::Statement>) -> Result<metamodelica::Ref<SCode::Statement>>,
) -> Result<metamodelica::Ref<SCode::AlgorithmSection>> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<SCode::Statement>) -> Result<metamodelica::Ref<SCode::Statement>>
            + 'static,
    >;

    let mut alg: metamodelica::Ref<SCode::AlgorithmSection> = alg;
    assign_field!(alg.statements = mapStatementsList(alg.statements.clone(), func)?);
    Ok(alg)
}

pub(crate) fn mapStatementsList(
    mut statements: metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<SCode::Statement>) -> Result<metamodelica::Ref<SCode::Statement>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Statement>>> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<SCode::Statement>) -> Result<metamodelica::Ref<SCode::Statement>>
            + 'static,
    >;

    let mut statements: metamodelica::List<metamodelica::Ref<SCode::Statement>> = statements;
    statements = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Statement>> = metamodelica::nil();
        for mut s in (statements).into_iter().cloned() {
            let __x = mapStatements(s.clone(), func)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(statements)
}

pub(crate) fn mapStatements(
    mut stmt: metamodelica::Ref<SCode::Statement>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<SCode::Statement>) -> Result<metamodelica::Ref<SCode::Statement>>,
) -> Result<metamodelica::Ref<SCode::Statement>> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<SCode::Statement>) -> Result<metamodelica::Ref<SCode::Statement>>
            + 'static,
    >;

    let mut stmt: metamodelica::Ref<SCode::Statement> = stmt;
    let () = (match &*stmt {
        SCode::Statement::ALG_IF {
            trueBranch: __stmt_trueBranch,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_IF;
                        trueBranch = mapStatementsList(__stmt_trueBranch.clone(), func)?,
                        elseIfBranch = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<SCode::Statement>>)> = metamodelica::nil();
                for mut b in (var_field!((*stmt).elseIfBranch, SCode::Statement::ALG_IF).clone()).into_iter().cloned() {
                    let __x = (Util::tuple21(b.clone()), mapStatementsList(Util::tuple22(b.clone()), func)?);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        elseBranch = mapStatementsList(var_field!((*stmt).elseBranch, SCode::Statement::ALG_IF).clone(), func)?
                    );
            ()
        }
        SCode::Statement::ALG_FOR {
            forBody: __stmt_forBody,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_FOR; forBody = mapStatementsList(__stmt_forBody.clone(), func)?);
            ()
        }
        SCode::Statement::ALG_PARFOR {
            parforBody: __stmt_parforBody,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_PARFOR; parforBody = mapStatementsList(__stmt_parforBody.clone(), func)?);
            ()
        }
        SCode::Statement::ALG_WHILE {
            whileBody: __stmt_whileBody,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHILE; whileBody = mapStatementsList(__stmt_whileBody.clone(), func)?);
            ()
        }
        SCode::Statement::ALG_WHEN_A {
            branches: __stmt_branches,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHEN_A; branches = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<SCode::Statement>>)> = metamodelica::nil();
                for mut b in (__stmt_branches.clone()).into_iter().cloned() {
                    let __x = (Util::tuple21(b.clone()), mapStatementsList(Util::tuple22(b.clone()), func)?);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        SCode::Statement::ALG_FAILURE {
            stmts: __stmt_stmts, ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_FAILURE; stmts = mapStatementsList(__stmt_stmts.clone(), func)?);
            ()
        }
        SCode::Statement::ALG_TRY { body: __stmt_body, .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_TRY; body = mapStatementsList(__stmt_body.clone(), func)?);
            assign_variant_field!(stmt => SCode::Statement::ALG_TRY; elseBody = mapStatementsList(var_field!((*stmt).body, SCode::Statement::ALG_TRY).clone(), func)?);
            ()
        }
        _ => (),
    });
    stmt = func(stmt)?;
    Ok(stmt)
}

pub fn mapStatementExps(
    mut stmt: metamodelica::Ref<SCode::Statement>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Absyn::Exp>>,
) -> Result<metamodelica::Ref<SCode::Statement>> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<Absyn::Exp>> + 'static,
    >;

    let mut stmt: metamodelica::Ref<SCode::Statement> = stmt;
    let () = (match &*stmt {
        SCode::Statement::ALG_ASSIGN {
            assignComponent: __stmt_assignComponent,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_ASSIGN;
                assignComponent = func(__stmt_assignComponent.clone())?,
                value = func(var_field!((*stmt).value, SCode::Statement::ALG_ASSIGN).clone())?
            );
            ()
        }
        SCode::Statement::ALG_IF {
            boolExpr: __stmt_boolExpr,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_IF;
                        boolExpr = func(__stmt_boolExpr.clone())?,
                        elseIfBranch = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<SCode::Statement>>)> = metamodelica::nil();
                for mut b in (var_field!((*stmt).elseIfBranch, SCode::Statement::ALG_IF).clone()).into_iter().cloned() {
                    let __x = (func(Util::tuple21(b.clone()))?, Util::tuple22(b.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            ()
        }
        SCode::Statement::ALG_FOR { .. } => {
            if (var_field!((*stmt).range, SCode::Statement::ALG_FOR)).is_some() {
                assign_variant_field!(stmt => SCode::Statement::ALG_FOR; range = Some(func(Util::getOption(var_field!((*stmt).range, SCode::Statement::ALG_FOR).clone())?)?));
            }
            ()
        }
        SCode::Statement::ALG_PARFOR { .. } => {
            if (var_field!((*stmt).range, SCode::Statement::ALG_PARFOR)).is_some() {
                assign_variant_field!(stmt => SCode::Statement::ALG_PARFOR; range = Some(func(Util::getOption(var_field!((*stmt).range, SCode::Statement::ALG_PARFOR).clone())?)?));
            }
            ()
        }
        SCode::Statement::ALG_WHILE {
            boolExpr: __stmt_boolExpr,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHILE; boolExpr = func(__stmt_boolExpr.clone())?);
            ()
        }
        SCode::Statement::ALG_WHEN_A {
            branches: __stmt_branches,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_WHEN_A; branches = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<SCode::Statement>>)> = metamodelica::nil();
                for mut b in (__stmt_branches.clone()).into_iter().cloned() {
                    let __x = (func(Util::tuple21(b.clone()))?, Util::tuple22(b.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        SCode::Statement::ALG_ASSERT {
            condition: __stmt_condition,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_ASSERT;
                condition = func(__stmt_condition.clone())?,
                message = func(var_field!((*stmt).message, SCode::Statement::ALG_ASSERT).clone())?,
                level = func(var_field!((*stmt).level, SCode::Statement::ALG_ASSERT).clone())?
            );
            ()
        }
        SCode::Statement::ALG_TERMINATE {
            message: __stmt_message,
            ..
        } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_TERMINATE; message = func(__stmt_message.clone())?);
            ()
        }
        SCode::Statement::ALG_REINIT { cref: __stmt_cref, .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_REINIT;
                cref = func(__stmt_cref.clone())?,
                newValue = func(var_field!((*stmt).newValue, SCode::Statement::ALG_REINIT).clone())?
            );
            ()
        }
        SCode::Statement::ALG_NORETCALL { exp: __stmt_exp, .. } => {
            assign_variant_field!(stmt => SCode::Statement::ALG_NORETCALL; exp = func(__stmt_exp.clone())?);
            ()
        }
        _ => (),
    });
    Ok(stmt)
}

pub fn lookupModInMod(mut name: &ArcStr, mut r#mod: &metamodelica::Ref<SCode::Mod>) -> metamodelica::Ref<SCode::Mod> {
    let mut outMod: metamodelica::Ref<SCode::Mod>;
    outMod = (match &**r#mod {
        SCode::Mod::MOD {
            subModLst: __mod_subModLst,
            ..
        } => {
            for mut m in &*__mod_subModLst.clone() {
                if metamodelica::stringEq(&m.ident, &name) {
                    outMod = m.r#mod.clone();
                    return outMod;
                }
            }
            openmodelica_frontend_types::SCode::Mod::interned_NOMOD()
        }
        _ => openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
    });
    outMod
}

pub fn isNonEmptyAlgorithm(mut alg: &metamodelica::Ref<SCode::AlgorithmSection>) -> bool {
    let mut res: bool = !((alg.statements).is_empty());
    res
}

pub fn onlyLiteralsInMod(mut r#mod: &metamodelica::Ref<SCode::Mod>) -> Result<bool> {
    let mut onlyLiterals: bool;
    onlyLiterals = (match &**r#mod {
        SCode::Mod::MOD {
            binding: __mod_binding,
            subModLst: __mod_subModLst,
            ..
        } => {
            if (__mod_binding).is_some() {
                onlyLiterals = AbsynUtil::onlyLiteralsInExp(Util::getOption(__mod_binding.clone())?)?;
            } else {
                onlyLiterals = true;
            }
            if onlyLiterals {
                for mut m in &*__mod_subModLst.clone() {
                    onlyLiterals = onlyLiteralsInMod(&m.r#mod)?;
                    if !(onlyLiterals) {
                        break;
                    }
                }
            }
            onlyLiterals
        }
        _ => true,
    });
    Ok(onlyLiterals)
}

pub fn transformPathedElementInProgram(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<SCode::Element>> + 'static,
    >,
    mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<(metamodelica::List<metamodelica::Ref<SCode::Element>>, bool)> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<SCode::Element>> + 'static,
    >;

    let mut program: metamodelica::List<metamodelica::Ref<SCode::Element>> = program;
    let mut success: bool;
    (program, success) = List::findMap(
        program,
        &({
            let __pe_b0 = path.clone();
            let __pe_b1: Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<SCode::Element>>
                    + 'static,
            > = func.clone();
            move |__pe_a2| transformPathedElementInElement(__pe_b0.clone(), __pe_b1.clone(), __pe_a2)
        }),
    )?;
    Ok((program, success))
}

pub(crate) fn transformPathedElementInElement(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<SCode::Element>> + 'static,
    >,
    mut element: metamodelica::Ref<SCode::Element>,
) -> Result<(metamodelica::Ref<SCode::Element>, bool)> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<SCode::Element>> + 'static,
    >;

    let mut element: metamodelica::Ref<SCode::Element> = element;
    let mut success: bool;
    let mut cdef: metamodelica::Ref<SCode::ClassDef>;
    success = isElementNamed(&(AbsynUtil::pathFirstIdent(&path)), &element);
    if success {
        if AbsynUtil::pathIsIdent(&path) {
            element = func(element)?;
        } else if isClass(&element) {
            (cdef, success) =
                transformPathedElementInClassDef(&(AbsynUtil::pathRest(path)?), func.clone(), getClassDef(&element)?)?;
            if success {
                element = setClassDef(cdef, element)?;
            }
        }
    }
    Ok((element, success))
}

pub(crate) fn transformPathedElementInClassDef(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<SCode::Element>> + 'static,
    >,
    mut cls: metamodelica::Ref<SCode::ClassDef>,
) -> Result<(metamodelica::Ref<SCode::ClassDef>, bool)> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<SCode::Element>) -> Result<metamodelica::Ref<SCode::Element>> + 'static,
    >;

    let mut cls: metamodelica::Ref<SCode::ClassDef> = cls;
    let mut success: bool;
    let mut elems: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut cdef: metamodelica::Ref<SCode::ClassDef>;
    success = (match &*cls {
        SCode::ClassDef::PARTS {
            elementLst: __cls_elementLst,
            ..
        } => {
            (elems, success) = transformPathedElementInProgram(path, func.clone(), __cls_elementLst.clone())?;
            if success {
                assign_variant_field!(cls => SCode::ClassDef::PARTS; elementLst = elems);
            }
            success
        }
        SCode::ClassDef::CLASS_EXTENDS {
            composition: __cls_composition,
            ..
        } => {
            (cdef, success) = transformPathedElementInClassDef(path, func.clone(), __cls_composition.clone())?;
            if success {
                assign_variant_field!(cls => SCode::ClassDef::CLASS_EXTENDS; composition = cdef);
            }
            success
        }
        _ => false,
    });
    Ok((cls, success))
}

pub fn makeMod(
    mut isFinal: bool,
    mut isEach: bool,
    mut subMods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    mut binding: Option<metamodelica::Ref<Absyn::Exp>>,
    mut comment: Option<ArcStr>,
    mut info: SourceInfo,
) -> metamodelica::Ref<SCode::Mod> {
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    r#mod = metamodelica::Ref::new(SCode::Mod::MOD {
        finalPrefix: if (isFinal) {
            openmodelica_frontend_types::SCode::Final::FINAL
        } else {
            openmodelica_frontend_types::SCode::Final::NOT_FINAL
        },
        eachPrefix: if (isEach) {
            openmodelica_frontend_types::SCode::Each::EACH
        } else {
            openmodelica_frontend_types::SCode::Each::NOT_EACH
        },
        subModLst: subMods,
        binding: binding,
        comment: comment,
        info: info,
    });
    r#mod
}

pub(crate) fn makeSingleAnnotation(
    mut name: ArcStr,
    mut value: metamodelica::Ref<Absyn::Exp>,
) -> metamodelica::Ref<SCode::Annotation> {
    let mut ann: metamodelica::Ref<SCode::Annotation>;
    ann = metamodelica::Ref::new(SCode::Annotation {
        modification: metamodelica::Ref::new(SCode::Mod::MOD {
            finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
            eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
            subModLst: list![metamodelica::Ref::new(SCode::SubMod {
                ident: name,
                r#mod: metamodelica::Ref::new(SCode::Mod::MOD {
                    finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL,
                    eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH,
                    subModLst: metamodelica::nil(),
                    binding: Some(value),
                    comment: None,
                    info: Absyn::dummyInfo.clone()
                })
            })],
            binding: None,
            comment: None,
            info: Absyn::dummyInfo.clone(),
        }),
    });
    ann
}

pub fn setAnnotationInComment(
    mut name: ArcStr,
    mut value: metamodelica::Ref<Absyn::Exp>,
    mut cmt: metamodelica::Ref<SCode::Comment>,
    mut replace: bool,
) -> Result<metamodelica::Ref<SCode::Comment>> {
    let mut cmt: metamodelica::Ref<SCode::Comment> = cmt;
    if (cmt.annotation_).is_none() {
        assign_field!(cmt.annotation_ = Some(makeSingleAnnotation(name, value)));
        return Ok(cmt);
    } else {
        assign_field!(
            cmt.annotation_ = Some(setAnnotationValue(
                name,
                value,
                Util::getOption(cmt.annotation_.clone())?,
                replace
            )?)
        );
    }
    Ok(cmt)
}

pub(crate) fn setAnnotationValue(
    mut name: ArcStr,
    mut value: metamodelica::Ref<Absyn::Exp>,
    mut ann: metamodelica::Ref<SCode::Annotation>,
    mut replace: bool,
) -> Result<metamodelica::Ref<SCode::Annotation>> {
    fn replace_mod(
        mut name: &ArcStr,
        mut value: metamodelica::Ref<Absyn::Exp>,
        mut replace: bool,
        mut r#mod: metamodelica::Ref<SCode::SubMod>,
    ) -> (metamodelica::Ref<SCode::SubMod>, bool) {
        let mut r#mod: metamodelica::Ref<SCode::SubMod> = r#mod;
        let mut found: bool;
        found = metamodelica::stringEq(&r#mod.ident, &name);
        if found && replace {
            assign_field!(r#mod.r#mod = setModifierBinding(Some(value), r#mod.r#mod.clone()));
        }
        (r#mod, found)
    }

    let mut ann: metamodelica::Ref<SCode::Annotation> = ann;
    let mut r#mod: metamodelica::Ref<SCode::Mod>;
    let mut submods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    let mut found: bool;
    let () = (::match_deref::match_deref! { match &(ann.clone()) {
        Deref @ SCode::Annotation { modification: __esc_mod @ Deref @ SCode::Mod::MOD { .. } } => {
            r#mod = (*__esc_mod).clone();
            (submods, found) = List::findMap(var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone(), &({ let __pe_b0 = name.clone(); let __pe_b1 = value.clone(); let __pe_b2 = replace; move |__pe_a3| Ok(replace_mod(&__pe_b0, __pe_b1.clone(), __pe_b2.clone(), __pe_a3)) }))?;
            if !(found) {
                submods = metamodelica::cons(metamodelica::Ref::new(SCode::SubMod { ident: name, r#mod: makeMod(false, false, metamodelica::nil(), Some(value), None, Absyn::dummyInfo.clone()) }), submods);
            }
            assign_variant_field!(r#mod => SCode::Mod::MOD; subModLst = submods);
            assign_field!(ann.modification = r#mod.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ann)
}
