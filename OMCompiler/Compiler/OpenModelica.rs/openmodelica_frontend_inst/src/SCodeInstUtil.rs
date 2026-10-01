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
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util_datatypes_basic::List;

fn constantBindingOrNone(
    mut inBinding: Option<metamodelica::Ref<Absyn::Exp>>,
) -> Result<Option<metamodelica::Ref<Absyn::Exp>>> {
    let mut outBinding: Option<metamodelica::Ref<Absyn::Exp>>;
    outBinding = (::match_deref::match_deref! { match &(inBinding.clone()) {
        Some(e) => {
            if (((AbsynUtil::getCrefFromExp(e.clone(), true, true)?)).is_empty()) {inBinding} else {None}
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBinding)
}

pub fn removeNonConstantBindingsKeepRedeclares(
    mut inMod: metamodelica::Ref<SCode::Mod>,
    mut onlyRedeclares: bool,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut outMod: metamodelica::Ref<SCode::Mod>;
    outMod = (match &*inMod {
        SCode::Mod::MOD {
            finalPrefix: fp,
            eachPrefix: ep,
            subModLst: sl,
            binding,
            comment: cmt,
            info: i,
        } => {
            let mut sl = (*sl).clone();
            let mut binding = (*binding).clone();
            binding = if (onlyRedeclares) {
                None
            } else {
                constantBindingOrNone(binding.clone())?
            };
            sl = removeNonConstantBindingsKeepRedeclaresFromSubMod(metamodelica::AsArg::as_arg(&sl), onlyRedeclares)?;
            metamodelica::Ref::new(SCode::Mod::MOD {
                finalPrefix: fp.clone(),
                eachPrefix: ep.clone(),
                subModLst: sl.clone(),
                binding: binding.clone(),
                comment: cmt.clone(),
                info: i.clone(),
            })
        }
        SCode::Mod::REDECL { .. } => inMod,
        _ => inMod,
    });
    Ok(outMod)
}

fn removeNonConstantBindingsKeepRedeclaresFromSubMod(
    mut inSl: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    mut onlyRedeclares: bool,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::SubMod>>> {
    let mut outSl: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    outSl = (::match_deref::match_deref! { match inSl {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::SubMod { ident: n, r#mod: m }, tail: rest } => {
            let mut sl: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
            let mut m = (*m).clone();
            m = removeNonConstantBindingsKeepRedeclares(m.clone(), onlyRedeclares)?;
            sl = removeNonConstantBindingsKeepRedeclaresFromSubMod(rest, onlyRedeclares)?;
            metamodelica::cons(metamodelica::Ref::new(SCode::SubMod { ident: n.clone(), r#mod: m.clone() }), sl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSl)
}

pub fn addRedeclareAsElementsToExtends(
    mut inElements: &metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut redeclareElements: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut outExtendsElements: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    outExtendsElements = (::match_deref::match_deref! { match &((inElements.clone(), redeclareElements)) {
        (_, Deref @ metamodelica::ListNode::Nil) => {
            inElements.clone()
        },
        (Deref @ metamodelica::ListNode::Nil, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath, visibility, modifications: r#mod, ann, info }, tail: rest }, redecls) => {
            let mut out: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            let mut redeclareMod: metamodelica::Ref<SCode::Mod>;
            let mut submods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
            let mut r#mod = (*r#mod).clone();
            submods = makeElementsIntoSubMods(openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, redecls.clone())?;
            redeclareMod = metamodelica::Ref::new(SCode::Mod::MOD { finalPrefix: openmodelica_frontend_types::SCode::Final::NOT_FINAL, eachPrefix: openmodelica_frontend_types::SCode::Each::NOT_EACH, subModLst: submods, binding: None, comment: None, info: info.clone() });
            r#mod = SCodeUtil::mergeSCodeMods(redeclareMod, r#mod.clone())?;
            out = addRedeclareAsElementsToExtends(metamodelica::AsArg::as_arg(&rest), redecls.clone())?;
            metamodelica::cons(metamodelica::Ref::new(SCode::Element::EXTENDS { baseClassPath: baseClassPath.clone(), visibility: visibility.clone(), modifications: r#mod.clone(), ann: ann.clone(), info: info.clone() }), out)
        },
        (Deref @ metamodelica::ListNode::Cons { head: el, tail: rest }, redecls) => {
            let mut out: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            out = addRedeclareAsElementsToExtends(metamodelica::AsArg::as_arg(&rest), redecls.clone())?;
            metamodelica::cons(el.clone(), out)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExtendsElements)
}

fn makeElementsIntoSubMods(
    mut inFinal: SCode::Final,
    mut inEach: SCode::Each,
    mut inElements: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::SubMod>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inFinal, inEach, inElements)) {
            (_, _, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(metamodelica::nil())
            },
            (f, e, Deref @ metamodelica::ListNode::Cons { head: el @ Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. }, tail: rest }) => {
                let mut newSubMods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- AbsynToSCode.makeElementsIntoSubMods ignoring class-extends redeclare-as-element: ")); __mm_s.push_str(&*SCodeDump::unparseElementStr(el.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                { (inFinal, inEach, inElements) = (f.clone(), e.clone(), rest.clone()); continue '__tco; }
            },
            (f, e, Deref @ metamodelica::ListNode::Cons { head: el @ Deref @ SCode::Element::COMPONENT { name: n, .. }, tail: rest }) => {
                let mut newSubMods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
                newSubMods = makeElementsIntoSubMods(f.clone(), e.clone(), rest.clone())?;
                return Ok(metamodelica::cons(metamodelica::Ref::new(SCode::SubMod { ident: n.clone(), r#mod: metamodelica::Ref::new(SCode::Mod::REDECL { finalPrefix: f.clone(), eachPrefix: e.clone(), element: el.clone() }) }), newSubMods))
            },
            (f, e, Deref @ metamodelica::ListNode::Cons { head: el @ Deref @ SCode::Element::CLASS { name: n, .. }, tail: rest }) => {
                let mut newSubMods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
                newSubMods = makeElementsIntoSubMods(f.clone(), e.clone(), rest.clone())?;
                return Ok(metamodelica::cons(metamodelica::Ref::new(SCode::SubMod { ident: n.clone(), r#mod: metamodelica::Ref::new(SCode::Mod::REDECL { finalPrefix: f.clone(), eachPrefix: e.clone(), element: el.clone() }) }), newSubMods))
            },
            (f, e, Deref @ metamodelica::ListNode::Cons { head: el, tail: rest }) => {
                let mut newSubMods: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- AbsynToSCode.makeElementsIntoSubMods ignoring redeclare-as-element redeclaration: ")); __mm_s.push_str(&*SCodeDump::unparseElementStr(el.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                { (inFinal, inEach, inElements) = (f.clone(), e.clone(), rest.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn removeReferenceInBinding(
    mut inBinding: Option<metamodelica::Ref<Absyn::Exp>>,
    mut inCref: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<Option<metamodelica::Ref<Absyn::Exp>>> {
    let mut outBinding: Option<metamodelica::Ref<Absyn::Exp>>;
    outBinding = (::match_deref::match_deref! { match &(inBinding.clone()) {
        Some(e) => {
            let mut crlst1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
            let mut crlst2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
            crlst1 = AbsynUtil::getCrefFromExp(e.clone(), true, true)?;
            crlst2 = AbsynUtil::removeCrefFromCrefs(&crlst1, inCref)?;
            if (intEq(((crlst1).len() as i32), ((crlst2).len() as i32))) {inBinding} else {None}
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBinding)
}

pub fn removeSelfReferenceFromMod(
    mut inMod: metamodelica::Ref<SCode::Mod>,
    mut inCref: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut outMod: metamodelica::Ref<SCode::Mod>;
    outMod = (match &*inMod {
        SCode::Mod::MOD {
            finalPrefix: fp,
            eachPrefix: ep,
            subModLst: sl,
            binding,
            comment: cmt,
            info: i,
        } => {
            let mut sl = (*sl).clone();
            let mut binding = (*binding).clone();
            binding = removeReferenceInBinding(binding.clone(), inCref.clone())?;
            sl = removeSelfReferenceFromSubMod(metamodelica::AsArg::as_arg(&sl), &inCref)?;
            metamodelica::Ref::new(SCode::Mod::MOD {
                finalPrefix: fp.clone(),
                eachPrefix: ep.clone(),
                subModLst: sl.clone(),
                binding: binding.clone(),
                comment: cmt.clone(),
                info: i.clone(),
            })
        }
        SCode::Mod::REDECL { .. } => inMod,
        _ => inMod,
    });
    Ok(outMod)
}

fn removeSelfReferenceFromSubMod(
    mut inSl: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    mut inCref: &metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::SubMod>>> {
    let mut outSl: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    outSl = (::match_deref::match_deref! { match inSl {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::SubMod { ident: n, r#mod: m }, tail: rest } => {
            let mut sl: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
            let mut m = (*m).clone();
            m = removeSelfReferenceFromMod(m.clone(), inCref.clone())?;
            sl = removeSelfReferenceFromSubMod(rest, inCref)?;
            metamodelica::cons(metamodelica::Ref::new(SCode::SubMod { ident: n.clone(), r#mod: m.clone() }), sl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSl)
}

fn expandEnumerationSubMod(
    mut inSubMod: metamodelica::Ref<SCode::SubMod>,
    mut inChanged: bool,
) -> Result<(metamodelica::Ref<SCode::SubMod>, bool)> {
    let mut outSubMod: metamodelica::Ref<SCode::SubMod>;
    let mut outChanged: bool;
    (outSubMod, outChanged) = (match &*inSubMod {
        SCode::SubMod { ident, r#mod } => {
            let mut mod1: metamodelica::Ref<SCode::Mod>;
            mod1 = expandEnumerationMod(r#mod.clone())?;
            if (referenceEq(&*(r#mod.clone()), &*(&*mod1))) {
                (inSubMod, inChanged)
            } else {
                (
                    metamodelica::Ref::new(SCode::SubMod {
                        ident: ident.clone(),
                        r#mod: mod1,
                    }),
                    true,
                )
            }
        }
        _ => (inSubMod, inChanged),
    });
    Ok((outSubMod, outChanged))
}

pub fn expandEnumerationMod(mut inMod: metamodelica::Ref<SCode::Mod>) -> Result<metamodelica::Ref<SCode::Mod>> {
    let mut outMod: metamodelica::Ref<SCode::Mod>;
    let mut f: SCode::Final;
    let mut e: SCode::Each;
    let mut el: metamodelica::Ref<SCode::Element>;
    let mut el1: metamodelica::Ref<SCode::Element>;
    let mut submod: metamodelica::List<metamodelica::Ref<SCode::SubMod>>;
    let mut binding: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut info: SourceInfo;
    let mut changed: bool;
    let mut cmt: Option<ArcStr>;
    outMod = (match &*inMod {
        SCode::Mod::REDECL {
            finalPrefix: __esc_f,
            eachPrefix: __esc_e,
            element: __esc_el,
        } => {
            f = (*__esc_f).clone();
            e = (*__esc_e).clone();
            el = (*__esc_el).clone();
            el1 = expandEnumerationClass(el.clone())?;
            if (referenceEq(&*(el.clone()), &*(&*el1))) {
                inMod
            } else {
                metamodelica::Ref::new(SCode::Mod::REDECL {
                    finalPrefix: f.clone(),
                    eachPrefix: e.clone(),
                    element: el1,
                })
            }
        }
        SCode::Mod::MOD {
            finalPrefix: __esc_f,
            eachPrefix: __esc_e,
            subModLst: __esc_submod,
            binding: __esc_binding,
            comment: __esc_cmt,
            info: __esc_info,
        } => {
            f = (*__esc_f).clone();
            e = (*__esc_e).clone();
            submod = (*__esc_submod).clone();
            binding = (*__esc_binding).clone();
            cmt = (*__esc_cmt).clone();
            info = (*__esc_info).clone();
            (submod, changed) = List::mapFold(metamodelica::AsArg::as_arg(&submod), &expandEnumerationSubMod, false)?;
            if (changed) {
                metamodelica::Ref::new(SCode::Mod::MOD {
                    finalPrefix: f.clone(),
                    eachPrefix: e.clone(),
                    subModLst: submod.clone(),
                    binding: binding.clone(),
                    comment: cmt.clone(),
                    info: info.clone(),
                })
            } else {
                inMod
            }
        }
        _ => inMod,
    });
    Ok(outMod)
}

pub fn expandEnumerationClass(
    mut inElement: metamodelica::Ref<SCode::Element>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut outElement: metamodelica::Ref<SCode::Element>;
    outElement = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::CLASS { name: n, restriction: SCode::Restriction::R_TYPE { .. }, prefixes, classDef: Deref @ SCode::ClassDef::ENUMERATION { enumLst: l }, cmt, info, .. } => {
            let mut c: metamodelica::Ref<SCode::Element>;
            c = expandEnumeration(n.clone(), l.clone(), prefixes.clone(), cmt.clone(), info.clone())?;
            c
        },
        Deref @ SCode::Element::EXTENDS { baseClassPath: p, visibility: v, modifications: m, ann, info } => {
            let mut m1: metamodelica::Ref<SCode::Mod>;
            m1 = expandEnumerationMod(m.clone())?;
            if (referenceEq(&*(m.clone()),&*(&*m1))) {inElement} else {metamodelica::Ref::new(SCode::Element::EXTENDS { baseClassPath: p.clone(), visibility: v.clone(), modifications: m1, ann: ann.clone(), info: info.clone() })}
        },
        _ => {
            inElement
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElement)
}

pub fn expandEnumeration(
    mut n: ArcStr,
    mut l: metamodelica::List<metamodelica::Ref<SCode::Enum>>,
    mut prefixes: metamodelica::Ref<SCode::Prefixes>,
    mut cmt: metamodelica::Ref<SCode::Comment>,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut outClass: metamodelica::Ref<SCode::Element>;
    outClass = metamodelica::Ref::new(SCode::Element::CLASS {
        name: n,
        prefixes: prefixes,
        encapsulatedPrefix: openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
        partialPrefix: openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL,
        restriction: openmodelica_frontend_types::SCode::Restriction::R_ENUMERATION,
        classDef: makeEnumParts(l, info.clone())?,
        cmt: cmt,
        info: info,
    });
    Ok(outClass)
}

fn makeEnumParts(
    mut inEnumLst: metamodelica::List<metamodelica::Ref<SCode::Enum>>,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<SCode::ClassDef>> {
    let mut classDef: metamodelica::Ref<SCode::ClassDef>;
    classDef = metamodelica::Ref::new(SCode::ClassDef::PARTS {
        elementLst: makeEnumComponents(inEnumLst, info)?,
        normalEquationLst: metamodelica::nil(),
        initialEquationLst: metamodelica::nil(),
        normalAlgorithmLst: metamodelica::nil(),
        initialAlgorithmLst: metamodelica::nil(),
        constraintLst: metamodelica::nil(),
        clsattrs: metamodelica::nil(),
        externalDecl: None,
    });
    Ok(classDef)
}

fn makeEnumComponents(
    mut inEnumLst: metamodelica::List<metamodelica::Ref<SCode::Enum>>,
    mut info: SourceInfo,
) -> Result<metamodelica::List<metamodelica::Ref<SCode::Element>>> {
    let mut outSCodeElementLst: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    outSCodeElementLst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
        for mut e in (inEnumLst).into_iter().cloned() {
            let __x = SCodeUtil::makeEnumType(&(e.clone()), info.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outSCodeElementLst)
}
