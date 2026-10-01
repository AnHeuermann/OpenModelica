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

use crate::BackendDAEOptimize;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendUtil;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::InlineArrayEquations;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::Inline;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::HashTableCG;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Debug;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// late inline functions stuff
//
// =============================================================================
pub(crate) fn lateInlineFunction(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDAE = inlineCalls(
        list![
            openmodelica_frontend_types::DAE::InlineType::NORM_INLINE,
            openmodelica_frontend_types::DAE::InlineType::AFTER_INDEX_RED_INLINE
        ],
        inDAE,
    )?;
    Ok(outDAE)
}

// =============================================================================
// normal inline functions stuff
//
// =============================================================================
pub(crate) fn normalInlineFunction(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    if Flags::getConfigEnum(Flags::INLINE_METHOD.clone())? == 1 {
        outDAE = inlineCalls(list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE], inDAE)?;
    } else {
        outDAE = inlineCallsBDAE(list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE], inDAE)?;
    }
    Ok(outDAE)
}

// =============================================================================
// inline calls stuff
//
// =============================================================================
fn inlineCalls(
    mut inITLst: metamodelica::List<DAE::InlineType>,
    mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut tpl: (
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    );
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    match '__try0: {
        shared = inBackendDAE.shared.clone();
        eqs = inBackendDAE.eqs.clone();
        tpl = (Some(shared.functionTree.clone()), inITLst.clone());
        eqs = unwrap_break_err!(List::map1(eqs.clone(), &inlineEquationSystem, tpl.clone()), '__try0);
        assign_field!(
            shared.globalKnownVars =
                unwrap_break_err!(inlineVariables(shared.globalKnownVars.clone(), tpl.clone()), '__try0).0,
            shared.externalObjects =
                unwrap_break_err!(inlineVariables(shared.externalObjects.clone(), tpl.clone()), '__try0).0,
            shared.initialEqs = unwrap_break_err!(inlineEquationArray(shared.initialEqs.clone(), &tpl), '__try0).0,
            shared.removedEqs = unwrap_break_err!(inlineEquationArray(shared.removedEqs.clone(), &tpl), '__try0).0
        );
        unwrap_break_err!(inlineEventInfo(&shared.eventInfo, tpl.clone()), '__try0);
        outBackendDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
            eqs: eqs.clone(),
            shared: shared.clone(),
        });
        Ok::<_, &'static str>((eqs.clone(), outBackendDAE.clone(), shared.clone(), tpl.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            eqs = __try0_o0;
            outBackendDAE = __try0_o1;
            shared = __try0_o2;
            tpl = __try0_o3;
        }
        Err(__try0_err) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::traceln(literal!("BackendInline.inlineCalls failed"))?;
            }
            return Err(__try0_err);
        }
    }
    Ok(outBackendDAE)
}

fn inlineEquationSystem(
    mut eqs: metamodelica::Ref<BackendDAE::EqSystem>,
    mut tpl: (
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut oeqs: metamodelica::Ref<BackendDAE::EqSystem> = eqs;
    assign_field!(oeqs.orderedVars = propagateAttributes(oeqs.orderedVars.clone(), oeqs.orderedEqs.clone(), &tpl)?);
    inlineVariables(oeqs.orderedVars.clone(), tpl.clone())?;
    inlineEquationArray(oeqs.orderedEqs.clone(), &tpl)?;
    inlineEquationArray(oeqs.removedEqs.clone(), &tpl)?;
    Ok(oeqs)
}

// =============================================================================
//        ATTRIBUTE PROPAGATION (min/max/nominal/unit/... see #15947)
// =============================================================================
fn propagateAttributes(
    mut vars: BackendDAE::Variables,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut tpl: &(
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
) -> Result<BackendDAE::Variables> {
    let mut vars: BackendDAE::Variables = vars;
    vars = BackendEquation::traverseEquationArray(
        eqns,
        &({
            let __pe_b1 = tpl.clone();
            move |__pe_a0, __pe_a2| propagateEqnAttributes(__pe_a0, &__pe_b1, __pe_a2)
        }),
        vars,
    )?;
    Ok(vars)
}

fn propagateEqnAttributes(
    mut eqn: metamodelica::Ref<BackendDAE::Equation>,
    mut tpl: &(
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
    mut vars: BackendDAE::Variables,
) -> Result<(metamodelica::Ref<BackendDAE::Equation>, BackendDAE::Variables)> {
    let mut eqn: metamodelica::Ref<BackendDAE::Equation> = eqn;
    let mut vars: BackendDAE::Variables = vars;
    vars = (match &*eqn {
        BackendDAE::Equation::EQUATION {
            exp: lhs, scalar: rhs, ..
        } => propagateOutput(lhs.clone(), rhs.clone(), tpl, vars)?,
        BackendDAE::Equation::ARRAY_EQUATION {
            left: lhs, right: rhs, ..
        } => propagateOutput(lhs.clone(), rhs.clone(), tpl, vars)?,
        BackendDAE::Equation::COMPLEX_EQUATION {
            left: lhs, right: rhs, ..
        } => propagateOutput(lhs.clone(), rhs.clone(), tpl, vars)?,
        BackendDAE::Equation::SOLVED_EQUATION {
            componentRef: cr,
            exp: rhs,
            ..
        } => propagateOutput(Expression::crefExp(cr.clone())?, rhs.clone(), tpl, vars)?,
        _ => vars,
    });
    (_, vars) = BackendEquation::traverseExpsOfEquation(
        eqn.clone(),
        (std::sync::Arc::new({
            let __pe_b1 = tpl.clone();
            move |__pe_a0, __pe_a2| propagateInputsExpDeep(__pe_a0, &__pe_b1, __pe_a2)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::Exp>,
                        BackendDAE::Variables,
                    ) -> Result<(metamodelica::Ref<DAE::Exp>, BackendDAE::Variables)>
                    + 'static,
            >),
        vars,
    )?;
    Ok((eqn, vars))
}

fn propagateInputsExpDeep(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut tpl: &(
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
    mut vars: BackendDAE::Variables,
) -> Result<(metamodelica::Ref<DAE::Exp>, BackendDAE::Variables)> {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut vars: BackendDAE::Variables = vars;
    (exp, vars) = Expression::traverseExpBottomUp(
        exp,
        &({
            let __pe_b1 = tpl.clone();
            move |__pe_a0, __pe_a2| Ok(propagateInputsExp(__pe_a0, &__pe_b1, __pe_a2))
        }),
        vars,
    )?;
    Ok((exp, vars))
}

fn propagateOutput(
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut tpl: &(
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
    mut vars: BackendDAE::Variables,
) -> Result<BackendDAE::Variables> {
    let mut vars: BackendDAE::Variables = vars;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut call_exp: metamodelica::Ref<DAE::Exp>;
    let mut p: metamodelica::Ref<Absyn::Path>;
    let mut outs: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    if Expression::isCref(&lhs) && isInlinableCall(&rhs, tpl) {
        let __pa0 = ::match_deref::match_deref! { match &(lhs) {
            Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        cr = metamodelica::Own::own(__pa0);
        call_exp = rhs;
    } else if Expression::isCref(&rhs) && isInlinableCall(&lhs, tpl) {
        let __pa1 = ::match_deref::match_deref! { match &(rhs) {
            Deref @ DAE::Exp::CREF { componentRef: __pa1, .. } => __pa1.clone(),
            _ => return Err("pattern mismatch"),
        } };
        cr = metamodelica::Own::own(__pa1);
        call_exp = lhs;
    } else {
        return Ok(vars);
    }
    let __pa2 = ::match_deref::match_deref! { match &(call_exp) {
        Deref @ DAE::Exp::CALL { path: __pa2, .. } => __pa2.clone(),
        _ => return Err("pattern mismatch"),
    } };
    p = metamodelica::Own::own(__pa2);
    if '__try3: {
        outs = unwrap_break_err!(DAEUtil::getFunctionOutputVars(&(unwrap_break_err!(Inline::getFunction(p.clone(), tpl), '__try3))), '__try3);
        if ((outs).len() as i32) == 1 {
            vars = mergeVarOntoCref(&cr, &(unwrap_break_err!((outs).head().cloned(), '__try3)), vars.clone());
        }
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    Ok(vars)
}

fn propagateInputsExp(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut tpl: &(
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
    mut vars: BackendDAE::Variables,
) -> (metamodelica::Ref<DAE::Exp>, BackendDAE::Variables) {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut vars: BackendDAE::Variables = vars;
    vars = (::match_deref::match_deref! { match &(&*exp) {
        Deref @ DAE::Exp::CALL { path: p, expLst: args, attr: Deref @ DAE::CallAttributes { inlineType: it, .. } } if (Inline::checkInlineType(it.clone(), tpl)) => {
            let mut ins: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            if '__try0: {
                ins = unwrap_break_err!(DAEUtil::getFunctionInputVars(&(unwrap_break_err!(Inline::getFunction(p.clone(), tpl), '__try0))), '__try0);
                if ((ins).len() as i32) == ((args).len() as i32) {
                    for mut t in &*List::zip(ins.clone(), args.clone()) {
                        vars = mergeVarOntoArg(&(Util::tuple21(t.clone())), &(Util::tuple22(t.clone())), vars.clone());
                    }
                }
                Ok::<(), &'static str>(())
            }.is_err() {
            }
            vars
        },
        _ => {
            vars
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exp, vars)
}

fn isInlinableCall(
    mut exp: &metamodelica::Ref<DAE::Exp>,
    mut tpl: &(
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match exp {
        Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { inlineType: it, .. }, .. } => {
            Inline::checkInlineType(it.clone(), tpl)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn mergeVarOntoArg(
    mut velem: &metamodelica::Ref<DAE::Element>,
    mut arg: &metamodelica::Ref<DAE::Exp>,
    mut vars: BackendDAE::Variables,
) -> BackendDAE::Variables {
    let mut vars: BackendDAE::Variables = vars;
    vars = (match &**arg {
        DAE::Exp::CREF { componentRef: cr, .. } => mergeVarOntoCref(cr, velem, vars),
        _ => vars,
    });
    vars
}

fn mergeVarOntoCref(
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut velem: &metamodelica::Ref<DAE::Element>,
    mut vars: BackendDAE::Variables,
) -> BackendDAE::Variables {
    let mut vars: BackendDAE::Variables = vars;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut index: i32;
    let mut src: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let mut dst: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    if '__try0: {
        (var, index) = unwrap_break_err!(BackendVariable::getVarSingle(cr, &vars), '__try0);
        let __pa1 = ::match_deref::match_deref! { match &((*velem)) {
            Deref @ DAE::Element::VAR { variableAttributesOption: __pa1, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        src = metamodelica::Own::own(__pa1);
        dst = unwrap_break_err!(mergeDAEAttributes(BackendVariable::getVariableAttributes(&var), src.clone()), '__try0);
        var = BackendVariable::setVarAttributes(var.clone(), dst.clone());
        vars = unwrap_break_err!(BackendVariable::setVarAt(vars.clone(), index, var.clone()), '__try0);
        Ok::<(), &'static str>(())
    }
    .is_err()
    {}
    vars
}

fn mergeDAEAttributes(
    mut dstOpt: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut srcOpt: Option<metamodelica::Ref<DAE::VariableAttributes>>,
) -> Result<Option<metamodelica::Ref<DAE::VariableAttributes>>> {
    let mut outOpt: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let mut dst: metamodelica::Ref<DAE::VariableAttributes>;
    let mut src: metamodelica::Ref<DAE::VariableAttributes>;
    if (srcOpt).is_none() {
        outOpt = dstOpt;
        return Ok(outOpt);
    }
    src = srcOpt.ok_or("pattern mismatch")?;
    dst = (::match_deref::match_deref! { match &(&dstOpt) {
        Some(dst) if (sameAttrKind(metamodelica::AsArg::as_arg(&dst), &src)) => dst.clone(),
        _ => emptyAttrLike(src.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outOpt = (::match_deref::match_deref! { match &((dst.clone(), src.clone())) {
        (Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { .. }, Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { .. }) => {
            assign_variant_field!(dst => DAE::VariableAttributes::VAR_ATTR_REAL;
                quantity = mergeOpt(var_field!((*dst).quantity, DAE::VariableAttributes::VAR_ATTR_REAL).clone(), attrConst(var_field!((*src).quantity, DAE::VariableAttributes::VAR_ATTR_REAL).clone())?),
                unit = mergeOpt(var_field!((*dst).unit, DAE::VariableAttributes::VAR_ATTR_REAL).clone(), attrConst(var_field!((*src).unit, DAE::VariableAttributes::VAR_ATTR_REAL).clone())?),
                displayUnit = mergeOpt(var_field!((*dst).displayUnit, DAE::VariableAttributes::VAR_ATTR_REAL).clone(), attrConst(var_field!((*src).displayUnit, DAE::VariableAttributes::VAR_ATTR_REAL).clone())?),
                min = tightestBound(var_field!((*dst).min, DAE::VariableAttributes::VAR_ATTR_REAL).clone(), attrConst(var_field!((*src).min, DAE::VariableAttributes::VAR_ATTR_REAL).clone())?, true)?,
                max = tightestBound(var_field!((*dst).max, DAE::VariableAttributes::VAR_ATTR_REAL).clone(), attrConst(var_field!((*src).max, DAE::VariableAttributes::VAR_ATTR_REAL).clone())?, false)?,
                start = mergeOpt(var_field!((*dst).start, DAE::VariableAttributes::VAR_ATTR_REAL).clone(), attrConst(var_field!((*src).start, DAE::VariableAttributes::VAR_ATTR_REAL).clone())?),
                fixed = mergeOpt(var_field!((*dst).fixed, DAE::VariableAttributes::VAR_ATTR_REAL).clone(), attrConst(var_field!((*src).fixed, DAE::VariableAttributes::VAR_ATTR_REAL).clone())?),
                nominal = mergeOpt(var_field!((*dst).nominal, DAE::VariableAttributes::VAR_ATTR_REAL).clone(), attrConst(var_field!((*src).nominal, DAE::VariableAttributes::VAR_ATTR_REAL).clone())?)
            );
            Some(dst)
        },
        (Deref @ DAE::VariableAttributes::VAR_ATTR_INT { .. }, Deref @ DAE::VariableAttributes::VAR_ATTR_INT { .. }) => {
            assign_variant_field!(dst => DAE::VariableAttributes::VAR_ATTR_INT;
                quantity = mergeOpt(var_field!((*dst).quantity, DAE::VariableAttributes::VAR_ATTR_INT).clone(), attrConst(var_field!((*src).quantity, DAE::VariableAttributes::VAR_ATTR_INT).clone())?),
                min = tightestBound(var_field!((*dst).min, DAE::VariableAttributes::VAR_ATTR_INT).clone(), attrConst(var_field!((*src).min, DAE::VariableAttributes::VAR_ATTR_INT).clone())?, true)?,
                max = tightestBound(var_field!((*dst).max, DAE::VariableAttributes::VAR_ATTR_INT).clone(), attrConst(var_field!((*src).max, DAE::VariableAttributes::VAR_ATTR_INT).clone())?, false)?,
                start = mergeOpt(var_field!((*dst).start, DAE::VariableAttributes::VAR_ATTR_INT).clone(), attrConst(var_field!((*src).start, DAE::VariableAttributes::VAR_ATTR_INT).clone())?),
                fixed = mergeOpt(var_field!((*dst).fixed, DAE::VariableAttributes::VAR_ATTR_INT).clone(), attrConst(var_field!((*src).fixed, DAE::VariableAttributes::VAR_ATTR_INT).clone())?)
            );
            Some(dst)
        },
        (Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { .. }, Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { .. }) => {
            assign_variant_field!(dst => DAE::VariableAttributes::VAR_ATTR_BOOL;
                quantity = mergeOpt(var_field!((*dst).quantity, DAE::VariableAttributes::VAR_ATTR_BOOL).clone(), attrConst(var_field!((*src).quantity, DAE::VariableAttributes::VAR_ATTR_BOOL).clone())?),
                start = mergeOpt(var_field!((*dst).start, DAE::VariableAttributes::VAR_ATTR_BOOL).clone(), attrConst(var_field!((*src).start, DAE::VariableAttributes::VAR_ATTR_BOOL).clone())?),
                fixed = mergeOpt(var_field!((*dst).fixed, DAE::VariableAttributes::VAR_ATTR_BOOL).clone(), attrConst(var_field!((*src).fixed, DAE::VariableAttributes::VAR_ATTR_BOOL).clone())?)
            );
            Some(dst)
        },
        (Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { .. }, Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { .. }) => {
            assign_variant_field!(dst => DAE::VariableAttributes::VAR_ATTR_STRING;
                quantity = mergeOpt(var_field!((*dst).quantity, DAE::VariableAttributes::VAR_ATTR_STRING).clone(), attrConst(var_field!((*src).quantity, DAE::VariableAttributes::VAR_ATTR_STRING).clone())?),
                start = mergeOpt(var_field!((*dst).start, DAE::VariableAttributes::VAR_ATTR_STRING).clone(), attrConst(var_field!((*src).start, DAE::VariableAttributes::VAR_ATTR_STRING).clone())?)
            );
            Some(dst)
        },
        (Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { .. }, Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { .. }) => {
            assign_variant_field!(dst => DAE::VariableAttributes::VAR_ATTR_ENUMERATION;
                quantity = mergeOpt(var_field!((*dst).quantity, DAE::VariableAttributes::VAR_ATTR_ENUMERATION).clone(), attrConst(var_field!((*src).quantity, DAE::VariableAttributes::VAR_ATTR_ENUMERATION).clone())?),
                min = mergeOpt(var_field!((*dst).min, DAE::VariableAttributes::VAR_ATTR_ENUMERATION).clone(), attrConst(var_field!((*src).min, DAE::VariableAttributes::VAR_ATTR_ENUMERATION).clone())?),
                max = mergeOpt(var_field!((*dst).max, DAE::VariableAttributes::VAR_ATTR_ENUMERATION).clone(), attrConst(var_field!((*src).max, DAE::VariableAttributes::VAR_ATTR_ENUMERATION).clone())?),
                start = mergeOpt(var_field!((*dst).start, DAE::VariableAttributes::VAR_ATTR_ENUMERATION).clone(), attrConst(var_field!((*src).start, DAE::VariableAttributes::VAR_ATTR_ENUMERATION).clone())?),
                fixed = mergeOpt(var_field!((*dst).fixed, DAE::VariableAttributes::VAR_ATTR_ENUMERATION).clone(), attrConst(var_field!((*src).fixed, DAE::VariableAttributes::VAR_ATTR_ENUMERATION).clone())?)
            );
            Some(dst)
        },
        _ => dstOpt,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outOpt)
}

fn sameAttrKind(
    mut a: &metamodelica::Ref<DAE::VariableAttributes>,
    mut b: &metamodelica::Ref<DAE::VariableAttributes>,
) -> bool {
    let mut same: bool;
    same = (::match_deref::match_deref! { match (a, b) {
        (Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { .. }, Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { .. }) => true,
        (Deref @ DAE::VariableAttributes::VAR_ATTR_INT { .. }, Deref @ DAE::VariableAttributes::VAR_ATTR_INT { .. }) => true,
        (Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { .. }, Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { .. }) => true,
        (Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { .. }, Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { .. }) => true,
        (Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { .. }, Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    same
}

fn emptyAttrLike(mut attr: metamodelica::Ref<DAE::VariableAttributes>) -> metamodelica::Ref<DAE::VariableAttributes> {
    let mut empty: metamodelica::Ref<DAE::VariableAttributes>;
    empty = (match &*attr {
        DAE::VariableAttributes::VAR_ATTR_REAL { .. } => DAE::emptyVarAttrReal().clone(),
        DAE::VariableAttributes::VAR_ATTR_INT { .. } => DAE::emptyVarAttrInt().clone(),
        DAE::VariableAttributes::VAR_ATTR_BOOL { .. } => DAE::emptyVarAttrBool().clone(),
        DAE::VariableAttributes::VAR_ATTR_STRING { .. } => DAE::emptyVarAttrString().clone(),
        DAE::VariableAttributes::VAR_ATTR_ENUMERATION { .. } => {
            metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION {
                quantity: None,
                min: None,
                max: None,
                start: None,
                fixed: None,
                equationBound: None,
                isProtected: None,
                finalPrefix: None,
                startOrigin: None,
            })
        }
        _ => attr,
    });
    empty
}

fn attrConst(mut inOpt: Option<metamodelica::Ref<DAE::Exp>>) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut outOpt: Option<metamodelica::Ref<DAE::Exp>>;
    outOpt = (::match_deref::match_deref! { match &(inOpt.clone()) {
        Some(e) if (!(Expression::expHasCrefs(e.clone())?)) => {
            inOpt
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outOpt)
}

fn mergeOpt(
    mut dst: Option<metamodelica::Ref<DAE::Exp>>,
    mut src: Option<metamodelica::Ref<DAE::Exp>>,
) -> Option<metamodelica::Ref<DAE::Exp>> {
    let mut res: Option<metamodelica::Ref<DAE::Exp>> = if ((dst).is_some()) { dst.clone() } else { src.clone() };
    res
}

fn tightestBound(
    mut dst: Option<metamodelica::Ref<DAE::Exp>>,
    mut src: Option<metamodelica::Ref<DAE::Exp>>,
    mut isMin: bool,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut res: Option<metamodelica::Ref<DAE::Exp>>;
    let mut de: metamodelica::Ref<DAE::Exp>;
    let mut se: metamodelica::Ref<DAE::Exp>;
    let mut dv: metamodelica::Real;
    let mut sv: metamodelica::Real;
    let mut dc: bool;
    let mut sc: bool;
    if (dst).is_none() {
        res = src;
    } else if (src).is_none() {
        res = dst;
    } else {
        let __pa0 = ::match_deref::match_deref! { match &(dst.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        de = metamodelica::Own::own(__pa0);
        let __pa1 = ::match_deref::match_deref! { match &(src.clone()) {
            Some(__pa1) => __pa1.clone(),
            _ => return Err("pattern mismatch"),
        } };
        se = metamodelica::Own::own(__pa1);
        (dc, dv) = constNumber(&de);
        (sc, sv) = constNumber(&se);
        if dc && sc {
            if isMin {
                res = if (sv > dv) { src } else { dst };
            } else {
                res = if (sv < dv) { src } else { dst };
            }
        } else {
            res = dst;
        }
    }
    Ok(res)
}

fn constNumber(mut exp: &metamodelica::Ref<DAE::Exp>) -> (bool, metamodelica::Real) {
    let mut isConst: bool;
    let mut value: metamodelica::Real;
    (isConst, value) = (match &**exp {
        DAE::Exp::RCONST { real: __exp_real } => (true, __exp_real.clone()),
        DAE::Exp::ICONST { integer: __exp_integer } => (true, intReal(__exp_integer.clone())),
        _ => (false, metamodelica::OrderedFloat(0.0_f64)),
    });
    (isConst, value)
}

fn inlineEquationArray(
    mut inEquationArray: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inElementList: &(
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
) -> Result<(
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    bool,
)> {
    let mut outEquationArray: metamodelica::Ref<
        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
    > = inEquationArray;
    let mut oInlined: bool;
    if let Ok(__iflet0) = inlineEquationOptArray(outEquationArray.clone(), inElementList) {
        oInlined = __iflet0;
    } else {
        if Flags::isSet(Flags::FAILTRACE.clone())? {
            Debug::trace(literal!("Inline.inlineEquationArray failed\n"))?;
        }
        return Err("fail");
    }
    Ok((outEquationArray, oInlined))
}

fn inlineEquationOptArray(
    mut inEqnArray: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut fns: &(
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
) -> Result<bool> {
    let mut oInlined: bool = false;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut inlined: bool;
    for mut i in 1..=ExpandableArray::getLastUsedIndex(inEqnArray.clone()) {
        if ExpandableArray::occupied(i, inEqnArray.clone()) {
            (eqn, inlined) = inlineEq(&(ExpandableArray::get(i, inEqnArray.clone())?), fns);
            if inlined {
                ExpandableArray::update(i, eqn, inEqnArray.clone())?;
                oInlined = true;
            }
        }
    }
    Ok(oInlined)
}

pub(crate) fn inlineEq(
    mut inEquation: &metamodelica::Ref<BackendDAE::Equation>,
    mut fns: &(
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
) -> (metamodelica::Ref<BackendDAE::Equation>, bool) {
    let mut outEquation: metamodelica::Ref<BackendDAE::Equation>;
    let mut inlined: bool;
    (outEquation, inlined) = 'mc: {
        let __mc_input = &**inEquation;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source, attr } => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut source = (*source).clone();
                    (e1_1, source, b1, _) = Inline::inlineExp(e1.clone(), fns.clone(), source.clone());
                    (e2_1, source, b2, _) = Inline::inlineExp(e2.clone(), fns.clone(), source.clone());
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: e1_1.clone(), scalar: e2_1.clone(), source: source.clone(), attr: attr.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize, left: e1, right: e2, source, attr, recordSize } => {
                            let mut e1_1: metamodelica::Ref<DAE::Exp>;
                            let mut e2_1: metamodelica::Ref<DAE::Exp>;
                            let mut b1: bool;
                            let mut b2: bool;
                            let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
                            let mut source = (*source).clone();
                            (e1_1, source, b1, _) = Inline::inlineExp(e1.clone(), fns.clone(), source.clone());
                            (e2_1, source, b2, _) = Inline::inlineExp(e2.clone(), fns.clone(), source.clone());
                            let true = (b1 || b2) else { return Err("pattern mismatch") };
                            eqn = (::match_deref::match_deref! { match &((&*e1_1, &*e2_1)) {
                (Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: e1.clone(), scalar: e2.clone(), source: source.clone(), attr: attr.clone() }),
                _ => metamodelica::Ref::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: dimSize.clone(), left: e1_1.clone(), right: e2_1.clone(), source: source.clone(), attr: attr.clone(), recordSize: recordSize.clone() }),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                            Ok((eqn.clone(), true))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::FOR_EQUATION { iter: e, start: e1, stop: e2, body: eqn, source, attr } => {
                    let mut eqn = (*eqn).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(inlineEq(metamodelica::AsArg::as_arg(&eqn), fns)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    eqn = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(BackendDAE::Equation::FOR_EQUATION { iter: e.clone(), start: e1.clone(), stop: e2.clone(), body: eqn.clone(), source: source.clone(), attr: attr.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cref, exp: e, source, attr } => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut source = (*source).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Inline::inlineExp(e.clone(), fns.clone(), source.clone())) {
                        (__pa0, __pa1, true, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e_1 = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    Ok((metamodelica::Ref::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: cref.clone(), exp: e_1.clone(), source: source.clone(), attr: attr.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, source, attr } => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut source = (*source).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Inline::inlineExp(e.clone(), fns.clone(), source.clone())) {
                        (__pa0, __pa1, true, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e_1 = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    Ok((metamodelica::Ref::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e_1.clone(), source: source.clone(), attr: attr.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ALGORITHM { size, alg: Deref @ DAE::Algorithm { statementLst: stmts }, source, expand: crefExpand, attr } => {
                    let mut alg: metamodelica::Ref<DAE::Algorithm>;
                    let mut stmts1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let __pa0 = ::match_deref::match_deref! { match &(Inline::inlineStatements(metamodelica::AsArg::as_arg(&stmts), fns, metamodelica::nil(), false)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    stmts1 = metamodelica::Own::own(__pa0);
                    alg = metamodelica::Ref::new(DAE::Algorithm { statementLst: stmts1.clone() });
                    Ok((metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: alg.clone(), source: source.clone(), expand: crefExpand.clone(), attr: attr.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::WHEN_EQUATION { size, whenEquation: weq, source, attr } => {
                    let mut weq_1: metamodelica::Ref<BackendDAE::WhenEquation>;
                    let mut source = (*source).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inlineWhenEq(metamodelica::AsArg::as_arg(&weq), fns, metamodelica::AsArg::as_arg(&source))?) {
                        (__pa0, __pa1, true) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    weq_1 = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    Ok((metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: weq_1.clone(), source: source.clone(), attr: attr.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::COMPLEX_EQUATION { size, left: e1, right: e2, source, attr } => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut source = (*source).clone();
                    (e1_1, source, b1, _) = Inline::inlineExp(e1.clone(), fns.clone(), source.clone());
                    (e2_1, source, b2, _) = Inline::inlineExp(e2.clone(), fns.clone(), source.clone());
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size.clone(), left: e1_1.clone(), right: e2_1.clone(), source: source.clone(), attr: attr.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::IF_EQUATION { conditions: explst, eqnstrue: eqnslst, eqnsfalse: eqns, source, attr } => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut explst = (*explst).clone();
                    let mut eqnslst = (*eqnslst).clone();
                    let mut eqns = (*eqns).clone();
                    let mut source = (*source).clone();
                    (explst, source, b1) = Inline::inlineExps(explst.clone(), fns, source.clone());
                    (eqnslst, b2) = inlineEqsLst(metamodelica::AsArg::as_arg(&eqnslst), fns, metamodelica::nil(), false);
                    (eqns, b3) = inlineEqs(metamodelica::AsArg::as_arg(&eqns), fns, metamodelica::nil(), false);
                    let true = (b1 || b2 || b3) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(BackendDAE::Equation::IF_EQUATION { conditions: explst.clone(), eqnstrue: eqnslst.clone(), eqnsfalse: eqns.clone(), source: source.clone(), attr: attr.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inEquation.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outEquation, inlined)
}

fn inlineEqsLst<'__b>(
    mut inEqnsList: &'__b metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inFunctions: &'__b (
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
    mut iAcc: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut iInlined: bool,
) -> (
    metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    bool,
) {
    '__tco: loop {
        ::match_deref::match_deref! { match inEqnsList {
            Deref @ metamodelica::ListNode::Nil => {
                return (iAcc.reverse(), iInlined)
            },
            Deref @ metamodelica::ListNode::Cons { head: eqn, tail: rest } => {
                let mut acc: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
                let mut inlined: bool;
                let mut eqn = (*eqn).clone();
                (eqn, inlined) = inlineEqs(metamodelica::AsArg::as_arg(&eqn), inFunctions, metamodelica::nil(), false);
                { (inEqnsList, inFunctions, iAcc, iInlined) = (rest, inFunctions, metamodelica::cons(eqn.clone(), iAcc), inlined || iInlined); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn inlineEqs<'__b>(
    mut inEqnsList: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inFunctions: &'__b (
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
    mut iAcc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut iInlined: bool,
) -> (metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>, bool) {
    '__tco: loop {
        ::match_deref::match_deref! { match inEqnsList {
            Deref @ metamodelica::ListNode::Nil => {
                return (iAcc.reverse(), iInlined)
            },
            Deref @ metamodelica::ListNode::Cons { head: eqn, tail: rest } => {
                let mut acc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut inlined: bool;
                let mut eqn = (*eqn).clone();
                (eqn, inlined) = inlineEq(metamodelica::AsArg::as_arg(&eqn), inFunctions);
                { (inEqnsList, inFunctions, iAcc, iInlined) = (rest, inFunctions, metamodelica::cons(eqn.clone(), iAcc), inlined || iInlined); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn inlineWhenEq(
    mut inWhenEquation: &metamodelica::Ref<BackendDAE::WhenEquation>,
    mut fns: &(
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
    mut inSource: &metamodelica::Ref<DAE::ElementSource>,
) -> Result<(
    metamodelica::Ref<BackendDAE::WhenEquation>,
    metamodelica::Ref<DAE::ElementSource>,
    bool,
)> {
    let mut outWhenEquation: metamodelica::Ref<BackendDAE::WhenEquation>;
    let mut outSource: metamodelica::Ref<DAE::ElementSource>;
    let mut inlined: bool;
    (outWhenEquation, outSource, inlined) = (match &**inWhenEquation {
        BackendDAE::WhenEquation {
            condition: cond,
            whenStmtLst,
            elsewhenPart: oelsewe,
        } => {
            let mut source: metamodelica::Ref<DAE::ElementSource>;
            let mut b1: bool;
            let mut b2: bool;
            let mut b3: bool;
            let mut elsewe: metamodelica::Ref<BackendDAE::WhenEquation>;
            let mut cond = (*cond).clone();
            let mut whenStmtLst = (*whenStmtLst).clone();
            let mut oelsewe = (*oelsewe).clone();
            (cond, source, b1, _) = Inline::inlineExp(cond.clone(), fns.clone(), inSource.clone());
            (whenStmtLst, b2) = inlineWhenOps(metamodelica::AsArg::as_arg(&whenStmtLst), fns.clone());
            if (oelsewe).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(oelsewe.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                elsewe = metamodelica::Own::own(__pa0);
                (elsewe, source, b3) = inlineWhenEq(&elsewe, fns, &source)?;
                oelsewe = Some(elsewe);
            } else {
                oelsewe = None;
                b3 = false;
            }
            (
                metamodelica::Ref::new(BackendDAE::WhenEquation {
                    condition: cond.clone(),
                    whenStmtLst: whenStmtLst.clone(),
                    elsewhenPart: oelsewe.clone(),
                }),
                source,
                b1 || b2 || b3,
            )
        }
    });
    Ok((outWhenEquation, outSource, inlined))
}

fn inlineWhenOps(
    mut inWhenOps: &metamodelica::List<BackendDAE::WhenOperator>,
    mut fns: (
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
) -> (metamodelica::List<BackendDAE::WhenOperator>, bool) {
    let mut outWhenOps: metamodelica::List<BackendDAE::WhenOperator> = metamodelica::nil();
    let mut inlined: bool = false;
    for mut whenOp in &**inWhenOps {
        let () = (match whenOp.clone() {
            BackendDAE::WhenOperator::ASSIGN {
                left: ref e1,
                right: ref e2,
                source: mut source,
            } => {
                let mut b: bool;
                let mut e2 = e2.clone();
                let mut source = source.clone();
                (e2, source, b, _) = Inline::inlineExp(e2.clone(), fns.clone(), source.clone());
                outWhenOps = metamodelica::cons(
                    if (b) {
                        BackendDAE::WhenOperator::ASSIGN {
                            left: e1.clone(),
                            right: e2.clone(),
                            source: source.clone(),
                        }
                    } else {
                        whenOp.clone()
                    },
                    outWhenOps,
                );
                inlined = inlined || b;
                ()
            }
            BackendDAE::WhenOperator::REINIT {
                stateVar: ref cr,
                value: ref e2,
                source: mut source,
            } => {
                let mut b: bool;
                let mut e2 = e2.clone();
                let mut source = source.clone();
                (e2, source, b, _) = Inline::inlineExp(e2.clone(), fns.clone(), source.clone());
                outWhenOps = metamodelica::cons(
                    if (b) {
                        BackendDAE::WhenOperator::REINIT {
                            stateVar: cr.clone(),
                            value: e2.clone(),
                            source: source.clone(),
                        }
                    } else {
                        whenOp.clone()
                    },
                    outWhenOps,
                );
                inlined = inlined || b;
                ()
            }
            BackendDAE::WhenOperator::ASSERT {
                condition: ref e1,
                message: ref e2,
                level: mut level,
                source: mut source,
            } => {
                let mut b: bool;
                let mut b2: bool;
                let mut e1 = e1.clone();
                let mut e2 = e2.clone();
                let mut source = source.clone();
                (e1, source, b, _) = Inline::inlineExp(e1.clone(), fns.clone(), source.clone());
                (e2, source, b2, _) = Inline::inlineExp(e2.clone(), fns.clone(), source.clone());
                outWhenOps = metamodelica::cons(
                    if (b || b2) {
                        BackendDAE::WhenOperator::ASSERT {
                            condition: e1.clone(),
                            message: e2.clone(),
                            level: level.clone(),
                            source: source.clone(),
                        }
                    } else {
                        whenOp.clone()
                    },
                    outWhenOps,
                );
                inlined = inlined || b || b2;
                ()
            }
            BackendDAE::WhenOperator::TERMINATE {
                message: ref e1,
                source: mut source,
            } => {
                let mut b: bool;
                let mut e1 = e1.clone();
                let mut source = source.clone();
                (e1, source, b, _) = Inline::inlineExp(e1.clone(), fns.clone(), source.clone());
                outWhenOps = metamodelica::cons(
                    if (b) {
                        BackendDAE::WhenOperator::TERMINATE {
                            message: e1.clone(),
                            source: source.clone(),
                        }
                    } else {
                        whenOp.clone()
                    },
                    outWhenOps,
                );
                inlined = inlined || b;
                ()
            }
            BackendDAE::WhenOperator::NORETCALL {
                exp: ref e1,
                source: mut source,
            } => {
                let mut b: bool;
                let mut e1 = e1.clone();
                let mut source = source.clone();
                (e1, source, b, _) = Inline::inlineExp(e1.clone(), fns.clone(), source.clone());
                outWhenOps = metamodelica::cons(
                    if (b) {
                        BackendDAE::WhenOperator::NORETCALL {
                            exp: e1.clone(),
                            source: source.clone(),
                        }
                    } else {
                        whenOp.clone()
                    },
                    outWhenOps,
                );
                inlined = inlined || b;
                ()
            }
        });
    }
    (outWhenOps, inlined)
}

fn inlineVariables(
    mut inVariables: BackendDAE::Variables,
    mut inElementList: (
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
) -> Result<(BackendDAE::Variables, bool)> {
    let mut outVariables: BackendDAE::Variables;
    let mut inlined: bool = false;
    (outVariables, inlined) = 'mc: {
        let __mc_input = (&inVariables, inElementList);
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (BackendDAE::Variables { varArr: BackendDAE::VariableArray { varOptArr: vararr, .. }, .. }, fns) => {
                    let mut inlined: bool = inlined.clone();
                    inlined = inlineVarOptArray(vararr.clone(), fns.clone())?;
                    Ok(((inVariables.clone(), inlined), inlined.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            inlined = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("Inline.inlineVariables failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outVariables, inlined))
}

fn inlineVarOptArray(
    mut inVarArray: metamodelica::Array<Option<metamodelica::Ref<BackendDAE::Var>>>,
    mut fns: (
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
) -> Result<bool> {
    let mut oInlined: bool = false;
    let mut b: bool;
    let mut var: Option<metamodelica::Ref<BackendDAE::Var>>;
    for mut index in 1..=metamodelica::arrayLength(inVarArray.clone()) {
        var = ({
            let __elt = (*metamodelica::index_checked(&inVarArray.borrow(), index)?).clone();
            __elt
        });
        (var, b) = inlineVarOpt(var, fns.clone());
        if b {
            metamodelica::arrayUpdate(inVarArray.clone(), index, var)?;
        }
        oInlined = oInlined || b;
    }
    Ok(oInlined)
}

fn inlineVarOpt(
    mut inVarOption: Option<metamodelica::Ref<BackendDAE::Var>>,
    mut fns: (
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
) -> (Option<metamodelica::Ref<BackendDAE::Var>>, bool) {
    let mut outVarOption: Option<metamodelica::Ref<BackendDAE::Var>>;
    let mut inlined: bool;
    (outVarOption, inlined) = (::match_deref::match_deref! { match &(inVarOption.clone()) {
        None => {
            (None, false)
        },
        Some(var) => {
            let mut var2: metamodelica::Ref<BackendDAE::Var>;
            let mut b: bool;
            (var2, b) = inlineVar(var.clone(), fns);
            (if (referenceEq(&*(var.clone()),&*(&*var2))) {inVarOption} else {Some(var2)}, b)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outVarOption, inlined)
}

fn inlineVar(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inElementList: (
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
) -> (metamodelica::Ref<BackendDAE::Var>, bool) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut inlined: bool;
    (outVar, inlined) = (match &*inVar {
        BackendDAE::Var {
            varName,
            varKind,
            varDirection,
            varParallelism,
            varType,
            bindExp: bind,
            tplExp,
            arryDim: arrayDim,
            source,
            values,
            tearingSelectOption: ts,
            hideResult,
            comment,
            connectorType: ct,
            innerOuter: io,
            unreplaceable,
            initNonlinear: _,
            encrypted: e,
        } => {
            let mut values1: Option<metamodelica::Ref<DAE::VariableAttributes>>;
            let mut b1: bool;
            let mut b2: bool;
            let mut bind = (*bind).clone();
            let mut source = (*source).clone();
            (bind, source, b1) = Inline::inlineExpOpt(bind.clone(), inElementList.clone(), source.clone());
            (values1, source, b2) = Inline::inlineStartAttribute(values.clone(), source.clone(), inElementList);
            (
                metamodelica::Ref::new(BackendDAE::Var {
                    varName: varName.clone(),
                    varKind: varKind.clone(),
                    varDirection: varDirection.clone(),
                    varParallelism: varParallelism.clone(),
                    varType: varType.clone(),
                    bindExp: bind.clone(),
                    tplExp: tplExp.clone(),
                    arryDim: arrayDim.clone(),
                    source: source.clone(),
                    values: values1,
                    tearingSelectOption: ts.clone(),
                    hideResult: hideResult.clone(),
                    comment: comment.clone(),
                    connectorType: ct.clone(),
                    innerOuter: io.clone(),
                    unreplaceable: unreplaceable.clone(),
                    initNonlinear: false,
                    encrypted: e.clone(),
                }),
                b1 || b2,
            )
        }
        _ => (inVar, false),
    });
    (outVar, inlined)
}

fn inlineEventInfo(
    mut inEventInfo: &BackendDAE::EventInfo,
    mut fns: (
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inEventInfo.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let BackendDAE::EventInfo {
                zeroCrossings: mut zclst,
                relations: mut relations,
                ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            inlineZeroCrossings(zclst.zc.clone(), fns.clone())?;
            inlineZeroCrossings(relations.zc.clone(), fns.clone())?;
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("Inline.inlineEventInfo failed\n"))?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn inlineZeroCrossings(
    mut inStmts: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>,
    mut fns: (
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
) -> Result<()> {
    DoubleEnded::mapNoCopy_1(
        inStmts,
        &fnptr!(
            inlineZeroCrossing,
            BackendDAE::ZeroCrossing,
            (
                Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
                metamodelica::List<DAE::InlineType>
            )
        ),
        fns,
    )?;
    Ok(())
}

fn inlineZeroCrossing(
    mut zc: BackendDAE::ZeroCrossing,
    mut fns: (
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
) -> BackendDAE::ZeroCrossing {
    let mut zc: BackendDAE::ZeroCrossing = zc;
    zc = (match zc.clone() {
        BackendDAE::ZeroCrossing { relation_: ref e, .. } => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            (e_1, _, _, _) = Inline::inlineExp(e.clone(), fns, DAE::emptyElementSource().clone());
            if (!(referenceEq(&*(e.clone()), &*(&*e_1)))) {
                BackendDAE::ZeroCrossing {
                    index: zc.index.clone(),
                    relation_: e_1,
                    occurEquLst: zc.occurEquLst.clone(),
                    iter: zc.iter.clone(),
                }
            } else {
                zc
            }
        }
        _ => zc,
    });
    zc
}

// =============================================================================
// inline append functions
//
// =============================================================================
fn inlineCallsBDAE(
    mut inITLst: metamodelica::List<DAE::InlineType>,
    mut inBackendDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut tpl: (
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    );
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    match '__try0: {
        if unwrap_break_err!(Flags::isSet(Flags::DUMPBACKENDINLINE.clone()), '__try0) {
            if unwrap_break_err!(Flags::getConfigEnum(Flags::INLINE_METHOD.clone()), '__try0) == 1 {
                metamodelica::print(literal!("\n############ BackendInline Method: replace ############"));
            } else if unwrap_break_err!(Flags::getConfigEnum(Flags::INLINE_METHOD.clone()), '__try0) == 2 {
                metamodelica::print(literal!("\n############ BackendInline Method: append ############"));
            }
        }
        shared = inBackendDAE.shared.clone();
        eqs = inBackendDAE.eqs.clone();
        tpl = (Some(shared.functionTree.clone()), inITLst.clone());
        if unwrap_break_err!(Flags::getConfigEnum(Flags::INLINE_METHOD.clone()), '__try0) == 1 {
            eqs = unwrap_break_err!(List::map1(eqs.clone(), &inlineEquationSystem, tpl.clone()), '__try0);
        } else if unwrap_break_err!(Flags::getConfigEnum(Flags::INLINE_METHOD.clone()), '__try0) == 2 {
            eqs = unwrap_break_err!(List::map2(eqs.clone(), &inlineEquationSystemAppend, tpl.clone(), shared.clone()), '__try0);
        }
        if unwrap_break_err!(Flags::isSet(Flags::DUMPBACKENDINLINE.clone()), '__try0) {
            unwrap_break_err!(BackendDump::dumpEqSystems(&eqs, &(literal!("Result DAE after Inline."))), '__try0);
        }
        assign_field!(
            shared.globalKnownVars =
                unwrap_break_err!(inlineVariables(shared.globalKnownVars.clone(), tpl.clone()), '__try0).0,
            shared.externalObjects =
                unwrap_break_err!(inlineVariables(shared.externalObjects.clone(), tpl.clone()), '__try0).0,
            shared.initialEqs = unwrap_break_err!(inlineEquationArray(shared.initialEqs.clone(), &tpl), '__try0).0,
            shared.removedEqs = unwrap_break_err!(inlineEquationArray(shared.removedEqs.clone(), &tpl), '__try0).0
        );
        outBackendDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
            eqs: eqs.clone(),
            shared: shared.clone(),
        });
        Ok::<_, &'static str>((eqs.clone(), outBackendDAE.clone(), shared.clone(), tpl.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            eqs = __try0_o0;
            outBackendDAE = __try0_o1;
            shared = __try0_o2;
            tpl = __try0_o3;
        }
        Err(__try0_err) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::traceln(literal!("BackendInline.inlineCallsBDAE failed"))?;
            }
            return Err(__try0_err);
        }
    }
    outBackendDAE = BackendDAEOptimize::simplifyComplexFunction1(outBackendDAE, false)?;
    Ok(outBackendDAE)
}

fn inlineEquationSystemAppend(
    mut eqs: metamodelica::Ref<BackendDAE::EqSystem>,
    mut tpl: (
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut oeqs: metamodelica::Ref<BackendDAE::EqSystem> = eqs;
    let mut shared: metamodelica::Ref<BackendDAE::Shared> = ishared;
    let mut new: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut inlined: bool = true;
    let mut eqnsArray: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    (eqnsArray, new, inlined, shared) = inlineEquationArrayAppend(oeqs.orderedEqs.clone(), tpl.clone(), shared)?;
    if inlined {
        assign_field!(oeqs.orderedEqs = eqnsArray);
        new = inlineEquationSystemAppend(new, tpl, shared)?;
        oeqs = BackendDAEUtil::mergeEqSystems(&new, oeqs)?;
    }
    Ok(oeqs)
}

fn inlineEquationArrayAppend(
    mut inEquationArray: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut fns: (
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
    mut iShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::Ref<BackendDAE::EqSystem>,
    bool,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut outEquationArray: metamodelica::Ref<
        ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>,
    > = inEquationArray;
    let mut outEqs: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oInlined: bool;
    let mut shared: metamodelica::Ref<BackendDAE::Shared> = iShared;
    match '__try0: {
        (outEqs, oInlined, shared) = unwrap_break_err!(inlineEquationOptArrayAppend(outEquationArray.clone(), fns.clone(), shared.clone()), '__try0);
        Ok::<_, &'static str>((oInlined.clone(), outEqs.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            oInlined = __try0_o0;
            outEqs = __try0_o1;
        }
        Err(_) => {
            oInlined = false;
            outEqs = BackendDAEUtil::createEqSystem(
                BackendVariable::listVar(metamodelica::nil())?,
                BackendEquation::listEquation(&(metamodelica::nil()))?,
                metamodelica::nil(),
                openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
                BackendEquation::emptyEqns(),
            );
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace(literal!("BackendInline.inlineEquationArrayAppend failed\n"))?;
            }
        }
    }
    Ok((outEquationArray, outEqs, oInlined, shared))
}

fn inlineEquationOptArrayAppend(
    mut inEqnArray: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut fns: (
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
    mut iShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    bool,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut outEqs: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oInlined: bool = false;
    let mut shared: metamodelica::Ref<BackendDAE::Shared> = iShared;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut inlined: bool;
    let mut tmpEqs: metamodelica::Ref<BackendDAE::EqSystem>;
    outEqs = BackendDAEUtil::createEqSystem(
        BackendVariable::listVar(metamodelica::nil())?,
        BackendEquation::listEquation(&(metamodelica::nil()))?,
        metamodelica::nil(),
        openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
        BackendEquation::emptyEqns(),
    );
    for mut i in 1..=ExpandableArray::getLastUsedIndex(inEqnArray.clone()) {
        if ExpandableArray::occupied(i, inEqnArray.clone()) {
            (eqn, tmpEqs, inlined, shared) =
                inlineEqAppend_debug(ExpandableArray::get(i, inEqnArray.clone())?, fns.clone(), shared)?;
            if inlined {
                outEqs = BackendDAEUtil::mergeEqSystems(&tmpEqs, outEqs)?;
                ExpandableArray::update(i, eqn, inEqnArray.clone())?;
                oInlined = true;
            }
        }
    }
    Ok((outEqs, oInlined, shared))
}

pub(crate) fn inlineEqAppend_debug(
    mut inEquationOption: metamodelica::Ref<BackendDAE::Equation>,
    mut inElementList: (
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
    mut iShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<BackendDAE::Equation>,
    metamodelica::Ref<BackendDAE::EqSystem>,
    bool,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut outEquationOption: metamodelica::Ref<BackendDAE::Equation>;
    let mut outEqs: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut inlined: bool;
    let mut shared: metamodelica::Ref<BackendDAE::Shared> = iShared;
    outEqs = BackendDAEUtil::createEqSystem(
        BackendVariable::listVar(metamodelica::nil())?,
        BackendEquation::listEquation(&(metamodelica::nil()))?,
        metamodelica::nil(),
        openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
        BackendEquation::emptyEqns(),
    );
    (outEquationOption, outEqs, inlined, shared) =
        inlineEqAppend(inEquationOption.clone(), inElementList, outEqs, shared);
    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? && inlined {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Equation before inline: "));
            __mm_s.push_str(&*BackendDump::equationString(&inEquationOption)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        BackendDump::dumpEqSystem(
            outEqs.clone(),
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Tmp DAE after Inline Eqn: "));
                __mm_s.push_str(&*BackendDump::equationString(&outEquationOption)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            }),
        )?;
    }
    Ok((outEquationOption, outEqs, inlined, shared))
}

fn inlineEqAppend(
    mut inEquation: metamodelica::Ref<BackendDAE::Equation>,
    mut fns: (
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
    mut inEqs: metamodelica::Ref<BackendDAE::EqSystem>,
    mut iShared: metamodelica::Ref<BackendDAE::Shared>,
) -> (
    metamodelica::Ref<BackendDAE::Equation>,
    metamodelica::Ref<BackendDAE::EqSystem>,
    bool,
    metamodelica::Ref<BackendDAE::Shared>,
) {
    let mut outEquation: metamodelica::Ref<BackendDAE::Equation>;
    let mut outEqs: metamodelica::Ref<BackendDAE::EqSystem> =
        <metamodelica::Ref<BackendDAE::EqSystem> as ::std::default::Default>::default();
    let mut inlined: bool;
    let mut shared: metamodelica::Ref<BackendDAE::Shared> = iShared;
    (outEquation, outEqs, inlined) = 'mc: {
        let __mc_input = inEquation.clone();
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source, attr } => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut e1 = (*e1).clone();
                    let mut e2 = (*e2).clone();
                    let mut source = (*source).clone();
                    let mut outEqs: metamodelica::Ref<BackendDAE::EqSystem> = outEqs.clone();
                    let mut shared: metamodelica::Ref<BackendDAE::Shared> = shared.clone();
                    (e1, source, outEqs, b1, shared) = inlineCallsAppend(e1.clone(), fns.clone(), source.clone(), inEqs.clone(), shared.clone());
                    (e2, source, outEqs, b2, shared) = inlineCallsAppend(e2.clone(), fns.clone(), source.clone(), outEqs.clone(), shared.clone());
                    b3 = b1 || b2;
                    Ok(((BackendEquation::generateEquation(e1.clone(), e2.clone(), source.clone(), attr.clone())?, outEqs.clone(), b3), outEqs.clone(), shared.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outEqs = __wb0;
            shared = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: e1, right: e2, source, attr, .. } => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut e1 = (*e1).clone();
                    let mut e2 = (*e2).clone();
                    let mut source = (*source).clone();
                    let mut outEqs: metamodelica::Ref<BackendDAE::EqSystem> = outEqs.clone();
                    let mut shared: metamodelica::Ref<BackendDAE::Shared> = shared.clone();
                    (e1, source, outEqs, b1, shared) = inlineCallsAppend(e1.clone(), fns.clone(), source.clone(), inEqs.clone(), shared.clone());
                    (e2, source, outEqs, b2, shared) = inlineCallsAppend(e2.clone(), fns.clone(), source.clone(), outEqs.clone(), shared.clone());
                    b3 = b1 || b2;
                    if b2 && Expression::isScalar(metamodelica::AsArg::as_arg(&e1))? && Expression::isTuple(metamodelica::AsArg::as_arg(&e2)) {
                        e2 = metamodelica::Ref::new(DAE::Exp::TSUB { exp: e2.clone(), ix: 1, ty: Expression::r#typeof(e1.clone())? });
                    }
                    Ok(((BackendEquation::generateEquation(e1.clone(), e2.clone(), source.clone(), attr.clone())?, outEqs.clone(), b3), outEqs.clone(), shared.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outEqs = __wb0;
            shared = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ARRAY_EQUATION { left: e1, right: e2, source, attr, .. } => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut e1 = (*e1).clone();
                    let mut e2 = (*e2).clone();
                    let mut source = (*source).clone();
                    let mut outEqs: metamodelica::Ref<BackendDAE::EqSystem> = outEqs.clone();
                    let mut shared: metamodelica::Ref<BackendDAE::Shared> = shared.clone();
                    (e1, source, outEqs, b1, shared) = inlineCallsAppend(e1.clone(), fns.clone(), source.clone(), inEqs.clone(), shared.clone());
                    (e2, source, outEqs, b2, shared) = inlineCallsAppend(e2.clone(), fns.clone(), source.clone(), outEqs.clone(), shared.clone());
                    b3 = b1 || b2;
                    Ok(((BackendEquation::generateEquation(e1.clone(), e2.clone(), source.clone(), attr.clone())?, outEqs.clone(), b3), outEqs.clone(), shared.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outEqs = __wb0;
            shared = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::SOLVED_EQUATION { componentRef: cref, exp: e2, source, attr } => {
                    let mut b2: bool;
                    let mut e2 = (*e2).clone();
                    let mut source = (*source).clone();
                    let mut outEqs: metamodelica::Ref<BackendDAE::EqSystem> = outEqs.clone();
                    let mut shared: metamodelica::Ref<BackendDAE::Shared> = shared.clone();
                    (e2, source, outEqs, b2, shared) = inlineCallsAppend(e2.clone(), fns.clone(), source.clone(), inEqs.clone(), shared.clone());
                    Ok(((metamodelica::Ref::new(BackendDAE::Equation::SOLVED_EQUATION { componentRef: cref.clone(), exp: e2.clone(), source: source.clone(), attr: attr.clone() }), outEqs.clone(), b2), outEqs.clone(), shared.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outEqs = __wb0;
            shared = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1, source, attr } => {
                    let mut b1: bool;
                    let mut e1 = (*e1).clone();
                    let mut source = (*source).clone();
                    let mut outEqs: metamodelica::Ref<BackendDAE::EqSystem> = outEqs.clone();
                    let mut shared: metamodelica::Ref<BackendDAE::Shared> = shared.clone();
                    (e1, source, outEqs, b1, shared) = inlineCallsAppend(e1.clone(), fns.clone(), source.clone(), inEqs.clone(), shared.clone());
                    Ok(((metamodelica::Ref::new(BackendDAE::Equation::RESIDUAL_EQUATION { exp: e1.clone(), source: source.clone(), attr: attr.clone() }), outEqs.clone(), b1), outEqs.clone(), shared.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outEqs = __wb0;
            shared = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                eqn @ Deref @ BackendDAE::Equation::ALGORITHM { size, alg: Deref @ DAE::Algorithm { statementLst: stmts }, source, expand: crefExpand, attr } => {
                    let mut b1: bool;
                    let mut stmts1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut eqn = (*eqn).clone();
                    (stmts1, b1) = Inline::inlineStatements(metamodelica::AsArg::as_arg(&stmts), &fns, metamodelica::nil(), false);
                    if b1 {
                        eqn = metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: size.clone(), alg: metamodelica::Ref::new(DAE::Algorithm { statementLst: stmts1.clone() }), source: source.clone(), expand: crefExpand.clone(), attr: attr.clone() });
                    }
                    Ok((eqn.clone(), inEqs.clone(), b1))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                eqn @ Deref @ BackendDAE::Equation::WHEN_EQUATION { size, whenEquation: weq, source, attr } => {
                    let mut b1: bool;
                    let mut weq_1: metamodelica::Ref<BackendDAE::WhenEquation>;
                    let mut eqn = (*eqn).clone();
                    let mut source = (*source).clone();
                    (weq_1, source, b1) = inlineWhenEq(metamodelica::AsArg::as_arg(&weq), &fns, metamodelica::AsArg::as_arg(&source))?;
                    if b1 {
                        eqn = metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: weq_1.clone(), source: source.clone(), attr: attr.clone() });
                    }
                    Ok((eqn.clone(), inEqs.clone(), b1))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                eqn @ Deref @ BackendDAE::Equation::IF_EQUATION { conditions: explst, eqnstrue: eqnslst, eqnsfalse: eqns, source, attr } => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut eqn = (*eqn).clone();
                    let mut explst = (*explst).clone();
                    let mut eqnslst = (*eqnslst).clone();
                    let mut eqns = (*eqns).clone();
                    let mut source = (*source).clone();
                    (explst, source, b1) = Inline::inlineExps(explst.clone(), &fns, source.clone());
                    (eqnslst, b2) = inlineEqsLst(metamodelica::AsArg::as_arg(&eqnslst), &fns, metamodelica::nil(), false);
                    (eqns, b3) = inlineEqs(metamodelica::AsArg::as_arg(&eqns), &fns, metamodelica::nil(), false);
                    b3 = b1 || b2 || b3;
                    if b3 {
                        eqn = metamodelica::Ref::new(BackendDAE::Equation::IF_EQUATION { conditions: explst.clone(), eqnstrue: eqnslst.clone(), eqnsfalse: eqns.clone(), source: source.clone(), attr: attr.clone() });
                    }
                    Ok((eqn.clone(), inEqs.clone(), b3))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inEquation.clone(), inEqs.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outEquation, outEqs, inlined, shared)
}

fn inlineCallsAppend(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut fns: (
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    ),
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut inEqs: metamodelica::Ref<BackendDAE::EqSystem>,
    mut iShared: metamodelica::Ref<BackendDAE::Shared>,
) -> (
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::ElementSource>,
    metamodelica::Ref<BackendDAE::EqSystem>,
    bool,
    metamodelica::Ref<BackendDAE::Shared>,
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outSource: metamodelica::Ref<DAE::ElementSource>;
    let mut outEqs: metamodelica::Ref<BackendDAE::EqSystem> =
        <metamodelica::Ref<BackendDAE::EqSystem> as ::std::default::Default>::default();
    let mut inlined: bool;
    let mut shared: metamodelica::Ref<BackendDAE::Shared> = iShared;
    (outExp, outSource, outEqs, inlined) = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e => {
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut b: bool;
                    let mut outEqs: metamodelica::Ref<BackendDAE::EqSystem> = outEqs.clone();
                    let (__pa0, (_, __pa1, __pa2, _)) = Expression::traverseExpBottomUp(e.clone(), &fnptr!(inlineCallsWork, metamodelica::Ref<DAE::Exp>, ((Option<metamodelica::Ref<AvlTreePathFunction::Tree>>, metamodelica::List<DAE::InlineType>), metamodelica::Ref<BackendDAE::EqSystem>, bool, bool)), (fns.clone(), inEqs.clone(), false, false))?;
                    e1 = metamodelica::Own::own(__pa0);
                    outEqs = metamodelica::Own::own(__pa1);
                    b = metamodelica::Own::own(__pa2);
                    source = inSource.clone();
                    e2 = e1.clone();
                    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? {
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\ninExp: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); ArcStr::from(__mm_s) });
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\noutExp: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(e2.clone())?); ArcStr::from(__mm_s) });
                    }
                    Ok(((e2.clone(), source.clone(), outEqs.clone(), b), outEqs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outEqs = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inSource.clone(), inEqs.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outSource, outEqs, inlined, shared)
}

fn inlineCallsWork(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: (
        (
            Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
            metamodelica::List<DAE::InlineType>,
        ),
        metamodelica::Ref<BackendDAE::EqSystem>,
        bool,
        bool,
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (
        (
            Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
            metamodelica::List<DAE::InlineType>,
        ),
        metamodelica::Ref<BackendDAE::EqSystem>,
        bool,
        bool,
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTuple: (
        (
            Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
            metamodelica::List<DAE::InlineType>,
        ),
        metamodelica::Ref<BackendDAE::EqSystem>,
        bool,
        bool,
    );
    (outExp, outTuple) = 'mc: {
        let __mc_input = (&*inExp, &inTuple);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { .. }, _) => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ DAE::Exp::CALL { path: p, expLst: args, attr: Deref @ DAE::CallAttributes { inlineType, .. } }, (fns, eqSys, _, false)) => {
                            if !((Inline::checkInlineType(inlineType.clone(), &(fns.clone())) && Flags::getConfigEnum(Flags::INLINE_METHOD.clone())? == 2)) { return Err("guard") }
                            let mut r#fn: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                            let mut outputCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut newExp: metamodelica::Ref<DAE::Exp>;
                            let mut comment: Option<metamodelica::Ref<SCode::Comment>>;
                            let mut funcname: ArcStr;
                            let mut newEqSys: metamodelica::Ref<BackendDAE::EqSystem>;
                            (r#fn, comment) = Inline::getFunctionBody(p.clone(), &(fns.clone()))?;
                            funcname = BackendUtil::modelicaStringToCStr(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?, false)?;
                            if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? {
                                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Inline Function ")); __mm_s.push_str(&*funcname); __mm_s.push_str(&*literal!(" type: ")); __mm_s.push_str(&*DAEDump::dumpInlineTypeStr(inlineType.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("in : ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                            }
                            (outputCrefs, newEqSys) = createEqnSysfromFunction(&r#fn, args.clone(), &funcname)?;
                            newExp = Expression::makeTuple(({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut cr in (outputCrefs.clone()).into_iter().cloned() {
                            let __x = Expression::crefExp(cr.clone())?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }))?;
                            if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? {
                                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("out: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(newExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                            }
                            if !(Inline::hasGenerateEventsAnnotation(comment.clone())) {
                                BackendDAEUtil::traverseBackendDAEExpsEqSystemWithUpdate(&newEqSys, (std::sync::Arc::new(addNoEvent) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, bool) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> + 'static>), false)?;
                            }
                            newEqSys = BackendDAEUtil::mergeEqSystems(&newEqSys, eqSys.clone())?;
                            Ok((newExp.clone(), (fns.clone(), newEqSys.clone(), true, false)))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: p, expLst: _, attr: Deref @ DAE::CallAttributes { inlineType, .. } }, (fns, eqSys, b, insideIfExp)) => {
                    let mut newExp: metamodelica::Ref<DAE::Exp>;
                    let mut funcname: ArcStr;
                    (newExp, _) = Inline::inlineCall(inExp.clone(), metamodelica::nil(), &(fns.clone()));
                    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? {
                        funcname = BackendUtil::modelicaStringToCStr(AbsynUtil::pathString(p.clone(), literal!("."), true, false)?, false)?;
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nBackendInline fallback replace implementation: ")); __mm_s.push_str(&*funcname); __mm_s.push_str(&*literal!(" type: ")); __mm_s.push_str(&*DAEDump::dumpInlineTypeStr(inlineType.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("in : ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                        metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("out: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(newExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    }
                    Ok((newExp.clone(), (fns.clone(), eqSys.clone(), b.clone(), insideIfExp.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTuple)
}

fn addNoEvent(mut inExp: metamodelica::Ref<DAE::Exp>, mut inB: bool) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outB: bool = inB;
    outExp = Expression::addNoEventToRelationsAndConds(inExp)?;
    outExp = Expression::addNoEventToEventTriggeringFunctions(outExp)?;
    Ok((outExp, outB))
}

fn createReplacementVariables(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut funcName: &ArcStr,
    mut inRepls: BackendVarTransform::VariableReplacements,
) -> Result<(
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    BackendVarTransform::VariableReplacements,
)> {
    let mut crVar: metamodelica::Ref<DAE::ComponentRef>;
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut outRepls: BackendVarTransform::VariableReplacements = inRepls;
    let mut eVar: metamodelica::Ref<DAE::Exp>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut arrExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut crefs1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    var = BackendVariable::createTmpVar(&inCref, funcName)?;
    crVar = BackendVariable::varCref(&var);
    eVar = Expression::crefExp(crVar.clone())?;
    let false = (Expression::isRecord(&eVar)) else {
        return Err("pattern mismatch");
    };
    outRepls = BackendVarTransform::addReplacement(outRepls, inCref.clone(), eVar.clone(), None)?;
    crefs = ComponentReference::expandCref(&inCref, false)?;
    crefs1 = ComponentReference::expandCref(&crVar, false)?;
    match '__try0: {
        arrExp = unwrap_break_err!(Expression::getArrayOrRangeContents(eVar.clone()), '__try0);
        Ok::<_, &'static str>((arrExp.clone(),))
    } {
        Ok((__try0_o0,)) => {
            arrExp = __try0_o0;
        }
        Err(_) => {
            arrExp = list![eVar.clone()];
        }
    }
    if ((crefs).len() as i32) != ((arrExp).len() as i32) {
        if Flags::isSet(Flags::FAILTRACE.clone())? {
            Debug::traceln({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "BackendInline.createReplacementVariables failed with array handling "
                ));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(eVar)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            })?;
        }
        return Err("fail");
    }
    for mut c in &*crefs {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(crefs1) {
            Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cr = metamodelica::Own::own(__pa1);
        crefs1 = metamodelica::Own::own(__pa2);
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(arrExp) {
            Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa3);
        arrExp = metamodelica::Own::own(__pa4);
        assign_field!(var.varName = cr);
        outVars = metamodelica::cons(var.clone(), outVars);
        outRepls = BackendVarTransform::addReplacement(outRepls, c.clone(), e, None)?;
    }
    outVars = outVars.reverse();
    Ok((crVar, outVars, outRepls))
}

fn createEqnSysfromFunction(
    mut fns: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inArgs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut funcname: &ArcStr,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    metamodelica::Ref<BackendDAE::EqSystem>,
)> {
    let mut oOutput: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut outEqs: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut args: metamodelica::List<metamodelica::Ref<DAE::Exp>> = inArgs;
    let mut repl: BackendVarTransform::VariableReplacements;
    let mut fnInputs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut argmap: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
    let mut checkcr: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCG::FuncHashCref,
            HashTableCG::FuncCrefEqual,
            HashTableCG::FuncCrefStr,
            HashTableCG::FuncExpStr,
        ),
    );
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut eqlst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\ncreate EqnSys from function: "));
            __mm_s.push_str(&*funcname);
            ArcStr::from(__mm_s)
        });
    }
    outEqs = BackendDAEUtil::createEqSystem(
        BackendVariable::listVar(metamodelica::nil())?,
        BackendEquation::listEquation(&(metamodelica::nil()))?,
        metamodelica::nil(),
        openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
        BackendEquation::emptyEqns(),
    );
    repl = BackendVarTransform::emptyReplacements();
    for mut r#fn in &**fns {
        let () = (::match_deref::match_deref! { match &(r#fn.clone()) {
            Deref @ DAE::Element::VAR { componentRef: __esc_cr, direction: DAE::VarDirection::INPUT { .. }, kind: DAE::VarKind::VARIABLE { .. }, .. } => {
                cr = (*__esc_cr).clone();
                fnInputs = metamodelica::cons(cr.clone(), fnInputs);
                ()
            },
            Deref @ DAE::Element::VAR { componentRef: cr, direction: DAE::VarDirection::OUTPUT { .. }, kind: DAE::VarKind::VARIABLE { .. }, .. } if (!(Expression::isRecordType(&(ComponentReference::crefTypeFull(metamodelica::AsArg::as_arg(&cr))?))) && ComponentReference::crefDepth(metamodelica::AsArg::as_arg(&cr))? > 0) => {
                let mut crVar: metamodelica::Ref<DAE::ComponentRef>;
                let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                (crVar, varLst, repl) = createReplacementVariables(cr.clone(), funcname, repl)?;
                outEqs = BackendVariable::addVarsDAE(&varLst, outEqs)?;
                oOutput = metamodelica::cons(crVar, oOutput);
                ()
            },
            Deref @ DAE::Element::VAR { componentRef: cr, protection: DAE::VarVisibility::PROTECTED { .. }, binding: None, .. } if (!(Expression::isRecordType(&(ComponentReference::crefTypeFull(metamodelica::AsArg::as_arg(&cr))?)))) => {
                let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                (_, varLst, repl) = createReplacementVariables(cr.clone(), funcname, repl)?;
                varLst = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
            for mut _var in (varLst).into_iter().cloned() {
                let __x = BackendVariable::setVarTS(_var.clone(), Some(openmodelica_backend_types::BackendDAE::TearingSelect::AVOID));
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                outEqs = BackendVariable::addVarsDAE(&varLst, outEqs)?;
                ()
            },
            Deref @ DAE::Element::VAR { componentRef: cr, protection: DAE::VarVisibility::PROTECTED { .. }, binding: Some(eBind), .. } if (!(Expression::isRecordType(&(ComponentReference::crefTypeFull(metamodelica::AsArg::as_arg(&cr))?)))) => {
                let mut crVar: metamodelica::Ref<DAE::ComponentRef>;
                let mut eVar: metamodelica::Ref<DAE::Exp>;
                let mut eq: metamodelica::Ref<BackendDAE::Equation>;
                let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                (crVar, varLst, repl) = createReplacementVariables(cr.clone(), funcname, repl)?;
                eVar = Expression::crefExp(crVar)?;
                varLst = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
            for mut _var in (varLst).into_iter().cloned() {
                let __x = BackendVariable::setVarTS(_var.clone(), Some(openmodelica_backend_types::BackendDAE::TearingSelect::AVOID));
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                outEqs = BackendVariable::addVarsDAE(&varLst, outEqs)?;
                eq = BackendEquation::generateEquation(eVar, eBind.clone(), DAE::emptyElementSource().clone(), BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone())?;
                outEqs = BackendEquation::equationAddDAE(eq, outEqs)?;
                ()
            },
            Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: st }, .. } => {
                eqlst = List::map(st.clone(), &move |__a0: metamodelica::Ref<DAE::Statement>| BackendEquation::statementEq(&__a0))?;
                outEqs = BackendEquation::equationsAddDAE(&eqlst, outEqs)?;
                ()
            },
            _ => return Err("match: no arm matched"),
        } });
    }
    oOutput = oOutput.reverse();
    if BackendDAEUtil::systemSize(&outEqs)? != BackendVariable::daenumVariables(&outEqs) {
        if Flags::isSet(Flags::FAILTRACE.clone())? {
            Debug::trace({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(
                    "newBackendInline.createEqnSysfromFunction failed for function "
                ));
                __mm_s.push_str(&*funcname);
                __mm_s.push_str(&*literal!("with different sizes\n"));
                ArcStr::from(__mm_s)
            })?;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*intString(BackendDAEUtil::systemSize(&outEqs)?));
                __mm_s.push_str(&*literal!(" <> "));
                __mm_s.push_str(&*intString(BackendVariable::daenumVariables(&outEqs)));
                ArcStr::from(__mm_s)
            });
        }
        return Err("fail");
    }
    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\noriginal function body of: "));
            __mm_s.push_str(&*funcname);
            ArcStr::from(__mm_s)
        });
        BackendDump::printEqSystem(outEqs.clone())?;
        metamodelica::print(literal!("\nDump replacements: "));
        BackendVarTransform::dumpReplacements(&repl)?;
    }
    assign_field!(
        outEqs.orderedEqs = BackendEquation::listEquation(
            &((InlineArrayEquations::getScalarArrayEqns(&(BackendEquation::equationList(outEqs.orderedEqs.clone())?)))
                .0)
        )?
    );
    outEqs = BackendVarTransform::performReplacementsEqSystem(outEqs, repl)?;
    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\n replaced protected and output for: "));
            __mm_s.push_str(&*funcname);
            ArcStr::from(__mm_s)
        });
        BackendDump::printEqSystem(outEqs.clone())?;
    }
    argmap = List::zip(fnInputs.reverse(), args);
    (argmap, checkcr) = Inline::extendCrefRecords(&argmap, HashTableCG::emptyHashTable())?;
    BackendDAEUtil::traverseBackendDAEExpsEqSystemWithUpdate(
        &outEqs,
        (std::sync::Arc::new(replaceArgs)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::Exp>,
                        (
                            metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<
                                        Option<(
                                            metamodelica::Ref<DAE::ComponentRef>,
                                            metamodelica::Ref<DAE::ComponentRef>,
                                        )>,
                                    >,
                                ),
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::ComponentRef>,
                                                metamodelica::Ref<DAE::ComponentRef>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                ),
                            ),
                            bool,
                        ),
                    ) -> Result<(
                        metamodelica::Ref<DAE::Exp>,
                        (
                            metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<
                                        Option<(
                                            metamodelica::Ref<DAE::ComponentRef>,
                                            metamodelica::Ref<DAE::ComponentRef>,
                                        )>,
                                    >,
                                ),
                                i32,
                                (
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::ComponentRef>,
                                                metamodelica::Ref<DAE::ComponentRef>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                    Arc<
                                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                            + 'static,
                                    >,
                                ),
                            ),
                            bool,
                        ),
                    )> + 'static,
            >),
        (argmap, checkcr, true),
    )?;
    if Flags::isSet(Flags::DUMPBACKENDINLINE_VERBOSE.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nreplaced input arguments for: "));
            __mm_s.push_str(&*funcname);
            ArcStr::from(__mm_s)
        });
        BackendDump::printEqSystem(outEqs.clone())?;
    }
    Ok((oOutput, outEqs))
}

fn addReplacement(
    mut iCr: metamodelica::Ref<DAE::ComponentRef>,
    mut iExp: metamodelica::Ref<DAE::Exp>,
    mut iRepl: BackendVarTransform::VariableReplacements,
) -> Result<BackendVarTransform::VariableReplacements> {
    let mut oRepl: BackendVarTransform::VariableReplacements;
    oRepl = (match &*iCr {
        DAE::ComponentRef::CREF_IDENT { identType: tp, .. }
            if (!(Expression::isRecordType(metamodelica::AsArg::as_arg(&tp)))
                && !(Expression::isArrayType(metamodelica::AsArg::as_arg(&tp)))) =>
        {
            BackendVarTransform::addReplacement(iRepl, iCr, iExp, None)?
        }
        DAE::ComponentRef::CREF_IDENT { identType: tp, .. }
            if (Expression::isArrayType(metamodelica::AsArg::as_arg(&tp))) =>
        {
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut arrExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            crefs = ComponentReference::expandCref(&iCr, false)?;
            repl = iRepl;
            arrExp = Expression::getArrayOrRangeContents(iExp)?;
            for mut c in &*crefs {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(arrExp) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                e = metamodelica::Own::own(__pa0);
                arrExp = metamodelica::Own::own(__pa1);
                repl = BackendVarTransform::addReplacement(repl, c.clone(), e, None)?;
            }
            repl
        }
        _ => return Err("fail"),
    });
    Ok(oRepl)
}

fn replaceArgs(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: (
        metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<
                    Option<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    )>,
                >,
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
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            ),
        ),
        bool,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<
                    Option<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    )>,
                >,
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
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            ),
        ),
        bool,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTuple: (
        metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<
                    Option<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    )>,
                >,
            ),
            i32,
            (
                HashTableCG::FuncHashCref,
                HashTableCG::FuncCrefEqual,
                HashTableCG::FuncCrefStr,
                HashTableCG::FuncExpStr,
            ),
        ),
        bool,
    );
    (outExp, outTuple) = Expression::traverseExpBottomUp(
        inExp,
        &fnptr!(
            Inline::replaceArgs,
            metamodelica::Ref<DAE::Exp>,
            (
                metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>,
                (
                    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                    (
                        i32,
                        i32,
                        metamodelica::Array<
                            Option<(
                                metamodelica::Ref<DAE::ComponentRef>,
                                metamodelica::Ref<DAE::ComponentRef>
                            )>,
                        >
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
                        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>
                    )
                ),
                bool
            )
        ),
        inTuple,
    )?;
    if !(Util::tuple33(outTuple.clone())) {
        if Flags::isSet(Flags::FAILTRACE.clone())? {
            Debug::traceln(literal!("BackendInline.replaceArgs failed"))?;
        }
        return Err("fail");
    }
    Ok((outExp, outTuple))
}
