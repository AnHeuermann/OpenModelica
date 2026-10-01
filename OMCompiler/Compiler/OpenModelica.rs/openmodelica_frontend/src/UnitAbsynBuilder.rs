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

use crate::FGraph;
use crate::FNode;
use crate::Lookup;
use crate::UnitAbsyn;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::HashTable;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_script_util::UnitParserExt;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Flags;
use openmodelica_util::MMath;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub(crate) fn registerUnitWeights(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut dae: &DAE::DAElist,
) -> Result<()> {
    let mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut du: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let () = (match dae.clone() {
        _ => (),
        DAE::DAElist { elementLst: ref elts } => {
            paths = List::unionList(
                &(List::map(
                    elts.clone(),
                    &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(DAEUtil::getClassList(&__a0))
                    },
                )?),
            )?;
            du = List::unionList(
                &(List::map1(
                    paths,
                    &move |__a0: metamodelica::Ref<Absyn::Path>,
                           __a1: (FCore::Cache, FCore::Graph)|
                          -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(retrieveUnitsFromEnv(&__a0, __a1))
                    },
                    (cache, env),
                )?),
            )?;
            registerUnitWeightDefineunits(&du);
            ()
        }
    });
    Ok(())
}

fn retrieveUnitsFromEnv(
    mut p: &metamodelica::Ref<Absyn::Path>,
    mut tpl: (FCore::Cache, FCore::Graph),
) -> metamodelica::List<metamodelica::Ref<SCode::Element>> {
    let mut du: metamodelica::List<metamodelica::Ref<SCode::Element>> = metamodelica::nil();
    du = 'mc: {
        let __mc_input = tpl.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut env: FCore::Graph;
            let mut r: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
            let mut du: metamodelica::List<metamodelica::Ref<SCode::Element>> = du.clone();
            (_, _, env) = Lookup::lookupClass(&(Util::tuple21(tpl.clone())), &(Util::tuple22(tpl.clone())), p, None)?;
            r = FGraph::lastScopeRef(&env)?;
            r = FNode::child(r.clone(), arcstr::literal!(FNode::duNodeName))?;
            let __pa0 = ::match_deref::match_deref! { match &(FNode::fromRef(r.clone())) {
                Deref @ FCore::Node { data: Deref @ FCore::Data::DU { els: __pa0 }, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            du = metamodelica::Own::own(__pa0);
            Ok((du.clone(), du.clone()))
        })() {
            du = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(metamodelica::nil())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    du
}

fn registerUnitWeightDefineunits(mut du: &metamodelica::List<metamodelica::Ref<SCode::Element>>) -> () {
    let () = (::match_deref::match_deref! { match du {
        Deref @ metamodelica::ListNode::Nil => {
            registerUnitWeightDefineunits2(&(list![metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("m"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: None, weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("kg"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: None, weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("s"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: None, weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("A"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: None, weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("k"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: None, weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("mol"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: None, weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("cd"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: None, weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("rad"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some(literal!("m/m")), weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("sr"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some(literal!("m2/m2")), weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("Hz"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some(literal!("s-1")), weight: Some(metamodelica::OrderedFloat(0.8_f64)), info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("N"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some(literal!("m.kg.s-2")), weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("Pa"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some(literal!("N/m2")), weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("W"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some(literal!("J/s")), weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("J"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some(literal!("N.m")), weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("C"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some(literal!("s.A")), weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("V"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some(literal!("W/A")), weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("F"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some(literal!("C/V")), weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("Ohm"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some(literal!("V/A")), weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("S"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some(literal!("A/V")), weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("Wb"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some(literal!("V.s")), weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("T"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some(literal!("Wb/m2")), weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("H"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some(literal!("Wb/A")), weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("lm"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some(literal!("cd.sr")), weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("lx"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some(literal!("lm/m2")), weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("Bq"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some(literal!("s-1")), weight: Some(metamodelica::OrderedFloat(0.8_f64)), info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("Gy"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some(literal!("J/kg")), weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("Sv"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some(literal!("cd.sr")), weight: None, info: SCodeUtil::dummyInfo.clone() }), metamodelica::Ref::new(SCode::Element::DEFINEUNIT { name: literal!("kat"), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, exp: Some(literal!("s-1.mol")), weight: None, info: SCodeUtil::dummyInfo.clone() })]));
            ()
        },
        _ => {
            registerUnitWeightDefineunits2(du);
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

fn registerUnitWeightDefineunits2(mut idu: &metamodelica::List<metamodelica::Ref<SCode::Element>>) -> () {
    let () = (::match_deref::match_deref! { match idu {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::DEFINEUNIT { name: n, weight: Some(w), .. }, tail: du } => {
            UnitParserExt::registerWeight(n.clone(), w.clone());
            registerUnitWeightDefineunits2(du);
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::DEFINEUNIT { weight: None, .. }, tail: du } => {
            registerUnitWeightDefineunits2(du);
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: du } => {
            registerUnitWeightDefineunits2(du);
            ()
        },
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ()
}

pub(crate) fn registerUnits(mut prg: &Absyn::Program) -> Result<()> {
    let () = (match prg.clone() {
        _ => (),
        _ => {
            let false = (Flags::getConfigBool(Flags::UNIT_CHECKING.clone())?) else {
                return Err("pattern mismatch");
            };
            ()
        }
    });
    Ok(())
}

fn registerUnitInClass(
    mut inTpl: &(
        metamodelica::Ref<Absyn::Class>,
        Option<metamodelica::Ref<Absyn::Path>>,
        i32,
    ),
) -> (
    metamodelica::Ref<Absyn::Class>,
    Option<metamodelica::Ref<Absyn::Path>>,
    i32,
) {
    let mut outTpl: (
        metamodelica::Ref<Absyn::Class>,
        Option<metamodelica::Ref<Absyn::Path>>,
        i32,
    );
    outTpl = 'mc: {
        let __mc_input = inTpl;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cl @ Deref @ Absyn::Class { .. }, pa, i) => {
                    let mut defunits: metamodelica::List<metamodelica::Ref<Absyn::Element>>;
                    let mut elts: metamodelica::List<metamodelica::Ref<Absyn::ElementItem>>;
                    elts = AbsynUtil::getElementItemsInClass(metamodelica::AsArg::as_arg(&cl))?;
                    defunits = AbsynUtil::getDefineUnitsInElements(&elts);
                    registerDefineunits(&defunits)?;
                    Ok((cl.clone(), pa.clone(), i.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cl, pa, i) => {
                    Ok((cl.clone(), pa.clone(), i.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outTpl
}

fn registerDefineunits(mut elts: &metamodelica::List<metamodelica::Ref<Absyn::Element>>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**elts;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    registerDefineunits2(&(list![metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("m"), args: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("kg"), args: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("s"), args: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("A"), args: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("k"), args: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("mol"), args: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("cd"), args: metamodelica::nil(), info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("rad"), args: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("exp"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("m/m") }) })], info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("sr"), args: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("exp"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("m2/m2") }) })], info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("Hz"), args: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("exp"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("s-1") }) }), metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("weight"), argValue: metamodelica::Ref::new(Absyn::Exp::REAL { value: literal!("0.8") }) })], info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("N"), args: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("exp"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("m.kg.s-2") }) })], info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("Pa"), args: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("exp"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("N/m2") }) })], info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("W"), args: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("exp"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("J/s") }) })], info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("J"), args: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("exp"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("N.m") }) })], info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("C"), args: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("exp"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("s.A") }) })], info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("V"), args: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("exp"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("W/A") }) })], info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("F"), args: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("exp"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("C/V") }) })], info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("Ohm"), args: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("exp"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("V/A") }) })], info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("S"), args: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("exp"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("A/V") }) })], info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("Wb"), args: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("exp"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("V.s") }) })], info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("T"), args: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("exp"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("Wb/m2") }) })], info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("H"), args: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("exp"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("Wb/A") }) })], info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("lm"), args: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("exp"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("cd.sr") }) })], info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("lx"), args: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("exp"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("lm/m2") }) })], info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("Bq"), args: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("exp"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("s-1") }) }), metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("weight"), argValue: metamodelica::Ref::new(Absyn::Exp::REAL { value: literal!("0.8") }) })], info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("Gy"), args: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("exp"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("J/kg") }) })], info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("Sv"), args: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("exp"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("cd.sr") }) })], info: Absyn::dummyInfo.clone() }), metamodelica::Ref::new(Absyn::Element::DEFINEUNIT { name: literal!("kat"), args: list![metamodelica::Ref::new(Absyn::NamedArg { argName: literal!("exp"), argValue: metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("s-1.mol") }) })], info: Absyn::dummyInfo.clone() })]))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    registerDefineunits2(elts)?;
                    Ok(())
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

fn registerDefineunits2(mut elts: &metamodelica::List<metamodelica::Ref<Absyn::Element>>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**elts;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: du @ Deref @ Absyn::Element::DEFINEUNIT { .. }, tail: rest } => {
                    let mut exp: ArcStr;
                    let mut name: ArcStr;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(AbsynToSCode::translateElement(metamodelica::AsArg::as_arg(&du), openmodelica_frontend_types::SCode::Visibility::PUBLIC)?) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::DEFINEUNIT { name: __pa0, visibility: _, exp: Some(__pa1), weight: _, .. }, tail: Deref @ metamodelica::ListNode::Nil } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    name = metamodelica::Own::own(__pa0);
                    exp = metamodelica::Own::own(__pa1);
                    UnitParserExt::addDerived(name.clone(), exp.clone());
                    registerDefineunits2(metamodelica::AsArg::as_arg(&rest))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: du @ Deref @ Absyn::Element::DEFINEUNIT { .. }, tail: rest } => {
                    let mut name: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(AbsynToSCode::translateElement(metamodelica::AsArg::as_arg(&du), openmodelica_frontend_types::SCode::Visibility::PUBLIC)?) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Element::DEFINEUNIT { name: __pa0, visibility: _, exp: None, weight: _, .. }, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    name = metamodelica::Own::own(__pa0);
                    UnitParserExt::addBase(name.clone());
                    registerDefineunits2(metamodelica::AsArg::as_arg(&rest))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("registerDefineunits failed\n"));
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

pub(crate) fn add(mut unit: &UnitAbsyn::Unit, mut ist: UnitAbsyn::Store) -> Result<(UnitAbsyn::Store, i32)> {
    let mut outSt: UnitAbsyn::Store;
    let mut index: i32 = 0;
    (outSt, index) = 'mc: {
        let __mc_input = ist;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let ref st @ UnitAbsyn::Store {
                storeVector: ref vector,
                numElts: ref numElts,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut st = st.clone();
            let mut index: i32 = index.clone();
            let true = (numElts.clone() == metamodelica::arrayLength(vector.clone())) else {
                return Err("pattern mismatch");
            };
            st = expandStore(&(st.clone()))?;
            (st, index) = add(unit, st.clone())?;
            Ok(((st.clone(), index), index.clone()))
        })() {
            index = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let UnitAbsyn::Store {
                storeVector: mut vector,
                numElts: mut numElts,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut newIndx: i32;
            newIndx = numElts.clone() + 1;
            vector = metamodelica::arrayUpdate(vector.clone(), newIndx, Some(unit.clone()))?;
            Ok((
                UnitAbsyn::Store {
                    storeVector: vector.clone(),
                    numElts: newIndx,
                },
                newIndx,
            ))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outSt, index))
}

pub(crate) fn updateInstStore(mut store: &UnitAbsyn::InstStore, mut st: UnitAbsyn::Store) -> UnitAbsyn::InstStore {
    let mut outStore: UnitAbsyn::InstStore;
    outStore = (match store.clone() {
        UnitAbsyn::InstStore::INSTSTORE {
            store: _,
            ht: mut ht,
            checkResult: mut res,
        } => UnitAbsyn::InstStore::INSTSTORE {
            store: st,
            ht: ht.clone(),
            checkResult: res.clone(),
        },
        UnitAbsyn::InstStore::NOSTORE { .. } => crate::UnitAbsyn::InstStore::NOSTORE,
    });
    outStore
}

fn expandStore(mut st: &UnitAbsyn::Store) -> Result<UnitAbsyn::Store> {
    let mut outSt: UnitAbsyn::Store;
    outSt = (match st.clone() {
        UnitAbsyn::Store {
            storeVector: mut vector,
            numElts: mut indx,
        } => {
            let mut incr: i32;
            incr = intMin(
                1,
                ((intReal(indx.clone()) * metamodelica::OrderedFloat(0.4_f64)).0.floor() as i32),
            );
            vector = Array::expand(incr, vector.clone(), None)?;
            UnitAbsyn::Store {
                storeVector: vector.clone(),
                numElts: indx.clone(),
            }
        }
    });
    Ok(outSt)
}

pub(crate) fn update(mut unit: UnitAbsyn::Unit, mut index: i32, mut st: &UnitAbsyn::Store) -> Result<UnitAbsyn::Store> {
    let mut outSt: UnitAbsyn::Store;
    outSt = 'mc: {
        let __mc_input = st.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let UnitAbsyn::Store {
                storeVector: mut vector,
                numElts: mut indx,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            vector = metamodelica::arrayUpdate(vector.clone(), index, Some(unit.clone()))?;
            Ok(UnitAbsyn::Store {
                storeVector: vector.clone(),
                numElts: indx.clone(),
            })
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("storing unit at index "));
            metamodelica::print(intString(index));
            metamodelica::print(literal!(" failed\n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outSt)
}

pub(crate) fn find(mut index: i32, mut st: &UnitAbsyn::Store) -> Result<UnitAbsyn::Unit> {
    let mut unit: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
    unit = 'mc: {
        let __mc_input = st.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let UnitAbsyn::Store {
                storeVector: mut vector,
                numElts: _,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut unit: UnitAbsyn::Unit = unit.clone();
            let __pa0 = ::match_deref::match_deref! { match &(({let __elt = (*metamodelica::index_checked(&vector.borrow(), index)?).clone(); __elt})) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            unit = metamodelica::Own::own(__pa0);
            Ok((unit.clone(), unit.clone()))
        })() {
            unit = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!(" finding store at index "));
            metamodelica::print(intString(index));
            metamodelica::print(literal!(" failed\n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(unit)
}

pub(crate) fn instGetStore(mut store: &UnitAbsyn::InstStore) -> UnitAbsyn::Store {
    let mut st: UnitAbsyn::Store;
    st = (match store.clone() {
        UnitAbsyn::InstStore::INSTSTORE {
            store: mut __esc_st,
            ht: _,
            checkResult: _,
        } => {
            st = __esc_st.clone();
            st
        }
        UnitAbsyn::InstStore::NOSTORE { .. } => emptyStore(),
    });
    st
}

pub(crate) fn emptyInstStore() -> UnitAbsyn::InstStore {
    let mut st: UnitAbsyn::InstStore;
    st = emptyInstStore2(false);
    st
}

fn emptyInstStore2(mut wantInstStore: bool) -> UnitAbsyn::InstStore {
    let mut st: UnitAbsyn::InstStore;
    st = (match wantInstStore {
        true => {
            let mut s: UnitAbsyn::Store;
            let mut ht: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                ),
                i32,
                (
                    HashTable::FuncHashCref,
                    HashTable::FuncCrefEqual,
                    HashTable::FuncCrefStr,
                    HashTable::FuncExpStr,
                ),
            );
            s = emptyStore();
            ht = HashTable::emptyHashTable();
            UnitAbsyn::InstStore::INSTSTORE {
                store: s,
                ht: ht,
                checkResult: None,
            }
        }
        _ => UnitAbsyn::noStore().clone(),
    });
    st
}

pub(crate) fn emptyStore() -> UnitAbsyn::Store {
    let mut st: UnitAbsyn::Store;
    let mut vector: metamodelica::Array<Option<UnitAbsyn::Unit>>;
    vector = arrayCreate(10, None);
    st = UnitAbsyn::Store {
        storeVector: vector.clone(),
        numElts: 0,
    };
    st
}

pub(crate) fn printTerms(mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>) -> Result<()> {
    metamodelica::print(printTermsStr(terms)?);
    Ok(())
}

pub(crate) fn printTermsStr(mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("{"));
        __mm_s.push_str(&*stringDelimitList(
            List::map(terms, &move |__a0: metamodelica::Ref<UnitAbsyn::UnitTerm>| {
                printTermStr(&__a0)
            })?,
            literal!(","),
        ));
        __mm_s.push_str(&*literal!("}"));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

pub(crate) fn printTermStr(mut term: &metamodelica::Ref<UnitAbsyn::UnitTerm>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match &**term {
        UnitAbsyn::UnitTerm::ADD {
            ut1: _,
            ut2: _,
            origExp: e,
        } => {
            let mut s1: ArcStr;
            s1 = ExpressionBasics::printExpStr(e.clone())?;
            s1
        }
        UnitAbsyn::UnitTerm::SUB {
            ut1: _,
            ut2: _,
            origExp: e,
        } => {
            let mut s1: ArcStr;
            s1 = ExpressionBasics::printExpStr(e.clone())?;
            s1
        }
        UnitAbsyn::UnitTerm::MUL {
            ut1: _,
            ut2: _,
            origExp: e,
        } => {
            let mut s1: ArcStr;
            s1 = ExpressionBasics::printExpStr(e.clone())?;
            s1
        }
        UnitAbsyn::UnitTerm::DIV {
            ut1: _,
            ut2: _,
            origExp: e,
        } => {
            let mut s1: ArcStr;
            s1 = ExpressionBasics::printExpStr(e.clone())?;
            s1
        }
        UnitAbsyn::UnitTerm::EQN {
            ut1: _,
            ut2: _,
            origExp: e,
        } => {
            let mut s1: ArcStr;
            s1 = ExpressionBasics::printExpStr(e.clone())?;
            s1
        }
        UnitAbsyn::UnitTerm::LOC { loc: _, origExp: e } => {
            let mut s1: ArcStr;
            s1 = ExpressionBasics::printExpStr(e.clone())?;
            s1
        }
        UnitAbsyn::UnitTerm::POW {
            ut1: _,
            exponent: MMath::Rational { nom: _, denom: _ },
            origExp: e,
        } => {
            let mut s1: ArcStr;
            s1 = ExpressionBasics::printExpStr(e.clone())?;
            s1
        }
    });
    Ok(r#str)
}

pub(crate) fn printInstStore(mut st: &UnitAbsyn::InstStore) -> Result<()> {
    let () = (match st.clone() {
        UnitAbsyn::InstStore::INSTSTORE {
            store: mut s,
            ht: mut h,
            checkResult: _,
        } => {
            metamodelica::print(literal!("instStore, s:"));
            printStore(metamodelica::AsArg::as_arg(&s))?;
            metamodelica::print(literal!("\nht:"));
            BaseHashTable::dumpHashTable(&(h.clone()))?;
            ()
        }
        UnitAbsyn::InstStore::NOSTORE { .. } => (),
    });
    Ok(())
}

pub(crate) fn printStore(mut st: &UnitAbsyn::Store) -> Result<()> {
    let () = (match st.clone() {
        UnitAbsyn::Store {
            storeVector: mut vector,
            numElts: _,
        } => {
            let mut lst: metamodelica::List<Option<UnitAbsyn::Unit>>;
            lst = vector
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<metamodelica::List<_>>();
            printStore2(&lst, 1)?;
            ()
        }
    });
    Ok(())
}

fn printStore2(mut lst: &metamodelica::List<Option<UnitAbsyn::Unit>>, mut indx: i32) -> Result<()> {
    let () = (::match_deref::match_deref! { match lst {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: Some(unit), tail: rest } => {
            metamodelica::print(intString(indx));
            metamodelica::print(literal!("->"));
            printUnit(metamodelica::AsArg::as_arg(&unit))?;
            metamodelica::print(literal!("\n"));
            printStore2(rest, indx + 1)?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: None, tail: _ } => {
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn printUnit(mut unit: &UnitAbsyn::Unit) -> Result<()> {
    let () = 'mc: {
        let __mc_input = unit;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                UnitAbsyn::Unit::SPECIFIED { specified: UnitAbsyn::SpecUnit { typeParameters: Deref @ metamodelica::ListNode::Nil, units: baseunits } } => {
                    metamodelica::print(printBaseUnitsStr(metamodelica::AsArg::as_arg(&baseunits)));
                    metamodelica::print(literal!(" ["));
                    metamodelica::print(unit2str(unit)?);
                    metamodelica::print(literal!("]"));
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                UnitAbsyn::Unit::SPECIFIED { specified: UnitAbsyn::SpecUnit { typeParameters: typeparams, units: baseunits } } => {
                    metamodelica::print(stringDelimitList(List::map(typeparams.clone(), &move |__a0: (MMath::Rational, UnitAbsyn::TypeParameter)| printTypeParameterStr(&__a0))?, literal!(",")));
                    metamodelica::print(printBaseUnitsStr(metamodelica::AsArg::as_arg(&baseunits)));
                    metamodelica::print(literal!(" ["));
                    metamodelica::print(unit2str(unit)?);
                    metamodelica::print(literal!("]"));
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                UnitAbsyn::Unit::UNSPECIFIED { .. } => {
                    metamodelica::print(literal!("Unspecified"));
                    Ok(())
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

fn printBaseUnitsStr(mut lst: &metamodelica::List<MMath::Rational>) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (::match_deref::match_deref! { match lst {
        Deref @ metamodelica::ListNode::Cons { head: MMath::Rational { nom: i1, denom: i2 }, tail: Deref @ metamodelica::ListNode::Cons { head: MMath::Rational { nom: i3, denom: i4 }, tail: _ } } => {
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("m^(")); __mm_s.push_str(&*intString(i1.clone())); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString(i2.clone())); __mm_s.push_str(&*literal!(")")); __mm_s.push_str(&*literal!("s^(")); __mm_s.push_str(&*intString(i3.clone())); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString(i4.clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
            r#str
        },
        Deref @ metamodelica::ListNode::Nil => {
            literal!("")
        },
        _ => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("printBaseUnitsStr failed len:")); __mm_s.push_str(&*intString(((lst).len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    r#str
}

fn printTypeParameterStr(mut typeParam: &(MMath::Rational, UnitAbsyn::TypeParameter)) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match typeParam.clone() {
        (
            MMath::Rational { nom: 0, denom: 0 },
            UnitAbsyn::TypeParameter {
                name: mut name,
                indx: mut indx,
            },
        ) => {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!("[indx ="));
                __mm_s.push_str(&*intString(indx.clone()));
                __mm_s.push_str(&*literal!("]"));
                ArcStr::from(__mm_s)
            };
            r#str
        }
        (
            MMath::Rational { nom: mut i1, denom: 1 },
            UnitAbsyn::TypeParameter {
                name: mut name,
                indx: mut indx,
            },
        ) => {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!("^"));
                __mm_s.push_str(&*intString(i1.clone()));
                __mm_s.push_str(&*literal!("[indx="));
                __mm_s.push_str(&*intString(indx.clone()));
                __mm_s.push_str(&*literal!("]"));
                ArcStr::from(__mm_s)
            };
            r#str
        }
        (
            MMath::Rational {
                nom: mut i1,
                denom: mut i2,
            },
            UnitAbsyn::TypeParameter {
                name: mut name,
                indx: mut indx,
            },
        ) => {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!("^("));
                __mm_s.push_str(&*intString(i1.clone()));
                __mm_s.push_str(&*literal!("/"));
                __mm_s.push_str(&*intString(i2.clone()));
                __mm_s.push_str(&*literal!(")"));
                __mm_s.push_str(&*literal!("[indx="));
                __mm_s.push_str(&*intString(indx.clone()));
                __mm_s.push_str(&*literal!("]"));
                ArcStr::from(__mm_s)
            };
            r#str
        }
    });
    Ok(r#str)
}

pub(crate) fn splitRationals(
    mut inRationals: &metamodelica::List<MMath::Rational>,
) -> Result<(metamodelica::List<i32>, metamodelica::List<i32>)> {
    let mut nums: metamodelica::List<i32>;
    let mut denoms: metamodelica::List<i32>;
    (nums, denoms) = (::match_deref::match_deref! { match inRationals {
        Deref @ metamodelica::ListNode::Nil => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: MMath::Rational { nom: i1, denom: i2 }, tail: rationals } => {
            (nums, denoms) = splitRationals(rationals)?;
            (metamodelica::cons(i1.clone(), nums), metamodelica::cons(i2.clone(), denoms))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((nums, denoms))
}

pub(crate) fn joinRationals(
    mut inums: &metamodelica::List<i32>,
    mut idenoms: &metamodelica::List<i32>,
) -> Result<metamodelica::List<MMath::Rational>> {
    let mut rationals: metamodelica::List<MMath::Rational>;
    rationals = (::match_deref::match_deref! { match (inums, idenoms) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::ListNode::Cons { head: i1, tail: nums }, Deref @ metamodelica::ListNode::Cons { head: i2, tail: denoms }) => {
            rationals = joinRationals(nums, denoms)?;
            metamodelica::cons(MMath::Rational { nom: i1.clone(), denom: i2.clone() }, rationals)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(rationals)
}

pub(crate) fn joinTypeParams(
    mut inums: &metamodelica::List<i32>,
    mut idenoms: &metamodelica::List<i32>,
    mut itpstrs: &metamodelica::List<ArcStr>,
    mut funcInstIdOpt: Option<i32>,
) -> Result<metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>> {
    let mut typeParams: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
    typeParams = (::match_deref::match_deref! { match (inums, idenoms, itpstrs) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::ListNode::Cons { head: i1, tail: nums }, Deref @ metamodelica::ListNode::Cons { head: i2, tail: denoms }, Deref @ metamodelica::ListNode::Cons { head: tpParam, tail: tpstrs }) => {
            let mut s: ArcStr;
            let mut tpParam = (*tpParam).clone();
            typeParams = joinTypeParams(nums, denoms, tpstrs, funcInstIdOpt.clone())?;
            s = Util::applyOptionOrDefault(funcInstIdOpt, &fnptr!(intString, i32), literal!(""))?;
            tpParam = { let mut __mm_s = String::new(); __mm_s.push_str(&*tpParam); __mm_s.push_str(&*s); ArcStr::from(__mm_s) };
            metamodelica::cons((MMath::Rational { nom: i1.clone(), denom: i2.clone() }, UnitAbsyn::TypeParameter { name: tpParam.clone(), indx: 0 }), typeParams)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(typeParams)
}

pub(crate) fn splitTypeParams(
    mut iTypeParams: &metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>,
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<ArcStr>,
)> {
    let mut nums: metamodelica::List<i32>;
    let mut denoms: metamodelica::List<i32>;
    let mut tpstrs: metamodelica::List<ArcStr>;
    (nums, denoms, tpstrs) = (::match_deref::match_deref! { match iTypeParams {
        Deref @ metamodelica::ListNode::Nil => {
            (metamodelica::nil(), metamodelica::nil(), metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: (MMath::Rational { nom: i1, denom: i2 }, UnitAbsyn::TypeParameter { name: tpParam, indx: _ }), tail: typeParams } => {
            (nums, denoms, tpstrs) = splitTypeParams(typeParams)?;
            (metamodelica::cons(i1.clone(), nums), metamodelica::cons(i2.clone(), denoms), metamodelica::cons(tpParam.clone(), tpstrs))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((nums, denoms, tpstrs))
}

pub(crate) fn instBuildUnitTerms(
    mut env: &FCore::Graph,
    mut dae: &DAE::DAElist,
    mut compDae: &DAE::DAElist,
    mut store: &UnitAbsyn::InstStore,
) -> Result<(
    UnitAbsyn::InstStore,
    metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>,
)> {
    let mut outStore: UnitAbsyn::InstStore;
    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>> = metamodelica::nil();
    (outStore, terms) = 'mc: {
        let __mc_input = store.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let UnitAbsyn::InstStore::NOSTORE { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok((crate::UnitAbsyn::InstStore::NOSTORE, metamodelica::nil()))
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let UnitAbsyn::InstStore::INSTSTORE {
                store: mut st,
                ht: mut ht,
                checkResult: mut res,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut terms2: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
            let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>> = terms.clone();
            (terms, st) = buildTerms(env, dae, &(ht.clone()), st.clone())?;
            (terms2, st) = buildTerms(env, compDae, &(ht.clone()), st.clone())?;
            terms = terms.clone().reverse();
            terms = List::append_reverse(&terms2, terms.clone());
            st = createTypeParameterLocations(st.clone())?;
            Ok((
                (
                    UnitAbsyn::InstStore::INSTSTORE {
                        store: st.clone(),
                        ht: ht.clone(),
                        checkResult: res.clone(),
                    },
                    terms.clone(),
                ),
                terms.clone(),
            ))
        })() {
            terms = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            metamodelica::print(literal!("instBuildUnitTerms failed!!\n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outStore, terms))
}

pub(crate) fn buildUnitTerms(
    mut env: &FCore::Graph,
    mut dae: &DAE::DAElist,
) -> Result<(
    metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>,
    UnitAbsyn::Store,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
    let mut store: UnitAbsyn::Store;
    let mut ht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            HashTable::FuncHashCref,
            HashTable::FuncCrefEqual,
            HashTable::FuncCrefStr,
            HashTable::FuncExpStr,
        ),
    );
    (store, ht) = buildStores(dae)?;
    (terms, store) = buildTerms(env, dae, &ht, store)?;
    store = createTypeParameterLocations(store)?;
    Ok((terms, store, ht))
}

pub(crate) fn instAddStore(
    mut istore: UnitAbsyn::InstStore,
    mut itp: &metamodelica::Ref<DAE::Type>,
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
) -> UnitAbsyn::InstStore {
    let mut outStore: UnitAbsyn::InstStore = UnitAbsyn::InstStore::NOSTORE;
    outStore = 'mc: {
        let __mc_input = (istore.clone(), &**itp);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (UnitAbsyn::InstStore::NOSTORE { .. }, _) => {
                    Ok(istore.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (UnitAbsyn::InstStore::INSTSTORE { store: st, ht, checkResult: res }, Deref @ DAE::Type::T_REAL { varLst }) => {
                            let mut unitStr: ArcStr;
                            let mut unit: UnitAbsyn::Unit;
                            let mut indx: i32;
                            let mut st = (*st).clone();
                            let mut ht = (*ht).clone();
                            let mut outStore: UnitAbsyn::InstStore = outStore.clone();
                            for mut v in &*varLst.clone() {
                                let () = (::match_deref::match_deref! { match &(v.clone()) {
                Deref @ DAE::Var { name: Deref @ "unit", binding: Deref @ DAE::Binding::EQBOUND { exp: Deref @ DAE::Exp::SCONST { string: __esc_unitStr }, .. }, .. } => {
                            unitStr = (*__esc_unitStr).clone();
                            unit = str2unit(unitStr.clone(), None)?;
                            unit = if (0 == stringCompare(&unitStr, &(literal!("")))) {crate::UnitAbsyn::Unit::UNSPECIFIED} else {unit.clone()};
                            (st, indx) = add(&unit, st.clone())?;
                            ht = BaseHashTable::add((cr.clone(), indx), ht.clone())?;
                            outStore = UnitAbsyn::InstStore::INSTSTORE { store: st.clone(), ht: ht.clone(), checkResult: res.clone() };
                            return Ok((outStore.clone(), outStore.clone()));
                            ()
                },
                _ => (),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                            }
                            (st, indx) = add(&(crate::UnitAbsyn::Unit::UNSPECIFIED), st.clone())?;
                            ht = BaseHashTable::add((cr.clone(), indx), ht.clone())?;
                            Ok((UnitAbsyn::InstStore::INSTSTORE { store: st.clone(), ht: ht.clone(), checkResult: res.clone() }, outStore.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            outStore = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (store, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: tp, .. }) => {
                    Ok(instAddStore(store.clone(), metamodelica::AsArg::as_arg(&tp), cr))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(istore.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outStore
}

pub(crate) fn storeSize(mut store: &UnitAbsyn::Store) -> i32 {
    let mut size: i32;
    size = (match store.clone() {
        UnitAbsyn::Store {
            storeVector: _,
            numElts: mut __esc_size,
        } => {
            size = __esc_size.clone();
            size
        }
    });
    size
}

fn createTypeParameterLocations(mut store: UnitAbsyn::Store) -> Result<UnitAbsyn::Store> {
    let mut outStore: UnitAbsyn::Store;
    let mut nextElement: i32;
    let mut storeSz: i32;
    storeSz = storeSize(&store);
    (outStore, _, nextElement) = createTypeParameterLocations2(store, HashTable::emptyHashTable(), 1, storeSz + 1)?;
    outStore = addUnspecifiedStores(nextElement - storeSz - 1, outStore)?;
    Ok(outStore)
}

fn addUnspecifiedStores(mut n: i32, mut istore: UnitAbsyn::Store) -> Result<UnitAbsyn::Store> {
    let mut outStore: UnitAbsyn::Store;
    outStore = 'mc: {
        let __mc_input = (n, istore);
        if let Ok(__v) = (|| -> Result<_> {
            let (0, mut store) = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(store.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (n < 0) else { return Err("pattern mismatch") };
            metamodelica::print(literal!("addUnspecifiedStores n < 0!\n"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, mut store) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (n > 0) else { return Err("pattern mismatch") };
            (store, _) = add(&(crate::UnitAbsyn::Unit::UNSPECIFIED), store.clone())?;
            store = addUnspecifiedStores(n - 1, store.clone())?;
            Ok(store.clone())
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStore)
}

fn createTypeParameterLocations2(
    mut istore: UnitAbsyn::Store,
    mut iht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut i: i32,
    mut inextElt: i32,
) -> Result<(
    UnitAbsyn::Store,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    i32,
)> {
    let mut outStore: UnitAbsyn::Store;
    let mut outHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            HashTable::FuncHashCref,
            HashTable::FuncCrefEqual,
            HashTable::FuncCrefStr,
            HashTable::FuncExpStr,
        ),
    );
    let mut outNextElt: i32;
    (outStore, outHt, outNextElt) = 'mc: {
        let __mc_input = (istore, iht, inextElt);
        if let Ok(__v) = (|| -> Result<_> {
            let (
                ref store @ UnitAbsyn::Store {
                    storeVector: _,
                    numElts: ref numElts,
                },
                mut ht,
                mut nextElt,
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let true = (i > numElts.clone()) else {
                return Err("pattern mismatch");
            };
            Ok((store.clone(), ht.clone(), nextElt.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (
                UnitAbsyn::Store {
                    storeVector: mut vect,
                    numElts: mut numElts,
                },
                mut ht,
                mut nextElt,
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut unit: UnitAbsyn::Unit;
            let mut store: UnitAbsyn::Store;
            let __pa0 = ::match_deref::match_deref! { match &(({let __elt = (*metamodelica::index_checked(&vect.borrow(), i)?).clone(); __elt})) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            unit = metamodelica::Own::own(__pa0);
            (unit, ht, nextElt) = createTypeParameterLocations3(&unit, ht.clone(), nextElt.clone())?;
            vect = metamodelica::arrayUpdate(vect.clone(), i, Some(unit.clone()))?;
            (store, ht, nextElt) = createTypeParameterLocations2(
                UnitAbsyn::Store {
                    storeVector: vect.clone(),
                    numElts: numElts.clone(),
                },
                ht.clone(),
                i + 1,
                nextElt.clone(),
            )?;
            Ok((store.clone(), ht.clone(), nextElt.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (
                UnitAbsyn::Store {
                    storeVector: mut vect,
                    numElts: mut numElts,
                },
                mut ht,
                mut nextElt,
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut store: UnitAbsyn::Store;
            (store, ht, nextElt) = createTypeParameterLocations2(
                UnitAbsyn::Store {
                    storeVector: vect.clone(),
                    numElts: numElts.clone(),
                },
                ht.clone(),
                i + 1,
                nextElt.clone(),
            )?;
            Ok((store.clone(), ht.clone(), nextElt.clone()))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outStore, outHt, outNextElt))
}

fn createTypeParameterLocations3(
    mut unit: &UnitAbsyn::Unit,
    mut iht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inextElt: i32,
) -> Result<(
    UnitAbsyn::Unit,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    i32,
)> {
    let mut outUnit: UnitAbsyn::Unit;
    let mut outHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            HashTable::FuncHashCref,
            HashTable::FuncCrefEqual,
            HashTable::FuncCrefStr,
            HashTable::FuncExpStr,
        ),
    );
    let mut outNextElt: i32;
    (outUnit, outHt, outNextElt) = (::match_deref::match_deref! { match &(unit) {
        UnitAbsyn::Unit::SPECIFIED { specified: UnitAbsyn::SpecUnit { typeParameters: params @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, units } } => {
            let mut ht = iht;
            let mut nextElt = inextElt;
            let mut params = (*params).clone();
            (params, ht, nextElt) = createTypeParameterLocations4(metamodelica::AsArg::as_arg(&params), ht, nextElt)?;
            (UnitAbsyn::Unit::SPECIFIED { specified: UnitAbsyn::SpecUnit { typeParameters: params.clone(), units: units.clone() } }, ht, nextElt)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outUnit, outHt, outNextElt))
}

fn createTypeParameterLocations4(
    mut iparams: &metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>,
    mut iht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inextElt: i32,
) -> Result<(
    metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    i32,
)> {
    let mut outParams: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
    let mut outHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            HashTable::FuncHashCref,
            HashTable::FuncCrefEqual,
            HashTable::FuncCrefStr,
            HashTable::FuncExpStr,
        ),
    );
    let mut outNextElt: i32;
    (outParams, outHt, outNextElt) = 'mc: {
        let __mc_input = (&**iparams, iht, inextElt);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, ht, nextElt) => {
                    Ok((metamodelica::nil(), ht.clone(), nextElt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (r, UnitAbsyn::TypeParameter { name, indx: 0 }), tail: params }, ht, nextElt) => {
                    let mut indx: i32;
                    let mut cref_: metamodelica::Ref<DAE::ComponentRef>;
                    let mut params = (*params).clone();
                    let mut ht = (*ht).clone();
                    let mut nextElt = (*nextElt).clone();
                    cref_ = ComponentReferenceBasics::makeCrefIdent(name.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                    indx = BaseHashTable::get(cref_.clone(), &(ht.clone()))?;
                    (params, ht, nextElt) = createTypeParameterLocations4(metamodelica::AsArg::as_arg(&params), ht.clone(), nextElt.clone())?;
                    Ok((metamodelica::cons((r.clone(), UnitAbsyn::TypeParameter { name: name.clone(), indx: indx }), params.clone()), ht.clone(), nextElt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (r, UnitAbsyn::TypeParameter { name, indx: 0 }), tail: params }, ht, nextElt) => {
                    let mut cref_: metamodelica::Ref<DAE::ComponentRef>;
                    let mut params = (*params).clone();
                    let mut ht = (*ht).clone();
                    let mut nextElt = (*nextElt).clone();
                    cref_ = ComponentReferenceBasics::makeCrefIdent(name.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                    ht = BaseHashTable::add((cref_.clone(), nextElt.clone()), ht.clone())?;
                    (params, ht, nextElt) = createTypeParameterLocations4(metamodelica::AsArg::as_arg(&params), ht.clone(), nextElt.clone())?;
                    Ok((metamodelica::cons((r.clone(), UnitAbsyn::TypeParameter { name: name.clone(), indx: nextElt.clone() }), params.clone()), ht.clone(), nextElt.clone() + 1))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: param, tail: params }, ht, nextElt) => {
                    let mut params = (*params).clone();
                    let mut ht = (*ht).clone();
                    let mut nextElt = (*nextElt).clone();
                    (params, ht, nextElt) = createTypeParameterLocations4(metamodelica::AsArg::as_arg(&params), ht.clone(), nextElt.clone())?;
                    Ok((metamodelica::cons(param.clone(), params.clone()), ht.clone(), nextElt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("createTypeParameterLocations4 failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outParams, outHt, outNextElt))
}

fn buildStores(
    mut dae: &DAE::DAElist,
) -> Result<(
    UnitAbsyn::Store,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut store: UnitAbsyn::Store;
    let mut ht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            HashTable::FuncHashCref,
            HashTable::FuncCrefEqual,
            HashTable::FuncCrefStr,
            HashTable::FuncExpStr,
        ),
    );
    (store, ht) = buildStores2(dae, &(emptyStore()), &(HashTable::emptyHashTable()))?;
    (store, ht) = buildStores3(dae, store, ht)?;
    Ok((store, ht))
}

fn buildTerms(
    mut env: &FCore::Graph,
    mut dae: &DAE::DAElist,
    mut ht: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut istore: UnitAbsyn::Store,
) -> Result<(
    metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>,
    UnitAbsyn::Store,
)> {
    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::Store;
    (terms, outStore) = 'mc: {
        let __mc_input = (dae, istore);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Nil }, store) => {
                    Ok((metamodelica::nil(), store.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUATION { exp: e1, scalar: e2, source: _ }, tail: elts } }, store) => {
                    let mut ut1: metamodelica::Ref<UnitAbsyn::UnitTerm>;
                    let mut ut2: metamodelica::Ref<UnitAbsyn::UnitTerm>;
                    let mut terms1: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut terms2: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut store = (*store).clone();
                    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>> = terms.clone();
                    (ut1, terms1, store) = buildTermExp(env, e1.clone(), false, ht.clone(), store.clone())?;
                    (ut2, terms2, store) = buildTermExp(env, e2.clone(), false, ht.clone(), store.clone())?;
                    (terms, store) = buildTerms(env, &(DAE::DAElist { elementLst: elts.clone() }), ht, store.clone())?;
                    terms = listAppend(terms1.clone(), listAppend(terms2.clone(), terms.clone()));
                    Ok(((metamodelica::cons(metamodelica::Ref::new(UnitAbsyn::UnitTerm::EQN { ut1: ut1.clone(), ut2: ut2.clone(), origExp: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e2.clone() }) }), terms.clone()), store.clone()), terms.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            terms = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUEQUATION { cr1, cr2, source: _ }, tail: elts } }, store) => {
                    let mut crefExp1: metamodelica::Ref<DAE::Exp>;
                    let mut crefExp2: metamodelica::Ref<DAE::Exp>;
                    let mut ut1: metamodelica::Ref<UnitAbsyn::UnitTerm>;
                    let mut ut2: metamodelica::Ref<UnitAbsyn::UnitTerm>;
                    let mut terms1: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut terms2: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut store = (*store).clone();
                    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>> = terms.clone();
                    crefExp1 = Expression::crefExp(cr1.clone())?;
                    crefExp2 = Expression::crefExp(cr2.clone())?;
                    (ut1, terms1, store) = buildTermExp(env, crefExp1.clone(), false, ht.clone(), store.clone())?;
                    (ut2, terms2, store) = buildTermExp(env, crefExp2.clone(), false, ht.clone(), store.clone())?;
                    (terms, store) = buildTerms(env, &(DAE::DAElist { elementLst: elts.clone() }), ht, store.clone())?;
                    terms = listAppend(terms1.clone(), listAppend(terms2.clone(), terms.clone()));
                    Ok(((metamodelica::cons(metamodelica::Ref::new(UnitAbsyn::UnitTerm::EQN { ut1: ut1.clone(), ut2: ut2.clone(), origExp: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: crefExp1.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: crefExp2.clone() }) }), terms.clone()), store.clone()), terms.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            terms = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { componentRef: cr1 @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: _ }, binding: Some(e1), .. }, tail: elts } }, store) => {
                    let mut crefExp1: metamodelica::Ref<DAE::Exp>;
                    let mut ut1: metamodelica::Ref<UnitAbsyn::UnitTerm>;
                    let mut ut2: metamodelica::Ref<UnitAbsyn::UnitTerm>;
                    let mut terms1: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut terms2: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut store = (*store).clone();
                    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>> = terms.clone();
                    crefExp1 = Expression::crefExp(cr1.clone())?;
                    (ut1, terms1, store) = buildTermExp(env, crefExp1.clone(), false, ht.clone(), store.clone())?;
                    (ut2, terms2, store) = buildTermExp(env, e1.clone(), false, ht.clone(), store.clone())?;
                    (terms, store) = buildTerms(env, &(DAE::DAElist { elementLst: elts.clone() }), ht, store.clone())?;
                    terms = listAppend(terms1.clone(), listAppend(terms2.clone(), terms.clone()));
                    Ok(((metamodelica::cons(metamodelica::Ref::new(UnitAbsyn::UnitTerm::EQN { ut1: ut1.clone(), ut2: ut2.clone(), origExp: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: crefExp1.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e1.clone() }) }), terms.clone()), store.clone()), terms.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            terms = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::DEFINE { componentRef: cr1, exp: e1, source: _ }, tail: elts } }, store) => {
                    let mut crefExp1: metamodelica::Ref<DAE::Exp>;
                    let mut ut1: metamodelica::Ref<UnitAbsyn::UnitTerm>;
                    let mut ut2: metamodelica::Ref<UnitAbsyn::UnitTerm>;
                    let mut terms1: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut terms2: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut store = (*store).clone();
                    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>> = terms.clone();
                    crefExp1 = Expression::crefExp(cr1.clone())?;
                    (ut1, terms1, store) = buildTermExp(env, crefExp1.clone(), false, ht.clone(), store.clone())?;
                    (ut2, terms2, store) = buildTermExp(env, e1.clone(), false, ht.clone(), store.clone())?;
                    (terms, store) = buildTerms(env, &(DAE::DAElist { elementLst: elts.clone() }), ht, store.clone())?;
                    terms = listAppend(terms1.clone(), listAppend(terms2.clone(), terms.clone()));
                    Ok(((metamodelica::cons(metamodelica::Ref::new(UnitAbsyn::UnitTerm::EQN { ut1: ut1.clone(), ut2: ut2.clone(), origExp: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: crefExp1.clone(), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e1.clone() }) }), terms.clone()), store.clone()), terms.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            terms = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: elts } }, store) => {
                    let mut store = (*store).clone();
                    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>> = terms.clone();
                    (terms, store) = buildTerms(env, &(DAE::DAElist { elementLst: elts.clone() }), ht, store.clone())?;
                    Ok(((terms.clone(), store.clone()), terms.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            terms = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((terms, outStore))
}

fn buildTermExp(
    mut env: &FCore::Graph,
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut idivOrMul: bool,
    mut iht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut istore: UnitAbsyn::Store,
) -> Result<(
    metamodelica::Ref<UnitAbsyn::UnitTerm>,
    metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>,
    UnitAbsyn::Store,
)> {
    let mut ut: metamodelica::Ref<UnitAbsyn::UnitTerm> =
        <metamodelica::Ref<UnitAbsyn::UnitTerm> as ::std::default::Default>::default();
    let mut extraTerms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
    let mut outStore: UnitAbsyn::Store;
    (ut, extraTerms, outStore) = 'mc: {
        let __mc_input = (exp, idivOrMul, iht, istore);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::ICONST { integer: i }, divOrMul, ht, store) => {
                    let mut indx: i32;
                    let mut s1: ArcStr;
                    let mut u: UnitAbsyn::Unit;
                    let mut ht = (*ht).clone();
                    let mut store = (*store).clone();
                    s1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*intString(tick())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) };
                    u = if (divOrMul.clone()) {str2unit(literal!("1"), None)?} else {crate::UnitAbsyn::Unit::UNSPECIFIED};
                    (store, indx) = add(&u, store.clone())?;
                    ht = BaseHashTable::add((ComponentReferenceBasics::makeCrefIdent(s1.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()), indx), ht.clone())?;
                    Ok((metamodelica::Ref::new(UnitAbsyn::UnitTerm::LOC { loc: indx, origExp: e.clone() }), metamodelica::nil(), store.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RCONST { real: r }, divOrMul, ht, store) => {
                    let mut indx: i32;
                    let mut s1: ArcStr;
                    let mut u: UnitAbsyn::Unit;
                    let mut ht = (*ht).clone();
                    let mut store = (*store).clone();
                    s1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$")); __mm_s.push_str(&*intString(tick())); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*realString(r.clone())); ArcStr::from(__mm_s) };
                    u = if (divOrMul.clone()) {str2unit(literal!("1"), None)?} else {crate::UnitAbsyn::Unit::UNSPECIFIED};
                    (store, indx) = add(&u, store.clone())?;
                    ht = BaseHashTable::add((ComponentReferenceBasics::makeCrefIdent(s1.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()), indx), ht.clone())?;
                    Ok((metamodelica::Ref::new(UnitAbsyn::UnitTerm::LOC { loc: indx, origExp: e.clone() }), metamodelica::nil(), store.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CAST { ty: _, exp: e1 }, divOrMul, ht, store) => {
                    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut store = (*store).clone();
                    let mut ut: metamodelica::Ref<UnitAbsyn::UnitTerm> = ut.clone();
                    (ut, terms, store) = buildTermExp(env, e1.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    Ok(((ut.clone(), terms.clone(), store.clone()), ut.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            ut = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, _, ht, store) => {
                    let mut indx: i32;
                    indx = BaseHashTable::get(cr.clone(), &(ht.clone()))?;
                    Ok((metamodelica::Ref::new(UnitAbsyn::UnitTerm::LOC { loc: indx, origExp: e.clone() }), metamodelica::nil(), store.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: _ }, exp2: e2 @ Deref @ DAE::Exp::ICONST { integer: i } }, divOrMul, ht, store) => {
                    let mut ut1: metamodelica::Ref<UnitAbsyn::UnitTerm>;
                    let mut terms1: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut terms2: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut store = (*store).clone();
                    let mut ut: metamodelica::Ref<UnitAbsyn::UnitTerm> = ut.clone();
                    (ut1, terms1, store) = buildTermExp(env, e1.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    (_, terms2, store) = buildTermExp(env, e2.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    terms = listAppend(terms1.clone(), terms2.clone());
                    ut = metamodelica::Ref::new(UnitAbsyn::UnitTerm::POW { ut1: ut1.clone(), exponent: MMath::Rational { nom: i.clone(), denom: 1 }, origExp: e.clone() });
                    Ok(((ut.clone(), terms.clone(), store.clone()), ut.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            ut = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: _ }, exp2: e2 @ Deref @ DAE::Exp::RCONST { real: r } }, divOrMul, ht, store) => {
                    let mut i: i32;
                    let mut ut1: metamodelica::Ref<UnitAbsyn::UnitTerm>;
                    let mut terms1: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut terms2: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut store = (*store).clone();
                    let mut ut: metamodelica::Ref<UnitAbsyn::UnitTerm> = ut.clone();
                    (ut1, terms1, store) = buildTermExp(env, e1.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    (_, terms2, store) = buildTermExp(env, e2.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    terms = listAppend(terms1.clone(), terms2.clone());
                    i = ((r.clone()).0.floor() as i32);
                    let true = (intReal(i) - r.clone() == metamodelica::OrderedFloat(0.0_f64)) else { return Err("pattern mismatch") };
                    ut = metamodelica::Ref::new(UnitAbsyn::UnitTerm::POW { ut1: ut1.clone(), exponent: MMath::Rational { nom: i, denom: 1 }, origExp: e.clone() });
                    Ok(((ut.clone(), terms.clone(), store.clone()), ut.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            ut = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 }, divOrMul, ht, store) => {
                    let mut ut1: metamodelica::Ref<UnitAbsyn::UnitTerm>;
                    let mut ut2: metamodelica::Ref<UnitAbsyn::UnitTerm>;
                    let mut terms1: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut terms2: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut divOrMul = (*divOrMul).clone();
                    let mut store = (*store).clone();
                    let mut ut: metamodelica::Ref<UnitAbsyn::UnitTerm> = ut.clone();
                    divOrMul = Expression::operatorDivOrMul(metamodelica::AsArg::as_arg(&op));
                    (ut1, terms1, store) = buildTermExp(env, e1.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    (ut2, terms2, store) = buildTermExp(env, e2.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    terms = listAppend(terms1.clone(), terms2.clone());
                    ut = buildTermOp(ut1.clone(), ut2.clone(), metamodelica::AsArg::as_arg(&op), e.clone())?;
                    Ok(((ut.clone(), terms.clone(), store.clone()), ut.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            ut = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: _ }, divOrMul, ht, store) => {
                    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut divOrMul = (*divOrMul).clone();
                    let mut store = (*store).clone();
                    let mut ut: metamodelica::Ref<UnitAbsyn::UnitTerm> = ut.clone();
                    divOrMul = Expression::operatorDivOrMul(metamodelica::AsArg::as_arg(&op));
                    (ut, terms, store) = buildTermExp(env, e1.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    if '__try0: {
                        unwrap_break_err!(buildTermExp(env, e1.clone(), divOrMul.clone(), ht.clone(), store.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Ok(((ut.clone(), terms.clone(), store.clone()), ut.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            ut = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 }, divOrMul, ht, store) => {
                    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut divOrMul = (*divOrMul).clone();
                    let mut store = (*store).clone();
                    let mut ut: metamodelica::Ref<UnitAbsyn::UnitTerm> = ut.clone();
                    divOrMul = Expression::operatorDivOrMul(metamodelica::AsArg::as_arg(&op));
                    if '__try0: {
                        unwrap_break_err!(buildTermExp(env, e1.clone(), divOrMul.clone(), ht.clone(), store.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    (ut, terms, store) = buildTermExp(env, e2.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    Ok(((ut.clone(), terms.clone(), store.clone()), ut.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            ut = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: _, exp: e1 }, divOrMul, ht, store) => {
                    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut store = (*store).clone();
                    let mut ut: metamodelica::Ref<UnitAbsyn::UnitTerm> = ut.clone();
                    (ut, terms, store) = buildTermExp(env, e1.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    Ok(((ut.clone(), terms.clone(), store.clone()), ut.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            ut = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::IFEXP { expCond: _, expThen: e1, expElse: e2 }, divOrMul, ht, store) => {
                    let mut ut1: metamodelica::Ref<UnitAbsyn::UnitTerm>;
                    let mut ut2: metamodelica::Ref<UnitAbsyn::UnitTerm>;
                    let mut terms1: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut terms2: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut divOrMul = (*divOrMul).clone();
                    let mut store = (*store).clone();
                    divOrMul = false;
                    (ut1, terms1, store) = buildTermExp(env, e1.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    (ut2, terms2, store) = buildTermExp(env, e2.clone(), divOrMul.clone(), ht.clone(), store.clone())?;
                    terms = listAppend(terms1.clone(), terms2.clone());
                    Ok((metamodelica::Ref::new(UnitAbsyn::UnitTerm::EQN { ut1: ut1.clone(), ut2: ut2.clone(), origExp: e.clone() }), terms.clone(), store.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path, expLst: expl, .. }, divOrMul, ht, store) => {
                    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut divOrMul = (*divOrMul).clone();
                    let mut store = (*store).clone();
                    let mut ut: metamodelica::Ref<UnitAbsyn::UnitTerm> = ut.clone();
                    divOrMul = false;
                    (ut, terms, store) = buildTermCall(env.clone(), path.clone(), e.clone(), metamodelica::AsArg::as_arg(&expl), divOrMul.clone(), &(ht.clone()), store.clone())?;
                    Ok(((ut.clone(), terms.clone(), store.clone()), ut.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            ut = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl }, _, ht, store) => {
                    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut uts: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut store = (*store).clone();
                    let mut ut: metamodelica::Ref<UnitAbsyn::UnitTerm> = ut.clone();
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("vector =")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    (uts, terms, store) = buildTermExpList(env, metamodelica::AsArg::as_arg(&expl), &(ht.clone()), store.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(buildArrayElementTerms(uts.clone(), expl.clone())?) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ut = metamodelica::Own::own(__pa0);
                    uts = metamodelica::Own::own(__pa1);
                    uts = listAppend(terms.clone(), uts.clone());
                    Ok(((ut.clone(), uts.clone(), store.clone()), ut.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            ut = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::MATRIX { matrix: mexpl, .. }, _, ht, store) => {
                    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut uts: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut store = (*store).clone();
                    let mut ut: metamodelica::Ref<UnitAbsyn::UnitTerm> = ut.clone();
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Matrix =")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    expl = List::flatten(mexpl.clone())?;
                    (uts, terms, store) = buildTermExpList(env, &expl, &(ht.clone()), store.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(buildArrayElementTerms(uts.clone(), expl.clone())?) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ut = metamodelica::Own::own(__pa0);
                    uts = metamodelica::Own::own(__pa1);
                    uts = listAppend(terms.clone(), uts.clone());
                    Ok(((ut.clone(), uts.clone(), store.clone()), ut.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            ut = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { .. }, _, _, _) => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("buildTermDAE.CALL failed exp: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((ut, extraTerms, outStore))
}

fn buildArrayElementTerms(
    mut iuts: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>,
    mut iexpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>> {
    let mut outUts: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>> = metamodelica::nil();
    let mut rest_ut: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>> = iuts;
    let mut ut1: metamodelica::Ref<UnitAbsyn::UnitTerm>;
    let mut ut2: metamodelica::Ref<UnitAbsyn::UnitTerm>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut rest_expl: metamodelica::List<metamodelica::Ref<DAE::Exp>> = iexpl;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    while !((rest_ut).is_empty()) {
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(rest_ut) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => return Err("pattern mismatch"),
        } };
        ut1 = metamodelica::Own::own(__pa0);
        ut2 = metamodelica::Own::own(__pa1);
        rest_ut = metamodelica::Own::own(__pa2);
        let (__pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(rest_expl) {
            Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: __pa6 } } => (__pa4.clone(), __pa5.clone(), __pa6.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e1 = metamodelica::Own::own(__pa4);
        e2 = metamodelica::Own::own(__pa5);
        rest_expl = metamodelica::Own::own(__pa6);
        ty = Expression::r#typeof(e1.clone())?;
        outUts = metamodelica::cons(
            metamodelica::Ref::new(UnitAbsyn::UnitTerm::EQN {
                ut1: ut1,
                ut2: ut2,
                origExp: metamodelica::Ref::new(DAE::Exp::ARRAY {
                    ty: ty,
                    scalar: true,
                    array: list![e1, e2],
                }),
            }),
            outUts,
        );
    }
    outUts = outUts.reverse();
    Ok(outUts)
}

fn buildTermCall(
    mut env: FCore::Graph,
    mut path: metamodelica::Ref<Absyn::Path>,
    mut funcCallExp: metamodelica::Ref<DAE::Exp>,
    mut expl: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut divOrMul: bool,
    mut ht: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut istore: UnitAbsyn::Store,
) -> Result<(
    metamodelica::Ref<UnitAbsyn::UnitTerm>,
    metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>,
    UnitAbsyn::Store,
)> {
    let mut ut: metamodelica::Ref<UnitAbsyn::UnitTerm>;
    let mut extraTerms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
    let mut outStore: UnitAbsyn::Store;
    (ut, extraTerms, outStore) = (match istore {
        mut store => {
            let mut formalParamIndxs: metamodelica::List<i32>;
            let mut funcInstId: i32;
            let mut actTermLst: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
            let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
            let mut extraTerms2: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
            let mut functp: metamodelica::Ref<DAE::Type>;
            (_, functp, _) = Lookup::lookupType(FCore::noCache(), env.clone(), path, None)?;
            funcInstId = tick();
            (store, formalParamIndxs) = buildFuncTypeStores(functp.clone(), funcInstId, store)?;
            (actTermLst, extraTerms, store) = buildTermExpList(&env, expl, ht, store)?;
            terms = buildFormal2ActualParamTerms(&formalParamIndxs, &actTermLst)?;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(buildResultTerms(&functp, funcInstId, funcCallExp, store)?) {
                (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, __pa1, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            ut = metamodelica::Own::own(__pa0);
            extraTerms2 = metamodelica::Own::own(__pa1);
            store = metamodelica::Own::own(__pa2);
            extraTerms = List::flatten(list![extraTerms, extraTerms2, terms])?;
            (ut, extraTerms, store)
        }
    });
    Ok((ut, extraTerms, outStore))
}

fn buildResultTerms(
    mut ifunctp: &metamodelica::Ref<DAE::Type>,
    mut funcInstId: i32,
    mut funcCallExp: metamodelica::Ref<DAE::Exp>,
    mut istore: UnitAbsyn::Store,
) -> Result<(
    metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>,
    metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>,
    UnitAbsyn::Store,
)> {
    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>> = metamodelica::nil();
    let mut extraTerms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::Store;
    (terms, extraTerms, outStore) = 'mc: {
        let __mc_input = (&**ifunctp, istore);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_FUNCTION { funcArg: _, funcResultType: functp, functionAttributes: _, path: _ }, store) => {
                    let mut unitStr: ArcStr;
                    let mut unit: UnitAbsyn::Unit;
                    let mut indx: i32;
                    let mut indx2: i32;
                    let mut unspec: bool;
                    let mut store = (*store).clone();
                    unitStr = getUnitStr(functp.clone())?;
                    unspec = 0 == stringCompare(&unitStr, &(literal!("")));
                    unit = str2unit(unitStr.clone(), Some(funcInstId))?;
                    unit = if (unspec) {crate::UnitAbsyn::Unit::UNSPECIFIED} else {unit.clone()};
                    (store, indx) = add(&unit, store.clone())?;
                    (store, indx2) = add(&(crate::UnitAbsyn::Unit::UNSPECIFIED), store.clone())?;
                    Ok((list![metamodelica::Ref::new(UnitAbsyn::UnitTerm::LOC { loc: indx2, origExp: funcCallExp.clone() })], list![metamodelica::Ref::new(UnitAbsyn::UnitTerm::EQN { ut1: metamodelica::Ref::new(UnitAbsyn::UnitTerm::LOC { loc: indx2, origExp: funcCallExp.clone() }), ut2: metamodelica::Ref::new(UnitAbsyn::UnitTerm::LOC { loc: indx, origExp: funcCallExp.clone() }), origExp: funcCallExp.clone() })], store.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_FUNCTION { funcResultType: Deref @ DAE::Type::T_TUPLE { types: typeLst, .. }, .. }, store) => {
                    let mut store = (*store).clone();
                    let mut extraTerms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>> = extraTerms.clone();
                    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>> = terms.clone();
                    (terms, extraTerms, store) = buildTupleResultTerms(metamodelica::AsArg::as_arg(&typeLst), funcInstId, &funcCallExp, store.clone())?;
                    Ok(((terms.clone(), extraTerms.clone(), store.clone()), extraTerms.clone(), terms.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            extraTerms = __wb0;
            terms = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("buildResultTerms failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((terms, extraTerms, outStore))
}

fn buildTupleResultTerms(
    mut ifunctps: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut funcInstId: i32,
    mut funcCallExp: &metamodelica::Ref<DAE::Exp>,
    mut istore: UnitAbsyn::Store,
) -> Result<(
    metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>,
    metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>,
    UnitAbsyn::Store,
)> {
    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
    let mut extraTerms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
    let mut outStore: UnitAbsyn::Store;
    (terms, extraTerms, outStore) = (::match_deref::match_deref! { match ifunctps {
        Deref @ metamodelica::ListNode::Nil => {
            let mut store = istore;
            (metamodelica::nil(), metamodelica::nil(), store)
        },
        Deref @ metamodelica::ListNode::Cons { head: tp, tail: functps } => {
            let mut store = istore;
            let mut terms1: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
            let mut terms2: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
            let mut extraTerms1: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
            let mut extraTerms2: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
            (terms1, extraTerms1, store) = buildResultTerms(metamodelica::AsArg::as_arg(&tp), funcInstId, funcCallExp.clone(), store)?;
            (terms2, extraTerms2, store) = buildTupleResultTerms(functps, funcInstId, funcCallExp, store)?;
            terms = listAppend(terms1, terms2);
            extraTerms = listAppend(extraTerms1, extraTerms2);
            (terms, extraTerms, store)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((terms, extraTerms, outStore))
}

fn buildTermExpList(
    mut env: &FCore::Graph,
    mut iexpl: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut ht: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut istore: UnitAbsyn::Store,
) -> Result<(
    metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>,
    metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>,
    UnitAbsyn::Store,
)> {
    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>> = metamodelica::nil();
    let mut extraTerms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>> = metamodelica::nil();
    let mut outStore: UnitAbsyn::Store;
    (terms, extraTerms, outStore) = 'mc: {
        let __mc_input = (&**iexpl, istore);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, store) => {
                    Ok((metamodelica::nil(), metamodelica::nil(), store.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: e, tail: expl }, store) => {
                    let mut eterms1: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut eterms2: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
                    let mut ut: metamodelica::Ref<UnitAbsyn::UnitTerm>;
                    let mut store = (*store).clone();
                    let mut extraTerms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>> = extraTerms.clone();
                    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>> = terms.clone();
                    (ut, eterms1, store) = buildTermExp(env, e.clone(), false, ht.clone(), store.clone())?;
                    (terms, eterms2, store) = buildTermExpList(env, metamodelica::AsArg::as_arg(&expl), ht, store.clone())?;
                    extraTerms = listAppend(eterms1.clone(), eterms2.clone());
                    Ok(((metamodelica::cons(ut.clone(), terms.clone()), extraTerms.clone(), store.clone()), extraTerms.clone(), terms.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            extraTerms = __wb0;
            terms = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: e, tail: _ }, _) => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("buildTermExpList failed for exp")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((terms, extraTerms, outStore))
}

fn buildFuncTypeStores(
    mut funcType: metamodelica::Ref<DAE::Type>,
    mut funcInstId: i32,
    mut istore: UnitAbsyn::Store,
) -> Result<(UnitAbsyn::Store, metamodelica::List<i32>)> {
    let mut outStore: UnitAbsyn::Store;
    let mut indxs: metamodelica::List<i32> = metamodelica::nil();
    (outStore, indxs) = 'mc: {
        let __mc_input = (funcType, istore);
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_FUNCTION { funcArg: args, .. }, store) => {
                    let mut store = (*store).clone();
                    let mut indxs: metamodelica::List<i32> = indxs.clone();
                    (store, indxs) = buildFuncTypeStores2(metamodelica::AsArg::as_arg(&args), funcInstId, store.clone())?;
                    Ok(((store.clone(), indxs.clone()), indxs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            indxs = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (tp, _) => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("buildFuncTypeStores failed, tp")); __mm_s.push_str(&*TypesDump::unparseType(tp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outStore, indxs))
}

fn buildFuncTypeStores2(
    mut ifargs: &metamodelica::List<metamodelica::Ref<DAE::FuncArg>>,
    mut funcInstId: i32,
    mut istore: UnitAbsyn::Store,
) -> Result<(UnitAbsyn::Store, metamodelica::List<i32>)> {
    let mut outStore: UnitAbsyn::Store;
    let mut indxs: metamodelica::List<i32>;
    (outStore, indxs) = (::match_deref::match_deref! { match ifargs {
        Deref @ metamodelica::ListNode::Nil => {
            let mut store = istore;
            (store, metamodelica::nil())
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::FuncArg { ty: tp, .. }, tail: fargs } => {
            let mut store = istore;
            let mut unitStr: ArcStr;
            let mut indx: i32;
            let mut unit: UnitAbsyn::Unit;
            unitStr = getUnitStr(tp.clone())?;
            unit = str2unit(unitStr.clone(), Some(funcInstId))?;
            unit = if (0 == stringCompare(&unitStr, &(literal!("")))) {crate::UnitAbsyn::Unit::UNSPECIFIED} else {unit};
            (store, indx) = add(&unit, store)?;
            (store, indxs) = buildFuncTypeStores2(fargs, funcInstId, store)?;
            (store, metamodelica::cons(indx, indxs))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outStore, indxs))
}

fn getUnitStr(mut itp: metamodelica::Ref<DAE::Type>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = 'mc: {
        let __mc_input = itp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ DAE::Type::T_REAL { varLst } => {
                            for mut v in &*varLst.clone() {
                                let () = (::match_deref::match_deref! { match &(v.clone()) {
                Deref @ DAE::Var { name: Deref @ "unit", binding: Deref @ DAE::Binding::EQBOUND { exp: Deref @ DAE::Exp::SCONST { string: __esc_str }, .. }, .. } => {
                            r#str = (*__esc_str).clone();
                            return Ok(r#str.clone());
                            ()
                },
                _ => (),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                            }
                            Ok(literal!(""))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { .. } => {
                    Ok(literal!(""))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty: tp, .. } => {
                    Ok(getUnitStr(tp.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                tp => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getUnitStr for type ")); __mm_s.push_str(&*TypesDump::unparseType(tp.clone())?); __mm_s.push_str(&*literal!(" failed\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(r#str)
}

fn buildFormal2ActualParamTerms(
    mut iformalParamIndxs: &metamodelica::List<i32>,
    mut iactualParamIndxs: &metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>,
) -> Result<metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>> {
    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>> = metamodelica::nil();
    terms = 'mc: {
        let __mc_input = (&**iformalParamIndxs, &**iactualParamIndxs);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: loc1, tail: formalParamIndxs }, Deref @ metamodelica::ListNode::Cons { head: ut, tail: actualParamIndxs }) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut terms: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>> = terms.clone();
                    terms = buildFormal2ActualParamTerms(metamodelica::AsArg::as_arg(&formalParamIndxs), metamodelica::AsArg::as_arg(&actualParamIndxs))?;
                    e = origExpInTerm(metamodelica::AsArg::as_arg(&ut));
                    Ok((metamodelica::cons(metamodelica::Ref::new(UnitAbsyn::UnitTerm::EQN { ut1: metamodelica::Ref::new(UnitAbsyn::UnitTerm::LOC { loc: loc1.clone(), origExp: e.clone() }), ut2: ut.clone(), origExp: e.clone() }), terms.clone()), terms.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            terms = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("buildFormal2ActualParamTerms failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(terms)
}

fn origExpInTerm(mut ut: &metamodelica::Ref<UnitAbsyn::UnitTerm>) -> metamodelica::Ref<DAE::Exp> {
    let mut origExp: metamodelica::Ref<DAE::Exp>;
    origExp = (match &**ut {
        UnitAbsyn::UnitTerm::ADD {
            ut1: _,
            ut2: _,
            origExp: e,
        } => e.clone(),
        UnitAbsyn::UnitTerm::SUB {
            ut1: _,
            ut2: _,
            origExp: e,
        } => e.clone(),
        UnitAbsyn::UnitTerm::MUL {
            ut1: _,
            ut2: _,
            origExp: e,
        } => e.clone(),
        UnitAbsyn::UnitTerm::DIV {
            ut1: _,
            ut2: _,
            origExp: e,
        } => e.clone(),
        UnitAbsyn::UnitTerm::EQN {
            ut1: _,
            ut2: _,
            origExp: e,
        } => e.clone(),
        UnitAbsyn::UnitTerm::LOC { loc: _, origExp: e } => e.clone(),
        UnitAbsyn::UnitTerm::POW {
            ut1: _,
            exponent: _,
            origExp: e,
        } => e.clone(),
    });
    origExp
}

fn buildTermOp(
    mut ut1: metamodelica::Ref<UnitAbsyn::UnitTerm>,
    mut ut2: metamodelica::Ref<UnitAbsyn::UnitTerm>,
    mut op: &DAE::Operator,
    mut origExp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<UnitAbsyn::UnitTerm>> {
    let mut ut: metamodelica::Ref<UnitAbsyn::UnitTerm>;
    ut = (match op.clone() {
        DAE::Operator::ADD { .. } => metamodelica::Ref::new(UnitAbsyn::UnitTerm::ADD {
            ut1: ut1,
            ut2: ut2,
            origExp: origExp,
        }),
        DAE::Operator::SUB { .. } => metamodelica::Ref::new(UnitAbsyn::UnitTerm::SUB {
            ut1: ut1,
            ut2: ut2,
            origExp: origExp,
        }),
        DAE::Operator::MUL { .. } => metamodelica::Ref::new(UnitAbsyn::UnitTerm::MUL {
            ut1: ut1,
            ut2: ut2,
            origExp: origExp,
        }),
        DAE::Operator::DIV { .. } => metamodelica::Ref::new(UnitAbsyn::UnitTerm::DIV {
            ut1: ut1,
            ut2: ut2,
            origExp: origExp,
        }),
        _ => return Err("match: no arm matched"),
    });
    Ok(ut)
}

fn buildStores2(
    mut dae: &DAE::DAElist,
    mut inStore: &UnitAbsyn::Store,
    mut inHt: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    UnitAbsyn::Store,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut outStore: UnitAbsyn::Store;
    let mut outHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            HashTable::FuncHashCref,
            HashTable::FuncCrefEqual,
            HashTable::FuncCrefStr,
            HashTable::FuncExpStr,
        ),
    );
    (outStore, outHt) = 'mc: {
        let __mc_input = dae;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Nil } => {
                    Ok((inStore.clone(), inHt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { componentRef: cr, variableAttributesOption: attropt, .. }, tail: elts } } => {
                    let mut indx: i32;
                    let mut unitStr: ArcStr;
                    let mut unit: UnitAbsyn::Unit;
                    let mut store: UnitAbsyn::Store;
                    let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
                    let __pa0 = ::match_deref::match_deref! { match &(DAEUtil::getUnitAttr(attropt.clone())) {
                        Deref @ DAE::Exp::SCONST { string: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    unitStr = metamodelica::Own::own(__pa0);
                    unit = str2unit(unitStr.clone(), None)?;
                    (store, indx) = add(&unit, inStore.clone())?;
                    ht = BaseHashTable::add((cr.clone(), indx), inHt.clone())?;
                    (store, ht) = buildStores2(&(DAE::DAElist { elementLst: elts.clone() }), &store, &ht)?;
                    Ok((store.clone(), ht.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { componentRef: cr, .. }, tail: _ } } => {
                    let mut indx: i32;
                    let mut store: UnitAbsyn::Store;
                    let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
                    (store, indx) = add(&(crate::UnitAbsyn::Unit::UNSPECIFIED), inStore.clone())?;
                    ht = BaseHashTable::add((cr.clone(), indx), inHt.clone())?;
                    Ok((store.clone(), ht.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: elts } } => {
                    let mut store: UnitAbsyn::Store;
                    let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
                    (store, ht) = buildStores2(&(DAE::DAElist { elementLst: elts.clone() }), inStore, inHt)?;
                    Ok((store.clone(), ht.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outStore, outHt))
}

fn buildStores3(
    mut dae: &DAE::DAElist,
    mut inStore: UnitAbsyn::Store,
    mut inHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    UnitAbsyn::Store,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut outStore: UnitAbsyn::Store;
    let mut outHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            HashTable::FuncHashCref,
            HashTable::FuncCrefEqual,
            HashTable::FuncCrefStr,
            HashTable::FuncExpStr,
        ),
    );
    (outStore, outHt) = 'mc: {
        let __mc_input = (dae, inStore, inHt);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Nil }, store, ht) => {
                    Ok((store.clone(), ht.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUATION { exp: e1, scalar: e2, source: _ }, tail: elts } }, store, ht) => {
                    let mut store = (*store).clone();
                    let mut ht = (*ht).clone();
                    (store, ht) = buildStoreExp(metamodelica::AsArg::as_arg(&e1), store.clone(), ht.clone(), None);
                    (store, ht) = buildStoreExp(metamodelica::AsArg::as_arg(&e2), store.clone(), ht.clone(), None);
                    (store, ht) = buildStores3(&(DAE::DAElist { elementLst: elts.clone() }), store.clone(), ht.clone())?;
                    Ok((store.clone(), ht.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::DAElist { elementLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: elts } }, store, ht) => {
                    let mut store = (*store).clone();
                    let mut ht = (*ht).clone();
                    (store, ht) = buildStores3(&(DAE::DAElist { elementLst: elts.clone() }), store.clone(), ht.clone())?;
                    Ok((store.clone(), ht.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outStore, outHt))
}

fn buildStoreExp(
    mut exp: &metamodelica::Ref<DAE::Exp>,
    mut inStore: UnitAbsyn::Store,
    mut inHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut parentOp: Option<DAE::Operator>,
) -> (
    UnitAbsyn::Store,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) {
    let mut outStore: UnitAbsyn::Store;
    let mut outHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        ),
        i32,
        (
            HashTable::FuncHashCref,
            HashTable::FuncCrefEqual,
            HashTable::FuncCrefStr,
            HashTable::FuncExpStr,
        ),
    );
    (outStore, outHt) = 'mc: {
        let __mc_input = (&**exp, inStore, inHt);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { real: r }, store, ht) => {
                    let mut s1: ArcStr;
                    let mut indx: i32;
                    let mut unit: UnitAbsyn::Unit;
                    let mut cref_: metamodelica::Ref<DAE::ComponentRef>;
                    let mut store = (*store).clone();
                    let mut ht = (*ht).clone();
                    unit = selectConstantUnit(parentOp.clone())?;
                    (store, indx) = add(&unit, store.clone())?;
                    s1 = realString(r.clone());
                    cref_ = ComponentReferenceBasics::makeCrefIdent(s1.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                    ht = BaseHashTable::add((cref_.clone(), indx), ht.clone())?;
                    Ok((store.clone(), ht.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CAST { ty: _, exp: Deref @ DAE::Exp::ICONST { integer: i } }, store, ht) => {
                    let mut s1: ArcStr;
                    let mut indx: i32;
                    let mut unit: UnitAbsyn::Unit;
                    let mut cref_: metamodelica::Ref<DAE::ComponentRef>;
                    let mut store = (*store).clone();
                    let mut ht = (*ht).clone();
                    unit = selectConstantUnit(parentOp.clone())?;
                    (store, indx) = add(&unit, store.clone())?;
                    s1 = intString(i.clone());
                    cref_ = ComponentReferenceBasics::makeCrefIdent(s1.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
                    ht = BaseHashTable::add((cref_.clone(), indx), ht.clone())?;
                    Ok((store.clone(), ht.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 }, store, ht) => {
                    let mut store = (*store).clone();
                    let mut ht = (*ht).clone();
                    (store, ht) = buildStoreExp(metamodelica::AsArg::as_arg(&e1), store.clone(), ht.clone(), Some(op.clone()));
                    (store, ht) = buildStoreExp(metamodelica::AsArg::as_arg(&e2), store.clone(), ht.clone(), Some(op.clone()));
                    Ok((store.clone(), ht.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: _, exp: e1 }, store, ht) => {
                    let mut store = (*store).clone();
                    let mut ht = (*ht).clone();
                    (store, ht) = buildStoreExp(metamodelica::AsArg::as_arg(&e1), store.clone(), ht.clone(), parentOp.clone());
                    Ok((store.clone(), ht.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: _, expThen: e1, expElse: e2 }, store, ht) => {
                    let mut store = (*store).clone();
                    let mut ht = (*ht).clone();
                    (store, ht) = buildStoreExp(metamodelica::AsArg::as_arg(&e1), store.clone(), ht.clone(), parentOp.clone());
                    (store, ht) = buildStoreExp(metamodelica::AsArg::as_arg(&e2), store.clone(), ht.clone(), parentOp.clone());
                    Ok((store.clone(), ht.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, store, ht) => {
                    Ok((store.clone(), ht.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outStore, outHt)
}

pub(crate) fn unitMultiply(mut u1: &UnitAbsyn::Unit, mut u2: &UnitAbsyn::Unit) -> Result<UnitAbsyn::Unit> {
    let mut u: UnitAbsyn::Unit;
    u = (match (u1.clone(), u2.clone()) {
        (
            UnitAbsyn::Unit::SPECIFIED {
                specified:
                    UnitAbsyn::SpecUnit {
                        typeParameters: ref tparams1,
                        units: ref units1,
                    },
            },
            UnitAbsyn::Unit::SPECIFIED {
                specified:
                    UnitAbsyn::SpecUnit {
                        typeParameters: ref tparams2,
                        units: ref units2,
                    },
            },
        ) => {
            let mut tparams: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
            let mut units: metamodelica::List<MMath::Rational>;
            tparams = listAppend(tparams1.clone(), tparams2.clone());
            units = List::threadMap(units1.clone(), units2.clone(), &MMath::addRational)?;
            UnitAbsyn::Unit::SPECIFIED {
                specified: UnitAbsyn::SpecUnit {
                    typeParameters: tparams,
                    units: units,
                },
            }
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(u)
}

fn selectConstantUnit(mut op: Option<DAE::Operator>) -> Result<UnitAbsyn::Unit> {
    let mut unit: UnitAbsyn::Unit;
    unit = (match op {
        None => crate::UnitAbsyn::Unit::UNSPECIFIED,
        Some(DAE::Operator::ADD { ty: _ }) => crate::UnitAbsyn::Unit::UNSPECIFIED,
        Some(DAE::Operator::SUB { ty: _ }) => crate::UnitAbsyn::Unit::UNSPECIFIED,
        Some(_) => str2unit(literal!("1"), None)?,
    });
    Ok(unit)
}

pub(crate) fn unit2str(mut unit: &UnitAbsyn::Unit) -> Result<ArcStr> {
    let mut res: ArcStr;
    res = (match unit.clone() {
        UnitAbsyn::Unit::SPECIFIED {
            specified:
                UnitAbsyn::SpecUnit {
                    typeParameters: ref typeParams,
                    units: mut units,
                },
        } => {
            let mut nums: metamodelica::List<i32>;
            let mut denoms: metamodelica::List<i32>;
            let mut tpnoms: metamodelica::List<i32>;
            let mut tpdenoms: metamodelica::List<i32>;
            let mut tpstrs: metamodelica::List<ArcStr>;
            (nums, denoms) = splitRationals(metamodelica::AsArg::as_arg(&units))?;
            (tpnoms, tpdenoms, tpstrs) = splitTypeParams(metamodelica::AsArg::as_arg(&typeParams))?;
            res = UnitParserExt::unit2str(
                nums,
                denoms,
                tpnoms,
                tpdenoms,
                tpstrs,
                metamodelica::OrderedFloat(1.0_f64),
                metamodelica::OrderedFloat(0.0_f64),
            );
            res
        }
        UnitAbsyn::Unit::UNSPECIFIED { .. } => {
            literal!("unspecified")
        }
    });
    Ok(res)
}

pub fn str2unit(mut res: ArcStr, mut funcInstIdOpt: Option<i32>) -> Result<UnitAbsyn::Unit> {
    let mut unit: UnitAbsyn::Unit;
    (unit, _, _) = str2unitWithScaleFactor(res, funcInstIdOpt)?;
    Ok(unit)
}

pub fn str2unitWithScaleFactor(
    mut res: ArcStr,
    mut funcInstIdOpt: Option<i32>,
) -> Result<(UnitAbsyn::Unit, metamodelica::Real, metamodelica::Real)> {
    let mut unit: UnitAbsyn::Unit;
    let mut scaleFactor: metamodelica::Real;
    let mut offset: metamodelica::Real;
    let mut nums: metamodelica::List<i32>;
    let mut denoms: metamodelica::List<i32>;
    let mut tpnoms: metamodelica::List<i32>;
    let mut tpdenoms: metamodelica::List<i32>;
    let mut tpstrs: metamodelica::List<ArcStr>;
    let mut typeParams: metamodelica::List<(MMath::Rational, UnitAbsyn::TypeParameter)>;
    let mut units: metamodelica::List<MMath::Rational>;
    (nums, denoms, tpnoms, tpdenoms, tpstrs, scaleFactor, offset) = UnitParserExt::str2unit(res)?;
    units = joinRationals(&nums, &denoms)?;
    typeParams = joinTypeParams(&tpnoms, &tpdenoms, &tpstrs, funcInstIdOpt)?;
    unit = UnitAbsyn::Unit::SPECIFIED {
        specified: UnitAbsyn::SpecUnit {
            typeParameters: typeParams,
            units: units,
        },
    };
    Ok((unit, scaleFactor, offset))
}

fn getDerivedUnitsHelper(
    mut baseUnit: UnitAbsyn::Unit,
    mut baseUnitStr: &ArcStr,
    mut inUnits: &metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outUnits: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut unit: UnitAbsyn::Unit;
    let mut b: bool;
    for mut unitStr in &**inUnits {
        if boolNot(stringEq(&baseUnitStr, &unitStr)) {
            unit = str2unit(unitStr.clone(), None)?;
            b = baseUnit.clone() == unit;
            if b {
                outUnits = metamodelica::cons(unitStr.clone(), outUnits);
            }
        }
    }
    Ok(outUnits)
}

pub fn getDerivedUnits(mut baseUnit: UnitAbsyn::Unit, mut baseUnitStr: &ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut derivedUnits: metamodelica::List<ArcStr>;
    let mut unitSymbols: metamodelica::List<ArcStr>;
    unitSymbols = UnitParserExt::allUnitSymbols();
    derivedUnits = getDerivedUnitsHelper(baseUnit, baseUnitStr, &unitSymbols)?;
    Ok(derivedUnits)
}

/* Tests  */
/* Test1:

model Test1 "CONSISTENT: All units defined. No inference"
  Position x;
  Velocity v;
  Acceleration a;
algorithm
  der(x) = v;
  der(v) = a;
end Test1;
*/
pub(crate) fn buildTest1() -> Result<(
    metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>,
    UnitAbsyn::Store,
)> {
    let mut ut: metamodelica::List<metamodelica::Ref<UnitAbsyn::UnitTerm>>;
    let mut sigma: UnitAbsyn::Store;
    let mut r0: MMath::Rational;
    let mut r1: MMath::Rational;
    let mut nr1: MMath::Rational;
    let mut nr2: MMath::Rational;
    let mut unitderx: UnitAbsyn::Unit;
    let mut unitderv: UnitAbsyn::Unit;
    let mut unitx: UnitAbsyn::Unit;
    let mut unitv: UnitAbsyn::Unit;
    let mut unita: UnitAbsyn::Unit;
    r0 = MMath::Rational { nom: 0, denom: 0 };
    r1 = MMath::Rational { nom: 1, denom: 0 };
    nr1 = MMath::Rational { nom: -1, denom: 0 };
    nr2 = MMath::Rational { nom: -2, denom: 0 };
    ut = list![
        metamodelica::Ref::new(UnitAbsyn::UnitTerm::EQN {
            ut1: metamodelica::Ref::new(UnitAbsyn::UnitTerm::LOC {
                loc: 1,
                origExp: metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("1") })
            }),
            ut2: metamodelica::Ref::new(UnitAbsyn::UnitTerm::LOC {
                loc: 4,
                origExp: metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("4") })
            }),
            origExp: metamodelica::Ref::new(DAE::Exp::SCONST {
                string: literal!("1==4")
            })
        }),
        metamodelica::Ref::new(UnitAbsyn::UnitTerm::EQN {
            ut1: metamodelica::Ref::new(UnitAbsyn::UnitTerm::LOC {
                loc: 2,
                origExp: metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("2") })
            }),
            ut2: metamodelica::Ref::new(UnitAbsyn::UnitTerm::LOC {
                loc: 5,
                origExp: metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("5") })
            }),
            origExp: metamodelica::Ref::new(DAE::Exp::SCONST {
                string: literal!("2==5")
            })
        })
    ];
    unitderx = UnitAbsyn::Unit::SPECIFIED {
        specified: UnitAbsyn::SpecUnit {
            typeParameters: metamodelica::nil(),
            units: list![r1, nr1, r0, r0, r0, r0, r0],
        },
    };
    unitderv = UnitAbsyn::Unit::SPECIFIED {
        specified: UnitAbsyn::SpecUnit {
            typeParameters: metamodelica::nil(),
            units: list![r1, nr2, r0, r0, r0, r0, r0],
        },
    };
    unitx = UnitAbsyn::Unit::SPECIFIED {
        specified: UnitAbsyn::SpecUnit {
            typeParameters: metamodelica::nil(),
            units: list![r1, r0, r0, r0, r0, r0, r0],
        },
    };
    unitv = UnitAbsyn::Unit::SPECIFIED {
        specified: UnitAbsyn::SpecUnit {
            typeParameters: metamodelica::nil(),
            units: list![r1, nr1, r0, r0, r0, r0, r0],
        },
    };
    unita = UnitAbsyn::Unit::SPECIFIED {
        specified: UnitAbsyn::SpecUnit {
            typeParameters: metamodelica::nil(),
            units: list![r1, nr2, r0, r0, r0, r0, r0],
        },
    };
    sigma = emptyStore();
    (sigma, _) = add(&unitderx, sigma)?;
    (sigma, _) = add(&unitderv, sigma)?;
    (sigma, _) = add(&unitx, sigma)?;
    (sigma, _) = add(&unitv, sigma)?;
    (sigma, _) = add(&unita, sigma)?;
    printStore(&sigma)?;
    Ok((ut, sigma))
}

/* Test2:
model Test2 "CONSISTENT: Subtraction operator. All units defined. No inference"
Position x,y,z;
algorithm
z = x-y;
end Test2;
*/
/*public function buildTest2

  output UnitAbsyn.UnitTerms ut;
  output UnitAbsyn.Locations sigma;
protected
  MMath.Rational r0,r1;
  algorithm
    r0 := MMath.RATIONAL(0,0);
    r1 := MMath.RATIONAL(1,0);
    ut := {
    UnitAbsyn.EQN(UnitAbsyn.LOC("z"),UnitAbsyn.SUB(UnitAbsyn.LOC("x"),UnitAbsyn.LOC("y")))
    };
    sigma := {
    UnitAbsyn.LOCATION("x",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,r0,r0,r0,r0,r0,r0}))), // x -> m
    UnitAbsyn.LOCATION("y",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,r0,r0,r0,r0,r0,r0}))), // y -> m
    UnitAbsyn.LOCATION("z",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,r0,r0,r0,r0,r0,r0}))) // z -> m
    };
 end buildTest2;
 */
/* Test3
 model Test3 "OVERDETERMINED: All units defined. No inference"
 Position x,y;
 Velocity z;
algorithm
 z = x-y;
end Test3;
 */
/*public function buildTest3
  output UnitAbsyn.UnitTerms ut;
  output UnitAbsyn.Locations sigma;
protected
  MMath.Rational r0,r1,nr1;
  algorithm
    r0 := MMath.RATIONAL(0,0);
    r1 := MMath.RATIONAL(1,0);
    nr1 := MMath.RATIONAL(-1,0);
    ut := {
    UnitAbsyn.EQN(UnitAbsyn.LOC("z"),UnitAbsyn.SUB(UnitAbsyn.LOC("x"),UnitAbsyn.LOC("y")))
    };
    sigma := {
    UnitAbsyn.LOCATION("x",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,r0,r0,r0,r0,r0,r0}))), // x -> m
    UnitAbsyn.LOCATION("y",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,r0,r0,r0,r0,r0,r0}))), // y -> m
    UnitAbsyn.LOCATION("z",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,nr1,r0,r0,r0,r0,r0}))) // z -> m/s
    };
 end buildTest3;
 */
/*
 Test5

 model Test5 "CONSTISTENT: Multiplication operator. Not all units defined. inference"
  Position x,y;
  Real z;
 algorithm
 z = x*y;
end test5;
*/
/*
 public function buildTest5
  output UnitAbsyn.UnitTerms ut;
  output UnitAbsyn.Locations sigma;
protected
  MMath.Rational r0,r1,nr1;
  algorithm
    r0 := MMath.RATIONAL(0,0);
    r1 := MMath.RATIONAL(1,0);
    nr1 := MMath.RATIONAL(-1,0);
    ut := {
    UnitAbsyn.EQN(UnitAbsyn.LOC("z"),UnitAbsyn.MUL(UnitAbsyn.LOC("x"),UnitAbsyn.LOC("y")))
    };
    sigma := {
    UnitAbsyn.LOCATION("x",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,r0,r0,r0,r0,r0,r0}))), // x -> m
    UnitAbsyn.LOCATION("y",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,r0,r0,r0,r0,r0,r0}))), // y -> m
    UnitAbsyn.LOCATION("z",UnitAbsyn.UNSPECIFIED())                                             // z -> unspecified
    };
 end buildTest5;
 */
/* Test 8


function Foo8
  input Real x;
  output Real y;
algorithm
  y := x+1; // 1 has unkown unit
end Foo8;

model Test8 "CONSISTENT. type inference in function call "
  Position x,y;
  Velocity v1,v2;

algorithm
  x = Foo8(y);
  v1 = Foo8(v2);
end Test8;
 */
/*public function buildTest8
  output UnitAbsyn.UnitTerms ut;
  output UnitAbsyn.Locations sigma;
protected
  MMath.Rational r0,r1,nr1;
  algorithm
    r0 := MMath.RATIONAL(0,0);
    r1 := MMath.RATIONAL(1,0);
    nr1 := MMath.RATIONAL(-1,0);
    ut := {
    UnitAbsyn.EQN(UnitAbsyn.LOC("x"),UnitAbsyn.LOC("Foo8(x)")),
    UnitAbsyn.EQN(UnitAbsyn.LOC("v1"),UnitAbsyn.LOC("Foo8(v2)"))
    };
    sigma := {
    UnitAbsyn.LOCATION("Foo8(y)",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,r0,r0,r0,r0,r0,r0}))), // Foo8(x) -> m
    UnitAbsyn.LOCATION("Foo8(v2)",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,nr1,r0,r0,r0,r0,r0}))), // Foo8(v2) -> m/s
    UnitAbsyn.LOCATION("v1",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,nr1,r0,r0,r0,r0,r0}))), // Foo8(v2) -> m/s
    UnitAbsyn.LOCATION("x",UnitAbsyn.SPECIFIED(UnitAbsyn.SPECUNIT({},{r1,r0,r0,r0,r0,r0,r0}))) // Foo8(v2) -> m
    };
 end buildTest8;
 */
