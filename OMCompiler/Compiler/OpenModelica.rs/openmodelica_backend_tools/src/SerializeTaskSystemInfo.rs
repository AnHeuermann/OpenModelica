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
use openmodelica_backend_types::BackendDAE;
use openmodelica_codegen_util::SimCodeCodegenUtil;
use openmodelica_frontend_base::Algorithm;
use openmodelica_frontend_base::ComponentReference::writeCref;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_dump::ExpressionBasics::printExpStr as expStr;
use openmodelica_frontend_types::DAE;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_util::Error;
use openmodelica_util::File;
use openmodelica_util::File::Escape::JSON;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn serializeParMod(mut code: &metamodelica::Ref<SimCode::SimCode>, mut withOperations: bool) -> Result<ArcStr> {
    let mut fileName: ArcStr;
    let (true, __pa0) = (serializeParModWork(code, withOperations)?) else {
        return Err("pattern mismatch");
    };
    fileName = metamodelica::Own::own(__pa0);
    Ok(fileName)
}

fn serializeParModWork(
    mut code: &metamodelica::Ref<SimCode::SimCode>,
    mut withOperations: bool,
) -> Result<(bool, ArcStr)> {
    let mut success: bool;
    let mut fileName: ArcStr;
    let mut file: File::File = File::File(File::noReference())?;
    let mut mi: SimCode::ModelInfo;
    let mut vars: SimCodeVar::SimVars;
    match '__try0: {
        let __arc3 = &(*code);
        let SimCode::SIMCODE {
            modelInfo: __pa2 @ SimCode::MODELINFO { vars: __pa1, .. },
            ..
        } = &**__arc3;
        vars = metamodelica::Own::own(__pa1);
        mi = metamodelica::Own::own(__pa2);
        fileName = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*code.fileNamePrefix);
            __mm_s.push_str(&*literal!("_ode.json"));
            ArcStr::from(__mm_s)
        };
        File::open(file.clone(), fileName.clone(), File::Mode::Write.clone());
        File::write(
            file.clone(),
            literal!("{\"format\":\"ParModelica task system info\",\"version\":1,\n\"info\":{\"name\":"),
        );
        serializePath(file.clone(), mi.name.clone());
        File::write(file.clone(), literal!(",\"description\":\""));
        File::writeEscape(file.clone(), mi.description.clone(), JSON.clone());
        File::write(file.clone(), literal!("\"},\n\"ode-equations\":["));
        File::write(file.clone(), literal!("{\"eqIndex\":0,\"tag\":\"dummy\"}"));
        for mut eq in &*unwrap_break_err!(SimCodeCodegenUtil::sortEqSystems(unwrap_break_err!(List::flatten(code.odeEquations.clone()), '__try0)), '__try0)
        {
            unwrap_break_err!(serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&eq), &(literal!("regular")), withOperations, 0, false, 0), '__try0);
        }
        File::write(file.clone(), literal!("\n]\n}"));
        success = true;
        Ok::<_, &'static str>((fileName.clone(), success.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            fileName = __try0_o0;
            success = __try0_o1;
        }
        Err(_) => {
            fileName = literal!("SerializeTaskSystemInfo.serializeParModWork failed");
            Error::addInternalError(
                fileName.clone(),
                metamodelica::sourceInfo!("SimCode/SerializeTaskSystemInfo.mo"),
            )?;
            success = false;
        }
    }
    Ok((success, fileName))
}

fn serializeEquation(
    mut file: File::File,
    mut eq: &metamodelica::Ref<SimCode::SimEqSystem>,
    mut section: &ArcStr,
    mut withOperations: bool,
    mut parent: i32,
    mut first: bool,
    mut assign_type: i32,
) -> Result<bool> {
    let mut success: bool;
    if !(first) {
        File::write(file.clone(), literal!(","));
    }
    success = (::match_deref::match_deref! { match eq {
        Deref @ SimCode::SimEqSystem::SES_RESIDUAL { exp: __eq_exp, index: __eq_index, source: __eq_source, .. } => {
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            File::write(file.clone(), literal!("\",\"tag\":\"residual\",\"uses\":["));
            serializeUses(file.clone(), &(Expression::extractUniqueCrefsFromExpDerPreStart(__eq_exp.clone(), true)?))?;
            File::write(file.clone(), literal!("],\"equation\":[\""));
            File::writeEscape(file.clone(), expStr(__eq_exp.clone())?, JSON.clone());
            File::write(file.clone(), literal!("\"],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations);
            File::write(file, literal!("}"));
            true
        },
        Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { cref: __eq_cref, exp: __eq_exp, index: __eq_index, source: __eq_source, .. } => {
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            if assign_type == 1 {
                File::write(file.clone(), literal!("\",\"tag\":\"torn\",\"defines\":[\""));
            } else if assign_type == 2 {
                File::write(file.clone(), literal!("\",\"tag\":\"jacobian\",\"defines\":[\""));
            } else {
                File::write(file.clone(), literal!("\",\"tag\":\"assign\",\"defines\":[\""));
            }
            writeCref(file.clone(), __eq_cref.clone(), JSON.clone())?;
            File::write(file.clone(), literal!("\"],\"uses\":["));
            serializeUses(file.clone(), &(Expression::extractUniqueCrefsFromExpDerPreStart(__eq_exp.clone(), true)?))?;
            File::write(file.clone(), literal!("],\"equation\":[\""));
            File::writeEscape(file.clone(), expStr(__eq_exp.clone())?, JSON.clone());
            File::write(file.clone(), literal!("\"],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations);
            File::write(file, literal!("}"));
            true
        },
        Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { cref: __eq_cref, exp: __eq_exp, index: __eq_index, source: __eq_source, .. } => {
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            if assign_type == 1 {
                File::write(file.clone(), literal!("\",\"tag\":\"torn\",\"defines\":[\""));
            } else if assign_type == 2 {
                File::write(file.clone(), literal!("\",\"tag\":\"jacobian\",\"defines\":[\""));
            } else {
                File::write(file.clone(), literal!("\",\"tag\":\"assign\",\"defines\":[\""));
            }
            writeCref(file.clone(), __eq_cref.clone(), JSON.clone())?;
            File::write(file.clone(), literal!("\"],\"uses\":["));
            serializeUses(file.clone(), &(Expression::extractUniqueCrefsFromExpDerPreStart(__eq_exp.clone(), true)?))?;
            File::write(file.clone(), literal!("],\"equation\":[\""));
            File::writeEscape(file.clone(), expStr(__eq_exp.clone())?, JSON.clone());
            File::write(file.clone(), literal!("\"],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations);
            File::write(file, literal!("}"));
            true
        },
        Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { exp: __eq_exp, index: __eq_index, lhs: __eq_lhs, source: __eq_source, .. } => {
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            if assign_type == 1 {
                File::write(file.clone(), literal!("\",\"tag\":\"torn\",\"defines\":[\""));
            } else if assign_type == 2 {
                File::write(file.clone(), literal!("\",\"tag\":\"jacobian\",\"defines\":[\""));
            } else {
                File::write(file.clone(), literal!("\",\"tag\":\"assign\",\"defines\":[\""));
            }
            writeCref(file.clone(), Expression::expCref(metamodelica::AsArg::as_arg(&__eq_lhs))?, JSON.clone())?;
            File::write(file.clone(), literal!("\"],\"uses\":["));
            serializeUses(file.clone(), &(Expression::extractUniqueCrefsFromExpDerPreStart(__eq_exp.clone(), true)?))?;
            File::write(file.clone(), literal!("],\"equation\":[\""));
            File::writeEscape(file.clone(), expStr(__eq_exp.clone())?, JSON.clone());
            File::write(file.clone(), literal!("\"],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations);
            File::write(file, literal!("}"));
            true
        },
        Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: lSystem @ Deref @ SimCode::LinearSystem { .. }, alternativeTearing: None, .. } => {
            let mut i: i32;
            let mut j: i32;
            let mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            let mut jeqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            let mut constantEqns: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            i = ((lSystem.beqs).len() as i32);
            j = ((lSystem.simJac).len() as i32);
            eqs = SimCodeCodegenUtil::sortEqSystems(lSystem.residual.clone())?;
            jeqs = (::match_deref::match_deref! { match &(lSystem.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::JacobianColumn { columnEqns: __esc_jeqs, constantEqns: __esc_constantEqns, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
            jeqs = (*__esc_jeqs).clone();
            constantEqns = (*__esc_constantEqns).clone();
            SimCodeCodegenUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), lSystem.index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            if lSystem.tornSystem.clone() {
                File::write(file.clone(), literal!("\",\"tag\":\"tornsystem\""));
            } else {
                File::write(file.clone(), literal!("\",\"tag\":\"system\""));
            }
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"linear\",\"unknowns\":")); __mm_s.push_str(&*intString(lSystem.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) });
            serializeUses(file.clone(), &(({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
        for mut v in (lSystem.vars.clone()).into_iter().cloned() {
            let __x = (match &*v.clone() {
        SimCodeVar::SimVar { .. } => v.name.clone(),
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })))?;
            File::write(file.clone(), literal!("],\"equation\":[{\"size\":"));
            File::write(file.clone(), intString(i));
            if i != 0 {
                File::write(file.clone(), literal!(",\"density\":"));
                File::writeReal(file.clone(), metamodelica::real_div_checked(metamodelica::OrderedFloat((j) as f64), (metamodelica::OrderedFloat((i * i) as f64)))?, literal!("%.2f"));
            }
            File::write(file.clone(), literal!(",\"A\":["));
            serializeList1(file.clone(), &lSystem.simJac, withOperations, &move |__a0: File::File, __a1: (i32, i32, metamodelica::Ref<SimCode::SimEqSystem>), __a2: bool| serializeLinearCell(__a0, &__a1, __a2))?;
            File::write(file.clone(), literal!("],\"b\":["));
            serializeList(file.clone(), &lSystem.beqs, &serializeExp)?;
            File::write(file.clone(), literal!("]}]"));
            File::write(file.clone(), literal!(",\n\"internal-equations\":["));
            if !((eqs).is_empty()) {
                serializeEquation(file.clone(), &((eqs).head().cloned()?), section, withOperations, lSystem.index.clone(), true, if (lSystem.tornSystem.clone()) {1} else {0})?;
                for mut e in &*(eqs).rest()? {
                    serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, lSystem.index.clone(), false, if (lSystem.tornSystem.clone()) {1} else {0})?;
                }
            }
            File::write(file.clone(), literal!("\n]"));
            File::write(file.clone(), literal!(",\n\"jacobian-equations\":["));
            if !((jeqs).is_empty()) {
                serializeEquation(file.clone(), &((jeqs).head().cloned()?), section, withOperations, lSystem.index.clone(), true, 2)?;
                for mut e in &*(jeqs).rest()? {
                    serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, lSystem.index.clone(), false, 2)?;
                }
            }
            File::write(file, literal!("\n]}"));
            true
        },
        Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: lSystem @ Deref @ SimCode::LinearSystem { .. }, alternativeTearing: Some(atL @ Deref @ SimCode::LinearSystem { .. }), .. } => {
            let mut i: i32;
            let mut j: i32;
            let mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            let mut jeqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            let mut constantEqns: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            i = ((lSystem.beqs).len() as i32);
            j = ((lSystem.simJac).len() as i32);
            eqs = SimCodeCodegenUtil::sortEqSystems(lSystem.residual.clone())?;
            if !((eqs).is_empty()) {
                serializeEquation(file.clone(), &((eqs).head().cloned()?), section, withOperations, lSystem.index.clone(), true, if (lSystem.tornSystem.clone()) {1} else {0})?;
                for mut e in &*(eqs).rest()? {
                    serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, lSystem.index.clone(), false, if (lSystem.tornSystem.clone()) {1} else {0})?;
                }
            }
            jeqs = (::match_deref::match_deref! { match &(lSystem.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::JacobianColumn { columnEqns: __esc_jeqs, constantEqns: __esc_constantEqns, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
            jeqs = (*__esc_jeqs).clone();
            constantEqns = (*__esc_constantEqns).clone();
            SimCodeCodegenUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !((jeqs).is_empty()) {
                File::write(file.clone(), literal!(","));
                serializeEquation(file.clone(), &((jeqs).head().cloned()?), section, withOperations, lSystem.index.clone(), true, 2)?;
                for mut e in &*(jeqs).rest()? {
                    serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, lSystem.index.clone(), false, 2)?;
                }
            }
            if (eqs).is_empty() && (jeqs).is_empty() {
                File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            } else {
                File::write(file.clone(), literal!(",\n{\"eqIndex\":"));
            }
            File::writeInt(file.clone(), lSystem.index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            if lSystem.tornSystem.clone() {
                File::write(file.clone(), literal!("\",\"tag\":\"tornsystem-dynamic\""));
            } else {
                File::write(file.clone(), literal!("\",\"tag\":\"system-dynamic\""));
            }
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"linear\",\"unknowns\":")); __mm_s.push_str(&*intString(lSystem.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) });
            serializeUses(file.clone(), &(({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
        for mut v in (lSystem.vars.clone()).into_iter().cloned() {
            let __x = (match &*v.clone() {
        SimCodeVar::SimVar { .. } => v.name.clone(),
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })))?;
            File::write(file.clone(), literal!("],\"equation\":[{\"size\":"));
            File::write(file.clone(), intString(i));
            if i != 0 {
                File::write(file.clone(), literal!(",\"density\":"));
                File::writeReal(file.clone(), metamodelica::real_div_checked(metamodelica::OrderedFloat((j) as f64), (metamodelica::OrderedFloat((i * i) as f64)))?, literal!("%.2f"));
            }
            File::write(file.clone(), literal!(",\"A\":["));
            serializeList1(file.clone(), &lSystem.simJac, withOperations, &move |__a0: File::File, __a1: (i32, i32, metamodelica::Ref<SimCode::SimEqSystem>), __a2: bool| serializeLinearCell(__a0, &__a1, __a2))?;
            File::write(file.clone(), literal!("],\"b\":["));
            serializeList(file.clone(), &lSystem.beqs, &serializeExp)?;
            File::write(file.clone(), literal!("]}]},"));
            i = ((atL.beqs).len() as i32);
            j = ((atL.simJac).len() as i32);
            eqs = SimCodeCodegenUtil::sortEqSystems(atL.residual.clone())?;
            if !((eqs).is_empty()) {
                serializeEquation(file.clone(), &((eqs).head().cloned()?), section, withOperations, atL.index.clone(), true, if (atL.tornSystem.clone()) {1} else {0})?;
                for mut e in &*(eqs).rest()? {
                    serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, atL.index.clone(), false, if (atL.tornSystem.clone()) {1} else {0})?;
                }
            }
            jeqs = (::match_deref::match_deref! { match &(atL.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::JacobianColumn { columnEqns: __esc_jeqs, constantEqns: __esc_constantEqns, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
            jeqs = (*__esc_jeqs).clone();
            constantEqns = (*__esc_constantEqns).clone();
            SimCodeCodegenUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !((jeqs).is_empty()) {
                File::write(file.clone(), literal!(","));
                serializeEquation(file.clone(), &((jeqs).head().cloned()?), section, withOperations, atL.index.clone(), true, 2)?;
                for mut e in &*(jeqs).rest()? {
                    serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, atL.index.clone(), false, 2)?;
                }
            }
            if (eqs).is_empty() && (jeqs).is_empty() {
                File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            } else {
                File::write(file.clone(), literal!(",\n{\"eqIndex\":"));
            }
            File::writeInt(file.clone(), atL.index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            if atL.tornSystem.clone() {
                File::write(file.clone(), literal!("\",\"tag\":\"tornsystem\""));
            } else {
                File::write(file.clone(), literal!("\",\"tag\":\"system\""));
            }
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"linear\",\"unknowns\":")); __mm_s.push_str(&*intString(atL.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) });
            serializeUses(file.clone(), &(({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
        for mut v in (atL.vars.clone()).into_iter().cloned() {
            let __x = (match &*v.clone() {
        SimCodeVar::SimVar { .. } => v.name.clone(),
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })))?;
            File::write(file.clone(), literal!("],\"equation\":[{\"size\":"));
            File::write(file.clone(), intString(i));
            if i != 0 {
                File::write(file.clone(), literal!(",\"density\":"));
                File::writeReal(file.clone(), metamodelica::real_div_checked(metamodelica::OrderedFloat((j) as f64), (metamodelica::OrderedFloat((i * i) as f64)))?, literal!("%.2f"));
            }
            File::write(file.clone(), literal!(",\"A\":["));
            serializeList1(file.clone(), &atL.simJac, withOperations, &move |__a0: File::File, __a1: (i32, i32, metamodelica::Ref<SimCode::SimEqSystem>), __a2: bool| serializeLinearCell(__a0, &__a1, __a2))?;
            File::write(file.clone(), literal!("],\"b\":["));
            serializeList(file.clone(), &atL.beqs, &serializeExp)?;
            File::write(file, literal!("]}]}"));
            true
        },
        Deref @ SimCode::SimEqSystem::SES_ALGORITHM { statements: Deref @ metamodelica::ListNode::Cons { head: stmt, tail: _ }, index: __eq_index, .. } => {
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut crefs2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*section); __mm_s.push_str(&*literal!("\",\"tag\":\"algorithm\",\"defines\":[")); ArcStr::from(__mm_s) });
            (crefs, crefs2) = Expression::extractUniqueCrefsFromStatmentS(var_field!((**eq).statements, SimCode::SimEqSystem::SES_ALGORITHM))?;
            serializeUses(file.clone(), &crefs)?;
            File::write(file.clone(), literal!("],\"uses\":["));
            serializeUses(file.clone(), &crefs2)?;
            File::write(file.clone(), literal!("],\"equation\":["));
            serializeList(file.clone(), var_field!((**eq).statements, SimCode::SimEqSystem::SES_ALGORITHM), &fnptr!(serializeStatement, File::File, metamodelica::Ref<DAE::Statement>))?;
            File::write(file.clone(), literal!("],\"source\":"));
            serializeSource(file.clone(), &(Algorithm::getStatementSource(metamodelica::AsArg::as_arg(&stmt))?), withOperations);
            File::write(file, literal!("}"));
            true
        },
        Deref @ SimCode::SimEqSystem::SES_INVERSE_ALGORITHM { statements: Deref @ metamodelica::ListNode::Cons { head: stmt, tail: _ }, index: __eq_index, .. } => {
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut crefs2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*section); __mm_s.push_str(&*literal!("\",\"tag\":\"algorithm\",\"defines\":[")); ArcStr::from(__mm_s) });
            (crefs, crefs2) = Expression::extractUniqueCrefsFromStatmentS(var_field!((**eq).statements, SimCode::SimEqSystem::SES_INVERSE_ALGORITHM))?;
            serializeUses(file.clone(), &crefs)?;
            File::write(file.clone(), literal!("],\"uses\":["));
            serializeUses(file.clone(), &crefs2)?;
            File::write(file.clone(), literal!("],\"equation\":["));
            serializeList(file.clone(), var_field!((**eq).statements, SimCode::SimEqSystem::SES_INVERSE_ALGORITHM), &fnptr!(serializeStatement, File::File, metamodelica::Ref<DAE::Statement>))?;
            File::write(file.clone(), literal!("],\"source\":"));
            serializeSource(file.clone(), &(Algorithm::getStatementSource(metamodelica::AsArg::as_arg(&stmt))?), withOperations);
            File::write(file, literal!("}"));
            true
        },
        Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: nlSystem @ Deref @ SimCode::NonlinearSystem { .. }, alternativeTearing: None, .. } => {
            let mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            let mut jeqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            let mut constantEqns: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            eqs = SimCodeCodegenUtil::sortEqSystems(nlSystem.eqs.clone())?;
            jeqs = (::match_deref::match_deref! { match &(nlSystem.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::JacobianColumn { columnEqns: __esc_jeqs, constantEqns: __esc_constantEqns, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
            jeqs = (*__esc_jeqs).clone();
            constantEqns = (*__esc_constantEqns).clone();
            SimCodeCodegenUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), nlSystem.index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            if nlSystem.tornSystem.clone() {
                File::write(file.clone(), literal!("\",\"tag\":\"tornsystem\""));
            } else {
                File::write(file.clone(), literal!("\",\"tag\":\"system\""));
            }
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"non-linear\",\"unknowns\":")); __mm_s.push_str(&*intString(nlSystem.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) });
            serializeUses(file.clone(), &nlSystem.crefs)?;
            File::write(file.clone(), literal!("],\"equation\":[["));
            serializeList(file.clone(), &eqs, &move |__a0: File::File, __a1: metamodelica::Ref<SimCode::SimEqSystem>| serializeEquationIndex(__a0, &__a1))?;
            File::write(file.clone(), literal!("],["));
            serializeList(file.clone(), &jeqs, &move |__a0: File::File, __a1: metamodelica::Ref<SimCode::SimEqSystem>| serializeEquationIndex(__a0, &__a1))?;
            File::write(file.clone(), literal!("]]"));
            File::write(file.clone(), literal!(",\n\"internal-equations\":["));
            if !((eqs).is_empty()) {
                serializeEquation(file.clone(), &((eqs).head().cloned()?), section, withOperations, nlSystem.index.clone(), true, if (nlSystem.tornSystem.clone()) {1} else {0})?;
                for mut e in &*(eqs).rest()? {
                    serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, nlSystem.index.clone(), false, if (nlSystem.tornSystem.clone()) {1} else {0})?;
                }
            }
            File::write(file.clone(), literal!("\n]"));
            File::write(file.clone(), literal!(",\n\"jacobian-equations\":["));
            if !((jeqs).is_empty()) {
                serializeEquation(file.clone(), &((jeqs).head().cloned()?), section, withOperations, nlSystem.index.clone(), true, 2)?;
                for mut e in &*(jeqs).rest()? {
                    serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, nlSystem.index.clone(), false, 2)?;
                }
            }
            File::write(file, literal!("\n]}"));
            true
        },
        Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: nlSystem @ Deref @ SimCode::NonlinearSystem { .. }, alternativeTearing: Some(atNL @ Deref @ SimCode::NonlinearSystem { .. }), .. } => {
            let mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            let mut jeqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            let mut constantEqns: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            eqs = SimCodeCodegenUtil::sortEqSystems(nlSystem.eqs.clone())?;
            serializeEquation(file.clone(), &((eqs).head().cloned()?), section, withOperations, nlSystem.index.clone(), true, if (nlSystem.tornSystem.clone()) {1} else {0})?;
            for mut e in &*(eqs).rest()? {
                serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, nlSystem.index.clone(), false, if (nlSystem.tornSystem.clone()) {1} else {0})?;
            }
            jeqs = (::match_deref::match_deref! { match &(nlSystem.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::JacobianColumn { columnEqns: __esc_jeqs, constantEqns: __esc_constantEqns, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
            jeqs = (*__esc_jeqs).clone();
            constantEqns = (*__esc_constantEqns).clone();
            SimCodeCodegenUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !((jeqs).is_empty()) {
                File::write(file.clone(), literal!(","));
                serializeEquation(file.clone(), &((jeqs).head().cloned()?), section, withOperations, nlSystem.index.clone(), true, 2)?;
                for mut e in &*(jeqs).rest()? {
                    serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, nlSystem.index.clone(), false, 2)?;
                }
            }
            File::write(file.clone(), literal!(",\n{\"eqIndex\":"));
            File::writeInt(file.clone(), nlSystem.index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            if nlSystem.tornSystem.clone() {
                File::write(file.clone(), literal!("\",\"tag\":\"tornsystem\""));
            } else {
                File::write(file.clone(), literal!("\",\"tag\":\"system\""));
            }
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"non-linear\",\"unknowns\":")); __mm_s.push_str(&*intString(nlSystem.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) });
            serializeUses(file.clone(), &nlSystem.crefs)?;
            File::write(file.clone(), literal!("],\"equation\":[["));
            serializeList(file.clone(), &eqs, &move |__a0: File::File, __a1: metamodelica::Ref<SimCode::SimEqSystem>| serializeEquationIndex(__a0, &__a1))?;
            File::write(file.clone(), literal!("],["));
            serializeList(file.clone(), &jeqs, &move |__a0: File::File, __a1: metamodelica::Ref<SimCode::SimEqSystem>| serializeEquationIndex(__a0, &__a1))?;
            File::write(file.clone(), literal!("]]},"));
            eqs = SimCodeCodegenUtil::sortEqSystems(atNL.eqs.clone())?;
            serializeEquation(file.clone(), &((eqs).head().cloned()?), section, withOperations, atNL.index.clone(), true, if (atNL.tornSystem.clone()) {1} else {0})?;
            for mut e in &*(eqs).rest()? {
                serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, atNL.index.clone(), false, if (atNL.tornSystem.clone()) {1} else {0})?;
            }
            jeqs = (::match_deref::match_deref! { match &(atNL.jacobianMatrix.clone()) {
        Some(Deref @ SimCode::JacobianMatrix { columns: Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::JacobianColumn { columnEqns: __esc_jeqs, constantEqns: __esc_constantEqns, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
            jeqs = (*__esc_jeqs).clone();
            constantEqns = (*__esc_constantEqns).clone();
            SimCodeCodegenUtil::sortEqSystems(listAppend(jeqs.clone(), constantEqns.clone()))?
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if !((jeqs).is_empty()) {
                File::write(file.clone(), literal!(","));
                serializeEquation(file.clone(), &((jeqs).head().cloned()?), section, withOperations, atNL.index.clone(), true, 2)?;
                for mut e in &*(jeqs).rest()? {
                    serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, atNL.index.clone(), false, 2)?;
                }
            }
            File::write(file.clone(), literal!(",\n{\"eqIndex\":"));
            File::writeInt(file.clone(), atNL.index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            if atNL.tornSystem.clone() {
                File::write(file.clone(), literal!("\",\"tag\":\"tornsystem\""));
            } else {
                File::write(file.clone(), literal!("\",\"tag\":\"system\""));
            }
            File::write(file.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(",\"display\":\"non-linear\",\"unknowns\":")); __mm_s.push_str(&*intString(atNL.nUnknowns.clone())); __mm_s.push_str(&*literal!(",\"defines\":[")); ArcStr::from(__mm_s) });
            serializeUses(file.clone(), &atNL.crefs)?;
            File::write(file.clone(), literal!("],\"equation\":[["));
            serializeList(file.clone(), &eqs, &move |__a0: File::File, __a1: metamodelica::Ref<SimCode::SimEqSystem>| serializeEquationIndex(__a0, &__a1))?;
            File::write(file.clone(), literal!("],["));
            serializeList(file.clone(), &jeqs, &move |__a0: File::File, __a1: metamodelica::Ref<SimCode::SimEqSystem>| serializeEquationIndex(__a0, &__a1))?;
            File::write(file, literal!("]]}"));
            true
        },
        Deref @ SimCode::SimEqSystem::SES_IFEQUATION { elsebranch: __eq_elsebranch, ifbranches: __eq_ifbranches, index: __eq_index, .. } => {
            let mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
            eqs = listAppend(List::flatten(({
        let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>> = metamodelica::nil();
        for mut e in (__eq_ifbranches.clone()).into_iter().cloned() {
            let __x = Util::tuple22(e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?, __eq_elsebranch.clone());
            serializeEquation(file.clone(), &((eqs).head().cloned()?), section, withOperations, 0, true, 0)?;
            for mut e in &*(eqs).rest()? {
                serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, 0, false, 0)?;
            }
            File::write(file.clone(), literal!(",\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            File::write(file.clone(), literal!("\",\"tag\":\"if-equation\",\"display\":\"if-equation\",\"equation\":["));
            serializeList(file.clone(), metamodelica::AsArg::as_arg(&__eq_ifbranches), &move |__a0: File::File, __a1: (metamodelica::Ref<DAE::Exp>, metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>)| serializeIfBranch(__a0, &__a1))?;
            File::write(file.clone(), literal!(","));
            serializeIfBranch(file.clone(), &((metamodelica::Ref::new(DAE::Exp::BCONST { bool: true }), __eq_elsebranch.clone())))?;
            File::write(file, literal!("]}"));
            true
        },
        Deref @ SimCode::SimEqSystem::SES_MIXED { cont: __eq_cont, discEqs: __eq_discEqs, discVars: __eq_discVars, index: __eq_index, .. } => {
            serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&__eq_cont), section, withOperations, 0, true, 0)?;
            for mut e in &*__eq_discEqs.clone() {
                serializeEquation(file.clone(), metamodelica::AsArg::as_arg(&e), section, withOperations, 0, false, 0)?;
            }
            File::write(file.clone(), literal!(",\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            File::write(file.clone(), literal!("\",\"tag\":\"container\",\"display\":\"mixed\",\"defines\":["));
            serializeUses(file.clone(), &(({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
        for mut v in (__eq_discVars.clone()).into_iter().cloned() {
            let __x = v.name.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })))?;
            File::write(file.clone(), literal!("],\"equation\":["));
            serializeEquationIndex(file.clone(), metamodelica::AsArg::as_arg(&__eq_cont))?;
            for mut e1 in &*__eq_discEqs.clone() {
                File::write(file.clone(), literal!(","));
                serializeEquationIndex(file.clone(), metamodelica::AsArg::as_arg(&e1))?;
            }
            File::write(file, literal!("]}"));
            true
        },
        Deref @ SimCode::SimEqSystem::SES_WHEN { conditions: __eq_conditions, elseWhen: __eq_elseWhen, index: __eq_index, source: __eq_source, whenStmtLst: __eq_whenStmtLst, .. } => {
            let mut whenOp: BackendDAE::WhenOperator;
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            for mut whenOps in &*__eq_whenStmtLst.clone() {
                let () = (match whenOps.clone() {
        mut __esc_whenOp @ BackendDAE::WhenOperator::ASSIGN { .. } => {
            whenOp = __esc_whenOp.clone();
            File::write(file.clone(), literal!("\",\"tag\":\"when\",\"defines\":["));
            serializeExp(file.clone(), var_field!(whenOp.left, BackendDAE::WhenOperator::ASSIGN).clone())?;
            File::write(file.clone(), literal!("],\"uses\":["));
            serializeUses(file.clone(), &(List::union(metamodelica::AsArg::as_arg(&__eq_conditions), &(Expression::extractUniqueCrefsFromExpDerPreStart(var_field!(whenOp.right, BackendDAE::WhenOperator::ASSIGN).clone(), true)?))))?;
            File::write(file.clone(), literal!("],\"equation\":["));
            serializeExp(file.clone(), var_field!(whenOp.right, BackendDAE::WhenOperator::ASSIGN).clone())?;
            File::write(file.clone(), literal!("],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations);
            File::write(file.clone(), literal!("}"));
            ()
        },
        mut __esc_whenOp @ BackendDAE::WhenOperator::REINIT { .. } => {
            whenOp = __esc_whenOp.clone();
            File::write(file.clone(), literal!("\",\"tag\":\"when\",\"defines\":["));
            serializeCref(file.clone(), var_field!(whenOp.stateVar, BackendDAE::WhenOperator::REINIT).clone())?;
            File::write(file.clone(), literal!("],\"uses\":["));
            serializeUses(file.clone(), &(List::union(metamodelica::AsArg::as_arg(&__eq_conditions), &(Expression::extractUniqueCrefsFromExpDerPreStart(var_field!(whenOp.value, BackendDAE::WhenOperator::REINIT).clone(), true)?))))?;
            File::write(file.clone(), literal!("],\"equation\":["));
            serializeExp(file.clone(), var_field!(whenOp.value, BackendDAE::WhenOperator::REINIT).clone())?;
            File::write(file.clone(), literal!("],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations);
            File::write(file.clone(), literal!("}"));
            ()
        },
        mut __esc_whenOp @ BackendDAE::WhenOperator::ASSERT { .. } => {
            whenOp = __esc_whenOp.clone();
            File::write(file.clone(), literal!("\",\"tag\":\"when\""));
            File::write(file.clone(), literal!(",\"uses\":["));
            crefs = listAppend(Expression::extractUniqueCrefsFromExpDerPreStart(var_field!(whenOp.condition, BackendDAE::WhenOperator::ASSERT).clone(), true)?, Expression::extractUniqueCrefsFromExpDerPreStart(var_field!(whenOp.message, BackendDAE::WhenOperator::ASSERT).clone(), true)?);
            serializeUses(file.clone(), &(List::union(metamodelica::AsArg::as_arg(&__eq_conditions), &crefs)))?;
            File::write(file.clone(), literal!("],\"equation\":["));
            serializeExp(file.clone(), var_field!(whenOp.message, BackendDAE::WhenOperator::ASSERT).clone())?;
            File::write(file.clone(), literal!("],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations);
            File::write(file.clone(), literal!("}"));
            ()
        },
        mut __esc_whenOp @ BackendDAE::WhenOperator::TERMINATE { .. } => {
            whenOp = __esc_whenOp.clone();
            File::write(file.clone(), literal!("\",\"tag\":\"when\""));
            File::write(file.clone(), literal!(",\"uses\":["));
            serializeUses(file.clone(), &(List::union(metamodelica::AsArg::as_arg(&__eq_conditions), &(Expression::extractUniqueCrefsFromExpDerPreStart(var_field!(whenOp.message, BackendDAE::WhenOperator::TERMINATE).clone(), true)?))))?;
            File::write(file.clone(), literal!("],\"equation\":["));
            serializeExp(file.clone(), var_field!(whenOp.message, BackendDAE::WhenOperator::TERMINATE).clone())?;
            File::write(file.clone(), literal!("],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations);
            File::write(file.clone(), literal!("}"));
            ()
        },
        mut __esc_whenOp @ BackendDAE::WhenOperator::NORETCALL { .. } => {
            whenOp = __esc_whenOp.clone();
            File::write(file.clone(), literal!("\",\"tag\":\"when\""));
            File::write(file.clone(), literal!(",\"uses\":["));
            serializeUses(file.clone(), &(List::union(metamodelica::AsArg::as_arg(&__eq_conditions), &(Expression::extractUniqueCrefsFromExpDerPreStart(var_field!(whenOp.exp, BackendDAE::WhenOperator::NORETCALL).clone(), true)?))))?;
            File::write(file.clone(), literal!("],\"equation\":["));
            serializeExp(file.clone(), var_field!(whenOp.exp, BackendDAE::WhenOperator::NORETCALL).clone())?;
            File::write(file.clone(), literal!("],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations);
            File::write(file.clone(), literal!("}"));
            ()
        },
    });
            }
            let () = (::match_deref::match_deref! { match &(__eq_elseWhen.clone()) {
        Some(e) => {
            if SimCodeCodegenUtil::simEqSystemIndex(metamodelica::AsArg::as_arg(&e))? != 0 {
                serializeEquation(file, metamodelica::AsArg::as_arg(&e), section, withOperations, 0, false, 0)?;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            true
        },
        Deref @ SimCode::SimEqSystem::SES_FOR_LOOP { cref: __eq_cref, exp: __eq_exp, index: __eq_index, source: __eq_source, .. } => {
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            if parent != 0 {
                File::write(file.clone(), literal!(",\"parent\":"));
                File::writeInt(file.clone(), parent, literal!("%d"));
            }
            File::write(file.clone(), literal!(",\"section\":\""));
            File::write(file.clone(), section.clone());
            if assign_type == 1 {
                File::write(file.clone(), literal!("\",\"tag\":\"torn\",\"defines\":[\""));
            } else if assign_type == 2 {
                File::write(file.clone(), literal!("\",\"tag\":\"jacobian\",\"defines\":[\""));
            } else {
                File::write(file.clone(), literal!("\",\"tag\":\"assign\",\"defines\":[\""));
            }
            writeCref(file.clone(), __eq_cref.clone(), JSON.clone())?;
            File::write(file.clone(), literal!("\"],\"uses\":["));
            serializeUses(file.clone(), &(Expression::extractUniqueCrefsFromExpDerPreStart(__eq_exp.clone(), true)?))?;
            File::write(file.clone(), literal!("],\"equation\":[\""));
            File::writeEscape(file.clone(), expStr(__eq_exp.clone())?, JSON.clone());
            File::write(file.clone(), literal!("\"],\"source\":"));
            serializeSource(file.clone(), metamodelica::AsArg::as_arg(&__eq_source), withOperations);
            File::write(file, literal!("}"));
            true
        },
        Deref @ SimCode::SimEqSystem::SES_ALIAS { aliasOf: __eq_aliasOf, index: __eq_index } => {
            File::write(file.clone(), literal!("\n{\"eqIndex\":"));
            File::writeInt(file.clone(), __eq_index.clone(), literal!("%d"));
            File::write(file.clone(), literal!(",\"tag\":\"alias\",\"equation\":["));
            File::writeInt(file.clone(), __eq_aliasOf.clone(), literal!("%d"));
            File::write(file.clone(), literal!("],\"section\":\""));
            File::write(file.clone(), section.clone());
            File::write(file, literal!("\"}"));
            true
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("serializeEquation failed: ")); __mm_s.push_str(&*anyString(eq.clone())); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("SimCode/SerializeTaskSystemInfo.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(success)
}

fn serializeLinearCell(
    mut file: File::File,
    mut cell: &(i32, i32, metamodelica::Ref<SimCode::SimEqSystem>),
    mut withOperations: bool,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(cell) {
        (i, j, eq @ Deref @ SimCode::SimEqSystem::SES_RESIDUAL { .. }) => {
            File::write(file.clone(), literal!("{\"row\":"));
            File::write(file.clone(), intString(i.clone()));
            File::write(file.clone(), literal!(",\"column\":"));
            File::write(file.clone(), intString(j.clone()));
            File::write(file.clone(), literal!(",\"exp\":\""));
            File::writeEscape(file.clone(), expStr(var_field!((**eq).exp, SimCode::SimEqSystem::SES_RESIDUAL).clone())?, JSON.clone());
            File::write(file.clone(), literal!("\",\"source\":"));
            serializeSource(file.clone(), var_field!((**eq).source, SimCode::SimEqSystem::SES_RESIDUAL), withOperations);
            File::write(file, literal!("}"));
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("SerializeTaskSystemInfo.serializeLinearCell failed. Expected only SES_RESIDUAL as input.")])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn serializeUses(
    mut file: File::File,
    mut crefs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match crefs {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: cr, tail: Deref @ metamodelica::ListNode::Nil } => {
            File::write(file.clone(), literal!("\""));
            writeCref(file.clone(), cr.clone(), JSON.clone())?;
            File::write(file, literal!("\""));
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: cr, tail: rest } => {
            File::write(file.clone(), literal!("\""));
            writeCref(file.clone(), cr.clone(), JSON.clone())?;
            File::write(file.clone(), literal!("\","));
            serializeUses(file, rest)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn serializeStatement(mut file: File::File, mut stmt: metamodelica::Ref<DAE::Statement>) -> () {
    File::write(file.clone(), literal!("\""));
    File::writeEscape(
        file.clone(),
        System::trim(DAEDump::ppStatementStr(stmt), literal!(" \u{c}\n\r\t\u{b}")),
        JSON.clone(),
    );
    File::write(file, literal!("\""));
    ()
}

fn serializeList<ArgType: Clone + 'static + metamodelica::gc::MMTrace>(
    mut file: File::File,
    mut lst: &metamodelica::List<ArgType>,
    mut func: &dyn ::std::ops::Fn(File::File, ArgType) -> Result<()>,
) -> Result<()> {
    pub type FuncType<ArgType: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(File::File, ArgType) -> Result<()> + 'static>;

    let () = (::match_deref::match_deref! { match lst {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: a, tail: Deref @ metamodelica::ListNode::Nil } => {
            func(file, a.clone())?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: a, tail: rest } => {
            func(file.clone(), a.clone())?;
            File::write(file.clone(), literal!(","));
            serializeList(file, rest, func)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn serializeList1<
    ArgType: Clone + 'static + metamodelica::gc::MMTrace,
    Extra: Clone + 'static + metamodelica::gc::MMTrace,
>(
    mut file: File::File,
    mut lst: &metamodelica::List<ArgType>,
    mut extra: Extra,
    mut func: &dyn ::std::ops::Fn(File::File, ArgType, Extra) -> Result<()>,
) -> Result<()> {
    pub type FuncType<ArgType: Clone + 'static, Extra: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(File::File, ArgType, Extra) -> Result<()> + 'static>;

    let () = (::match_deref::match_deref! { match lst {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: a, tail: Deref @ metamodelica::ListNode::Nil } => {
            func(file, a.clone(), extra)?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: a, tail: rest } => {
            func(file.clone(), a.clone(), extra.clone())?;
            File::write(file.clone(), literal!(","));
            serializeList1(file, rest, extra, func)?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn serializeExp(mut file: File::File, mut exp: metamodelica::Ref<DAE::Exp>) -> Result<()> {
    File::write(file.clone(), literal!("\""));
    File::writeEscape(file.clone(), expStr(exp)?, JSON.clone());
    File::write(file, literal!("\""));
    Ok(())
}

fn serializeCref(mut file: File::File, mut cr: metamodelica::Ref<DAE::ComponentRef>) -> Result<()> {
    File::write(file.clone(), literal!("\""));
    writeCref(file.clone(), cr, JSON.clone())?;
    File::write(file, literal!("\""));
    Ok(())
}

fn serializeString(mut file: File::File, mut string: ArcStr) -> () {
    File::write(file.clone(), literal!("\""));
    File::writeEscape(file.clone(), string, JSON.clone());
    File::write(file, literal!("\""));
    ()
}

fn serializePath(mut file: File::File, mut path: metamodelica::Ref<Absyn::Path>) -> () {
    let mut p: metamodelica::Ref<Absyn::Path> = path;
    let mut b: bool = true;
    File::write(file.clone(), literal!("\""));
    while b {
        (p, b) = (match &*p {
            Absyn::Path::IDENT { name: __p_name } => {
                File::writeEscape(file.clone(), __p_name.clone(), JSON.clone());
                (p, false)
            }
            Absyn::Path::QUALIFIED {
                name: __p_name,
                path: __p_path,
            } => {
                File::writeEscape(file.clone(), __p_name.clone(), JSON.clone());
                File::write(file.clone(), literal!("."));
                (__p_path.clone(), true)
            }
            Absyn::Path::FULLYQUALIFIED { path: __p_path } => (__p_path.clone(), true),
        });
    }
    File::write(file, literal!("\""));
    ()
}

fn serializeEquationIndex(mut file: File::File, mut eq: &metamodelica::Ref<SimCode::SimEqSystem>) -> Result<()> {
    File::writeInt(file, SimCodeCodegenUtil::simEqSystemIndex(eq)?, literal!("%d"));
    Ok(())
}

fn serializeIfBranch(
    mut file: File::File,
    mut branch: &(
        metamodelica::Ref<DAE::Exp>,
        metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
    ),
) -> Result<()> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    (exp, eqs) = branch.clone();
    File::write(file.clone(), literal!("["));
    serializeExp(file.clone(), exp)?;
    File::write(file.clone(), literal!(","));
    serializeList(file.clone(), &eqs, &move |__a0: File::File,
                                             __a1: metamodelica::Ref<
        SimCode::SimEqSystem,
    >| serializeEquationIndex(__a0, &__a1))?;
    File::write(file, literal!("]"));
    Ok(())
}

fn serializeSource(
    mut file: File::File,
    mut source: &metamodelica::Ref<DAE::ElementSource>,
    mut withOperations: bool,
) -> () {
    File::write(file, literal!("{}"));
    ()
}
