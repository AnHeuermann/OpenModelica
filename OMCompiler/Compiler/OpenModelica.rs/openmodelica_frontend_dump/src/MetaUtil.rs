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
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util_datatypes_basic::List;

pub fn createMetaClassesInProgram(mut inProgram: Absyn::Program) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program = inProgram;
    let mut classes: metamodelica::List<metamodelica::Ref<Absyn::Class>> = metamodelica::nil();
    let mut meta_classes: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
    if !(Config::acceptMetaModelicaGrammar()?) {
        return Ok(outProgram);
    }
    let () = (match outProgram.clone() {
        Absyn::Program { .. } => {
            for mut c in &*outProgram.classes.clone() {
                let mut c = c.clone();
                (c, meta_classes) = createMetaClasses(c)?;
                classes = metamodelica::cons(c, listAppend(meta_classes, classes));
            }
            outProgram.classes = Dangerous::listReverseInPlace(classes);
            ()
        }
        _ => (),
    });
    Ok(outProgram)
}

fn createMetaClasses(
    mut inClass: metamodelica::Ref<Absyn::Class>,
) -> Result<(
    metamodelica::Ref<Absyn::Class>,
    metamodelica::List<metamodelica::Ref<Absyn::Class>>,
)> {
    let mut outClass: metamodelica::Ref<Absyn::Class> = inClass;
    let mut outMetaClasses: metamodelica::List<metamodelica::Ref<Absyn::Class>> = metamodelica::nil();
    let mut body: metamodelica::Ref<Absyn::ClassDef>;
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    let () = (::match_deref::match_deref! { match &(outClass.clone()) {
        Deref @ Absyn::Class { restriction: Absyn::Restriction::R_UNIONTYPE { .. }, body: __esc_body @ Deref @ Absyn::ClassDef::PARTS { classParts: __esc_parts, .. }, .. } => {
            body = (*__esc_body).clone();
            parts = (*__esc_parts).clone();
            (parts, outMetaClasses) = fixClassParts(parts.clone(), outClass.name.clone(), var_field!((*body).typeVars, Absyn::ClassDef::PARTS).clone())?;
            assign_variant_field!(body => Absyn::ClassDef::PARTS; classParts = parts.clone());
            assign_field!(outClass.body = body.clone());
            ()
        },
        Deref @ Absyn::Class { restriction: Absyn::Restriction::R_UNIONTYPE { .. }, body: __esc_body @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: __esc_parts, .. }, .. } => {
            body = (*__esc_body).clone();
            parts = (*__esc_parts).clone();
            (parts, outMetaClasses) = fixClassParts(parts.clone(), outClass.name.clone(), metamodelica::nil())?;
            assign_variant_field!(body => Absyn::ClassDef::CLASS_EXTENDS; parts = parts.clone());
            assign_field!(outClass.body = body.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    let () = (::match_deref::match_deref! { match &(outClass.clone()) {
        Deref @ Absyn::Class { body: __esc_body @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
            body = (*__esc_body).clone();
            assign_variant_field!(body => Absyn::ClassDef::PARTS; classParts = createMetaClassesFromClassParts(var_field!((*body).classParts, Absyn::ClassDef::PARTS).clone())?);
            assign_field!(outClass.body = body.clone());
            ()
        },
        Deref @ Absyn::Class { body: __esc_body @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, .. } => {
            body = (*__esc_body).clone();
            assign_variant_field!(body => Absyn::ClassDef::CLASS_EXTENDS; parts = createMetaClassesFromClassParts(var_field!((*body).parts, Absyn::ClassDef::CLASS_EXTENDS).clone())?);
            assign_field!(outClass.body = body.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClass, outMetaClasses))
}

fn createMetaClassesFromClassParts(
    mut inClassParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut outClassParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    outClassParts = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
        for mut p in (inClassParts).into_iter().cloned() {
            let __x = (match &*p.clone() {
                Absyn::ClassPart::PUBLIC { contents: __p_contents } => {
                    assign_variant_field!(p => Absyn::ClassPart::PUBLIC; contents = createMetaClassesFromElementItems(__p_contents.clone())?);
                    p.clone()
                }
                Absyn::ClassPart::PROTECTED { contents: __p_contents } => {
                    assign_variant_field!(p => Absyn::ClassPart::PROTECTED; contents = createMetaClassesFromElementItems(__p_contents.clone())?);
                    p.clone()
                }
                _ => p.clone(),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outClassParts)
}

fn createMetaClassesFromElementItems(
    mut inElementItems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>> {
    let mut outElementItems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = metamodelica::nil();
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut meta_classes: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
    let mut els: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    for mut e in &*inElementItems.reverse() {
        let mut e = e.clone();
        e = (::match_deref::match_deref! { match &(e.clone()) {
            Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: __esc_cls, .. }, .. } } => {
                cls = (*__esc_cls).clone();
                (cls, meta_classes) = createMetaClasses(cls.clone())?;
                els = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = metamodelica::nil();
            for mut c in (meta_classes).into_iter().cloned() {
                let __x = setElementItemClass(e.clone(), c.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                outElementItems = listAppend(els, outElementItems);
                setElementItemClass(e, cls.clone())
            },
            _ => e,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        outElementItems = metamodelica::cons(e, outElementItems);
    }
    Ok(outElementItems)
}

fn setElementItemClass(
    mut inElementItem: metamodelica::Ref<Absyn::ElementItem>,
    mut inClass: metamodelica::Ref<Absyn::Class>,
) -> metamodelica::Ref<Absyn::ElementItem> {
    let mut outElementItem: metamodelica::Ref<Absyn::ElementItem> = inElementItem;
    outElementItem = (::match_deref::match_deref! { match &(outElementItem.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: e @ Deref @ Absyn::Element::ELEMENT { specification: es @ Deref @ Absyn::ElementSpec::CLASSDEF { .. }, .. } } => {
            let mut e = (*e).clone();
            let mut es = (*es).clone();
            assign_variant_field!(es => Absyn::ElementSpec::CLASSDEF; class_ = inClass);
            assign_variant_field!(e => Absyn::Element::ELEMENT; specification = es.clone());
            assign_variant_field!(outElementItem => Absyn::ElementItem::ELEMENTITEM; element = e.clone());
            outElementItem
        },
        _ => {
            outElementItem
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outElementItem
}

fn convertElementToClass(
    mut inElementItem: &metamodelica::Ref<Absyn::ElementItem>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    let __pa0 = ::match_deref::match_deref! { match &((*inElementItem)) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: __pa0, .. }, .. } } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outClass = metamodelica::Own::own(__pa0);
    Ok(outClass)
}

fn fixClassParts(
    mut inClassParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut inClassName: ArcStr,
    mut typeVars: metamodelica::List<ArcStr>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    metamodelica::List<metamodelica::Ref<Absyn::Class>>,
)> {
    let mut outClassParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    let mut outMetaClasses: metamodelica::List<metamodelica::Ref<Absyn::Class>> = metamodelica::nil();
    let mut meta_classes: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
    let mut els: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    outClassParts = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
        for mut p in (inClassParts).into_iter().cloned() {
            let __x = (match &*p.clone() {
                Absyn::ClassPart::PUBLIC { contents: __p_contents } => {
                    (els, meta_classes) = fixElementItems(__p_contents.clone(), inClassName.clone(), typeVars.clone())?;
                    assign_variant_field!(p => Absyn::ClassPart::PUBLIC; contents = els.clone());
                    outMetaClasses = listAppend(meta_classes.clone(), outMetaClasses.clone());
                    p.clone()
                }
                Absyn::ClassPart::PROTECTED { contents: __p_contents } => {
                    (els, meta_classes) = fixElementItems(__p_contents.clone(), inClassName.clone(), typeVars.clone())?;
                    assign_variant_field!(p => Absyn::ClassPart::PROTECTED; contents = els.clone());
                    outMetaClasses = listAppend(meta_classes.clone(), outMetaClasses.clone());
                    p.clone()
                }
                _ => p.clone(),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((outClassParts, outMetaClasses))
}

fn fixElementItems(
    mut inElementItems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut inName: ArcStr,
    mut typeVars: metamodelica::List<ArcStr>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    metamodelica::List<metamodelica::Ref<Absyn::Class>>,
)> {
    let mut outElementItems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut outMetaClasses: metamodelica::List<metamodelica::Ref<Absyn::Class>> = metamodelica::nil();
    let mut index: i32 = 0;
    let mut singleton: bool = ({
        let mut __acc: i32 = 0;
        for mut e in (inElementItems.clone()).into_iter().cloned() {
            let __x = if (AbsynUtil::isElementItem(&(e.clone()))) { 1 } else { 0 };
            __acc += __x;
        }
        __acc
    }) == 1;
    let mut c: metamodelica::Ref<Absyn::Class>;
    let mut r: Absyn::Restriction;
    outElementItems = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = metamodelica::nil();
        for mut e in (inElementItems).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(e.clone()) {
                Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: __esc_c @ Deref @ Absyn::Class { restriction: Absyn::Restriction::R_RECORD { .. }, .. }, .. }, .. } } => {
                    c = (*__esc_c).clone();
                    let mut body: metamodelica::Ref<Absyn::ClassDef>;
                    body = c.body.clone();
                    let () = (::match_deref::match_deref! { match &(&*body) {
                Deref @ Absyn::ClassDef::PARTS { typeVars: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. } => {
                    Error::addSourceMessage(&(Error::METARECORD_WITH_TYPEVARS.clone()), list![stringDelimitList(var_field!((*body).typeVars, Absyn::ClassDef::PARTS).clone(), literal!(","))], &c.info)?;
                    return Err("fail")
                },
                _ => (),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                    r = Absyn::Restriction::R_METARECORD { name: metamodelica::Ref::new(Absyn::Path::IDENT { name: inName.clone() }), index: index, singleton: singleton, moved: true, typeVars: typeVars.clone() };
                    assign_field!(c.restriction = r.clone());
                    outMetaClasses = metamodelica::cons(c.clone(), outMetaClasses.clone());
                    r = Absyn::Restriction::R_METARECORD { name: metamodelica::Ref::new(Absyn::Path::IDENT { name: inName.clone() }), index: index, singleton: singleton, moved: false, typeVars: typeVars.clone() };
                    assign_field!(c.restriction = r.clone());
                    index = index + 1;
                    setElementItemClass(e.clone(), c.clone())
                },
                _ => {
                    e.clone()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((outElementItems, outMetaClasses))
}

pub fn transformArrayNodesToListNodes(
    mut inList: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Exp>> {
    let mut outList: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    outList = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
        for mut e in (inList).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(e.clone()) {
                Deref @ Absyn::Exp::ARRAY { arrayExp: Deref @ metamodelica::ListNode::Nil } => metamodelica::Ref::new(Absyn::Exp::LIST { exps: metamodelica::nil() }),
                Deref @ Absyn::Exp::ARRAY { arrayExp: __e_arrayExp } => metamodelica::Ref::new(Absyn::Exp::LIST { exps: transformArrayNodesToListNodes(__e_arrayExp.clone()) }),
                _ => e.clone(),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outList
}
