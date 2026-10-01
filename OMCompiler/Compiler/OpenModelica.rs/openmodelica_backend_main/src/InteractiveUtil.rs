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

use crate::Interactive;
use crate::Interactive::Access;
use crate::NFApi;
use crate::StaticScript;
use openmodelica_ast::Absyn;
use openmodelica_backend::SymbolTable;
use openmodelica_error::ErrorExt;
use openmodelica_frontend::Ceval;
use openmodelica_frontend::ConnectionGraph;
use openmodelica_frontend::FBuiltin;
use openmodelica_frontend::FGraph;
use openmodelica_frontend::InnerOuter;
use openmodelica_frontend::Inst;
use openmodelica_frontend::Lookup;
use openmodelica_frontend::Mod;
use openmodelica_frontend::UnitAbsyn;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_inst::InstTypes;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_loader::Parser;
use openmodelica_program_util::ProgramUtil;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Print;
use openmodelica_util::Settings;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util::Vector;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

//public imports
// protected imports
pub type GraphicEnvCache = Interactive::GraphicEnvCache;

pub type AnnotationType = Interactive::AnnotationType;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum Visibility {
    PUBLIC = 1,
    PROTECTED = 2,
    ANY = 3,
}
impl PartialOrd for Visibility {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Visibility {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for Visibility {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub(crate) fn getExtendsElementspecInClass(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>> {
    let mut outAbsynElementSpecLst: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>>;
    outAbsynElementSpecLst = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut ext: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>>;
            ext = getExtendsElementspecInClassparts(metamodelica::AsArg::as_arg(&parts));
            ext
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut ext: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>>;
            ext = getExtendsElementspecInClassparts(metamodelica::AsArg::as_arg(&parts));
            ext
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: tp, arrayDim: _ }, arguments: eltArg, .. }, .. } => {
            list![metamodelica::Ref::new(Absyn::ElementSpec::EXTENDS { path: tp.clone(), elementArg: eltArg.clone(), annotationOpt: None })]
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAbsynElementSpecLst
}

fn getExtendsElementspecInClassparts<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elts }, tail: rest } => {
                let mut lst1: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>>;
                let mut lst2: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>>;
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>>;
                lst1 = getExtendsElementspecInClassparts(rest);
                lst2 = getExtendsElementspecInElementitems(metamodelica::AsArg::as_arg(&elts));
                return listAppend(lst1, lst2)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elts }, tail: rest } => {
                let mut lst1: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>>;
                let mut lst2: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>>;
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>>;
                lst1 = getExtendsElementspecInClassparts(rest);
                lst2 = getExtendsElementspecInElementitems(metamodelica::AsArg::as_arg(&elts));
                return listAppend(lst1, lst2)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                let mut res: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>>;
                { inAbsynClassPartLst = rest; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn getExtendsElementspecInElementitems(
    mut inAbsynElementItemLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>> {
    let mut outAbsynElementSpecLst: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>>;
    outAbsynElementSpecLst = 'mc: {
        let __mc_input = &**inAbsynElementItemLst;
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
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: el }, tail: rest } => {
                    let mut elt: metamodelica::Ref<Absyn::ElementSpec>;
                    let mut res: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>>;
                    elt = getExtendsElementspecInElement(metamodelica::AsArg::as_arg(&el))?;
                    res = getExtendsElementspecInElementitems(metamodelica::AsArg::as_arg(&rest));
                    Ok(metamodelica::cons(elt.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut res: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>>;
                    res = getExtendsElementspecInElementitems(metamodelica::AsArg::as_arg(&rest));
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outAbsynElementSpecLst
}

fn getExtendsElementspecInElement(
    mut inElement: &metamodelica::Ref<Absyn::Element>,
) -> Result<metamodelica::Ref<Absyn::ElementSpec>> {
    let mut outElementSpec: metamodelica::Ref<Absyn::ElementSpec>;
    outElementSpec = (::match_deref::match_deref! { match inElement {
        Deref @ Absyn::Element::ELEMENT { specification: ext @ Deref @ Absyn::ElementSpec::EXTENDS { .. }, .. } => {
            ext.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outElementSpec)
}

pub(crate) fn removeElementModifiers(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut inComponentName: &ArcStr,
    mut inProgram: Absyn::Program,
    mut keepRedeclares: bool,
) -> (Absyn::Program, bool) {
    let mut outProgram: Absyn::Program;
    let mut outResult: bool;
    let mut within_: Absyn::Within;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    match '__try0: {
        within_ = unwrap_break_err!(ProgramUtil::buildWithin(path.clone()), '__try0);
        cls = unwrap_break_err!(ProgramUtil::getPathedClassInProgram(path.clone(), &inProgram, false, false), '__try0);
        cls = unwrap_break_err!(clearComponentModifiersInClass(cls.clone(), inComponentName, keepRedeclares), '__try0);
        outProgram = unwrap_break_err!(ProgramUtil::updateProgram(Absyn::Program { classes: list![cls.clone()], within_: within_.clone() }, inProgram.clone(), false, false), '__try0);
        outResult = true;
        Ok::<_, &'static str>((outProgram.clone(), outResult.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outProgram = __try0_o0;
            outResult = __try0_o1;
        }
        Err(_) => {
            outProgram = inProgram.clone();
            outResult = false;
        }
    }
    (outProgram, outResult)
}

pub(crate) fn clearComponentModifiersInClass(
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut inComponentName: &ArcStr,
    mut keepRedeclares: bool,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class> = inClass.clone();
    let __pa0 = ::match_deref::match_deref! { match &(AbsynUtil::traverseClassComponents(inClass, (std::sync::Arc::new({ let __pe_b2 = inComponentName.clone(); let __pe_b3 = keepRedeclares; move |__pe_a0, __pe_a1| clearComponentModifiersInCompitems(__pe_a0, __pe_a1, &__pe_b2, __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, bool) -> Result<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, bool, bool)> + 'static>), false)?) {
        (__pa0, true) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outClass = metamodelica::Own::own(__pa0);
    Ok(outClass)
}

fn clearComponentModifiersInCompitems(
    mut inComponents: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
    mut inFound: bool,
    mut inComponentName: &ArcStr,
    mut keepRedeclares: bool,
) -> Result<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, bool, bool)> {
    let mut outComponents: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>> = metamodelica::nil();
    let mut outFound: bool;
    let mut outContinue: bool;
    let mut item: metamodelica::Ref<Absyn::ComponentItem>;
    let mut rest_items: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>> = inComponents.clone();
    let mut comp: Absyn::Component;
    while !((rest_items).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_items) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        item = metamodelica::Own::own(__pa0);
        rest_items = metamodelica::Own::own(__pa1);
        if metamodelica::stringEq(&(AbsynUtil::componentName(&item)?), &inComponentName) {
            let () = (match &*item {
                Absyn::ComponentItem {
                    component: __esc_comp @ Absyn::Component { .. },
                    ..
                } => {
                    comp = (*__esc_comp).clone();
                    comp.modification = if (!(keepRedeclares)) {
                        None
                    } else {
                        stripModifiersKeepRedeclares(comp.modification.clone())?
                    };
                    assign_field!(item.component = comp.clone());
                    ()
                }
            });
            outComponents = List::append_reverse(&outComponents, metamodelica::cons(item, rest_items));
            outFound = true;
            outContinue = false;
            return Ok((outComponents, outFound, outContinue));
        }
        outComponents = metamodelica::cons(item, outComponents);
    }
    outComponents = inComponents;
    outFound = false;
    outContinue = true;
    Ok((outComponents, outFound, outContinue))
}

fn stripModifiersKeepRedeclares(
    mut inMod: Option<metamodelica::Ref<Absyn::Modification>>,
) -> Result<Option<metamodelica::Ref<Absyn::Modification>>> {
    let mut outMod: Option<metamodelica::Ref<Absyn::Modification>>;
    outMod = (::match_deref::match_deref! { match &(inMod) {
        None => {
            None
        },
        Some(Deref @ Absyn::Modification { elementArgLst: ea, eqMod: _ }) => {
            let mut m: metamodelica::Ref<Absyn::Modification>;
            let mut ea = (*ea).clone();
            ea = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
        for mut e in (ea.clone()).into_iter().cloned() {
            if !((match &*e.clone() {
        Absyn::ElementArg::REDECLARATION { .. } => true,
        _ => false,
    })) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            m = metamodelica::Ref::new(Absyn::Modification { elementArgLst: ea.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() });
            Some(m)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMod)
}

pub(crate) fn setElementModifier(
    mut inClass: metamodelica::Ref<Absyn::Path>,
    mut inElementName: &metamodelica::Ref<Absyn::Path>,
    mut inMod: &metamodelica::Ref<Absyn::Modification>,
    mut program: Absyn::Program,
) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut outResult: bool;
    let mut within_: Absyn::Within;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    match '__try0: {
        cls = unwrap_break_err!(ProgramUtil::getPathedClassInProgram(inClass.clone(), &program, false, false), '__try0);
        (cls, outResult) = unwrap_break_err!(setElementSubmodifierInClass(cls.clone(), inElementName, inMod), '__try0);
        within_ = unwrap_break_err!(ProgramUtil::buildWithin(inClass.clone()), '__try0);
        program = unwrap_break_err!(ProgramUtil::updateProgram(Absyn::Program { classes: list![cls.clone()], within_: within_.clone() }, program.clone(), false, false), '__try0);
        Ok::<_, &'static str>((outResult.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outResult = __try0_o0;
        }
        Err(_) => {
            outResult = false;
        }
    }
    (program, outResult)
}

pub(crate) fn setExtendsModifier(
    mut className: metamodelica::Ref<Absyn::Path>,
    mut extendsName: &metamodelica::Ref<Absyn::Path>,
    mut elementName: &metamodelica::Ref<Absyn::Path>,
    mut r#mod: &metamodelica::Ref<Absyn::Modification>,
    mut program: Absyn::Program,
) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut result: bool;
    let mut within_: Absyn::Within;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut env: GraphicEnvCache;
    match '__try0: {
        cls =
            unwrap_break_err!(ProgramUtil::getPathedClassInProgram(className.clone(), &program, false, false), '__try0);
        env = unwrap_break_err!(Interactive::getClassEnv(program.clone(), className.clone()), '__try0);
        (cls, result) = unwrap_break_err!(setExtendsSubmodifierInClass(cls.clone(), extendsName, elementName, r#mod, &env), '__try0);
        within_ = unwrap_break_err!(ProgramUtil::buildWithin(className.clone()), '__try0);
        program = unwrap_break_err!(ProgramUtil::updateProgram(Absyn::Program { classes: list![cls.clone()], within_: within_.clone() }, program.clone(), false, false), '__try0);
        Ok::<_, &'static str>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = false;
        }
    }
    (program, result)
}

fn setElementSubmodifierInClass(
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut inElementName: &metamodelica::Ref<Absyn::Path>,
    mut inMod: &metamodelica::Ref<Absyn::Modification>,
) -> Result<(metamodelica::Ref<Absyn::Class>, bool)> {
    let mut outClass: metamodelica::Ref<Absyn::Class> = inClass.clone();
    let mut found: bool;
    (outClass, found) = AbsynUtil::traverseClassElements(
        inClass,
        (std::sync::Arc::new({
            let __pe_b2 = inElementName.clone();
            let __pe_b3 = inMod.clone();
            move |__pe_a0, __pe_a1| setSubmodifierInElement(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Element>,
                        bool,
                    ) -> Result<(metamodelica::Ref<Absyn::Element>, bool, bool)>
                    + 'static,
            >),
        false,
    )?;
    Ok((outClass, found))
}

fn setExtendsSubmodifierInClass(
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut extendsPath: &metamodelica::Ref<Absyn::Path>,
    mut elementName: &metamodelica::Ref<Absyn::Path>,
    mut r#mod: &metamodelica::Ref<Absyn::Modification>,
    mut env: &GraphicEnvCache,
) -> Result<(metamodelica::Ref<Absyn::Class>, bool)> {
    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    let mut found: bool;
    (cls, found) = AbsynUtil::traverseClassElements(
        cls,
        (std::sync::Arc::new({
            let __pe_b1 = extendsPath.clone();
            let __pe_b2 = elementName.clone();
            let __pe_b3 = r#mod.clone();
            let __pe_b4 = env.clone();
            move |__pe_a0, __pe_a5| {
                Ok(setExtendsSubmodifierInElement(
                    __pe_a0,
                    &__pe_b1,
                    __pe_b2.clone(),
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                    __pe_a5,
                ))
            }
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Element>,
                        bool,
                    ) -> Result<(metamodelica::Ref<Absyn::Element>, bool, bool)>
                    + 'static,
            >),
        false,
    )?;
    Ok((cls, found))
}

fn setSubmodifierInElement(
    mut element: metamodelica::Ref<Absyn::Element>,
    mut found: bool,
    mut elementName: metamodelica::Ref<Absyn::Path>,
    mut r#mod: metamodelica::Ref<Absyn::Modification>,
) -> Result<(metamodelica::Ref<Absyn::Element>, bool, bool)> {
    let mut element: metamodelica::Ref<Absyn::Element> = element;
    let mut found: bool = found;
    let mut outContinue: bool = true;
    if AbsynUtil::isElementNamed(&(AbsynUtil::pathFirstIdent(&elementName)), &element)? {
        if '__try0: {
            let () = (match &*element {
        Absyn::Element::ELEMENT { specification: __element_specification, .. } => {
            assign_variant_field!(element => Absyn::Element::ELEMENT; specification = unwrap_break_err!(setSubmodifierInElementSpec(elementName.clone(), r#mod.clone(), __element_specification.clone()), '__try0));
            ()
        },
        _ => break '__try0 Err::<_, _>("match: no arm matched"),
    });
            found = true;
            outContinue = false;
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    Ok((element, found, outContinue))
}

fn setExtendsSubmodifierInElement(
    mut element: metamodelica::Ref<Absyn::Element>,
    mut extendsPath: &metamodelica::Ref<Absyn::Path>,
    mut elementName: metamodelica::Ref<Absyn::Path>,
    mut r#mod: metamodelica::Ref<Absyn::Modification>,
    mut env: GraphicEnvCache,
    mut found: bool,
) -> (metamodelica::Ref<Absyn::Element>, bool, bool) {
    let mut element: metamodelica::Ref<Absyn::Element> = element;
    let mut found: bool = found;
    let mut outContinue: bool = true;
    let mut ext_spec: metamodelica::Ref<Absyn::ElementSpec>;
    let mut full_path: metamodelica::Ref<Absyn::Path> =
        <metamodelica::Ref<Absyn::Path> as ::std::default::Default>::default();
    let mut eargs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut opt_mod: Option<metamodelica::Ref<Absyn::Modification>> = None;
    found = 'mc: {
        let __mc_input = element.clone();
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ Absyn::Element::ELEMENT { specification: ext_spec @ Deref @ Absyn::ElementSpec::EXTENDS { elementArg: eargs, .. }, .. } => {
                            let mut ext_spec = (*ext_spec).clone();
                            let mut element: metamodelica::Ref<Absyn::Element> = element.clone();
                            let mut full_path: metamodelica::Ref<Absyn::Path> = full_path.clone();
                            let mut opt_mod: Option<metamodelica::Ref<Absyn::Modification>> = opt_mod.clone();
                            (_, full_path) = Interactive::mkFullyQual(env.clone(), var_field!((*ext_spec).path, Absyn::ElementSpec::EXTENDS).clone(), false)?;
                            let true = (AbsynUtil::pathEqual(extendsPath, &full_path)) else { return Err("pattern mismatch") };
                            if metamodelica::stringEq(&(AbsynUtil::pathFirstIdent(&elementName)), &(literal!("_"))) {
                                opt_mod = propagateMod(elementName.clone(), r#mod.clone(), Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: eargs.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })))?;
                            } else {
                                opt_mod = propagateMod(AbsynUtil::prefixPath(literal!("dummy"), elementName.clone()), r#mod.clone(), Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: eargs.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })))?;
                            }
                            assign_variant_field!(ext_spec => Absyn::ElementSpec::EXTENDS; elementArg = (::match_deref::match_deref! { match &(opt_mod.clone()) {
                Some(Deref @ Absyn::Modification { elementArgLst: eargs, .. }) => eargs.clone(),
                _ => metamodelica::nil(),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } }));
                            assign_variant_field!(element => Absyn::Element::ELEMENT; specification = ext_spec.clone());
                            Ok((true, element.clone(), full_path.clone(), opt_mod.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            element = __wb0;
            full_path = __wb1;
            opt_mod = __wb2;
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
    outContinue = !(found);
    (element, found, outContinue)
}

fn setSubmodifierInElementSpec(
    mut elementName: metamodelica::Ref<Absyn::Path>,
    mut r#mod: metamodelica::Ref<Absyn::Modification>,
    mut elSpec: metamodelica::Ref<Absyn::ElementSpec>,
) -> Result<metamodelica::Ref<Absyn::ElementSpec>> {
    let mut elSpec: metamodelica::Ref<Absyn::ElementSpec> = elSpec;
    let () = (match &*elSpec {
        Absyn::ElementSpec::CLASSDEF {
            class_: __elSpec_class_,
            ..
        } => {
            assign_variant_field!(elSpec => Absyn::ElementSpec::CLASSDEF; class_ = setSubmodifierInClass(elementName, __elSpec_class_.clone(), r#mod)?);
            ()
        }
        Absyn::ElementSpec::COMPONENTS {
            components: __elSpec_components,
            ..
        } => {
            assign_variant_field!(elSpec => Absyn::ElementSpec::COMPONENTS; components = setComponentSubmodifierInCompitems(__elSpec_components.clone(), false, elementName, r#mod)?.0);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(elSpec)
}

fn setSubmodifierInClass(
    mut inElementName: metamodelica::Ref<Absyn::Path>,
    mut inClass: metamodelica::Ref<Absyn::Class>,
    mut inMod: metamodelica::Ref<Absyn::Modification>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    let mut cls: metamodelica::Ref<Absyn::Class> = inClass;
    let mut r#mod: metamodelica::Ref<Absyn::Modification>;
    let mut body: metamodelica::Ref<Absyn::ClassDef>;
    outClass = (match &*cls {
        Absyn::Class { .. } => {
            body = cls.body.clone();
            body = (match &*body {
                Absyn::ClassDef::DERIVED {
                    arguments: __body_arguments,
                    ..
                } => {
                    let __pa0 = ::match_deref::match_deref! { match &(propagateMod(inElementName, inMod, Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: __body_arguments.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })))?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    r#mod = metamodelica::Own::own(__pa0);
                    assign_variant_field!(body => Absyn::ClassDef::DERIVED; arguments = (match &*r#mod {
                        Absyn::Modification { .. } => r#mod.elementArgLst.clone(),
                    }));
                    body
                }
                _ => return Err("match: no arm matched"),
            });
            assign_field!(cls.body = body);
            cls
        }
    });
    Ok(outClass)
}

pub(crate) fn setComponentSubmodifierInCompitems(
    mut inComponents: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
    mut inFound: bool,
    mut inComponentName: metamodelica::Ref<Absyn::Path>,
    mut inMod: metamodelica::Ref<Absyn::Modification>,
) -> Result<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, bool, bool)> {
    let mut outComponents: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>> = metamodelica::nil();
    let mut outFound: bool;
    let mut outContinue: bool;
    let mut item: metamodelica::Ref<Absyn::ComponentItem>;
    let mut rest_items: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>> = inComponents.clone();
    let mut comp: Absyn::Component;
    let mut comp_id: ArcStr;
    comp_id = AbsynUtil::pathFirstIdent(&inComponentName);
    while !((rest_items).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_items) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        item = metamodelica::Own::own(__pa0);
        rest_items = metamodelica::Own::own(__pa1);
        if metamodelica::stringEq(&(AbsynUtil::componentName(&item)?), &comp_id) {
            let () = (match &*item {
                Absyn::ComponentItem {
                    component: __esc_comp @ Absyn::Component { .. },
                    ..
                } => {
                    comp = (*__esc_comp).clone();
                    comp.modification = propagateMod(inComponentName, inMod, comp.modification.clone())?;
                    assign_field!(item.component = comp.clone());
                    ()
                }
            });
            outComponents = List::append_reverse(&outComponents, metamodelica::cons(item, rest_items));
            outFound = true;
            outContinue = false;
            return Ok((outComponents, outFound, outContinue));
        }
        outComponents = metamodelica::cons(item, outComponents);
    }
    outComponents = inComponents;
    outFound = false;
    outContinue = true;
    Ok((outComponents, outFound, outContinue))
}

pub(crate) fn propagateMod(
    mut inComponentName: metamodelica::Ref<Absyn::Path>,
    mut inNewMod: metamodelica::Ref<Absyn::Modification>,
    mut inOldMod: Option<metamodelica::Ref<Absyn::Modification>>,
) -> Result<Option<metamodelica::Ref<Absyn::Modification>>> {
    let mut outMod: Option<metamodelica::Ref<Absyn::Modification>>;
    let mut new_args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut old_args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut new_eqmod: metamodelica::Ref<Absyn::EqMod>;
    let mut old_eqmod: metamodelica::Ref<Absyn::EqMod>;
    let mut r#mod: metamodelica::Ref<Absyn::Modification>;
    if (inOldMod).is_some() {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inOldMod) {
            Some(Deref @ Absyn::Modification { elementArgLst: __pa0, eqMod: __pa1 }) => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        old_args = metamodelica::Own::own(__pa0);
        old_eqmod = metamodelica::Own::own(__pa1);
    } else {
        old_args = metamodelica::nil();
        old_eqmod = openmodelica_ast::Absyn::EqMod::interned_NOMOD();
    }
    if AbsynUtil::pathIsIdent(&inComponentName) {
        let __arc4 = inNewMod.clone();
        let Absyn::CLASSMOD {
            elementArgLst: __pa2,
            eqMod: __pa3,
        } = &*__arc4;
        new_args = metamodelica::Own::own(__pa2);
        new_eqmod = metamodelica::Own::own(__pa3);
        if new_eqmod.clone() == openmodelica_ast::Absyn::EqMod::interned_NOMOD() && !((new_args).is_empty()) {
            new_eqmod = old_eqmod;
        }
        if !(AbsynUtil::isEmptyMod(&inNewMod)) {
            new_args = mergeElementArgs(old_args, &new_args)?;
        }
        r#mod = metamodelica::Ref::new(Absyn::Modification {
            elementArgLst: new_args,
            eqMod: new_eqmod,
        });
    } else {
        new_args = propagateMod2(inComponentName, old_args, inNewMod)?;
        r#mod = metamodelica::Ref::new(Absyn::Modification {
            elementArgLst: new_args,
            eqMod: old_eqmod,
        });
    }
    outMod = if (AbsynUtil::isEmptyMod(&r#mod)) {
        None
    } else {
        Some(r#mod)
    };
    Ok(outMod)
}

fn mergeElementArgs(
    mut inOldArgs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inNewArgs: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>> {
    let mut outArgs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = inOldArgs.clone();
    let mut found: bool;
    if (inOldArgs).is_empty() {
        outArgs = removeEmptySubMods(inNewArgs);
    } else if (inNewArgs).is_empty() {
        outArgs = inOldArgs;
    } else {
        for mut narg in &**inNewArgs {
            (outArgs, found) = List::replaceOnTrue(
                narg.clone(),
                outArgs,
                &({
                    let __pe_b1 = narg.clone();
                    move |__pe_a0| AbsynUtil::elementArgEqualName(&__pe_a0, &__pe_b1)
                }),
            )?;
            if !(found) {
                outArgs = List::appendElt(narg.clone(), outArgs);
            }
        }
        outArgs = removeEmptySubMods(&outArgs);
    }
    Ok(outArgs)
}

fn removeEmptySubMods(
    mut subMods: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> {
    let mut outSubMods: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
    let mut r#mod: metamodelica::Ref<Absyn::Modification>;
    for mut m in &**subMods {
        let mut m = m.clone();
        let () = (::match_deref::match_deref! { match &(m.clone()) {
            Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(__esc_mod), .. } => {
                r#mod = (*__esc_mod).clone();
                assign_field!(r#mod.elementArgLst = removeEmptySubMods(&r#mod.elementArgLst));
                assign_variant_field!(m => Absyn::ElementArg::MODIFICATION; modification = if (AbsynUtil::isEmptyMod(metamodelica::AsArg::as_arg(&r#mod))) {None} else {Some(r#mod.clone())});
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        if !(AbsynUtil::isEmptySubMod(&m)) {
            outSubMods = metamodelica::cons(m, outSubMods);
        }
    }
    outSubMods = Dangerous::listReverseInPlace(outSubMods);
    outSubMods
}

fn propagateMod2(
    mut inComponentName: metamodelica::Ref<Absyn::Path>,
    mut inSubMods: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inNewMod: metamodelica::Ref<Absyn::Modification>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>> {
    let mut outSubMods: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
    let mut submod: metamodelica::Ref<Absyn::ElementArg>;
    let mut rest_submods: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = inSubMods.clone();
    let mut comp_name: metamodelica::Ref<Absyn::Path>;
    let mut comp_rest: metamodelica::Ref<Absyn::Path>;
    while !((rest_submods).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_submods) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        submod = metamodelica::Own::own(__pa0);
        rest_submods = metamodelica::Own::own(__pa1);
        comp_name = AbsynUtil::pathRest(inComponentName.clone())?;
        comp_rest = comp_name.clone();
        loop {
            if AbsynUtil::pathEqual(&comp_name, &(AbsynUtil::elementArgName(&submod)?)) {
                let () = (match &*submod {
                    Absyn::ElementArg::MODIFICATION {
                        modification: __submod_modification,
                        ..
                    } => {
                        if !(AbsynUtil::pathIsIdent(&comp_name)) {
                            comp_name = AbsynUtil::pathPrefix(&comp_name)?;
                            comp_rest = AbsynUtil::removePrefix(comp_name, comp_rest)?;
                        }
                        assign_variant_field!(submod => Absyn::ElementArg::MODIFICATION; modification = propagateMod(comp_rest, inNewMod, __submod_modification.clone())?);
                        if (var_field!((*submod).modification, Absyn::ElementArg::MODIFICATION)).is_some() {
                            rest_submods = metamodelica::cons(submod, rest_submods);
                        }
                        ()
                    }
                    Absyn::ElementArg::REDECLARATION {
                        elementSpec: __submod_elementSpec,
                        ..
                    } => {
                        assign_variant_field!(submod => Absyn::ElementArg::REDECLARATION; elementSpec = setSubmodifierInElementSpec(comp_rest, inNewMod, __submod_elementSpec.clone())?);
                        rest_submods = metamodelica::cons(submod, rest_submods);
                        ()
                    }
                    _ => (),
                });
                outSubMods = List::append_reverse(&outSubMods, rest_submods);
                return Ok(outSubMods);
            }
            if AbsynUtil::pathIsIdent(&comp_name) {
                break;
            } else {
                comp_name = AbsynUtil::pathPrefix(&comp_name)?;
            }
        }
        outSubMods = metamodelica::cons(submod, outSubMods);
    }
    if !(AbsynUtil::isEmptyMod(&inNewMod)) {
        submod = createNestedSubMod(AbsynUtil::pathRest(inComponentName)?, inNewMod)?;
        outSubMods = metamodelica::cons(submod, outSubMods).reverse();
    } else {
        outSubMods = inSubMods;
    }
    Ok(outSubMods)
}

fn createNestedSubMod(
    mut inComponentName: metamodelica::Ref<Absyn::Path>,
    mut inMod: metamodelica::Ref<Absyn::Modification>,
) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
    let mut outSubMod: metamodelica::Ref<Absyn::ElementArg>;
    if AbsynUtil::pathIsIdent(&inComponentName) {
        outSubMod = metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION {
            finalPrefix: false,
            eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH,
            path: inComponentName,
            modification: Some(inMod),
            comment: None,
            info: Absyn::dummyInfo.clone(),
        });
    } else {
        outSubMod = createNestedSubMod(AbsynUtil::pathRest(inComponentName.clone())?, inMod)?;
        outSubMod = metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION {
            finalPrefix: false,
            eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH,
            path: AbsynUtil::pathFirstPath(&inComponentName),
            modification: Some(metamodelica::Ref::new(Absyn::Modification {
                elementArgLst: list![outSubMod],
                eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD(),
            })),
            comment: None,
            info: Absyn::dummyInfo.clone(),
        });
    }
    Ok(outSubMod)
}

pub(crate) fn getElementModifierValue(
    mut classRef: &metamodelica::Ref<Absyn::ComponentRef>,
    mut varRef: &metamodelica::Ref<Absyn::ComponentRef>,
    mut subModRef: &metamodelica::Ref<Absyn::ComponentRef>,
    mut program: &Absyn::Program,
) -> ArcStr {
    let mut valueStr: ArcStr;
    let mut cls_path: metamodelica::Ref<Absyn::Path>;
    let mut name: ArcStr;
    let mut elName: ArcStr;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
    let mut elems: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
    let mut found: bool = false;
    let mut components: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
    let mut optMod: Option<metamodelica::Ref<Absyn::Modification>>;
    match '__try0: {
        cls_path = unwrap_break_err!(AbsynUtil::crefToPath(classRef), '__try0);
        elName = unwrap_break_err!(AbsynUtil::crefIdent(varRef), '__try0);
        cls = unwrap_break_err!(ProgramUtil::getPathedClassInProgram(cls_path.clone(), program, false, false), '__try0);
        elems = getElementsInClass(&cls);
        for mut e in &*elems {
            args = (::match_deref::match_deref! { match &(e.clone()) {
                Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { name, body: Deref @ Absyn::ClassDef::DERIVED { arguments: __esc_args, .. }, .. }, .. }, .. } if (stringEq(&name, &elName)) => {
                    args = (*__esc_args).clone();
                    found = true;
                    args.clone()
                },
                Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: __esc_components, .. }, .. } => {
                    components = (*__esc_components).clone();
                    for mut c in &*components.clone() {
                        let __arc2 = c.clone();
                        let Absyn::COMPONENTITEM { component: Absyn::COMPONENT { name: __pa0, modification: __pa1, .. }, .. } = &*__arc2;
                        name = metamodelica::Own::own(__pa0);
                        optMod = metamodelica::Own::own(__pa1);
                        if stringEq(&name, &elName) {
                            let __arc4 = optMod.clone().unwrap_or(metamodelica::Ref::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() }));
                            let Absyn::CLASSMOD { elementArgLst: __pa3, .. } = &*__arc4;
                            args = metamodelica::Own::own(__pa3);
                            found = true;
                        }
                    }
                    args.clone()
                },
                _ => metamodelica::nil(),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            if found {
                break;
            }
        }
        if found {
            valueStr = unwrap_break_err!(getModificationValueStr(args.clone(), &(unwrap_break_err!(AbsynUtil::crefToPath(subModRef), '__try0))), '__try0);
        } else {
            valueStr = literal!("");
        }
        Ok::<_, &'static str>((valueStr.clone(),))
    } {
        Ok((__try0_o0,)) => {
            valueStr = __try0_o0;
        }
        Err(_) => {
            valueStr = literal!("");
        }
    }
    valueStr
}

pub(crate) fn getModificationValueStr(
    mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut path: &metamodelica::Ref<Absyn::Path>,
) -> Result<ArcStr> {
    let mut value: ArcStr = literal!("");
    let mut name: ArcStr;
    let mut rest_args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = args;
    let mut arg: metamodelica::Ref<Absyn::ElementArg>;
    let mut found: bool = false;
    let mut elSpec: metamodelica::Ref<Absyn::ElementSpec>;
    let mut exp: metamodelica::Ref<Absyn::Exp>;
    while !(found) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_args) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        arg = metamodelica::Own::own(__pa0);
        rest_args = metamodelica::Own::own(__pa1);
        found = (::match_deref::match_deref! { match &(arg.clone()) {
            Deref @ Absyn::ElementArg::MODIFICATION { modification: __arg_modification, path: __arg_path, .. } if (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&__arg_path), path)) => {
                let __pa0 = ::match_deref::match_deref! { match &(__arg_modification.clone()) {
                    Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: __pa0, .. }, .. }) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                exp = metamodelica::Own::own(__pa0);
                value = Dump::printExpStr(exp)?;
                true
            },
            Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name }, modification: __arg_modification, .. } if (metamodelica::stringEq(&name, &(AbsynUtil::pathFirstIdent(path)))) => {
                let __pa0 = ::match_deref::match_deref! { match &(__arg_modification.clone()) {
                    Some(Deref @ Absyn::Modification { elementArgLst: __pa0, .. }) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                rest_args = metamodelica::Own::own(__pa0);
                value = getModificationValueStr(rest_args.clone(), &(AbsynUtil::pathRest(path.clone())?))?;
                true
            },
            Deref @ Absyn::ElementArg::REDECLARATION { elementSpec: elSpec, .. } if (metamodelica::stringEq(&(AbsynUtil::pathFirstIdent(path)), &(AbsynUtil::elementSpecName(metamodelica::AsArg::as_arg(&elSpec))?))) => {
                value = System::escapedString(Dump::unparseElementArgStr(arg)?, false);
                true
            },
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(value)
}

pub(crate) fn getElementModifierValues(
    mut inComponentRef1: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef2: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<Absyn::ComponentRef>,
    mut inProgram4: Absyn::Program,
) -> ArcStr {
    let mut outString: ArcStr;
    outString = 'mc: {
        let __mc_input = (inComponentRef1, inComponentRef2, inComponentRef3, inProgram4);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (class_, ident, subident, p) => {
                    let mut p_class: metamodelica::Ref<Absyn::Path>;
                    let mut name: ArcStr;
                    let mut res: ArcStr;
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut elems: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
                    let mut compelts: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>>;
                    let mut compelts_1: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
                    let mut r#mod: metamodelica::Ref<Absyn::Modification>;
                    let mut elementArgLst: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    p_class = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&class_))?;
                    let __pa0 = ::match_deref::match_deref! { match &(AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&ident))?) {
                        Deref @ Absyn::Path::IDENT { name: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    name = metamodelica::Own::own(__pa0);
                    cdef = ProgramUtil::getPathedClassInProgram(p_class.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    elems = getElementsInClass(&cdef);
                    compelts = List::map(elems.clone(), &move |__a0: metamodelica::Ref<Absyn::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(getComponentitemsInElement(&__a0)) })?;
                    compelts_1 = List::flatten(compelts.clone())?;
                    let __pa1 = ::match_deref::match_deref! { match &(List::select1(compelts_1.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::ComponentItem>, __a1: ArcStr| componentitemNamed(&__a0, __a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ComponentItem>, ArcStr) -> Result<bool> + 'static>), name.clone())?) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { modification: Some(Deref @ Absyn::Modification { elementArgLst: __pa1, .. }), .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil } => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    elementArgLst = metamodelica::Own::own(__pa1);
                    r#mod = getModificationValues(elementArgLst.clone(), AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&subident))?)?;
                    res = unparseMods(r#mod.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("Error"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outString
}

pub(crate) fn unparseMods(mut r#mod: metamodelica::Ref<Absyn::Modification>) -> Result<ArcStr> {
    let mut s: ArcStr;
    let mut arg: metamodelica::Ref<Absyn::ElementArg>;
    s = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ Absyn::Modification { elementArgLst: Deref @ metamodelica::ListNode::Cons { head: __esc_arg @ Deref @ Absyn::ElementArg::REDECLARATION { .. }, tail: _ }, .. } => {
            arg = (*__esc_arg).clone();
            System::escapedString(Dump::unparseElementArgStr(arg.clone())?, false)
        },
        _ => Dump::unparseModificationStr(r#mod)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(s)
}

fn getModificationValues(
    mut inAbsynElementArgLst: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<Absyn::Modification>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inAbsynElementArgLst, inPath.clone())) {
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: p1, modification: Some(r#mod), .. }, tail: _ }, p2) if (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&p1), metamodelica::AsArg::as_arg(&p2))) => {
                return Ok(r#mod.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: name1 }, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), .. }, tail: _ }, Deref @ Absyn::Path::QUALIFIED { name: name2, path: p2 }) if (stringEq(&name1, &name2)) => {
                let mut res: metamodelica::Ref<Absyn::Modification>;
                { (inAbsynElementArgLst, inPath) = (args.clone(), p2.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: elArg @ Deref @ Absyn::ElementArg::REDECLARATION { elementSpec: elSpec, .. }, tail: _ }, p1) if (metamodelica::stringEq(&(AbsynUtil::pathFirstIdent(metamodelica::AsArg::as_arg(&p1))), &(AbsynUtil::elementSpecName(metamodelica::AsArg::as_arg(&elSpec))?))) => {
                return Ok(metamodelica::Ref::new(Absyn::Modification { elementArgLst: list![elArg.clone()], eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() }))
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, _) => {
                let mut r#mod: metamodelica::Ref<Absyn::Modification>;
                { (inAbsynElementArgLst, inPath) = (rest.clone(), inPath); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn getElementModifierNames(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut inElementName: ArcStr,
    mut inProgram3: Absyn::Program,
) -> metamodelica::List<ArcStr> {
    let mut outList: metamodelica::List<ArcStr>;
    outList = ({
        let mut r#mod: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
        let mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
        let mut found: bool = false;
        'mc: {
            let __mc_input = inProgram3;
            if let Ok(__v) = (|| -> Result<_> {
                let mut p = __mc_input.clone() else {
                    return Err("nomatch");
                };
                let mut cdef: metamodelica::Ref<Absyn::Class>;
                let mut elems: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
                let mut res: metamodelica::List<ArcStr>;
                let mut name: ArcStr;
                let mut components: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
                let mut optMod: Option<metamodelica::Ref<Absyn::Modification>>;
                cdef = ProgramUtil::getPathedClassInProgram(path.clone(), &(p.clone()), false, false)?;
                elems = getElementsInClass(&cdef);
                for mut e in &*elems {
                    r#mod = (::match_deref::match_deref! { match &(e.clone()) {
                        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { name, body: Deref @ Absyn::ClassDef::DERIVED { arguments: __esc_args, .. }, .. }, .. }, .. } if (stringEq(&name, &inElementName)) => {
                            args = (*__esc_args).clone();
                            found = true;
                            args.clone()
                        },
                        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: __esc_components, .. }, .. } => {
                            components = (*__esc_components).clone();
                            for mut c in &*components.clone() {
                                let __arc2 = c.clone();
                                let Absyn::COMPONENTITEM { component: Absyn::COMPONENT { name: __pa0, modification: __pa1, .. }, .. } = &*__arc2;
                                name = metamodelica::Own::own(__pa0);
                                optMod = metamodelica::Own::own(__pa1);
                                if stringEq(&name, &inElementName) {
                                    let __arc4 = optMod.clone().unwrap_or(metamodelica::Ref::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() }));
                                    let Absyn::CLASSMOD { elementArgLst: __pa3, .. } = &*__arc4;
                                    r#mod = metamodelica::Own::own(__pa3);
                                    found = true;
                                }
                            }
                            r#mod.clone()
                        },
                        _ => metamodelica::nil(),
                        _ => unreachable!("match_deref! exhaustiveness placeholder"),
                    } });
                    if found {
                        break;
                    }
                }
                res = getModificationNames(&r#mod, true);
                Ok(res.clone())
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                let _ = __mc_input.clone() else { return Err("nomatch") };
                Ok(metamodelica::nil())
            })() {
                break 'mc __v;
            }
            panic!("matchcontinue: no arm matched")
        }
    });
    outList
}

pub(crate) fn getExtendsModifierNames(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut extendsPath: &metamodelica::Ref<Absyn::Path>,
    mut useQuotes: bool,
    mut program: Absyn::Program,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut extmod: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut res: metamodelica::List<ArcStr>;
    let mut silent: bool = !(Flags::isSet(Flags::NF_API_NOISE.clone())?);
    if silent {
        ErrorExt::setCheckpoint(literal!("InteractiveUtil.getExtendsModifierNames"));
    }
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(getPathedExtendsInProgram(classPath.clone(), extendsPath, program.clone())) {
            Some(Deref @ Absyn::ElementSpec::EXTENDS { elementArg: __pa1, .. }) => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        extmod = metamodelica::Own::own(__pa1);
        res = getModificationNames(&extmod, true);
        if useQuotes {
            res = Interactive::insertQuotesToList(&res);
        }
        result = ValuesMake::makeArray(
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
                for mut s in (res.clone()).into_iter().cloned() {
                    let __x = ValuesMake::makeCodeTypeName(AbsynUtil::makeIdentPathFromString(s.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        );
        Ok::<_, &'static str>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeString(literal!("Error"));
        }
    }
    if silent {
        ErrorExt::rollBack(literal!("InteractiveUtil.getExtendsModifierNames"));
    }
    Ok(result)
}

fn getModificationNames(
    mut inAbsynElementArgLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut includeRedeclares: bool,
) -> metamodelica::List<ArcStr> {
    let mut outStringLst: metamodelica::List<ArcStr>;
    outStringLst = 'mc: {
        let __mc_input = &**inAbsynElementArgLst;
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
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name }, modification: None, .. }, tail: rest } => {
                    let mut names: metamodelica::List<ArcStr>;
                    names = getModificationNames(metamodelica::AsArg::as_arg(&rest), includeRedeclares);
                    Ok(metamodelica::cons(name.clone(), names.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: p, modification: Some(Deref @ Absyn::Modification { elementArgLst: Deref @ metamodelica::ListNode::Nil, eqMod: _ }), .. }, tail: rest } => {
                    let mut names: metamodelica::List<ArcStr>;
                    let mut name: ArcStr;
                    name = AbsynUtil::pathString(p.clone(), literal!("."), true, false)?;
                    names = getModificationNames(metamodelica::AsArg::as_arg(&rest), includeRedeclares);
                    Ok(metamodelica::cons(name.clone(), names.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: p, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, eqMod: Deref @ Absyn::EqMod::EQMOD { .. } }), .. }, tail: rest } => {
                            let mut names: metamodelica::List<ArcStr>;
                            let mut names2: metamodelica::List<ArcStr>;
                            let mut res: metamodelica::List<ArcStr>;
                            let mut name: ArcStr;
                            name = AbsynUtil::pathString(p.clone(), literal!("."), true, false)?;
                            names2 = ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut n in (getModificationNames(metamodelica::AsArg::as_arg(&args), includeRedeclares)).into_iter().cloned() {
                            let __x = stringAppend(stringAppend(name.clone(), literal!(".")), n.clone());
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            names = getModificationNames(metamodelica::AsArg::as_arg(&rest), includeRedeclares);
                            res = listAppend(names2.clone(), names.clone());
                            Ok(metamodelica::cons(name.clone(), res.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { path: p, modification: Some(Deref @ Absyn::Modification { elementArgLst: args, eqMod: _ }), .. }, tail: rest } => {
                            let mut names: metamodelica::List<ArcStr>;
                            let mut names2: metamodelica::List<ArcStr>;
                            let mut res: metamodelica::List<ArcStr>;
                            let mut name: ArcStr;
                            name = AbsynUtil::pathString(p.clone(), literal!("."), true, false)?;
                            names2 = ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut n in (getModificationNames(metamodelica::AsArg::as_arg(&args), includeRedeclares)).into_iter().cloned() {
                            let __x = stringAppend(stringAppend(name.clone(), literal!(".")), n.clone());
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            names = getModificationNames(metamodelica::AsArg::as_arg(&rest), includeRedeclares);
                            res = listAppend(names2.clone(), names.clone());
                            Ok(res.clone())
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::REDECLARATION { elementSpec: elSpec, .. }, tail: rest } => {
                    if !((includeRedeclares)) { return Err("guard") }
                    let mut names: metamodelica::List<ArcStr>;
                    let mut name: ArcStr;
                    name = AbsynUtil::elementSpecName(metamodelica::AsArg::as_arg(&elSpec))?;
                    names = getModificationNames(metamodelica::AsArg::as_arg(&rest), includeRedeclares);
                    Ok(metamodelica::cons(name.clone(), names.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    let mut names: metamodelica::List<ArcStr>;
                    names = getModificationNames(metamodelica::AsArg::as_arg(&rest), includeRedeclares);
                    Ok(names.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outStringLst
}

pub(crate) fn getElementBinding(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut parameterName: &ArcStr,
    mut program: &Absyn::Program,
) -> ArcStr {
    let mut bindingStr: ArcStr;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut component: metamodelica::Ref<Absyn::ComponentItem>;
    match '__try0: {
        cls = unwrap_break_err!(ProgramUtil::getPathedClassInProgram(path.clone(), program, false, false), '__try0);
        component = unwrap_break_err!(getComponentInClass(&cls, parameterName), '__try0);
        bindingStr = unwrap_break_err!(Dump::printExpStr(unwrap_break_err!(getVariableBindingInComponentitem(&component), '__try0)), '__try0);
        Ok::<_, &'static str>((bindingStr.clone(),))
    } {
        Ok((__try0_o0,)) => {
            bindingStr = __try0_o0;
        }
        Err(_) => {
            bindingStr = literal!("");
        }
    }
    bindingStr
}

pub(crate) fn getComponentInClass(
    mut cls: &metamodelica::Ref<Absyn::Class>,
    mut componentName: &ArcStr,
) -> Result<metamodelica::Ref<Absyn::ComponentItem>> {
    let mut component: metamodelica::Ref<Absyn::ComponentItem>;
    let mut body: metamodelica::Ref<Absyn::ClassDef>;
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    let mut elements: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut components: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
    let __arc1 = &(*cls);
    let Absyn::CLASS { body: __pa0, .. } = &**__arc1;
    body = metamodelica::Own::own(__pa0);
    parts = (match &*body {
        Absyn::ClassDef::PARTS {
            classParts: __body_classParts,
            ..
        } => __body_classParts.clone(),
        Absyn::ClassDef::CLASS_EXTENDS {
            parts: __body_parts, ..
        } => __body_parts.clone(),
        _ => return Err("match: no arm matched"),
    });
    for mut part in &*parts {
        elements = (match &*part.clone() {
            Absyn::ClassPart::PUBLIC {
                contents: __part_contents,
            } => __part_contents.clone(),
            Absyn::ClassPart::PROTECTED {
                contents: __part_contents,
            } => __part_contents.clone(),
            _ => metamodelica::nil(),
        });
        for mut e in &*elements {
            components = (::match_deref::match_deref! { match &(e.clone()) {
                Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: __esc_components, .. }, .. } } => {
                    components = (*__esc_components).clone();
                    components.clone()
                },
                _ => metamodelica::nil(),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            for mut c in &*components {
                if metamodelica::stringEq(
                    &(AbsynUtil::componentName(metamodelica::AsArg::as_arg(&c))?),
                    &componentName,
                ) {
                    component = c.clone();
                    return Ok(component);
                }
            }
        }
    }
    return Err("fail");
    Ok(component)
}

pub(crate) fn getNthComponentInClass(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut nth: i32,
) -> Result<metamodelica::Ref<Absyn::Element>> {
    let mut outElement: metamodelica::Ref<Absyn::Element>;
    let mut r#pub: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
    let mut pro: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
    let mut n: i32;
    r#pub = getPublicComponentsInClass(inClass);
    n = ((r#pub).len() as i32);
    if nth <= n {
        outElement = (r#pub).get(nth)?;
    } else {
        pro = getProtectedComponentsInClass(inClass);
        outElement = (pro).get(nth - n)?;
    }
    Ok(outElement)
}

pub(crate) fn getComponentsInClass(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut visibility: Visibility,
) -> metamodelica::List<metamodelica::Ref<Absyn::Element>> {
    let mut outAbsynElementLst: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
    outAbsynElementLst = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: Deref @ metamodelica::ListNode::Nil, .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: lst, .. }, .. } => {
            let mut lst1: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
            let mut res: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
            res = metamodelica::nil();
            for mut elt in &*lst.clone() {
                res = (match &*elt.clone() {
        Absyn::ClassPart::PUBLIC { contents: __elt_contents } if (visibility != Visibility::PROTECTED.clone()) => {
            lst1 = getComponentsInElementitems(metamodelica::AsArg::as_arg(&__elt_contents));
            List::append_reverse(&lst1, res)
        },
        Absyn::ClassPart::PROTECTED { contents: __elt_contents } if (visibility != Visibility::PUBLIC.clone()) => {
            lst1 = getComponentsInElementitems(metamodelica::AsArg::as_arg(&__elt_contents));
            List::append_reverse(&lst1, res)
        },
        _ => res,
    });
            }
            Dangerous::listReverseInPlace(res)
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: Deref @ metamodelica::ListNode::Nil, .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: lst, .. }, .. } => {
            let mut lst1: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
            let mut res: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
            res = metamodelica::nil();
            for mut elt in &*lst.clone() {
                res = (match &*elt.clone() {
        Absyn::ClassPart::PUBLIC { contents: __elt_contents } if (visibility != Visibility::PROTECTED.clone()) => {
            lst1 = getComponentsInElementitems(metamodelica::AsArg::as_arg(&__elt_contents));
            List::append_reverse(&lst1, res)
        },
        Absyn::ClassPart::PROTECTED { contents: __elt_contents } if (visibility != Visibility::PUBLIC.clone()) => {
            lst1 = getComponentsInElementitems(metamodelica::AsArg::as_arg(&__elt_contents));
            List::append_reverse(&lst1, res)
        },
        _ => res,
    });
            }
            Dangerous::listReverseInPlace(res)
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAbsynElementLst
}

pub(crate) fn getPublicComponentsInClass(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Element>> {
    let mut components: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
    components = getComponentsInClass(inClass, Visibility::PUBLIC.clone());
    components
}

pub(crate) fn getProtectedComponentsInClass(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Element>> {
    let mut components: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
    components = getComponentsInClass(inClass, Visibility::PROTECTED.clone());
    components
}

pub(crate) fn getComponentsInElementitems(
    mut inAbsynElementItemLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Element>> {
    let mut outAbsynElementLst: metamodelica::List<metamodelica::Ref<Absyn::Element>> = metamodelica::nil();
    for mut el in &**inAbsynElementItemLst {
        let () = (::match_deref::match_deref! { match &(el.clone()) {
            Deref @ Absyn::ElementItem::ELEMENTITEM { element: elt @ Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { .. }, .. } } => {
                outAbsynElementLst = metamodelica::cons(elt.clone(), outAbsynElementLst);
                ()
            },
            _ => {
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    outAbsynElementLst = Dangerous::listReverseInPlace(outAbsynElementLst);
    outAbsynElementLst
}

pub(crate) fn getVariableBindingInComponentitem(
    mut inComponentItem: &metamodelica::Ref<Absyn::ComponentItem>,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    outExp = (::match_deref::match_deref! { match inComponentItem {
        Deref @ Absyn::ComponentItem { component: Absyn::Component { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: e, .. }, .. }), .. }, .. } => {
            e.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

pub(crate) fn componentitemNamed(
    mut inComponentItem: &metamodelica::Ref<Absyn::ComponentItem>,
    mut inIdent: ArcStr,
) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &((&**inComponentItem, inIdent)) {
        (Deref @ Absyn::ComponentItem { component: Absyn::Component { name: id1, .. }, .. }, id2) if (stringEq(&id1, &id2)) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

pub(crate) fn getComponentitemsInElement(
    mut inElement: &metamodelica::Ref<Absyn::Element>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>> {
    let mut outAbsynComponentItemLst: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
    outAbsynComponentItemLst = (::match_deref::match_deref! { match inElement {
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: l, .. }, .. } => {
            l.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAbsynComponentItemLst
}

pub(crate) fn createEnvironment(
    mut p: Absyn::Program,
    mut os: Option<metamodelica::List<metamodelica::Ref<SCode::Element>>>,
    mut modelPath: metamodelica::Ref<Absyn::Path>,
) -> Result<Interactive::GraphicEnvCache> {
    let mut genv: Interactive::GraphicEnvCache;
    let mut env: FCore::Graph;
    let mut env_1: FCore::Graph;
    let mut env2: FCore::Graph;
    let mut s: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut c: metamodelica::Ref<SCode::Element>;
    let mut id: ArcStr;
    let mut encflag: SCode::Encapsulated;
    let mut restr: SCode::Restriction;
    let mut ci_state: ClassInf::State;
    let mut cache: FCore::Cache;
    let mut permissive: bool;
    if Flags::isSet(Flags::NF_API.clone())? {
        genv = Interactive::GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE {
            program: p,
            modelPath: modelPath,
            cache: FCore::emptyCache(),
            env: FGraph::emptyGraph().clone(),
        };
    } else {
        s = os.unwrap_or(AbsynToSCode::translateAbsyn2SCode(p)?);
        (cache, env) = Inst::makeEnvFromProgram(&s)?;
        let (__pa0, __pa4, __pa1, __pa2, __pa3, __pa5) = ::match_deref::match_deref! { match &(Lookup::lookupClass(&cache, &env, &modelPath, None)?) {
            (__pa0, __pa4 @ Deref @ SCode::Element::CLASS { name: __pa1, encapsulatedPrefix: __pa2, restriction: __pa3, .. }, __pa5) => (__pa0.clone(), __pa4.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa5.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cache = metamodelica::Own::own(__pa0);
        id = metamodelica::Own::own(__pa1);
        encflag = metamodelica::Own::own(__pa2);
        restr = metamodelica::Own::own(__pa3);
        c = metamodelica::Own::own(__pa4);
        env_1 = metamodelica::Own::own(__pa5);
        env2 = FGraph::openScope(env_1, encflag, id, FGraph::restrictionToScopeType(&restr))?;
        ci_state = ClassInfUtil::start(&restr, FGraph::getGraphName(&env2)?)?;
        permissive = Flags::getConfigBool(Flags::PERMISSIVE.clone())?;
        FlagsUtil::setConfigBool(Flags::PERMISSIVE.clone(), true)?;
        match '__try7: {
            (_, env2, _, _, _) = unwrap_break_err!(Inst::partialInstClassIn(cache.clone(), env2.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state.clone(), c.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), 0), '__try7);
            unwrap_break_err!(FlagsUtil::setConfigBool(Flags::PERMISSIVE.clone(), permissive), '__try7);
            Ok::<_, &'static str>((env2.clone(),))
        } {
            Ok((__try7_o0,)) => {
                env2 = __try7_o0;
            }
            Err(__try7_err) => {
                FlagsUtil::setConfigBool(Flags::PERMISSIVE.clone(), permissive)?;
                return Err(__try7_err);
            }
        }
        genv = Interactive::GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE {
            program: SymbolTable::getAbsyn(),
            modelPath: modelPath,
            cache: cache,
            env: env2,
        };
    }
    Ok(genv)
}

pub(crate) fn getClassCommentInCommentOpt(mut inComment: Option<metamodelica::Ref<Absyn::Comment>>) -> ArcStr {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match &(inComment) {
        Some(Deref @ Absyn::Comment { comment: Some(__esc_outString), .. }) => {
            outString = (*__esc_outString).clone();
            outString.clone()
        },
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outString
}

pub(crate) fn getElementAnnotationsFromElts(
    mut els: metamodelica::List<metamodelica::Ref<Absyn::Element>>,
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut inFullProgram: Absyn::Program,
    mut inModelPath: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut graphicProgramSCode: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut env: FCore::Graph;
    let mut placementProgram: Absyn::Program;
    let mut cache: GraphicEnvCache;
    if !(Flags::isSet(Flags::NF_API.clone())?) {
        placementProgram = modelicaAnnotationProgram(Config::getAnnotationVersion()?)?;
        graphicProgramSCode = AbsynToSCode::translateAbsyn2SCode(placementProgram)?;
        (_, env) = Inst::makeEnvFromProgram(&graphicProgramSCode)?;
    } else {
        env = FGraph::emptyGraph().clone();
    }
    cache = Interactive::GraphicEnvCache::GRAPHIC_ENV_NO_CACHE {
        program: inFullProgram,
        modelPath: inModelPath,
    };
    result = getElementitemsAnnotations(els, env, inClass, cache)?;
    Ok(result)
}

fn getElementitemsAnnotations(
    mut inElements: metamodelica::List<metamodelica::Ref<Absyn::Element>>,
    mut inEnv: FCore::Graph,
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut inCache: GraphicEnvCache,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut result: metamodelica::Ref<Values::Value>;
    let mut res: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
    let mut accum: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
    let mut cache: GraphicEnvCache = inCache.clone();
    let mut items: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
    let mut cc: Option<metamodelica::Ref<Absyn::ConstrainClass>>;
    let mut annotations: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
    let mut cmt: Option<metamodelica::Ref<Absyn::Comment>>;
    let mut fullProgram: Absyn::Program;
    let mut modelPath: metamodelica::Ref<Absyn::Path>;
    if Flags::isSet(Flags::NF_API.clone())? {
        (fullProgram, modelPath) = Interactive::cacheProgramAndPath(&inCache);
        result = makeAnnotationArrayValue(NFApi::evaluateAnnotations(fullProgram, modelPath, &inElements)?);
        return Ok(result);
    }
    for mut e in &*inElements.reverse() {
        accum = 'mc: {
            let __mc_input = e.clone();
            if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: items, .. }, constrainClass: cc, .. } => {
                        let mut cache: Interactive::GraphicEnvCache = cache.clone();
                        let mut res: metamodelica::List<metamodelica::Ref<Values::Value>> = res.clone();
                        (res, cache) = getElementitemsAnnotationsFromItems(items.clone(), getAnnotationsFromConstraintClass(cc.clone()), inEnv.clone(), inClass, cache.clone())?;
                        Ok((listAppend(res.clone(), accum.clone()), cache.clone(), res.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                cache = __wb0;
                res = __wb1;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                            Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { comment: cmt, .. }, .. }, .. }, constrainClass: cc, .. } => {
                                let mut annotations: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = annotations.clone();
                                let mut cache: Interactive::GraphicEnvCache = cache.clone();
                                let mut res: metamodelica::List<metamodelica::Ref<Values::Value>> = res.clone();
                                annotations = (::match_deref::match_deref! { match &(cmt.clone()) {
                    Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs: __esc_annotations }), .. }) => {
                                annotations = (*__esc_annotations).clone();
                                annotations.clone()
                    },
                    _ => metamodelica::nil(),
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                                (res, cache) = getElementitemsAnnotationsFromElArgs(annotations.clone(), getAnnotationsFromConstraintClass(cc.clone()), inEnv.clone(), inClass, cache.clone())?;
                                Ok((metamodelica::cons(ValuesMake::makeArray(res.clone()), accum.clone()), annotations.clone(), cache.clone(), res.clone()))
                            }
                            _ => return Err("nomatch"),
                        }}
            })() {
                annotations = __wb0;
                cache = __wb1;
                res = __wb2;
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { .. }, .. } => {
                        Ok(metamodelica::cons(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: metamodelica::nil(), dimLst: list![0] }), accum.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. }, .. }, .. } => {
                        Ok(metamodelica::cons(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: metamodelica::nil(), dimLst: list![0] }), accum.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        Ok(accum.clone())
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            return Err("matchcontinue: no arm matched");
        };
    }
    result = ValuesMake::makeArray(accum);
    Ok(result)
}

fn getElementitemsAnnotationsFromElArgs(
    mut inAnnotations: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut ccAnnotations: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inEnv: FCore::Graph,
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut inCache: GraphicEnvCache,
) -> Result<(metamodelica::List<metamodelica::Ref<Values::Value>>, GraphicEnvCache)> {
    let mut result: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let mut outCache: GraphicEnvCache = inCache;
    let mut annotations: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut strl: metamodelica::List<ArcStr>;
    annotations = listAppend(inAnnotations, ccAnnotations);
    (strl, outCache) = getElementitemsAnnotationsElArgs(annotations, inEnv, inClass, outCache, true)?;
    result = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut s in (strl).into_iter().cloned() {
            let __x = ValuesMake::makeCodeTypeNameStr(s.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((result, outCache))
}

fn getAnnotationsFromConstraintClass(
    mut inCC: Option<metamodelica::Ref<Absyn::ConstrainClass>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> {
    let mut outElArgLst: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    outElArgLst = (::match_deref::match_deref! { match &(inCC) {
        Some(Deref @ Absyn::ConstrainClass { comment: Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs }), .. }), .. }) => {
            elementArgs.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outElArgLst
}

pub(crate) fn getElementitemsAnnotationsElArgs(
    mut inElementArgs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inEnv: FCore::Graph,
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut inCache: GraphicEnvCache,
    mut addAnnotationName: bool,
) -> Result<(metamodelica::List<ArcStr>, GraphicEnvCache)> {
    let mut outStringLst: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut outCache: GraphicEnvCache = inCache.clone();
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut ann_name: ArcStr;
    let mut eq_aexp: metamodelica::Ref<Absyn::Exp>;
    let mut graphic_exp: metamodelica::Ref<Absyn::Exp> = metamodelica::Ref::new(Absyn::Exp::BREAK);
    let mut eq_dexp: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut graphic_dexp: metamodelica::Ref<DAE::Exp> =
        <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut prop: DAE::Properties = <DAE::Properties as ::std::default::Default>::default();
    let mut info: SourceInfo;
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut env: FCore::Graph = <FCore::Graph as ::std::default::Default>::default();
    let mut env2: FCore::Graph = <FCore::Graph as ::std::default::Default>::default();
    let mut r#mod: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut stripped_mod: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
    let mut graphic_mod: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
    let mut c: metamodelica::Ref<SCode::Element> =
        <metamodelica::Ref<SCode::Element> as ::std::default::Default>::default();
    let mut smod: metamodelica::Ref<SCode::Mod> = metamodelica::Ref::new(SCode::Mod::NOMOD);
    let mut dmod: metamodelica::Ref<DAE::Mod> = metamodelica::Ref::new(DAE::Mod::NOMOD);
    let mut dae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    let mut is_icon: bool = false;
    let mut is_diagram: bool = false;
    let mut graphic_prog: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut placement_cls: metamodelica::Ref<SCode::Element> =
        <metamodelica::Ref<SCode::Element> as ::std::default::Default>::default();
    for mut e in &*inElementArgs.reverse() {
        let mut e = e.clone();
        e = AbsynUtil::createChoiceArray(e)?;
        r#str = 'mc: {
            let __mc_input = &*e;
            if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: ann_name }, modification: Some(Deref @ Absyn::Modification { elementArgLst: Deref @ metamodelica::ListNode::Nil, eqMod: Deref @ Absyn::EqMod::EQMOD { exp: eq_aexp, .. } }), info, .. } => {
                        let mut cache: FCore::Cache = cache.clone();
                        let mut env: FCore::Graph = env.clone();
                        let mut eq_dexp: metamodelica::Ref<DAE::Exp> = eq_dexp.clone();
                        let mut outCache: Interactive::GraphicEnvCache = outCache.clone();
                        let mut prop: DAE::Properties = prop.clone();
                        let mut r#str: ArcStr = r#str.clone();
                        (cache, env, _, outCache) = buildEnvForGraphicProgram(outCache.clone(), &(metamodelica::nil()))?;
                        (_, eq_dexp, prop) = StaticScript::elabGraphicsExp(cache.clone(), env.clone(), eq_aexp.clone(), false, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                        (cache, eq_dexp, prop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), eq_dexp.clone(), prop.clone(), false, info.clone())?;
                        (eq_dexp, _) = ExpressionSimplify::simplify1(eq_dexp.clone())?;
                        Print::clearErrorBuf();
                        r#str = ExpressionBasics::printExpStr(eq_dexp.clone())?;
                        Ok((stringAppendList(list![ann_name.clone(), literal!("="), r#str.clone()]), cache.clone(), env.clone(), eq_dexp.clone(), outCache.clone(), prop.clone(), r#str.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                cache = __wb0;
                env = __wb1;
                eq_dexp = __wb2;
                outCache = __wb3;
                prop = __wb4;
                r#str = __wb5;
                break 'mc __v;
            }
            if let Ok((
                __v,
                __wb0,
                __wb1,
                __wb2,
                __wb3,
                __wb4,
                __wb5,
                __wb6,
                __wb7,
                __wb8,
                __wb9,
                __wb10,
                __wb11,
                __wb12,
                __wb13,
                __wb14,
                __wb15,
                __wb16,
                __wb17,
            )) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: ann_name }, modification: Some(Deref @ Absyn::Modification { elementArgLst: r#mod, eqMod: Deref @ Absyn::EqMod::NOMOD { .. } }), info, .. } => {
                        let mut c: metamodelica::Ref<SCode::Element> = c.clone();
                        let mut cache: FCore::Cache = cache.clone();
                        let mut dae: DAE::DAElist = dae.clone();
                        let mut dmod: metamodelica::Ref<DAE::Mod> = dmod.clone();
                        let mut env: FCore::Graph = env.clone();
                        let mut env2: FCore::Graph = env2.clone();
                        let mut graphic_dexp: metamodelica::Ref<DAE::Exp> = graphic_dexp.clone();
                        let mut graphic_exp: metamodelica::Ref<Absyn::Exp> = graphic_exp.clone();
                        let mut graphic_mod: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = graphic_mod.clone();
                        let mut graphic_prog: Absyn::Program = graphic_prog.clone();
                        let mut is_diagram: bool = is_diagram.clone();
                        let mut is_icon: bool = is_icon.clone();
                        let mut outCache: Interactive::GraphicEnvCache = outCache.clone();
                        let mut placement_cls: metamodelica::Ref<SCode::Element> = placement_cls.clone();
                        let mut prop: DAE::Properties = prop.clone();
                        let mut smod: metamodelica::Ref<SCode::Mod> = smod.clone();
                        let mut r#str: ArcStr = r#str.clone();
                        let mut stripped_mod: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = stripped_mod.clone();
                        if !(listMember(ann_name.clone(), list![literal!("Icon"), literal!("Diagram"), literal!("choices")])) {
                            (cache, env, _, outCache) = buildEnvForGraphicProgram(outCache.clone(), metamodelica::AsArg::as_arg(&r#mod))?;
                            (cache, c, env2) = Lookup::lookupClassIdent(cache.clone(), inEnv.clone(), metamodelica::AsArg::as_arg(&ann_name), None)?;
                            smod = AbsynToSCode::translateMod(Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: r#mod.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, info.clone(), false)?;
                            (cache, dmod) = Mod::elabMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, smod.clone(), false, Mod::ModScope::COMPONENT { name: ann_name.clone() }, Absyn::dummyInfo.clone())?;
                            c = SCodeUtil::classSetPartial(c.clone(), openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL)?;
                            (_, _, _, _, dae, _, _, _, _, _) = Inst::instClass(cache.clone(), env2.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsyn::noStore().clone(), dmod.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, c.clone(), metamodelica::nil(), false, openmodelica_frontend_inst::InstTypes::CallingScope::TOP_CALL, ConnectionGraph::EMPTY().clone(), &(Connect::emptySet().clone()))?;
                            r#str = DAEUtil::getVariableBindingsStr(DAEUtil::daeElements(&dae))?;
                        } else {
                            is_icon = metamodelica::stringEq(&ann_name, &(literal!("Icon")));
                            is_diagram = metamodelica::stringEq(&ann_name, &(literal!("Diagram"))) || metamodelica::stringEq(&ann_name, &(literal!("choices")));
                            (stripped_mod, graphic_mod) = AbsynUtil::stripGraphicsAndInteractionModification(metamodelica::AsArg::as_arg(&r#mod))?;
                            ErrorExt::setCheckpoint(literal!("buildEnvForGraphicProgram"));
                            match '__try0: {
                                (cache, env, graphic_prog, _) = unwrap_break_err!(buildEnvForGraphicProgram(inCache.clone(), metamodelica::AsArg::as_arg(&r#mod)), '__try0);
                                ErrorExt::rollBack(literal!("buildEnvForGraphicProgram"));
                                Ok::<_, &'static str>((cache.clone(), env.clone(), graphic_prog.clone()))
                            } {
                                Ok((__try0_o0, __try0_o1, __try0_o2)) => {
                                            cache = __try0_o0;
                                            env = __try0_o1;
                                            graphic_prog = __try0_o2;
                                }
                                Err(_) => {
                                            ErrorExt::delCheckpoint(literal!("buildEnvForGraphicProgram"));
                                            (cache, env, graphic_prog, _) = buildEnvForGraphicProgram(inCache.clone(), &(metamodelica::nil()))?;
                                }
                            }
                            smod = AbsynToSCode::translateMod(Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: stripped_mod.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, info.clone(), false)?;
                            (cache, dmod) = Mod::elabMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, smod.clone(), false, Mod::ModScope::COMPONENT { name: ann_name.clone() }, info.clone())?;
                            placement_cls = AbsynToSCode::translateClass(ProgramUtil::getClassInProgram(metamodelica::AsArg::as_arg(&ann_name), &graphic_prog)?)?;
                            (cache, _, _, _, dae, _, _, _, _, _) = Inst::instClass(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsyn::noStore().clone(), dmod.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, placement_cls.clone(), metamodelica::nil(), false, openmodelica_frontend_inst::InstTypes::CallingScope::TOP_CALL, ConnectionGraph::EMPTY().clone(), &(Connect::emptySet().clone()))?;
                            r#str = DAEUtil::getVariableBindingsStr(DAEUtil::daeElements(&dae))?;
                            if is_icon || is_diagram {
                                if '__try1: {
                                            let __pa2 = ::match_deref::match_deref! { match &(graphic_mod.clone()) {
                                                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: __pa2, .. }, .. }), .. }, tail: Deref @ metamodelica::ListNode::Nil } => __pa2.clone(),
                                                _ => break '__try1 Err::<_, _>("pattern mismatch"),
                                            } };
                                            graphic_exp = metamodelica::Own::own(__pa2);
                                            (_, graphic_dexp, prop) = unwrap_break_err!(StaticScript::elabGraphicsExp(cache.clone(), env.clone(), graphic_exp.clone(), false, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone()), '__try1);
                                            if is_icon {
                                                ErrorExt::setCheckpoint(literal!("getAnnotationString: Icon"));
                                                (cache, graphic_dexp, _) = unwrap_break_err!(Ceval::cevalIfConstant(cache.clone(), env.clone(), graphic_dexp.clone(), prop.clone(), false, info.clone()), '__try1);
                                                (graphic_dexp, _) = unwrap_break_err!(ExpressionSimplify::simplify1(graphic_dexp.clone()), '__try1);
                                                ErrorExt::rollBack(literal!("getAnnotationString: Icon"));
                                            }
                                            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*unwrap_break_err!(ExpressionBasics::printExpStr(graphic_dexp.clone()), '__try1)); ArcStr::from(__mm_s) };
                                            Ok::<(), &'static str>(())
                                }.is_err() {
                                }
                            }
                            Print::clearErrorBuf();
                        }
                        Ok((if (addAnnotationName) {stringAppendList(list![ann_name.clone(), literal!("("), r#str.clone(), literal!(")")])} else {r#str.clone()}, c.clone(), cache.clone(), dae.clone(), dmod.clone(), env.clone(), env2.clone(), graphic_dexp.clone(), graphic_exp.clone(), graphic_mod.clone(), graphic_prog.clone(), is_diagram.clone(), is_icon.clone(), outCache.clone(), placement_cls.clone(), prop.clone(), smod.clone(), r#str.clone(), stripped_mod.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                c = __wb0;
                cache = __wb1;
                dae = __wb2;
                dmod = __wb3;
                env = __wb4;
                env2 = __wb5;
                graphic_dexp = __wb6;
                graphic_exp = __wb7;
                graphic_mod = __wb8;
                graphic_prog = __wb9;
                is_diagram = __wb10;
                is_icon = __wb11;
                outCache = __wb12;
                placement_cls = __wb13;
                prop = __wb14;
                smod = __wb15;
                r#str = __wb16;
                stripped_mod = __wb17;
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: ann_name }, modification: None, .. } => {
                        let mut c: metamodelica::Ref<SCode::Element> = c.clone();
                        let mut cache: FCore::Cache = cache.clone();
                        let mut dae: DAE::DAElist = dae.clone();
                        let mut env: FCore::Graph = env.clone();
                        let mut outCache: Interactive::GraphicEnvCache = outCache.clone();
                        let mut r#str: ArcStr = r#str.clone();
                        (cache, _, _, outCache) = buildEnvForGraphicProgram(outCache.clone(), &(metamodelica::nil()))?;
                        (cache, c, env) = Lookup::lookupClassIdent(cache.clone(), inEnv.clone(), metamodelica::AsArg::as_arg(&ann_name), None)?;
                        c = SCodeUtil::classSetPartial(c.clone(), openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL)?;
                        (_, _, _, _, dae, _, _, _, _, _) = Inst::instClass(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsyn::noStore().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, c.clone(), metamodelica::nil(), false, openmodelica_frontend_inst::InstTypes::CallingScope::TOP_CALL, ConnectionGraph::EMPTY().clone(), &(Connect::emptySet().clone()))?;
                        r#str = DAEUtil::getVariableBindingsStr(DAEUtil::daeElements(&dae))?;
                        Ok((if (addAnnotationName) {stringAppendList(list![ann_name.clone(), literal!("("), r#str.clone(), literal!(")")])} else {r#str.clone()}, c.clone(), cache.clone(), dae.clone(), env.clone(), outCache.clone(), r#str.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                c = __wb0;
                cache = __wb1;
                dae = __wb2;
                env = __wb3;
                outCache = __wb4;
                r#str = __wb5;
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: ann_name }, info, .. } => {
                        let mut r#str: ArcStr = r#str.clone();
                        r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("error evaluating: annotation(")); __mm_s.push_str(&*Dump::unparseElementArgStr(e.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                        r#str = Util::escapeQuotes(r#str.clone())?;
                        Ok((stringAppendList(list![ann_name.clone(), literal!("(\""), r#str.clone(), literal!("\")")]), r#str.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                r#str = __wb0;
                break 'mc __v;
            }
            return Err("matchcontinue: no arm matched");
        };
        outStringLst = metamodelica::cons(r#str.clone(), outStringLst);
    }
    Ok((outStringLst, outCache))
}

fn getElementitemsAnnotationsFromItems(
    mut inComponentItems: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
    mut ccAnnotations: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
    mut inEnv: FCore::Graph,
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut inCache: GraphicEnvCache,
) -> Result<(metamodelica::List<metamodelica::Ref<Values::Value>>, GraphicEnvCache)> {
    let mut result: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
    let mut outCache: GraphicEnvCache = inCache;
    let mut annotations: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut strl: metamodelica::List<ArcStr>;
    for mut comp in &*inComponentItems.reverse() {
        annotations = (::match_deref::match_deref! { match &(comp.clone()) {
            Deref @ Absyn::ComponentItem { comment: Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs: __esc_annotations }), .. }), .. } => {
                annotations = (*__esc_annotations).clone();
                listAppend(annotations.clone(), ccAnnotations.clone())
            },
            _ => ccAnnotations.clone(),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        (strl, outCache) =
            getElementitemsAnnotationsElArgs(annotations.clone(), inEnv.clone(), inClass, outCache, true)?;
        result = metamodelica::cons(makeAnnotationArrayValue(strl), result);
    }
    Ok((result, outCache))
}

pub(crate) fn modelicaAnnotationProgram(mut annotationVersion: ArcStr) -> Result<Absyn::Program> {
    let mut annotationProgram: Absyn::Program;
    let mut filename: ArcStr;
    filename = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?);
        __mm_s.push_str(&*literal!("/lib/omc/AnnotationsBuiltin_"));
        __mm_s.push_str(&*Util::stringReplaceChar(
            annotationVersion,
            literal!("."),
            literal!("_"),
        )?);
        __mm_s.push_str(&*literal!(".mo"));
        ArcStr::from(__mm_s)
    };
    annotationProgram = Parser::parse(
        filename,
        literal!("UTF-8"),
        literal!(""),
        None,
        Config::acceptedGrammar()?,
        Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?,
        Flags::getConfigBool(Flags::STRICT.clone())?,
    )?;
    Ok(annotationProgram)
}

pub(crate) fn buildEnvForGraphicProgram(
    mut inCache: GraphicEnvCache,
    mut inAnnotationMod: &metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>,
) -> Result<(FCore::Cache, FCore::Graph, Absyn::Program, GraphicEnvCache)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outGraphicProgram: Absyn::Program;
    let mut outGraphicEnvCache: GraphicEnvCache;
    (outCache, outEnv, outGraphicProgram, outGraphicEnvCache) = (match inCache.clone() {
        Interactive::GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE { .. } => (
            var_field!(inCache.cache, Interactive::GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE).clone(),
            var_field!(inCache.env, Interactive::GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE).clone(),
            Absyn::dummyProgram.clone(),
            inCache,
        ),
        Interactive::GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE { .. } => {
            if AbsynUtil::onlyLiteralsInAnnotationMod(inAnnotationMod) {
                outCache = var_field!(inCache.cache, Interactive::GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE).clone();
                outEnv = var_field!(inCache.env, Interactive::GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE).clone();
                outGraphicEnvCache = inCache.clone();
                outGraphicProgram =
                    var_field!(inCache.program, Interactive::GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE).clone();
            } else {
                (outCache, outEnv, outGraphicProgram) = buildEnvForGraphicProgramFull(
                    var_field!(inCache.program, Interactive::GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE).clone(),
                    var_field!(
                        inCache.modelPath,
                        Interactive::GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE
                    )
                    .clone(),
                )?;
                outGraphicEnvCache = Interactive::GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE {
                    program: var_field!(inCache.program, Interactive::GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE)
                        .clone(),
                    modelPath: var_field!(
                        inCache.modelPath,
                        Interactive::GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE
                    )
                    .clone(),
                    cache: outCache.clone(),
                    env: outEnv.clone(),
                };
            }
            (outCache, outEnv, outGraphicProgram, outGraphicEnvCache)
        }
        Interactive::GraphicEnvCache::GRAPHIC_ENV_NO_CACHE { .. } => {
            let mut scode_program: metamodelica::List<metamodelica::Ref<SCode::Element>>;
            if AbsynUtil::onlyLiteralsInAnnotationMod(inAnnotationMod) {
                outGraphicProgram = modelicaAnnotationProgram(Config::getAnnotationVersion()?)?;
                scode_program = AbsynToSCode::translateAbsyn2SCode(outGraphicProgram.clone())?;
                (outCache, outEnv) = Inst::makeEnvFromProgram(&scode_program)?;
                outGraphicEnvCache = Interactive::GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE {
                    program: var_field!(inCache.program, Interactive::GraphicEnvCache::GRAPHIC_ENV_NO_CACHE).clone(),
                    modelPath: var_field!(inCache.modelPath, Interactive::GraphicEnvCache::GRAPHIC_ENV_NO_CACHE)
                        .clone(),
                    cache: outCache.clone(),
                    env: outEnv.clone(),
                };
            } else {
                (outCache, outEnv, outGraphicProgram) = buildEnvForGraphicProgramFull(
                    var_field!(inCache.program, Interactive::GraphicEnvCache::GRAPHIC_ENV_NO_CACHE).clone(),
                    var_field!(inCache.modelPath, Interactive::GraphicEnvCache::GRAPHIC_ENV_NO_CACHE).clone(),
                )?;
                outGraphicEnvCache = Interactive::GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE {
                    program: var_field!(inCache.program, Interactive::GraphicEnvCache::GRAPHIC_ENV_NO_CACHE).clone(),
                    modelPath: var_field!(inCache.modelPath, Interactive::GraphicEnvCache::GRAPHIC_ENV_NO_CACHE)
                        .clone(),
                    cache: outCache.clone(),
                    env: outEnv.clone(),
                };
            }
            (outCache, outEnv, outGraphicProgram, outGraphicEnvCache)
        }
    });
    Ok((outCache, outEnv, outGraphicProgram, outGraphicEnvCache))
}

fn buildEnvForGraphicProgramFull(
    mut inProgram: Absyn::Program,
    mut inModelPath: metamodelica::Ref<Absyn::Path>,
) -> Result<(FCore::Cache, FCore::Graph, Absyn::Program)> {
    let mut outCache: FCore::Cache = FCore::emptyCache();
    let mut outEnv: FCore::Graph = FGraph::empty();
    let mut outProgram: Absyn::Program;
    let mut check_model: bool;
    let mut eval_param: bool;
    let mut failed: bool = false;
    let mut graphics_mode: bool;
    let mut graphic_program: Absyn::Program;
    let mut scode_program: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    graphic_program = modelicaAnnotationProgram(Config::getAnnotationVersion()?)?;
    outProgram = ProgramUtil::updateProgram(graphic_program, inProgram, false, false)?;
    scode_program = AbsynToSCode::translateAbsyn2SCode(outProgram.clone())?;
    check_model = Flags::getConfigBool(Flags::CHECK_MODEL.clone())?;
    eval_param = Config::getEvaluateParametersInAnnotations()?;
    graphics_mode = Config::getGraphicsExpMode()?;
    FlagsUtil::setConfigBool(Flags::CHECK_MODEL.clone(), true)?;
    Config::setEvaluateParametersInAnnotations(true)?;
    Config::setGraphicsExpMode(true)?;
    if '__try0: {
        (outCache, outEnv, _, _) = unwrap_break_err!(Inst::instantiateClass(FCore::emptyCache(), InnerOuter::emptyInstHierarchy().clone(), scode_program.clone(), inModelPath.clone(), true, true, true), '__try0);
        Ok::<(), &'static str>(())
    }.is_err() {
        failed = true;
    }
    Config::setEvaluateParametersInAnnotations(eval_param)?;
    FlagsUtil::setConfigBool(Flags::CHECK_MODEL.clone(), check_model)?;
    Config::setGraphicsExpMode(graphics_mode)?;
    if failed {
        return Err("fail");
    }
    Ok((outCache, outEnv, outProgram))
}

pub(crate) fn getElementsInClass(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Element>> {
    let mut outAbsynElementLst: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
    outAbsynElementLst = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: Deref @ metamodelica::ListNode::Nil, .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: lst, .. }, .. } => {
            let mut lst1: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
            let mut res: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
            let mut elts: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            res = metamodelica::nil();
            for mut elt in &*lst.clone() {
                let () = (match &*elt.clone() {
        Absyn::ClassPart::PUBLIC { contents: __esc_elts } => {
            elts = (*__esc_elts).clone();
            lst1 = getElementsInElementitems(metamodelica::AsArg::as_arg(&elts));
            res = List::append_reverse(&lst1, res);
            ()
        },
        Absyn::ClassPart::PROTECTED { contents: __esc_elts } => {
            elts = (*__esc_elts).clone();
            lst1 = getElementsInElementitems(metamodelica::AsArg::as_arg(&elts));
            res = List::append_reverse(&lst1, res);
            ()
        },
        _ => (),
    });
            }
            res = Dangerous::listReverseInPlace(res);
            res
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: Deref @ metamodelica::ListNode::Nil, .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: lst, .. }, .. } => {
            let mut lst1: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
            let mut res: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
            let mut elts: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            res = metamodelica::nil();
            for mut elt in &*lst.clone() {
                let () = (match &*elt.clone() {
        Absyn::ClassPart::PUBLIC { contents: __esc_elts } => {
            elts = (*__esc_elts).clone();
            lst1 = getElementsInElementitems(metamodelica::AsArg::as_arg(&elts));
            res = List::append_reverse(&lst1, res);
            ()
        },
        Absyn::ClassPart::PROTECTED { contents: __esc_elts } => {
            elts = (*__esc_elts).clone();
            lst1 = getElementsInElementitems(metamodelica::AsArg::as_arg(&elts));
            res = List::append_reverse(&lst1, res);
            ()
        },
        _ => (),
    });
            }
            res = Dangerous::listReverseInPlace(res);
            res
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAbsynElementLst
}

pub(crate) fn getPublicElementsInClass(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Element>> {
    let mut outAbsynElementLst: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
    outAbsynElementLst = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: Deref @ metamodelica::ListNode::Nil, .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: lst, .. }, .. } => {
            let mut lst1: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
            let mut res: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
            let mut elts: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            res = metamodelica::nil();
            for mut elt in &*lst.clone() {
                let () = (match &*elt.clone() {
        Absyn::ClassPart::PUBLIC { contents: __esc_elts } => {
            elts = (*__esc_elts).clone();
            lst1 = getElementsInElementitems(metamodelica::AsArg::as_arg(&elts));
            res = List::append_reverse(&lst1, res);
            ()
        },
        _ => (),
    });
            }
            res = Dangerous::listReverseInPlace(res);
            res
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: Deref @ metamodelica::ListNode::Nil, .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: lst, .. }, .. } => {
            let mut lst1: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
            let mut res: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
            let mut elts: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            res = metamodelica::nil();
            for mut elt in &*lst.clone() {
                let () = (match &*elt.clone() {
        Absyn::ClassPart::PUBLIC { contents: __esc_elts } => {
            elts = (*__esc_elts).clone();
            lst1 = getElementsInElementitems(metamodelica::AsArg::as_arg(&elts));
            res = List::append_reverse(&lst1, res);
            ()
        },
        _ => (),
    });
            }
            res = Dangerous::listReverseInPlace(res);
            res
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAbsynElementLst
}

pub(crate) fn getProtectedElementsInClass(
    mut inClass: &metamodelica::Ref<Absyn::Class>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Element>> {
    let mut outAbsynElementLst: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
    outAbsynElementLst = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: Deref @ metamodelica::ListNode::Nil, .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: lst, .. }, .. } => {
            let mut lst1: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
            let mut res: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
            let mut elts: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            res = metamodelica::nil();
            for mut elt in &*lst.clone() {
                let () = (match &*elt.clone() {
        Absyn::ClassPart::PROTECTED { contents: __esc_elts } => {
            elts = (*__esc_elts).clone();
            lst1 = getElementsInElementitems(metamodelica::AsArg::as_arg(&elts));
            res = List::append_reverse(&lst1, res);
            ()
        },
        _ => (),
    });
            }
            res = Dangerous::listReverseInPlace(res);
            res
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: Deref @ metamodelica::ListNode::Nil, .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: lst, .. }, .. } => {
            let mut lst1: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
            let mut res: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
            let mut elts: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            res = metamodelica::nil();
            for mut elt in &*lst.clone() {
                let () = (match &*elt.clone() {
        Absyn::ClassPart::PROTECTED { contents: __esc_elts } => {
            elts = (*__esc_elts).clone();
            lst1 = getElementsInElementitems(metamodelica::AsArg::as_arg(&elts));
            res = List::append_reverse(&lst1, res);
            ()
        },
        _ => (),
    });
            }
            res = Dangerous::listReverseInPlace(res);
            res
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAbsynElementLst
}

fn getElementsInElementitems(
    mut inAbsynElementItemLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Element>> {
    let mut outAbsynElementLst: metamodelica::List<metamodelica::Ref<Absyn::Element>> = metamodelica::nil();
    for mut el in &**inAbsynElementItemLst {
        let () = (match &*el.clone() {
            Absyn::ElementItem::ELEMENTITEM { element: elt } => {
                outAbsynElementLst = metamodelica::cons(elt.clone(), outAbsynElementLst);
                ()
            }
            _ => (),
        });
    }
    outAbsynElementLst = Dangerous::listReverseInPlace(outAbsynElementLst);
    outAbsynElementLst
}

pub(crate) fn dimensionListValues(
    mut dims: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
    vals = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut d in (dims).into_iter().cloned() {
            let __x = ValuesMake::makeCodeTypeNameStr(Dump::printSubscriptStr(&(d.clone()))?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(vals)
}

fn getElementInfo(
    mut element: &metamodelica::Ref<Absyn::Element>,
    mut isPublic: bool,
    mut quoteNames: bool,
    mut onlyComponents: bool,
    mut env: GraphicEnvCache,
    mut infos: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut infos: metamodelica::List<metamodelica::Ref<Values::Value>> = infos;
    let mut attr: Absyn::ElementAttributes;
    let mut ty: metamodelica::Ref<Absyn::Path>;
    let mut cc_path: metamodelica::Ref<Absyn::Path>;
    let mut comps: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
    let mut name: ArcStr;
    let mut cmt: ArcStr;
    let mut common_info: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let mut info: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut restriction: Absyn::Restriction;
    let mut opt_cmt: Option<metamodelica::Ref<Absyn::Comment>>;
    let mut opt_cc: Option<metamodelica::Ref<Absyn::ConstrainClass>>;
    let mut opt_adim: Option<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>>;
    let mut common_dims: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let mut dims: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let mut dims_val: metamodelica::Ref<Values::Value>;
    infos = (::match_deref::match_deref! { match element {
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: __esc_attr, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __esc_ty, arrayDim: __esc_opt_adim }, components: __esc_comps }, constrainClass: __esc_opt_cc, .. } => {
            attr = (*__esc_attr).clone();
            ty = (*__esc_ty).clone();
            opt_adim = (*__esc_opt_adim).clone();
            comps = (*__esc_comps).clone();
            opt_cc = (*__esc_opt_cc).clone();
            common_info = metamodelica::nil();
            ty = qualifyPath(env.clone(), ty.clone(), false)?;
            common_dims = dimensionListValues(attr.arrayDim.clone())?;
            if !(onlyComponents) {
                cc_path = getConstrainClassPath(env.clone(), opt_cc.clone());
                if quoteNames {
                    common_info = metamodelica::cons(ValuesMake::makeString(AbsynUtil::pathString(cc_path, literal!("."), true, false)?), common_info);
                } else {
                    common_info = metamodelica::cons(ValuesMake::makeCodeTypeName(getConstrainClassPath(env, opt_cc.clone())), common_info);
                }
            }
            common_info = getElementAttributeValues(element, isPublic, quoteNames, common_info)?;
            for mut comp in &*comps.clone().reverse() {
                name = comp.component.name.clone();
                cmt = getComponentComment(metamodelica::AsArg::as_arg(&comp), element);
                dims = dimensionListValues(comp.component.arrayDim.clone())?;
                dims_val = ValuesMake::makeArray(listAppend(dims, common_dims.clone()));
                if quoteNames {
                    dims_val = ValuesMake::makeString(ValuesDump::printValStr(&dims_val)?);
                }
                info = List::appendElt(dims_val, common_info.clone());
                info = metamodelica::cons(ValuesMake::makeString(cmt), info);
                if quoteNames {
                    info = metamodelica::cons(ValuesMake::makeString(name), info);
                    info = metamodelica::cons(ValuesMake::makeString(AbsynUtil::pathString(ty.clone(), literal!("."), true, false)?), info);
                } else {
                    info = metamodelica::cons(ValuesMake::makeCodeTypeNameStr(name), info);
                    info = metamodelica::cons(ValuesMake::makeCodeTypeName(ty.clone()), info);
                }
                if !(onlyComponents) {
                    info = metamodelica::cons(ValuesMake::makeString(literal!("co")), metamodelica::cons(ValuesMake::makeString(literal!("-")), info));
                }
                infos = metamodelica::cons(ValuesMake::makeArray(info), infos);
            }
            infos
        },
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: __esc_cls @ Deref @ Absyn::Class { name: __esc_name, restriction: __esc_restriction, body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: __esc_ty, arrayDim: __esc_opt_adim }, attributes: __esc_attr, comment: __esc_opt_cmt, .. }, .. }, .. }, constrainClass: __esc_opt_cc, .. } if (!(onlyComponents)) => {
            cls = (*__esc_cls).clone();
            name = (*__esc_name).clone();
            restriction = (*__esc_restriction).clone();
            ty = (*__esc_ty).clone();
            opt_adim = (*__esc_opt_adim).clone();
            attr = (*__esc_attr).clone();
            opt_cmt = (*__esc_opt_cmt).clone();
            opt_cc = (*__esc_opt_cc).clone();
            ty = qualifyPath(env.clone(), ty.clone(), false)?;
            cmt = getConstrainingClassComment(opt_cc.clone());
            if stringEmpty(&cmt) {
                cmt = getClassCommentInCommentOpt(opt_cmt.clone());
            }
            dims = if ((opt_adim).is_some()) {dimensionListValues(opt_adim.clone().ok_or("pattern mismatch")?)?} else {metamodelica::nil()};
            dims = listAppend(dimensionListValues(attr.arrayDim.clone())?, dims);
            dims_val = ValuesMake::makeArray(dims);
            if quoteNames {
                dims_val = ValuesMake::makeString(ValuesDump::printValStr(&dims_val)?);
            }
            info = list![dims_val];
            info = metamodelica::cons(ValuesMake::makeCodeTypeName(getConstrainClassPath(env, opt_cc.clone())), info);
            info = getElementAttributeValues(element, isPublic, quoteNames, info)?;
            info = metamodelica::cons(ValuesMake::makeString(cmt), info);
            info = metamodelica::cons(ValuesMake::makeCodeTypeNameStr(name.clone()), info);
            info = metamodelica::cons(ValuesMake::makeCodeTypeName(ty.clone()), info);
            info = metamodelica::cons(ValuesMake::makeString(Dump::unparseRestrictionStr(restriction.clone())?), info);
            info = metamodelica::cons(ValuesMake::makeString(literal!("cl")), info);
            infos = metamodelica::cons(ValuesMake::makeArray(info), infos);
            infos
        },
        _ => infos,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(infos)
}

pub(crate) fn getElementAttributeValues(
    mut element: &metamodelica::Ref<Absyn::Element>,
    mut isPublic: bool,
    mut quoteNames: bool,
    mut attrValues: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut attrValues: metamodelica::List<metamodelica::Ref<Values::Value>> = attrValues;
    let mut attr: Absyn::ElementAttributes;
    attrValues = (match &**element {
        Absyn::Element::ELEMENT {
            finalPrefix: __element_finalPrefix,
            innerOuter: __element_innerOuter,
            specification: __element_specification,
            ..
        } => {
            attr = (::match_deref::match_deref! { match &(__element_specification.clone()) {
                Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { attributes: __esc_attr, .. }, .. }, .. } => {
                    attr = (*__esc_attr).clone();
                    attr.clone()
                },
                Deref @ Absyn::ElementSpec::COMPONENTS { attributes: __esc_attr, .. } => {
                    attr = (*__esc_attr).clone();
                    attr.clone()
                },
                _ => return Err("match: no arm matched"),
            } });
            attrValues = metamodelica::cons(
                ValuesMake::makeString(attrVariabilityStr(&attr)?),
                metamodelica::cons(
                    ValuesMake::makeString(innerOuterStr(__element_innerOuter.clone())),
                    metamodelica::cons(ValuesMake::makeString(attrDirectionStr(&attr)?), attrValues),
                ),
            );
            if quoteNames {
                attrValues = metamodelica::cons(
                    ValuesMake::makeString(ArcStr::from(::std::format!("{}", __element_finalPrefix.clone()))),
                    metamodelica::cons(
                        ValuesMake::makeString(ArcStr::from(::std::format!("{}", attr.flowPrefix.clone()))),
                        metamodelica::cons(
                            ValuesMake::makeString(ArcStr::from(::std::format!("{}", attr.streamPrefix.clone()))),
                            metamodelica::cons(
                                ValuesMake::makeString(ArcStr::from(::std::format!(
                                    "{}",
                                    AbsynUtil::isElementReplaceable(element)
                                ))),
                                attrValues,
                            ),
                        ),
                    ),
                );
            } else {
                attrValues = metamodelica::cons(
                    ValuesMake::makeBoolean(__element_finalPrefix.clone()),
                    metamodelica::cons(
                        ValuesMake::makeBoolean(attr.flowPrefix.clone()),
                        metamodelica::cons(
                            ValuesMake::makeBoolean(attr.streamPrefix.clone()),
                            metamodelica::cons(
                                ValuesMake::makeBoolean(AbsynUtil::isElementReplaceable(element)),
                                attrValues,
                            ),
                        ),
                    ),
                );
            }
            attrValues = metamodelica::cons(
                ValuesMake::makeString(if (isPublic) {
                    literal!("public")
                } else {
                    literal!("protected")
                }),
                attrValues,
            );
            attrValues
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(attrValues)
}

pub(crate) fn qualifyPath(
    mut inEnv: GraphicEnvCache,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut failOnError: bool,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::Path::FULLYQUALIFIED { path: _ } => inPath,
        Deref @ Absyn::Path::IDENT { name: Deref @ "Real" } => inPath,
        Deref @ Absyn::Path::IDENT { name: Deref @ "Integer" } => inPath,
        Deref @ Absyn::Path::IDENT { name: Deref @ "Boolean" } => inPath,
        Deref @ Absyn::Path::IDENT { name: Deref @ "String" } => inPath,
        _ => {
            match '__try0: {
                if unwrap_break_err!(Flags::isSet(Flags::NF_API.clone()), '__try0) {
                    (_, outPath) = unwrap_break_err!(Interactive::mkFullyQual(inEnv.clone(), inPath.clone(), failOnError), '__try0);
                } else {
                    outPath = qualifyType(&(unwrap_break_err!(Interactive::envFromGraphicEnvCache(inEnv.clone()), '__try0)), inPath.clone());
                }
                Ok::<_, &'static str>((outPath.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    outPath = __try0_o0;
                }
                Err(_) => {
                    if failOnError {
                        return Err("fail");
                    } else {
                        outPath = inPath.clone();
                    }
                }
            }
            outPath
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPath)
}

pub(crate) fn getConstrainClassPath(
    mut inEnv: GraphicEnvCache,
    mut occ: Option<metamodelica::Ref<Absyn::ConstrainClass>>,
) -> metamodelica::Ref<Absyn::Path> {
    let mut path: metamodelica::Ref<Absyn::Path>;
    path = 'mc: {
        let __mc_input = occ;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ Absyn::ConstrainClass { elementSpec: Deref @ Absyn::ElementSpec::EXTENDS { path, .. }, .. }) => {
                    Ok(qualifyPath(inEnv.clone(), path.clone(), false)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("$Any") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    path
}

pub(crate) fn qualifyType(
    mut inEnv: &FCore::Graph,
    mut p: metamodelica::Ref<Absyn::Path>,
) -> metamodelica::Ref<Absyn::Path> {
    let mut fqp: metamodelica::Ref<Absyn::Path> = p.clone();
    let mut oenv_path: Option<metamodelica::Ref<Absyn::Path>> = None;
    let mut env_path: metamodelica::Ref<Absyn::Path> =
        <metamodelica::Ref<Absyn::Path> as ::std::default::Default>::default();
    let mut tp_path: metamodelica::Ref<Absyn::Path> =
        <metamodelica::Ref<Absyn::Path> as ::std::default::Default>::default();
    let mut pkg_path: metamodelica::Ref<Absyn::Path> =
        <metamodelica::Ref<Absyn::Path> as ::std::default::Default>::default();
    let mut tp_name: ArcStr = arcstr::literal!("");
    let mut env: FCore::Graph = <FCore::Graph as ::std::default::Default>::default();
    if AbsynUtil::pathIsFullyQualified(&p) {
        return fqp;
    }
    fqp = 'mc: {
        let __mc_input = ();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4)) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            let mut env: FCore::Graph = env.clone();
            let mut env_path: metamodelica::Ref<Absyn::Path> = env_path.clone();
            let mut oenv_path: Option<metamodelica::Ref<Absyn::Path>> = oenv_path.clone();
            let mut tp_name: ArcStr = tp_name.clone();
            let mut tp_path: metamodelica::Ref<Absyn::Path> = tp_path.clone();
            (_, _, env) = Lookup::lookupClass(&(FCore::emptyCache()), inEnv, &p, None)?;
            oenv_path = FGraph::getScopePath(&env)?;
            if (oenv_path).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(oenv_path.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                env_path = metamodelica::Own::own(__pa0);
                tp_name = AbsynUtil::pathLastIdent(&p);
                tp_path = AbsynUtil::suffixPath(&env_path, &tp_name);
            } else {
                tp_path = p.clone();
            }
            Ok((
                tp_path.clone(),
                env.clone(),
                env_path.clone(),
                oenv_path.clone(),
                tp_name.clone(),
                tp_path.clone(),
            ))
        })() {
            env = __wb0;
            env_path = __wb1;
            oenv_path = __wb2;
            tp_name = __wb3;
            tp_path = __wb4;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4)) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            let mut env: FCore::Graph = env.clone();
            let mut env_path: metamodelica::Ref<Absyn::Path> = env_path.clone();
            let mut oenv_path: Option<metamodelica::Ref<Absyn::Path>> = oenv_path.clone();
            let mut pkg_path: metamodelica::Ref<Absyn::Path> = pkg_path.clone();
            let mut tp_path: metamodelica::Ref<Absyn::Path> = tp_path.clone();
            pkg_path = AbsynUtil::pathFirstPath(&p);
            (_, _, env) = Lookup::lookupClass(&(FCore::emptyCache()), inEnv, &pkg_path, None)?;
            oenv_path = FGraph::getScopePath(&env)?;
            if (oenv_path).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(oenv_path.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                env_path = metamodelica::Own::own(__pa0);
                tp_path = AbsynUtil::joinPaths(env_path.clone(), p.clone())?;
            } else {
                tp_path = p.clone();
            }
            Ok((
                tp_path.clone(),
                env.clone(),
                env_path.clone(),
                oenv_path.clone(),
                pkg_path.clone(),
                tp_path.clone(),
            ))
        })() {
            env = __wb0;
            env_path = __wb1;
            oenv_path = __wb2;
            pkg_path = __wb3;
            tp_path = __wb4;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(p.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    fqp
}

pub(crate) fn getElementsInfo(
    mut elements: metamodelica::List<metamodelica::Ref<Absyn::Element>>,
    mut isPublic: bool,
    mut useQuotes: bool,
    mut onlyComponents: bool,
    mut env: Interactive::GraphicEnvCache,
    mut infos: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::List<metamodelica::Ref<Values::Value>>> {
    let mut infos: metamodelica::List<metamodelica::Ref<Values::Value>> = infos;
    for mut elem in &*elements.reverse() {
        infos = getElementInfo(
            metamodelica::AsArg::as_arg(&elem),
            isPublic,
            useQuotes,
            onlyComponents,
            env.clone(),
            infos,
        )?;
    }
    Ok(infos)
}

pub(crate) fn keywordReplaceable(mut inAbsynRedeclareKeywordsOption: Option<Absyn::RedeclareKeywords>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match inAbsynRedeclareKeywordsOption {
        Some(Absyn::RedeclareKeywords::REPLACEABLE { .. }) => true,
        Some(Absyn::RedeclareKeywords::REDECLARE_REPLACEABLE { .. }) => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn innerOuterStr(mut inInnerOuter: Absyn::InnerOuter) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inInnerOuter {
        Absyn::InnerOuter::INNER { .. } => literal!("inner"),
        Absyn::InnerOuter::OUTER { .. } => literal!("outer"),
        Absyn::InnerOuter::NOT_INNER_OUTER { .. } => literal!("none"),
        Absyn::InnerOuter::INNER_OUTER { .. } => literal!("innerouter"),
    });
    outString
}

pub(crate) fn attrFlowStr(mut inElementAttributes: &Absyn::ElementAttributes) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inElementAttributes.clone() {
        Absyn::ElementAttributes { flowPrefix: mut f, .. } => {
            let mut res: ArcStr;
            res = boolString(f.clone());
            res
        }
    });
    outString
}

pub(crate) fn attrStreamStr(mut inElementAttributes: &Absyn::ElementAttributes) -> ArcStr {
    let mut outString: ArcStr;
    outString = (match inElementAttributes.clone() {
        Absyn::ElementAttributes {
            streamPrefix: mut s, ..
        } => {
            let mut res: ArcStr;
            res = boolString(s.clone());
            res
        }
    });
    outString
}

pub(crate) fn attrParallelismStr(mut inElementAttributes: &Absyn::ElementAttributes) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inElementAttributes.clone() {
        Absyn::ElementAttributes {
            parallelism: Absyn::Parallelism::PARGLOBAL { .. },
            ..
        } => literal!("parglobal"),
        Absyn::ElementAttributes {
            parallelism: Absyn::Parallelism::PARLOCAL { .. },
            ..
        } => literal!("parlocal"),
        Absyn::ElementAttributes {
            parallelism: Absyn::Parallelism::NON_PARALLEL { .. },
            ..
        } => literal!(""),
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

pub(crate) fn attrVariabilityStr(mut inElementAttributes: &Absyn::ElementAttributes) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inElementAttributes.clone() {
        Absyn::ElementAttributes {
            variability: Absyn::Variability::VAR { .. },
            ..
        } => literal!("unspecified"),
        Absyn::ElementAttributes {
            variability: Absyn::Variability::DISCRETE { .. },
            ..
        } => literal!("discrete"),
        Absyn::ElementAttributes {
            variability: Absyn::Variability::PARAM { .. },
            ..
        } => literal!("parameter"),
        Absyn::ElementAttributes {
            variability: Absyn::Variability::CONST { .. },
            ..
        } => literal!("constant"),
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

pub(crate) fn attrDirectionStr(mut inElementAttributes: &Absyn::ElementAttributes) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (match inElementAttributes.clone() {
        Absyn::ElementAttributes {
            direction: Absyn::Direction::INPUT { .. },
            ..
        } => literal!("input"),
        Absyn::ElementAttributes {
            direction: Absyn::Direction::OUTPUT { .. },
            ..
        } => literal!("output"),
        Absyn::ElementAttributes {
            direction: Absyn::Direction::BIDIR { .. },
            ..
        } => literal!("unspecified"),
        _ => return Err("match: no arm matched"),
    });
    Ok(outString)
}

pub(crate) fn getConstrainingClassComment(
    mut constrainingClass: Option<metamodelica::Ref<Absyn::ConstrainClass>>,
) -> ArcStr {
    let mut comment: ArcStr;
    comment = (::match_deref::match_deref! { match &(constrainingClass) {
        Some(Deref @ Absyn::ConstrainClass { comment: Some(Deref @ Absyn::Comment { comment: Some(__esc_comment), .. }), .. }) => {
            comment = (*__esc_comment).clone();
            comment.clone()
        },
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    comment
}

pub(crate) fn getComponentComment(
    mut component: &metamodelica::Ref<Absyn::ComponentItem>,
    mut element: &metamodelica::Ref<Absyn::Element>,
) -> ArcStr {
    let mut comment: ArcStr;
    comment = getConstrainingClassComment(AbsynUtil::getElementConstrainingClass(element));
    if stringEmpty(&comment) {
        comment = getClassCommentInCommentOpt(component.comment.clone());
    }
    comment
}

pub(crate) fn getComponentItemsNameAndComment(
    mut inComponents: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
    mut inElement: &metamodelica::Ref<Absyn::Element>,
) -> metamodelica::List<metamodelica::List<ArcStr>> {
    let mut outStrings: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut name: ArcStr;
    let mut cmt_str: ArcStr;
    for mut comp in &*inComponents.reverse() {
        let () = (match &*comp.clone() {
            Absyn::ComponentItem {
                component: Absyn::Component { name: __esc_name, .. },
                ..
            } => {
                name = (*__esc_name).clone();
                cmt_str = getComponentComment(metamodelica::AsArg::as_arg(&comp), inElement);
                cmt_str = StringUtil::quote(cmt_str);
                outStrings = metamodelica::cons(list![name.clone(), cmt_str], outStrings);
                ()
            }
            _ => (),
        });
    }
    outStrings
}

pub(crate) fn replaceEquationList(
    mut inAbsynClassPartLst: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut inAbsynEquationItemLst: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut outAbsynClassPartLst: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    outAbsynClassPartLst = (::match_deref::match_deref! { match inAbsynClassPartLst {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::EQUATIONS { .. }, tail: rest } => {
            let mut newequationlst = inAbsynEquationItemLst;
            metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::EQUATIONS { contents: newequationlst }), rest.clone())
        },
        Deref @ metamodelica::ListNode::Cons { head: x, tail: xs } => {
            let mut new = inAbsynEquationItemLst;
            let mut ys: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            ys = replaceEquationList(xs, new)?;
            metamodelica::cons(x.clone(), ys)
        },
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAbsynClassPartLst)
}

pub(crate) fn getEquationList<'__b>(
    mut inAbsynClassPartLst: &'__b metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inAbsynClassPartLst {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::EQUATIONS { contents: lst }, tail: _ } => {
                return Ok(lst.clone())
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                let mut ys: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
                { inAbsynClassPartLst = xs; continue '__tco; }
            },
            _ => {
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn annotationListToAbsyn(
    mut inAbsynNamedArgLst: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
) -> Result<metamodelica::Ref<Absyn::Annotation>> {
    let mut outAnnotation: metamodelica::Ref<Absyn::Annotation>;
    let mut args: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
    for mut arg in &**inAbsynNamedArgLst {
        args = (::match_deref::match_deref! { match &(arg.clone()) {
            Deref @ Absyn::NamedArg { argName: Deref @ "annotate", argValue: e } => {
                let mut eltarg: metamodelica::Ref<Absyn::ElementArg>;
                eltarg = recordConstructorToModification(metamodelica::AsArg::as_arg(&e))?;
                metamodelica::cons(eltarg, args)
            },
            Deref @ Absyn::NamedArg { argName: Deref @ "comment", .. } => {
                args
            },
            _ => {
                args
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    outAnnotation = metamodelica::Ref::new(Absyn::Annotation {
        elementArgs: Dangerous::listReverseInPlace(args),
    });
    Ok(outAnnotation)
}

fn recordConstructorToModification(
    mut inExp: &metamodelica::Ref<Absyn::Exp>,
) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
    let mut outElementArg: metamodelica::Ref<Absyn::ElementArg>;
    outElementArg = 'mc: {
        let __mc_input = &**inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { function_: cr, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: e @ Deref @ Absyn::Exp::CALL { .. }, tail: Deref @ metamodelica::ListNode::Nil }, argNames: nargs }, .. } => {
                    let mut eltarglst: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    let mut res: metamodelica::Ref<Absyn::ElementArg>;
                    let mut emod: metamodelica::Ref<Absyn::ElementArg>;
                    let mut p: metamodelica::Ref<Absyn::Path>;
                    eltarglst = List::map(nargs.clone(), &move |__a0: metamodelica::Ref<Absyn::NamedArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(namedargToModification(&__a0)) })?;
                    emod = recordConstructorToModification(metamodelica::AsArg::as_arg(&e))?;
                    p = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&cr))?;
                    res = metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: p.clone(), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: metamodelica::cons(emod.clone(), eltarglst.clone()), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), comment: None, info: Absyn::dummyInfo.clone() });
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { function_: cr, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Nil, argNames: nargs }, .. } => {
                    let mut eltarglst: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    let mut res: metamodelica::Ref<Absyn::ElementArg>;
                    let mut p: metamodelica::Ref<Absyn::Path>;
                    eltarglst = List::map(nargs.clone(), &move |__a0: metamodelica::Ref<Absyn::NamedArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(namedargToModification(&__a0)) })?;
                    p = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&cr))?;
                    res = metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: p.clone(), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: eltarglst.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), comment: None, info: Absyn::dummyInfo.clone() });
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { function_: cr, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, argNames: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    let mut res: metamodelica::Ref<Absyn::ElementArg>;
                    let mut p: metamodelica::Ref<Absyn::Path>;
                    p = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&cr))?;
                    res = metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: p.clone(), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: e.clone(), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() });
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Print::printBuf(literal!("InteractiveUtil.recordConstructorToModification failed, exp="))?;
                    Print::printBuf(Dump::printExpStr(inExp.clone())?)?;
                    Print::printBuf(literal!("\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outElementArg)
}

fn namedargToModification(mut inNamedArg: &metamodelica::Ref<Absyn::NamedArg>) -> metamodelica::Ref<Absyn::ElementArg> {
    let mut outElementArg: metamodelica::Ref<Absyn::ElementArg>;
    outElementArg = 'mc: {
        let __mc_input = &**inNamedArg;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::NamedArg { argName: id, argValue: c @ Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Nil, .. }, .. } } => {
                    let mut elts: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
                    let mut res: metamodelica::Ref<Absyn::ElementArg>;
                    let __pa0 = ::match_deref::match_deref! { match &(recordConstructorToModification(metamodelica::AsArg::as_arg(&c))?) {
                        Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: __pa0, eqMod: _ }), comment: None, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    elts = metamodelica::Own::own(__pa0);
                    res = metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: metamodelica::Ref::new(Absyn::Path::IDENT { name: id.clone() }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: elts.clone(), eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })), comment: None, info: Absyn::dummyInfo.clone() });
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::NamedArg { argName: id, argValue: e } => {
                    let mut res: metamodelica::Ref<Absyn::ElementArg>;
                    res = metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: metamodelica::Ref::new(Absyn::Path::IDENT { name: id.clone() }), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: e.clone(), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() });
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Print::printBuf(literal!("- InteractiveUtil.namedargToModification failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outElementArg
}

pub(crate) fn getAllInheritedClasses(
    mut inClassName: metamodelica::Ref<Absyn::Path>,
    mut inProgram: Absyn::Program,
) -> metamodelica::List<metamodelica::Ref<Absyn::Path>> {
    let mut outBaseClassNames: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut genv: GraphicEnvCache = <Interactive::GraphicEnvCache as ::std::default::Default>::default();
    outBaseClassNames = ({
        let mut allPaths: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
        'mc: {
            let __mc_input = (inClassName, inProgram);
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (p_class, p) => {
                        let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                        let mut fqpaths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                        let mut cdef: metamodelica::Ref<Absyn::Class>;
                        let mut exts: metamodelica::List<metamodelica::Ref<Absyn::ElementSpec>>;
                        let mut genv: Interactive::GraphicEnvCache = genv.clone();
                        cdef = ProgramUtil::getPathedClassInProgram(p_class.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                        exts = getExtendsElementspecInClass(&cdef);
                        paths = List::map(exts.clone(), &move |__a0: metamodelica::Ref<Absyn::ElementSpec>| getBaseClassNameFromExtends(&__a0))?;
                        fqpaths = metamodelica::nil();
                        match '__try0: {
                            genv = unwrap_break_err!(createEnvironment(p.clone(), None, p_class.clone()), '__try0);
                            for mut pt in &*paths {
                                fqpaths = metamodelica::cons(unwrap_break_err!(qualifyPath(genv.clone(), pt.clone(), false), '__try0), fqpaths.clone());
                            }
                            fqpaths = fqpaths.clone().reverse();
                            Ok::<_, &'static str>((fqpaths.clone(),))
                        } {
                            Ok((__try0_o0,)) => {
                                fqpaths = __try0_o0;
                            }
                            Err(_) => {
                                fqpaths = paths.clone();
                            }
                        }
                        allPaths = metamodelica::nil();
                        for mut pt in &*fqpaths {
                            allPaths = List::append_reverse(&(getAllInheritedClasses(pt.clone(), p.clone())), allPaths.clone());
                        }
                        allPaths = Dangerous::listReverseInPlace(List::unique(&allPaths));
                        Ok((listAppend(fqpaths.clone(), allPaths.clone()), genv.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                genv = __wb0;
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        Ok(metamodelica::nil())
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            panic!("matchcontinue: no arm matched")
        }
    });
    outBaseClassNames
}

pub(crate) fn getBaseClassNameFromExtends(
    mut inElementSpec: &metamodelica::Ref<Absyn::ElementSpec>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut outBaseClassPath: metamodelica::Ref<Absyn::Path>;
    outBaseClassPath = (match &**inElementSpec {
        Absyn::ElementSpec::EXTENDS { path, .. } => path.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(outBaseClassPath)
}

pub mod ClassEntry {
    use super::*;
    #[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct ClassEntry {
        pub path: metamodelica::Ref<Absyn::Path>,
        pub cls: metamodelica::Ref<Absyn::Class>,
    }

    impl metamodelica::gc::MMTrace for ClassEntry {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.path, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.cls, __mmv)?;
            Ok(())
        }
    }
    impl Default for ClassEntry {
        fn default() -> Self {
            Self {
                path: Default::default(),
                cls: Default::default(),
            }
        }
    }

    pub type CLASS_ENTRY = ClassEntry;

    pub(crate) fn getPath(mut entry: &metamodelica::Ref<ClassEntry>) -> metamodelica::Ref<Absyn::Path> {
        let mut path: metamodelica::Ref<Absyn::Path> = entry.path.clone();
        path
    }

    pub(crate) fn greaterEq(
        mut entry1: &metamodelica::Ref<ClassEntry>,
        mut entry2: &metamodelica::Ref<ClassEntry>,
    ) -> Result<bool> {
        let mut res: bool = AbsynUtil::pathGe(entry1.path.clone(), entry2.path.clone())?;
        Ok(res)
    }

    pub(crate) fn equal(
        mut entry1: &metamodelica::Ref<ClassEntry>,
        mut entry2: &metamodelica::Ref<ClassEntry>,
    ) -> bool {
        let mut res: bool = referenceEq(&*(entry1.cls.clone()), &*(entry2.cls.clone()));
        res
    }
}

pub(crate) fn getAllSubtypeOf(
    mut baseClass: metamodelica::Ref<Absyn::Path>,
    mut parentClass: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
    mut includePartial: bool,
    mut sort: bool,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut classes: metamodelica::List<metamodelica::Ref<ClassEntry::ClassEntry>>;
    classes = getAllSubtypeOf2(baseClass, parentClass, program, includePartial, sort)?;
    paths = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
        for mut c in (classes).into_iter().cloned() {
            let __x = ClassEntry::getPath(&(c.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(paths)
}

pub(crate) fn getReplaceableChoices(
    mut baseClass: metamodelica::Ref<Absyn::Path>,
    mut parentClass: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
    mut includePartial: bool,
    mut sort: bool,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut res: metamodelica::Ref<Values::Value>;
    let mut classes: metamodelica::List<metamodelica::Ref<ClassEntry::ClassEntry>>;
    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
    let mut name_val: metamodelica::Ref<Values::Value>;
    let mut cmt_val: metamodelica::Ref<Values::Value>;
    classes = getAllSubtypeOf2(baseClass, parentClass, program, includePartial, sort)?;
    for mut entry in &*classes {
        name_val = ValuesMake::makeString(AbsynUtil::pathString(entry.path.clone(), literal!("."), true, false)?);
        cmt_val = ValuesMake::makeString(AbsynUtil::classDefStringComment(&entry.cls.body));
        vals = metamodelica::cons(ValuesMake::makeArray(list![name_val, cmt_val]), vals);
    }
    res = ValuesMake::makeArray(Dangerous::listReverseInPlace(vals));
    Ok(res)
}

fn getAllSubtypeOfCandidates(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut parentClass: &metamodelica::Ref<Absyn::Path>,
    mut program: &Absyn::Program,
    mut includePartial: bool,
    mut candidates: metamodelica::List<metamodelica::Ref<ClassEntry::ClassEntry>>,
) -> Result<metamodelica::List<metamodelica::Ref<ClassEntry::ClassEntry>>> {
    let mut candidates: metamodelica::List<metamodelica::Ref<ClassEntry::ClassEntry>> = candidates;
    let mut cdef: metamodelica::Ref<Absyn::Class>;
    let mut names: metamodelica::List<ArcStr>;
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut is_parent: bool;
    if let Ok(__iflet0) = ProgramUtil::getPathedClassInProgram(path.clone(), program, false, false) {
        cdef = __iflet0;
    } else {
        return Ok(candidates);
    }
    if includePartial || AbsynUtil::isNotPartial(&cdef) {
        candidates = metamodelica::cons(
            metamodelica::Ref::new(ClassEntry::ClassEntry {
                path: path.clone(),
                cls: cdef.clone(),
            }),
            candidates,
        );
        is_parent = AbsynUtil::pathEqual(&path, parentClass);
        if AbsynUtil::isPackageRestriction(&cdef.restriction) || is_parent {
            names = getClassnamesInClassListNoPartial(&path, program, &cdef, is_parent, false)?;
            paths = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
                for mut n in (names).into_iter().cloned() {
                    let __x = AbsynUtil::suffixPath(&path, &(n.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            candidates = List::fold(
                &paths,
                &({
                    let __pe_b1 = parentClass.clone();
                    let __pe_b2 = program.clone();
                    let __pe_b3 = includePartial;
                    move |__pe_a0, __pe_a4| {
                        getAllSubtypeOfCandidates(__pe_a0, &__pe_b1, &__pe_b2, __pe_b3.clone(), __pe_a4)
                    }
                }),
                candidates,
            )?;
        }
    }
    Ok(candidates)
}

pub(crate) fn getAllSubtypeOf2(
    mut baseClass: metamodelica::Ref<Absyn::Path>,
    mut parentClass: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
    mut includePartial: bool,
    mut sort: bool,
) -> Result<metamodelica::List<metamodelica::Ref<ClassEntry::ClassEntry>>> {
    let mut entries: metamodelica::List<metamodelica::Ref<ClassEntry::ClassEntry>>;
    let mut strlst: metamodelica::List<ArcStr>;
    let mut p: metamodelica::Ref<Absyn::Path> = <metamodelica::Ref<Absyn::Path> as ::std::default::Default>::default();
    let mut parent: metamodelica::Ref<Absyn::Path>;
    let mut base_class: metamodelica::Ref<Absyn::Path>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut base_entry: metamodelica::Ref<ClassEntry::ClassEntry>;
    let mut classes: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
    let mut result_path_lst: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut acc: metamodelica::List<metamodelica::Ref<ClassEntry::ClassEntry>>;
    let mut locals: metamodelica::List<metamodelica::Ref<ClassEntry::ClassEntry>>;
    let mut candidates: metamodelica::List<(
        metamodelica::Ref<Absyn::Path>,
        metamodelica::List<metamodelica::Ref<ClassEntry::ClassEntry>>,
    )> = metamodelica::nil();
    let mut genv: GraphicEnvCache;
    let mut opt_path: Option<metamodelica::Ref<Absyn::Path>>;
    for mut ext in &*getAllInheritedClasses(parentClass.clone(), program.clone()) {
        acc = getAllSubtypeOfCandidates(
            ext.clone(),
            metamodelica::AsArg::as_arg(&ext),
            &program,
            includePartial,
            metamodelica::nil(),
        )?;
        candidates = metamodelica::cons((ext.clone(), acc), candidates);
    }
    let Absyn::PROGRAM { classes: __pa0, .. } = &program;
    classes = metamodelica::Own::own(__pa0);
    if !(includePartial) {
        classes = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Class>> = metamodelica::nil();
            for mut c in (classes).into_iter().cloned() {
                if !(AbsynUtil::isNotPartial(&(c.clone()))) {
                    continue;
                }
                let __x = c.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
    }
    strlst = List::map(
        classes,
        &move |__a0: metamodelica::Ref<Absyn::Class>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(AbsynUtil::getClassName(&__a0))
        },
    )?;
    result_path_lst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Path>> = metamodelica::nil();
        for mut r#str in (strlst).into_iter().cloned() {
            let __x = AbsynUtil::makeIdentPathFromString(r#str.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    acc = metamodelica::nil();
    for mut p in &*result_path_lst {
        let mut p = p.clone();
        acc = getAllSubtypeOfCandidates(p, &parentClass, &program, includePartial, acc)?;
    }
    candidates = metamodelica::cons((parentClass.clone(), acc), candidates);
    match '__try1: {
        genv = unwrap_break_err!(createEnvironment(program.clone(), None, parentClass.clone()), '__try1);
        base_class = unwrap_break_err!(qualifyPath(genv.clone(), baseClass.clone(), true), '__try1);
        Ok::<_, &'static str>((base_class.clone(), genv.clone()))
    } {
        Ok((__try1_o0, __try1_o1)) => {
            base_class = __try1_o0;
            genv = __try1_o1;
        }
        Err(_) => {
            entries = metamodelica::nil();
            return Ok(entries);
        }
    }
    entries = metamodelica::nil();
    locals = metamodelica::nil();
    for mut tup in &*candidates {
        (parent, acc) = tup.clone();
        for mut entry in &*acc {
            let mut entry = entry.clone();
            if isSubtypeOf(entry.path.clone(), base_class.clone(), program.clone())? {
                opt_path = AbsynUtil::removePrefixOpt(&parent, &entry.path);
                if (opt_path).is_some() {
                    assign_field!(entry.path = opt_path.ok_or("pattern mismatch")?);
                    locals = metamodelica::cons(entry, locals);
                } else {
                    entries = metamodelica::cons(entry, entries);
                }
            }
        }
    }
    cls = ProgramUtil::getPathedClassInProgram(base_class.clone(), &program, false, false)?;
    base_entry = metamodelica::Ref::new(ClassEntry::ClassEntry {
        path: base_class,
        cls: cls,
    });
    for mut tup in &*candidates {
        (_, acc) = tup.clone();
        if List::contains(&acc, base_entry.clone(), &move |__a0: metamodelica::Ref<
            ClassEntry::ClassEntry,
        >,
                                                           __a1: metamodelica::Ref<
            ClassEntry::ClassEntry,
        >|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(ClassEntry::equal(&__a0, &__a1))
        })? {
            entries = metamodelica::cons(base_entry, entries);
            break;
        }
    }
    entries = listAppend(locals, entries);
    entries = List::uniqueOnTrue(&entries, &move |__a0: metamodelica::Ref<ClassEntry::ClassEntry>,
                                                  __a1: metamodelica::Ref<ClassEntry::ClassEntry>|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(ClassEntry::equal(&__a0, &__a1))
    })?;
    if sort {
        entries = List::sort(
            entries,
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<ClassEntry::ClassEntry>,
                      __a1: metamodelica::Ref<ClassEntry::ClassEntry>| {
                    ClassEntry::greaterEq(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<ClassEntry::ClassEntry>,
                            metamodelica::Ref<ClassEntry::ClassEntry>,
                        ) -> Result<bool>
                        + 'static,
                >),
        )?;
    }
    Ok(entries)
}

fn isSubtypeOf(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut baseClassPath: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
) -> Result<bool> {
    let mut res: bool;
    let mut base_classes: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    base_classes = getAllInheritedClasses(classPath, program);
    res = List::contains(&base_classes, baseClassPath, &move |__a0: metamodelica::Ref<
        Absyn::Path,
    >,
                                                              __a1: metamodelica::Ref<
        Absyn::Path,
    >|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(AbsynUtil::pathSuffixOfr(&__a0, &__a1))
    })?;
    Ok(res)
}

pub(crate) fn updateConnectionAnnotation(
    mut inClass: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inFrom: ArcStr,
    mut inTo: ArcStr,
    mut inAnnotation: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inProgram: Absyn::Program,
) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program;
    let mut class_path: metamodelica::Ref<Absyn::Path>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut class_within: Absyn::Within;
    class_path = AbsynUtil::crefToPath(inClass)?;
    cls = ProgramUtil::getPathedClassInProgram(class_path.clone(), &inProgram, false, false)?;
    cls = updateConnectionAnnotationInClass(cls, inFrom, inTo, annotationListToAbsyn(inAnnotation)?)?;
    class_within = if (AbsynUtil::pathIsIdent(&class_path)) {
        openmodelica_ast::Absyn::Within::TOP
    } else {
        Absyn::Within::WITHIN {
            path: AbsynUtil::stripLast(&class_path)?,
        }
    };
    outProgram = ProgramUtil::updateProgram(
        Absyn::Program {
            classes: list![cls],
            within_: class_within,
        },
        inProgram,
        false,
        false,
    )?;
    Ok(outProgram)
}

pub(crate) fn updateConnectionAnnotationInClass(
    mut inClass1: metamodelica::Ref<Absyn::Class>,
    mut inFrom: ArcStr,
    mut inTo: ArcStr,
    mut inAnnotation: metamodelica::Ref<Absyn::Annotation>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    outClass = (::match_deref::match_deref! { match &(inClass1) {
        __esc_outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts: parts, ann, comment: cmt }, .. } => {
            let mut from = inFrom;
            let mut to = inTo;
            let mut annotation_ = inAnnotation;
            outClass = (*__esc_outClass).clone();
            let mut eqlst: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqlst_1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            eqlst = getEquationList(metamodelica::AsArg::as_arg(&parts))?;
            eqlst_1 = updateConnectionAnnotationInEqList(&eqlst, &from, &to, annotation_)?;
            parts2 = replaceEquationList(metamodelica::AsArg::as_arg(&parts), eqlst_1)?;
            assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts2, ann: ann.clone(), comment: cmt.clone() }));
            outClass.clone()
        },
        __esc_outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcname, modifications: modif, parts, ann, comment: cmt }, .. } => {
            let mut from = inFrom;
            let mut to = inTo;
            let mut annotation_ = inAnnotation;
            outClass = (*__esc_outClass).clone();
            let mut eqlst: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqlst_1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            eqlst = getEquationList(metamodelica::AsArg::as_arg(&parts))?;
            eqlst_1 = updateConnectionAnnotationInEqList(&eqlst, &from, &to, annotation_)?;
            parts2 = replaceEquationList(metamodelica::AsArg::as_arg(&parts), eqlst_1)?;
            assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcname.clone(), modifications: modif.clone(), comment: cmt.clone(), parts: parts2, ann: ann.clone() }));
            outClass.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outClass)
}

fn updateConnectionAnnotationInEqList(
    mut equations: &metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut from: &ArcStr,
    mut to: &ArcStr,
    mut ann: metamodelica::Ref<Absyn::Annotation>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>> {
    let mut outEquations: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>> = metamodelica::nil();
    let mut c1: metamodelica::Ref<Absyn::ComponentRef>;
    let mut c2: metamodelica::Ref<Absyn::ComponentRef>;
    let mut c1_str: ArcStr;
    let mut c2_str: ArcStr;
    let mut found: bool = false;
    for mut eq in &**equations {
        let mut eq = eq.clone();
        if !(found) {
            eq = (::match_deref::match_deref! { match &(eq.clone()) {
                Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: Deref @ Absyn::Equation::EQ_CONNECT { connector1: __esc_c1, connector2: __esc_c2 }, .. } => {
                    c1 = (*__esc_c1).clone();
                    c2 = (*__esc_c2).clone();
                    c1_str = AbsynUtil::crefString(metamodelica::AsArg::as_arg(&c1))?;
                    c2_str = AbsynUtil::crefString(metamodelica::AsArg::as_arg(&c2))?;
                    if metamodelica::stringEq(&c1_str, &from) && metamodelica::stringEq(&c2_str, &to) {
                        found = true;
                    }
                    if !(found) {
                        found = metamodelica::stringEq(&c1_str, &to) && metamodelica::stringEq(&c2_str, &from);
                    }
                    if found {
                        assign_variant_field!(eq => Absyn::EquationItem::EQUATIONITEM; comment = Some(metamodelica::Ref::new(Absyn::Comment { annotation_: Some(ann.clone()), comment: None })));
                    }
                    eq
                },
                _ => eq,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
        }
        outEquations = metamodelica::cons(eq, outEquations);
    }
    outEquations = Dangerous::listReverseInPlace(outEquations);
    Ok(outEquations)
}

pub(crate) fn updateConnectionNames(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inFrom: ArcStr,
    mut inTo: ArcStr,
    mut inFromNew: ArcStr,
    mut inToNew: ArcStr,
    mut inProgram: Absyn::Program,
) -> Result<(bool, Absyn::Program)> {
    let mut outResult: bool;
    let mut outProgram: Absyn::Program;
    (outResult, outProgram) = 'mc: {
        let __mc_input = (inPath, inFrom, inTo, inFromNew, inToNew, inProgram);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, from, to, fromNew, toNew, p @ Absyn::Program { .. }) => {
                    let mut modelwithin: metamodelica::Ref<Absyn::Path>;
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut newcdef: metamodelica::Ref<Absyn::Class>;
                    let mut newp: Absyn::Program;
                    modelwithin = AbsynUtil::stripLast(metamodelica::AsArg::as_arg(&path))?;
                    cdef = ProgramUtil::getPathedClassInProgram(path.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    newcdef = updateConnectionNamesInClass(cdef.clone(), from.clone(), to.clone(), fromNew.clone(), toNew.clone())?;
                    newp = ProgramUtil::updateProgram(Absyn::Program { classes: list![newcdef.clone()], within_: Absyn::Within::WITHIN { path: modelwithin.clone() } }, p.clone(), false, false)?;
                    Ok((true, newp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, from, to, fromNew, toNew, p @ Absyn::Program { .. }) => {
                    let mut cdef: metamodelica::Ref<Absyn::Class>;
                    let mut newcdef: metamodelica::Ref<Absyn::Class>;
                    let mut newp: Absyn::Program;
                    cdef = ProgramUtil::getPathedClassInProgram(path.clone(), metamodelica::AsArg::as_arg(&p), false, false)?;
                    newcdef = updateConnectionNamesInClass(cdef.clone(), from.clone(), to.clone(), fromNew.clone(), toNew.clone())?;
                    newp = ProgramUtil::updateProgram(Absyn::Program { classes: list![newcdef.clone()], within_: openmodelica_ast::Absyn::Within::TOP }, p.clone(), false, false)?;
                    Ok((true, newp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, p @ Absyn::Program { .. }) => {
                    Ok((false, p.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outResult, outProgram))
}

fn updateConnectionNamesInClass(
    mut inClass1: metamodelica::Ref<Absyn::Class>,
    mut inFrom: ArcStr,
    mut inTo: ArcStr,
    mut inFromNew: ArcStr,
    mut inToNew: ArcStr,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    outClass = (::match_deref::match_deref! { match &(inClass1) {
        __esc_outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts: parts, ann, comment: cmt }, .. } => {
            let mut from = inFrom;
            let mut to = inTo;
            let mut fromNew = inFromNew;
            let mut toNew = inToNew;
            outClass = (*__esc_outClass).clone();
            let mut eqlst: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqlst_1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            eqlst = getEquationList(metamodelica::AsArg::as_arg(&parts))?;
            eqlst_1 = updateConnectionNamesInEqList(&eqlst, &from, &to, fromNew, toNew)?;
            parts2 = replaceEquationList(metamodelica::AsArg::as_arg(&parts), eqlst_1)?;
            assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts2, ann: ann.clone(), comment: cmt.clone() }));
            outClass.clone()
        },
        __esc_outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcname, modifications: modif, parts, ann, comment: cmt }, .. } => {
            let mut from = inFrom;
            let mut to = inTo;
            let mut fromNew = inFromNew;
            let mut toNew = inToNew;
            outClass = (*__esc_outClass).clone();
            let mut eqlst: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut eqlst_1: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
            let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            eqlst = getEquationList(metamodelica::AsArg::as_arg(&parts))?;
            eqlst_1 = updateConnectionNamesInEqList(&eqlst, &from, &to, fromNew, toNew)?;
            parts2 = replaceEquationList(metamodelica::AsArg::as_arg(&parts), eqlst_1)?;
            assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcname.clone(), modifications: modif.clone(), comment: cmt.clone(), parts: parts2, ann: ann.clone() }));
            outClass.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outClass)
}

fn updateConnectionNamesInEqList(
    mut equations: &metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut from: &ArcStr,
    mut to: &ArcStr,
    mut fromNew: ArcStr,
    mut toNew: ArcStr,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>> {
    let mut outEquations: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>> = metamodelica::nil();
    let mut c1: metamodelica::Ref<Absyn::ComponentRef>;
    let mut c2: metamodelica::Ref<Absyn::ComponentRef>;
    let mut c1_str: ArcStr;
    let mut c2_str: ArcStr;
    let mut found: bool = false;
    for mut eq in &**equations {
        let mut eq = eq.clone();
        if !(found) {
            eq = (::match_deref::match_deref! { match &(eq.clone()) {
                Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: Deref @ Absyn::Equation::EQ_CONNECT { connector1: __esc_c1, connector2: __esc_c2 }, .. } => {
                    c1 = (*__esc_c1).clone();
                    c2 = (*__esc_c2).clone();
                    c1_str = AbsynUtil::crefString(metamodelica::AsArg::as_arg(&c1))?;
                    c2_str = AbsynUtil::crefString(metamodelica::AsArg::as_arg(&c2))?;
                    found = if (metamodelica::stringEq(&c1_str, &from) && metamodelica::stringEq(&c2_str, &to)) {true} else {metamodelica::stringEq(&c1_str, &to) && metamodelica::stringEq(&c2_str, &from)};
                    if found {
                        assign_variant_field!(eq => Absyn::EquationItem::EQUATIONITEM; equation_ = metamodelica::Ref::new(Absyn::Equation::EQ_CONNECT { connector1: Parser::stringCref(fromNew.clone())?, connector2: Parser::stringCref(toNew.clone())? }));
                    }
                    eq
                },
                _ => eq,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
        }
        outEquations = metamodelica::cons(eq, outEquations);
    }
    outEquations = Dangerous::listReverseInPlace(outEquations);
    Ok(outEquations)
}

fn getClassnamesInClassListNoPartial(
    mut inPath: &metamodelica::Ref<Absyn::Path>,
    mut inProgram: &Absyn::Program,
    mut inClass: &metamodelica::Ref<Absyn::Class>,
    mut inShowProtected: bool,
    mut includeConstants: bool,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outString: metamodelica::List<ArcStr>;
    if AbsynUtil::isPartial(inClass) {
        outString = metamodelica::nil();
        return Ok(outString);
    }
    outString = (::match_deref::match_deref! { match inClass {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut b = inShowProtected;
            let mut c = includeConstants;
            let mut strlist: metamodelica::List<ArcStr>;
            strlist = getClassnamesInPartsNoPartial(metamodelica::AsArg::as_arg(&parts), b, c)?;
            strlist
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut b = inShowProtected;
            let mut c = includeConstants;
            let mut strlist: metamodelica::List<ArcStr>;
            strlist = getClassnamesInPartsNoPartial(metamodelica::AsArg::as_arg(&parts), b, c)?;
            strlist
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { .. }, .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::OVERLOAD { .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::ENUMERATION { .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PDER { .. }, .. } => {
            metamodelica::nil()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outString)
}

pub(crate) fn getClassnamesInPartsNoPartial(
    mut inAbsynClassPartLst: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut inShowProtected: bool,
    mut includeConstants: bool,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outStringLst: metamodelica::List<ArcStr>;
    outStringLst = 'mc: {
        let __mc_input = (&**inAbsynClassPartLst, inShowProtected, includeConstants);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elts }, tail: rest }, b, c) => {
                    let mut l1: metamodelica::List<ArcStr>;
                    let mut l2: metamodelica::List<ArcStr>;
                    let mut res: metamodelica::List<ArcStr>;
                    l1 = getClassnamesInEltsNoPartial(metamodelica::AsArg::as_arg(&elts), c.clone())?;
                    l2 = getClassnamesInPartsNoPartial(metamodelica::AsArg::as_arg(&rest), b.clone(), c.clone())?;
                    res = listAppend(l1.clone(), l2.clone());
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elts }, tail: rest }, true, c) => {
                    let mut l1: metamodelica::List<ArcStr>;
                    let mut l2: metamodelica::List<ArcStr>;
                    let mut res: metamodelica::List<ArcStr>;
                    l1 = getClassnamesInEltsNoPartial(metamodelica::AsArg::as_arg(&elts), c.clone())?;
                    l2 = getClassnamesInPartsNoPartial(metamodelica::AsArg::as_arg(&rest), true, c.clone())?;
                    res = listAppend(l1.clone(), l2.clone());
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, b, c) => {
                    let mut res: metamodelica::List<ArcStr>;
                    res = getClassnamesInPartsNoPartial(metamodelica::AsArg::as_arg(&rest), b.clone(), c.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStringLst)
}

pub(crate) fn getClassnamesInEltsNoPartial(
    mut inAbsynElementItemLst: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut includeConstants: bool,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outStringLst: metamodelica::List<ArcStr>;
    let mut delst: DoubleEnded::MutableList<ArcStr>;
    delst = DoubleEnded::fromList(&(metamodelica::nil()))?;
    for mut elt in &**inAbsynElementItemLst {
        let () = (::match_deref::match_deref! { match &(elt.clone()) {
            Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { partialPrefix: false, body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName: id, .. }, .. }, .. }, .. } } => {
                DoubleEnded::push_back(delst.clone(), id.clone())?;
                ()
            },
            Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { partialPrefix: false, name: id, .. }, .. }, .. } } => {
                DoubleEnded::push_back(delst.clone(), id.clone())?;
                ()
            },
            Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { variability: Absyn::Variability::CONST { .. }, .. }, components: lst, .. }, .. } } if (includeConstants) => {
                DoubleEnded::push_list_back(delst.clone(), &(ProgramUtil::getComponentItemsName(lst.clone(), false)))?;
                ()
            },
            _ => {
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    outStringLst = DoubleEnded::toListAndClear(delst, metamodelica::nil())?;
    Ok(outStringLst)
}

pub(crate) fn removeInnerClass(
    mut inClass1: metamodelica::Ref<Absyn::Class>,
    mut inClass2: metamodelica::Ref<Absyn::Class>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut outClass: metamodelica::Ref<Absyn::Class>;
    outClass = 'mc: {
        let __mc_input = (inClass1, inClass2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts: parts, ann, comment: cmt }, .. }) => {
                    let mut publst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut publst2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    let mut outClass = (*outClass).clone();
                    publst = ProgramUtil::getPublicList(metamodelica::AsArg::as_arg(&parts));
                    publst2 = removeClassInElementitemlist(publst.clone(), metamodelica::AsArg::as_arg(&c1))?;
                    parts2 = ProgramUtil::replacePublicList(metamodelica::AsArg::as_arg(&parts), publst2.clone())?;
                    assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts2.clone(), ann: ann.clone(), comment: cmt.clone() }));
                    Ok(outClass.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts: parts, ann, comment: cmt }, .. }) => {
                    let mut prolst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut prolst2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    let mut outClass = (*outClass).clone();
                    prolst = ProgramUtil::getProtectedList(metamodelica::AsArg::as_arg(&parts));
                    prolst2 = removeClassInElementitemlist(prolst.clone(), metamodelica::AsArg::as_arg(&c1))?;
                    parts2 = ProgramUtil::replaceProtectedList(metamodelica::AsArg::as_arg(&parts), prolst2.clone())?;
                    assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts2.clone(), ann: ann.clone(), comment: cmt.clone() }));
                    Ok(outClass.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcname, modifications: modif, parts, comment: cmt, ann }, .. }) => {
                    let mut publst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut publst2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    let mut outClass = (*outClass).clone();
                    publst = ProgramUtil::getPublicList(metamodelica::AsArg::as_arg(&parts));
                    publst2 = removeClassInElementitemlist(publst.clone(), metamodelica::AsArg::as_arg(&c1))?;
                    parts2 = ProgramUtil::replacePublicList(metamodelica::AsArg::as_arg(&parts), publst2.clone())?;
                    assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcname.clone(), modifications: modif.clone(), comment: cmt.clone(), parts: parts2.clone(), ann: ann.clone() }));
                    Ok(outClass.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcname, modifications: modif, parts, comment: cmt, ann }, .. }) => {
                    let mut prolst: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut prolst2: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut parts2: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
                    let mut outClass = (*outClass).clone();
                    prolst = ProgramUtil::getProtectedList(metamodelica::AsArg::as_arg(&parts));
                    prolst2 = removeClassInElementitemlist(prolst.clone(), metamodelica::AsArg::as_arg(&c1))?;
                    parts2 = ProgramUtil::replaceProtectedList(metamodelica::AsArg::as_arg(&parts), prolst2.clone())?;
                    assign_field!(outClass.body = metamodelica::Ref::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: bcname.clone(), modifications: modif.clone(), comment: cmt.clone(), parts: parts2.clone(), ann: ann.clone() }));
                    Ok(outClass.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { name: n, .. }, Deref @ Absyn::Class { name: a, info: file_info, .. }) => {
                    Error::addSourceMessage(&(Error::CLASS_NOT_FOUND.clone()), list![n.clone(), a.clone()], metamodelica::AsArg::as_arg(&file_info))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outClass)
}

fn removeClassInElementitemlist(
    mut inElements: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut inClass: &metamodelica::Ref<Absyn::Class>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>> {
    let mut outElements: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut name: ArcStr;
    let __arc1 = &(*inClass);
    let Absyn::CLASS { name: __pa0, .. } = &**__arc1;
    name = metamodelica::Own::own(__pa0);
    (outElements, _) =
        List::deleteMemberOnTrue(name, inElements, &move |__a0: ArcStr,
                                                          __a1: metamodelica::Ref<Absyn::ElementItem>|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(ProgramUtil::classElementItemIsNamed(&__a0, &__a1))
        })?;
    Ok(outElements)
}

pub(crate) fn getPathedElementInProgram(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut program: &Absyn::Program,
) -> Result<metamodelica::Ref<Absyn::Element>> {
    let mut element: metamodelica::Ref<Absyn::Element>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    match '__try0: {
        cls = unwrap_break_err!(ProgramUtil::getClassInProgram(&(AbsynUtil::pathFirstIdent(&path)), program), '__try0);
        Ok::<_, &'static str>((cls.clone(),))
    } {
        Ok((__try0_o0,)) => {
            cls = __try0_o0;
        }
        Err(_) => {
            cls = ProgramUtil::getClassInProgram(
                &(AbsynUtil::pathFirstIdent(&path)),
                &((FBuiltin::getInitialFunctions()?).0),
            )?;
        }
    }
    if AbsynUtil::pathIsIdent(&path) {
        element = metamodelica::Ref::new(Absyn::Element::ELEMENT {
            finalPrefix: false,
            redeclareKeywords: None,
            innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
            specification: metamodelica::Ref::new(Absyn::ElementSpec::CLASSDEF {
                replaceable_: false,
                class_: cls.clone(),
            }),
            info: cls.info.clone(),
            constrainClass: None,
        });
    } else {
        let __pa1 = ::match_deref::match_deref! { match &(getPathedElementInClass(AbsynUtil::pathRest(path)?, &cls)?) {
            Some(__pa1) => __pa1.clone(),
            _ => return Err("pattern mismatch"),
        } };
        element = metamodelica::Own::own(__pa1);
    }
    Ok(element)
}

fn getPathedElementInClass(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut cls: &metamodelica::Ref<Absyn::Class>,
) -> Result<Option<metamodelica::Ref<Absyn::Element>>> {
    let mut element: Option<metamodelica::Ref<Absyn::Element>> = None;
    for mut part in &*AbsynUtil::getClassPartsInClass(cls) {
        element = getPathedElementInClassPart(path.clone(), metamodelica::AsArg::as_arg(&part))?;
        if (element).is_some() {
            break;
        }
    }
    Ok(element)
}

fn getPathedElementInClassPart(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut part: &metamodelica::Ref<Absyn::ClassPart>,
) -> Result<Option<metamodelica::Ref<Absyn::Element>>> {
    let mut element: Option<metamodelica::Ref<Absyn::Element>> = None;
    let mut e: metamodelica::Ref<Absyn::Element>;
    for mut item in &*AbsynUtil::getElementItemsInClassPart(part) {
        if AbsynUtil::isElementItemNamed(&(AbsynUtil::pathFirstIdent(&path)), metamodelica::AsArg::as_arg(&item))? {
            let __pa0 = ::match_deref::match_deref! { match &(item.clone()) {
                Deref @ Absyn::ElementItem::ELEMENTITEM { element: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            if AbsynUtil::pathIsIdent(&path) {
                element = Some(e);
            } else {
                element = getPathedElementInElement(AbsynUtil::pathRest(path)?, &e)?;
            }
            break;
        }
    }
    Ok(element)
}

fn getPathedElementInElement(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut element: &metamodelica::Ref<Absyn::Element>,
) -> Result<Option<metamodelica::Ref<Absyn::Element>>> {
    let mut outElement: Option<metamodelica::Ref<Absyn::Element>>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    outElement = (::match_deref::match_deref! { match element {
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: __esc_cls, .. }, .. } => {
            cls = (*__esc_cls).clone();
            getPathedElementInClass(path, metamodelica::AsArg::as_arg(&cls))?
        },
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElement)
}

pub(crate) fn getPathedExtendsInProgram(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut extendsPath: &metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
) -> Option<metamodelica::Ref<Absyn::ElementSpec>> {
    let mut extendsSpec: Option<metamodelica::Ref<Absyn::ElementSpec>>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut env: GraphicEnvCache;
    if '__try0: {
        cls =
            unwrap_break_err!(ProgramUtil::getPathedClassInProgram(classPath.clone(), &program, false, false), '__try0);
        env = unwrap_break_err!(Interactive::getClassEnv(program.clone(), classPath.clone()), '__try0);
        for mut ext in &*getExtendsElementspecInClass(&cls) {
            let mut ext = ext.clone();
            ext = unwrap_break_err!(Interactive::makeExtendsFullyQualified(&ext, env.clone()), '__try0);
            if AbsynUtil::pathEqual(
                extendsPath,
                &(unwrap_break_err!(AbsynUtil::elementSpecToPath(&ext), '__try0)),
            ) {
                extendsSpec = Some(ext.clone());
                return extendsSpec;
            }
        }
        Ok::<(), &'static str>(())
    }
    .is_err()
    {}
    extendsSpec = None;
    extendsSpec
}

pub(crate) fn transformPathedElementInList<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inList: metamodelica::List<T>,
    mut inFunc: &dyn ::std::ops::Fn(T) -> Result<(T, Option<metamodelica::Ref<Absyn::Element>>, bool)>,
) -> Result<(metamodelica::List<T>, Option<metamodelica::Ref<Absyn::Element>>, bool)> {
    pub type FuncType<T: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<(T, Option<metamodelica::Ref<Absyn::Element>>, bool)> + 'static>;

    let mut outList: metamodelica::List<T> = metamodelica::nil();
    let mut outElement: Option<metamodelica::Ref<Absyn::Element>> = None;
    let mut outFound: bool = false;
    let mut e: T;
    let mut rest: metamodelica::List<T> = inList;
    while !((rest).is_empty()) && !(outFound) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        (e, outElement, outFound) = inFunc(e)?;
        outList = metamodelica::cons(e, outList);
    }
    outList = List::append_reverse(&outList, rest);
    Ok((outList, outElement, outFound))
}

pub(crate) fn transformPathedElementInProgram(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::Element>> + 'static,
    >,
    mut program: Absyn::Program,
) -> Result<(Absyn::Program, Option<metamodelica::Ref<Absyn::Element>>, bool)> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::Element>> + 'static,
    >;

    fn transform_class(
        mut path: metamodelica::Ref<Absyn::Path>,
        mut func: Arc<
            dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::Element>>
                + 'static,
        >,
        mut cls: metamodelica::Ref<Absyn::Class>,
    ) -> Result<(
        metamodelica::Ref<Absyn::Class>,
        Option<metamodelica::Ref<Absyn::Element>>,
        bool,
    )> {
        let mut cls: metamodelica::Ref<Absyn::Class> = cls;
        let mut outElement: Option<metamodelica::Ref<Absyn::Element>>;
        let mut found: bool;
        let mut elem: metamodelica::Ref<Absyn::Element>;
        found = metamodelica::stringEq(&(AbsynUtil::pathFirstIdent(&(path.clone()))), &cls.name);
        if found {
            if AbsynUtil::pathIsIdent(&(path.clone())) {
                elem = metamodelica::Ref::new(Absyn::Element::ELEMENT {
                    finalPrefix: false,
                    redeclareKeywords: None,
                    innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
                    specification: metamodelica::Ref::new(Absyn::ElementSpec::CLASSDEF {
                        replaceable_: false,
                        class_: cls.clone(),
                    }),
                    info: cls.info.clone(),
                    constrainClass: None,
                });
                elem = func(elem)?;
                outElement = Some(elem.clone());
                let __pa0 = ::match_deref::match_deref! { match &(elem) {
                    Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: __pa0, .. }, .. } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                cls = metamodelica::Own::own(__pa0);
            } else {
                (cls, outElement, found) =
                    transformPathedElementInClass(&(AbsynUtil::pathRest(path.clone())?), func.clone(), cls)?;
            }
        } else {
            outElement = None;
        }
        Ok((cls, outElement, found))
    }

    let mut program: Absyn::Program = program;
    let mut element: Option<metamodelica::Ref<Absyn::Element>>;
    let mut success: bool;
    let mut clss: metamodelica::List<metamodelica::Ref<Absyn::Class>>;
    (clss, element, success) = transformPathedElementInList(
        program.classes.clone(),
        &({
            let __pe_b0 = path.clone();
            let __pe_b1 = func.clone();
            move |__pe_a2| transform_class(__pe_b0.clone(), __pe_b1.clone(), __pe_a2)
        }),
    )?;
    if success {
        program.classes = clss;
    }
    Ok((program, element, success))
}

fn transformPathedElementInClass(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::Element>> + 'static,
    >,
    mut cls: metamodelica::Ref<Absyn::Class>,
) -> Result<(
    metamodelica::Ref<Absyn::Class>,
    Option<metamodelica::Ref<Absyn::Element>>,
    bool,
)> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::Element>> + 'static,
    >;

    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    let mut element: Option<metamodelica::Ref<Absyn::Element>>;
    let mut success: bool;
    let mut def: metamodelica::Ref<Absyn::ClassDef>;
    (def, element, success) = transformPathedElementInClassDef(path, func.clone(), cls.body.clone())?;
    if success {
        assign_field!(cls.body = def);
    }
    Ok((cls, element, success))
}

fn transformPathedElementInClassDef(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::Element>> + 'static,
    >,
    mut def: metamodelica::Ref<Absyn::ClassDef>,
) -> Result<(
    metamodelica::Ref<Absyn::ClassDef>,
    Option<metamodelica::Ref<Absyn::Element>>,
    bool,
)> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::Element>> + 'static,
    >;

    let mut def: metamodelica::Ref<Absyn::ClassDef> = def;
    let mut element: Option<metamodelica::Ref<Absyn::Element>> = None;
    let mut success: bool;
    let mut parts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    success = (match &*def {
        Absyn::ClassDef::PARTS {
            classParts: __def_classParts,
            ..
        } => {
            (parts, element, success) = transformPathedElementInList(
                __def_classParts.clone(),
                &({
                    let __pe_b0 = path.clone();
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Absyn::Element>,
                            ) -> Result<metamodelica::Ref<Absyn::Element>>
                            + 'static,
                    > = func.clone();
                    move |__pe_a2| transformPathedElementInClassPart(&__pe_b0, __pe_b1.clone(), __pe_a2)
                }),
            )?;
            if success {
                assign_variant_field!(def => Absyn::ClassDef::PARTS; classParts = parts);
            }
            success
        }
        Absyn::ClassDef::CLASS_EXTENDS { parts: __def_parts, .. } => {
            (parts, element, success) = transformPathedElementInList(
                __def_parts.clone(),
                &({
                    let __pe_b0 = path.clone();
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Absyn::Element>,
                            ) -> Result<metamodelica::Ref<Absyn::Element>>
                            + 'static,
                    > = func.clone();
                    move |__pe_a2| transformPathedElementInClassPart(&__pe_b0, __pe_b1.clone(), __pe_a2)
                }),
            )?;
            if success {
                assign_variant_field!(def => Absyn::ClassDef::CLASS_EXTENDS; parts = parts);
            }
            success
        }
        _ => false,
    });
    Ok((def, element, success))
}

fn transformPathedElementInClassPart(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::Element>> + 'static,
    >,
    mut part: metamodelica::Ref<Absyn::ClassPart>,
) -> Result<(
    metamodelica::Ref<Absyn::ClassPart>,
    Option<metamodelica::Ref<Absyn::Element>>,
    bool,
)> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::Element>> + 'static,
    >;

    let mut part: metamodelica::Ref<Absyn::ClassPart> = part;
    let mut element: Option<metamodelica::Ref<Absyn::Element>> = None;
    let mut success: bool;
    let mut items: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    success = (match &*part {
        Absyn::ClassPart::PUBLIC {
            contents: __part_contents,
        } => {
            (items, element, success) = transformPathedElementInList(
                __part_contents.clone(),
                &({
                    let __pe_b0 = path.clone();
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Absyn::Element>,
                            ) -> Result<metamodelica::Ref<Absyn::Element>>
                            + 'static,
                    > = func.clone();
                    move |__pe_a2| transformPathedElementInElementItem(__pe_b0.clone(), __pe_b1.clone(), __pe_a2)
                }),
            )?;
            if success {
                assign_variant_field!(part => Absyn::ClassPart::PUBLIC; contents = items);
            }
            success
        }
        Absyn::ClassPart::PROTECTED {
            contents: __part_contents,
        } => {
            (items, element, success) = transformPathedElementInList(
                __part_contents.clone(),
                &({
                    let __pe_b0 = path.clone();
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Absyn::Element>,
                            ) -> Result<metamodelica::Ref<Absyn::Element>>
                            + 'static,
                    > = func.clone();
                    move |__pe_a2| transformPathedElementInElementItem(__pe_b0.clone(), __pe_b1.clone(), __pe_a2)
                }),
            )?;
            if success {
                assign_variant_field!(part => Absyn::ClassPart::PROTECTED; contents = items);
            }
            success
        }
        _ => false,
    });
    Ok((part, element, success))
}

fn transformPathedElementInElementItem(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::Element>> + 'static,
    >,
    mut item: metamodelica::Ref<Absyn::ElementItem>,
) -> Result<(
    metamodelica::Ref<Absyn::ElementItem>,
    Option<metamodelica::Ref<Absyn::Element>>,
    bool,
)> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::Element>> + 'static,
    >;

    let mut item: metamodelica::Ref<Absyn::ElementItem> = item;
    let mut outElement: Option<metamodelica::Ref<Absyn::Element>> = None;
    let mut success: bool;
    let mut element: metamodelica::Ref<Absyn::Element>;
    success = (match &*item {
        Absyn::ElementItem::ELEMENTITEM { .. }
            if (AbsynUtil::isElementItemNamed(&(AbsynUtil::pathFirstIdent(&path)), &item)?) =>
        {
            if AbsynUtil::pathIsIdent(&path) {
                assign_variant_field!(item => Absyn::ElementItem::ELEMENTITEM; element = func(var_field!((*item).element, Absyn::ElementItem::ELEMENTITEM).clone())?);
                outElement = Some(var_field!((*item).element, Absyn::ElementItem::ELEMENTITEM).clone());
                success = true;
            } else {
                (element, outElement, success) = transformPathedElementInElement(
                    &(AbsynUtil::pathRest(path.clone())?),
                    func.clone(),
                    var_field!((*item).element, Absyn::ElementItem::ELEMENTITEM).clone(),
                )?;
                if success {
                    assign_variant_field!(item => Absyn::ElementItem::ELEMENTITEM; element = element);
                }
            }
            success
        }
        _ => false,
    });
    Ok((item, outElement, success))
}

fn transformPathedElementInElement(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::Element>> + 'static,
    >,
    mut element: metamodelica::Ref<Absyn::Element>,
) -> Result<(
    metamodelica::Ref<Absyn::Element>,
    Option<metamodelica::Ref<Absyn::Element>>,
    bool,
)> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::Element>> + 'static,
    >;

    let mut element: metamodelica::Ref<Absyn::Element> = element;
    let mut outElement: Option<metamodelica::Ref<Absyn::Element>> = None;
    let mut success: bool;
    let mut spec: metamodelica::Ref<Absyn::ElementSpec>;
    success = (match &*element {
        Absyn::Element::ELEMENT {
            specification: __element_specification,
            ..
        } => {
            (spec, outElement, success) =
                transformPathedElementInElementSpec(path, func.clone(), __element_specification.clone())?;
            if success {
                assign_variant_field!(element => Absyn::Element::ELEMENT; specification = spec);
            }
            success
        }
        _ => false,
    });
    Ok((element, outElement, success))
}

fn transformPathedElementInElementSpec(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::Element>> + 'static,
    >,
    mut spec: metamodelica::Ref<Absyn::ElementSpec>,
) -> Result<(
    metamodelica::Ref<Absyn::ElementSpec>,
    Option<metamodelica::Ref<Absyn::Element>>,
    bool,
)> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::Element>> + 'static,
    >;

    let mut spec: metamodelica::Ref<Absyn::ElementSpec> = spec;
    let mut element: Option<metamodelica::Ref<Absyn::Element>> = None;
    let mut success: bool;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    success = (match &*spec {
        Absyn::ElementSpec::CLASSDEF {
            class_: __spec_class_, ..
        } => {
            (cls, element, success) = transformPathedElementInClass(path, func.clone(), __spec_class_.clone())?;
            if success {
                assign_variant_field!(spec => Absyn::ElementSpec::CLASSDEF; class_ = cls);
            }
            success
        }
        _ => false,
    });
    Ok((spec, element, success))
}

pub(crate) fn getPathedClassRestriction(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut program: &Absyn::Program,
) -> Absyn::Restriction {
    let mut restriction: Absyn::Restriction;
    match '__try0: {
        let __arc2 =
            unwrap_break_err!(ProgramUtil::getPathedClassInProgram(path.clone(), program, false, false), '__try0);
        let Absyn::CLASS { restriction: __pa1, .. } = &*__arc2;
        restriction = metamodelica::Own::own(__pa1);
        Ok::<_, &'static str>((restriction.clone(),))
    } {
        Ok((__try0_o0,)) => {
            restriction = __try0_o0;
        }
        Err(_) => {
            restriction = openmodelica_ast::Absyn::Restriction::R_UNKNOWN;
        }
    }
    restriction
}

pub(crate) fn getPathedSCodeElementInProgram(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut program: metamodelica::List<metamodelica::Ref<SCode::Element>>,
) -> Result<metamodelica::Ref<SCode::Element>> {
    let mut element: metamodelica::Ref<SCode::Element>;
    let mut name: ArcStr;
    name = AbsynUtil::pathFirstIdent(&path);
    element = List::find(
        &program,
        &({
            let __pe_b0 = name;
            move |__pe_a1| Ok(SCodeUtil::isElementNamed(&__pe_b0, &__pe_a1))
        }),
    )?;
    if !(AbsynUtil::pathIsIdent(&path)) {
        element = getPathedSCodeElementInProgram(AbsynUtil::pathRest(path)?, SCodeUtil::getClassElements(&element))?;
    }
    Ok(element)
}

pub(crate) fn getElementAnnotation(
    mut elementPath: metamodelica::Ref<Absyn::Path>,
    mut program: &Absyn::Program,
) -> ArcStr {
    let mut annotationString: ArcStr;
    let mut elem: metamodelica::Ref<Absyn::Element>;
    let mut ann: Option<metamodelica::Ref<Absyn::Annotation>>;
    let mut eargs: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    match '__try0: {
        elem = unwrap_break_err!(getPathedElementInProgram(elementPath.clone(), program), '__try0);
        ann = unwrap_break_err!(AbsynUtil::getElementAnnotation(&elem, &(AbsynUtil::pathLastIdent(&elementPath))), '__try0);
        if (ann).is_some() {
            let __pa1 = ::match_deref::match_deref! { match &(ann.clone()) {
                Some(Deref @ Absyn::Annotation { elementArgs: __pa1 }) => __pa1.clone(),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            eargs = metamodelica::Own::own(__pa1);
            annotationString = unwrap_break_err!(List::toString(eargs.clone(), &Dump::unparseElementArgStr, List::Style::FLAT_BRACKETS.clone()), '__try0);
        } else {
            annotationString = literal!("()");
        }
        Ok::<_, &'static str>((annotationString.clone(),))
    } {
        Ok((__try0_o0,)) => {
            annotationString = __try0_o0;
        }
        Err(_) => {
            annotationString = literal!("");
        }
    }
    annotationString
}

pub(crate) fn setElementAnnotation(
    mut elementPath: &metamodelica::Ref<Absyn::Path>,
    mut annotationMod: &metamodelica::Ref<Absyn::Modification>,
    mut program: Absyn::Program,
) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut success: bool = true;
    let mut ann: Option<metamodelica::Ref<Absyn::Annotation>>;
    let mut name: ArcStr;
    let mut elem_opt: Option<metamodelica::Ref<Absyn::Element>>;
    match '__try0: {
        if (annotationMod.elementArgLst).is_empty() {
            ann = None;
        } else {
            ann = Some(metamodelica::Ref::new(Absyn::Annotation {
                elementArgs: annotationMod.elementArgLst.clone(),
            }));
        }
        name = AbsynUtil::pathLastIdent(elementPath);
        (program, elem_opt, success) = unwrap_break_err!(transformPathedElementInProgram(elementPath, (std::sync::Arc::new({ let __pe_b1 = name.clone(); let __pe_b2 = ann.clone(); move |__pe_a0| AbsynUtil::setElementAnnotation(__pe_a0, &__pe_b1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::Element>> + 'static>), program.clone()), '__try0);
        if success {
            unwrap_break_err!(SymbolTable::setAbsynElement(program.clone(), &(unwrap_break_err!(elem_opt.clone().ok_or("pattern mismatch"), '__try0)), elementPath), '__try0);
        }
        Ok::<_, &'static str>((success.clone(),))
    } {
        Ok((__try0_o0,)) => {
            success = __try0_o0;
        }
        Err(_) => {
            success = false;
        }
    }
    (program, success)
}

pub(crate) fn loadClassContentString(
    mut content: ArcStr,
    mut classPath: &metamodelica::Ref<Absyn::Path>,
    mut offsetX: i32,
    mut offsetY: i32,
    mut program: Absyn::Program,
) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut success: bool = true;
    let mut parsed_body: metamodelica::Ref<Absyn::ClassDef>;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(Parser::parsestring(stringAppendList(list![literal!("model dummy\n"), content.clone(), literal!("end dummy;\n")]), literal!("<interactive>"), unwrap_break_err!(Config::acceptedGrammar(), '__try0), unwrap_break_err!(Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone()), '__try0), unwrap_break_err!(Flags::getConfigBool(Flags::STRICT.clone()), '__try0)), '__try0)) {
            Absyn::Program { classes: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Class { body: __pa1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        parsed_body = metamodelica::Own::own(__pa1);
        parsed_body = unwrap_break_err!(offsetAnnotationsInClassDef(parsed_body.clone(), offsetX, offsetY), '__try0);
        (program, _, success) = unwrap_break_err!(transformPathedElementInProgram(classPath, (std::sync::Arc::new({ let __pe_b1 = parsed_body.clone(); move |__pe_a0| mergeClassContents(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::Element>> + 'static>), program.clone()), '__try0);
        Ok::<_, &'static str>((success.clone(),))
    } {
        Ok((__try0_o0,)) => {
            success = __try0_o0;
        }
        Err(_) => {
            success = false;
        }
    }
    (program, success)
}

pub(crate) fn mergeClassContents(
    mut element: metamodelica::Ref<Absyn::Element>,
    mut newContent: metamodelica::Ref<Absyn::ClassDef>,
) -> Result<metamodelica::Ref<Absyn::Element>> {
    let mut element: metamodelica::Ref<Absyn::Element> = element;
    let mut spec: metamodelica::Ref<Absyn::ElementSpec>;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut old_content: metamodelica::Ref<Absyn::ClassDef>;
    let mut new_content: metamodelica::Ref<Absyn::ClassDef>;
    new_content = resolveMergeContentsConflicts(&element, newContent)?;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { specification: __esc_spec @ Deref @ Absyn::ElementSpec::CLASSDEF { class_: __esc_cls @ Deref @ Absyn::Class { body: __esc_old_content, .. }, .. }, .. } => {
            spec = (*__esc_spec).clone();
            cls = (*__esc_cls).clone();
            old_content = (*__esc_old_content).clone();
            let () = (::match_deref::match_deref! { match &((old_content.clone(), new_content.clone())) {
        (Deref @ Absyn::ClassDef::PARTS { .. }, Deref @ Absyn::ClassDef::PARTS { .. }) => {
            assign_variant_field!(old_content => Absyn::ClassDef::PARTS;
                classParts = mergeClassParts(var_field!((*new_content).classParts, Absyn::ClassDef::PARTS), var_field!((*old_content).classParts, Absyn::ClassDef::PARTS).clone())?,
                ann = mergeAnnotationLists(var_field!((*new_content).ann, Absyn::ClassDef::PARTS).clone(), var_field!((*old_content).ann, Absyn::ClassDef::PARTS))?
            );
            ()
        },
        (Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, Deref @ Absyn::ClassDef::PARTS { .. }) => {
            assign_variant_field!(old_content => Absyn::ClassDef::CLASS_EXTENDS;
                parts = mergeClassParts(var_field!((*new_content).classParts, Absyn::ClassDef::PARTS), var_field!((*old_content).parts, Absyn::ClassDef::CLASS_EXTENDS).clone())?,
                ann = mergeAnnotationLists(var_field!((*new_content).ann, Absyn::ClassDef::PARTS).clone(), var_field!((*old_content).ann, Absyn::ClassDef::CLASS_EXTENDS))?
            );
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
            assign_field!(cls.body = old_content.clone());
            assign_variant_field!(spec => Absyn::ElementSpec::CLASSDEF; class_ = cls.clone());
            assign_variant_field!(element => Absyn::Element::ELEMENT; specification = spec.clone());
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(element)
}

fn mergeClassParts(
    mut newParts: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut oldParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut outParts: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    let mut parts: metamodelica::Ref<Vector::Vector<metamodelica::Ref<Absyn::ClassPart>>>;
    let mut op: Option<metamodelica::Ref<Absyn::ClassPart>>;
    let mut p: metamodelica::Ref<Absyn::ClassPart>;
    let mut index: i32;
    parts = Vector::fromList(oldParts);
    for mut part in &**newParts {
        let mut part = part.clone();
        let () = (match &*part {
            Absyn::ClassPart::PUBLIC { .. } => {
                (op, index) = Vector::findLast(
                    parts.clone(),
                    (std::sync::Arc::new(
                        move |__a0: metamodelica::Ref<Absyn::ClassPart>| -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(AbsynUtil::isElementSection(&__a0))
                        },
                    )
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ClassPart>) -> Result<bool> + 'static,
                        >),
                )?;
                let () = (::match_deref::match_deref! { match &(op) {
                    Some(__esc_p @ Deref @ Absyn::ClassPart::PUBLIC { .. }) => {
                        p = (*__esc_p).clone();
                        assign_variant_field!(part => Absyn::ClassPart::PUBLIC; contents = listAppend(var_field!((*p).contents, Absyn::ClassPart::PUBLIC).clone(), var_field!((*part).contents, Absyn::ClassPart::PUBLIC).clone()));
                        Vector::updateNoBounds(parts.clone(), index, part);
                        ()
                    },
                    _ => {
                        Vector::insert(parts.clone(), part, std::cmp::max(index + 1, 1))?;
                        ()
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                ()
            }
            Absyn::ClassPart::PROTECTED { .. } => {
                (op, index) = Vector::findLast(
                    parts.clone(),
                    (std::sync::Arc::new(
                        move |__a0: metamodelica::Ref<Absyn::ClassPart>| -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(AbsynUtil::isElementSection(&__a0))
                        },
                    )
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ClassPart>) -> Result<bool> + 'static,
                        >),
                )?;
                let () = (::match_deref::match_deref! { match &(op) {
                    Some(__esc_p @ Deref @ Absyn::ClassPart::PROTECTED { .. }) => {
                        p = (*__esc_p).clone();
                        assign_variant_field!(part => Absyn::ClassPart::PROTECTED; contents = listAppend(var_field!((*p).contents, Absyn::ClassPart::PROTECTED).clone(), var_field!((*part).contents, Absyn::ClassPart::PROTECTED).clone()));
                        Vector::updateNoBounds(parts.clone(), index, part);
                        ()
                    },
                    _ => {
                        Vector::insert(parts.clone(), part, std::cmp::max(index + 1, 1))?;
                        ()
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                ()
            }
            Absyn::ClassPart::EQUATIONS { .. } => {
                (op, index) = Vector::findLast(
                    parts.clone(),
                    (std::sync::Arc::new(
                        move |__a0: metamodelica::Ref<Absyn::ClassPart>| -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(AbsynUtil::isEquationSection(&__a0))
                        },
                    )
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ClassPart>) -> Result<bool> + 'static,
                        >),
                )?;
                let () = (::match_deref::match_deref! { match &(op) {
                    Some(__esc_p @ Deref @ Absyn::ClassPart::EQUATIONS { .. }) => {
                        p = (*__esc_p).clone();
                        assign_variant_field!(part => Absyn::ClassPart::EQUATIONS; contents = listAppend(var_field!((*p).contents, Absyn::ClassPart::EQUATIONS).clone(), var_field!((*part).contents, Absyn::ClassPart::EQUATIONS).clone()));
                        Vector::updateNoBounds(parts.clone(), index, part);
                        ()
                    },
                    _ => {
                        if index == -1 {
                            (_, index) = Vector::findLast(parts.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::ClassPart>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::isElementSection(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ClassPart>) -> Result<bool> + 'static>))?;
                        }
                        Vector::insert(parts.clone(), part, std::cmp::max(index + 1, 1))?;
                        ()
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                ()
            }
            Absyn::ClassPart::INITIALEQUATIONS { .. } => {
                (op, index) = Vector::findLast(
                    parts.clone(),
                    (std::sync::Arc::new(
                        move |__a0: metamodelica::Ref<Absyn::ClassPart>| -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(AbsynUtil::isEquationSection(&__a0))
                        },
                    )
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ClassPart>) -> Result<bool> + 'static,
                        >),
                )?;
                let () = (::match_deref::match_deref! { match &(op) {
                    Some(__esc_p @ Deref @ Absyn::ClassPart::INITIALEQUATIONS { .. }) => {
                        p = (*__esc_p).clone();
                        assign_variant_field!(part => Absyn::ClassPart::INITIALEQUATIONS; contents = listAppend(var_field!((*p).contents, Absyn::ClassPart::INITIALEQUATIONS).clone(), var_field!((*part).contents, Absyn::ClassPart::INITIALEQUATIONS).clone()));
                        Vector::updateNoBounds(parts.clone(), index, part);
                        ()
                    },
                    _ => {
                        if index == -1 {
                            (_, index) = Vector::findLast(parts.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::ClassPart>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::isElementSection(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ClassPart>) -> Result<bool> + 'static>))?;
                        }
                        Vector::insert(parts.clone(), part, std::cmp::max(index + 1, 1))?;
                        ()
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                ()
            }
            Absyn::ClassPart::EXTERNAL { .. } => {
                (_, index) = Vector::findLast(
                    parts.clone(),
                    (std::sync::Arc::new(
                        move |__a0: metamodelica::Ref<Absyn::ClassPart>| -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(AbsynUtil::isExternalPart(&__a0))
                        },
                    )
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ClassPart>) -> Result<bool> + 'static,
                        >),
                )?;
                if index != -1 {
                    Vector::updateNoBounds(parts.clone(), index, part);
                } else {
                    Vector::push(parts.clone(), part);
                }
                ()
            }
            _ => {
                Vector::push(parts.clone(), part);
                ()
            }
        });
    }
    outParts = Vector::toList(parts);
    Ok(outParts)
}

fn mergeAnnotationLists(
    mut newAnnotations: metamodelica::List<metamodelica::Ref<Absyn::Annotation>>,
    mut oldAnnotations: &metamodelica::List<metamodelica::Ref<Absyn::Annotation>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Annotation>>> {
    let mut outAnnotations: metamodelica::List<metamodelica::Ref<Absyn::Annotation>>;
    let mut old_ann: metamodelica::Ref<Absyn::Annotation>;
    if (oldAnnotations).is_empty() {
        outAnnotations = newAnnotations;
    } else {
        old_ann = (oldAnnotations).head().cloned()?;
        for mut new_ann in &*newAnnotations {
            old_ann = AbsynUtil::mergeAnnotations(old_ann, new_ann.clone(), true, true)?;
        }
        outAnnotations = metamodelica::cons(old_ann, (oldAnnotations).rest()?);
    }
    Ok(outAnnotations)
}

fn resolveMergeContentsConflicts(
    mut oldElement: &metamodelica::Ref<Absyn::Element>,
    mut newContent: metamodelica::Ref<Absyn::ClassDef>,
) -> Result<metamodelica::Ref<Absyn::ClassDef>> {
    let mut newContent: metamodelica::Ref<Absyn::ClassDef> = newContent;
    let mut old_names: metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>;
    let mut rename_map: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>;
    let mut new_name: ArcStr;
    let mut index: i32;
    let mut conflicting_names: metamodelica::List<ArcStr> = metamodelica::nil();
    old_names = UnorderedSet::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        13,
    );
    for mut e in &*AbsynUtil::getElementItemsInElement(oldElement)? {
        for mut name in &*AbsynUtil::elementItemNames(metamodelica::AsArg::as_arg(&e))? {
            UnorderedSet::add(name.clone(), old_names.clone())?;
        }
    }
    rename_map = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    for mut e in &*AbsynUtil::getElementItemsInClassDef(&newContent)? {
        for mut name in &*AbsynUtil::elementItemNames(metamodelica::AsArg::as_arg(&e))? {
            if UnorderedSet::contains(name.clone(), old_names.clone())? {
                conflicting_names = metamodelica::cons(name.clone(), conflicting_names);
            } else {
                UnorderedSet::add(name.clone(), old_names.clone())?;
            }
        }
    }
    if (conflicting_names).is_empty() {
        return Ok(newContent);
    }
    for mut name in &*conflicting_names.reverse() {
        index = 1;
        new_name = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*name);
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index)));
            ArcStr::from(__mm_s)
        };
        while UnorderedSet::contains(new_name.clone(), old_names.clone())? {
            index = index + 1;
            new_name = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index)));
                ArcStr::from(__mm_s)
            };
        }
        UnorderedMap::add(name.clone(), new_name.clone(), rename_map.clone())?;
        UnorderedSet::add(new_name, old_names.clone())?;
    }
    if !(UnorderedMap::isEmpty(rename_map.clone())) {
        newContent = renameElementsInClassDef(newContent, rename_map)?;
    }
    Ok(newContent)
}

fn renameElementsInElement(
    mut element: metamodelica::Ref<Absyn::Element>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::Ref<Absyn::Element>> {
    let mut element: metamodelica::Ref<Absyn::Element> = element;
    let () = (match &*element {
        Absyn::Element::ELEMENT {
            specification: __element_specification,
            ..
        } => {
            assign_variant_field!(element => Absyn::Element::ELEMENT;
                specification = renameElementsInElementSpec(__element_specification.clone(), nameMap.clone(), true)?,
                constrainClass = renameElementsInConstrainClassOpt(var_field!((*element).constrainClass, Absyn::Element::ELEMENT).clone(), nameMap)?
            );
            ()
        }
        _ => (),
    });
    Ok(element)
}

fn renameElementsInElementSpec(
    mut spec: metamodelica::Ref<Absyn::ElementSpec>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
    mut renameElement: bool,
) -> Result<metamodelica::Ref<Absyn::ElementSpec>> {
    let mut spec: metamodelica::Ref<Absyn::ElementSpec> = spec;
    let () = (match &*spec {
        Absyn::ElementSpec::CLASSDEF {
            class_: __spec_class_, ..
        } => {
            assign_variant_field!(spec => Absyn::ElementSpec::CLASSDEF; class_ = renameElementsInClass(__spec_class_.clone(), nameMap, renameElement)?);
            ()
        }
        Absyn::ElementSpec::EXTENDS {
            elementArg: __spec_elementArg,
            ..
        } => {
            assign_variant_field!(spec => Absyn::ElementSpec::EXTENDS;
                        elementArg = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
                for mut a in (__spec_elementArg.clone()).into_iter().cloned() {
                    let __x = renameElementsInElementArg(a.clone(), nameMap.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        annotationOpt = renameElementsInAnnotationOpt(var_field!((*spec).annotationOpt, Absyn::ElementSpec::EXTENDS).clone(), nameMap)?
                    );
            ()
        }
        Absyn::ElementSpec::COMPONENTS {
            attributes: __spec_attributes,
            ..
        } => {
            assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS;
                        attributes = renameElementsInAttributes(__spec_attributes.clone(), nameMap.clone())?,
                        typeSpec = renameElementsInTypeSpec(var_field!((*spec).typeSpec, Absyn::ElementSpec::COMPONENTS).clone(), nameMap.clone())?,
                        components = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>> = metamodelica::nil();
                for mut c in (var_field!((*spec).components, Absyn::ElementSpec::COMPONENTS).clone()).into_iter().cloned() {
                    let __x = renameElementsInComponentItem(c.clone(), nameMap.clone(), renameElement)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            ()
        }
        _ => (),
    });
    Ok(spec)
}

fn renameElementsInClass(
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
    mut renameElement: bool,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    if renameElement {
        assign_field!(cls.name = renameElementsInIdent(cls.name.clone(), nameMap.clone())?);
    }
    assign_field!(cls.body = renameElementsInClassDef(cls.body.clone(), nameMap)?);
    Ok(cls)
}

fn renameElementsInClassDef(
    mut classDef: metamodelica::Ref<Absyn::ClassDef>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::Ref<Absyn::ClassDef>> {
    let mut classDef: metamodelica::Ref<Absyn::ClassDef> = classDef;
    let () = (match &*classDef {
        Absyn::ClassDef::PARTS {
            classParts: __classDef_classParts,
            ..
        } => {
            assign_variant_field!(classDef => Absyn::ClassDef::PARTS;
                        classParts = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
                for mut p in (__classDef_classParts.clone()).into_iter().cloned() {
                    let __x = renameElementsInClassPart(p.clone(), nameMap.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        ann = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Annotation>> = metamodelica::nil();
                for mut a in (var_field!((*classDef).ann, Absyn::ClassDef::PARTS).clone()).into_iter().cloned() {
                    let __x = renameElementsInAnnotation(a.clone(), nameMap.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            ()
        }
        Absyn::ClassDef::DERIVED {
            typeSpec: __classDef_typeSpec,
            ..
        } => {
            assign_variant_field!(classDef => Absyn::ClassDef::DERIVED;
                        typeSpec = renameElementsInTypeSpec(__classDef_typeSpec.clone(), nameMap.clone())?,
                        attributes = renameElementsInAttributes(var_field!((*classDef).attributes, Absyn::ClassDef::DERIVED).clone(), nameMap.clone())?,
                        arguments = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
                for mut a in (var_field!((*classDef).arguments, Absyn::ClassDef::DERIVED).clone()).into_iter().cloned() {
                    let __x = renameElementsInElementArg(a.clone(), nameMap.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        comment = renameElementsInCommentOpt(var_field!((*classDef).comment, Absyn::ClassDef::DERIVED).clone(), nameMap)?
                    );
            ()
        }
        Absyn::ClassDef::CLASS_EXTENDS {
            modifications: __classDef_modifications,
            ..
        } => {
            assign_variant_field!(classDef => Absyn::ClassDef::CLASS_EXTENDS;
                        modifications = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
                for mut a in (__classDef_modifications.clone()).into_iter().cloned() {
                    let __x = renameElementsInElementArg(a.clone(), nameMap.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        parts = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
                for mut p in (var_field!((*classDef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone()).into_iter().cloned() {
                    let __x = renameElementsInClassPart(p.clone(), nameMap.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        ann = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Annotation>> = metamodelica::nil();
                for mut a in (var_field!((*classDef).ann, Absyn::ClassDef::CLASS_EXTENDS).clone()).into_iter().cloned() {
                    let __x = renameElementsInAnnotation(a.clone(), nameMap.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            ()
        }
        _ => (),
    });
    Ok(classDef)
}

fn renameElementsInClassPart(
    mut part: metamodelica::Ref<Absyn::ClassPart>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::Ref<Absyn::ClassPart>> {
    let mut part: metamodelica::Ref<Absyn::ClassPart> = part;
    let () = (match &*part {
        Absyn::ClassPart::PUBLIC {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::PUBLIC; contents = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = metamodelica::nil();
                for mut i in (__part_contents.clone()).into_iter().cloned() {
                    let __x = renameElementsInElementItem(i.clone(), nameMap.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        Absyn::ClassPart::PROTECTED {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::PROTECTED; contents = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = metamodelica::nil();
                for mut i in (__part_contents.clone()).into_iter().cloned() {
                    let __x = renameElementsInElementItem(i.clone(), nameMap.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        Absyn::ClassPart::CONSTRAINTS {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::CONSTRAINTS; contents = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
                for mut e in (__part_contents.clone()).into_iter().cloned() {
                    let __x = (renameElementsInExp(e.clone(), nameMap.clone())?).0;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        Absyn::ClassPart::EQUATIONS {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::EQUATIONS; contents = renameElementsInEquationItems(__part_contents.clone(), nameMap)?);
            ()
        }
        Absyn::ClassPart::INITIALEQUATIONS {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::INITIALEQUATIONS; contents = renameElementsInEquationItems(__part_contents.clone(), nameMap)?);
            ()
        }
        Absyn::ClassPart::ALGORITHMS {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::ALGORITHMS; contents = renameElementsInAlgorithmItems(__part_contents.clone(), nameMap)?);
            ()
        }
        Absyn::ClassPart::INITIALALGORITHMS {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::INITIALALGORITHMS; contents = renameElementsInAlgorithmItems(__part_contents.clone(), nameMap)?);
            ()
        }
        Absyn::ClassPart::EXTERNAL {
            externalDecl: __part_externalDecl,
            ..
        } => {
            assign_variant_field!(part => Absyn::ClassPart::EXTERNAL;
                externalDecl = renameElementsInExternalDecl(__part_externalDecl.clone(), nameMap.clone())?,
                annotation_ = renameElementsInAnnotationOpt(var_field!((*part).annotation_, Absyn::ClassPart::EXTERNAL).clone(), nameMap)?
            );
            ()
        }
        _ => (),
    });
    Ok(part)
}

fn renameElementsInElementItem(
    mut item: metamodelica::Ref<Absyn::ElementItem>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::Ref<Absyn::ElementItem>> {
    let mut item: metamodelica::Ref<Absyn::ElementItem> = item;
    let () = (match &*item {
        Absyn::ElementItem::ELEMENTITEM {
            element: __item_element,
        } => {
            assign_variant_field!(item => Absyn::ElementItem::ELEMENTITEM; element = renameElementsInElement(__item_element.clone(), nameMap)?);
            ()
        }
        _ => (),
    });
    Ok(item)
}

fn renameElementsInEquationItems(
    mut items: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>> {
    let mut items: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>> = items;
    items = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>> = metamodelica::nil();
        for mut i in (items).into_iter().cloned() {
            let __x = renameElementsInEquationItem(i.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(items)
}

fn renameElementsInEquationItem(
    mut item: metamodelica::Ref<Absyn::EquationItem>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::Ref<Absyn::EquationItem>> {
    let mut item: metamodelica::Ref<Absyn::EquationItem> = item;
    let () = (match &*item {
        Absyn::EquationItem::EQUATIONITEM {
            equation_: __item_equation_,
            ..
        } => {
            assign_variant_field!(item => Absyn::EquationItem::EQUATIONITEM;
                equation_ = renameElementsInEquation(__item_equation_.clone(), nameMap.clone())?,
                comment = renameElementsInCommentOpt(var_field!((*item).comment, Absyn::EquationItem::EQUATIONITEM).clone(), nameMap)?
            );
            ()
        }
        _ => (),
    });
    Ok(item)
}

fn renameElementsInEquation(
    mut eq: metamodelica::Ref<Absyn::Equation>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::Ref<Absyn::Equation>> {
    let mut eq: metamodelica::Ref<Absyn::Equation> = eq;
    let () = (match &*eq {
        Absyn::Equation::EQ_IF { ifExp: __eq_ifExp, .. } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_IF;
                        ifExp = AbsynUtil::traverseExp(__eq_ifExp.clone(), (std::sync::Arc::new(renameElementsInExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>)> + 'static>), nameMap.clone())?.0,
                        equationTrueItems = renameElementsInEquationItems(var_field!((*eq).equationTrueItems, Absyn::Equation::EQ_IF).clone(), nameMap.clone())?,
                        elseIfBranches = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>)> = metamodelica::nil();
                for mut b in (var_field!((*eq).elseIfBranches, Absyn::Equation::EQ_IF).clone()).into_iter().cloned() {
                    let __x = renameElementsInEquationBranch(b.clone(), nameMap.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        equationElseItems = renameElementsInEquationItems(var_field!((*eq).equationElseItems, Absyn::Equation::EQ_IF).clone(), nameMap)?
                    );
            ()
        }
        Absyn::Equation::EQ_EQUALS {
            leftSide: __eq_leftSide,
            ..
        } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_EQUALS;
                leftSide = AbsynUtil::traverseExp(__eq_leftSide.clone(), (std::sync::Arc::new(renameElementsInExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>)> + 'static>), nameMap.clone())?.0,
                rightSide = AbsynUtil::traverseExp(var_field!((*eq).rightSide, Absyn::Equation::EQ_EQUALS).clone(), (std::sync::Arc::new(renameElementsInExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>)> + 'static>), nameMap)?.0
            );
            ()
        }
        Absyn::Equation::EQ_PDE {
            leftSide: __eq_leftSide,
            ..
        } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_PDE;
                leftSide = AbsynUtil::traverseExp(__eq_leftSide.clone(), (std::sync::Arc::new(renameElementsInExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>)> + 'static>), nameMap.clone())?.0,
                rightSide = AbsynUtil::traverseExp(var_field!((*eq).rightSide, Absyn::Equation::EQ_PDE).clone(), (std::sync::Arc::new(renameElementsInExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>)> + 'static>), nameMap)?.0
            );
            ()
        }
        Absyn::Equation::EQ_CONNECT {
            connector1: __eq_connector1,
            ..
        } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_CONNECT;
                connector1 = renameElementsInCref(__eq_connector1.clone(), nameMap.clone(), false)?,
                connector2 = renameElementsInCref(var_field!((*eq).connector2, Absyn::Equation::EQ_CONNECT).clone(), nameMap, false)?
            );
            ()
        }
        Absyn::Equation::EQ_FOR {
            iterators: __eq_iterators,
            ..
        } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_FOR;
                        iterators = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>> = metamodelica::nil();
                for mut i in (__eq_iterators.clone()).into_iter().cloned() {
                    let __x = renameElementsInIterator(i.clone(), nameMap.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        forEquations = renameElementsInEquationItems(var_field!((*eq).forEquations, Absyn::Equation::EQ_FOR).clone(), nameMap)?
                    );
            ()
        }
        Absyn::Equation::EQ_WHEN_E {
            whenExp: __eq_whenExp, ..
        } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_WHEN_E;
                        whenExp = AbsynUtil::traverseExp(__eq_whenExp.clone(), (std::sync::Arc::new(renameElementsInExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>)> + 'static>), nameMap.clone())?.0,
                        whenEquations = renameElementsInEquationItems(var_field!((*eq).whenEquations, Absyn::Equation::EQ_WHEN_E).clone(), nameMap.clone())?,
                        elseWhenEquations = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>)> = metamodelica::nil();
                for mut b in (var_field!((*eq).elseWhenEquations, Absyn::Equation::EQ_WHEN_E).clone()).into_iter().cloned() {
                    let __x = renameElementsInEquationBranch(b.clone(), nameMap.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            ()
        }
        Absyn::Equation::EQ_NORETCALL {
            functionName: __eq_functionName,
            ..
        } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_NORETCALL;
                functionName = renameElementsInCref(__eq_functionName.clone(), nameMap.clone(), false)?,
                functionArgs = AbsynUtil::traverseExpBidirFunctionArgs(var_field!((*eq).functionArgs, Absyn::Equation::EQ_NORETCALL).clone(), (std::sync::Arc::new(renameElementsInExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>)> + 'static>), std::sync::Arc::new(fnptr!(AbsynUtil::dummyTraverseExp, metamodelica::Ref<Absyn::Exp>, _)), nameMap)?.0
            );
            ()
        }
        Absyn::Equation::EQ_FAILURE { equ: __eq_equ } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_FAILURE; equ = renameElementsInEquationItem(__eq_equ.clone(), nameMap)?);
            ()
        }
        _ => (),
    });
    Ok(eq)
}

fn renameElementsInEquationBranch(
    mut branch: (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    ),
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<(
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
)> {
    let mut branch: (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    ) = branch;
    let mut cond: metamodelica::Ref<Absyn::Exp>;
    let mut body: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    (cond, body) = branch;
    (cond, _) = AbsynUtil::traverseExp(
        cond,
        (std::sync::Arc::new(renameElementsInExp)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
                    ) -> Result<(
                        metamodelica::Ref<Absyn::Exp>,
                        metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
                    )> + 'static,
            >),
        nameMap.clone(),
    )?;
    body = renameElementsInEquationItems(body, nameMap)?;
    branch = (cond, body);
    Ok(branch)
}

fn renameElementsInIterator(
    mut iter: metamodelica::Ref<Absyn::ForIterator>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::Ref<Absyn::ForIterator>> {
    let mut iter: metamodelica::Ref<Absyn::ForIterator> = iter;
    if (iter.range).is_some() {
        assign_field!(
            iter.range = Some(
                (AbsynUtil::traverseExp(
                    iter.range.clone().ok_or("pattern mismatch")?,
                    (std::sync::Arc::new(renameElementsInExp)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Absyn::Exp>,
                                    metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
                                ) -> Result<(
                                    metamodelica::Ref<Absyn::Exp>,
                                    metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>
                                )> + 'static,
                        >),
                    nameMap
                )?)
                .0
            )
        );
    }
    Ok(iter)
}

fn renameElementsInAlgorithmItems(
    mut items: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>> {
    let mut items: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>> = items;
    items = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>> = metamodelica::nil();
        for mut i in (items).into_iter().cloned() {
            let __x = renameElementsInAlgorithmItem(i.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(items)
}

fn renameElementsInAlgorithmItem(
    mut item: metamodelica::Ref<Absyn::AlgorithmItem>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::Ref<Absyn::AlgorithmItem>> {
    let mut item: metamodelica::Ref<Absyn::AlgorithmItem> = item;
    let () = (match &*item {
        Absyn::AlgorithmItem::ALGORITHMITEM {
            algorithm_: __item_algorithm_,
            ..
        } => {
            assign_variant_field!(item => Absyn::AlgorithmItem::ALGORITHMITEM;
                algorithm_ = renameElementsInAlgorithm(__item_algorithm_.clone(), nameMap.clone())?,
                comment = renameElementsInCommentOpt(var_field!((*item).comment, Absyn::AlgorithmItem::ALGORITHMITEM).clone(), nameMap)?
            );
            ()
        }
        _ => (),
    });
    Ok(item)
}

fn renameElementsInAlgorithm(
    mut alg: metamodelica::Ref<Absyn::Algorithm>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::Ref<Absyn::Algorithm>> {
    let mut alg: metamodelica::Ref<Absyn::Algorithm> = alg;
    let () = (match &*alg {
        Absyn::Algorithm::ALG_ASSIGN {
            assignComponent: __alg_assignComponent,
            ..
        } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_ASSIGN;
                assignComponent = AbsynUtil::traverseExp(__alg_assignComponent.clone(), (std::sync::Arc::new(renameElementsInExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>)> + 'static>), nameMap.clone())?.0,
                value = AbsynUtil::traverseExp(var_field!((*alg).value, Absyn::Algorithm::ALG_ASSIGN).clone(), (std::sync::Arc::new(renameElementsInExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>)> + 'static>), nameMap)?.0
            );
            ()
        }
        Absyn::Algorithm::ALG_IF { ifExp: __alg_ifExp, .. } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_IF;
                        ifExp = AbsynUtil::traverseExp(__alg_ifExp.clone(), (std::sync::Arc::new(renameElementsInExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>)> + 'static>), nameMap.clone())?.0,
                        trueBranch = renameElementsInAlgorithmItems(var_field!((*alg).trueBranch, Absyn::Algorithm::ALG_IF).clone(), nameMap.clone())?,
                        elseIfAlgorithmBranch = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>)> = metamodelica::nil();
                for mut b in (var_field!((*alg).elseIfAlgorithmBranch, Absyn::Algorithm::ALG_IF).clone()).into_iter().cloned() {
                    let __x = renameElementsInAlgorithmBranch(b.clone(), nameMap.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        elseBranch = renameElementsInAlgorithmItems(var_field!((*alg).elseBranch, Absyn::Algorithm::ALG_IF).clone(), nameMap)?
                    );
            ()
        }
        Absyn::Algorithm::ALG_FOR {
            iterators: __alg_iterators,
            ..
        } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_FOR;
                        iterators = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>> = metamodelica::nil();
                for mut i in (__alg_iterators.clone()).into_iter().cloned() {
                    let __x = renameElementsInIterator(i.clone(), nameMap.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        forBody = renameElementsInAlgorithmItems(var_field!((*alg).forBody, Absyn::Algorithm::ALG_FOR).clone(), nameMap)?
                    );
            ()
        }
        Absyn::Algorithm::ALG_PARFOR {
            iterators: __alg_iterators,
            ..
        } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_PARFOR;
                        iterators = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>> = metamodelica::nil();
                for mut i in (__alg_iterators.clone()).into_iter().cloned() {
                    let __x = renameElementsInIterator(i.clone(), nameMap.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        parforBody = renameElementsInAlgorithmItems(var_field!((*alg).parforBody, Absyn::Algorithm::ALG_PARFOR).clone(), nameMap)?
                    );
            ()
        }
        Absyn::Algorithm::ALG_WHILE {
            boolExpr: __alg_boolExpr,
            ..
        } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_WHILE;
                boolExpr = AbsynUtil::traverseExp(__alg_boolExpr.clone(), (std::sync::Arc::new(renameElementsInExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>)> + 'static>), nameMap.clone())?.0,
                whileBody = renameElementsInAlgorithmItems(var_field!((*alg).whileBody, Absyn::Algorithm::ALG_WHILE).clone(), nameMap)?
            );
            ()
        }
        Absyn::Algorithm::ALG_WHEN_A {
            boolExpr: __alg_boolExpr,
            ..
        } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_WHEN_A;
                        boolExpr = AbsynUtil::traverseExp(__alg_boolExpr.clone(), (std::sync::Arc::new(renameElementsInExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>)> + 'static>), nameMap.clone())?.0,
                        whenBody = renameElementsInAlgorithmItems(var_field!((*alg).whenBody, Absyn::Algorithm::ALG_WHEN_A).clone(), nameMap.clone())?,
                        elseWhenAlgorithmBranch = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>)> = metamodelica::nil();
                for mut b in (var_field!((*alg).elseWhenAlgorithmBranch, Absyn::Algorithm::ALG_WHEN_A).clone()).into_iter().cloned() {
                    let __x = renameElementsInAlgorithmBranch(b.clone(), nameMap.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            ()
        }
        Absyn::Algorithm::ALG_NORETCALL {
            functionCall: __alg_functionCall,
            ..
        } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_NORETCALL;
                functionCall = renameElementsInCref(__alg_functionCall.clone(), nameMap.clone(), false)?,
                functionArgs = AbsynUtil::traverseExpBidirFunctionArgs(var_field!((*alg).functionArgs, Absyn::Algorithm::ALG_NORETCALL).clone(), (std::sync::Arc::new(renameElementsInExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>)> + 'static>), std::sync::Arc::new(fnptr!(AbsynUtil::dummyTraverseExp, metamodelica::Ref<Absyn::Exp>, _)), nameMap)?.0
            );
            ()
        }
        _ => (),
    });
    Ok(alg)
}

fn renameElementsInAlgorithmBranch(
    mut branch: (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    ),
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<(
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
)> {
    let mut branch: (
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    ) = branch;
    let mut cond: metamodelica::Ref<Absyn::Exp>;
    let mut body: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
    (cond, body) = branch;
    (cond, _) = AbsynUtil::traverseExp(
        cond,
        (std::sync::Arc::new(renameElementsInExp)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Absyn::Exp>,
                        metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
                    ) -> Result<(
                        metamodelica::Ref<Absyn::Exp>,
                        metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
                    )> + 'static,
            >),
        nameMap.clone(),
    )?;
    body = renameElementsInAlgorithmItems(body, nameMap)?;
    branch = (cond, body);
    Ok(branch)
}

fn renameElementsInElementArg(
    mut arg: metamodelica::Ref<Absyn::ElementArg>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
    let mut arg: metamodelica::Ref<Absyn::ElementArg> = arg;
    let () = (match &*arg {
        Absyn::ElementArg::MODIFICATION {
            modification: __arg_modification,
            ..
        } => {
            assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; modification = renameElementsInModificationOpt(__arg_modification.clone(), nameMap)?);
            ()
        }
        Absyn::ElementArg::REDECLARATION {
            elementSpec: __arg_elementSpec,
            ..
        } => {
            assign_variant_field!(arg => Absyn::ElementArg::REDECLARATION;
                elementSpec = renameElementsInElementSpec(__arg_elementSpec.clone(), nameMap.clone(), false)?,
                constrainClass = renameElementsInConstrainClassOpt(var_field!((*arg).constrainClass, Absyn::ElementArg::REDECLARATION).clone(), nameMap)?
            );
            ()
        }
        _ => (),
    });
    Ok(arg)
}

fn renameElementsInConstrainClassOpt(
    mut cc: Option<metamodelica::Ref<Absyn::ConstrainClass>>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<Option<metamodelica::Ref<Absyn::ConstrainClass>>> {
    let mut cc: Option<metamodelica::Ref<Absyn::ConstrainClass>> = cc;
    cc = Util::applyOption(
        cc,
        &({
            let __pe_b1 = nameMap;
            move |__pe_a0| renameElementsInConstrainClass(__pe_a0, __pe_b1.clone())
        }),
    )?;
    Ok(cc)
}

fn renameElementsInConstrainClass(
    mut cc: metamodelica::Ref<Absyn::ConstrainClass>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::Ref<Absyn::ConstrainClass>> {
    let mut cc: metamodelica::Ref<Absyn::ConstrainClass> = cc;
    assign_field!(
        cc.elementSpec = renameElementsInElementSpec(cc.elementSpec.clone(), nameMap.clone(), true)?,
        cc.comment = renameElementsInCommentOpt(cc.comment.clone(), nameMap)?
    );
    Ok(cc)
}

fn renameElementsInCommentOpt(
    mut comment: Option<metamodelica::Ref<Absyn::Comment>>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<Option<metamodelica::Ref<Absyn::Comment>>> {
    let mut comment: Option<metamodelica::Ref<Absyn::Comment>> = comment;
    comment = Util::applyOption(
        comment,
        &({
            let __pe_b1 = nameMap;
            move |__pe_a0| renameElementsInComment(__pe_a0, __pe_b1.clone())
        }),
    )?;
    Ok(comment)
}

fn renameElementsInComment(
    mut comment: metamodelica::Ref<Absyn::Comment>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::Ref<Absyn::Comment>> {
    let mut comment: metamodelica::Ref<Absyn::Comment> = comment;
    assign_field!(
        comment.annotation_ = Util::applyOption(
            comment.annotation_.clone(),
            &({
                let __pe_b1 = nameMap;
                move |__pe_a0| renameElementsInAnnotation(__pe_a0, __pe_b1.clone())
            })
        )?
    );
    Ok(comment)
}

fn renameElementsInAnnotationOpt(
    mut ann: Option<metamodelica::Ref<Absyn::Annotation>>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<Option<metamodelica::Ref<Absyn::Annotation>>> {
    let mut ann: Option<metamodelica::Ref<Absyn::Annotation>> = ann;
    ann = Util::applyOption(
        ann,
        &({
            let __pe_b1 = nameMap;
            move |__pe_a0| renameElementsInAnnotation(__pe_a0, __pe_b1.clone())
        }),
    )?;
    Ok(ann)
}

fn renameElementsInAnnotation(
    mut ann: metamodelica::Ref<Absyn::Annotation>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::Ref<Absyn::Annotation>> {
    let mut ann: metamodelica::Ref<Absyn::Annotation> = ann;
    assign_field!(
        ann.elementArgs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
            for mut a in (ann.elementArgs.clone()).into_iter().cloned() {
                let __x = renameElementsInElementArg(a.clone(), nameMap.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    Ok(ann)
}

fn renameElementsInModificationOpt(
    mut r#mod: Option<metamodelica::Ref<Absyn::Modification>>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<Option<metamodelica::Ref<Absyn::Modification>>> {
    let mut r#mod: Option<metamodelica::Ref<Absyn::Modification>> = r#mod;
    r#mod = Util::applyOption(
        r#mod,
        &({
            let __pe_b1 = nameMap;
            move |__pe_a0| renameElementsInModification(__pe_a0, __pe_b1.clone())
        }),
    )?;
    Ok(r#mod)
}

fn renameElementsInModification(
    mut r#mod: metamodelica::Ref<Absyn::Modification>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::Ref<Absyn::Modification>> {
    let mut r#mod: metamodelica::Ref<Absyn::Modification> = r#mod;
    assign_field!(
        r#mod.elementArgLst = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>> = metamodelica::nil();
            for mut a in (r#mod.elementArgLst.clone()).into_iter().cloned() {
                let __x = renameElementsInElementArg(a.clone(), nameMap.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        r#mod.eqMod = renameElementsInEqMod(r#mod.eqMod.clone(), nameMap)?
    );
    Ok(r#mod)
}

fn renameElementsInEqMod(
    mut eqMod: metamodelica::Ref<Absyn::EqMod>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::Ref<Absyn::EqMod>> {
    let mut eqMod: metamodelica::Ref<Absyn::EqMod> = eqMod;
    let () = (match &*eqMod {
        Absyn::EqMod::EQMOD { exp: __eqMod_exp, .. } => {
            assign_variant_field!(eqMod => Absyn::EqMod::EQMOD; exp = AbsynUtil::traverseExp(__eqMod_exp.clone(), (std::sync::Arc::new(renameElementsInExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>)> + 'static>), nameMap)?.0);
            ()
        }
        _ => (),
    });
    Ok(eqMod)
}

fn renameElementsInExp(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<(
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
)> {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    let mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>> = nameMap;
    let () = (match &*exp {
        Absyn::Exp::CREF {
            componentRef: __exp_componentRef,
        } => {
            assign_variant_field!(exp => Absyn::Exp::CREF; componentRef = renameElementsInCref(__exp_componentRef.clone(), nameMap.clone(), false)?);
            ()
        }
        Absyn::Exp::CALL {
            function_: __exp_function_,
            ..
        } => {
            assign_variant_field!(exp => Absyn::Exp::CALL; function_ = renameElementsInCref(__exp_function_.clone(), nameMap.clone(), false)?);
            ()
        }
        Absyn::Exp::PARTEVALFUNCTION {
            function_: __exp_function_,
            ..
        } => {
            assign_variant_field!(exp => Absyn::Exp::PARTEVALFUNCTION; function_ = renameElementsInCref(__exp_function_.clone(), nameMap.clone(), false)?);
            ()
        }
        _ => (),
    });
    Ok((exp, nameMap))
}

fn renameElementsInCref(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
    mut onlySubs: bool,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut cref: metamodelica::Ref<Absyn::ComponentRef> = cref;
    let () = (match &*cref {
        Absyn::ComponentRef::CREF_QUAL { .. } => {
            if !(onlySubs) {
                assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL; name = renameElementsInIdent(var_field!((*cref).name, Absyn::ComponentRef::CREF_QUAL).clone(), nameMap.clone())?);
            }
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL;
                        subscripts = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Subscript>> = metamodelica::nil();
                for mut s in (var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_QUAL).clone()).into_iter().cloned() {
                    let __x = renameElementsInSubscript(s.clone(), nameMap.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        componentRef = renameElementsInCref(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone(), nameMap, true)?
                    );
            ()
        }
        Absyn::ComponentRef::CREF_IDENT { .. } => {
            if !(onlySubs) {
                assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; name = renameElementsInIdent(var_field!((*cref).name, Absyn::ComponentRef::CREF_IDENT).clone(), nameMap.clone())?);
            }
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; subscripts = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Subscript>> = metamodelica::nil();
                for mut s in (var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone()).into_iter().cloned() {
                    let __x = renameElementsInSubscript(s.clone(), nameMap.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        _ => (),
    });
    Ok(cref)
}

fn renameElementsInPath(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::Ref<Absyn::Path>> {
    let mut path: metamodelica::Ref<Absyn::Path> = path;
    let () = (match &*path {
        Absyn::Path::QUALIFIED { name: __path_name, .. } => {
            assign_variant_field!(path => Absyn::Path::QUALIFIED; name = renameElementsInIdent(__path_name.clone(), nameMap)?);
            ()
        }
        Absyn::Path::IDENT { name: __path_name } => {
            assign_variant_field!(path => Absyn::Path::IDENT; name = renameElementsInIdent(__path_name.clone(), nameMap)?);
            ()
        }
        _ => (),
    });
    Ok(path)
}

fn renameElementsInIdent(
    mut ident: ArcStr,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<ArcStr> {
    let mut ident: ArcStr = ident;
    ident = UnorderedMap::getOrDefault(ident.clone(), nameMap, ident)?;
    Ok(ident)
}

fn renameElementsInSubscripts(
    mut subs: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Subscript>>> {
    let mut subs: metamodelica::List<metamodelica::Ref<Absyn::Subscript>> = subs;
    subs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Subscript>> = metamodelica::nil();
        for mut s in (subs).into_iter().cloned() {
            let __x = renameElementsInSubscript(s.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(subs)
}

fn renameElementsInSubscript(
    mut sub: metamodelica::Ref<Absyn::Subscript>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::Ref<Absyn::Subscript>> {
    let mut sub: metamodelica::Ref<Absyn::Subscript> = sub;
    let () = (match &*sub {
        Absyn::Subscript::SUBSCRIPT {
            subscript: __sub_subscript,
        } => {
            assign_variant_field!(sub => Absyn::Subscript::SUBSCRIPT; subscript = AbsynUtil::traverseExp(__sub_subscript.clone(), (std::sync::Arc::new(renameElementsInExp) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<(metamodelica::Ref<Absyn::Exp>, metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>)> + 'static>), nameMap)?.0);
            ()
        }
        _ => (),
    });
    Ok(sub)
}

fn renameElementsInExternalDecl(
    mut extDecl: metamodelica::Ref<Absyn::ExternalDecl>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::Ref<Absyn::ExternalDecl>> {
    let mut extDecl: metamodelica::Ref<Absyn::ExternalDecl> = extDecl;
    assign_field!(
        extDecl.args = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
            for mut a in (extDecl.args.clone()).into_iter().cloned() {
                let __x = (renameElementsInExp(a.clone(), nameMap.clone())?).0;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        extDecl.annotation_ = renameElementsInAnnotationOpt(extDecl.annotation_.clone(), nameMap)?
    );
    Ok(extDecl)
}

fn renameElementsInTypeSpec(
    mut spec: metamodelica::Ref<Absyn::TypeSpec>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<metamodelica::Ref<Absyn::TypeSpec>> {
    let mut spec: metamodelica::Ref<Absyn::TypeSpec> = spec;
    let () = (match &*spec {
        Absyn::TypeSpec::TPATH { path: __spec_path, .. } => {
            assign_variant_field!(spec => Absyn::TypeSpec::TPATH;
                path = renameElementsInPath(__spec_path.clone(), nameMap.clone())?,
                arrayDim = Util::applyOption(var_field!((*spec).arrayDim, Absyn::TypeSpec::TPATH).clone(), &({ let __pe_b1 = nameMap; move |__pe_a0| renameElementsInSubscripts(__pe_a0, __pe_b1.clone()) }))?
            );
            ()
        }
        Absyn::TypeSpec::TCOMPLEX { path: __spec_path, .. } => {
            assign_variant_field!(spec => Absyn::TypeSpec::TCOMPLEX;
                path = renameElementsInPath(__spec_path.clone(), nameMap.clone())?,
                arrayDim = Util::applyOption(var_field!((*spec).arrayDim, Absyn::TypeSpec::TCOMPLEX).clone(), &({ let __pe_b1 = nameMap; move |__pe_a0| renameElementsInSubscripts(__pe_a0, __pe_b1.clone()) }))?
            );
            ()
        }
    });
    Ok(spec)
}

fn renameElementsInAttributes(
    mut attrs: Absyn::ElementAttributes,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
) -> Result<Absyn::ElementAttributes> {
    let mut attrs: Absyn::ElementAttributes = attrs;
    attrs.arrayDim = renameElementsInSubscripts(attrs.arrayDim.clone(), nameMap)?;
    Ok(attrs)
}

fn renameElementsInComponentItem(
    mut component: metamodelica::Ref<Absyn::ComponentItem>,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
    mut renameElement: bool,
) -> Result<metamodelica::Ref<Absyn::ComponentItem>> {
    let mut component: metamodelica::Ref<Absyn::ComponentItem> = component;
    let mut exp: metamodelica::Ref<Absyn::Exp>;
    assign_field!(
        component.component = renameElementsInComponent(component.component.clone(), nameMap.clone(), renameElement)?
    );
    if (component.condition).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(component.condition.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        exp = metamodelica::Own::own(__pa0);
        (exp, _) = AbsynUtil::traverseExp(
            exp,
            (std::sync::Arc::new(renameElementsInExp)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Absyn::Exp>,
                            metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
                        ) -> Result<(
                            metamodelica::Ref<Absyn::Exp>,
                            metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
                        )> + 'static,
                >),
            nameMap.clone(),
        )?;
        assign_field!(component.condition = Some(exp));
    }
    assign_field!(component.comment = renameElementsInCommentOpt(component.comment.clone(), nameMap)?);
    Ok(component)
}

fn renameElementsInComponent(
    mut component: Absyn::Component,
    mut nameMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>,
    mut renameElement: bool,
) -> Result<Absyn::Component> {
    let mut component: Absyn::Component = component;
    if renameElement {
        component.name = renameElementsInIdent(component.name.clone(), nameMap.clone())?;
    }
    component.arrayDim = renameElementsInSubscripts(component.arrayDim.clone(), nameMap.clone())?;
    component.modification = renameElementsInModificationOpt(component.modification.clone(), nameMap)?;
    Ok(component)
}

pub(crate) fn getInheritedAnnotation(
    mut modelPath: metamodelica::Ref<Absyn::Path>,
    mut annotationName: ArcStr,
    mut program: Absyn::Program,
    mut printConflictWarning: bool,
) -> Result<Option<metamodelica::Ref<Absyn::Modification>>> {
    let mut outAnnotation: Option<metamodelica::Ref<Absyn::Modification>> = None;
    let mut cls: metamodelica::Ref<Absyn::Class>;
    let mut extends_paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut extends_oannl: metamodelica::List<Option<metamodelica::Ref<Absyn::Modification>>>;
    let mut extends_ann: metamodelica::Ref<Absyn::Modification>;
    let mut extends_ann2: metamodelica::Ref<Absyn::Modification>;
    let mut extends_path: metamodelica::Ref<Absyn::Path>;
    cls = ProgramUtil::getPathedClassInProgram(modelPath.clone(), &program, false, false)?;
    outAnnotation = AbsynUtil::lookupClassAnnotation(&cls, &annotationName)?;
    ErrorExt::setCheckpoint(literal!("InteractiveUtil.getInheritedAnnotation"));
    match '__try0: {
        extends_paths = unwrap_break_err!(NFApi::getInheritedClasses(modelPath.clone(), program.clone()), '__try0);
        Ok::<_, &'static str>((extends_paths.clone(),))
    } {
        Ok((__try0_o0,)) => {
            extends_paths = __try0_o0;
        }
        Err(_) => {
            extends_paths = metamodelica::nil();
        }
    }
    ErrorExt::rollBack(literal!("InteractiveUtil.getInheritedAnnotation"));
    if (extends_paths).is_empty() {
        return Ok(outAnnotation);
    }
    extends_oannl = ({
        let mut __acc: metamodelica::List<Option<metamodelica::Ref<Absyn::Modification>>> = metamodelica::nil();
        for mut ep in (extends_paths.clone()).into_iter().cloned() {
            let __x = getInheritedAnnotation(ep.clone(), annotationName.clone(), program.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    while !((extends_oannl).is_empty()) {
        if ((extends_oannl).head().cloned()?).is_some() {
            extends_ann = ((extends_oannl).head().cloned()?).ok_or("pattern mismatch")?;
            if (outAnnotation).is_some() {
                outAnnotation = Some(AbsynUtil::mergeModifiers(
                    &(outAnnotation.ok_or("pattern mismatch")?),
                    &extends_ann,
                )?);
            } else {
                outAnnotation = Some(extends_ann.clone());
            }
            if printConflictWarning {
                let (__pa1, __pa2) = ::match_deref::match_deref! { match &(extends_paths) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                extends_path = metamodelica::Own::own(__pa1);
                extends_paths = metamodelica::Own::own(__pa2);
                for mut a in &*(extends_oannl).rest()? {
                    if (a).is_some() {
                        let __pa3 = ::match_deref::match_deref! { match &(a.clone()) {
                            Some(__pa3) => __pa3.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        extends_ann2 = metamodelica::Own::own(__pa3);
                        if !(extends_ann.clone() == extends_ann2.clone()) {
                            Error::addMessage(
                                Error::CONFLICTING_INHERITED_ANNOTATIONS.clone(),
                                list![
                                    annotationName,
                                    AbsynUtil::pathString(modelPath, literal!("."), true, false)?,
                                    Dump::unparseModificationStr(extends_ann)?,
                                    AbsynUtil::pathString(extends_path, literal!("."), true, false)?,
                                    Dump::unparseModificationStr(extends_ann2)?,
                                    AbsynUtil::pathString(
                                        (extends_paths).head().cloned()?,
                                        literal!("."),
                                        true,
                                        false
                                    )?
                                ],
                            )?;
                            break;
                        }
                        extends_paths = (extends_paths).rest()?;
                    }
                }
            }
            return Ok(outAnnotation);
        }
        extends_oannl = (extends_oannl).rest()?;
        extends_paths = (extends_paths).rest()?;
    }
    Ok(outAnnotation)
}

pub(crate) fn setElementType(
    mut elementPath: &metamodelica::Ref<Absyn::Path>,
    mut className: metamodelica::Ref<Absyn::ComponentRef>,
    mut program: Absyn::Program,
) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut success: bool = true;
    let mut elem_opt: Option<metamodelica::Ref<Absyn::Element>>;
    let mut ty: metamodelica::Ref<Absyn::TypeSpec>;
    match '__try0: {
        ty = unwrap_break_err!(AbsynUtil::crefToTypeSpec(className.clone()), '__try0);
        (program, elem_opt, success) = unwrap_break_err!(transformPathedElementInProgram(elementPath, (std::sync::Arc::new({ let __pe_b1 = ty.clone(); let __pe_b2 = false; move |__pe_a0| AbsynUtil::setElementType(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Element>) -> Result<metamodelica::Ref<Absyn::Element>> + 'static>), program.clone()), '__try0);
        if success {
            unwrap_break_err!(SymbolTable::setAbsynElement(program.clone(), &(unwrap_break_err!(elem_opt.clone().ok_or("pattern mismatch"), '__try0)), elementPath), '__try0);
        }
        Ok::<_, &'static str>((success.clone(),))
    } {
        Ok((__try0_o0,)) => {
            success = __try0_o0;
        }
        Err(_) => {
            success = false;
        }
    }
    (program, success)
}

pub(crate) fn makeCommentFromArgs(
    mut commentExp: &metamodelica::Ref<Absyn::Exp>,
    mut annotationExp: &metamodelica::Ref<Absyn::Exp>,
    mut oldComment: Option<metamodelica::Ref<Absyn::Comment>>,
) -> Result<Option<metamodelica::Ref<Absyn::Comment>>> {
    let mut comment: Option<metamodelica::Ref<Absyn::Comment>>;
    let mut ann: Option<metamodelica::Ref<Absyn::Annotation>>;
    let mut cmt: Option<ArcStr>;
    cmt = (::match_deref::match_deref! { match commentExp {
        Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::ListNode::Nil } => None,
        Deref @ Absyn::Exp::STRING { value: __commentExp_value } => Some(__commentExp_value.clone()),
        _ => return Err("match: no arm matched"),
    } });
    ann = (::match_deref::match_deref! { match annotationExp {
        Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::ListNode::Nil } => None,
        _ => Some(metamodelica::Ref::new(Absyn::Annotation { elementArgs: list![recordConstructorToModification(annotationExp)?] })),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if (cmt).is_some() || (ann).is_some() {
        cmt = if ((cmt).is_some()) {
            cmt
        } else {
            AbsynUtil::getCommentOptComment(oldComment.clone())?
        };
        ann = if ((ann).is_some()) {
            ann
        } else {
            AbsynUtil::getCommentOptAnnotation(oldComment)?
        };
        comment = Some(metamodelica::Ref::new(Absyn::Comment {
            annotation_: ann,
            comment: cmt,
        }));
    } else {
        comment = oldComment;
    }
    Ok(comment)
}

pub(crate) fn makeModifierFromArgs(
    mut bindingExp: metamodelica::Ref<Absyn::Exp>,
    mut modifier: metamodelica::Ref<Absyn::Modification>,
    mut info: SourceInfo,
    mut oldModifier: Option<metamodelica::Ref<Absyn::Modification>>,
) -> Result<Option<metamodelica::Ref<Absyn::Modification>>> {
    let mut outModifier: Option<metamodelica::Ref<Absyn::Modification>>;
    outModifier = (::match_deref::match_deref! { match &((bindingExp.clone(), modifier.clone())) {
        (Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::ListNode::Nil }, Deref @ Absyn::Modification { elementArgLst: Deref @ metamodelica::ListNode::Nil, .. }) => oldModifier,
        (Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::ListNode::Nil }, _) => Some(modifier),
        (_, Deref @ Absyn::Modification { .. }) => Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: modifier.elementArgLst.clone(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: bindingExp, info: info }) })),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outModifier)
}

pub(crate) fn accessClass(
    mut classPath: metamodelica::Ref<Absyn::Path>,
    mut program: Absyn::Program,
    mut r#fn: &dyn ::std::ops::Fn(
        metamodelica::Ref<Absyn::Path>,
        Absyn::Program,
        Access,
    ) -> Result<metamodelica::Ref<Values::Value>>,
    mut evaluateParams: bool,
    mut graphicsExpMode: bool,
    mut accessLevel: Access,
) -> Result<metamodelica::Ref<Values::Value>> {
    pub type Fn = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Absyn::Path>,
                Absyn::Program,
                Access,
            ) -> Result<metamodelica::Ref<Values::Value>>
            + 'static,
    >;

    let mut result: metamodelica::Ref<Values::Value>;
    let mut access: Access;
    let mut silent: bool = false;
    let mut eval_params: bool;
    let mut graphics_exp_mode: bool;
    eval_params = Config::getEvaluateParametersInAnnotations()?;
    graphics_exp_mode = Config::getGraphicsExpMode()?;
    match '__try0: {
        access = Interactive::checkAccessAnnotationAndEncryption(classPath.clone(), program.clone());
        if access < accessLevel {
            unwrap_break_err!(Error::addMessage(Error::ACCESS_ENCRYPTED_PROTECTED_CONTENTS.clone(), metamodelica::nil()), '__try0);
            result = ValuesMake::makeBoolean(false);
            return Ok(result);
        }
        silent = !(unwrap_break_err!(Flags::isSet(Flags::NF_API_NOISE.clone()), '__try0));
        if silent {
            ErrorExt::setCheckpoint(literal!("InteractiveUtil.accessClass"));
        }
        unwrap_break_err!(Config::setEvaluateParametersInAnnotations(evaluateParams), '__try0);
        unwrap_break_err!(Config::setGraphicsExpMode(graphicsExpMode), '__try0);
        result = unwrap_break_err!(r#fn(classPath.clone(), program.clone(), access), '__try0);
        Ok::<_, &'static str>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeBoolean(false);
        }
    }
    if silent {
        ErrorExt::rollBack(literal!("InteractiveUtil.accessClass"));
    }
    Config::setGraphicsExpMode(graphics_exp_mode)?;
    Config::setEvaluateParametersInAnnotations(eval_params)?;
    Ok(result)
}

pub(crate) fn makeAnnotationArrayValue(
    mut annotations: metamodelica::List<ArcStr>,
) -> metamodelica::Ref<Values::Value> {
    let mut arr: metamodelica::Ref<Values::Value>;
    arr = ValuesMake::makeArray(
        ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
            for mut s in (annotations).into_iter().cloned() {
                let __x = ValuesMake::makeCodeTypeNameStr(s.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    );
    arr
}

pub(crate) fn parseWithinPath(mut path: metamodelica::Ref<Absyn::Path>) -> Absyn::Within {
    let mut outWithin: Absyn::Within;
    outWithin = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "__OpenModelica_TopLevel" } => openmodelica_ast::Absyn::Within::TOP,
        _ => Absyn::Within::WITHIN { path: path },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outWithin
}

pub(crate) fn offsetAnnotationsInClassDef(
    mut cdef: metamodelica::Ref<Absyn::ClassDef>,
    mut x: i32,
    mut y: i32,
) -> Result<metamodelica::Ref<Absyn::ClassDef>> {
    let mut cdef: metamodelica::Ref<Absyn::ClassDef> = cdef;
    if x == 0 && y == 0 {
        return Ok(cdef);
    }
    let () = (match &*cdef {
        Absyn::ClassDef::PARTS {
            classParts: __cdef_classParts,
            ..
        } => {
            assign_variant_field!(cdef => Absyn::ClassDef::PARTS;
                        classParts = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
                for mut p in (__cdef_classParts.clone()).into_iter().cloned() {
                    let __x = offsetAnnotationsInClassPart(p.clone(), x, y)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        ann = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Annotation>> = metamodelica::nil();
                for mut a in (var_field!((*cdef).ann, Absyn::ClassDef::PARTS).clone()).into_iter().cloned() {
                    let __x = offsetDiagramAnnotation(a.clone(), x, y)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            ()
        }
        Absyn::ClassDef::DERIVED {
            comment: __cdef_comment,
            ..
        } => {
            assign_variant_field!(cdef => Absyn::ClassDef::DERIVED; comment = offsetDiagramAnnotationInOptComment(__cdef_comment.clone(), x, y)?);
            ()
        }
        Absyn::ClassDef::ENUMERATION {
            comment: __cdef_comment,
            ..
        } => {
            assign_variant_field!(cdef => Absyn::ClassDef::ENUMERATION; comment = offsetDiagramAnnotationInOptComment(__cdef_comment.clone(), x, y)?);
            ()
        }
        Absyn::ClassDef::OVERLOAD {
            comment: __cdef_comment,
            ..
        } => {
            assign_variant_field!(cdef => Absyn::ClassDef::OVERLOAD; comment = offsetDiagramAnnotationInOptComment(__cdef_comment.clone(), x, y)?);
            ()
        }
        Absyn::ClassDef::CLASS_EXTENDS {
            parts: __cdef_parts, ..
        } => {
            assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS;
                        parts = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>> = metamodelica::nil();
                for mut p in (__cdef_parts.clone()).into_iter().cloned() {
                    let __x = offsetAnnotationsInClassPart(p.clone(), x, y)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
                        ann = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Annotation>> = metamodelica::nil();
                for mut a in (var_field!((*cdef).ann, Absyn::ClassDef::CLASS_EXTENDS).clone()).into_iter().cloned() {
                    let __x = offsetDiagramAnnotation(a.clone(), x, y)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
                    );
            ()
        }
        Absyn::ClassDef::PDER {
            comment: __cdef_comment,
            ..
        } => {
            assign_variant_field!(cdef => Absyn::ClassDef::PDER; comment = offsetDiagramAnnotationInOptComment(__cdef_comment.clone(), x, y)?);
            ()
        }
    });
    Ok(cdef)
}

pub(crate) fn offsetAnnotationsInClassPart(
    mut part: metamodelica::Ref<Absyn::ClassPart>,
    mut x: i32,
    mut y: i32,
) -> Result<metamodelica::Ref<Absyn::ClassPart>> {
    let mut part: metamodelica::Ref<Absyn::ClassPart> = part;
    let () = (match &*part {
        Absyn::ClassPart::PUBLIC {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::PUBLIC; contents = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = metamodelica::nil();
                for mut i in (__part_contents.clone()).into_iter().cloned() {
                    let __x = offsetAnnotationsInElementItem(i.clone(), x, y)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        Absyn::ClassPart::PROTECTED {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::PROTECTED; contents = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = metamodelica::nil();
                for mut i in (__part_contents.clone()).into_iter().cloned() {
                    let __x = offsetAnnotationsInElementItem(i.clone(), x, y)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        Absyn::ClassPart::EQUATIONS {
            contents: __part_contents,
        } => {
            assign_variant_field!(part => Absyn::ClassPart::EQUATIONS; contents = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>> = metamodelica::nil();
                for mut e in (__part_contents.clone()).into_iter().cloned() {
                    let __x = offsetAnnotationsInEquationItem(e.clone(), x, y);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        _ => (),
    });
    Ok(part)
}

pub(crate) fn offsetAnnotationsInElementItem(
    mut item: metamodelica::Ref<Absyn::ElementItem>,
    mut x: i32,
    mut y: i32,
) -> Result<metamodelica::Ref<Absyn::ElementItem>> {
    let mut item: metamodelica::Ref<Absyn::ElementItem> = item;
    let () = (match &*item {
        Absyn::ElementItem::ELEMENTITEM {
            element: __item_element,
        } => {
            assign_variant_field!(item => Absyn::ElementItem::ELEMENTITEM; element = offsetAnnotationsInElement(__item_element.clone(), x, y)?);
            ()
        }
        _ => (),
    });
    Ok(item)
}

pub(crate) fn offsetAnnotationsInElement(
    mut element: metamodelica::Ref<Absyn::Element>,
    mut x: i32,
    mut y: i32,
) -> Result<metamodelica::Ref<Absyn::Element>> {
    let mut element: metamodelica::Ref<Absyn::Element> = element;
    let () = (match &*element {
        Absyn::Element::ELEMENT {
            specification: __element_specification,
            ..
        } => {
            assign_variant_field!(element => Absyn::Element::ELEMENT; specification = offsetAnnotationsInElementSpec(__element_specification.clone(), x, y)?);
            ()
        }
        _ => (),
    });
    Ok(element)
}

pub(crate) fn offsetAnnotationsInElementSpec(
    mut spec: metamodelica::Ref<Absyn::ElementSpec>,
    mut x: i32,
    mut y: i32,
) -> Result<metamodelica::Ref<Absyn::ElementSpec>> {
    let mut spec: metamodelica::Ref<Absyn::ElementSpec> = spec;
    let () = (match &*spec {
        Absyn::ElementSpec::COMPONENTS {
            components: __spec_components,
            ..
        } => {
            assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; components = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>> = metamodelica::nil();
                for mut c in (__spec_components.clone()).into_iter().cloned() {
                    let __x = offsetAnnotationsInComponentItem(c.clone(), x, y)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        _ => (),
    });
    Ok(spec)
}

pub(crate) static PLACEMENT_ORIGIN_PATH: std::sync::LazyLock<metamodelica::Ref<Absyn::Path>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(Absyn::Path::QUALIFIED {
            name: literal!("Placement"),
            path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                name: literal!("transformation"),
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("origin"),
                }),
            }),
        })
    });

pub(crate) static PLACEMENT_ICON_TRANSFORMATION_PATH: std::sync::LazyLock<metamodelica::Ref<Absyn::Path>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(Absyn::Path::QUALIFIED {
            name: literal!("Placement"),
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("iconTransformation"),
            }),
        })
    });

pub(crate) static LINE_POINTS_PATH: std::sync::LazyLock<metamodelica::Ref<Absyn::Path>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(Absyn::Path::QUALIFIED {
            name: literal!("Line"),
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("points"),
            }),
        })
    });

pub(crate) static DIAGRAM_GRAPHICS_PATH: std::sync::LazyLock<metamodelica::Ref<Absyn::Path>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(Absyn::Path::QUALIFIED {
            name: literal!("Diagram"),
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("graphics"),
            }),
        })
    });

pub(crate) fn offsetAnnotationsInComponentItem(
    mut item: metamodelica::Ref<Absyn::ComponentItem>,
    mut x: i32,
    mut y: i32,
) -> Result<metamodelica::Ref<Absyn::ComponentItem>> {
    let mut item: metamodelica::Ref<Absyn::ComponentItem> = item;
    let mut oann: Option<metamodelica::Ref<Absyn::Annotation>>;
    let mut ann: metamodelica::Ref<Absyn::Annotation>;
    oann = AbsynUtil::getCommentOptAnnotation(item.comment.clone())?;
    ann = if ((oann).is_some()) {
        oann.ok_or("pattern mismatch")?
    } else {
        metamodelica::Ref::new(Absyn::Annotation {
            elementArgs: metamodelica::nil(),
        })
    };
    ann = AbsynUtil::transformAnnotationArg(
        ann,
        PLACEMENT_ORIGIN_PATH.clone(),
        (std::sync::Arc::new({
            let __pe_b1 = x;
            let __pe_b2 = y;
            move |__pe_a0| offsetOriginAnnotation(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ElementArg>) -> Result<metamodelica::Ref<Absyn::ElementArg>>
                    + 'static,
            >),
        true,
    )?;
    ann = offsetIconTransformationAnnotation(ann, x, y);
    item = AbsynUtil::setComponentItemAnnotation(item, Some(ann))?;
    Ok(item)
}

pub(crate) fn offsetIconTransformationAnnotation(
    mut ann: metamodelica::Ref<Absyn::Annotation>,
    mut x: i32,
    mut y: i32,
) -> metamodelica::Ref<Absyn::Annotation> {
    fn r#impl(
        mut arg: metamodelica::Ref<Absyn::ElementArg>,
        mut x: i32,
        mut y: i32,
    ) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
        let mut arg: metamodelica::Ref<Absyn::ElementArg> = arg;
        let mut r#mod: metamodelica::Ref<Absyn::Modification>;
        let () = (::match_deref::match_deref! { match &(arg.clone()) {
            Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(__esc_mod), .. } => {
                r#mod = (*__esc_mod).clone();
                assign_field!(r#mod.elementArgLst = AbsynUtil::transformAnnotationInArgs(r#mod.elementArgLst.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("origin") }), (std::sync::Arc::new({ let __pe_b1 = x; let __pe_b2 = y; move |__pe_a0| offsetOriginAnnotation(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ElementArg>) -> Result<metamodelica::Ref<Absyn::ElementArg>> + 'static>), true)?);
                assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; modification = Some(r#mod.clone()));
                ()
            },
            _ => return Err("match: no arm matched"),
        } });
        Ok(arg)
    }

    let mut ann: metamodelica::Ref<Absyn::Annotation> = ann;
    if '__try0: {
        ann = unwrap_break_err!(AbsynUtil::transformAnnotationArg(ann.clone(), PLACEMENT_ICON_TRANSFORMATION_PATH.clone(), (std::sync::Arc::new({ let __pe_b1 = x; let __pe_b2 = y; move |__pe_a0| r#impl(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ElementArg>) -> Result<metamodelica::Ref<Absyn::ElementArg>> + 'static>), false), '__try0);
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    ann
}

pub(crate) fn offsetOriginAnnotation(
    mut arg: metamodelica::Ref<Absyn::ElementArg>,
    mut x: i32,
    mut y: i32,
) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
    let mut arg: metamodelica::Ref<Absyn::ElementArg> = arg;
    let mut r#mod: metamodelica::Ref<Absyn::Modification>;
    let mut eq_mod: metamodelica::Ref<Absyn::EqMod>;
    let () = (match &*arg {
        Absyn::ElementArg::MODIFICATION {
            modification: __arg_modification,
            ..
        } => {
            if (__arg_modification).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(__arg_modification.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                r#mod = metamodelica::Own::own(__pa0);
            } else {
                r#mod = metamodelica::Ref::new(Absyn::Modification {
                    elementArgLst: metamodelica::nil(),
                    eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD(),
                });
            }
            eq_mod = r#mod.eqMod.clone();
            assign_field!(
                r#mod.eqMod = (match &*eq_mod {
                    Absyn::EqMod::EQMOD { exp: __eq_mod_exp, .. } => {
                        assign_variant_field!(eq_mod => Absyn::EqMod::EQMOD; exp = offsetPointExpression(__eq_mod_exp.clone(), x, y));
                        eq_mod
                    }
                    _ => metamodelica::Ref::new(Absyn::EqMod::EQMOD {
                        exp: makeOrigin(x, y),
                        info: Absyn::dummyInfo.clone()
                    }),
                })
            );
            assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; modification = Some(r#mod));
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(arg)
}

pub(crate) fn makeOrigin(mut x: i32, mut y: i32) -> metamodelica::Ref<Absyn::Exp> {
    let mut origin: metamodelica::Ref<Absyn::Exp> = metamodelica::Ref::new(Absyn::Exp::ARRAY {
        arrayExp: list![
            metamodelica::Ref::new(Absyn::Exp::INTEGER { value: x }),
            metamodelica::Ref::new(Absyn::Exp::INTEGER { value: y })
        ],
    });
    origin
}

pub(crate) fn offsetPointExpression(
    mut point: metamodelica::Ref<Absyn::Exp>,
    mut x: i32,
    mut y: i32,
) -> metamodelica::Ref<Absyn::Exp> {
    let mut point: metamodelica::Ref<Absyn::Exp> = point;
    let mut e1: metamodelica::Ref<Absyn::Exp>;
    let mut e2: metamodelica::Ref<Absyn::Exp>;
    point = (::match_deref::match_deref! { match &(point.clone()) {
        Deref @ Absyn::Exp::ARRAY { arrayExp: Deref @ metamodelica::ListNode::Cons { head: __esc_e1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_e2, tail: Deref @ metamodelica::ListNode::Nil } } } => {
            e1 = (*__esc_e1).clone();
            e2 = (*__esc_e2).clone();
            metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: list![offsetIntegerExpression(e1.clone(), x), offsetIntegerExpression(e2.clone(), y)] })
        },
        _ => metamodelica::Ref::new(Absyn::Exp::BINARY { exp1: point, op: openmodelica_ast::Absyn::Operator::ADD, exp2: makeOrigin(x, y) }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    point
}

pub(crate) fn offsetIntegerExpression(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut offset: i32,
) -> metamodelica::Ref<Absyn::Exp> {
    let mut exp: metamodelica::Ref<Absyn::Exp> = exp;
    let mut v: i32;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::INTEGER { value: __exp_value } => metamodelica::Ref::new(Absyn::Exp::INTEGER { value: __exp_value.clone() + offset }),
        Deref @ Absyn::Exp::UNARY { op: Absyn::Operator::UPLUS { .. }, exp: Deref @ Absyn::Exp::INTEGER { value: __esc_v } } => {
            v = (*__esc_v).clone();
            metamodelica::Ref::new(Absyn::Exp::INTEGER { value: v.clone() + offset })
        },
        Deref @ Absyn::Exp::UNARY { op: Absyn::Operator::UMINUS { .. }, exp: Deref @ Absyn::Exp::INTEGER { value: __esc_v } } => {
            v = (*__esc_v).clone();
            metamodelica::Ref::new(Absyn::Exp::INTEGER { value: -(v.clone()) + offset })
        },
        _ => if (offset > 0) {metamodelica::Ref::new(Absyn::Exp::BINARY { exp1: exp, op: openmodelica_ast::Absyn::Operator::ADD, exp2: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: offset }) })} else if (offset < 0) {metamodelica::Ref::new(Absyn::Exp::BINARY { exp1: exp, op: openmodelica_ast::Absyn::Operator::SUB, exp2: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: -(offset) }) })} else {exp},
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

pub(crate) fn offsetLineExpression(
    mut line: metamodelica::Ref<Absyn::Exp>,
    mut x: i32,
    mut y: i32,
) -> metamodelica::Ref<Absyn::Exp> {
    let mut line: metamodelica::Ref<Absyn::Exp> = line;
    let () = (match &*line {
        Absyn::Exp::ARRAY {
            arrayExp: __line_arrayExp,
        } => {
            assign_variant_field!(line => Absyn::Exp::ARRAY; arrayExp = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
                for mut p in (__line_arrayExp.clone()).into_iter().cloned() {
                    let __x = offsetPointExpression(p.clone(), x, y);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        _ => (),
    });
    line
}

pub(crate) fn offsetAnnotationsInEquationItem(
    mut item: metamodelica::Ref<Absyn::EquationItem>,
    mut x: i32,
    mut y: i32,
) -> metamodelica::Ref<Absyn::EquationItem> {
    let mut item: metamodelica::Ref<Absyn::EquationItem> = item;
    let mut cmt: metamodelica::Ref<Absyn::Comment>;
    let mut ann: metamodelica::Ref<Absyn::Annotation>;
    let () = 'mc: {
        let __mc_input = item.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::EquationItem::EQUATIONITEM { comment: Some(cmt @ Deref @ Absyn::Comment { annotation_: Some(ann), .. }), .. } => {
                    let mut cmt = (*cmt).clone();
                    let mut ann = (*ann).clone();
                    let mut item: metamodelica::Ref<Absyn::EquationItem> = item.clone();
                    ann = AbsynUtil::transformAnnotationArg(ann.clone(), LINE_POINTS_PATH.clone(), (std::sync::Arc::new({ let __pe_b1 = x; let __pe_b2 = y; move |__pe_a0| Ok(offsetConnectionLineAnnotation(__pe_a0, __pe_b1.clone(), __pe_b2.clone())) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ElementArg>) -> Result<metamodelica::Ref<Absyn::ElementArg>> + 'static>), false)?;
                    assign_field!(cmt.annotation_ = Some(ann.clone()));
                    assign_variant_field!(item => Absyn::EquationItem::EQUATIONITEM; comment = Some(cmt.clone()));
                    Ok(((), item.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            item = __wb0;
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
    item
}

pub(crate) fn offsetConnectionLineAnnotation(
    mut arg: metamodelica::Ref<Absyn::ElementArg>,
    mut x: i32,
    mut y: i32,
) -> metamodelica::Ref<Absyn::ElementArg> {
    let mut arg: metamodelica::Ref<Absyn::ElementArg> = arg;
    let mut eq_mod: metamodelica::Ref<Absyn::EqMod>;
    let () = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: __esc_eq_mod @ Deref @ Absyn::EqMod::EQMOD { .. }, .. }), .. } => {
            eq_mod = (*__esc_eq_mod).clone();
            assign_variant_field!(eq_mod => Absyn::EqMod::EQMOD; exp = offsetLineExpression(var_field!((*eq_mod).exp, Absyn::EqMod::EQMOD).clone(), x, y));
            assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; modification = Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: eq_mod.clone() })));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    arg
}

pub(crate) fn offsetDiagramAnnotationInOptComment(
    mut cmt: Option<metamodelica::Ref<Absyn::Comment>>,
    mut x: i32,
    mut y: i32,
) -> Result<Option<metamodelica::Ref<Absyn::Comment>>> {
    let mut cmt: Option<metamodelica::Ref<Absyn::Comment>> = cmt;
    let mut cmt_str: Option<ArcStr>;
    let mut ann: metamodelica::Ref<Absyn::Annotation>;
    let () = (::match_deref::match_deref! { match &(cmt.clone()) {
        Some(Deref @ Absyn::Comment { annotation_: Some(__esc_ann), comment: __esc_cmt_str }) => {
            ann = (*__esc_ann).clone();
            cmt_str = (*__esc_cmt_str).clone();
            ann = offsetDiagramAnnotation(ann.clone(), x, y)?;
            cmt = Some(metamodelica::Ref::new(Absyn::Comment { annotation_: Some(ann.clone()), comment: cmt_str.clone() }));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cmt)
}

pub(crate) fn offsetDiagramAnnotation(
    mut ann: metamodelica::Ref<Absyn::Annotation>,
    mut x: i32,
    mut y: i32,
) -> Result<metamodelica::Ref<Absyn::Annotation>> {
    let mut ann: metamodelica::Ref<Absyn::Annotation> = ann;
    ann = AbsynUtil::transformAnnotationArg(
        ann,
        DIAGRAM_GRAPHICS_PATH.clone(),
        (std::sync::Arc::new({
            let __pe_b1 = x;
            let __pe_b2 = y;
            move |__pe_a0| offsetGraphicsAnnotation(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Absyn::ElementArg>) -> Result<metamodelica::Ref<Absyn::ElementArg>>
                    + 'static,
            >),
        true,
    )?;
    Ok(ann)
}

pub(crate) fn offsetGraphicsAnnotation(
    mut arg: metamodelica::Ref<Absyn::ElementArg>,
    mut x: i32,
    mut y: i32,
) -> Result<metamodelica::Ref<Absyn::ElementArg>> {
    let mut arg: metamodelica::Ref<Absyn::ElementArg> = arg;
    let mut eq_mod: metamodelica::Ref<Absyn::EqMod>;
    let () = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: __esc_eq_mod @ Deref @ Absyn::EqMod::EQMOD { .. }, .. }), .. } => {
            eq_mod = (*__esc_eq_mod).clone();
            assign_variant_field!(eq_mod => Absyn::EqMod::EQMOD; exp = offsetGraphicsExpression(var_field!((*eq_mod).exp, Absyn::EqMod::EQMOD).clone(), x, y)?);
            assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; modification = Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: eq_mod.clone() })));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(arg)
}

pub(crate) fn offsetGraphicsExpression(
    mut graphics: metamodelica::Ref<Absyn::Exp>,
    mut x: i32,
    mut y: i32,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut graphics: metamodelica::Ref<Absyn::Exp> = graphics;
    let () = (match &*graphics {
        Absyn::Exp::ARRAY {
            arrayExp: __graphics_arrayExp,
        } => {
            assign_variant_field!(graphics => Absyn::Exp::ARRAY; arrayExp = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Absyn::Exp>> = metamodelica::nil();
                for mut p in (__graphics_arrayExp.clone()).into_iter().cloned() {
                    let __x = offsetGraphicsItemExpression(p.clone(), x, y)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        _ => (),
    });
    Ok(graphics)
}

pub(crate) fn offsetGraphicsItemExpression(
    mut item: metamodelica::Ref<Absyn::Exp>,
    mut x: i32,
    mut y: i32,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    fn offset_named_origin(
        mut arg: metamodelica::Ref<Absyn::NamedArg>,
        mut x: i32,
        mut y: i32,
    ) -> (metamodelica::Ref<Absyn::NamedArg>, bool) {
        let mut arg: metamodelica::Ref<Absyn::NamedArg> = arg;
        let mut found: bool;
        found = metamodelica::stringEq(&arg.argName, &(literal!("origin")));
        if found {
            assign_field!(arg.argValue = offsetPointExpression(arg.argValue.clone(), x, y));
        }
        (arg, found)
    }

    let mut item: metamodelica::Ref<Absyn::Exp> = item;
    let mut args: metamodelica::Ref<Absyn::FunctionArgs>;
    let mut named_args: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
    let mut found: bool;
    let mut visible: metamodelica::Ref<Absyn::Exp>;
    let mut origin: metamodelica::Ref<Absyn::Exp>;
    let mut rest: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    let () = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::Exp::CALL { functionArgs: __esc_args @ Deref @ Absyn::FunctionArgs::FUNCTIONARGS { .. }, .. } => {
            args = (*__esc_args).clone();
            if ((var_field!((*args).args, Absyn::FunctionArgs::FUNCTIONARGS)).len() as i32) >= 2 {
                let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(var_field!((*args).args, Absyn::FunctionArgs::FUNCTIONARGS).clone()) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                visible = metamodelica::Own::own(__pa0);
                origin = metamodelica::Own::own(__pa1);
                rest = metamodelica::Own::own(__pa2);
                origin = offsetPointExpression(origin, x, y);
                assign_variant_field!(args => Absyn::FunctionArgs::FUNCTIONARGS; args = metamodelica::cons(visible, metamodelica::cons(origin, rest)));
            } else {
                (named_args, found) = List::findMap(var_field!((*args).argNames, Absyn::FunctionArgs::FUNCTIONARGS).clone(), &({ let __pe_b1 = x; let __pe_b2 = y; move |__pe_a0| Ok(offset_named_origin(__pe_a0, __pe_b1.clone(), __pe_b2.clone())) }))?;
                if found {
                    assign_variant_field!(args => Absyn::FunctionArgs::FUNCTIONARGS; argNames = named_args);
                } else {
                    assign_variant_field!(args => Absyn::FunctionArgs::FUNCTIONARGS; argNames = metamodelica::cons(metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("origin"), argValue: makeOrigin(x, y) }), var_field!((*args).argNames, Absyn::FunctionArgs::FUNCTIONARGS).clone()));
                }
            }
            assign_variant_field!(item => Absyn::Exp::CALL; functionArgs = args.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(item)
}

pub(crate) fn addToPublic(
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut element: metamodelica::Ref<Absyn::ElementItem>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    let mut elems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = metamodelica::nil();
    let mut cdef: metamodelica::Ref<Absyn::ClassDef>;
    let () = 'mc: {
        let __mc_input = cls.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: metamodelica::Ref<Absyn::Class> = cls.clone();
                    let mut elems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = elems.clone();
                    elems = ProgramUtil::getPublicList(var_field!((*cdef).classParts, Absyn::ClassDef::PARTS));
                    elems = List::appendElt(element.clone(), elems.clone());
                    assign_variant_field!(cdef => Absyn::ClassDef::PARTS; classParts = ProgramUtil::replacePublicList(var_field!((*cdef).classParts, Absyn::ClassDef::PARTS), elems.clone())?);
                    assign_field!(cls.body = cdef.clone());
                    Ok(((), cls.clone(), elems.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cls = __wb0;
            elems = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: metamodelica::Ref<Absyn::Class> = cls.clone();
                    assign_variant_field!(cdef => Absyn::ClassDef::PARTS; classParts = metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: list![element.clone()] }), var_field!((*cdef).classParts, Absyn::ClassDef::PARTS).clone()));
                    assign_field!(cls.body = cdef.clone());
                    Ok(((), cls.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cls = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: metamodelica::Ref<Absyn::Class> = cls.clone();
                    let mut elems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = elems.clone();
                    elems = ProgramUtil::getPublicList(var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS));
                    elems = List::appendElt(element.clone(), elems.clone());
                    assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; parts = ProgramUtil::replacePublicList(var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS), elems.clone())?);
                    assign_field!(cls.body = cdef.clone());
                    Ok(((), cls.clone(), elems.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cls = __wb0;
            elems = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: metamodelica::Ref<Absyn::Class> = cls.clone();
                    assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; parts = metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: list![element.clone()] }), var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone()));
                    assign_field!(cls.body = cdef.clone());
                    Ok(((), cls.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cls = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(cls)
}

pub(crate) fn addToProtected(
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut element: metamodelica::Ref<Absyn::ElementItem>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    let mut elems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = metamodelica::nil();
    let mut cdef: metamodelica::Ref<Absyn::ClassDef>;
    let () = 'mc: {
        let __mc_input = cls.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: metamodelica::Ref<Absyn::Class> = cls.clone();
                    let mut elems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = elems.clone();
                    elems = ProgramUtil::getProtectedList(var_field!((*cdef).classParts, Absyn::ClassDef::PARTS));
                    elems = List::appendElt(element.clone(), elems.clone());
                    assign_variant_field!(cdef => Absyn::ClassDef::PARTS; classParts = ProgramUtil::replaceProtectedList(var_field!((*cdef).classParts, Absyn::ClassDef::PARTS), elems.clone())?);
                    assign_field!(cls.body = cdef.clone());
                    Ok(((), cls.clone(), elems.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cls = __wb0;
            elems = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: metamodelica::Ref<Absyn::Class> = cls.clone();
                    assign_variant_field!(cdef => Absyn::ClassDef::PARTS; classParts = metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PROTECTED { contents: list![element.clone()] }), var_field!((*cdef).classParts, Absyn::ClassDef::PARTS).clone()));
                    assign_field!(cls.body = cdef.clone());
                    Ok(((), cls.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cls = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: metamodelica::Ref<Absyn::Class> = cls.clone();
                    let mut elems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> = elems.clone();
                    elems = ProgramUtil::getProtectedList(var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS));
                    elems = List::appendElt(element.clone(), elems.clone());
                    assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; parts = ProgramUtil::replaceProtectedList(var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS), elems.clone())?);
                    assign_field!(cls.body = cdef.clone());
                    Ok(((), cls.clone(), elems.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cls = __wb0;
            elems = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: metamodelica::Ref<Absyn::Class> = cls.clone();
                    assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; parts = metamodelica::cons(metamodelica::Ref::new(Absyn::ClassPart::PROTECTED { contents: list![element.clone()] }), var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone()));
                    assign_field!(cls.body = cdef.clone());
                    Ok(((), cls.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cls = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(cls)
}

pub(crate) fn addToEquation(
    mut cls: metamodelica::Ref<Absyn::Class>,
    mut eq: metamodelica::Ref<Absyn::EquationItem>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut cls: metamodelica::Ref<Absyn::Class> = cls;
    let mut eqlst: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>> = metamodelica::nil();
    let mut cdef: metamodelica::Ref<Absyn::ClassDef>;
    let () = 'mc: {
        let __mc_input = cls.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: metamodelica::Ref<Absyn::Class> = cls.clone();
                    let mut eqlst: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>> = eqlst.clone();
                    eqlst = getEquationList(var_field!((*cdef).classParts, Absyn::ClassDef::PARTS))?;
                    eqlst = List::appendElt(eq.clone(), eqlst.clone());
                    assign_variant_field!(cdef => Absyn::ClassDef::PARTS; classParts = replaceEquationList(var_field!((*cdef).classParts, Absyn::ClassDef::PARTS), eqlst.clone())?);
                    assign_field!(cls.body = cdef.clone());
                    Ok(((), cls.clone(), eqlst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cls = __wb0;
            eqlst = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: metamodelica::Ref<Absyn::Class> = cls.clone();
                    assign_variant_field!(cdef => Absyn::ClassDef::PARTS; classParts = List::appendElt(metamodelica::Ref::new(Absyn::ClassPart::EQUATIONS { contents: list![eq.clone()] }), var_field!((*cdef).classParts, Absyn::ClassDef::PARTS).clone()));
                    assign_field!(cls.body = cdef.clone());
                    Ok(((), cls.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cls = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: metamodelica::Ref<Absyn::Class> = cls.clone();
                    let mut eqlst: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>> = eqlst.clone();
                    eqlst = getEquationList(var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS))?;
                    eqlst = List::appendElt(eq.clone(), eqlst.clone());
                    assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; parts = replaceEquationList(var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS), eqlst.clone())?);
                    assign_field!(cls.body = cdef.clone());
                    Ok(((), cls.clone(), eqlst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cls = __wb0;
            eqlst = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: metamodelica::Ref<Absyn::Class> = cls.clone();
                    assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; parts = List::appendElt(metamodelica::Ref::new(Absyn::ClassPart::EQUATIONS { contents: list![eq.clone()] }), var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone()));
                    assign_field!(cls.body = cdef.clone());
                    Ok(((), cls.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cls = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(cls)
}
