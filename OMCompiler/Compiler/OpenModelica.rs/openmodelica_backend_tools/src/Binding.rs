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
use openmodelica_ast::GlobalScript;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_types::SCode;
use openmodelica_loader::Parser;
use openmodelica_program_util::ProgramUtil;
use openmodelica_util::System;

// Imports
// Aliases
pub type Ident = ArcStr;

pub type Path = metamodelica::Ref<Absyn::Path>;

pub type TypeSpec = metamodelica::Ref<Absyn::TypeSpec>;

// Types
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Mediator {
    pub mType: ArcStr,
    pub template: ArcStr,
    pub clients: metamodelica::List<Client>,
    pub providers: metamodelica::List<Provider>,
    pub preferred: metamodelica::List<Preferred>,
}

impl metamodelica::gc::MMTrace for Mediator {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.mType, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.template, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.clients, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.providers, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.preferred, __mmv)?;
        Ok(())
    }
}
pub type MEDIATOR = Mediator;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Client {
    pub modelID: ArcStr,
    pub component: ArcStr,
    pub template: ArcStr,
    pub isMandatory: bool,
}

impl metamodelica::gc::MMTrace for Client {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.modelID, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.component, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.template, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isMandatory, __mmv)?;
        Ok(())
    }
}
pub type CLIENT = Client;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Provider {
    pub modelID: ArcStr,
    pub component: ArcStr,
    pub template: ArcStr,
}

impl metamodelica::gc::MMTrace for Provider {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.modelID, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.component, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.template, __mmv)?;
        Ok(())
    }
}
pub type PROVIDER = Provider;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Preferred {
    pub clientInstancePath: ArcStr,
    pub providerInstancePath: ArcStr,
}

impl metamodelica::gc::MMTrace for Preferred {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.clientInstancePath, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.providerInstancePath, __mmv)?;
        Ok(())
    }
}
pub type PREFERRED = Preferred;

/// internal client list representation
#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum Client_e {
    CLIENT_E {
        components: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
        typeSpec: TypeSpec,
        rootType: TypeSpec,
        def: metamodelica::Ref<Absyn::Class>,
        instance: metamodelica::List<metamodelica::List<ArcStr>>,
        predecessors: metamodelica::Ref<Client_e>,
        mediator: metamodelica::List<Mediator>,
    },
    NO_PRED,
}
impl metamodelica::gc::MMTrace for Client_e {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Client_e::CLIENT_E {
                components,
                typeSpec,
                rootType,
                def,
                instance,
                predecessors,
                mediator,
            } => {
                metamodelica::gc::MMTrace::mm_accept(components, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(typeSpec, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rootType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(def, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(instance, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(predecessors, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(mediator, __mmv)?;
                Ok(())
            }
            Client_e::NO_PRED => Ok(()),
        }
    }
}
impl Client_e {
    pub fn interned_NO_PRED() -> metamodelica::Ref<Client_e> {
        static INTERNED: std::sync::LazyLock<metamodelica::Ref<Client_e>> =
            std::sync::LazyLock::new(|| metamodelica::Ref::new(Client_e::NO_PRED));
        (*INTERNED).clone()
    }
}
pub fn interned_NO_PRED() -> metamodelica::Ref<Client_e> {
    Client_e::interned_NO_PRED()
}
pub(crate) use self::Client_e::{CLIENT_E, NO_PRED};

pub fn inferBindings(mut model_path: Path, mut env: Absyn::Program) -> Result<Absyn::Program> {
    let mut out_model_def: Absyn::Program;
    let mut ms: metamodelica::List<Mediator>;
    let mut model_def: metamodelica::Ref<Absyn::Class>;
    let mut out_vmodel: metamodelica::Ref<Absyn::Class>;
    let mut scode_def: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut client_list: metamodelica::List<metamodelica::Ref<Client_e>>;
    model_def = ProgramUtil::getPathedClassInProgram(model_path.clone(), &env, false, false)?;
    scode_def = AbsynToSCode::translateAbsyn2SCode(env.clone())?;
    ms = getMediatorDefsElements(&scode_def, metamodelica::nil());
    client_list = buildInstList(
        &model_def,
        &env,
        &(crate::Binding::Client_e::interned_NO_PRED()),
        &ms,
        &(metamodelica::nil()),
        &(metamodelica::nil()),
    )?;
    out_vmodel = inferBindingClientList(&client_list, model_def, &env)?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Dump::unparseClassStr(out_vmodel.clone())?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    out_model_def = ProgramUtil::updateProgram(
        Absyn::Program {
            classes: list![out_vmodel],
            within_: ProgramUtil::buildWithin(model_path)?,
        },
        env,
        false,
        false,
    )?;
    Ok(out_model_def)
}

pub fn generateVerificationScenarios(mut package_path: Path, mut in_env: Absyn::Program) -> Result<Absyn::Program> {
    let mut out_env: Absyn::Program;
    let mut ms: metamodelica::List<Mediator>;
    let mut package_def: metamodelica::Ref<Absyn::Class>;
    let mut out_vmodel: metamodelica::Ref<Absyn::Class>;
    let mut autogen_class: metamodelica::Ref<Absyn::Class>;
    let mut autogen_class2: metamodelica::Ref<Absyn::Class>;
    let mut scode_def: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut design_alts: metamodelica::List<(metamodelica::Ref<SCode::Element>, ArcStr)>;
    let mut reqs: metamodelica::List<(metamodelica::Ref<SCode::Element>, ArcStr)>;
    let mut scenarios: metamodelica::List<(metamodelica::Ref<SCode::Element>, ArcStr)>;
    let mut client_list: metamodelica::List<metamodelica::Ref<Client_e>>;
    let mut ag_elems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut autogen_model_list: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    let mut autogen_model: metamodelica::Ref<Absyn::ElementItem>;
    let mut i: i32;
    scode_def = AbsynToSCode::translateAbsyn2SCode(in_env.clone())?;
    design_alts = getAllElementsOfType(
        &scode_def,
        &(literal!("VVDRlib.Verification.Design")),
        &(literal!("")),
        metamodelica::nil(),
    );
    reqs = getAllElementsOfType(
        &scode_def,
        &(literal!("VVDRlib.Verification.Requirement")),
        &(literal!("")),
        metamodelica::nil(),
    );
    scenarios = getAllElementsOfType(
        &scode_def,
        &(literal!("VVDRlib.Verification.Scenario")),
        &(literal!("")),
        metamodelica::nil(),
    );
    ms = getMediatorDefsElements(&scode_def, metamodelica::nil());
    package_def = ProgramUtil::getPathedClassInProgram(package_path.clone(), &in_env, false, false)?;
    autogen_model_list = metamodelica::nil();
    i = 0;
    for mut s in &*scenarios {
        for mut d in &*design_alts {
            ag_elems = populateModel(
                &(metamodelica::cons(s.clone(), metamodelica::cons(d.clone(), reqs.clone()))),
                0,
                metamodelica::nil(),
            )?;
            ag_elems = metamodelica::cons(
                metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM {
                    element: metamodelica::Ref::new(Absyn::Element::ELEMENT {
                        finalPrefix: false,
                        redeclareKeywords: None,
                        innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
                        specification: metamodelica::Ref::new(Absyn::ElementSpec::EXTENDS {
                            path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                                name: literal!("VVDRlib"),
                                path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                                    name: literal!("Verification"),
                                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                                        name: literal!("VerificationModel"),
                                    }),
                                }),
                            }),
                            elementArg: metamodelica::nil(),
                            annotationOpt: None,
                        }),
                        info: Absyn::dummyInfo.clone(),
                        constrainClass: None,
                    }),
                }),
                ag_elems,
            );
            autogen_class = metamodelica::Ref::new(Absyn::Class {
                name: {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("verif_model_autogen_"));
                    __mm_s.push_str(&*intString(i));
                    ArcStr::from(__mm_s)
                },
                partialPrefix: false,
                finalPrefix: false,
                encapsulatedPrefix: false,
                restriction: openmodelica_ast::Absyn::Restriction::R_MODEL,
                body: metamodelica::Ref::new(Absyn::ClassDef::PARTS {
                    typeVars: metamodelica::nil(),
                    classAttrs: metamodelica::nil(),
                    classParts: list![metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: ag_elems })],
                    ann: metamodelica::nil(),
                    comment: Some(literal!("Autogenerated verification model")),
                }),
                commentsBeforeClass: metamodelica::nil(),
                commentsBeforeEnd: metamodelica::nil(),
                commentsAfterEnd: metamodelica::nil(),
                info: Absyn::dummyInfo.clone(),
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("*** Autogenerated class: "));
                __mm_s.push_str(&*literal!("verif_model_autogen_"));
                __mm_s.push_str(&*intString(i));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            client_list = buildInstList(
                &autogen_class,
                &in_env,
                &(crate::Binding::Client_e::interned_NO_PRED()),
                &ms,
                &(metamodelica::nil()),
                &(metamodelica::nil()),
            )?;
            autogen_class2 = inferBindingClientList(&client_list, autogen_class, &in_env)?;
            autogen_model = metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM {
                element: metamodelica::Ref::new(Absyn::Element::ELEMENT {
                    finalPrefix: false,
                    redeclareKeywords: None,
                    innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
                    specification: metamodelica::Ref::new(Absyn::ElementSpec::CLASSDEF {
                        replaceable_: false,
                        class_: autogen_class2,
                    }),
                    info: Absyn::dummyInfo.clone(),
                    constrainClass: None,
                }),
            });
            autogen_model_list = metamodelica::cons(autogen_model, autogen_model_list);
            i = i + 1;
        }
    }
    out_vmodel = updatePackage(package_def, autogen_model_list)?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("******** Autogenerated classes:\n"));
        __mm_s.push_str(&*Dump::unparseClassStr(out_vmodel.clone())?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    out_env = ProgramUtil::updateProgram(
        Absyn::Program {
            classes: list![out_vmodel],
            within_: ProgramUtil::buildWithin(package_path)?,
        },
        in_env,
        false,
        false,
    )?;
    Ok(out_env)
}

pub(crate) fn updatePackage(
    mut in_class: metamodelica::Ref<Absyn::Class>,
    mut ag_elems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut out_class: metamodelica::Ref<Absyn::Class>;
    out_class = (::match_deref::match_deref! { match &(in_class) {
        __esc_out_class @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts: _, ann, comment }, .. } => {
            out_class = (*__esc_out_class).clone();
            assign_field!(out_class.body = metamodelica::Ref::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: list![metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: ag_elems })], ann: ann.clone(), comment: comment.clone() }));
            out_class.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(out_class)
}

fn populateModel<'__b>(
    mut element_defs: &'__b metamodelica::List<(metamodelica::Ref<SCode::Element>, ArcStr)>,
    mut autoVal: i32,
    mut elements_in: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match element_defs {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(elements_in)
            },
            Deref @ metamodelica::ListNode::Cons { head: (Deref @ SCode::Element::CLASS { name: cname, prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: _, classDef: _, cmt: _, info: _ }, p_path), tail: rest } => {
                let mut el: metamodelica::Ref<Absyn::Element>;
                let mut nName: ArcStr;
                nName = if (metamodelica::stringEq(&p_path, &(literal!("")))) {cname.clone()} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*p_path); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*cname); ArcStr::from(__mm_s) }};
                el = metamodelica::Ref::new(Absyn::Element::ELEMENT { finalPrefix: false, redeclareKeywords: None, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, specification: metamodelica::Ref::new(Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { flowPrefix: false, streamPrefix: false, parallelism: openmodelica_ast::Absyn::Parallelism::NON_PARALLEL, variability: openmodelica_ast::Absyn::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD, arrayDim: metamodelica::nil() }, typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH { path: AbsynUtil::stringPath(nName)?, arrayDim: None }), components: list![metamodelica::Ref::new(Absyn::ComponentItem { component: Absyn::Component { name: { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("_agen_")); __mm_s.push_str(&*cname); __mm_s.push_str(&*intString(autoVal)); ArcStr::from(__mm_s) }, arrayDim: metamodelica::nil(), modification: None }, condition: None, comment: None })] }), info: Absyn::dummyInfo.clone(), constrainClass: None });
                { (element_defs, autoVal, elements_in) = (rest, autoVal + 1, metamodelica::cons(metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM { element: el }), elements_in)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getAllElementsOfType<'__b>(
    mut element_defs: &'__b metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut typeName: &'__b Ident,
    mut pathInProg: &'__b ArcStr,
    mut elements_in: metamodelica::List<(metamodelica::Ref<SCode::Element>, ArcStr)>,
) -> metamodelica::List<(metamodelica::Ref<SCode::Element>, ArcStr)> {
    '__tco: loop {
        ::match_deref::match_deref! { match element_defs {
            Deref @ metamodelica::ListNode::Nil => {
                return elements_in
            },
            Deref @ metamodelica::ListNode::Cons { head: el, tail: rest } => {
                let mut m: metamodelica::List<(metamodelica::Ref<SCode::Element>, ArcStr)>;
                m = listAppend(getAllElementsOfType2(el.clone(), typeName, pathInProg.clone()), elements_in);
                { (element_defs, typeName, pathInProg, elements_in) = (rest, typeName, pathInProg, m); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn getAllElementsOfType2(
    mut el: metamodelica::Ref<SCode::Element>,
    mut typeName: &Ident,
    mut pathInProg: ArcStr,
) -> metamodelica::List<(metamodelica::Ref<SCode::Element>, ArcStr)> {
    let mut res_elem: metamodelica::List<(metamodelica::Ref<SCode::Element>, ArcStr)>;
    res_elem = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ SCode::Element::CLASS { name: Deref @ "Modelica", prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::Restriction::R_PACKAGE { .. }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: _, normalEquationLst: _, initialEquationLst: _, normalAlgorithmLst: _, initialAlgorithmLst: _, constraintLst: _, clsattrs: _, externalDecl: _ }, cmt: _, info: _ } => {
            metamodelica::print(literal!("**** Ignoring Standard Modelica library\n"));
            metamodelica::nil()
        },
        Deref @ SCode::Element::CLASS { name: Deref @ "OpenModelica", prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::Restriction::R_PACKAGE { .. }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: _, normalEquationLst: _, initialEquationLst: _, normalAlgorithmLst: _, initialAlgorithmLst: _, constraintLst: _, clsattrs: _, externalDecl: _ }, cmt: _, info: _ } => {
            metamodelica::print(literal!("**** Ignoring Open Modelica library\n"));
            metamodelica::nil()
        },
        Deref @ SCode::Element::CLASS { name: Deref @ "Complex", prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::Restriction::R_PACKAGE { .. }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: _, normalEquationLst: _, initialEquationLst: _, normalAlgorithmLst: _, initialAlgorithmLst: _, constraintLst: _, clsattrs: _, externalDecl: _ }, cmt: _, info: _ } => {
            metamodelica::print(literal!("**** Ignoring Complex library\n"));
            metamodelica::nil()
        },
        Deref @ SCode::Element::CLASS { name, prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::Restriction::R_PACKAGE { .. }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elist, normalEquationLst: _, initialEquationLst: _, normalAlgorithmLst: _, initialAlgorithmLst: _, constraintLst: _, clsattrs: _, externalDecl: _ }, cmt: _, info: _ } => {
            let mut nName: ArcStr;
            nName = if (metamodelica::stringEq(&pathInProg, &(literal!("")))) {name.clone()} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*pathInProg); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }};
            getAllElementsOfType(metamodelica::AsArg::as_arg(&elist), typeName, &nName, metamodelica::nil())
        },
        Deref @ SCode::Element::CLASS { name: _, prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: _, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elist, normalEquationLst: _, initialEquationLst: _, normalAlgorithmLst: _, initialAlgorithmLst: _, constraintLst: _, clsattrs: _, externalDecl: _ }, cmt: _, info: _ } if (isOfType(metamodelica::AsArg::as_arg(&elist), typeName)) => {
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*** Found a ")); __mm_s.push_str(&*typeName); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            list![(el, pathInProg)]
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res_elem
}

fn isOfType(mut elems: &metamodelica::List<metamodelica::Ref<SCode::Element>>, mut typeName: &ArcStr) -> bool {
    let mut result: bool;
    result = 'mc: {
        let __mc_input = &**elems;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: p, visibility: _, modifications: _, ann: _, info: _ }, tail: _ } => {
                    let true = (metamodelica::stringEq(&(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?), &typeName)) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(isOfType(metamodelica::AsArg::as_arg(&rest), typeName))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    result
}

fn inferBindingClientList<'__b>(
    mut client_list: &'__b metamodelica::List<metamodelica::Ref<Client_e>>,
    mut vmodel: metamodelica::Ref<Absyn::Class>,
    mut env: &'__b Absyn::Program,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    '__tco: loop {
        ::match_deref::match_deref! { match client_list {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(vmodel)
            },
            Deref @ metamodelica::ListNode::Cons { head: ce, tail: rest } => {
                let mut upd_vmodel: metamodelica::Ref<Absyn::Class>;
                upd_vmodel = inferBindingClient(metamodelica::AsArg::as_arg(&ce), vmodel, env)?;
                { (client_list, vmodel, env) = (rest, upd_vmodel, env); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn inferBindingClient(
    mut client_e: &metamodelica::Ref<Client_e>,
    mut vmodel: metamodelica::Ref<Absyn::Class>,
    mut env: &Absyn::Program,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut out_vmodel: metamodelica::Ref<Absyn::Class>;
    out_vmodel = (::match_deref::match_deref! { match client_e {
        Deref @ Client_e::CLIENT_E { components: _, typeSpec, rootType, def: _, instance: iname, predecessors: _, mediator: Deref @ metamodelica::ListNode::Cons { head: Mediator { mType: _, template, clients: _, providers, preferred: Deref @ metamodelica::ListNode::Nil }, tail: _ } } => {
            let mut out_es: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, ArcStr)>;
            let mut exp: metamodelica::Ref<Absyn::Exp>;
            let mut new_exp: metamodelica::Ref<Absyn::Exp>;
            let mut out_class: metamodelica::Ref<Absyn::Class>;
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("... infer binding ")); __mm_s.push_str(&*Dump::unparseTypeSpec(typeSpec.clone())?); __mm_s.push_str(&*literal!("     ")); __mm_s.push_str(&*Dump::unparseTypeSpec(rootType.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            out_es = getProviders(metamodelica::AsArg::as_arg(&providers), &vmodel, env, metamodelica::nil())?;
            if metamodelica::stringEq(&template, &(literal!(""))) {
                out_class = updateClass(&vmodel, typeSpec, rootType, &out_es, iname, env, false, &(metamodelica::nil()), &(literal!("")))?;
            } else {
                let __pa0 = ::match_deref::match_deref! { match &(Parser::parsestringexp(template.clone(), literal!("<interactive>"))?) {
                    GlobalScript::Statements { interactiveStmtLst: Deref @ metamodelica::ListNode::Cons { head: GlobalScript::Statement::IEXP { exp: __pa0, info: _ }, tail: Deref @ metamodelica::ListNode::Nil }, semicolon: _ } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                exp = metamodelica::Own::own(__pa0);
                new_exp = parseAggregator(&exp, &(metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![metamodelica::Ref::new(Absyn::Exp::LIST { exps: toExpList(&out_es, metamodelica::nil()) })], argNames: metamodelica::nil() })));
                out_class = updateClass(&vmodel, typeSpec, rootType, &(list![(new_exp, literal!(""))]), iname, env, false, &(metamodelica::nil()), &(literal!("")))?;
            }
            out_class
        },
        Deref @ Client_e::CLIENT_E { components: _, typeSpec, rootType, def: _, instance: iname, predecessors: _, mediator: Deref @ metamodelica::ListNode::Cons { head: Mediator { mType: _, template: _, clients: _, providers, preferred }, tail: _ } } => {
            let mut out_es: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, ArcStr)>;
            let mut out_class: metamodelica::Ref<Absyn::Class>;
            out_es = getProviders(metamodelica::AsArg::as_arg(&providers), &vmodel, env, metamodelica::nil())?;
            out_class = updateClass(&vmodel, typeSpec, rootType, &out_es, iname, env, true, metamodelica::AsArg::as_arg(&preferred), &(literal!("")))?;
            out_class
        },
        Deref @ Client_e::NO_PRED { .. } => {
            vmodel
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(out_vmodel)
}

pub(crate) fn toExpList<'__b>(
    mut e_list: &'__b metamodelica::List<(metamodelica::Ref<Absyn::Exp>, ArcStr)>,
    mut in_es: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Exp>> {
    '__tco: loop {
        ::match_deref::match_deref! { match e_list {
            Deref @ metamodelica::ListNode::Nil => {
                return in_es
            },
            Deref @ metamodelica::ListNode::Cons { head: (exp, _), tail: rest } => {
                { (e_list, in_es) = (rest, metamodelica::cons(exp.clone(), in_es)); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn updateClass(
    mut in_class: &metamodelica::Ref<Absyn::Class>,
    mut typeSpec: &TypeSpec,
    mut rootType: &TypeSpec,
    mut exp: &metamodelica::List<(metamodelica::Ref<Absyn::Exp>, ArcStr)>,
    mut instance_name: &metamodelica::List<metamodelica::List<ArcStr>>,
    mut defs: &Absyn::Program,
    mut hasPreferred: bool,
    mut preferred: &metamodelica::List<Preferred>,
    mut path: &ArcStr,
) -> Result<metamodelica::Ref<Absyn::Class>> {
    let mut out_class: metamodelica::Ref<Absyn::Class>;
    let __arc1 = &(*in_class);
    let __pa0 = (*__arc1).clone();
    let Absyn::CLASS { .. } = &**__arc1;
    out_class = metamodelica::Own::own(__pa0);
    assign_field!(
        out_class.body = parseClassDef(
            &(out_class.body.clone()),
            defs,
            typeSpec,
            rootType,
            exp,
            instance_name,
            hasPreferred,
            preferred,
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*path);
                __mm_s.push_str(&*out_class.name);
                __mm_s.push_str(&*literal!("."));
                ArcStr::from(__mm_s)
            })
        )?
    );
    Ok(out_class)
}

fn parseClassDef(
    mut in_def: &metamodelica::Ref<Absyn::ClassDef>,
    mut defs: &Absyn::Program,
    mut typeSpec: &TypeSpec,
    mut rootType: &TypeSpec,
    mut exp: &metamodelica::List<(metamodelica::Ref<Absyn::Exp>, ArcStr)>,
    mut instance_name: &metamodelica::List<metamodelica::List<ArcStr>>,
    mut hasPreferred: bool,
    mut preferred: &metamodelica::List<Preferred>,
    mut path: &ArcStr,
) -> Result<metamodelica::Ref<Absyn::ClassDef>> {
    let mut out_def: metamodelica::Ref<Absyn::ClassDef>;
    out_def = (match &**in_def {
        Absyn::ClassDef::PARTS {
            typeVars,
            classAttrs,
            classParts,
            ann,
            comment,
        } => {
            let mut nclsp: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            nclsp = parseClassParts(
                classParts,
                defs,
                typeSpec,
                rootType,
                exp,
                instance_name,
                hasPreferred,
                preferred,
                path,
            )?;
            metamodelica::Ref::new(Absyn::ClassDef::PARTS {
                typeVars: typeVars.clone(),
                classAttrs: classAttrs.clone(),
                classParts: nclsp,
                ann: ann.clone(),
                comment: comment.clone(),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(out_def)
}

fn parseClassParts(
    mut classes: &metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>,
    mut defs: &Absyn::Program,
    mut typeSpec: &TypeSpec,
    mut rootType: &TypeSpec,
    mut exp: &metamodelica::List<(metamodelica::Ref<Absyn::Exp>, ArcStr)>,
    mut instance_name: &metamodelica::List<metamodelica::List<ArcStr>>,
    mut hasPreferred: bool,
    mut preferred: &metamodelica::List<Preferred>,
    mut path: &ArcStr,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>> {
    let mut out_classes: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
    out_classes = (::match_deref::match_deref! { match classes {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: cls, tail: r_classes } => {
            let mut nr_classes: metamodelica::List<metamodelica::Ref<Absyn::ClassPart>>;
            let mut n_cls: metamodelica::Ref<Absyn::ClassPart>;
            n_cls = parseClassPart(cls.clone(), defs, typeSpec, rootType, exp, instance_name, hasPreferred, preferred, path)?;
            nr_classes = parseClassParts(r_classes, defs, typeSpec, rootType, exp, instance_name, hasPreferred, preferred, path)?;
            metamodelica::cons(n_cls, nr_classes)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_classes)
}

fn parseClassPart(
    mut in_def: metamodelica::Ref<Absyn::ClassPart>,
    mut defs: &Absyn::Program,
    mut typeSpec: &TypeSpec,
    mut rootType: &TypeSpec,
    mut exp: &metamodelica::List<(metamodelica::Ref<Absyn::Exp>, ArcStr)>,
    mut instance_name: &metamodelica::List<metamodelica::List<ArcStr>>,
    mut hasPreferred: bool,
    mut preferred: &metamodelica::List<Preferred>,
    mut path: &ArcStr,
) -> Result<metamodelica::Ref<Absyn::ClassPart>> {
    let mut out_def: metamodelica::Ref<Absyn::ClassPart>;
    out_def = (match &*in_def {
        Absyn::ClassPart::PUBLIC { contents: elems } => {
            let mut elems1: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
            elems1 = parseElems(
                metamodelica::AsArg::as_arg(&elems),
                defs,
                typeSpec,
                rootType,
                exp,
                instance_name,
                hasPreferred,
                preferred,
                path,
            );
            metamodelica::Ref::new(Absyn::ClassPart::PUBLIC { contents: elems1 })
        }
        _ => in_def,
    });
    Ok(out_def)
}

fn parseElems(
    mut in_elems: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut defs: &Absyn::Program,
    mut typeSpec: &TypeSpec,
    mut rootType: &TypeSpec,
    mut exp2: &metamodelica::List<(metamodelica::Ref<Absyn::Exp>, ArcStr)>,
    mut instance_name: &metamodelica::List<metamodelica::List<ArcStr>>,
    mut hasPreferred: bool,
    mut preferred: &metamodelica::List<Preferred>,
    mut pathInClass: &ArcStr,
) -> metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> {
    let mut out_elems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    out_elems = 'mc: {
        let __mc_input = &**in_elems;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(in_elems.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix, redeclareKeywords, innerOuter, specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes, typeSpec: tSpec, components }, info, constrainClass } }, tail: rest } => {
                    let mut e_list: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut newName: bool;
                    if metamodelica::stringEq(&(AbsynUtil::typeSpecPathString(rootType)?), &(AbsynUtil::typeSpecPathString(metamodelica::AsArg::as_arg(&tSpec))?)) && !((exp2).is_empty()) {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("... found instance provider ")); __mm_s.push_str(&*Dump::unparseTypeSpec(tSpec.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        if hasPreferred {
                            e_list = applyModifiersPreferred(metamodelica::AsArg::as_arg(&components), exp2, instance_name, pathInClass, finalPrefix.clone(), redeclareKeywords.clone(), innerOuter.clone(), metamodelica::AsArg::as_arg(&info), constrainClass.clone(), metamodelica::AsArg::as_arg(&attributes), metamodelica::AsArg::as_arg(&tSpec), preferred);
                        } else {
                            newName = ((exp2).len() as i32) != 1;
                            e_list = applyModifiers(metamodelica::AsArg::as_arg(&components), exp2, instance_name, 0, finalPrefix.clone(), redeclareKeywords.clone(), innerOuter.clone(), metamodelica::AsArg::as_arg(&info), constrainClass.clone(), metamodelica::AsArg::as_arg(&attributes), metamodelica::AsArg::as_arg(&tSpec), newName);
                        }
                    } else {
                        e_list = list![metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM { element: metamodelica::Ref::new(Absyn::Element::ELEMENT { finalPrefix: finalPrefix.clone(), redeclareKeywords: redeclareKeywords.clone(), innerOuter: innerOuter.clone(), specification: metamodelica::Ref::new(Absyn::ElementSpec::COMPONENTS { attributes: attributes.clone(), typeSpec: tSpec.clone(), components: components.clone() }), info: info.clone(), constrainClass: constrainClass.clone() }) })];
                    }
                    Ok(listAppend(e_list.clone(), parseElems(metamodelica::AsArg::as_arg(&rest), defs, typeSpec, rootType, exp2, instance_name, hasPreferred, preferred, pathInClass)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: e_item, tail: rest } => {
                    Ok(metamodelica::cons(e_item.clone(), parseElems(metamodelica::AsArg::as_arg(&rest), defs, typeSpec, rootType, exp2, instance_name, hasPreferred, preferred, pathInClass)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    out_elems
}

fn applyModifiersPreferred(
    mut comps: &metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
    mut exp: &metamodelica::List<(metamodelica::Ref<Absyn::Exp>, ArcStr)>,
    mut instance_name: &metamodelica::List<metamodelica::List<ArcStr>>,
    mut typeSp: &ArcStr,
    mut finalPrefix: bool,
    mut redeclareKeywords: Option<Absyn::RedeclareKeywords>,
    mut innerOuter: Absyn::InnerOuter,
    mut info: &SourceInfo,
    mut constrainClass: Option<metamodelica::Ref<Absyn::ConstrainClass>>,
    mut attributes: &Absyn::ElementAttributes,
    mut tSpec: &TypeSpec,
    mut preferred: &metamodelica::List<Preferred>,
) -> metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> {
    let mut out_elems: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    out_elems = 'mc: {
        let __mc_input = &**exp;
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
                Deref @ metamodelica::ListNode::Cons { head: (e, ename), tail: rest } => {
                    let mut cnew: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
                    let mut client_pref: ArcStr;
                    let mut enew: metamodelica::Ref<Absyn::ElementItem>;
                    client_pref = getPreferredBinding(metamodelica::AsArg::as_arg(&ename), preferred)?;
                    cnew = applyModifierPreferred(comps, metamodelica::AsArg::as_arg(&e), &client_pref, instance_name, metamodelica::AsArg::as_arg(&ename));
                    enew = metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM { element: metamodelica::Ref::new(Absyn::Element::ELEMENT { finalPrefix: finalPrefix, redeclareKeywords: redeclareKeywords.clone(), innerOuter: innerOuter, specification: metamodelica::Ref::new(Absyn::ElementSpec::COMPONENTS { attributes: attributes.clone(), typeSpec: tSpec.clone(), components: cnew.clone() }), info: info.clone(), constrainClass: constrainClass.clone() }) });
                    Ok(metamodelica::cons(enew.clone(), applyModifiersPreferred(comps, metamodelica::AsArg::as_arg(&rest), instance_name, typeSp, finalPrefix, redeclareKeywords.clone(), innerOuter, info, constrainClass.clone(), attributes, tSpec, preferred)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(applyModifiersPreferred(comps, metamodelica::AsArg::as_arg(&rest), instance_name, typeSp, finalPrefix, redeclareKeywords.clone(), innerOuter, info, constrainClass.clone(), attributes, tSpec, preferred))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    out_elems
}

fn getPreferredBinding(mut ename: &ArcStr, mut elems: &metamodelica::List<Preferred>) -> Result<ArcStr> {
    let mut cl_name: ArcStr;
    cl_name = 'mc: {
        let __mc_input = &**elems;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Preferred { clientInstancePath: c_id, providerInstancePath: p_id }, tail: _ } => {
                    let true = (metamodelica::stringEq(&p_id, &ename)) else { return Err("pattern mismatch") };
                    Ok(c_id.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(getPreferredBinding(ename, metamodelica::AsArg::as_arg(&rest))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(cl_name)
}

fn applyModifierPreferred(
    mut comps: &metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
    mut exp: &metamodelica::Ref<Absyn::Exp>,
    mut typeSp: &ArcStr,
    mut instance_name: &metamodelica::List<metamodelica::List<ArcStr>>,
    mut ename: &ArcStr,
) -> metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>> {
    let mut out_comps: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
    out_comps = 'mc: {
        let __mc_input = &**comps;
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
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name, arrayDim, modification: _ }, condition, comment }, tail: _ } => {
                    let mut cnew: metamodelica::Ref<Absyn::ComponentItem>;
                    let true = (metamodelica::stringEq(&typeSp, &name)) else { return Err("pattern mismatch") };
                    cnew = metamodelica::Ref::new(Absyn::ComponentItem { component: Absyn::Component { name: name.clone(), arrayDim: arrayDim.clone(), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: buildComponentModifiers(instance_name, exp)?, eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })) }, condition: condition.clone(), comment: comment.clone() });
                    Ok(list![cnew.clone()])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(applyModifierPreferred(metamodelica::AsArg::as_arg(&rest), exp, typeSp, instance_name, ename))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    out_comps
}

fn applyModifiers<'__b>(
    mut comps: &'__b metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
    mut exp: &'__b metamodelica::List<(metamodelica::Ref<Absyn::Exp>, ArcStr)>,
    mut instance_name: &'__b metamodelica::List<metamodelica::List<ArcStr>>,
    mut counter: i32,
    mut finalPrefix: bool,
    mut redeclareKeywords: Option<Absyn::RedeclareKeywords>,
    mut innerOuter: Absyn::InnerOuter,
    mut info: &'__b SourceInfo,
    mut constrainClass: Option<metamodelica::Ref<Absyn::ConstrainClass>>,
    mut attributes: &'__b Absyn::ElementAttributes,
    mut tSpec: &'__b TypeSpec,
    mut newName: bool,
) -> metamodelica::List<metamodelica::Ref<Absyn::ElementItem>> {
    '__tco: loop {
        ::match_deref::match_deref! { match exp {
            Deref @ metamodelica::ListNode::Nil => {
                return metamodelica::nil()
            },
            Deref @ metamodelica::ListNode::Cons { head: (e, _), tail: rest } => {
                let mut cnew: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
                let mut enew: metamodelica::Ref<Absyn::ElementItem>;
                cnew = applyModifier(comps, metamodelica::AsArg::as_arg(&e), instance_name, counter, newName);
                enew = metamodelica::Ref::new(Absyn::ElementItem::ELEMENTITEM { element: metamodelica::Ref::new(Absyn::Element::ELEMENT { finalPrefix: finalPrefix, redeclareKeywords: redeclareKeywords.clone(), innerOuter: innerOuter, specification: metamodelica::Ref::new(Absyn::ElementSpec::COMPONENTS { attributes: attributes.clone(), typeSpec: tSpec.clone(), components: cnew }), info: info.clone(), constrainClass: constrainClass.clone() }) });
                return metamodelica::cons(enew, applyModifiers(comps, rest, instance_name, counter + 1, finalPrefix, redeclareKeywords, innerOuter, info, constrainClass, attributes, tSpec, newName))
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { (comps, exp, instance_name, counter, finalPrefix, redeclareKeywords, innerOuter, info, constrainClass, attributes, tSpec, newName) = (comps, rest, instance_name, counter, finalPrefix, redeclareKeywords, innerOuter, info, constrainClass, attributes, tSpec, newName); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn applyModifier(
    mut comps: &metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
    mut exp: &metamodelica::Ref<Absyn::Exp>,
    mut instance_name: &metamodelica::List<metamodelica::List<ArcStr>>,
    mut counter: i32,
    mut newName: bool,
) -> metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>> {
    let mut out_comps: metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>;
    out_comps = 'mc: {
        let __mc_input = &**comps;
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
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name, arrayDim, modification: _ }, condition, comment }, tail: _ } => {
                    let mut cnew: metamodelica::Ref<Absyn::ComponentItem>;
                    let mut new_name: ArcStr;
                    new_name = if (newName) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!("_autogen_bind_")); __mm_s.push_str(&*intString(counter)); ArcStr::from(__mm_s) }} else {name.clone()};
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("**** Applying modifier ")); __mm_s.push_str(&*new_name); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    cnew = metamodelica::Ref::new(Absyn::ComponentItem { component: Absyn::Component { name: new_name.clone(), arrayDim: arrayDim.clone(), modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: buildComponentModifiers(instance_name, exp)?, eqMod: openmodelica_ast::Absyn::EqMod::interned_NOMOD() })) }, condition: condition.clone(), comment: comment.clone() });
                    Ok(list![cnew.clone()])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(applyModifier(metamodelica::AsArg::as_arg(&rest), exp, instance_name, counter, newName))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    out_comps
}

fn buildComponentModifiers(
    mut name_list: &metamodelica::List<metamodelica::List<ArcStr>>,
    mut exp: &metamodelica::Ref<Absyn::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>> {
    let mut out_modifiers: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    let mut parsed_names: metamodelica::List<ArcStr>;
    parsed_names = buildAllComponentModifiers(name_list);
    out_modifiers = buildComponentModifiers2(&parsed_names, exp)?;
    Ok(out_modifiers)
}

fn buildComponentModifiers2(
    mut name_list: &metamodelica::List<ArcStr>,
    mut exp: &metamodelica::Ref<Absyn::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>> {
    let mut out_modifiers: metamodelica::List<metamodelica::Ref<Absyn::ElementArg>>;
    out_modifiers = (::match_deref::match_deref! { match name_list {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: name, tail: rest } => {
            metamodelica::cons(metamodelica::Ref::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: AbsynUtil::stringPath(name.clone())?, modification: Some(metamodelica::Ref::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: metamodelica::Ref::new(Absyn::EqMod::EQMOD { exp: exp.clone(), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() }), buildComponentModifiers2(rest, exp)?)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_modifiers)
}

fn buildAllComponentModifiers(
    mut name_list: &metamodelica::List<metamodelica::List<ArcStr>>,
) -> metamodelica::List<ArcStr> {
    let mut out_names: metamodelica::List<ArcStr>;
    out_names = (::match_deref::match_deref! { match name_list {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: l, tail: rest } => {
            let mut tmp_names: metamodelica::List<ArcStr>;
            tmp_names = buildAllComponentModifiers(rest);
            buildAllComponentModifiers2(metamodelica::AsArg::as_arg(&l), &tmp_names)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out_names
}

fn buildAllComponentModifiers2(
    mut name_list: &metamodelica::List<ArcStr>,
    mut name_list2: &metamodelica::List<ArcStr>,
) -> metamodelica::List<ArcStr> {
    let mut out_names: metamodelica::List<ArcStr>;
    out_names = (::match_deref::match_deref! { match name_list {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: s, tail: rest } => {
            let mut tmp_names: metamodelica::List<ArcStr>;
            tmp_names = buildAllComponentModifiers2(rest, name_list2);
            listAppend(buildAllComponentModifiers3(s.clone(), name_list2), tmp_names)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out_names
}

fn buildAllComponentModifiers3(
    mut prefix: ArcStr,
    mut name_list2: &metamodelica::List<ArcStr>,
) -> metamodelica::List<ArcStr> {
    let mut out_names: metamodelica::List<ArcStr>;
    out_names = (::match_deref::match_deref! { match name_list2 {
        Deref @ metamodelica::ListNode::Nil => {
            list![prefix]
        },
        Deref @ metamodelica::ListNode::Cons { head: s, tail: rest } => {
            metamodelica::cons({ let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*prefix); ArcStr::from(__mm_s) }, buildAllComponentModifiers2(rest, name_list2))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out_names
}

fn parseAggregator(
    mut in_eq: &metamodelica::Ref<Absyn::Exp>,
    mut fargs: &metamodelica::Ref<Absyn::FunctionArgs>,
) -> metamodelica::Ref<Absyn::Exp> {
    let mut out_eq: metamodelica::Ref<Absyn::Exp>;
    out_eq = (match &**in_eq {
        Absyn::Exp::BINARY { exp1, op, exp2 } => {
            let mut nexp1: metamodelica::Ref<Absyn::Exp>;
            let mut nexp2: metamodelica::Ref<Absyn::Exp>;
            nexp1 = parseAggregator(exp1, fargs);
            nexp2 = parseAggregator(exp2, fargs);
            metamodelica::Ref::new(Absyn::Exp::BINARY {
                exp1: nexp1,
                op: op.clone(),
                exp2: nexp2,
            })
        }
        Absyn::Exp::LBINARY { exp1, op, exp2 } => {
            let mut nexp1: metamodelica::Ref<Absyn::Exp>;
            let mut nexp2: metamodelica::Ref<Absyn::Exp>;
            nexp1 = parseAggregator(exp1, fargs);
            nexp2 = parseAggregator(exp2, fargs);
            metamodelica::Ref::new(Absyn::Exp::LBINARY {
                exp1: nexp1,
                op: op.clone(),
                exp2: nexp2,
            })
        }
        Absyn::Exp::RELATION { exp1, op, exp2 } => {
            let mut nexp1: metamodelica::Ref<Absyn::Exp>;
            let mut nexp2: metamodelica::Ref<Absyn::Exp>;
            nexp1 = parseAggregator(exp1, fargs);
            nexp2 = parseAggregator(exp2, fargs);
            metamodelica::Ref::new(Absyn::Exp::RELATION {
                exp1: nexp1,
                op: op.clone(),
                exp2: nexp2,
            })
        }
        Absyn::Exp::UNARY { op, exp: exp2 } => {
            let mut nexp1: metamodelica::Ref<Absyn::Exp>;
            nexp1 = parseAggregator(exp2, fargs);
            metamodelica::Ref::new(Absyn::Exp::UNARY {
                op: op.clone(),
                exp: nexp1,
            })
        }
        Absyn::Exp::LUNARY { op, exp: exp2 } => {
            let mut nexp1: metamodelica::Ref<Absyn::Exp>;
            nexp1 = parseAggregator(exp2, fargs);
            metamodelica::Ref::new(Absyn::Exp::LUNARY {
                op: op.clone(),
                exp: nexp1,
            })
        }
        Absyn::Exp::IFEXP {
            ifExp: ife,
            trueBranch: exp1,
            elseBranch: exp2,
            elseIfBranch: elif,
        } => {
            let mut nexp1: metamodelica::Ref<Absyn::Exp>;
            let mut nexp2: metamodelica::Ref<Absyn::Exp>;
            let mut nife: metamodelica::Ref<Absyn::Exp>;
            nife = parseAggregator(ife, fargs);
            nexp1 = parseAggregator(exp1, fargs);
            nexp2 = parseAggregator(exp2, fargs);
            metamodelica::Ref::new(Absyn::Exp::IFEXP {
                ifExp: nife,
                trueBranch: nexp1,
                elseBranch: nexp2,
                elseIfBranch: elif.clone(),
            })
        }
        Absyn::Exp::CALL {
            function_: crf,
            typeVars: __in_eq_typeVars,
            ..
        } => metamodelica::Ref::new(Absyn::Exp::CALL {
            function_: crf.clone(),
            functionArgs: fargs.clone(),
            typeVars: __in_eq_typeVars.clone(),
        }),
        _ => in_eq.clone(),
    });
    out_eq
}

pub(crate) fn getProviders<'__b>(
    mut providers: &'__b metamodelica::List<Provider>,
    mut vmodel: &'__b metamodelica::Ref<Absyn::Class>,
    mut env: &'__b Absyn::Program,
    mut in_es: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, ArcStr)>,
) -> Result<metamodelica::List<(metamodelica::Ref<Absyn::Exp>, ArcStr)>> {
    '__tco: loop {
        ::match_deref::match_deref! { match providers {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(in_es)
            },
            Deref @ metamodelica::ListNode::Cons { head: Provider { modelID: className, component: _, template }, tail: rest } => {
                let mut comps: metamodelica::List<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, ArcStr)>;
                let mut exps: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, ArcStr)>;
                let mut new_es: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, ArcStr)>;
                let mut mlist: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                let mut exp: metamodelica::Ref<Absyn::Exp>;
                mlist = AbsynUtil::getElementItemsInClass(vmodel)?;
                comps = getAllProviderInstances(metamodelica::AsArg::as_arg(&className), metamodelica::AsArg::as_arg(&template), &mlist, env, &(metamodelica::nil()), &(literal!("")));
                let __pa0 = ::match_deref::match_deref! { match &(Parser::parsestringexp(template.clone(), literal!("<interactive>"))?) {
                    GlobalScript::Statements { interactiveStmtLst: Deref @ metamodelica::ListNode::Cons { head: GlobalScript::Statement::IEXP { exp: __pa0, info: _ }, tail: Deref @ metamodelica::ListNode::Nil }, semicolon: _ } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                exp = metamodelica::Own::own(__pa0);
                exps = applyTemplate(&exp, &comps, metamodelica::nil());
                new_es = listAppend(exps, in_es);
                { (providers, vmodel, env, in_es) = (rest, vmodel, env, new_es); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn applyTemplate<'__b>(
    mut exp: &'__b metamodelica::Ref<Absyn::Exp>,
    mut comps: &'__b metamodelica::List<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, ArcStr)>,
    mut in_es: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, ArcStr)>,
) -> metamodelica::List<(metamodelica::Ref<Absyn::Exp>, ArcStr)> {
    '__tco: loop {
        ::match_deref::match_deref! { match comps {
            Deref @ metamodelica::ListNode::Nil => {
                return in_es
            },
            Deref @ metamodelica::ListNode::Cons { head: (clist, pathInClass), tail: rest } => {
                let mut new_es: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, ArcStr)>;
                new_es = applyTemplate2(exp, metamodelica::AsArg::as_arg(&clist), in_es, metamodelica::AsArg::as_arg(&pathInClass));
                { (exp, comps, in_es) = (exp, rest, new_es); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { (exp, comps, in_es) = (exp, rest, in_es); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn applyTemplate2<'__b>(
    mut exp: &'__b metamodelica::Ref<Absyn::Exp>,
    mut comps: &'__b metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
    mut in_es: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, ArcStr)>,
    mut pathInClass: &'__b ArcStr,
) -> metamodelica::List<(metamodelica::Ref<Absyn::Exp>, ArcStr)> {
    '__tco: loop {
        ::match_deref::match_deref! { match comps {
            Deref @ metamodelica::ListNode::Nil => {
                return in_es
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name, arrayDim: _, modification: _ }, condition: _, comment: _ }, tail: rest } => {
                let mut newName: ArcStr;
                newName = if (metamodelica::stringEq(&pathInClass, &(literal!("")))) {name.clone()} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*pathInClass); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }};
                { (exp, comps, in_es, pathInClass) = (exp, rest, metamodelica::cons((parseExpression(exp, &newName), newName), in_es), pathInClass); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { (exp, comps, in_es, pathInClass) = (exp, rest, in_es, pathInClass); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn parseExpression(mut in_eq: &metamodelica::Ref<Absyn::Exp>, mut fargs: &ArcStr) -> metamodelica::Ref<Absyn::Exp> {
    let mut out_eq: metamodelica::Ref<Absyn::Exp>;
    out_eq = (match &**in_eq {
        Absyn::Exp::BINARY { exp1, op, exp2 } => {
            let mut nexp1: metamodelica::Ref<Absyn::Exp>;
            let mut nexp2: metamodelica::Ref<Absyn::Exp>;
            nexp1 = parseExpression(exp1, fargs);
            nexp2 = parseExpression(exp2, fargs);
            metamodelica::Ref::new(Absyn::Exp::BINARY {
                exp1: nexp1,
                op: op.clone(),
                exp2: nexp2,
            })
        }
        Absyn::Exp::LBINARY { exp1, op, exp2 } => {
            let mut nexp1: metamodelica::Ref<Absyn::Exp>;
            let mut nexp2: metamodelica::Ref<Absyn::Exp>;
            nexp1 = parseExpression(exp1, fargs);
            nexp2 = parseExpression(exp2, fargs);
            metamodelica::Ref::new(Absyn::Exp::LBINARY {
                exp1: nexp1,
                op: op.clone(),
                exp2: nexp2,
            })
        }
        Absyn::Exp::RELATION { exp1, op, exp2 } => {
            let mut nexp1: metamodelica::Ref<Absyn::Exp>;
            let mut nexp2: metamodelica::Ref<Absyn::Exp>;
            nexp1 = parseExpression(exp1, fargs);
            nexp2 = parseExpression(exp2, fargs);
            metamodelica::Ref::new(Absyn::Exp::RELATION {
                exp1: nexp1,
                op: op.clone(),
                exp2: nexp2,
            })
        }
        Absyn::Exp::UNARY { op, exp: exp2 } => {
            let mut nexp1: metamodelica::Ref<Absyn::Exp>;
            nexp1 = parseExpression(exp2, fargs);
            metamodelica::Ref::new(Absyn::Exp::UNARY {
                op: op.clone(),
                exp: nexp1,
            })
        }
        Absyn::Exp::LUNARY { op, exp: exp2 } => {
            let mut nexp1: metamodelica::Ref<Absyn::Exp>;
            nexp1 = parseExpression(exp2, fargs);
            metamodelica::Ref::new(Absyn::Exp::LUNARY {
                op: op.clone(),
                exp: nexp1,
            })
        }
        Absyn::Exp::IFEXP {
            ifExp: ife,
            trueBranch: exp1,
            elseBranch: exp2,
            elseIfBranch: elif,
        } => {
            let mut nexp1: metamodelica::Ref<Absyn::Exp>;
            let mut nexp2: metamodelica::Ref<Absyn::Exp>;
            let mut nife: metamodelica::Ref<Absyn::Exp>;
            nife = parseExpression(ife, fargs);
            nexp1 = parseExpression(exp1, fargs);
            nexp2 = parseExpression(exp2, fargs);
            metamodelica::Ref::new(Absyn::Exp::IFEXP {
                ifExp: nife,
                trueBranch: nexp1,
                elseBranch: nexp2,
                elseIfBranch: elif.clone(),
            })
        }
        Absyn::Exp::CREF { componentRef: crf } => {
            let mut new_crf: metamodelica::Ref<Absyn::ComponentRef>;
            new_crf = updateCRF(crf, fargs);
            metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: new_crf })
        }
        _ => in_eq.clone(),
    });
    out_eq
}

fn updateCRF<'__b>(
    mut componentRef: &'__b metamodelica::Ref<Absyn::ComponentRef>,
    mut name: &'__b ArcStr,
) -> metamodelica::Ref<Absyn::ComponentRef> {
    '__tco: loop {
        ::match_deref::match_deref! { match componentRef {
            Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cRef } => {
                { (componentRef, name) = (cRef, name); continue '__tco; }
            },
            Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "getPath", subscripts, componentRef: cRef } => {
                return metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: name.clone(), subscripts: subscripts.clone(), componentRef: cRef.clone() })
            },
            Deref @ Absyn::ComponentRef::CREF_QUAL { name: id, subscripts, componentRef: cRef } => {
                let mut new_cRef: metamodelica::Ref<Absyn::ComponentRef>;
                new_cRef = updateCRF(cRef, name);
                return metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: id.clone(), subscripts: subscripts.clone(), componentRef: new_cRef })
            },
            Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "getPath", subscripts } => {
                return metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: subscripts.clone() })
            },
            _ => {
                return componentRef.clone()
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn getAllProviderInstances(
    mut className: &ArcStr,
    mut template: &ArcStr,
    mut e_items: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut env: &Absyn::Program,
    mut in_components: &metamodelica::List<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, ArcStr)>,
    mut pathInClass: &ArcStr,
) -> metamodelica::List<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, ArcStr)> {
    let mut out_components: metamodelica::List<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, ArcStr)>;
    out_components = 'mc: {
        let __mc_input = &**e_items;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(in_components.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: _, redeclareKeywords: _, innerOuter: _, specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: _, typeSpec, components }, info: _, constrainClass: _ } }, tail: rest } => {
                    let mut re_items: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    let mut cnew: metamodelica::List<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, ArcStr)>;
                    let mut cnew2: metamodelica::List<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, ArcStr)>;
                    let mut path: Path;
                    let mut def: metamodelica::Ref<Absyn::Class>;
                    path = AbsynUtil::typeSpecPath(metamodelica::AsArg::as_arg(&typeSpec));
                    def = ProgramUtil::getPathedClassInProgram(path.clone(), env, false, false)?;
                    if metamodelica::stringEq(&(AbsynUtil::typeSpecPathString(metamodelica::AsArg::as_arg(&typeSpec))?), &className) {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("... found provider ")); __mm_s.push_str(&*className); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        cnew = metamodelica::cons((components.clone(), pathInClass.clone()), in_components.clone());
                    } else {
                        cnew = in_components.clone();
                    }
                    re_items = AbsynUtil::getElementItemsInClass(&def)?;
                    cnew2 = parseComponents(className, template, &re_items, env, metamodelica::AsArg::as_arg(&components), cnew.clone(), pathInClass);
                    Ok(getAllProviderInstances(className, template, metamodelica::AsArg::as_arg(&rest), env, &cnew2, pathInClass))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(getAllProviderInstances(className, template, metamodelica::AsArg::as_arg(&rest), env, in_components, pathInClass))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    out_components
}

fn parseComponents<'__b>(
    mut className: &'__b ArcStr,
    mut template: &'__b ArcStr,
    mut e_items: &'__b metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut env: &'__b Absyn::Program,
    mut components: &'__b metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
    mut in_components: metamodelica::List<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, ArcStr)>,
    mut pathInClass: &'__b ArcStr,
) -> metamodelica::List<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, ArcStr)> {
    '__tco: loop {
        ::match_deref::match_deref! { match components {
            Deref @ metamodelica::ListNode::Nil => {
                return in_components
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name, arrayDim: _, modification: _ }, condition: _, comment: _ }, tail: rest } => {
                let mut tmp: metamodelica::List<(metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>, ArcStr)>;
                let mut newName: ArcStr;
                newName = if (metamodelica::stringEq(&pathInClass, &(literal!("")))) {name.clone()} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*pathInClass); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }};
                tmp = getAllProviderInstances(className, template, e_items, env, &in_components, &newName);
                { (className, template, e_items, env, components, in_components, pathInClass) = (className, template, e_items, env, rest, tmp, pathInClass); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { (className, template, e_items, env, components, in_components, pathInClass) = (className, template, e_items, env, rest, in_components, pathInClass); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn buildInstList(
    mut clazz: &metamodelica::Ref<Absyn::Class>,
    mut env: &Absyn::Program,
    mut predecessors: &metamodelica::Ref<Client_e>,
    mut mediators: &metamodelica::List<Mediator>,
    mut client_list_in: &metamodelica::List<metamodelica::Ref<Client_e>>,
    mut instance_list: &metamodelica::List<metamodelica::List<ArcStr>>,
) -> Result<metamodelica::List<metamodelica::Ref<Client_e>>> {
    let mut client_list: metamodelica::List<metamodelica::Ref<Client_e>>;
    let mut e_items: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    e_items = AbsynUtil::getElementItemsInClass(clazz)?;
    client_list = parseElementInstList(
        &e_items,
        env,
        &(crate::Binding::Client_e::interned_NO_PRED()),
        mediators,
        client_list_in,
        instance_list,
    );
    Ok(client_list)
}

fn buildInstList2(
    mut clazz: &metamodelica::Ref<Absyn::Class>,
    mut env: &Absyn::Program,
    mut predecessors: &metamodelica::Ref<Client_e>,
    mut mediators: &metamodelica::List<Mediator>,
    mut client_list_in: &metamodelica::List<metamodelica::Ref<Client_e>>,
    mut instance_list: &metamodelica::List<metamodelica::List<ArcStr>>,
    mut rootType: &TypeSpec,
) -> Result<metamodelica::List<metamodelica::Ref<Client_e>>> {
    let mut client_list: metamodelica::List<metamodelica::Ref<Client_e>>;
    let mut e_items: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
    e_items = AbsynUtil::getElementItemsInClass(clazz)?;
    client_list = parseElementInstList2(
        &e_items,
        env,
        &(crate::Binding::Client_e::interned_NO_PRED()),
        mediators,
        client_list_in,
        instance_list,
        rootType,
    );
    Ok(client_list)
}

fn isAlreadyInList<'__b>(
    mut ts: &'__b metamodelica::Ref<Absyn::TypeSpec>,
    mut predecessors: &'__b metamodelica::List<metamodelica::Ref<Client_e>>,
) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match predecessors {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(false)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Client_e::CLIENT_E { components: _, typeSpec: _, rootType: ots, def: _, instance: _, predecessors: _, mediator: _ }, tail: rest } => {
                if (AbsynUtil::typeSpecEqual(ts, metamodelica::AsArg::as_arg(&ots))?) {return Ok(true)} else {{ (ts, predecessors) = (ts, rest); continue '__tco; }}
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn parseElementInstList(
    mut e_items: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut env: &Absyn::Program,
    mut predecessors: &metamodelica::Ref<Client_e>,
    mut mediators: &metamodelica::List<Mediator>,
    mut in_client_list: &metamodelica::List<metamodelica::Ref<Client_e>>,
    mut instance_list: &metamodelica::List<metamodelica::List<ArcStr>>,
) -> metamodelica::List<metamodelica::Ref<Client_e>> {
    let mut client_list: metamodelica::List<metamodelica::Ref<Client_e>>;
    client_list = 'mc: {
        let __mc_input = &**e_items;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(in_client_list.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: _, redeclareKeywords: _, innerOuter: _, specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: _, typeSpec, components }, info: _, constrainClass: _ } }, tail: rest } => {
                    let mut path: Path;
                    let mut iname: ArcStr;
                    let mut new_predecessors: metamodelica::Ref<Client_e>;
                    let mut def: metamodelica::Ref<Absyn::Class>;
                    let mut l1: metamodelica::List<metamodelica::Ref<Client_e>>;
                    let mut l2: metamodelica::List<metamodelica::Ref<Client_e>>;
                    let mut isCl: bool;
                    let mut m: metamodelica::List<Mediator>;
                    path = AbsynUtil::typeSpecPath(metamodelica::AsArg::as_arg(&typeSpec));
                    def = ProgramUtil::getPathedClassInProgram(path.clone(), env, false, false)?;
                    (isCl, iname, m) = isClient(AbsynUtil::typeSpecPathString(metamodelica::AsArg::as_arg(&typeSpec))?, mediators, metamodelica::nil())?;
                    if isCl && !(isAlreadyInList(metamodelica::AsArg::as_arg(&typeSpec), in_client_list)?) {
                        new_predecessors = metamodelica::Ref::new(Client_e::CLIENT_E { components: components.clone(), typeSpec: typeSpec.clone(), rootType: typeSpec.clone(), def: def.clone(), instance: metamodelica::cons(list![iname.clone()], instance_list.clone()), predecessors: predecessors.clone(), mediator: m.clone() });
                        l2 = metamodelica::cons(new_predecessors.clone(), in_client_list.clone());
                    } else {
                        new_predecessors = predecessors.clone();
                        l2 = in_client_list.clone();
                    }
                    l1 = buildInstList2(&def, env, &new_predecessors, mediators, &l2, instance_list, metamodelica::AsArg::as_arg(&typeSpec))?;
                    Ok(parseElementInstList(metamodelica::AsArg::as_arg(&rest), env, predecessors, mediators, &l1, instance_list))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(parseElementInstList(metamodelica::AsArg::as_arg(&rest), env, predecessors, mediators, in_client_list, instance_list))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    client_list
}

fn getComponentNames(
    mut l: &metamodelica::List<metamodelica::Ref<Absyn::ComponentItem>>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    res = (::match_deref::match_deref! { match l {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: ci, tail: r } => {
            metamodelica::cons(AbsynUtil::componentName(metamodelica::AsArg::as_arg(&ci))?, getComponentNames(r)?)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

fn parseElementInstList2(
    mut e_items: &metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>,
    mut env: &Absyn::Program,
    mut predecessors: &metamodelica::Ref<Client_e>,
    mut mediators: &metamodelica::List<Mediator>,
    mut in_client_list: &metamodelica::List<metamodelica::Ref<Client_e>>,
    mut instance_list: &metamodelica::List<metamodelica::List<ArcStr>>,
    mut rootType: &TypeSpec,
) -> metamodelica::List<metamodelica::Ref<Client_e>> {
    let mut client_list: metamodelica::List<metamodelica::Ref<Client_e>>;
    client_list = 'mc: {
        let __mc_input = &**e_items;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(in_client_list.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: _, redeclareKeywords: _, innerOuter: _, specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: _, typeSpec, components }, info: _, constrainClass: _ } }, tail: rest } => {
                    let mut path: Path;
                    let mut iname: ArcStr;
                    let mut new_predecessors: metamodelica::Ref<Client_e>;
                    let mut def: metamodelica::Ref<Absyn::Class>;
                    let mut l1: metamodelica::List<metamodelica::Ref<Client_e>>;
                    let mut l2: metamodelica::List<metamodelica::Ref<Client_e>>;
                    let mut isCl: bool;
                    let mut m: metamodelica::List<Mediator>;
                    path = AbsynUtil::typeSpecPath(metamodelica::AsArg::as_arg(&typeSpec));
                    def = ProgramUtil::getPathedClassInProgram(path.clone(), env, false, false)?;
                    (isCl, iname, m) = isClient(AbsynUtil::typeSpecPathString(metamodelica::AsArg::as_arg(&typeSpec))?, mediators, metamodelica::nil())?;
                    if isCl {
                        new_predecessors = metamodelica::Ref::new(Client_e::CLIENT_E { components: components.clone(), typeSpec: typeSpec.clone(), rootType: rootType.clone(), def: def.clone(), instance: metamodelica::cons(list![iname.clone()], metamodelica::cons(getComponentNames(metamodelica::AsArg::as_arg(&components))?, instance_list.clone())), predecessors: predecessors.clone(), mediator: m.clone() });
                        l2 = metamodelica::cons(new_predecessors.clone(), in_client_list.clone());
                    } else {
                        new_predecessors = predecessors.clone();
                        l2 = in_client_list.clone();
                    }
                    l1 = buildInstList2(&def, env, &new_predecessors, mediators, &l2, &(metamodelica::cons(getComponentNames(metamodelica::AsArg::as_arg(&components))?, instance_list.clone())), rootType)?;
                    Ok(parseElementInstList2(metamodelica::AsArg::as_arg(&rest), env, predecessors, mediators, &l1, instance_list, rootType))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(parseElementInstList2(metamodelica::AsArg::as_arg(&rest), env, predecessors, mediators, in_client_list, instance_list, rootType))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    client_list
}

fn isClient(
    mut ci_name: ArcStr,
    mut mediators: &metamodelica::List<Mediator>,
    mut in_m: metamodelica::List<Mediator>,
) -> Result<(bool, ArcStr, metamodelica::List<Mediator>)> {
    let mut isClient: bool;
    let mut iname: ArcStr;
    let mut m: metamodelica::List<Mediator>;
    (isClient, iname, m) = 'mc: {
        let __mc_input = &**mediators;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((false, literal!(""), in_m.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Mediator { mType, template, clients, providers, preferred }, tail: _ } => {
                    let mut nm: ArcStr;
                    let (true, __pa0) = (isClientInMediator(&ci_name, metamodelica::AsArg::as_arg(&clients))) else { return Err("pattern mismatch") };
                    nm = metamodelica::Own::own(__pa0);
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("... found client : ")); __mm_s.push_str(&*ci_name); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok((true, nm.clone(), metamodelica::cons(Mediator { mType: mType.clone(), template: template.clone(), clients: clients.clone(), providers: providers.clone(), preferred: preferred.clone() }, in_m.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(self::isClient(ci_name.clone(), metamodelica::AsArg::as_arg(&rest), in_m.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((isClient, iname, m))
}

fn isClientInMediator(mut ci_name: &ArcStr, mut clients: &metamodelica::List<Client>) -> (bool, ArcStr) {
    let mut isClient: bool;
    let mut iname: ArcStr;
    (isClient, iname) = 'mc: {
        let __mc_input = &**clients;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((false, literal!("")))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Client { modelID: name, component: inst, template: _, isMandatory: _ }, tail: _ } => {
                    let true = (metamodelica::stringEq(&name, &ci_name)) else { return Err("pattern mismatch") };
                    Ok((true, inst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(isClientInMediator(ci_name, metamodelica::AsArg::as_arg(&rest)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (isClient, iname)
}

fn getMediatorDefsElements<'__b>(
    mut mediator_defs: &'__b metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut mediators_in: metamodelica::List<Mediator>,
) -> metamodelica::List<Mediator> {
    '__tco: loop {
        ::match_deref::match_deref! { match mediator_defs {
            Deref @ metamodelica::ListNode::Nil => {
                return mediators_in
            },
            Deref @ metamodelica::ListNode::Cons { head: el, tail: rest } => {
                let mut m: metamodelica::List<Mediator>;
                m = listAppend(getMediatorDefsElement(metamodelica::AsArg::as_arg(&el)), mediators_in);
                { (mediator_defs, mediators_in) = (rest, m); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn getMediatorDefsElement(mut el: &metamodelica::Ref<SCode::Element>) -> metamodelica::List<Mediator> {
    let mut mediator: metamodelica::List<Mediator>;
    mediator = 'mc: {
        let __mc_input = &**el;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: _, prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::Restriction::R_PACKAGE { .. }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elist, normalEquationLst: _, initialEquationLst: _, normalAlgorithmLst: _, initialAlgorithmLst: _, constraintLst: _, clsattrs: _, externalDecl: _ }, cmt: _, info: _ } => {
                    Ok(getMediatorDefsElements(metamodelica::AsArg::as_arg(&elist), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: _, prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::Restriction::R_RECORD { isOperator: _ }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elist, normalEquationLst: _, initialEquationLst: _, normalAlgorithmLst: _, initialAlgorithmLst: _, constraintLst: _, clsattrs: _, externalDecl: _ }, cmt: _, info: _ } => {
                    let mut r#mod: metamodelica::Ref<SCode::Mod>;
                    let mut cMod: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut pMod: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut prMod: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut template: ArcStr;
                    let mut mType: ArcStr;
                    let mut str1: ArcStr;
                    let mut str2: ArcStr;
                    let mut cls: metamodelica::List<Client>;
                    let mut prvs: metamodelica::List<Provider>;
                    let mut pref: metamodelica::List<Preferred>;
                    let __pa0 = ::match_deref::match_deref! { match &(extendsType(metamodelica::AsArg::as_arg(&elist), &(literal!("Mediator")))) {
                        (true, Some(__pa0)) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    r#mod = metamodelica::Own::own(__pa0);
                    let __pa1 = ::match_deref::match_deref! { match &(getValue(&r#mod, &(literal!("template")), &(literal!("string")))?) {
                        Deref @ Absyn::Exp::STRING { value: __pa1 } => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    template = metamodelica::Own::own(__pa1);
                    str1 = System::stringReplace(template.clone(), literal!("%"), literal!(""))?;
                    str2 = System::stringReplace(str1.clone(), literal!(":"), literal!("all"))?;
                    let __pa2 = ::match_deref::match_deref! { match &(getValue(&r#mod, &(literal!("mType")), &(literal!("string")))?) {
                        Deref @ Absyn::Exp::STRING { value: __pa2 } => __pa2.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    mType = metamodelica::Own::own(__pa2);
                    let __pa3 = ::match_deref::match_deref! { match &(getValue(&r#mod, &(literal!("clients")), &(literal!("array")))?) {
                        Deref @ Absyn::Exp::ARRAY { arrayExp: __pa3 } => __pa3.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cMod = metamodelica::Own::own(__pa3);
                    cls = getClientList(&cMod, metamodelica::nil())?;
                    let __pa4 = ::match_deref::match_deref! { match &(getValue(&r#mod, &(literal!("providers")), &(literal!("array")))?) {
                        Deref @ Absyn::Exp::ARRAY { arrayExp: __pa4 } => __pa4.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    pMod = metamodelica::Own::own(__pa4);
                    prvs = getProviderList(&pMod, metamodelica::nil())?;
                    let __pa5 = ::match_deref::match_deref! { match &(getValue(&r#mod, &(literal!("preferred")), &(literal!("array")))?) {
                        Deref @ Absyn::Exp::ARRAY { arrayExp: __pa5 } => __pa5.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    prMod = metamodelica::Own::own(__pa5);
                    pref = getPreferredList(&prMod, metamodelica::nil())?;
                    Ok(list![Mediator { mType: mType.clone(), template: str2.clone(), clients: cls.clone(), providers: prvs.clone(), preferred: pref.clone() }])
                }
                _ => return Err("nomatch"),
            }}
        })() {
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
    };
    mediator
}

fn getPreferredList<'__b>(
    mut e: &'__b metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut val: metamodelica::List<Preferred>,
) -> Result<metamodelica::List<Preferred>> {
    '__tco: loop {
        ::match_deref::match_deref! { match e {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(val)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CALL { function_: _, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: _, argNames }, .. }, tail: rest } => {
                let mut clientInstancePath: ArcStr;
                let mut providerInstancePath: ArcStr;
                clientInstancePath = getArg(metamodelica::AsArg::as_arg(&argNames), &(literal!("clientInstancePath")));
                providerInstancePath = getArg(metamodelica::AsArg::as_arg(&argNames), &(literal!("providerInstancePath")));
                { (e, val) = (rest, metamodelica::cons(Preferred { clientInstancePath: clientInstancePath, providerInstancePath: providerInstancePath }, val)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getClientList<'__b>(
    mut e: &'__b metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut val: metamodelica::List<Client>,
) -> Result<metamodelica::List<Client>> {
    '__tco: loop {
        ::match_deref::match_deref! { match e {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(val)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CALL { function_: _, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: _, argNames }, .. }, tail: rest } => {
                let mut className: ArcStr;
                let mut instance: ArcStr;
                let mut template: ArcStr;
                let mut isM: ArcStr;
                let mut isMandatory: bool;
                className = getArg(metamodelica::AsArg::as_arg(&argNames), &(literal!("modelID")));
                instance = getArg(metamodelica::AsArg::as_arg(&argNames), &(literal!("component")));
                template = getArg(metamodelica::AsArg::as_arg(&argNames), &(literal!("template")));
                isM = getArg(metamodelica::AsArg::as_arg(&argNames), &(literal!("isMandatory")));
                if metamodelica::stringEq(&isM, &(literal!("true"))) {
                    isMandatory = true;
                } else {
                    isMandatory = false;
                }
                { (e, val) = (rest, metamodelica::cons(Client { modelID: className, component: instance, template: template, isMandatory: isMandatory }, val)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getProviderList<'__b>(
    mut e: &'__b metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut val: metamodelica::List<Provider>,
) -> Result<metamodelica::List<Provider>> {
    '__tco: loop {
        ::match_deref::match_deref! { match e {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(val)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CALL { function_: _, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: _, argNames }, .. }, tail: rest } => {
                let mut className: ArcStr;
                let mut providerTemplate: ArcStr;
                let mut instance: ArcStr;
                className = getArg(metamodelica::AsArg::as_arg(&argNames), &(literal!("modelID")));
                instance = getArg(metamodelica::AsArg::as_arg(&argNames), &(literal!("component")));
                providerTemplate = getArg(metamodelica::AsArg::as_arg(&argNames), &(literal!("template")));
                { (e, val) = (rest, metamodelica::cons(Provider { modelID: className, component: instance, template: providerTemplate }, val)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getArg(mut argNames: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, mut name: &ArcStr) -> ArcStr {
    let mut val: ArcStr;
    val = 'mc: {
        let __mc_input = &**argNames;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::NamedArg { argName: nname, argValue: Deref @ Absyn::Exp::STRING { value: r#str } }, tail: _ } => {
                    let mut str1: ArcStr;
                    let mut str2: ArcStr;
                    str1 = System::stringReplace(r#str.clone(), literal!("%"), literal!(""))?;
                    str2 = System::stringReplace(str1.clone(), literal!(":"), literal!("all"))?;
                    let true = (metamodelica::stringEq(&nname, &name)) else { return Err("pattern mismatch") };
                    Ok(str2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(getArg(metamodelica::AsArg::as_arg(&rest), name))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    val
}

fn extendsType<'__b>(
    mut elems: &'__b metamodelica::List<metamodelica::Ref<SCode::Element>>,
    mut typeName: &'__b ArcStr,
) -> (bool, Option<metamodelica::Ref<SCode::Mod>>) {
    '__tco: loop {
        ::match_deref::match_deref! { match elems {
            Deref @ metamodelica::ListNode::Nil => {
                return (false, None)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: Deref @ Absyn::Path::IDENT { name: tName }, visibility: _, modifications: r#mod, ann: _, info: _ }, tail: _ } if (metamodelica::stringEq(&tName, &typeName)) => {
                return (true, Some(r#mod.clone()))
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { (elems, typeName) = (rest, typeName); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn getValue(
    mut r#mod: &metamodelica::Ref<SCode::Mod>,
    mut name: &Ident,
    mut retype: &ArcStr,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut val: metamodelica::Ref<Absyn::Exp>;
    val = (match &**r#mod {
        SCode::Mod::MOD {
            finalPrefix: _,
            eachPrefix: _,
            subModLst: smod,
            binding: _,
            comment: _,
            ..
        } => getValueR(smod, name, retype)?,
        _ => return Err("match: no arm matched"),
    });
    Ok(val)
}

fn getValueR(
    mut smod: &metamodelica::List<metamodelica::Ref<SCode::SubMod>>,
    mut name: &Ident,
    mut retype: &ArcStr,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut val: metamodelica::Ref<Absyn::Exp>;
    val = 'mc: {
        let __mc_input = (&**smod, retype.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ "bool") => {
                    Ok(metamodelica::Ref::new(Absyn::Exp::BOOL { value: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ "array") => {
                    Ok(metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ "string") => {
                    Ok(metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::SubMod { ident: n, r#mod: Deref @ SCode::Mod::MOD { finalPrefix: _, eachPrefix: _, subModLst: _, binding: Some(eval), comment: _, .. } }, tail: _ }, _) => {
                    if !metamodelica::stringEq(&n, &name) {
                        return Err("fail");
                    }
                    Ok(eval.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, _) => {
                    Ok(getValueR(metamodelica::AsArg::as_arg(&rest), name, retype)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(val)
}
