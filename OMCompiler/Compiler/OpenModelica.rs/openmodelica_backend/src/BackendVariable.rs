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

use crate::BackendDAEUtil;
use crate::CommonSubExpression;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend::HashSet;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::BaseHashSet;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

/* =======================================================
 *
 *  Section for functions that deals with Var
 *
 * =======================================================
 */
pub(crate) fn varEqual(
    mut inVar1: &metamodelica::Ref<BackendDAE::Var>,
    mut inVar2: &metamodelica::Ref<BackendDAE::Var>,
) -> Result<bool> {
    let mut outBoolean: bool = ComponentReferenceBasics::crefEqualNoStringCompare(&inVar1.varName, &inVar2.varName)?;
    Ok(outBoolean)
}

pub(crate) fn setVarFixed(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inBoolean: bool,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
    let mut oattr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    oattr = if ((inVar.values).is_some()) {
        inVar.values.clone()
    } else {
        Some(getVariableAttributefromType(&inVar.varType)?)
    };
    assign_field!(
        outVar.values = DAEUtil::setFixedAttr(
            oattr,
            Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: inBoolean }))
        )?
    );
    Ok(outVar)
}

pub(crate) fn removeFixedAttribute(
    mut var: metamodelica::Ref<BackendDAE::Var>,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut var: metamodelica::Ref<BackendDAE::Var> = var;
    if (var.values).is_some() {
        assign_field!(var.values = DAEUtil::setFixedAttr(var.values.clone(), None)?);
    }
    Ok(var)
}

pub(crate) fn removeStartAttribute(
    mut var: metamodelica::Ref<BackendDAE::Var>,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut var: metamodelica::Ref<BackendDAE::Var> = var;
    if (var.values).is_some() {
        assign_field!(var.values = DAEUtil::setStartAttrOption(var.values.clone(), None)?);
    }
    Ok(var)
}

pub(crate) fn varFixed(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { fixed: Some(Deref @ DAE::Exp::BCONST { bool: fixed }), .. }), .. } => {
            fixed.clone()
        },
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { fixed: Some(Deref @ DAE::Exp::BCONST { bool: fixed }), .. }), .. } => {
            fixed.clone()
        },
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { fixed: Some(Deref @ DAE::Exp::BCONST { bool: fixed }), .. }), .. } => {
            fixed.clone()
        },
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { fixed: Some(Deref @ DAE::Exp::BCONST { bool: fixed }), .. }), .. } => {
            fixed.clone()
        },
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { fixed: Some(Deref @ DAE::Exp::BCONST { bool: fixed }), .. }), .. } => {
            fixed.clone()
        },
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM { .. }, .. } => {
            true
        },
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::CONST { .. }, bindExp: Some(_), .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn setVarStartValue(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
    let mut oattr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    oattr = if ((inVar.values).is_some()) {
        inVar.values.clone()
    } else {
        Some(getVariableAttributefromType(&inVar.varType)?)
    };
    assign_field!(outVar.values = DAEUtil::setStartAttr(oattr, inExp)?);
    Ok(outVar)
}

pub(crate) fn setVarStartValueOption(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inExp: Option<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
    let mut oattr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    oattr = if ((inVar.values).is_some()) {
        inVar.values.clone()
    } else {
        Some(getVariableAttributefromType(&inVar.varType)?)
    };
    assign_field!(outVar.values = DAEUtil::setStartAttrOption(oattr, inExp)?);
    Ok(outVar)
}

pub(crate) fn setVarStartOrigin(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut startOrigin: Option<DAE::StartOrigin>,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
    let mut oattr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    oattr = if ((inVar.values).is_some()) {
        inVar.values.clone()
    } else {
        Some(getVariableAttributefromType(&inVar.varType)?)
    };
    assign_field!(outVar.values = DAEUtil::setStartOrigin(oattr, startOrigin)?);
    Ok(outVar)
}

pub(crate) fn setVarAttributes(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inAttr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
) -> metamodelica::Ref<BackendDAE::Var> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar;
    assign_field!(outVar.values = inAttr);
    outVar
}

pub(crate) fn varStartValue(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut sv: metamodelica::Ref<DAE::Exp>;
    sv = DAEUtil::getStartAttr(inVar.values.clone(), &inVar.varType)?;
    Ok(sv)
}

pub(crate) fn varUnreplaceable(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outUnreplaceable: bool = inVar.unreplaceable.clone();
    outUnreplaceable
}

pub fn setVarUnreplaceable(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut value: bool,
) -> metamodelica::Ref<BackendDAE::Var> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar;
    assign_field!(outVar.unreplaceable = value);
    outVar
}

pub(crate) fn setVarInitNonlinear(
    mut var: metamodelica::Ref<BackendDAE::Var>,
    mut value: bool,
) -> metamodelica::Ref<BackendDAE::Var> {
    let mut var: metamodelica::Ref<BackendDAE::Var> = var;
    assign_field!(var.initNonlinear = value);
    var
}

pub(crate) fn varStartValueFail(mut v: &metamodelica::Ref<BackendDAE::Var>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut sv: metamodelica::Ref<DAE::Exp>;
    let mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let __arc1 = &(*v);
    let BackendDAE::VAR { values: __pa0, .. } = &**__arc1;
    attr = metamodelica::Own::own(__pa0);
    sv = DAEUtil::getStartAttrFail(attr)?;
    Ok(sv)
}

pub(crate) fn varNominalValueFail(mut v: &metamodelica::Ref<BackendDAE::Var>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut sv: metamodelica::Ref<DAE::Exp>;
    let mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let __arc1 = &(*v);
    let BackendDAE::VAR { values: __pa0, .. } = &**__arc1;
    attr = metamodelica::Own::own(__pa0);
    sv = DAEUtil::getNominalAttrFail(attr)?;
    Ok(sv)
}

pub(crate) fn varMinValueFail(mut v: &metamodelica::Ref<BackendDAE::Var>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut sv: metamodelica::Ref<DAE::Exp>;
    let mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let __arc1 = &(*v);
    let BackendDAE::VAR { values: __pa0, .. } = &**__arc1;
    attr = metamodelica::Own::own(__pa0);
    sv = DAEUtil::getMinAttrFail(attr)?;
    Ok(sv)
}

pub(crate) fn varMaxValueFail(mut v: &metamodelica::Ref<BackendDAE::Var>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut sv: metamodelica::Ref<DAE::Exp>;
    let mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let __arc1 = &(*v);
    let BackendDAE::VAR { values: __pa0, .. } = &**__arc1;
    attr = metamodelica::Own::own(__pa0);
    sv = DAEUtil::getMaxAttrFail(attr)?;
    Ok(sv)
}

pub(crate) fn varStartValueType(mut v: &metamodelica::Ref<BackendDAE::Var>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut sv: metamodelica::Ref<DAE::Exp>;
    let mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let __arc2 = &(*v);
    let BackendDAE::VAR {
        values: __pa0,
        varType: __pa1,
        ..
    } = &**__arc2;
    attr = metamodelica::Own::own(__pa0);
    ty = metamodelica::Own::own(__pa1);
    sv = DAEUtil::getStartAttr(attr, &ty)?;
    Ok(sv)
}

pub(crate) fn varStartValueOption(mut v: &metamodelica::Ref<BackendDAE::Var>) -> Option<metamodelica::Ref<DAE::Exp>> {
    let mut sv: Option<metamodelica::Ref<DAE::Exp>>;
    sv = 'mc: {
        let __mc_input = &**v;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Var { values: attr, .. } => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    exp = DAEUtil::getStartAttrFail(attr.clone())?;
                    Ok(Some(exp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(None)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    sv
}

pub(crate) fn varHasStartValue(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outHasStartValue: bool;
    let mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let __arc1 = &(*inVar);
    let BackendDAE::VAR { values: __pa0, .. } = &**__arc1;
    attr = metamodelica::Own::own(__pa0);
    outHasStartValue = DAEUtil::hasStartAttr(attr);
    outHasStartValue
}

pub(crate) fn varHasNoStartValue(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outHasNoStartValue: bool;
    outHasNoStartValue = !(varHasStartValue(inVar));
    outHasNoStartValue
}

pub(crate) fn varStartOrigin(mut v: &metamodelica::Ref<BackendDAE::Var>) -> Result<Option<DAE::StartOrigin>> {
    let mut so: Option<DAE::StartOrigin>;
    let mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let __arc1 = &(*v);
    let BackendDAE::VAR { values: __pa0, .. } = &**__arc1;
    attr = metamodelica::Own::own(__pa0);
    so = DAEUtil::getStartOrigin(attr)?;
    Ok(so)
}

pub(crate) fn varBindExp(mut v: &metamodelica::Ref<BackendDAE::Var>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut sv: metamodelica::Ref<DAE::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &((*v)) {
        Deref @ BackendDAE::Var { bindExp: Some(__pa0), .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    sv = metamodelica::Own::own(__pa0);
    Ok(sv)
}

pub(crate) fn varHasConstantBindExp(mut v: &metamodelica::Ref<BackendDAE::Var>) -> Result<bool> {
    let mut out: bool;
    out = (::match_deref::match_deref! { match v {
        Deref @ BackendDAE::Var { bindExp: Some(e), .. } => {
            Expression::isConst(e.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

pub(crate) fn varHasNonConstantBindExpOrStartValue(mut v: &metamodelica::Ref<BackendDAE::Var>) -> Result<bool> {
    let mut out: bool;
    out = (::match_deref::match_deref! { match v {
        Deref @ BackendDAE::Var { bindExp: Some(e), .. } => {
            !(Expression::isConstValue(metamodelica::AsArg::as_arg(&e))?)
        },
        _ => {
            !(varHasConstantStartExp(v))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

pub(crate) fn varHasConstantStartExp(mut v: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut out: bool;
    let mut e: metamodelica::Ref<DAE::Exp>;
    match '__try0: {
        e = unwrap_break_err!(varStartValueFail(v), '__try0);
        out = unwrap_break_err!(Expression::isConstValue(&e), '__try0);
        Ok::<_, &'static str>((out.clone(),))
    } {
        Ok((__try0_o0,)) => {
            out = __try0_o0;
        }
        Err(_) => {
            out = true;
        }
    }
    out
}

pub(crate) fn varHasBindExp(mut v: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut out: bool;
    out = (::match_deref::match_deref! { match v {
        Deref @ BackendDAE::Var { bindExp: Some(_), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out
}

pub(crate) fn varBindExpStartValue(mut v: &metamodelica::Ref<BackendDAE::Var>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut sv: metamodelica::Ref<DAE::Exp>;
    sv = (::match_deref::match_deref! { match v {
        Deref @ BackendDAE::Var { bindExp: Some(e), .. } => {
            e.clone()
        },
        _ => {
            varStartValueFail(v)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(sv)
}

pub(crate) fn varBindExpStartValueNoFail(
    mut v: &metamodelica::Ref<BackendDAE::Var>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut sv: metamodelica::Ref<DAE::Exp>;
    sv = (::match_deref::match_deref! { match v {
        Deref @ BackendDAE::Var { bindExp: Some(e), .. } => {
            e.clone()
        },
        _ => {
            varStartValue(v)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(sv)
}

pub(crate) fn varStateSelect(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> DAE::StateSelect {
    let mut outStateSelect: DAE::StateSelect;
    outStateSelect = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(stateselect), .. }), .. } => {
            stateselect.clone()
        },
        _ => {
            openmodelica_frontend_types::DAE::StateSelect::DEFAULT
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outStateSelect
}

pub(crate) fn varHasStateSelect(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(_), .. }), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn varStateSelectAlways(mut v: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match v {
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(DAE::StateSelect::ALWAYS { .. }), .. }), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn notVarStateSelectAlways(mut v: &metamodelica::Ref<BackendDAE::Var>, mut level: i32) -> bool {
    let mut b: bool;
    b = (match &**v {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::STATE { index: diffcount, .. },
            ..
        } => !(varStateSelectAlways(v) && (diffcount.clone() == level || diffcount.clone() == 1)),
        _ => true,
    });
    b
}

pub(crate) fn varStateSelectNever(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut isNever: bool;
    isNever = (match varStateSelect(inVar) {
        DAE::StateSelect::NEVER { .. } => true,
        _ => false,
    });
    isNever
}

pub(crate) fn varStateSelectAvoid(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut isAvoid: bool;
    isAvoid = (match varStateSelect(inVar) {
        DAE::StateSelect::AVOID { .. } => true,
        _ => false,
    });
    isAvoid
}

pub(crate) fn varStateSelectPrefer(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut isPrefer: bool;
    isPrefer = (match varStateSelect(inVar) {
        DAE::StateSelect::PREFER { .. } => true,
        _ => false,
    });
    isPrefer
}

pub(crate) fn setVarStateSelect(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut stateSelect: DAE::StateSelect,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
    let mut oattr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    oattr = if ((inVar.values).is_some()) {
        inVar.values.clone()
    } else {
        Some(getVariableAttributefromType(&inVar.varType)?)
    };
    assign_field!(outVar.values = DAEUtil::setStateSelect(oattr, stateSelect)?);
    Ok(outVar)
}

pub(crate) fn varStateSelectForced(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut isForced: bool;
    isForced = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(DAE::StateSelect::ALWAYS { .. }), .. }), .. } => true,
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(DAE::StateSelect::PREFER { .. }), .. }), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isForced
}

pub(crate) fn isNaturalState(mut var: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut natural: bool;
    natural = (match &**var {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::STATE { natural: n, .. },
            ..
        } => n.clone(),
        _ => false,
    });
    natural
}

pub(crate) fn isArtificialState(mut var: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut artificial: bool;
    artificial = (match &**var {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::STATE { natural: n, .. },
            ..
        } => !(n.clone()),
        _ => false,
    });
    artificial
}

pub(crate) fn varStateDerivative(
    mut inVar: &metamodelica::Ref<BackendDAE::Var>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut dcr: metamodelica::Ref<DAE::ComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match &((*inVar)) {
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: Some(__pa0), .. }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    dcr = metamodelica::Own::own(__pa0);
    Ok(dcr)
}

pub(crate) fn varHasStateDerivative(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { derName: Some(_), .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn setStateDerivative(
    mut var: metamodelica::Ref<BackendDAE::Var>,
    mut derName: Option<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut var: metamodelica::Ref<BackendDAE::Var> = var;
    let mut index: i32;
    let mut natural: bool;
    let BackendDAE::STATE {
        index: __pa0,
        natural: __pa1,
        ..
    } = (var.varKind.clone())
    else {
        return Err("pattern mismatch");
    };
    index = metamodelica::Own::own(__pa0);
    natural = metamodelica::Own::own(__pa1);
    assign_field!(
        var.varKind = BackendDAE::VarKind::STATE {
            index: index,
            derName: derName,
            natural: natural
        }
    );
    Ok(var)
}

pub(crate) fn getVariableAttributefromType(
    mut inType: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::VariableAttributes>> {
    let mut attr: metamodelica::Ref<DAE::VariableAttributes>;
    attr = (match &**inType {
        DAE::Type::T_REAL { .. } => metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL {
            quantity: None,
            unit: None,
            displayUnit: None,
            min: None,
            max: None,
            start: None,
            fixed: None,
            nominal: None,
            stateSelectOption: None,
            uncertainOption: None,
            distributionOption: None,
            equationBound: None,
            isProtected: None,
            finalPrefix: None,
            startOrigin: None,
        }),
        DAE::Type::T_INTEGER { .. } => metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_INT {
            quantity: None,
            min: None,
            max: None,
            start: None,
            fixed: None,
            uncertainOption: None,
            distributionOption: None,
            equationBound: None,
            isProtected: None,
            finalPrefix: None,
            startOrigin: None,
        }),
        DAE::Type::T_BOOL { .. } => metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_BOOL {
            quantity: None,
            start: None,
            fixed: None,
            equationBound: None,
            isProtected: None,
            finalPrefix: None,
            startOrigin: None,
        }),
        DAE::Type::T_STRING { .. } => metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_STRING {
            quantity: None,
            start: None,
            fixed: None,
            equationBound: None,
            isProtected: None,
            finalPrefix: None,
            startOrigin: None,
        }),
        DAE::Type::T_ENUMERATION { .. } => metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION {
            quantity: None,
            min: None,
            max: None,
            start: None,
            fixed: None,
            equationBound: None,
            isProtected: None,
            finalPrefix: None,
            startOrigin: None,
        }),
        _ => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::trace(literal!("getVariableAttributefromType called with unsopported Type!\n"))?;
            }
            metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL {
                quantity: None,
                unit: None,
                displayUnit: None,
                min: None,
                max: None,
                start: None,
                fixed: None,
                nominal: None,
                stateSelectOption: None,
                uncertainOption: None,
                distributionOption: None,
                equationBound: None,
                isProtected: None,
                finalPrefix: None,
                startOrigin: None,
            })
        }
    });
    Ok(attr)
}

pub(crate) fn setVarFinal(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut finalPrefix: bool,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
    let mut oattr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    oattr = if ((inVar.values).is_some()) {
        inVar.values.clone()
    } else {
        Some(getVariableAttributefromType(&inVar.varType)?)
    };
    assign_field!(outVar.values = DAEUtil::setFinalAttr(oattr, finalPrefix)?);
    Ok(outVar)
}

pub(crate) fn setVarMinMax(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inMin: Option<metamodelica::Ref<DAE::Exp>>,
    mut inMax: Option<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
    let mut oattr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    if (inMin).is_some() || (inMax).is_some() {
        oattr = if ((inVar.values).is_some()) {
            inVar.values.clone()
        } else {
            Some(getVariableAttributefromType(&inVar.varType)?)
        };
        assign_field!(outVar.values = DAEUtil::setMinMax(oattr, inMin, inMax)?);
    }
    Ok(outVar)
}

pub(crate) fn varNominalValue(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &((*inVar)) {
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { nominal: Some(__pa0), .. }), .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outExp = metamodelica::Own::own(__pa0);
    Ok(outExp)
}

pub(crate) fn setVarNominalValue(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
    let mut oattr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    oattr = if ((inVar.values).is_some()) {
        inVar.values.clone()
    } else {
        Some(getVariableAttributefromType(&inVar.varType)?)
    };
    assign_field!(outVar.values = DAEUtil::setNominalAttr(oattr, inExp)?);
    Ok(outVar)
}

pub(crate) fn varType(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    let __arc1 = &(*inVar);
    let BackendDAE::VAR { varType: __pa0, .. } = &**__arc1;
    outType = metamodelica::Own::own(__pa0);
    outType
}

pub(crate) fn varKind(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> BackendDAE::VarKind {
    let mut outVarKind: BackendDAE::VarKind;
    let __arc1 = &(*inVar);
    let BackendDAE::VAR { varKind: __pa0, .. } = &**__arc1;
    outVarKind = metamodelica::Own::own(__pa0);
    outVarKind
}

pub(crate) fn varNominal(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> Result<metamodelica::Real> {
    let mut outReal: metamodelica::Real;
    let __pa0 = ::match_deref::match_deref! { match &((*inVar)) {
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { nominal: Some(Deref @ DAE::Exp::RCONST { real: __pa0 }), .. }), .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outReal = metamodelica::Own::own(__pa0);
    Ok(outReal)
}

pub(crate) fn varHasNominalValue(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBool: bool;
    match '__try0: {
        ::match_deref::match_deref! { match &((*inVar)) {
            Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { nominal: Some(Deref @ DAE::Exp::RCONST { .. }), .. }), .. } => (),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        outBool = true;
        Ok::<_, &'static str>((outBool.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outBool = __try0_o0;
        }
        Err(_) => {
            outBool = false;
        }
    }
    outBool
}

pub fn varCref(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    let __arc1 = &(*inVar);
    let BackendDAE::VAR { varName: __pa0, .. } = &**__arc1;
    outComponentRef = metamodelica::Own::own(__pa0);
    outComponentRef
}

pub fn isStateVar(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::STATE { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isState(mut inCref: metamodelica::Ref<DAE::ComponentRef>, mut inVars: &BackendDAE::Variables) -> bool {
    let mut outBool: bool;
    outBool = 'mc: {
        let __mc_input = inVars.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            ::match_deref::match_deref! { match &(getVar(inCref.clone(), inVars)?) {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, tail: _ }, _) => (),
                _ => return Err("pattern mismatch"),
            } };
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
    outBool
}

pub(crate) fn isNonStateVar(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::VARIABLE { .. },
            ..
        } => true,
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::DUMMY_DER { .. },
            ..
        } => true,
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::DUMMY_STATE { .. },
            ..
        } => true,
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::DISCRETE { .. },
            ..
        } => true,
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::STATE_DER { .. },
            ..
        } => true,
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::OPT_CONSTR { .. },
            ..
        } => true,
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::OPT_FCONSTR { .. },
            ..
        } => true,
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::OPT_INPUT_WITH_DER { .. },
            ..
        } => true,
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::OPT_INPUT_DER { .. },
            ..
        } => true,
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::OPT_TGRID { .. },
            ..
        } => true,
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::OPT_LOOP_INPUT { .. },
            ..
        } => true,
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::ALG_STATE { .. },
            ..
        } => true,
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::LOOP_ITERATION { .. },
            ..
        } => true,
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::LOOP_SOLVED { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isClockedStateVar(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBool: bool;
    outBool = (match &**inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::CLOCKED_STATE { .. },
            ..
        } => true,
        _ => false,
    });
    outBool
}

pub(crate) fn isClockedState(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inVars: &BackendDAE::Variables,
) -> bool {
    let mut outBool: bool;
    outBool = 'mc: {
        let __mc_input = inVars.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            ::match_deref::match_deref! { match &(getVar(inCref.clone(), inVars)?) {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::CLOCKED_STATE { .. }, .. }, tail: _ }, _) => (),
                _ => return Err("pattern mismatch"),
            } };
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
    outBool
}

pub fn varHasUncertainValueRefine(mut var: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match var {
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { uncertainOption: Some(DAE::Uncertainty::REFINE { .. }), .. }), .. } => true,
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { uncertainOption: Some(DAE::Uncertainty::REFINE { .. }), .. }), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn varHasUncertainValuePropagate(mut var: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match var {
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { uncertainOption: Some(DAE::Uncertainty::PROPAGATE { .. }), .. }), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn varDistribution(
    mut var: &metamodelica::Ref<BackendDAE::Var>,
) -> Result<metamodelica::Ref<DAE::Distribution>> {
    let mut d: metamodelica::Ref<DAE::Distribution>;
    d = (::match_deref::match_deref! { match var {
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { distributionOption: Some(__esc_d), .. }), .. } => {
            d = (*__esc_d).clone();
            d.clone()
        },
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { distributionOption: Some(__esc_d), .. }), .. } => {
            d = (*__esc_d).clone();
            d.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(d)
}

pub fn varTryGetDistribution(
    mut var: &metamodelica::Ref<BackendDAE::Var>,
) -> Option<metamodelica::Ref<DAE::Distribution>> {
    let mut dout: Option<metamodelica::Ref<DAE::Distribution>>;
    let mut d: Option<metamodelica::Ref<DAE::Distribution>>;
    dout = (::match_deref::match_deref! { match var {
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { distributionOption: __esc_d @ Some(_), .. }), .. } => {
            d = (*__esc_d).clone();
            d.clone()
        },
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { distributionOption: __esc_d @ Some(_), .. }), .. } => {
            d = (*__esc_d).clone();
            d.clone()
        },
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    dout
}

pub(crate) fn varUncertainty(mut var: &metamodelica::Ref<BackendDAE::Var>) -> Result<DAE::Uncertainty> {
    let mut u: DAE::Uncertainty;
    u = (::match_deref::match_deref! { match var {
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { uncertainOption: Some(__esc_u), .. }), .. } => {
            u = (*__esc_u).clone();
            u.clone()
        },
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { uncertainOption: Some(__esc_u), .. }), .. } => {
            u = (*__esc_u).clone();
            u.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(u)
}

pub(crate) fn varHasDistributionAttribute(mut var: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match var {
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { distributionOption: Some(_), .. }), .. } => true,
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { distributionOption: Some(_), .. }), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn varHasUncertaintyAttribute(mut var: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match var {
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { uncertainOption: Some(_), .. }), .. } => true,
        Deref @ BackendDAE::Var { values: Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { uncertainOption: Some(_), .. }), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn isDummyStateVar(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::DUMMY_STATE { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isDummyDerVar(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::DUMMY_DER { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isStateDerVar(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::STATE_DER { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isStateorStateDerVar(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::STATE { .. },
            ..
        } => true,
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::STATE_DER { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isVarDiscrete(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::DISCRETE { .. }, .. } => true,
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM { .. }, .. } => true,
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::CONST { .. }, .. } => true,
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_INTEGER { .. }, .. } => true,
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_BOOL { .. }, .. } => true,
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_ENUMERATION { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn isVarNonDifferentiable(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::DISCRETE { .. }, .. } => true,
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_INTEGER { .. }, .. } => true,
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_BOOL { .. }, .. } => true,
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_ENUMERATION { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn isVarClockedState(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    let mut test: ArcStr;
    outBoolean = (match &**inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::CLOCKED_STATE { .. },
            ..
        } => true,
        _ => false,
    });
    test = literal!("");
    outBoolean
}

pub(crate) fn isDiscrete(
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut vars: &BackendDAE::Variables,
) -> Result<bool> {
    let mut outBoolean: bool;
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    (v, _) = getVarSingle(cr, vars)?;
    outBoolean = isVarDiscrete(&v);
    Ok(outBoolean)
}

pub(crate) fn isVarNonDiscrete(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = !(isVarDiscrete(inVar));
    outBoolean
}

pub(crate) fn hasDiscreteVar(mut inBackendDAEVarLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>) -> bool {
    let mut outBoolean: bool = false;
    for mut v in &**inBackendDAEVarLst {
        outBoolean = isVarDiscrete(metamodelica::AsArg::as_arg(&v));
        if outBoolean {
            break;
        }
    }
    outBoolean
}

pub(crate) fn hasContinuousVar<'__b>(
    mut inBackendDAEVarLst: &'__b metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match inBackendDAEVarLst {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::VARIABLE { .. }, varType: Deref @ DAE::Type::T_REAL { .. }, .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::VARIABLE { .. }, varType: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_REAL { .. }, .. }, .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE_DER { .. }, .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_DER { .. }, .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::DUMMY_STATE { .. }, .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_CONSTR { .. }, .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_FCONSTR { .. }, .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_INPUT_WITH_DER { .. }, .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_INPUT_DER { .. }, .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_TGRID { .. }, .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::OPT_LOOP_INPUT { .. }, .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::ALG_STATE { .. }, .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: vs } => {
                { inBackendDAEVarLst = vs; continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Nil => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn isVarNonDiscreteAlg(mut var: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut result: bool;
    result = (::match_deref::match_deref! { match var {
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_REAL { .. }, .. } => isVarAlg(var) && !(isVarDiscreteRealAlg(var)) || isOptInputVar(var),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub(crate) fn isOptInputVar(mut var: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut b: bool;
    b = (match var.varKind.clone() {
        BackendDAE::VarKind::OPT_LOOP_INPUT { .. } => true,
        BackendDAE::VarKind::OPT_INPUT_WITH_DER { .. } => true,
        BackendDAE::VarKind::OPT_INPUT_DER { .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isVarDiscreteRealAlg(mut var: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut result: bool;
    result = (::match_deref::match_deref! { match var {
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::DISCRETE { .. }, varType: Deref @ DAE::Type::T_REAL { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub(crate) fn isVarAlg(mut var: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut result: bool;
    result = (match var.varKind.clone() {
        BackendDAE::VarKind::VARIABLE { .. } => true,
        BackendDAE::VarKind::DISCRETE { .. } => true,
        BackendDAE::VarKind::DUMMY_DER { .. } => true,
        BackendDAE::VarKind::DUMMY_STATE { .. } => true,
        BackendDAE::VarKind::CLOCKED_STATE { .. } => true,
        _ => false,
    });
    result
}

pub(crate) fn isVarConst(mut var: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut result: bool;
    result = (::match_deref::match_deref! { match var {
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_BOOL { .. }, .. } => false,
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_INTEGER { .. }, .. } => false,
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_ENUMERATION { .. }, .. } => false,
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_STRING { .. }, .. } => false,
        _ if (isConst(var)) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub(crate) fn isVarStringConst(mut var: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut result: bool;
    result = (::match_deref::match_deref! { match var {
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_STRING { .. }, .. } if (isConst(var)) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub(crate) fn isVarIntConst(mut var: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut result: bool;
    result = (::match_deref::match_deref! { match var {
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_INTEGER { .. }, .. } if (isConst(var)) => true,
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_ENUMERATION { .. }, .. } if (isConst(var)) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub(crate) fn isVarBoolConst(mut var: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut result: bool;
    result = (::match_deref::match_deref! { match var {
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_BOOL { .. }, .. } if (isConst(var)) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

/* TODO: Is this correct? */
pub(crate) fn isVarParam(mut var: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut result: bool;
    result = (::match_deref::match_deref! { match var {
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_BOOL { .. }, .. } => false,
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_INTEGER { .. }, .. } => false,
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_STRING { .. }, .. } => false,
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_ENUMERATION { .. }, .. } => false,
        _ if (isParam(var)) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub(crate) fn isVarStringParam(mut var: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut result: bool;
    result = (::match_deref::match_deref! { match var {
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_STRING { .. }, .. } if (isParam(var)) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub(crate) fn isVarIntParam(mut var: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut result: bool;
    result = (::match_deref::match_deref! { match var {
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_INTEGER { .. }, .. } if (isParam(var)) => true,
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_ENUMERATION { .. }, .. } if (isParam(var)) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub(crate) fn isVarBoolParam(mut var: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut result: bool;
    result = (::match_deref::match_deref! { match var {
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_BOOL { .. }, .. } if (isParam(var)) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub(crate) fn isVarConnector(mut var: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut result: bool;
    result = (::match_deref::match_deref! { match var {
        Deref @ BackendDAE::Var { connectorType: Deref @ DAE::ConnectorType::NON_CONNECTOR { .. }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

pub(crate) fn isFlowVar(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { connectorType: Deref @ DAE::ConnectorType::FLOW { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn isConst(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::CONST { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub fn isParam(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::PARAM { .. },
            ..
        } => true,
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::OPT_TGRID { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn makeParam(mut var: metamodelica::Ref<BackendDAE::Var>) -> metamodelica::Ref<BackendDAE::Var> {
    let mut var: metamodelica::Ref<BackendDAE::Var> = var;
    assign_field!(var.varKind = openmodelica_backend_types::BackendDAE::VarKind::PARAM);
    var
}

pub(crate) fn makeParamOutputsOnly(
    mut var: metamodelica::Ref<BackendDAE::Var>,
    mut fixed: bool,
) -> Result<(metamodelica::Ref<BackendDAE::Var>, bool)> {
    let mut var: metamodelica::Ref<BackendDAE::Var> = var;
    let mut fixed: bool = fixed;
    assign_field!(var.varKind = openmodelica_backend_types::BackendDAE::VarKind::PARAM);
    var = setHideResult(var, Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: true })));
    assign_field!(
        var.values = if ((var.values).is_some()) {
            var.values.clone()
        } else {
            Some(getVariableAttributefromType(&var.varType)?)
        }
    );
    if (DAEUtil::getFixedAttr(var.values.clone())).is_none() {
        assign_field!(
            var.values = DAEUtil::setFixedAttr(
                var.values.clone(),
                Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: fixed }))
            )?
        );
    }
    Ok((var, fixed))
}

pub(crate) fn isParamOrConstant(mut invar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outbool: bool = false;
    outbool = isParam(invar) || isConst(invar);
    outbool
}

pub(crate) fn isIntParam(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM { .. }, varType: Deref @ DAE::Type::T_INTEGER { .. }, .. } => true,
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM { .. }, varType: Deref @ DAE::Type::T_ENUMERATION { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn isBoolParam(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM { .. }, varType: Deref @ DAE::Type::T_BOOL { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn isStringParam(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM { .. }, varType: Deref @ DAE::Type::T_STRING { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn isExtObj(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::EXTOBJ { fullClassName: _ },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isAlgState(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::ALG_STATE { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isRealParam(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::PARAM { .. }, varType: Deref @ DAE::Type::T_REAL { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn isRealOptimizeConstraintsVars(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::OPT_CONSTR { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isDAEmodeVar(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::DAE_RESIDUAL_VAR { .. },
            ..
        } => true,
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::DAE_AUX_VAR { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub fn isDAEmodeResVar(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::DAE_RESIDUAL_VAR { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub fn isDAEmodeAuxVar(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::DAE_AUX_VAR { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isRealOptimizeFinalConstraintsVars(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::OPT_FCONSTR { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isRealOptimizeDerInput(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::OPT_INPUT_DER { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isAlgebraicOldState(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::ALG_STATE_OLD { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isCSEVar(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inVar {
        BackendDAE::Var { .. } if (CommonSubExpression::isCSECref(&inVar.varName)) => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isRESVar(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &*inVar.varName.clone() {
        DAE::ComponentRef::CREF_IDENT { ident: s, .. } => StringUtil::startsWith(s.clone(), literal!("$res")),
        DAE::ComponentRef::CREF_QUAL { ident: s, .. } => StringUtil::startsWith(s.clone(), literal!("$res")),
        _ => false,
    });
    outBoolean
}

pub(crate) fn hasMayerTermAnno(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { comment: Some(comm), .. } => {
            SCodeUtil::commentHasBooleanNamedAnnotation(metamodelica::AsArg::as_arg(&comm), &(literal!("isMayer")))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn hasOpenModelicaBoundaryConditionAnnotation(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { comment: Some(comm), .. } => {
            SCodeUtil::commentHasBooleanNamedAnnotation(metamodelica::AsArg::as_arg(&comm), &(literal!("__OpenModelica_BoundaryCondition")))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn hasLagrangeTermAnno(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { comment: Some(comm), .. } => {
            SCodeUtil::commentHasBooleanNamedAnnotation(metamodelica::AsArg::as_arg(&comm), &(literal!("isLagrange")))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn hasConTermAnno(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { comment: Some(comm), .. } => {
            SCodeUtil::commentHasBooleanNamedAnnotation(metamodelica::AsArg::as_arg(&comm), &(literal!("isConstraint")))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn hasFinalConTermAnno(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { comment: Some(comm), .. } => {
            SCodeUtil::commentHasBooleanNamedAnnotation(metamodelica::AsArg::as_arg(&comm), &(literal!("isFinalConstraint")))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn hasTimeGridAnno(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { comment: Some(comm), .. } => {
            SCodeUtil::commentHasBooleanNamedAnnotation(metamodelica::AsArg::as_arg(&comm), &(literal!("isTimeGrid")))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn isNonRealParam(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = !(isRealParam(inVar));
    outBoolean
}

pub fn isInput(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inVar {
        BackendDAE::Var {
            varDirection: DAE::VarDirection::INPUT { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub fn isOutputVar(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inVar {
        BackendDAE::Var {
            varDirection: DAE::VarDirection::OUTPUT { .. },
            ..
        } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isOutputAliasVar(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    let mut s: ArcStr;
    outBoolean = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { varName: Deref @ DAE::ComponentRef::CREF_IDENT { ident: __esc_s, .. }, .. } => {
            s = (*__esc_s).clone();
            if (StringUtil::startsWith(s.clone(), literal!("$outputAlias"))) {true} else {false}
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn isRealVar(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { varType: Deref @ DAE::Type::T_REAL { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn isRealOutputVar(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { varDirection: DAE::VarDirection::OUTPUT { .. }, varType: Deref @ DAE::Type::T_REAL { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub(crate) fn isOutput(mut inCref: metamodelica::Ref<DAE::ComponentRef>, mut inVars: &BackendDAE::Variables) -> bool {
    let mut outBool: bool;
    outBool = 'mc: {
        let __mc_input = inVars.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            ::match_deref::match_deref! { match &(getVar(inCref.clone(), inVars)?) {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varDirection: DAE::VarDirection::OUTPUT { .. }, .. }, tail: _ }, _) => (),
                _ => return Err("pattern mismatch"),
            } };
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
    outBool
}

pub(crate) fn isProtectedVar(mut v: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut hidden: bool = DAEUtil::getProtectedAttr(v.values.clone());
    hidden
}

pub(crate) fn isProtected(mut v: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(v.values.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { isProtected: Some(__esc_b), .. }) => {
            b = (*__esc_b).clone();
            b.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { isProtected: Some(__esc_b), .. }) => {
            b = (*__esc_b).clone();
            b.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { isProtected: Some(__esc_b), .. }) => {
            b = (*__esc_b).clone();
            b.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { isProtected: Some(__esc_b), .. }) => {
            b = (*__esc_b).clone();
            b.clone()
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { isProtected: Some(__esc_b), .. }) => {
            b = (*__esc_b).clone();
            b.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn hasVarEvaluateAnnotationTrueOrFinal(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut select: bool;
    select = isFinalVar(inVar) || hasVarEvaluateAnnotationTrue(inVar);
    select
}

pub(crate) fn hasVarEvaluateAnnotationTrueOrProtected(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut select: bool;
    select = isProtectedVar(inVar) || hasVarEvaluateAnnotationTrue(inVar);
    select
}

pub(crate) fn hasVarEvaluateAnnotationTrueOrFinalOrProtected(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut select: bool;
    select = isFinalOrProtectedVar(inVar) || hasVarEvaluateAnnotationTrue(inVar);
    select
}

pub(crate) fn hasVarEvaluateAnnotation(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut select: bool;
    select = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { comment: Some(Deref @ SCode::Comment { annotation_: Some(anno), .. }), .. } => {
            SCodeUtil::hasBooleanNamedAnnotation(metamodelica::AsArg::as_arg(&anno), &(literal!("Evaluate")))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    select
}

pub(crate) fn hasVarEvaluateAnnotationTrue(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut isTrue: bool;
    let mut ann: metamodelica::Ref<SCode::Annotation>;
    let mut val: metamodelica::Ref<Absyn::Exp>;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &((*inVar)) {
            Deref @ BackendDAE::Var { comment: Some(Deref @ SCode::Comment { annotation_: Some(__pa1), .. }), .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        ann = metamodelica::Own::own(__pa1);
        let __pa2 = ::match_deref::match_deref! { match &(SCodeUtil::lookupAnnotationBinding(&ann, &(literal!("Evaluate")))) {
            Some(__pa2) => __pa2.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        val = metamodelica::Own::own(__pa2);
        isTrue = stringEqual(
            &(unwrap_break_err!(Dump::printExpStr(val.clone()), '__try0)),
            &(literal!("true")),
        );
        Ok::<_, &'static str>((isTrue.clone(),))
    } {
        Ok((__try0_o0,)) => {
            isTrue = __try0_o0;
        }
        Err(_) => {
            isTrue = false;
        }
    }
    isTrue
}

pub(crate) fn hasVarEvaluateAnnotationFalse(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut isFalse: bool;
    let mut ann: metamodelica::Ref<SCode::Annotation>;
    let mut val: metamodelica::Ref<Absyn::Exp>;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &((*inVar)) {
            Deref @ BackendDAE::Var { comment: Some(Deref @ SCode::Comment { annotation_: Some(__pa1), .. }), .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        ann = metamodelica::Own::own(__pa1);
        let __pa2 = ::match_deref::match_deref! { match &(SCodeUtil::lookupAnnotationBinding(&ann, &(literal!("Evaluate")))) {
            Some(__pa2) => __pa2.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        val = metamodelica::Own::own(__pa2);
        isFalse = stringEqual(
            &(unwrap_break_err!(Dump::printExpStr(val.clone()), '__try0)),
            &(literal!("false")),
        );
        Ok::<_, &'static str>((isFalse.clone(),))
    } {
        Ok((__try0_o0,)) => {
            isFalse = __try0_o0;
        }
        Err(_) => {
            isFalse = false;
        }
    }
    isFalse
}

pub(crate) fn hasAnnotation(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut hasAnnot: bool;
    hasAnnot = (::match_deref::match_deref! { match inVar {
        Deref @ BackendDAE::Var { comment: Some(Deref @ SCode::Comment { annotation_: Some(_), .. }), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasAnnot
}

pub(crate) fn getNamedAnnotation(
    mut inVar: &metamodelica::Ref<BackendDAE::Var>,
    mut inName: &ArcStr,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut outValue: metamodelica::Ref<Absyn::Exp>;
    let mut ann: metamodelica::Ref<SCode::Annotation>;
    let __pa0 = ::match_deref::match_deref! { match &((*inVar)) {
        Deref @ BackendDAE::Var { comment: Some(Deref @ SCode::Comment { annotation_: Some(__pa0), .. }), .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    ann = metamodelica::Own::own(__pa0);
    let __pa1 = ::match_deref::match_deref! { match &(SCodeUtil::lookupAnnotationBinding(&ann, inName)) {
        Some(__pa1) => __pa1.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outValue = metamodelica::Own::own(__pa1);
    Ok(outValue)
}

pub(crate) fn getAnnotationComment(
    mut inVar: &metamodelica::Ref<BackendDAE::Var>,
) -> Result<Option<metamodelica::Ref<SCode::Comment>>> {
    let mut comment: Option<metamodelica::Ref<SCode::Comment>>;
    comment = (match &**inVar {
        BackendDAE::Var { comment: com, .. } => com.clone(),
        _ => return Err("fail"),
    });
    Ok(comment)
}

pub(crate) fn createpDerVar(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    cr = varCref(&inVar);
    cr = ComponentReferenceBasics::makeCrefQual(
        arcstr::literal!(BackendDAE::partialDerivativeNamePrefix),
        DAE::T_REAL_DEFAULT().clone(),
        metamodelica::nil(),
        cr,
    );
    outVar = copyVarNewName(cr, inVar);
    outVar = setVarKind(outVar, openmodelica_backend_types::BackendDAE::VarKind::JAC_TMP_VAR)?;
    Ok(outVar)
}

pub(crate) fn createClockedState(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    cr = ComponentReferenceBasics::makeCrefQual(
        arcstr::literal!(DAE::previousNamePrefix),
        DAE::T_REAL_DEFAULT().clone(),
        metamodelica::nil(),
        inVar.varName.clone(),
    );
    outVar = copyVarNewName(cr, inVar);
    outVar = setVarKind(outVar, openmodelica_backend_types::BackendDAE::VarKind::JAC_TMP_VAR)?;
    Ok(outVar)
}

pub(crate) fn createAliasDerVar(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    cr = ComponentReference::prependStringCref(arcstr::literal!(BackendDAE::derivativeNamePrefix), inCref)?;
    outVar = metamodelica::Ref::new(BackendDAE::Var {
        varName: cr,
        varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE,
        varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
        varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
        varType: DAE::T_REAL_DEFAULT().clone(),
        bindExp: None,
        tplExp: None,
        arryDim: metamodelica::nil(),
        source: DAE::emptyElementSource().clone(),
        values: None,
        tearingSelectOption: None,
        hideResult: None,
        comment: None,
        connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
        innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER,
        unreplaceable: false,
        initNonlinear: false,
        encrypted: false,
    });
    Ok(outVar)
}

pub(crate) fn createVar(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut prependStringCref: &ArcStr,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    cr = ComponentReference::appendStringLastIdent(prependStringCref, inCref)?;
    outVar = makeVar(cr)?;
    Ok(outVar)
}

pub(crate) fn createTmpVar(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut prependStringCref: &ArcStr,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    outVar = createVar(
        inCref,
        &({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*prependStringCref);
            __mm_s.push_str(&*intString(System::tmpTickIndex(Global::tmpVariableIndex.clone())));
            ArcStr::from(__mm_s)
        }),
    )?;
    Ok(outVar)
}

pub(crate) fn createCSEVar(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    outVar = (match &*inCref {
        _ if (ComponentReference::traverseCref(
            &inCref,
            &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: bool| ComponentReference::crefIsRec(&__a0, __a1),
            false,
        )?) =>
        {
            let mut source: metamodelica::Ref<DAE::ElementSource>;
            let mut path: metamodelica::Ref<Absyn::Path>;
            let mut varKind: BackendDAE::VarKind;
            let __pa0 = ::match_deref::match_deref! { match &(inType.clone()) {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: __pa0 }, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            path = metamodelica::Own::own(__pa0);
            source = metamodelica::Ref::new(DAE::ElementSource {
                info: Absyn::dummyInfo.clone(),
                partOfLst: metamodelica::nil(),
                instance: openmodelica_frontend_types::DAE::ComponentPrefix::interned_NOCOMPPRE(),
                connectEquationOptLst: metamodelica::nil(),
                typeLst: list![path],
                operations: metamodelica::nil(),
                comment: metamodelica::nil(),
            });
            varKind = if (Types::isDiscreteType(&inType)) {
                openmodelica_backend_types::BackendDAE::VarKind::DISCRETE
            } else {
                openmodelica_backend_types::BackendDAE::VarKind::VARIABLE
            };
            outVar = metamodelica::Ref::new(BackendDAE::Var {
                varName: inCref.clone(),
                varKind: varKind,
                varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
                varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
                varType: inType,
                bindExp: None,
                tplExp: None,
                arryDim: metamodelica::nil(),
                source: source,
                values: DAEUtil::setProtectedAttr(None, true)?,
                tearingSelectOption: Some(openmodelica_backend_types::BackendDAE::TearingSelect::NEVER),
                hideResult: Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: true })),
                comment: None,
                connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
                innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER,
                unreplaceable: true,
                initNonlinear: false,
                encrypted: false,
            });
            outVar
        }
        _ => {
            let mut varKind: BackendDAE::VarKind;
            varKind = if (Types::isDiscreteType(&inType)) {
                openmodelica_backend_types::BackendDAE::VarKind::DISCRETE
            } else {
                openmodelica_backend_types::BackendDAE::VarKind::VARIABLE
            };
            outVar = metamodelica::Ref::new(BackendDAE::Var {
                varName: inCref.clone(),
                varKind: varKind,
                varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
                varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
                varType: inType,
                bindExp: None,
                tplExp: None,
                arryDim: metamodelica::nil(),
                source: DAE::emptyElementSource().clone(),
                values: DAEUtil::setProtectedAttr(None, true)?,
                tearingSelectOption: Some(openmodelica_backend_types::BackendDAE::TearingSelect::NEVER),
                hideResult: Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: true })),
                comment: None,
                connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
                innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER,
                unreplaceable: true,
                initNonlinear: false,
                encrypted: false,
            });
            outVar
        }
    });
    Ok(outVar)
}

pub(crate) fn generateVar(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut varKind: BackendDAE::VarKind,
    mut varType: metamodelica::Ref<DAE::Type>,
    mut subs: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
) -> metamodelica::Ref<BackendDAE::Var> {
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    var = metamodelica::Ref::new(BackendDAE::Var {
        varName: cr,
        varKind: varKind,
        varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
        varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
        varType: varType,
        bindExp: None,
        tplExp: None,
        arryDim: subs,
        source: DAE::emptyElementSource().clone(),
        values: attr,
        tearingSelectOption: None,
        hideResult: None,
        comment: None,
        connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
        innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER,
        unreplaceable: false,
        initNonlinear: false,
        encrypted: false,
    });
    var
}

pub(crate) fn generateArrayVar(
    mut name: metamodelica::Ref<DAE::ComponentRef>,
    mut varKind: BackendDAE::VarKind,
    mut varType: metamodelica::Ref<DAE::Type>,
    mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    outVars = (match &*varType {
        DAE::Type::T_ARRAY { ty: tp, dims } => {
            let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
            crlst = ComponentReference::expandCref(&name, false)?;
            vars = List::map4(
                crlst,
                &fnptr!(
                    generateVar,
                    metamodelica::Ref<DAE::ComponentRef>,
                    BackendDAE::VarKind,
                    metamodelica::Ref<DAE::Type>,
                    metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
                    Option<metamodelica::Ref<DAE::VariableAttributes>>
                ),
                varKind,
                tp.clone(),
                dims.clone(),
                None,
            )?;
            vars
        }
        _ => {
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            var = metamodelica::Ref::new(BackendDAE::Var {
                varName: name,
                varKind: varKind,
                varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
                varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
                varType: varType,
                bindExp: None,
                tplExp: None,
                arryDim: metamodelica::nil(),
                source: DAE::emptyElementSource().clone(),
                values: attr,
                tearingSelectOption: None,
                hideResult: None,
                comment: None,
                connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
                innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER,
                unreplaceable: false,
                initNonlinear: false,
                encrypted: false,
            });
            list![var]
        }
    });
    Ok(outVars)
}

pub(crate) fn createCSEArrayVar(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inArryDim: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    outVar = (match &*inCref {
        _ if (ComponentReference::traverseCref(
            &inCref,
            &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: bool| ComponentReference::crefIsRec(&__a0, __a1),
            false,
        )?) =>
        {
            let mut source: metamodelica::Ref<DAE::ElementSource>;
            let mut path: metamodelica::Ref<Absyn::Path>;
            let mut varKind: BackendDAE::VarKind;
            let __pa0 = ::match_deref::match_deref! { match &(inType.clone()) {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: __pa0 }, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            path = metamodelica::Own::own(__pa0);
            source = metamodelica::Ref::new(DAE::ElementSource {
                info: Absyn::dummyInfo.clone(),
                partOfLst: metamodelica::nil(),
                instance: openmodelica_frontend_types::DAE::ComponentPrefix::interned_NOCOMPPRE(),
                connectEquationOptLst: metamodelica::nil(),
                typeLst: list![path],
                operations: metamodelica::nil(),
                comment: metamodelica::nil(),
            });
            varKind = if (Types::isDiscreteType(&inType)) {
                openmodelica_backend_types::BackendDAE::VarKind::DISCRETE
            } else {
                openmodelica_backend_types::BackendDAE::VarKind::VARIABLE
            };
            outVar = metamodelica::Ref::new(BackendDAE::Var {
                varName: inCref.clone(),
                varKind: varKind,
                varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
                varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
                varType: inType,
                bindExp: None,
                tplExp: None,
                arryDim: inArryDim,
                source: source,
                values: DAEUtil::setProtectedAttr(None, true)?,
                tearingSelectOption: Some(openmodelica_backend_types::BackendDAE::TearingSelect::NEVER),
                hideResult: Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: true })),
                comment: None,
                connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
                innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER,
                unreplaceable: true,
                initNonlinear: false,
                encrypted: false,
            });
            outVar
        }
        _ => {
            let mut varKind: BackendDAE::VarKind;
            varKind = if (Types::isDiscreteType(&inType)) {
                openmodelica_backend_types::BackendDAE::VarKind::DISCRETE
            } else {
                openmodelica_backend_types::BackendDAE::VarKind::VARIABLE
            };
            outVar = metamodelica::Ref::new(BackendDAE::Var {
                varName: inCref.clone(),
                varKind: varKind,
                varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
                varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
                varType: inType,
                bindExp: None,
                tplExp: None,
                arryDim: inArryDim,
                source: DAE::emptyElementSource().clone(),
                values: DAEUtil::setProtectedAttr(None, true)?,
                tearingSelectOption: Some(openmodelica_backend_types::BackendDAE::TearingSelect::NEVER),
                hideResult: Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: true })),
                comment: None,
                connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
                innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER,
                unreplaceable: true,
                initNonlinear: false,
                encrypted: false,
            });
            outVar
        }
    });
    Ok(outVar)
}

pub(crate) fn copyVarNewName(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
) -> metamodelica::Ref<BackendDAE::Var> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar;
    assign_field!(outVar.varName = cr);
    outVar
}

pub(crate) fn setVarKindForVar(
    mut idx: i32,
    mut kind: BackendDAE::VarKind,
    mut varsIn: BackendDAE::Variables,
) -> Result<BackendDAE::Variables> {
    let mut varsOut: BackendDAE::Variables;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    var = getVarAt(&varsIn, idx)?;
    var = setVarKind(var, kind)?;
    varsOut = setVarAt(varsIn, idx, var)?;
    Ok(varsOut)
}

pub(crate) fn setVarsKind(
    mut inVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inVarKind: BackendDAE::VarKind,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    outVars = List::map1(inVars, &setVarKind, inVarKind)?;
    Ok(outVars)
}

pub(crate) fn setVarKind(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inVarKind: BackendDAE::VarKind,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar;
    assign_field!(outVar.varKind = inVarKind);
    if isDummyStateVar(&outVar) && varStateSelectAlways(&outVar) {
        Error::addMessage(
            Error::NON_STATE_STATESELECT_ALWAYS.clone(),
            list![ComponentReference::crefStr(&(varCref(&outVar)))?],
        )?;
    }
    Ok(outVar)
}

pub(crate) fn setVarTS(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inTS: Option<BackendDAE::TearingSelect>,
) -> metamodelica::Ref<BackendDAE::Var> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar;
    assign_field!(outVar.tearingSelectOption = inTS);
    outVar
}

pub(crate) fn setBindExp(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inBindExp: Option<metamodelica::Ref<DAE::Exp>>,
) -> metamodelica::Ref<BackendDAE::Var> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar;
    assign_field!(outVar.bindExp = inBindExp);
    outVar
}

pub(crate) fn setHideResult(
    mut varIn: metamodelica::Ref<BackendDAE::Var>,
    mut hideResultB: Option<metamodelica::Ref<DAE::Exp>>,
) -> metamodelica::Ref<BackendDAE::Var> {
    let mut varOut: metamodelica::Ref<BackendDAE::Var> = varIn;
    assign_field!(varOut.hideResult = hideResultB);
    varOut
}

pub(crate) fn setVarDirectionTpl(
    mut var: metamodelica::Ref<BackendDAE::Var>,
    mut dir: DAE::VarDirection,
) -> (metamodelica::Ref<BackendDAE::Var>, DAE::VarDirection) {
    let mut var: metamodelica::Ref<BackendDAE::Var> = var;
    let mut dir: DAE::VarDirection = dir;
    assign_field!(var.varDirection = dir);
    (var, dir)
}

pub fn setVarDirection(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inVarDirection: DAE::VarDirection,
) -> metamodelica::Ref<BackendDAE::Var> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar;
    assign_field!(outVar.varDirection = inVarDirection);
    outVar
}

pub(crate) fn getVarDirection(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> DAE::VarDirection {
    let mut varDirection: DAE::VarDirection = inVar.varDirection.clone();
    varDirection
}

pub(crate) fn getVarNominalValue(mut InVar: &metamodelica::Ref<BackendDAE::Var>) -> metamodelica::Ref<DAE::Exp> {
    let mut nom: metamodelica::Ref<DAE::Exp> = DAEUtil::getNominalAttr(InVar.values.clone());
    nom
}

pub(crate) fn getVarKind(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> BackendDAE::VarKind {
    let mut varKind: BackendDAE::VarKind = inVar.varKind.clone();
    varKind
}

pub(crate) fn getVarKindForVar(mut idx: i32, mut varsIn: &BackendDAE::Variables) -> Result<BackendDAE::VarKind> {
    let mut kind: BackendDAE::VarKind;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    var = getVarAt(varsIn, idx)?;
    kind = getVarKind(&var);
    Ok(kind)
}

pub fn isVarOnTopLevelAndOutput(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool = isOutputVar(inVar);
    outBoolean
}

pub fn isVarOnTopLevelAndInput(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool = isInput(inVar);
    outBoolean
}

pub(crate) fn isVarOnTopLevelAndInputNoDerInput(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut outBoolean: bool = isVarOnTopLevelAndInput(inVar) && !(isRealOptimizeDerInput(inVar));
    outBoolean
}

pub(crate) fn isFinalVar(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut b: bool = DAEUtil::getFinalAttr(inVar.values.clone());
    b
}

pub(crate) fn isFinalOrProtectedVar(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> bool {
    let mut b: bool = isFinalVar(inVar) || isProtectedVar(inVar);
    b
}

pub(crate) fn isChangeable(mut v: &metamodelica::Ref<BackendDAE::Var>) -> Result<bool> {
    let mut isValueChangeable: bool = isVarOnTopLevelAndInput(v)
        || varFixed(v)
            && !(hasVarEvaluateAnnotationTrueOrFinalOrProtected(v))
            && if (isParam(v)) {
                varHasConstantBindExp(v)? || !(varHasBindExp(v)) && varHasConstantStartExp(v)
            } else {
                varHasConstantStartExp(v)
            };
    Ok(isValueChangeable)
}

pub(crate) fn getVariableAttributes(
    mut inVar: &metamodelica::Ref<BackendDAE::Var>,
) -> Option<metamodelica::Ref<DAE::VariableAttributes>> {
    let mut outAttr: Option<metamodelica::Ref<DAE::VariableAttributes>> = inVar.values.clone();
    outAttr
}

pub(crate) fn getVarSource(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> metamodelica::Ref<DAE::ElementSource> {
    let mut outSource: metamodelica::Ref<DAE::ElementSource> = inVar.source.clone();
    outSource
}

pub(crate) fn getVarType(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type> = inVar.varType.clone();
    outType
}

pub(crate) fn getMinMaxAsserts(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inAsserts: metamodelica::List<metamodelica::Ref<DAE::Algorithm>>,
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    metamodelica::List<metamodelica::Ref<DAE::Algorithm>>,
) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
    let mut outAsserts: metamodelica::List<metamodelica::Ref<DAE::Algorithm>>;
    outAsserts = 'mc: {
        let __mc_input = inVar;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::CONST { .. }, .. } => {
                    Ok(inAsserts.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Var { varName: name, values: attr, varType, source, .. } => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut cond: metamodelica::Ref<DAE::Exp>;
                    let mut msg: metamodelica::Ref<DAE::Exp>;
                    let mut level: metamodelica::Ref<DAE::Exp>;
                    let mut min: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut max: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut r#str: ArcStr;
                    let mut format: ArcStr;
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    (min, max) = DAEUtil::getMinMaxValues(attr.clone());
                    if (min).is_none() && (max).is_none() {
                        return Err("fail");
                    }
                    e = Expression::crefExp(name.clone())?;
                    tp = BackendDAEUtil::makeExpType(varType.clone());
                    cond = getMinMaxAsserts1(min.clone(), max.clone(), e.clone(), tp.clone())?;
                    (cond, _) = ExpressionSimplify::simplify(cond.clone())?;
                    let false = (Expression::isConstTrue(&cond)) else { return Err("pattern mismatch") };
                    r#str = getMinMaxAsserts1Str(min.clone(), max.clone(), &(ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&name))?))?;
                    if Flags::isSet(Flags::WARNING_MINMAX_ATTRIBUTES.clone())? {
                        level = DAE::ASSERTIONLEVEL_WARNING().clone();
                    } else {
                        level = DAE::ASSERTIONLEVEL_ERROR().clone();
                    }
                    format = if (Types::isRealOrSubTypeReal(tp.clone())) {literal!("g")} else {literal!("d")};
                    msg = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::SCONST { string: r#str.clone() }), operator: DAE::Operator::ADD { ty: DAE::T_STRING_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("String") }), expLst: list![e.clone(), metamodelica::Ref::new(DAE::Exp::SCONST { string: format.clone() })], attr: DAE::callAttrBuiltinString().clone() }) });
                    BackendDAEUtil::checkAssertCondition(&cond, msg.clone(), &level, &(ElementSource::getElementSourceFileInfo(source.clone())))?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(DAE::Algorithm { statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_ASSERT { cond: cond.clone(), msg: msg.clone(), level: level.clone(), source: source.clone() })] }), inAsserts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inAsserts.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outVar, outAsserts)
}

fn getMinMaxAsserts1(
    mut omin: Option<metamodelica::Ref<DAE::Exp>>,
    mut omax: Option<metamodelica::Ref<DAE::Exp>>,
    mut e: metamodelica::Ref<DAE::Exp>,
    mut tp: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut cond: metamodelica::Ref<DAE::Exp>;
    cond = (::match_deref::match_deref! { match &((omin, omax)) {
        (Some(min), Some(max)) => {
            metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e.clone(), operator: DAE::Operator::GREATEREQ { ty: tp.clone() }, exp2: min.clone(), index: -1, optionExpisASUB: None }), operator: DAE::Operator::AND { ty: DAE::T_BOOL_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e, operator: DAE::Operator::LESSEQ { ty: tp }, exp2: max.clone(), index: -1, optionExpisASUB: None }) })
        },
        (Some(min), None) => {
            metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e, operator: DAE::Operator::GREATEREQ { ty: tp }, exp2: min.clone(), index: -1, optionExpisASUB: None })
        },
        (None, Some(max)) => {
            metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e, operator: DAE::Operator::LESSEQ { ty: tp }, exp2: max.clone(), index: -1, optionExpisASUB: None })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(cond)
}

fn getMinMaxAsserts1Str(
    mut omin: Option<metamodelica::Ref<DAE::Exp>>,
    mut omax: Option<metamodelica::Ref<DAE::Exp>>,
    mut varStr: &ArcStr,
) -> Result<ArcStr> {
    let mut msg: ArcStr;
    msg = (::match_deref::match_deref! { match &((omin, omax)) {
        (Some(min), Some(max)) => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable violating min/max constraint: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(min.clone())?); __mm_s.push_str(&*literal!(" <= ")); __mm_s.push_str(&*varStr); __mm_s.push_str(&*literal!(" <= ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(max.clone())?); __mm_s.push_str(&*literal!(", has value: ")); ArcStr::from(__mm_s) }
        },
        (Some(min), None) => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable violating min constraint: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(min.clone())?); __mm_s.push_str(&*literal!(" <= ")); __mm_s.push_str(&*varStr); __mm_s.push_str(&*literal!(", has value: ")); ArcStr::from(__mm_s) }
        },
        (None, Some(max)) => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable violating max constraint: ")); __mm_s.push_str(&*varStr); __mm_s.push_str(&*literal!(" <= ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(max.clone())?); __mm_s.push_str(&*literal!(", has value: ")); ArcStr::from(__mm_s) }
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(msg)
}

pub(crate) fn varSortFunc(
    mut v1: &metamodelica::Ref<BackendDAE::Var>,
    mut v2: &metamodelica::Ref<BackendDAE::Var>,
) -> Result<bool> {
    let mut greaterThan: bool;
    greaterThan = ComponentReferenceBasics::crefSortFunc(&(varCref(v1)), &(varCref(v2)))?;
    Ok(greaterThan)
}

pub(crate) fn sortInitialVars(
    mut vars: BackendDAE::Variables,
    mut fixableVars: &BackendDAE::Variables,
) -> Result<BackendDAE::Variables> {
    let mut vars: BackendDAE::Variables = vars;
    let mut var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut fixable_start: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut fixable: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut non_fixable: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    var_lst = varList(&vars)?;
    (fixable, non_fixable) = List::splitOnTrue(
        &var_lst,
        &({
            let __pe_b1 = fixableVars.clone();
            move |__pe_a0| Ok(containsVar(&__pe_a0, &__pe_b1))
        }),
    )?;
    (fixable_start, fixable) = List::splitOnTrue(
        &fixable,
        &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(varHasStartValue(&__a0))
        },
    )?;
    var_lst = listAppend(listAppend(fixable_start, fixable.reverse()), non_fixable.reverse());
    vars = listVar(var_lst)?;
    Ok(vars)
}

pub(crate) fn getAlias(
    mut inVar: &metamodelica::Ref<BackendDAE::Var>,
) -> Result<(metamodelica::Ref<DAE::ComponentRef>, bool)> {
    let mut outCr: metamodelica::Ref<DAE::ComponentRef>;
    let mut negated: bool;
    let mut e: metamodelica::Ref<DAE::Exp>;
    e = varBindExp(inVar)?;
    (outCr, negated) = getAlias1(&e)?;
    Ok((outCr, negated))
}

fn getAlias1(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<(metamodelica::Ref<DAE::ComponentRef>, bool)> {
    let mut outCr: metamodelica::Ref<DAE::ComponentRef>;
    let mut negated: bool;
    (outCr, negated) = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::CREF { componentRef: name, .. } => {
            (name.clone(), false)
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: name, .. } } => {
            (name.clone(), true)
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: name, .. } } => {
            (name.clone(), true)
        },
        Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: name, .. } } => {
            (name.clone(), true)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: name, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut name = (*name).clone();
            name = ComponentReference::crefPrefixDer(name.clone());
            (name.clone(), false)
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: name, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } } => {
            let mut name = (*name).clone();
            name = ComponentReference::crefPrefixDer(name.clone());
            (name.clone(), true)
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: name, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } } => {
            let mut name = (*name).clone();
            name = ComponentReference::crefPrefixDer(name.clone());
            (name.clone(), true)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCr, negated))
}

pub(crate) fn daenumVariables(mut syst: &metamodelica::Ref<BackendDAE::EqSystem>) -> i32 {
    let mut n: i32;
    let mut vars: BackendDAE::Variables;
    vars = daeVars(syst);
    n = varsSize(&vars);
    n
}

/* =======================================================
 *
 *  Section for functions that deals with VariablesArray
 *
 * =======================================================
 */
fn copyArray(mut inVariableArray: BackendDAE::VariableArray) -> BackendDAE::VariableArray {
    let mut outVariableArray: BackendDAE::VariableArray = inVariableArray.clone();
    outVariableArray.varOptArr = metamodelica::arrayFromVec(inVariableArray.varOptArr.clone().borrow().clone());
    outVariableArray
}

fn vararrayEmpty(mut inSize: i32) -> BackendDAE::VariableArray {
    let mut outArray: BackendDAE::VariableArray;
    let mut arr: metamodelica::Array<Option<metamodelica::Ref<BackendDAE::Var>>>;
    arr = arrayCreate(inSize, None);
    outArray = BackendDAE::VariableArray {
        numberOfElements: 0,
        varOptArr: arr.clone(),
    };
    outArray
}

fn vararrayAdd(
    mut inVariableArray: BackendDAE::VariableArray,
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
) -> Result<BackendDAE::VariableArray> {
    let mut outVariableArray: BackendDAE::VariableArray;
    let mut num_elems: i32;
    let mut arr: metamodelica::Array<Option<metamodelica::Ref<BackendDAE::Var>>>;
    let BackendDAE::VARIABLE_ARRAY {
        numberOfElements: __pa0,
        varOptArr: __pa1,
    } = inVariableArray;
    num_elems = metamodelica::Own::own(__pa0);
    arr = metamodelica::Own::own(__pa1);
    num_elems = num_elems + 1;
    arr = Array::expandOnDemand(num_elems, arr.clone(), metamodelica::OrderedFloat(1.4_f64), None)?;
    metamodelica::arrayUpdate(arr.clone(), num_elems, Some(inVar))?;
    outVariableArray = BackendDAE::VariableArray {
        numberOfElements: num_elems,
        varOptArr: arr.clone(),
    };
    Ok(outVariableArray)
}

fn vararraySetnth(
    mut inVariableArray: BackendDAE::VariableArray,
    mut inIndex: i32,
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
) -> Result<BackendDAE::VariableArray> {
    let mut outVariableArray: BackendDAE::VariableArray = inVariableArray.clone();
    let true = (inIndex <= inVariableArray.numberOfElements.clone()) else {
        return Err("pattern mismatch");
    };
    metamodelica::arrayUpdate(inVariableArray.varOptArr.clone(), inIndex, Some(inVar))?;
    Ok(outVariableArray)
}

fn vararrayNth(
    mut inVariableArray: &BackendDAE::VariableArray,
    mut inIndex: i32,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let true = (inIndex <= inVariableArray.numberOfElements.clone()) else {
        return Err("pattern mismatch");
    };
    let __pa0 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(inVariableArray.varOptArr.clone(), inIndex)?) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outVar = metamodelica::Own::own(__pa0);
    Ok(outVar)
}

fn vararrayDelete(
    mut inVariableArray: BackendDAE::VariableArray,
    mut inIndex: i32,
) -> Result<(BackendDAE::VariableArray, metamodelica::Ref<BackendDAE::Var>)> {
    let mut outVariableArray: BackendDAE::VariableArray = inVariableArray;
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let __pa0 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(outVariableArray.varOptArr.clone(), inIndex)?) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outVar = metamodelica::Own::own(__pa0);
    metamodelica::arrayUpdate(outVariableArray.varOptArr.clone(), inIndex, None)?;
    Ok((outVariableArray, outVar))
}

fn vararrayList(
    mut inArray: BackendDAE::VariableArray,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut varOptArr: metamodelica::Array<Option<metamodelica::Ref<BackendDAE::Var>>>;
    let BackendDAE::VARIABLE_ARRAY { varOptArr: __pa0, .. } = inArray;
    varOptArr = metamodelica::Own::own(__pa0);
    outVars = metamodelica::nil();
    for mut i in ({
        let __s = metamodelica::arrayLength(varOptArr.clone());
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        if ({
            let __elt = (*metamodelica::index_checked(&varOptArr.borrow(), i)?).clone();
            __elt
        })
        .is_some()
        {
            outVars = metamodelica::cons(
                ({
                    let __elt = (*metamodelica::index_checked(&varOptArr.borrow(), i)?).clone();
                    __elt
                })
                .ok_or("pattern mismatch")?,
                outVars,
            );
        }
    }
    Ok(outVars)
}

/* =======================================================
 *
 *  Section for functions that deals with Variables
 *
 * =======================================================
 */
pub(crate) fn copyVariables(mut inVariables: BackendDAE::Variables) -> BackendDAE::Variables {
    let mut outVariables: BackendDAE::Variables;
    outVariables = inVariables.clone();
    outVariables.crefIndices = metamodelica::arrayFromVec(inVariables.crefIndices.clone().borrow().clone());
    outVariables.prefixIndices = metamodelica::arrayFromVec(inVariables.prefixIndices.clone().borrow().clone());
    outVariables.varArr = copyArray(inVariables.varArr.clone());
    outVariables
}

pub fn emptyVars(mut inSize: i32) -> BackendDAE::Variables {
    let mut outVariables: BackendDAE::Variables;
    let mut indices: metamodelica::Array<metamodelica::List<BackendDAE::CrefIndex>>;
    let mut buckets: i32;
    let mut arr_size: i32;
    let mut arr: BackendDAE::VariableArray;
    arr_size = std::cmp::max(inSize, BaseHashTable::lowBucketSize.clone());
    buckets = bucketCount(arr_size);
    indices = arrayCreate(buckets, metamodelica::nil());
    arr = vararrayEmpty(arr_size);
    outVariables = BackendDAE::Variables {
        crefIndices: indices.clone(),
        prefixIndices: arrayCreate(buckets, metamodelica::nil()),
        varArr: arr,
        bucketSize: buckets,
        numberOfVars: 0,
        hasStartVars: false,
    };
    outVariables
}

fn bucketCount(mut numVars: i32) -> i32 {
    let mut buckets: i32 = ((intReal(std::cmp::max(numVars, BaseHashTable::lowBucketSize.clone()))
        * metamodelica::OrderedFloat(1.4_f64))
    .0
    .floor() as i32);
    buckets
}

fn growBuckets(mut vars: BackendDAE::Variables) -> Result<BackendDAE::Variables> {
    let mut vars: BackendDAE::Variables = vars;
    let mut indices: metamodelica::Array<metamodelica::List<BackendDAE::CrefIndex>>;
    let mut buckets: i32;
    let mut idx: i32;
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    buckets = bucketCount(2 * vars.numberOfVars.clone());
    indices = arrayCreate(buckets, metamodelica::nil());
    vars.crefIndices = indices.clone();
    vars.prefixIndices = arrayCreate(buckets, metamodelica::nil());
    vars.bucketSize = buckets;
    for mut i in 1..=vars.varArr.numberOfElements.clone() {
        if ({
            let __elt = (*metamodelica::index_checked(&vars.varArr.varOptArr.borrow(), i)?).clone();
            __elt
        })
        .is_some()
        {
            let __pa0 = ::match_deref::match_deref! { match &(({let __elt = (*metamodelica::index_checked(&vars.varArr.varOptArr.borrow(), i)?).clone(); __elt})) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            v = metamodelica::Own::own(__pa0);
            idx = intMod(ComponentReferenceBasics::hashComponentRef(&v.varName)?, buckets) + 1;
            metamodelica::arrayUpdate(
                indices.clone(),
                idx,
                metamodelica::cons(
                    BackendDAE::CrefIndex {
                        cref: v.varName.clone(),
                        index: i - 1,
                    },
                    ({
                        let __elt = (*metamodelica::index_checked(&indices.borrow(), idx)?).clone();
                        __elt
                    }),
                ),
            )?;
            updatePrefixIndices(v.varName.clone(), i - 1, &vars)?;
        }
    }
    Ok(vars)
}

pub(crate) fn emptyVarsSized(mut size: i32) -> BackendDAE::Variables {
    let mut outVariables: BackendDAE::Variables = emptyVars(size);
    outVariables
}

pub(crate) fn isCrefInVarList(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inVars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<bool> {
    let mut isInList: bool = false;
    for mut v in &**inVars {
        if ComponentReferenceBasics::crefEqual(&(varCref(metamodelica::AsArg::as_arg(&v))), inCref)? {
            isInList = true;
            return Ok(isInList);
        }
    }
    Ok(isInList)
}

pub(crate) fn areAllCrefsInVarList(
    mut inCrefs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inVars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<bool> {
    let mut isInList: bool = true;
    for mut cref in &**inCrefs {
        if !(isCrefInVarList(metamodelica::AsArg::as_arg(&cref), inVars)?) {
            isInList = false;
            return Ok(isInList);
        }
    }
    Ok(isInList)
}

pub(crate) fn areAllCrefsPrimaryParameters(
    mut inCrefs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inVars: &BackendDAE::Variables,
) -> bool {
    let mut isPrimary: bool = true;
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    for mut cref in &**inCrefs {
        match '__try0: {
            (v, _) = unwrap_break_err!(getVar2(metamodelica::AsArg::as_arg(&cref), inVars), '__try0);
            let true = (isParam(&v) && varFixed(&v)) else {
                break '__try0 Err::<_, _>("pattern mismatch");
            };
            Ok::<_, &'static str>((v.clone(),))
        } {
            Ok((__try0_o0,)) => {
                v = __try0_o0;
            }
            Err(_) => {
                isPrimary = false;
                return isPrimary;
            }
        }
    }
    isPrimary
}

pub fn varList(
    mut inVariables: &BackendDAE::Variables,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut outVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    outVarLst = vararrayList(inVariables.varArr.clone())?;
    Ok(outVarLst)
}

pub fn listVar(mut inVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    let mut size: i32;
    size = ((inVarLst).len() as i32);
    outVariables = emptyVarsSized(size);
    outVariables = addVars(&(inVarLst.reverse()), outVariables)?;
    Ok(outVariables)
}

pub(crate) fn listVarSized(
    mut inVarLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut size: i32,
) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    outVariables = List::fold(inVarLst, &addVar, emptyVarsSized(size))?;
    Ok(outVariables)
}

pub fn listVar1(
    mut inVarLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    let mut size: i32;
    size = ((inVarLst).len() as i32);
    outVariables = List::fold(inVarLst, &addVar, emptyVarsSized(size))?;
    Ok(outVariables)
}

pub(crate) fn listVar2(
    mut inVarLst1: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inVarLst2: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    let mut size: i32;
    size = ((inVarLst1).len() as i32) + ((inVarLst2).len() as i32);
    outVariables = List::fold(
        inVarLst2,
        &addVar,
        List::fold(inVarLst1, &addVar, emptyVarsSized(size))?,
    )?;
    Ok(outVariables)
}

pub fn equationSystemsVarsLst(
    mut systs: &metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut v: BackendDAE::Variables;
    for mut es in &**systs {
        let __arc1 = es.clone();
        let BackendDAE::EQSYSTEM { orderedVars: __pa0, .. } = &*__arc1;
        v = metamodelica::Own::own(__pa0);
        vars = varList(&v)?;
        outVars = List::append_reverse(&vars, outVars);
    }
    outVars = metamodelica::Dangerous::listReverseInPlace(outVars);
    Ok(outVars)
}

pub fn daeVars(mut inEqSystem: &metamodelica::Ref<BackendDAE::EqSystem>) -> BackendDAE::Variables {
    let mut vars: BackendDAE::Variables = inEqSystem.orderedVars.clone();
    vars
}

pub fn daeGlobalKnownVars(mut inShared: &metamodelica::Ref<BackendDAE::Shared>) -> BackendDAE::Variables {
    let mut outGlobalKnownVars: BackendDAE::Variables = inShared.globalKnownVars.clone();
    outGlobalKnownVars
}

pub(crate) fn daeAliasVars(mut inShared: &metamodelica::Ref<BackendDAE::Shared>) -> BackendDAE::Variables {
    let mut outAliasVars: BackendDAE::Variables = inShared.aliasVars.clone();
    outAliasVars
}

pub fn varsSize(mut inVariables: &BackendDAE::Variables) -> i32 {
    let mut outNumVariables: i32 = inVariables.varArr.numberOfElements.clone();
    outNumVariables
}

/*
public function varDim
  "Returns the dimension of variables in the Variables structure.
  NOTE: function fail if dimension is not constant
  "
  input BackendDAE.Var inVar;
  output Integer outDimVariables = 1;
protected
  DAE.Dimensions dims;
  Integer n;
algorithm
  BackendDAE.VAR(arryDim=dims) := inVar;
  for dim in dims loop
    DAE.DIM_INTEGER(n) := dim;
    outDimVariables := n * outDimVariables;
  end for;
end varDim;
*/
fn varsLoadFactor(mut inVariables: &BackendDAE::Variables, mut inIncrease: i32) -> Result<metamodelica::Real> {
    let mut outLoadFactor: metamodelica::Real;
    outLoadFactor = metamodelica::real_div_checked(
        intReal(inVariables.numberOfVars.clone() + inIncrease),
        intReal(inVariables.bucketSize.clone()),
    )?;
    Ok(outLoadFactor)
}

pub(crate) fn isVariable(
    mut inComponentRef1: metamodelica::Ref<DAE::ComponentRef>,
    mut inVariables2: BackendDAE::Variables,
    mut inVariables3: BackendDAE::Variables,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (inComponentRef1, inVariables2, inVariables3);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, vars, _) => {
                    let mut kind: BackendDAE::VarKind;
                    let __pa0 = ::match_deref::match_deref! { match &(getVar(cr.clone(), metamodelica::AsArg::as_arg(&vars))?) {
                        (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: __pa0, .. }, tail: _ }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    kind = metamodelica::Own::own(__pa0);
                    isVarKindVariable(&kind)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, _, globalKnownVars) => {
                    let mut kind: BackendDAE::VarKind;
                    let __pa0 = ::match_deref::match_deref! { match &(getVar(cr.clone(), metamodelica::AsArg::as_arg(&globalKnownVars))?) {
                        (Deref @ metamodelica::ListNode::Cons { head: Deref @ BackendDAE::Var { varKind: __pa0, .. }, tail: _ }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    kind = metamodelica::Own::own(__pa0);
                    isVarKindVariable(&kind)?;
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

pub(crate) fn isVarKindVariable(mut inVarKind: &BackendDAE::VarKind) -> Result<()> {
    let () = (match inVarKind.clone() {
        BackendDAE::VarKind::VARIABLE { .. } => (),
        BackendDAE::VarKind::STATE { .. } => (),
        BackendDAE::VarKind::DUMMY_STATE { .. } => (),
        BackendDAE::VarKind::DUMMY_DER { .. } => (),
        BackendDAE::VarKind::DISCRETE { .. } => (),
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

pub(crate) fn isVarKindState(mut inVarKind: &BackendDAE::VarKind) -> bool {
    let mut result: bool;
    result = (match inVarKind.clone() {
        BackendDAE::VarKind::STATE { .. } => true,
        _ => false,
    });
    result
}

pub(crate) fn isTopLevelInputOrOutput(
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut inVars: &BackendDAE::Variables,
    mut inGlobalKnownVars: &BackendDAE::Variables,
) -> bool {
    let mut outBoolean: bool;
    outBoolean = 'mc: {
        let __mc_input = &*inComponentRef;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let __pa0 = ::match_deref::match_deref! { match &(getVar(inComponentRef.clone(), inVars)?) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    v = metamodelica::Own::own(__pa0);
                    Ok(isVarOnTopLevelAndOutput(&v))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let __pa0 = ::match_deref::match_deref! { match &(getVar(inComponentRef.clone(), inGlobalKnownVars)?) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    v = metamodelica::Own::own(__pa0);
                    Ok(isVarOnTopLevelAndInput(&v))
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

pub(crate) fn deleteCrefs(
    mut varlst: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut vars: BackendDAE::Variables,
) -> Result<BackendDAE::Variables> {
    let mut vars_1: BackendDAE::Variables;
    vars_1 = List::fold(
        varlst,
        &fnptr!(removeCref, metamodelica::Ref<DAE::ComponentRef>, BackendDAE::Variables),
        vars,
    )?;
    vars_1 = listVar1(&(varList(&vars_1)?))?;
    Ok(vars_1)
}

pub(crate) fn deleteVars(
    mut inDelVars: BackendDAE::Variables,
    mut inVariables: BackendDAE::Variables,
) -> BackendDAE::Variables {
    let mut outVariables: BackendDAE::Variables;
    outVariables = 'mc: {
        let __mc_input = inVariables.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut newvars: BackendDAE::Variables;
            let true = (intGt(varsSize(&inDelVars), 0)) else {
                return Err("pattern mismatch");
            };
            newvars = traverseBackendDAEVars(
                inDelVars.clone(),
                (std::sync::Arc::new(fnptr!(
                    deleteVars1,
                    metamodelica::Ref<BackendDAE::Var>,
                    BackendDAE::Variables
                ))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<BackendDAE::Var>,
                                BackendDAE::Variables,
                            )
                                -> Result<(metamodelica::Ref<BackendDAE::Var>, BackendDAE::Variables)>
                            + 'static,
                    >),
                inVariables.clone(),
            )?;
            newvars = listVar1(&(varList(&newvars)?))?;
            Ok(newvars.clone())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(inVariables.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outVariables
}

fn deleteVars1(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inVars: BackendDAE::Variables,
) -> (metamodelica::Ref<BackendDAE::Var>, BackendDAE::Variables) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
    let mut outVars: BackendDAE::Variables;
    outVars = removeCref(inVar.varName.clone(), inVars);
    (outVar, outVars)
}

pub(crate) fn deleteVar(
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut inVariables: &BackendDAE::Variables,
) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    outVariables = (::match_deref::match_deref! { match &(inComponentRef) {
        cr => {
            let mut vars: BackendDAE::Variables;
            let mut ilst: metamodelica::List<i32>;
            (_, ilst) = getVar(cr.clone(), inVariables)?;
            (vars, _) = removeVars(&ilst, inVariables, &(metamodelica::nil()));
            vars = listVar1(&(varList(&vars)?))?;
            vars
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVariables)
}

pub(crate) fn deleteVarIfExistsAndReturn(
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut inVariables: BackendDAE::Variables,
) -> (
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    BackendDAE::Variables,
) {
    let mut outVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut outVariables: BackendDAE::Variables = inVariables.clone();
    let mut ilst: metamodelica::List<i32>;
    if '__try0: {
        (outVarLst, ilst) = unwrap_break_err!(getVar(inComponentRef.clone(), &inVariables), '__try0);
        (outVariables, _) = removeVars(&ilst, &inVariables, &(metamodelica::nil()));
        outVariables = unwrap_break_err!(listVar1(&(unwrap_break_err!(varList(&outVariables), '__try0))), '__try0);
        Ok::<(), &'static str>(())
    }
    .is_err()
    {}
    (outVarLst, outVariables)
}

pub(crate) fn removeCrefs(
    mut varlst: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut vars: BackendDAE::Variables,
) -> Result<BackendDAE::Variables> {
    let mut vars_1: BackendDAE::Variables;
    vars_1 = List::fold(
        varlst,
        &fnptr!(removeCref, metamodelica::Ref<DAE::ComponentRef>, BackendDAE::Variables),
        vars,
    )?;
    Ok(vars_1)
}

pub(crate) fn removeCref(
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut inVariables: BackendDAE::Variables,
) -> BackendDAE::Variables {
    let mut outVariables: BackendDAE::Variables;
    outVariables = 'mc: {
        let __mc_input = inComponentRef;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cr => {
                    let mut vars: BackendDAE::Variables;
                    let mut ilst: metamodelica::List<i32>;
                    (_, ilst) = getVar(cr.clone(), &inVariables)?;
                    (vars, _) = removeVars(&ilst, &inVariables, &(metamodelica::nil()));
                    Ok(vars.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inVariables.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outVariables
}

pub(crate) fn removeVars(
    mut inVarPos: &metamodelica::List<i32>,
    mut inVariables: &BackendDAE::Variables,
    mut iAcc: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> (
    BackendDAE::Variables,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) {
    let mut outVariables: BackendDAE::Variables;
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    (outVariables, outVars) = 'mc: {
        let __mc_input = &**inVarPos;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((inVariables.clone(), iAcc.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: i, tail: ilst } => {
                    let mut vars: BackendDAE::Variables;
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let mut acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    (vars, v) = removeVar(i.clone(), inVariables.clone())?;
                    (vars, acc) = removeVars(metamodelica::AsArg::as_arg(&ilst), &vars, &(metamodelica::cons(v.clone(), iAcc.clone())));
                    Ok((vars.clone(), acc.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: ilst } => {
                    let mut vars: BackendDAE::Variables;
                    let mut acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    (vars, acc) = removeVars(metamodelica::AsArg::as_arg(&ilst), inVariables, iAcc);
                    Ok((vars.clone(), acc.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outVariables, outVars)
}

pub(crate) fn removeVarDAE(
    mut inVarPos: i32,
    mut inEqSystem: metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Var>,
)> {
    let mut outEqSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut vars: BackendDAE::Variables;
    (vars, outVar) = removeVar(inVarPos, inEqSystem.orderedVars.clone())?;
    outEqSystem = BackendDAEUtil::setEqSystVars(inEqSystem, vars);
    Ok((outEqSystem, outVar))
}

pub(crate) fn removeAliasVars(
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> metamodelica::Ref<BackendDAE::Shared> {
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    outShared = BackendDAEUtil::setSharedAliasVars(inShared, emptyVars(BaseHashTable::bigBucketSize.clone()));
    outShared
}

pub(crate) fn removeVar(
    mut inIndex: i32,
    mut inVariables: BackendDAE::Variables,
) -> Result<(BackendDAE::Variables, metamodelica::Ref<BackendDAE::Var>)> {
    let mut outVariables: BackendDAE::Variables;
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut indices: metamodelica::Array<metamodelica::List<BackendDAE::CrefIndex>>;
    let mut cr_indices: metamodelica::List<BackendDAE::CrefIndex>;
    let mut arr: BackendDAE::VariableArray;
    let mut buckets: i32;
    let mut num_vars: i32;
    let mut hash_idx: i32;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let BackendDAE::VARIABLES {
        crefIndices: __pa0,
        varArr: __pa1,
        bucketSize: __pa2,
        numberOfVars: __pa3,
        ..
    } = &inVariables;
    indices = metamodelica::Own::own(__pa0);
    arr = metamodelica::Own::own(__pa1);
    buckets = metamodelica::Own::own(__pa2);
    num_vars = metamodelica::Own::own(__pa3);
    let (__pa4, __pa6, __pa5) = ::match_deref::match_deref! { match &(vararrayDelete(arr, inIndex)?) {
        (__pa4, __pa6 @ Deref @ BackendDAE::Var { varName: __pa5, .. }) => (__pa4.clone(), __pa6.clone(), __pa5.clone()),
        _ => unreachable!(),
    } };
    arr = metamodelica::Own::own(__pa4);
    cr = metamodelica::Own::own(__pa5);
    outVar = metamodelica::Own::own(__pa6);
    hash_idx = intMod(ComponentReferenceBasics::hashComponentRef(&cr)?, buckets) + 1;
    cr_indices = ({
        let __elt = (*metamodelica::index_checked(&indices.borrow(), hash_idx)?).clone();
        __elt
    });
    (cr_indices, _) = List::deleteMemberOnTrue(
        BackendDAE::CrefIndex {
            cref: cr,
            index: inIndex - 1,
        },
        cr_indices,
        &move |__a0: BackendDAE::CrefIndex, __a1: BackendDAE::CrefIndex| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(removeVar2(&__a0, &__a1))
        },
    )?;
    metamodelica::arrayUpdate(indices.clone(), hash_idx, cr_indices)?;
    outVariables = inVariables;
    outVariables.varArr = arr;
    outVariables.numberOfVars = num_vars - 1;
    Ok((outVariables, outVar))
}

fn removeVar2(mut inCrefIndex1: &BackendDAE::CrefIndex, mut inCrefIndex2: &BackendDAE::CrefIndex) -> bool {
    let mut outMatch: bool;
    outMatch = inCrefIndex1.index.clone() == inCrefIndex2.index.clone();
    outMatch
}

pub(crate) fn isKnownAndParam(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut knownVars: BackendDAE::Variables,
) -> Result<bool> {
    let mut outBoolean: bool;
    let mut tpl: (bool, BackendDAE::Variables) = (true, knownVars.clone());
    let (_, (__pa0, _)) = Expression::traverseExpBottomUp(
        inExp,
        &fnptr!(
            isKnownAndParamWork,
            metamodelica::Ref<DAE::Exp>,
            (bool, BackendDAE::Variables)
        ),
        tpl,
    )?;
    outBoolean = metamodelica::Own::own(__pa0);
    Ok(outBoolean)
}

fn isKnownAndParamWork(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut tpl: (bool, BackendDAE::Variables),
) -> (metamodelica::Ref<DAE::Exp>, (bool, BackendDAE::Variables)) {
    let mut inExp: metamodelica::Ref<DAE::Exp> = inExp;
    let mut tpl: (bool, BackendDAE::Variables) = tpl;
    let mut outBoolean: bool;
    let mut knownVars: BackendDAE::Variables;
    (outBoolean, knownVars) = tpl;
    tpl = (::match_deref::match_deref! { match &((&*inExp, outBoolean)) {
        (_, false) => {
            (false, knownVars)
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, _) => {
            (crefIsParam(cr.clone(), &knownVars), knownVars)
        },
        _ => {
            (true, knownVars)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (inExp, tpl)
}

pub(crate) fn crefIsParam(
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut inVariables: &BackendDAE::Variables,
) -> bool {
    let mut outBool: bool = true;
    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    if '__try0: {
        (varlst, _) = unwrap_break_err!(getVar(inComponentRef.clone(), inVariables), '__try0);
        for mut var in &*varlst {
            outBool = isParam(metamodelica::AsArg::as_arg(&var));
            if !(outBool) {
                return outBool;
            }
        }
        Ok::<(), &'static str>(())
    }
    .is_err()
    {
        outBool = false;
    }
    outBool
}

pub(crate) fn existsVar(
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut inVariables: &BackendDAE::Variables,
    mut skipDiscrete: bool,
) -> bool {
    let mut outExists: bool;
    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    match '__try0: {
        (varlst, _) = unwrap_break_err!(getVar(inComponentRef.clone(), inVariables), '__try0);
        varlst = if (skipDiscrete) {
            unwrap_break_err!(List::select(varlst.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isVarNonDiscrete(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>)), '__try0)
        } else {
            varlst.clone()
        };
        outExists = !((varlst).is_empty());
        Ok::<_, &'static str>((outExists.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outExists = __try0_o0;
        }
        Err(_) => {
            outExists = false;
        }
    }
    outExists
}

pub(crate) fn existsAnyVar(
    mut inComponentRefs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inVariables: &BackendDAE::Variables,
    mut skipDiscrete: bool,
) -> bool {
    let mut outExists: bool = false;
    for mut cref in &**inComponentRefs {
        if existsVar(cref.clone(), inVariables, skipDiscrete) && !(isState(cref.clone(), inVariables)) {
            outExists = true;
            break;
        }
    }
    outExists
}

pub fn makeVar(mut cr: metamodelica::Ref<DAE::ComponentRef>) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    let mut tp: metamodelica::Ref<DAE::Type> = ComponentReference::crefLastType(&cr)?;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>> = Expression::arrayDimension(&tp);
    v = metamodelica::Ref::new(BackendDAE::Var {
        varName: cr,
        varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE,
        varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
        varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
        varType: Types::arrayElementType(&tp),
        bindExp: None,
        tplExp: None,
        arryDim: dims,
        source: DAE::emptyElementSource().clone(),
        values: None,
        tearingSelectOption: None,
        hideResult: None,
        comment: None,
        connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
        innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER,
        unreplaceable: false,
        initNonlinear: false,
        encrypted: false,
    });
    Ok(v)
}

pub(crate) fn addVarDAE(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inEqSystem: metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut outEqSystem: metamodelica::Ref<BackendDAE::EqSystem>;
    outEqSystem = BackendDAEUtil::setEqSystVars(inEqSystem.clone(), addVar(inVar, inEqSystem.orderedVars.clone())?);
    Ok(outEqSystem)
}

pub(crate) fn addVarsDAE(
    mut inVars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inEqSystem: metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut outEqSystem: metamodelica::Ref<BackendDAE::EqSystem> = inEqSystem;
    outEqSystem = List::fold(inVars, &addVarDAE, outEqSystem)?;
    Ok(outEqSystem)
}

pub(crate) fn addGlobalKnownVarDAE(
    mut inGlobalKnownVar: metamodelica::Ref<BackendDAE::Var>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<metamodelica::Ref<BackendDAE::Shared>> {
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    outShared = BackendDAEUtil::setSharedGlobalKnownVars(
        inShared.clone(),
        addVar(inGlobalKnownVar, inShared.globalKnownVars.clone())?,
    );
    Ok(outShared)
}

pub(crate) fn addNewGlobalKnownVarDAE(
    mut inGlobalKnownVar: metamodelica::Ref<BackendDAE::Var>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<metamodelica::Ref<BackendDAE::Shared>> {
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    outShared = BackendDAEUtil::setSharedGlobalKnownVars(
        inShared.clone(),
        addNewVar(inGlobalKnownVar, inShared.globalKnownVars.clone())?,
    );
    Ok(outShared)
}

pub(crate) fn addAliasVarDAE(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<metamodelica::Ref<BackendDAE::Shared>> {
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    outShared = BackendDAEUtil::setSharedAliasVars(inShared.clone(), addVar(inVar, inShared.aliasVars.clone())?);
    Ok(outShared)
}

pub(crate) fn addNewAliasVarDAE(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<metamodelica::Ref<BackendDAE::Shared>> {
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    outShared = BackendDAEUtil::setSharedAliasVars(inShared.clone(), addNewVar(inVar, inShared.aliasVars.clone())?);
    Ok(outShared)
}

pub(crate) fn addVar(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inVariables: BackendDAE::Variables,
) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables = inVariables.clone();
    let mut hash: i32;
    let mut hash_idx: i32;
    let mut arr_idx: i32;
    let mut indices: metamodelica::List<BackendDAE::CrefIndex>;
    hash = ComponentReferenceBasics::hashComponentRef(&inVar.varName)?;
    hash_idx = intMod(hash, inVariables.bucketSize.clone()) + 1;
    indices = metamodelica::arrayGet(inVariables.crefIndices.clone(), hash_idx)?;
    match '__try0: {
        arr_idx = unwrap_break_err!(findCrefIndex(&inVar.varName, &indices), '__try0);
        let true = (arr_idx >= 0) else {
            break '__try0 Err::<_, _>("pattern mismatch");
        };
        outVariables.varArr =
            unwrap_break_err!(vararraySetnth(inVariables.varArr.clone(), arr_idx + 1, inVar.clone()), '__try0);
        Ok::<_, &'static str>((outVariables.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outVariables = __try0_o0;
        }
        Err(_) => {
            if outVariables.numberOfVars.clone() >= outVariables.bucketSize.clone() {
                outVariables = growBuckets(outVariables.clone())?;
                hash_idx = intMod(hash, outVariables.bucketSize.clone()) + 1;
                indices = metamodelica::arrayGet(outVariables.crefIndices.clone(), hash_idx)?;
            }
            outVariables.varArr = vararrayAdd(outVariables.varArr.clone(), inVar.clone())?;
            metamodelica::arrayUpdate(
                outVariables.crefIndices.clone(),
                hash_idx,
                metamodelica::cons(
                    BackendDAE::CrefIndex {
                        cref: inVar.varName.clone(),
                        index: outVariables.numberOfVars.clone(),
                    },
                    indices.clone(),
                ),
            )?;
            updatePrefixIndices(
                inVar.varName.clone(),
                outVariables.numberOfVars.clone(),
                &(outVariables.clone()),
            )?;
            outVariables.numberOfVars = outVariables.numberOfVars.clone() + 1;
            if !(outVariables.hasStartVars.clone()) && ComponentReference::isStartCref(&inVar.varName) {
                outVariables.hasStartVars = true;
            }
        }
    }
    Ok(outVariables)
}

pub fn addVars(
    mut inVars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inVariables: BackendDAE::Variables,
) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    outVariables = List::fold(inVars, &addVar, inVariables)?;
    Ok(outVariables)
}

pub(crate) fn addNewVars(
    mut inVars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inVariables: BackendDAE::Variables,
) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    outVariables = List::fold(inVars, &addNewVar, inVariables)?;
    Ok(outVariables)
}

pub(crate) fn addNewVar(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inVariables: BackendDAE::Variables,
) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    let mut hashvec: metamodelica::Array<metamodelica::List<BackendDAE::CrefIndex>>;
    let mut varr: BackendDAE::VariableArray;
    let mut bsize: i32;
    let mut num_vars: i32;
    let mut idx: i32;
    let mut indices: metamodelica::List<BackendDAE::CrefIndex>;
    outVariables = if (inVariables.numberOfVars.clone() >= inVariables.bucketSize.clone()) {
        growBuckets(inVariables)?
    } else {
        inVariables
    };
    let BackendDAE::VARIABLES {
        crefIndices: __pa0,
        varArr: __pa1,
        bucketSize: __pa2,
        numberOfVars: __pa3,
        ..
    } = &outVariables;
    hashvec = metamodelica::Own::own(__pa0);
    varr = metamodelica::Own::own(__pa1);
    bsize = metamodelica::Own::own(__pa2);
    num_vars = metamodelica::Own::own(__pa3);
    idx = intMod(ComponentReferenceBasics::hashComponentRef(&inVar.varName)?, bsize) + 1;
    varr = vararrayAdd(varr, inVar.clone())?;
    indices = ({
        let __elt = (*metamodelica::index_checked(&hashvec.borrow(), idx)?).clone();
        __elt
    });
    metamodelica::arrayUpdate(
        hashvec.clone(),
        idx,
        metamodelica::cons(
            BackendDAE::CrefIndex {
                cref: inVar.varName.clone(),
                index: num_vars,
            },
            indices,
        ),
    )?;
    updatePrefixIndices(inVar.varName.clone(), num_vars, &outVariables)?;
    outVariables.varArr = varr;
    outVariables.numberOfVars = num_vars + 1;
    if !(outVariables.hasStartVars.clone()) && ComponentReference::isStartCref(&inVar.varName) {
        outVariables.hasStartVars = true;
    }
    Ok(outVariables)
}

pub(crate) fn addVariables(
    mut inSrcVars: BackendDAE::Variables,
    mut inDestVars: BackendDAE::Variables,
) -> Result<BackendDAE::Variables> {
    let mut outVars: BackendDAE::Variables = inDestVars;
    let mut vars: metamodelica::Array<Option<metamodelica::Ref<BackendDAE::Var>>>;
    let mut num_vars: i32;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut ovar: Option<metamodelica::Ref<BackendDAE::Var>>;
    let BackendDAE::VARIABLES {
        varArr:
            BackendDAE::VARIABLE_ARRAY {
                numberOfElements: __pa0,
                varOptArr: __pa1,
            },
        ..
    } = inSrcVars;
    num_vars = metamodelica::Own::own(__pa0);
    vars = metamodelica::Own::own(__pa1);
    for mut i in 1..=num_vars {
        ovar = ({
            let __elt = (*metamodelica::index_checked(&vars.borrow(), i)?).clone();
            __elt
        });
        if (ovar).is_some() {
            let __pa2 = ::match_deref::match_deref! { match &(ovar) {
                Some(__pa2) => __pa2.clone(),
                _ => return Err("pattern mismatch"),
            } };
            var = metamodelica::Own::own(__pa2);
            outVars = addVar(var, outVars)?;
        }
    }
    Ok(outVars)
}

pub fn getVarAt(
    mut inVariables: &BackendDAE::Variables,
    mut inIndex: i32,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    outVar = vararrayNth(&inVariables.varArr, inIndex)?;
    Ok(outVar)
}

pub(crate) fn setVarAt(
    mut inVariables: BackendDAE::Variables,
    mut inIndex: i32,
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables = inVariables.clone();
    vararraySetnth(inVariables.varArr.clone(), inIndex, inVar)?;
    Ok(outVariables)
}

pub(crate) fn getVarAtIndexFirst(
    mut inIndex: i32,
    mut inVariables: &BackendDAE::Variables,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    outVar = getVarAt(inVariables, inIndex)?;
    Ok(outVar)
}

pub(crate) fn getVarSharedAt(
    mut inInteger: i32,
    mut inShared: &metamodelica::Ref<BackendDAE::Shared>,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    outVar = getVarAt(&inShared.globalKnownVars, inInteger)?;
    Ok(outVar)
}

pub(crate) fn getVarDAE(
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut inEqSystem: &metamodelica::Ref<BackendDAE::EqSystem>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<i32>,
)> {
    let mut outVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut outIntegerLst: metamodelica::List<i32>;
    (outVarLst, outIntegerLst) = getVar(inComponentRef, &inEqSystem.orderedVars)?;
    Ok((outVarLst, outIntegerLst))
}

pub(crate) fn getVarShared(
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut inShared: &metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<i32>,
)> {
    let mut outVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut outIntegerLst: metamodelica::List<i32>;
    (outVarLst, outIntegerLst) = getVar(inComponentRef, &inShared.globalKnownVars)?;
    Ok((outVarLst, outIntegerLst))
}

pub(crate) fn containsVar(
    mut var: &metamodelica::Ref<BackendDAE::Var>,
    mut inVariables: &BackendDAE::Variables,
) -> bool {
    let mut outB: bool = containsCref(var.varName.clone(), inVariables);
    outB
}

pub(crate) fn containsCref(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut inVariables: &BackendDAE::Variables,
) -> bool {
    let mut outB: bool;
    match '__try0: {
        unwrap_break_err!(getVar(cr.clone(), inVariables), '__try0);
        outB = true;
        Ok::<_, &'static str>((outB.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outB = __try0_o0;
        }
        Err(_) => {
            outB = false;
        }
    }
    outB
}

pub fn getVar(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut inVariables: &BackendDAE::Variables,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<i32>,
)> {
    let mut outVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut outIntegerLst: metamodelica::List<i32> = metamodelica::nil();
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    let mut hash: i32;
    let mut indx: i32;
    let mut depth: i32;
    let mut found: bool;
    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
    let mut indices: metamodelica::List<i32>;
    let mut live: metamodelica::List<i32> = metamodelica::nil();
    let mut nsubs: i32;
    let mut bucket: i32;
    let mut died: bool = false;
    let mut var_opt: Option<metamodelica::Ref<BackendDAE::Var>>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    hash = ComponentReferenceBasics::hashComponentRef(&cr)?;
    match '__try0: {
        (v, indx) = unwrap_break_err!(getVarHashed(&cr, hash, inVariables), '__try0);
        outVarLst = list![v.clone()];
        outIntegerLst = if (true/* isPresent not implemented in Rust */) {
            list![indx]
        } else {
            metamodelica::nil()
        };
        found = true;
        Ok::<_, &'static str>((found.clone(),))
    } {
        Ok((__try0_o0,)) => {
            found = __try0_o0;
        }
        Err(_) => {
            found = false;
        }
    }
    if found {
        return Ok((outVarLst, outIntegerLst));
    }
    if isPrefixQuery(&cr)? {
        (indices, depth, nsubs, bucket) = getPrefixIndices(cr.clone(), hash, inVariables)?;
        (ty, dims) = TypesDump::flattenArrayType(&(ComponentReference::crefLastType(&cr)?));
        for mut i in &*indices.reverse() {
            var_opt = metamodelica::arrayGet(inVariables.varArr.varOptArr.clone(), i.clone() + 1)?;
            if (var_opt).is_some() {
                live = metamodelica::cons(i.clone(), live);
                let __pa1 = ::match_deref::match_deref! { match &(var_opt) {
                    Some(__pa1) => __pa1.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                v = metamodelica::Own::own(__pa1);
                if isElementOf(v.varName.clone(), depth, ty.clone(), ((dims).len() as i32))? {
                    outVarLst = metamodelica::cons(v, outVarLst);
                    outIntegerLst = metamodelica::cons(i.clone() + 1, outIntegerLst);
                }
            } else {
                died = true;
            }
        }
        if died {
            setPrefixIndices(cr, depth, nsubs, bucket, live, inVariables)?;
        }
        if (outVarLst).is_empty() {
            return Err("fail");
        }
        return Ok((outVarLst, outIntegerLst));
    }
    if isScalarQuery(&cr)? {
        return Err("fail");
    }
    match '__try2: {
        crlst = unwrap_break_err!(ComponentReference::expandCref(&cr, true), '__try2);
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(getVarLst(&crlst, inVariables)) {
            (__pa3 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, __pa4) => (__pa3.clone(), __pa4.clone()),
            _ => break '__try2 Err::<_, _>("pattern mismatch"),
        } };
        outVarLst = metamodelica::Own::own(__pa3);
        outIntegerLst = metamodelica::Own::own(__pa4);
        Ok::<_, &'static str>((crlst.clone(), outIntegerLst.clone(), outVarLst.clone()))
    } {
        Ok((__try2_o0, __try2_o1, __try2_o2)) => {
            crlst = __try2_o0;
            outIntegerLst = __try2_o1;
            outVarLst = __try2_o2;
        }
        Err(_) => {
            let __pa5 = ::match_deref::match_deref! { match &(replaceVarWithWholeDim(&cr, false)?) {
                (__pa5, true) => __pa5.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr1 = metamodelica::Own::own(__pa5);
            crlst = ComponentReference::expandCref(&cr1, true)?;
            let (__pa6, __pa7) = ::match_deref::match_deref! { match &(getVarLst(&crlst, inVariables)) {
                (__pa6 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, __pa7) => (__pa6.clone(), __pa7.clone()),
                _ => return Err("pattern mismatch"),
            } };
            outVarLst = metamodelica::Own::own(__pa6);
            outIntegerLst = metamodelica::Own::own(__pa7);
        }
    }
    Ok((outVarLst, outIntegerLst))
}

fn isPrefixQuery(mut cr: &metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> {
    let mut b: bool;
    b = (match &**cr {
        DAE::ComponentRef::CREF_IDENT {
            identType: __cr_identType,
            subscriptLst: __cr_subscriptLst,
            ..
        } => {
            List::all(
                metamodelica::AsArg::as_arg(&__cr_subscriptLst),
                &move |__a0: metamodelica::Ref<DAE::Subscript>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(isIntSubscript(&__a0))
                },
            )? && isExpandableType(metamodelica::AsArg::as_arg(&__cr_identType))
        }
        DAE::ComponentRef::CREF_QUAL {
            componentRef: __cr_componentRef,
            identType: __cr_identType,
            subscriptLst: __cr_subscriptLst,
            ..
        } => {
            List::all(
                metamodelica::AsArg::as_arg(&__cr_subscriptLst),
                &move |__a0: metamodelica::Ref<DAE::Subscript>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(isIntSubscript(&__a0))
                },
            )? && ((__cr_subscriptLst).len() as i32)
                >= Types::numberOfDimensions(metamodelica::AsArg::as_arg(&__cr_identType))
                && isPrefixQuery(metamodelica::AsArg::as_arg(&__cr_componentRef))?
        }
        _ => false,
    });
    Ok(b)
}

fn isExpandableType(mut ty: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    b = (match &**ty {
        DAE::Type::T_ARRAY { .. } => true,
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::RECORD { .. },
            ..
        } => true,
        _ => false,
    });
    b
}

fn isScalarQuery(mut cr: &metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> {
    let mut b: bool;
    b = (match &**cr {
        DAE::ComponentRef::CREF_IDENT {
            identType: __cr_identType,
            subscriptLst: __cr_subscriptLst,
            ..
        } => {
            List::all(
                metamodelica::AsArg::as_arg(&__cr_subscriptLst),
                &move |__a0: metamodelica::Ref<DAE::Subscript>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(isIntSubscript(&__a0))
                },
            )? && ((__cr_subscriptLst).len() as i32)
                >= Types::numberOfDimensions(metamodelica::AsArg::as_arg(&__cr_identType))
                && !(Types::isRecord(&(Types::arrayElementType(metamodelica::AsArg::as_arg(&__cr_identType)))))
        }
        DAE::ComponentRef::CREF_QUAL {
            componentRef: __cr_componentRef,
            identType: __cr_identType,
            subscriptLst: __cr_subscriptLst,
            ..
        } => {
            List::all(
                metamodelica::AsArg::as_arg(&__cr_subscriptLst),
                &move |__a0: metamodelica::Ref<DAE::Subscript>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(isIntSubscript(&__a0))
                },
            )? && ((__cr_subscriptLst).len() as i32)
                >= Types::numberOfDimensions(metamodelica::AsArg::as_arg(&__cr_identType))
                && isScalarQuery(metamodelica::AsArg::as_arg(&__cr_componentRef))?
        }
        _ => false,
    });
    Ok(b)
}

fn isElementOf(
    mut var: metamodelica::Ref<DAE::ComponentRef>,
    mut depth: i32,
    mut ty: metamodelica::Ref<DAE::Type>,
    mut ndims: i32,
) -> Result<bool> {
    let mut b: bool = false;
    let mut v: metamodelica::Ref<DAE::ComponentRef> = var;
    let mut t: metamodelica::Ref<DAE::Type> = ty;
    let mut n: i32 = ndims;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut fty: Option<metamodelica::Ref<DAE::Type>>;
    for mut i in 2..=depth {
        v = ComponentReference::crefRest(&v)?;
    }
    while ((ComponentReference::crefFirstSubs(&v)).len() as i32) == n {
        if ComponentReference::crefIsIdent(&v) {
            b = true;
            return Ok(b);
        }
        v = ComponentReference::crefRest(&v)?;
        fty = recordFieldType(&t, &(ComponentReferenceBasics::crefFirstIdent(&v)?));
        if (fty).is_none() {
            return Ok(b);
        }
        (t, dims) = TypesDump::flattenArrayType(&(fty.ok_or("pattern mismatch")?));
        n = ((dims).len() as i32);
    }
    Ok(b)
}

fn recordFieldType(mut ty: &metamodelica::Ref<DAE::Type>, mut name: &ArcStr) -> Option<metamodelica::Ref<DAE::Type>> {
    let mut fty: Option<metamodelica::Ref<DAE::Type>> = None;
    fty = (match &**ty {
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::RECORD { .. },
            varLst: __ty_varLst,
            ..
        } => {
            for mut f in &*__ty_varLst.clone() {
                if metamodelica::stringEq(&f.name, &name) {
                    fty = Some(f.ty.clone());
                    break;
                }
            }
            fty
        }
        _ => None,
    });
    fty
}

fn isIntSubscript(mut sub: &metamodelica::Ref<DAE::Subscript>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match sub {
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn getPrefixIndices(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut hash: i32,
    mut vars: &BackendDAE::Variables,
) -> Result<(metamodelica::List<i32>, i32, i32, i32)> {
    let mut indices: metamodelica::List<i32>;
    let mut depth: i32 = 0;
    let mut nsubs: i32 = 0;
    let mut bucket: i32;
    let mut c: metamodelica::Ref<DAE::ComponentRef> = cr.clone();
    let mut last: bool = false;
    while !(last) {
        (nsubs, last, c) = (match &*c {
            DAE::ComponentRef::CREF_IDENT {
                subscriptLst: __c_subscriptLst,
                ..
            } => (((__c_subscriptLst).len() as i32), true, c),
            DAE::ComponentRef::CREF_QUAL {
                componentRef: __c_componentRef,
                ..
            } => (0, false, __c_componentRef.clone()),
            _ => return Err("match: no arm matched"),
        });
        depth = depth + 1;
    }
    bucket = intMod(hash, vars.bucketSize.clone()) + 1;
    for mut e in &*metamodelica::arrayGet(vars.prefixIndices.clone(), bucket)? {
        if e.depth.clone() == depth
            && e.numSubscripts.clone() == nsubs
            && prefixEqual(e.cref.clone(), cr.clone(), depth, nsubs)?
        {
            indices = e.indices.clone();
            return Ok((indices, depth, nsubs, bucket.clone()));
        }
    }
    return Err("fail");
    Ok((indices, depth, nsubs, bucket))
}

fn setPrefixIndices(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut depth: i32,
    mut nsubs: i32,
    mut bucket: i32,
    mut indices: metamodelica::List<i32>,
    mut vars: &BackendDAE::Variables,
) -> Result<()> {
    let mut entries: metamodelica::List<BackendDAE::PrefixIndex> =
        metamodelica::arrayGet(vars.prefixIndices.clone(), bucket)?;
    let mut acc: metamodelica::List<BackendDAE::PrefixIndex> = metamodelica::nil();
    let mut e: BackendDAE::PrefixIndex;
    while !((entries).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(entries) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        entries = metamodelica::Own::own(__pa1);
        if e.depth.clone() == depth
            && e.numSubscripts.clone() == nsubs
            && prefixEqual(e.cref.clone(), cr.clone(), depth, nsubs)?
        {
            e.indices = indices.clone();
            metamodelica::arrayUpdate(
                vars.prefixIndices.clone(),
                bucket,
                List::append_reverse(
                    &acc,
                    if ((indices).is_empty()) {
                        entries
                    } else {
                        metamodelica::cons(e, entries)
                    },
                ),
            )?;
            return Ok(());
        }
        acc = metamodelica::cons(e, acc);
    }
    Ok(())
}

fn updatePrefixIndices(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut index: i32,
    mut vars: &BackendDAE::Variables,
) -> Result<()> {
    let mut c: metamodelica::Ref<DAE::ComponentRef> = cr.clone();
    let mut hash: i32 = ComponentReferenceBasics::crefHashSeed.clone();
    let mut depth: i32 = 0;
    let mut nsubs: i32;
    let mut count: i32;
    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut last: bool;
    let mut ok: bool;
    let mut store: bool;
    loop {
        (subs, ty, last, ok) = (match &*c {
            DAE::ComponentRef::CREF_IDENT {
                identType: __c_identType,
                subscriptLst: __c_subscriptLst,
                ..
            } => (__c_subscriptLst.clone(), __c_identType.clone(), true, true),
            DAE::ComponentRef::CREF_QUAL {
                identType: __c_identType,
                subscriptLst: __c_subscriptLst,
                ..
            } => (__c_subscriptLst.clone(), __c_identType.clone(), false, true),
            _ => (metamodelica::nil(), DAE::T_UNKNOWN_DEFAULT().clone(), true, false),
        });
        if !(ok) {
            return Ok(());
        }
        depth = depth + 1;
        hash = ComponentReferenceBasics::crefHashIdent(&(ComponentReferenceBasics::crefFirstIdent(&c)?), hash);
        count = ((subs).len() as i32);
        nsubs = 0;
        store = isExpandableType(&ty);
        if store && !(last && count == 0) {
            updatePrefixIndex(cr.clone(), depth, nsubs, hash, index, vars)?;
        }
        for mut sub in &*subs {
            hash = ComponentReferenceBasics::crefHashSubscript(metamodelica::AsArg::as_arg(&sub), hash)?;
            nsubs = nsubs + 1;
            if store && !(last && nsubs == count) {
                updatePrefixIndex(cr.clone(), depth, nsubs, hash, index, vars)?;
            }
        }
        if last {
            return Ok(());
        }
        c = (match &*c {
            DAE::ComponentRef::CREF_QUAL {
                componentRef: __c_componentRef,
                ..
            } => __c_componentRef.clone(),
            _ => return Err("match: no arm matched"),
        });
    }
    Ok(())
}

fn updatePrefixIndex(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut depth: i32,
    mut nsubs: i32,
    mut hash: i32,
    mut index: i32,
    mut vars: &BackendDAE::Variables,
) -> Result<()> {
    let mut b: i32 = intMod(hash, vars.bucketSize.clone()) + 1;
    let mut entries: metamodelica::List<BackendDAE::PrefixIndex> =
        metamodelica::arrayGet(vars.prefixIndices.clone(), b)?;
    let mut acc: metamodelica::List<BackendDAE::PrefixIndex> = metamodelica::nil();
    let mut e: BackendDAE::PrefixIndex;
    while !((entries).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(entries) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        entries = metamodelica::Own::own(__pa1);
        if e.depth.clone() == depth
            && e.numSubscripts.clone() == nsubs
            && prefixEqual(e.cref.clone(), cr.clone(), depth, nsubs)?
        {
            e.indices = metamodelica::cons(index, e.indices.clone());
            metamodelica::arrayUpdate(
                vars.prefixIndices.clone(),
                b,
                List::append_reverse(&acc, metamodelica::cons(e, entries)),
            )?;
            return Ok(());
        }
        acc = metamodelica::cons(e, acc);
    }
    metamodelica::arrayUpdate(
        vars.prefixIndices.clone(),
        b,
        metamodelica::cons(
            BackendDAE::PrefixIndex {
                cref: cr,
                depth: depth,
                numSubscripts: nsubs,
                indices: list![index],
            },
            metamodelica::arrayGet(vars.prefixIndices.clone(), b)?,
        ),
    )?;
    Ok(())
}

fn prefixEqual(
    mut cr1: metamodelica::Ref<DAE::ComponentRef>,
    mut cr2: metamodelica::Ref<DAE::ComponentRef>,
    mut depth: i32,
    mut nsubs: i32,
) -> Result<bool> {
    let mut equal: bool = false;
    let mut c1: metamodelica::Ref<DAE::ComponentRef> = cr1;
    let mut c2: metamodelica::Ref<DAE::ComponentRef> = cr2;
    let mut s1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let mut s2: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let mut sub1: metamodelica::Ref<DAE::Subscript>;
    let mut sub2: metamodelica::Ref<DAE::Subscript>;
    for mut i in 1..=depth - 1 {
        if !(metamodelica::stringEq(
            &(ComponentReferenceBasics::crefFirstIdent(&c1)?),
            &(ComponentReferenceBasics::crefFirstIdent(&c2)?),
        ) && ExpressionBasics::subscriptEqual(
            &(ComponentReference::crefFirstSubs(&c1)),
            &(ComponentReference::crefFirstSubs(&c2)),
        )?) {
            return Ok(equal);
        }
        c1 = ComponentReference::crefRest(&c1)?;
        c2 = ComponentReference::crefRest(&c2)?;
    }
    if !metamodelica::stringEq(
        &(ComponentReferenceBasics::crefFirstIdent(&c1)?),
        &(ComponentReferenceBasics::crefFirstIdent(&c2)?),
    ) {
        return Ok(equal);
    }
    s1 = ComponentReference::crefFirstSubs(&c1);
    s2 = ComponentReference::crefFirstSubs(&c2);
    for mut i in 1..=nsubs {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(s1) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        sub1 = metamodelica::Own::own(__pa0);
        s1 = metamodelica::Own::own(__pa1);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(s2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        sub2 = metamodelica::Own::own(__pa2);
        s2 = metamodelica::Own::own(__pa3);
        if !(subscriptEq(&sub1, &sub2)?) {
            return Ok(equal);
        }
    }
    equal = true;
    Ok(equal)
}

fn subscriptEq(
    mut sub1: &metamodelica::Ref<DAE::Subscript>,
    mut sub2: &metamodelica::Ref<DAE::Subscript>,
) -> Result<bool> {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match (sub1, sub2) {
        (Deref @ DAE::Subscript::WHOLEDIM { .. }, Deref @ DAE::Subscript::WHOLEDIM { .. }) => true,
        (Deref @ DAE::Subscript::INDEX { .. }, Deref @ DAE::Subscript::INDEX { .. }) => ExpressionBasics::expEqual(var_field!((**sub1).exp, DAE::Subscript::INDEX), var_field!((**sub2).exp, DAE::Subscript::INDEX).clone())?,
        (Deref @ DAE::Subscript::SLICE { .. }, Deref @ DAE::Subscript::SLICE { .. }) => ExpressionBasics::expEqual(var_field!((**sub1).exp, DAE::Subscript::SLICE), var_field!((**sub2).exp, DAE::Subscript::SLICE).clone())?,
        (Deref @ DAE::Subscript::WHOLE_NONEXP { .. }, Deref @ DAE::Subscript::WHOLE_NONEXP { .. }) => ExpressionBasics::expEqual(var_field!((**sub1).exp, DAE::Subscript::WHOLE_NONEXP), var_field!((**sub2).exp, DAE::Subscript::WHOLE_NONEXP).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

pub(crate) fn getVarSingle(
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut inVariables: &BackendDAE::Variables,
) -> Result<(metamodelica::Ref<BackendDAE::Var>, i32)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outInteger: i32 = 0;
    (outVar, outInteger) = 'mc: {
        let __mc_input = inVariables.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut v: metamodelica::Ref<BackendDAE::Var>;
            let mut indx: i32;
            (v, indx) = getVar2(cr, inVariables)?;
            Ok((v.clone(), indx))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut v: metamodelica::Ref<BackendDAE::Var>;
            let mut indx: i32;
            let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            crlst = ComponentReference::expandCref(cr, true)?;
            if true
            /* isPresent not implemented in Rust */
            {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getVarLst(&crlst, inVariables)) {
                    (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil }) => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                v = metamodelica::Own::own(__pa0);
                indx = metamodelica::Own::own(__pa1);
            } else {
                let __pa4 = ::match_deref::match_deref! { match &(getVarLst(&crlst, inVariables)) {
                    (Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Nil }, _) => __pa4.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                v = metamodelica::Own::own(__pa4);
                indx = 0;
            }
            Ok((v.clone(), indx))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut v: metamodelica::Ref<BackendDAE::Var>;
            let mut indx: i32;
            let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
            let __pa0 = ::match_deref::match_deref! { match &(replaceVarWithWholeDim(cr, false)?) {
                (__pa0, true) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr1 = metamodelica::Own::own(__pa0);
            crlst = ComponentReference::expandCref(&cr1, true)?;
            if true
            /* isPresent not implemented in Rust */
            {
                let (__pa1, __pa2) = ::match_deref::match_deref! { match &(getVarLst(&crlst, inVariables)) {
                    (Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil }) => (__pa1.clone(), __pa2.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                v = metamodelica::Own::own(__pa1);
                indx = metamodelica::Own::own(__pa2);
            } else {
                let __pa5 = ::match_deref::match_deref! { match &(getVarLst(&crlst, inVariables)) {
                    (Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Nil }, _) => __pa5.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                v = metamodelica::Own::own(__pa5);
                indx = 0;
            }
            Ok((v.clone(), indx))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outVar, outInteger))
}

pub(crate) fn getVarTryHard(
    mut cref: metamodelica::Ref<DAE::ComponentRef>,
    mut vars: &BackendDAE::Variables,
) -> Option<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut var_lst_opt: Option<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut strippedCref: metamodelica::Ref<DAE::ComponentRef>;
    match '__try0: {
        (var, _) = unwrap_break_err!(getVarSingle(&cref, vars), '__try0);
        var_lst_opt = Some(list![var.clone()]);
        Ok::<_, &'static str>((var_lst_opt.clone(),))
    } {
        Ok((__try0_o0,)) => {
            var_lst_opt = __try0_o0;
        }
        Err(_) => {
            match '__try1: {
                (var_lst, _) = unwrap_break_err!(getVar(cref.clone(), vars), '__try1);
                var_lst_opt = Some(var_lst.clone());
                Ok::<_, &'static str>((var_lst_opt.clone(),))
            } {
                Ok((__try1_o0,)) => {
                    var_lst_opt = __try1_o0;
                }
                Err(_) => {
                    match '__try2: {
                        strippedCref = ComponentReference::crefStripSubsExceptModelSubs(cref.clone());
                        (var, _) = unwrap_break_err!(getVarSingle(&strippedCref, vars), '__try2);
                        var_lst_opt = Some(list![var.clone()]);
                        Ok::<_, &'static str>((var_lst_opt.clone(),))
                    } {
                        Ok((__try2_o0,)) => {
                            var_lst_opt = __try2_o0;
                        }
                        Err(_) => {
                            var_lst_opt = None;
                        }
                    }
                }
            }
        }
    }
    var_lst_opt
}

fn replaceVarWithWholeDim(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut iPerformed: bool,
) -> Result<(metamodelica::Ref<DAE::ComponentRef>, bool)> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut oPerformed: bool;
    (outCref, oPerformed) = (match &**inCref {
        DAE::ComponentRef::CREF_QUAL {
            ident: name,
            identType: ty,
            subscriptLst: subs,
            componentRef: cr,
        } => {
            let mut cr_1: metamodelica::Ref<DAE::ComponentRef>;
            let mut subs_1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            let mut b: bool;
            (subs_1, b) = replaceVarWithWholeDimSubs(subs, iPerformed)?;
            (cr_1, b) = replaceVarWithWholeDim(cr, b)?;
            (
                if (metamodelica::ReferenceEq::reference_eq(&(subs_1), &(subs.clone()))
                    && referenceEq(&*(&*cr_1), &*(cr.clone())))
                {
                    inCref.clone()
                } else {
                    metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
                        ident: name.clone(),
                        identType: ty.clone(),
                        subscriptLst: subs_1,
                        componentRef: cr_1,
                    })
                },
                b,
            )
        }
        DAE::ComponentRef::CREF_IDENT {
            ident: name,
            identType: ty,
            subscriptLst: subs,
        } => {
            let mut subs_1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            let mut b: bool;
            (subs_1, b) = replaceVarWithWholeDimSubs(subs, iPerformed)?;
            (
                if (metamodelica::ReferenceEq::reference_eq(&(subs_1), &(subs.clone()))) {
                    inCref.clone()
                } else {
                    metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                        ident: name.clone(),
                        identType: ty.clone(),
                        subscriptLst: subs_1,
                    })
                },
                b,
            )
        }
        DAE::ComponentRef::OPTIMICA_ATTR_INST_CREF { .. } => (inCref.clone(), iPerformed),
        DAE::ComponentRef::WILD { .. } => (inCref.clone(), iPerformed),
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!("BackendVariable.replaceVarWithWholeDim: Unknown cref")],
            )?;
            return Err("fail");
        }
    });
    Ok((outCref, oPerformed))
}

fn replaceVarWithWholeDimSubs(
    mut inSubscript: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut iPerformed: bool,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Subscript>>, bool)> {
    let mut outSubscript: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let mut oPerformed: bool;
    (outSubscript, oPerformed) = (::match_deref::match_deref! { match inSubscript {
        Deref @ metamodelica::ListNode::Nil => {
            (inSubscript.clone(), iPerformed)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: rest } => {
            let mut b: bool;
            (_, b) = replaceVarWithWholeDimSubs(rest, iPerformed)?;
            (metamodelica::cons(openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(), rest.clone()), b)
        },
        Deref @ metamodelica::ListNode::Cons { head: sub @ Deref @ DAE::Subscript::SLICE { exp: sub_exp }, tail: rest } => {
            let mut res: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            let mut b: bool;
            let mut r#const: bool;
            (res, b) = replaceVarWithWholeDimSubs(rest, iPerformed)?;
            r#const = Expression::isConst(sub_exp.clone())?;
            res = if (r#const) {metamodelica::cons(sub.clone(), rest.clone())} else {metamodelica::cons(openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(), rest.clone())};
            (res, b || !(r#const))
        },
        Deref @ metamodelica::ListNode::Cons { head: sub @ Deref @ DAE::Subscript::INDEX { exp: sub_exp }, tail: rest } => {
            let mut sub_exp_: metamodelica::Ref<DAE::Exp>;
            let mut res: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            let mut b: bool;
            let mut r#const: bool;
            let mut calcRange: bool;
            (sub_exp_, calcRange) = computeRangeExps(sub_exp.clone());
            (res, b) = replaceVarWithWholeDimSubs(rest, iPerformed)?;
            r#const = Expression::isConst(sub_exp_.clone())?;
            res = metamodelica::cons(if (r#const) {if (referenceEq(&*(sub_exp.clone()),&*(&*sub_exp_))) {sub.clone()} else {metamodelica::Ref::new(DAE::Subscript::INDEX { exp: sub_exp_ })}} else {openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM()}, rest.clone());
            (res, b || !(r#const) || calcRange)
        },
        Deref @ metamodelica::ListNode::Cons { head: sub @ Deref @ DAE::Subscript::WHOLE_NONEXP { exp: sub_exp }, tail: rest } => {
            let mut res: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            let mut b: bool;
            let mut r#const: bool;
            (res, b) = replaceVarWithWholeDimSubs(rest, iPerformed)?;
            r#const = Expression::isConst(sub_exp.clone())?;
            res = if (r#const) {metamodelica::cons(sub.clone(), rest.clone())} else {metamodelica::cons(openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(), rest.clone())};
            (res, b || !(r#const))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outSubscript, oPerformed))
}

fn computeRangeExps(mut inExp: metamodelica::Ref<DAE::Exp>) -> (metamodelica::Ref<DAE::Exp>, bool) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut isCalculated: bool;
    (outExp, isCalculated) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::RANGE { ty, start: Deref @ DAE::Exp::ICONST { integer: 1 }, stop: Deref @ DAE::Exp::ICONST { integer: stop1 }, .. }, operator: DAE::Operator::ADD { .. }, exp2: Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::ICONST { integer: 1 }, stop: Deref @ DAE::Exp::ICONST { integer: stop2 }, .. } } => {
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut stop2 = (*stop2).clone();
            stop2 = stop1.clone() + stop2.clone();
            exp = metamodelica::Ref::new(DAE::Exp::RANGE { ty: ty.clone(), start: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }), step: None, stop: metamodelica::Ref::new(DAE::Exp::ICONST { integer: stop2.clone() }) });
            (exp, true)
        },
        _ => {
            (inExp, false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, isCalculated)
}

pub(crate) fn getVarLst(
    mut inComponentRefLst: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inVariables: &BackendDAE::Variables,
) -> (
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<i32>,
) {
    let mut outVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut outIntegerLst: metamodelica::List<i32> = metamodelica::nil();
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    let mut indx: i32;
    if true
    /* isPresent not implemented in Rust */
    {
        for mut cr in &**inComponentRefLst {
            if '__try0: {
                (v, indx) = unwrap_break_err!(getVar2(metamodelica::AsArg::as_arg(&cr), inVariables), '__try0);
                outVarLst = metamodelica::cons(v.clone(), outVarLst.clone());
                outIntegerLst = metamodelica::cons(indx, outIntegerLst.clone());
                Ok::<(), &'static str>(())
            }
            .is_err()
            {}
        }
    } else {
        for mut cr in &**inComponentRefLst {
            if '__try1: {
                (v, indx) = unwrap_break_err!(getVar2(metamodelica::AsArg::as_arg(&cr), inVariables), '__try1);
                outVarLst = metamodelica::cons(v.clone(), outVarLst.clone());
                Ok::<(), &'static str>(())
            }
            .is_err()
            {}
        }
    }
    (outVarLst, outIntegerLst)
}

pub(crate) fn getVar2(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inVariables: &BackendDAE::Variables,
) -> Result<(metamodelica::Ref<BackendDAE::Var>, i32)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outIndex: i32;
    (outVar, outIndex) = getVarHashed(inCref, ComponentReferenceBasics::hashComponentRef(inCref)?, inVariables)?;
    Ok((outVar, outIndex))
}

fn getVarHashed(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut hash: i32,
    mut inVariables: &BackendDAE::Variables,
) -> Result<(metamodelica::Ref<BackendDAE::Var>, i32)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outIndex: i32;
    outIndex = findCrefIndex(
        inCref,
        &(metamodelica::arrayGet(
            inVariables.crefIndices.clone(),
            intMod(hash, inVariables.bucketSize.clone()) + 1,
        )?),
    )?;
    let true = (outIndex >= 0) else {
        return Err("pattern mismatch");
    };
    outIndex = outIndex + 1;
    outVar = vararrayNth(&inVariables.varArr, outIndex)?;
    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(&outVar.varName, inCref)?) else {
        return Err("pattern mismatch");
    };
    Ok((outVar, outIndex))
}

fn findCrefIndex(
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut indices: &metamodelica::List<BackendDAE::CrefIndex>,
) -> Result<i32> {
    let mut index: i32 = -1;
    for mut ci in &**indices {
        if ComponentReferenceBasics::crefEqualNoStringCompare(&ci.cref, cr)? {
            index = ci.index.clone();
            return Ok(index);
        }
    }
    Ok(index)
}

pub(crate) fn getVarIndexFromVars(
    mut inVars: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inVariables: &BackendDAE::Variables,
) -> metamodelica::List<i32> {
    let mut outIndices: metamodelica::List<i32> = metamodelica::nil();
    for mut var in &**inVars {
        (_, outIndices) = traversingVarIndexFinder(var.clone(), inVariables, outIndices);
    }
    outIndices = outIndices.reverse();
    outIndices
}

pub(crate) fn getVarIndexFromVariables(
    mut inVariables: BackendDAE::Variables,
    mut inVariables2: &BackendDAE::Variables,
) -> Result<metamodelica::List<i32>> {
    let mut v_lst: metamodelica::List<i32>;
    v_lst = traverseBackendDAEVars(
        inVariables,
        (std::sync::Arc::new({
            let __pe_b1 = inVariables2.clone();
            move |__pe_a0, __pe_a2| Ok(traversingVarIndexFinder(__pe_a0, &__pe_b1, __pe_a2))
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        metamodelica::List<i32>,
                    )
                        -> Result<(metamodelica::Ref<BackendDAE::Var>, metamodelica::List<i32>)>
                    + 'static,
            >),
        metamodelica::nil(),
    )?
    .reverse();
    Ok(v_lst)
}

fn traversingVarIndexFinder(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inVars: &BackendDAE::Variables,
    mut inIndices: metamodelica::List<i32>,
) -> (metamodelica::Ref<BackendDAE::Var>, metamodelica::List<i32>) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
    let mut outIndices: metamodelica::List<i32>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut indices: metamodelica::List<i32>;
    match '__try0: {
        cr = varCref(&inVar);
        (_, indices) = unwrap_break_err!(getVar(cr.clone(), inVars), '__try0);
        outIndices = List::append_reverse(&indices, inIndices.clone());
        Ok::<_, &'static str>((outIndices.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outIndices = __try0_o0;
        }
        Err(_) => {
            outIndices = inIndices.clone();
        }
    }
    (outVar, outIndices)
}

pub(crate) fn getVarIndexFromVariablesIndexInFirstSet(
    mut inVariables: BackendDAE::Variables,
    mut inVariables2: BackendDAE::Variables,
) -> Result<metamodelica::List<i32>> {
    let mut v_lst: metamodelica::List<i32>;
    let mut a: Mutable::Mutable<metamodelica::List<i32>>;
    (_, a, _) = traverseBackendDAEVars(
        inVariables,
        (std::sync::Arc::new(fnptr!(
            traversingVarIndexInFirstSetFinder,
            metamodelica::Ref<BackendDAE::Var>,
            (
                BackendDAE::Variables,
                Mutable::Mutable<metamodelica::List<i32>>,
                Mutable::Mutable<i32>
            )
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        (
                            BackendDAE::Variables,
                            Mutable::Mutable<metamodelica::List<i32>>,
                            Mutable::Mutable<i32>,
                        ),
                    ) -> Result<(
                        metamodelica::Ref<BackendDAE::Var>,
                        (
                            BackendDAE::Variables,
                            Mutable::Mutable<metamodelica::List<i32>>,
                            Mutable::Mutable<i32>,
                        ),
                    )> + 'static,
            >),
        (inVariables2, Mutable::create(metamodelica::nil()), Mutable::create(1)),
    )?;
    v_lst = Mutable::access(a).reverse();
    Ok(v_lst)
}

fn traversingVarIndexInFirstSetFinder(
    mut var: metamodelica::Ref<BackendDAE::Var>,
    mut data: (
        BackendDAE::Variables,
        Mutable::Mutable<metamodelica::List<i32>>,
        Mutable::Mutable<i32>,
    ),
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    (
        BackendDAE::Variables,
        Mutable::Mutable<metamodelica::List<i32>>,
        Mutable::Mutable<i32>,
    ),
) {
    let mut var: metamodelica::Ref<BackendDAE::Var> = var;
    let mut data: (
        BackendDAE::Variables,
        Mutable::Mutable<metamodelica::List<i32>>,
        Mutable::Mutable<i32>,
    ) = data;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut vars: BackendDAE::Variables;
    let mut l: Mutable::Mutable<metamodelica::List<i32>>;
    let mut i: Mutable::Mutable<i32>;
    (vars, l, i) = data.clone();
    if '__try0: {
        cr = varCref(&var);
        unwrap_break_err!(getVar(cr.clone(), &vars), '__try0);
        Mutable::update(
            l.clone(),
            metamodelica::cons(Mutable::access(i.clone()), Mutable::access(l.clone())),
        );
        Ok::<(), &'static str>(())
    }
    .is_err()
    {}
    Mutable::update(i.clone(), Mutable::access(i) + 1);
    (var, data)
}

pub fn mergeVariables(
    mut inVariables1: BackendDAE::Variables,
    mut inVariables2: BackendDAE::Variables,
    mut copy: bool,
) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    let mut num_vars: i32;
    num_vars = varsSize(&inVariables2);
    if varsLoadFactor(&inVariables1, num_vars)? > metamodelica::OrderedFloat((1) as f64) {
        outVariables = emptyVarsSized(varsSize(&inVariables1) + num_vars);
        outVariables = addVariables(inVariables1, outVariables)?;
    } else if copy {
        outVariables = copyVariables(inVariables1);
    } else {
        outVariables = inVariables1;
    }
    outVariables = addVariables(inVariables2, outVariables)?;
    Ok(outVariables)
}

pub(crate) fn rehashVariables(mut inVariables: BackendDAE::Variables) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    let mut load: metamodelica::Real = varsLoadFactor(&inVariables, 0)?;
    if load < metamodelica::OrderedFloat(0.5_f64) || load > metamodelica::OrderedFloat(1.0_f64) {
        outVariables = emptyVarsSized(varsSize(&inVariables));
        outVariables = addVariables(inVariables, outVariables)?;
    } else {
        outVariables = inVariables;
    }
    Ok(outVariables)
}

pub fn traverseBackendDAEVars<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inVariables: BackendDAE::Variables,
    mut inFunc: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<BackendDAE::Var>,
                ArgT,
            ) -> Result<(metamodelica::Ref<BackendDAE::Var>, ArgT)>
            + 'static,
    >,
    mut inArg: ArgT,
) -> Result<ArgT> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<BackendDAE::Var>,
                ArgT,
            ) -> Result<(metamodelica::Ref<BackendDAE::Var>, ArgT)>
            + 'static,
    >;

    let mut outArg: ArgT;
    let mut num_vars: i32;
    let mut vars: metamodelica::Array<Option<metamodelica::Ref<BackendDAE::Var>>>;
    let BackendDAE::VARIABLES {
        varArr:
            BackendDAE::VARIABLE_ARRAY {
                numberOfElements: __pa0,
                varOptArr: __pa1,
            },
        ..
    } = inVariables;
    num_vars = metamodelica::Own::own(__pa0);
    vars = metamodelica::Own::own(__pa1);
    outArg = BackendDAEUtil::traverseArrayNoCopy(
        vars.clone(),
        inFunc.clone(),
        &move |__a0: Option<metamodelica::Ref<BackendDAE::Var>>, __a1: _, __a2: _| {
            traverseBackendDAEVars2(__a0, metamodelica::arc_ref(&__a1), __a2)
        },
        inArg,
        num_vars,
    )?;
    Ok(outArg)
}

pub type filterFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>;

pub(crate) fn filterCrefs(
    mut variables: BackendDAE::Variables,
    mut func: Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>,
    mut acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = acc;
    acc = traverseBackendDAEVars(
        variables,
        (std::sync::Arc::new({
            let __pe_b1 = func.clone();
            move |__pe_a0, __pe_a2| filterTraverse(__pe_a0, &*__pe_b1, __pe_a2)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    ) -> Result<(
                        metamodelica::Ref<BackendDAE::Var>,
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    )> + 'static,
            >),
        acc,
    )?;
    Ok(acc)
}

fn filterTraverse(
    mut var: metamodelica::Ref<BackendDAE::Var>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool>,
    mut acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut var: metamodelica::Ref<BackendDAE::Var> = var;
    let mut acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = acc;
    if func(var.clone())? {
        acc = metamodelica::cons(var.varName.clone(), acc);
    }
    Ok((var, acc))
}

fn traverseBackendDAEVars2<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inVar: Option<metamodelica::Ref<BackendDAE::Var>>,
    mut inFunc: &dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, ArgT) -> Result<(metamodelica::Ref<BackendDAE::Var>, ArgT)>,
    mut inArg: ArgT,
) -> Result<ArgT> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<BackendDAE::Var>,
                ArgT,
            ) -> Result<(metamodelica::Ref<BackendDAE::Var>, ArgT)>
            + 'static,
    >;

    let mut outArg: ArgT;
    outArg = (::match_deref::match_deref! { match &(inVar) {
        Some(v) => {
            let mut arg: ArgT;
            (_, arg) = inFunc(v.clone(), inArg)?;
            arg
        },
        _ => {
            inArg
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outArg)
}

pub(crate) fn traverseBackendDAEVarsWithStop<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inVariables: BackendDAE::Variables,
    mut inFunc: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<BackendDAE::Var>,
                ArgT,
            ) -> Result<(metamodelica::Ref<BackendDAE::Var>, bool, ArgT)>
            + 'static,
    >,
    mut inArg: ArgT,
) -> Result<ArgT> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<BackendDAE::Var>,
                ArgT,
            ) -> Result<(metamodelica::Ref<BackendDAE::Var>, bool, ArgT)>
            + 'static,
    >;

    let mut outArg: ArgT;
    let mut num_vars: i32;
    let mut vars: metamodelica::Array<Option<metamodelica::Ref<BackendDAE::Var>>>;
    let BackendDAE::VARIABLES {
        varArr:
            BackendDAE::VARIABLE_ARRAY {
                numberOfElements: __pa0,
                varOptArr: __pa1,
            },
        ..
    } = inVariables;
    num_vars = metamodelica::Own::own(__pa0);
    vars = metamodelica::Own::own(__pa1);
    outArg = BackendDAEUtil::traverseArrayNoCopyWithStop(
        vars.clone(),
        inFunc.clone(),
        &move |__a0: Option<metamodelica::Ref<BackendDAE::Var>>, __a1: _, __a2: _| {
            traverseBackendDAEVarsWithStop2(__a0, metamodelica::arc_ref(&__a1), __a2)
        },
        inArg,
        num_vars,
    )?;
    Ok(outArg)
}

fn traverseBackendDAEVarsWithStop2<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inVar: Option<metamodelica::Ref<BackendDAE::Var>>,
    mut inFunc: &dyn ::std::ops::Fn(
        metamodelica::Ref<BackendDAE::Var>,
        ArgT,
    ) -> Result<(metamodelica::Ref<BackendDAE::Var>, bool, ArgT)>,
    mut inArg: ArgT,
) -> Result<(bool, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<BackendDAE::Var>,
                ArgT,
            ) -> Result<(metamodelica::Ref<BackendDAE::Var>, bool, ArgT)>
            + 'static,
    >;

    let mut outContinue: bool;
    let mut outArg: ArgT;
    (outContinue, outArg) = (::match_deref::match_deref! { match &(inVar) {
        None => {
            (true, inArg)
        },
        Some(v) => {
            let mut arg: ArgT;
            let mut cont: bool;
            (_, cont, arg) = inFunc(v.clone(), inArg)?;
            (cont, arg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outContinue, outArg))
}

pub(crate) fn traverseBackendDAE<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut dae: metamodelica::Ref<BackendDAE::BackendDAE>,
    mut inFunc: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<BackendDAE::Var>,
                ArgT,
            ) -> Result<(metamodelica::Ref<BackendDAE::Var>, ArgT)>
            + 'static,
    >,
    mut arg: ArgT,
) -> Result<(metamodelica::Ref<BackendDAE::BackendDAE>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<BackendDAE::Var>,
                ArgT,
            ) -> Result<(metamodelica::Ref<BackendDAE::Var>, ArgT)>
            + 'static,
    >;

    let mut dae: metamodelica::Ref<BackendDAE::BackendDAE> = dae;
    let mut arg: ArgT = arg;
    for mut syst in &*dae.eqs.clone() {
        (_, arg) = traverseBackendDAEVarsWithUpdate(syst.orderedVars.clone(), inFunc.clone(), arg)?;
    }
    (_, arg) = traverseBackendDAEVarsWithUpdate(dae.shared.globalKnownVars.clone(), inFunc.clone(), arg)?;
    (_, arg) = traverseBackendDAEVarsWithUpdate(dae.shared.localKnownVars.clone(), inFunc.clone(), arg)?;
    (_, arg) = traverseBackendDAEVarsWithUpdate(dae.shared.externalObjects.clone(), inFunc.clone(), arg)?;
    (_, arg) = traverseBackendDAEVarsWithUpdate(dae.shared.aliasVars.clone(), inFunc.clone(), arg)?;
    Ok((dae, arg))
}

pub(crate) fn traverseBackendDAEVarsWithUpdate<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inVariables: BackendDAE::Variables,
    mut inFunc: Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<BackendDAE::Var>,
                ArgT,
            ) -> Result<(metamodelica::Ref<BackendDAE::Var>, ArgT)>
            + 'static,
    >,
    mut inArg: ArgT,
) -> Result<(BackendDAE::Variables, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<BackendDAE::Var>,
                ArgT,
            ) -> Result<(metamodelica::Ref<BackendDAE::Var>, ArgT)>
            + 'static,
    >;

    let mut outVariables: BackendDAE::Variables;
    let mut outArg: ArgT;
    let mut num_vars1: i32;
    let mut num_vars2: i32;
    let mut vars: metamodelica::Array<Option<metamodelica::Ref<BackendDAE::Var>>>;
    let BackendDAE::VARIABLES {
        varArr:
            BackendDAE::VARIABLE_ARRAY {
                numberOfElements: __pa0,
                varOptArr: __pa1,
            },
        numberOfVars: __pa2,
        ..
    } = &inVariables;
    num_vars1 = metamodelica::Own::own(__pa0);
    vars = metamodelica::Own::own(__pa1);
    num_vars2 = metamodelica::Own::own(__pa2);
    if num_vars1 != num_vars2 {
        Error::addInternalError(
            literal!("function traverseBackendDAEVarsWithUpdate failed"),
            metamodelica::sourceInfo!("BackEnd/BackendVariable.mo"),
        )?;
        return Err("fail");
    }
    (vars, outArg) = BackendDAEUtil::traverseArrayNoCopyWithUpdate(
        vars.clone(),
        inFunc.clone(),
        &move |__a0: Option<metamodelica::Ref<BackendDAE::Var>>, __a1: _, __a2: _| {
            traverseBackendDAEVarsWithUpdate2(__a0, metamodelica::arc_ref(&__a1), __a2)
        },
        inArg,
        num_vars1,
    )?;
    outVariables = inVariables;
    outVariables.varArr = BackendDAE::VariableArray {
        numberOfElements: num_vars1,
        varOptArr: vars.clone(),
    };
    Ok((outVariables, outArg))
}

fn traverseBackendDAEVarsWithUpdate2<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inVar: Option<metamodelica::Ref<BackendDAE::Var>>,
    mut inFunc: &dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>, ArgT) -> Result<(metamodelica::Ref<BackendDAE::Var>, ArgT)>,
    mut inArg: ArgT,
) -> Result<(Option<metamodelica::Ref<BackendDAE::Var>>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<BackendDAE::Var>,
                ArgT,
            ) -> Result<(metamodelica::Ref<BackendDAE::Var>, ArgT)>
            + 'static,
    >;

    let mut outVar: Option<metamodelica::Ref<BackendDAE::Var>>;
    let mut outArg: ArgT;
    (outVar, outArg) = (::match_deref::match_deref! { match &(inVar.clone()) {
        None => {
            (inVar, inArg)
        },
        Some(v) => {
            let mut ov: Option<metamodelica::Ref<BackendDAE::Var>>;
            let mut new_v: metamodelica::Ref<BackendDAE::Var>;
            let mut arg: ArgT;
            (new_v, arg) = inFunc(v.clone(), inArg)?;
            ov = if (referenceEq(&*(v.clone()),&*(&*new_v))) {inVar} else {Some(new_v)};
            (ov, arg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outVar, outArg))
}

pub(crate) fn getAllCrefFromVariables(
    mut inVariables: BackendDAE::Variables,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut cr_lst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    cr_lst = traverseBackendDAEVars(
        inVariables,
        (std::sync::Arc::new(fnptr!(
            traversingVarCrefFinder,
            metamodelica::Ref<BackendDAE::Var>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    ) -> Result<(
                        metamodelica::Ref<BackendDAE::Var>,
                        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                    )> + 'static,
            >),
        metamodelica::nil(),
    )?;
    Ok(cr_lst)
}

fn traversingVarCrefFinder(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    outVar = inVar.clone();
    outCrefs = metamodelica::cons(varCref(&inVar), inCrefs);
    (outVar, outCrefs)
}

pub fn collectVarKindVarinVariables(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inVarArrays: (
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>,
        BackendDAE::Variables,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    (
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>,
        BackendDAE::Variables,
    ),
)> {
    pub type checkVarKindFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>;

    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
    let mut outVarArrays: (
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>,
        BackendDAE::Variables,
    ) = inVarArrays.clone();
    let mut vararray: BackendDAE::Variables;
    let mut checkVarKind: Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>;
    (checkVarKind, vararray) = inVarArrays;
    outVarArrays = (match &*inVar {
        _ if (checkVarKind(inVar.clone())?) => {
            vararray = addVar(inVar.clone(), vararray)?;
            (checkVarKind.clone(), vararray)
        }
        _ => outVarArrays,
    });
    Ok((outVar, outVarArrays))
}

pub(crate) fn getAllDiscreteVarFromVariables(
    mut inVariables: BackendDAE::Variables,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut v_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    v_lst = traverseBackendDAEVars(
        inVariables,
        (std::sync::Arc::new(fnptr!(
            traversingisisVarDiscreteFinder,
            metamodelica::Ref<BackendDAE::Var>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                    ) -> Result<(
                        metamodelica::Ref<BackendDAE::Var>,
                        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                    )> + 'static,
            >),
        metamodelica::nil(),
    )?;
    Ok(v_lst)
}

fn traversingisisVarDiscreteFinder(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) {
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    let mut v_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    v = inVar;
    v_lst = List::consOnTrue(isVarDiscrete(&v), v.clone(), inVars);
    (v, v_lst)
}

pub(crate) fn getAllStateVarFromVariables(
    mut inVariables: BackendDAE::Variables,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut v_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    v_lst = traverseBackendDAEVars(
        inVariables,
        (std::sync::Arc::new(fnptr!(
            traversingisStateVarFinder,
            metamodelica::Ref<BackendDAE::Var>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                    ) -> Result<(
                        metamodelica::Ref<BackendDAE::Var>,
                        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                    )> + 'static,
            >),
        metamodelica::nil(),
    )?;
    Ok(v_lst)
}

pub(crate) fn getAllClockedStatesFromVariables(
    mut inVariables: BackendDAE::Variables,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut v_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    v_lst = traverseBackendDAEVars(
        inVariables,
        (std::sync::Arc::new(fnptr!(
            traversingisClockedStateVarFinder,
            metamodelica::Ref<BackendDAE::Var>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                    ) -> Result<(
                        metamodelica::Ref<BackendDAE::Var>,
                        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                    )> + 'static,
            >),
        metamodelica::nil(),
    )?;
    Ok(v_lst)
}

pub(crate) fn getNumStateVarFromVariables(mut inVariables: BackendDAE::Variables) -> Result<i32> {
    let mut count: i32;
    count = traverseBackendDAEVars(
        inVariables,
        (std::sync::Arc::new(fnptr!(traversingisStateCount, metamodelica::Ref<BackendDAE::Var>, i32))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        i32,
                    ) -> Result<(metamodelica::Ref<BackendDAE::Var>, i32)>
                    + 'static,
            >),
        0,
    )?;
    Ok(count)
}

fn traversingisStateVarFinder(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) {
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    let mut v_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    v = inVar;
    v_lst = List::consOnTrue(isStateVar(&v), v.clone(), inVars);
    (v, v_lst)
}

fn traversingisClockedStateVarFinder(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) {
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    let mut v_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    v = inVar;
    v_lst = List::consOnTrue(isClockedStateVar(&v), v.clone(), inVars);
    (v, v_lst)
}

fn traversingisStateCount(
    mut v: metamodelica::Ref<BackendDAE::Var>,
    mut count: i32,
) -> (metamodelica::Ref<BackendDAE::Var>, i32) {
    let mut v: metamodelica::Ref<BackendDAE::Var> = v;
    let mut count: i32 = count;
    if isStateVar(&v) {
        count = count + 1;
    }
    (v, count)
}

pub(crate) fn getAllVarIndicesFromVariables(
    mut inVariables: BackendDAE::Variables,
    mut isFunc: Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<i32>,
)> {
    pub type FindFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>;

    let mut v_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut i_lst: metamodelica::List<i32>;
    let mut v_a: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>;
    let mut i_a: metamodelica::Array<metamodelica::List<i32>>;
    v_a = arrayCreate(1, metamodelica::nil());
    i_a = arrayCreate(1, metamodelica::nil());
    traverseBackendDAEVars(
        inVariables,
        (std::sync::Arc::new({
            let __pe_b1 = v_a.clone();
            let __pe_b2 = i_a.clone();
            let __pe_b3: Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static> =
                isFunc.clone();
            move |__pe_a0, __pe_a4| traversingisXXXFinder(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), &*__pe_b3, __pe_a4)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        metamodelica::Array<i32>,
                    )
                        -> Result<(metamodelica::Ref<BackendDAE::Var>, metamodelica::Array<i32>)>
                    + 'static,
            >),
        arrayCreate(1, 1),
    )?;
    v_lst = ({
        let __elt = (*metamodelica::index_checked(&v_a.borrow(), 1)?).clone();
        __elt
    });
    i_lst = ({
        let __elt = (*metamodelica::index_checked(&i_a.borrow(), 1)?).clone();
        __elt
    });
    Ok((v_lst, i_lst))
}

fn traversingisXXXFinder(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut v_lst: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>,
    mut i_lst: metamodelica::Array<metamodelica::List<i32>>,
    mut isFunc: &dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool>,
    mut i: metamodelica::Array<i32>,
) -> Result<(metamodelica::Ref<BackendDAE::Var>, metamodelica::Array<i32>)> {
    pub type FindFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>;

    let mut inVar: metamodelica::Ref<BackendDAE::Var> = inVar;
    let mut i: metamodelica::Array<i32> = i;
    if isFunc(inVar.clone())? {
        metamodelica::arrayUpdate(
            v_lst.clone(),
            1,
            metamodelica::cons(
                inVar.clone(),
                ({
                    let __elt = (*metamodelica::index_checked(&v_lst.borrow(), 1)?).clone();
                    __elt
                }),
            ),
        )?;
        metamodelica::arrayUpdate(
            i_lst.clone(),
            1,
            metamodelica::cons(
                ({
                    let __elt = (*metamodelica::index_checked(&i.borrow(), 1)?).clone();
                    __elt
                }),
                ({
                    let __elt = (*metamodelica::index_checked(&i_lst.borrow(), 1)?).clone();
                    __elt
                }),
            ),
        )?;
    }
    {
        let __cell0 = ({
            let __elt = (*metamodelica::index_checked(&i.borrow(), 1)?).clone();
            __elt
        }) + 1;
        let __idx0 = 1;
        *metamodelica::index_mut_checked(&mut i.clone().borrow_mut(), __idx0)? = __cell0;
    }
    Ok((inVar, i))
}

pub(crate) fn mergeVariableOperations(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inOps: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
    let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
    ops = inOps.reverse();
    assign_field!(outVar.source = List::foldr(&ops, &ElementSource::addSymbolicTransformation, inVar.source.clone())?);
    Ok(outVar)
}

pub(crate) fn mergeAliasVars(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inAVar: metamodelica::Ref<BackendDAE::Var>,
    mut negate: bool,
    mut globalKnownVars: BackendDAE::Variables,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut v1: metamodelica::Ref<BackendDAE::Var>;
    let mut v2: metamodelica::Ref<BackendDAE::Var>;
    let mut fixed: bool;
    let mut fixeda: bool;
    let mut sv: Option<metamodelica::Ref<DAE::Exp>>;
    let mut sva: Option<metamodelica::Ref<DAE::Exp>>;
    let mut so: Option<DAE::StartOrigin>;
    let mut soa: Option<DAE::StartOrigin>;
    fixed = varFixed(&inVar);
    fixeda = varFixed(&inAVar);
    sv = varStartValueOption(&inVar);
    sva = varStartValueOption(&inAVar);
    so = varStartOrigin(&inVar)?;
    soa = varStartOrigin(&inAVar)?;
    v1 = mergeStartFixed(inVar, fixed, sv, so, &inAVar, fixeda, sva, soa, negate, globalKnownVars)?;
    v2 = mergeNominalAttribute(inAVar.clone(), v1, negate);
    outVar = mergeMinMaxAttribute(inAVar, v2, negate);
    Ok(outVar)
}

fn mergeStartFixed(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut fixed: bool,
    mut sv: Option<metamodelica::Ref<DAE::Exp>>,
    mut so: Option<DAE::StartOrigin>,
    mut inAVar: &metamodelica::Ref<BackendDAE::Var>,
    mut fixeda: bool,
    mut sva: Option<metamodelica::Ref<DAE::Exp>>,
    mut soa: Option<DAE::StartOrigin>,
    mut negate: bool,
    mut globalKnownVars: BackendDAE::Variables,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    outVar = 'mc: {
        let __mc_input = (inVar, fixed, &sv, &**inAVar, fixeda, &sva);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, true, _, _, false, _) => {
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, false, _, _, true, Some(sb)) => {
                    let mut v1: metamodelica::Ref<BackendDAE::Var>;
                    let mut v2: metamodelica::Ref<BackendDAE::Var>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    e = if (negate) {Expression::negate(sb.clone())?} else {sb.clone()};
                    v1 = setVarStartValue(v.clone(), e.clone())?;
                    v2 = setVarFixed(v1.clone(), true)?;
                    Ok(v2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, false, None, _, true, None) => {
                    let mut v1: metamodelica::Ref<BackendDAE::Var>;
                    v1 = setVarFixed(v.clone(), true)?;
                    Ok(v1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, false, Some(_), _, true, None) => {
                    let mut v1: metamodelica::Ref<BackendDAE::Var>;
                    setVarStartValueOption(v.clone(), None)?;
                    v1 = setVarFixed(v.clone(), true)?;
                    Ok(v1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, false, None, _, false, None) => {
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, false, Some(_), _, false, None) => {
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, false, None, _, false, Some(sb)) => {
                    let mut v1: metamodelica::Ref<BackendDAE::Var>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    e = if (negate) {Expression::negate(sb.clone())?} else {sb.clone()};
                    v1 = setVarStartValue(v.clone(), e.clone())?;
                    Ok(v1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { varType: ty, .. }, false, _, Deref @ BackendDAE::Var { varType: tya, .. }, false, _) => {
                    let mut v1: metamodelica::Ref<BackendDAE::Var>;
                    let mut sa: metamodelica::Ref<DAE::Exp>;
                    let mut sb: metamodelica::Ref<DAE::Exp>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut origin: Option<DAE::StartOrigin>;
                    sa = startValueType(sv.clone(), ty.clone())?;
                    sb = startValueType(sva.clone(), tya.clone())?;
                    e = if (negate) {Expression::negate(sb.clone())?} else {sb.clone()};
                    (e, origin) = getNonZeroStart(false, sa.clone(), so.clone(), e.clone(), soa.clone(), globalKnownVars.clone())?;
                    setVarStartValue(v.clone(), e.clone())?;
                    v1 = setVarStartOrigin(v.clone(), origin.clone())?;
                    Ok(v1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { varName: cr, varType: ty, .. }, false, _, Deref @ BackendDAE::Var { varName: cra, varType: tya, .. }, false, _) => {
                    let mut sa: metamodelica::Ref<DAE::Exp>;
                    let mut sb: metamodelica::Ref<DAE::Exp>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut i: i32;
                    let mut ia: i32;
                    sa = startValueType(sv.clone(), ty.clone())?;
                    sb = startValueType(sva.clone(), tya.clone())?;
                    e = if (negate) {Expression::negate(sb.clone())?} else {sb.clone()};
                    i = ComponentReference::crefDepth(metamodelica::AsArg::as_arg(&cr))?;
                    ia = ComponentReference::crefDepth(metamodelica::AsArg::as_arg(&cra))?;
                    Ok(mergeStartFixed1(intLt(ia, i), v.clone(), metamodelica::AsArg::as_arg(&cr), sa.clone(), metamodelica::AsArg::as_arg(&cra), e.clone(), soa.clone(), negate, literal!(" have start values "))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, true, None, _, true, None) => {
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { varType: ty, .. }, true, _, Deref @ BackendDAE::Var { varType: tya, .. }, true, _) => {
                    let mut v1: metamodelica::Ref<BackendDAE::Var>;
                    let mut sa: metamodelica::Ref<DAE::Exp>;
                    let mut sb: metamodelica::Ref<DAE::Exp>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut origin: Option<DAE::StartOrigin>;
                    sa = startValueType(sv.clone(), ty.clone())?;
                    sb = startValueType(sva.clone(), tya.clone())?;
                    e = if (negate) {Expression::negate(sb.clone())?} else {sb.clone()};
                    (e, origin) = getNonZeroStart(true, sa.clone(), so.clone(), e.clone(), soa.clone(), globalKnownVars.clone())?;
                    setVarStartValue(v.clone(), e.clone())?;
                    v1 = setVarStartOrigin(v.clone(), origin.clone())?;
                    Ok(v1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { varName: cr, varType: ty, .. }, true, _, Deref @ BackendDAE::Var { varName: cra, varType: tya, .. }, true, _) => {
                    let mut sa: metamodelica::Ref<DAE::Exp>;
                    let mut sb: metamodelica::Ref<DAE::Exp>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut i: i32;
                    let mut ia: i32;
                    sa = startValueType(sv.clone(), ty.clone())?;
                    sb = startValueType(sva.clone(), tya.clone())?;
                    e = if (negate) {Expression::negate(sb.clone())?} else {sb.clone()};
                    i = ComponentReference::crefDepth(metamodelica::AsArg::as_arg(&cr))?;
                    ia = ComponentReference::crefDepth(metamodelica::AsArg::as_arg(&cra))?;
                    Ok(mergeStartFixed1(intLt(ia, i), v.clone(), metamodelica::AsArg::as_arg(&cr), sa.clone(), metamodelica::AsArg::as_arg(&cra), e.clone(), soa.clone(), negate, literal!(" both fixed and have start values "))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVar)
}

fn startValueType(
    mut iExp: Option<metamodelica::Ref<DAE::Exp>>,
    mut iTy: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut oExp: metamodelica::Ref<DAE::Exp>;
    oExp = (::match_deref::match_deref! { match &(iExp) {
        Some(e) => {
            e.clone()
        },
        None if (Types::isRealOrSubTypeReal(iTy.clone())) => {
            metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })
        },
        None if (Types::isIntegerOrSubTypeInteger(iTy.clone())) => {
            metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 })
        },
        None if (Types::isBooleanOrSubTypeBoolean(iTy.clone())) => {
            metamodelica::Ref::new(DAE::Exp::BCONST { bool: false })
        },
        None if (Types::isStringOrSubTypeString(iTy.clone())) => {
            metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") })
        },
        None if (Types::isEnumerationOrSubTypeEnumeration(iTy.clone())) => {
            Types::getNthEnumLiteral(&iTy, 1)?
        },
        _ => {
            metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oExp)
}

fn mergeStartFixed1(
    mut b: bool,
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut sv: metamodelica::Ref<DAE::Exp>,
    mut cra: &metamodelica::Ref<DAE::ComponentRef>,
    mut sva: metamodelica::Ref<DAE::Exp>,
    mut soa: Option<DAE::StartOrigin>,
    mut negate: bool,
    mut s4: ArcStr,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    outVar = (match b {
        false => {
            let mut s: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut s5: ArcStr;
            let mut s6: ArcStr;
            s1 = ComponentReferenceBasics::printComponentRefStr(cr)?;
            s2 = if (negate) { literal!(" = -") } else { literal!(" = ") };
            s3 = ComponentReferenceBasics::printComponentRefStr(cra)?;
            s5 = ExpressionBasics::printExpStr(sv)?;
            s6 = ExpressionBasics::printExpStr(sva)?;
            s = stringAppendList(list![
                literal!("Alias variables "),
                s1.clone(),
                s2,
                s3,
                s4,
                s5,
                literal!(" != "),
                s6,
                literal!(". Use value from "),
                s1,
                literal!(".")
            ]);
            Error::addMessage(Error::COMPILER_WARNING.clone(), list![s])?;
            inVar
        }
        true => {
            let mut s: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut s3: ArcStr;
            let mut s5: ArcStr;
            let mut s6: ArcStr;
            let mut v: metamodelica::Ref<BackendDAE::Var>;
            s1 = ComponentReferenceBasics::printComponentRefStr(cr)?;
            s2 = if (negate) { literal!(" = -") } else { literal!(" = ") };
            s3 = ComponentReferenceBasics::printComponentRefStr(cra)?;
            s5 = ExpressionBasics::printExpStr(sv)?;
            s6 = ExpressionBasics::printExpStr(sva.clone())?;
            s = stringAppendList(list![
                literal!("Alias variables "),
                s1,
                s2,
                s3.clone(),
                s4,
                s5,
                literal!(" != "),
                s6,
                literal!(". Use value from "),
                s3,
                literal!(".")
            ]);
            Error::addMessage(Error::COMPILER_WARNING.clone(), list![s])?;
            v = setVarStartValue(inVar, sva)?;
            v = setVarStartOrigin(v, soa)?;
            v
        }
    });
    Ok(outVar)
}

fn replaceCrefWithBindExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: (
        BackendDAE::Variables,
        bool,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
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
            ),
        ),
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (
        BackendDAE::Variables,
        bool,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
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
            ),
        ),
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTuple: (
        BackendDAE::Variables,
        bool,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
            i32,
            (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
        ),
    );
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), &inTuple);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (vars, _, hs)) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut hs = (*hs).clone();
                    let false = (BaseHashSet::has(cr.clone(), &(hs.clone()))?) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(getVarSingle(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&vars))?) {
                        (Deref @ BackendDAE::Var { bindExp: Some(__pa0), .. }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    hs = BaseHashSet::add(cr.clone(), &(hs.clone()))?;
                    let (__pa2, (_, _, __pa3)) = Expression::traverseExpBottomUp(e.clone(), &fnptr!(replaceCrefWithBindExp, metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, bool, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))), (vars.clone(), false, hs.clone()))?;
                    e = metamodelica::Own::own(__pa2);
                    hs = metamodelica::Own::own(__pa3);
                    Ok((e.clone(), (vars.clone(), true, hs.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { .. }, (vars, _, hs)) => {
                    Ok((e.clone(), (vars.clone(), true, hs.clone())))
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

fn getNonZeroStart(
    mut mustBeEqual: bool,
    mut exp1: metamodelica::Ref<DAE::Exp>,
    mut so: Option<DAE::StartOrigin>,
    mut exp2: metamodelica::Ref<DAE::Exp>,
    mut sao: Option<DAE::StartOrigin>,
    mut globalKnownVars: BackendDAE::Variables,
) -> Result<(metamodelica::Ref<DAE::Exp>, Option<DAE::StartOrigin>)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outStartOrigin: Option<DAE::StartOrigin>;
    (outExp, outStartOrigin) = 'mc: {
        let __mc_input = mustBeEqual;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut origin: Option<DAE::StartOrigin>;
            let true = (ExpressionBasics::expEqual(&exp1, exp2.clone())?) else {
                return Err("pattern mismatch");
            };
            origin = if (startOriginCompare(sao.clone(), so.clone())? < 0) {
                sao.clone()
            } else {
                so.clone()
            };
            Ok((exp1.clone(), origin.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let false = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut exp1_1: metamodelica::Ref<DAE::Exp>;
            let mut origin: Option<DAE::StartOrigin>;
            let false = (startOriginCompare(so.clone(), sao.clone())? == 0) else {
                return Err("pattern mismatch");
            };
            (exp1_1, origin) = if (startOriginCompare(sao.clone(), so.clone())? < 0) {
                (exp2.clone(), sao.clone())
            } else {
                (exp1.clone(), so.clone())
            };
            Ok((exp1_1.clone(), origin.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut exp2_1: metamodelica::Ref<DAE::Exp>;
            let mut exp1_1: metamodelica::Ref<DAE::Exp>;
            let mut b1: bool;
            let mut b2: bool;
            let mut origin: Option<DAE::StartOrigin>;
            let (__pa0, (_, __pa1, _)) = Expression::traverseExpBottomUp(
                exp1.clone(),
                &fnptr!(
                    replaceCrefWithBindExp,
                    metamodelica::Ref<DAE::Exp>,
                    (
                        BackendDAE::Variables,
                        bool,
                        (
                            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                            (
                                i32,
                                i32,
                                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>
                            ),
                            i32,
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
                                Arc<
                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                        + 'static,
                                >
                            )
                        )
                    )
                ),
                (globalKnownVars.clone(), false, HashSet::emptyHashSet()),
            )?;
            exp1_1 = metamodelica::Own::own(__pa0);
            b1 = metamodelica::Own::own(__pa1);
            let (__pa2, (_, __pa3, _)) = Expression::traverseExpBottomUp(
                exp2.clone(),
                &fnptr!(
                    replaceCrefWithBindExp,
                    metamodelica::Ref<DAE::Exp>,
                    (
                        BackendDAE::Variables,
                        bool,
                        (
                            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                            (
                                i32,
                                i32,
                                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>
                            ),
                            i32,
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
                                Arc<
                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr>
                                        + 'static,
                                >
                            )
                        )
                    )
                ),
                (globalKnownVars.clone(), false, HashSet::emptyHashSet()),
            )?;
            exp2_1 = metamodelica::Own::own(__pa2);
            b2 = metamodelica::Own::own(__pa3);
            (exp1_1, _) = ExpressionSimplify::condsimplify(b1, exp1_1.clone())?;
            (exp2_1, _) = ExpressionSimplify::condsimplify(b2, exp2_1.clone())?;
            let true = (ExpressionBasics::expEqual(&exp1_1, exp2_1.clone())?) else {
                return Err("pattern mismatch");
            };
            exp1_1 = if (b1) { exp1.clone() } else { exp2.clone() };
            origin = if (startOriginCompare(sao.clone(), so.clone())? < 0) {
                sao.clone()
            } else {
                so.clone()
            };
            Ok((exp1_1.clone(), origin.clone()))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, outStartOrigin))
}

pub(crate) fn startOriginCompare(mut so1: Option<DAE::StartOrigin>, mut so2: Option<DAE::StartOrigin>) -> Result<i32> {
    let mut cmp: i32;
    let mut k1: i32;
    let mut a1: i32;
    let mut r1: i32;
    let mut k2: i32;
    let mut a2: i32;
    let mut r2: i32;
    (k1, a1, r1) = startOriginRank(so1)?;
    (k2, a2, r2) = startOriginRank(so2)?;
    cmp = if (k1 != k2) {
        k1 - k2
    } else if (a1 != a2) {
        a1 - a2
    } else {
        r1 - r2
    };
    Ok(cmp)
}

fn startOriginRank(mut so: Option<DAE::StartOrigin>) -> Result<(i32, i32, i32)> {
    let mut kind: i32;
    let mut actual: i32 = 0;
    let mut raw: i32 = 0;
    kind = (match so {
        Some(mut origin @ DAE::StartOrigin::CONFIDENCE { .. }) => {
            actual = var_field!(origin.actual, DAE::StartOrigin::CONFIDENCE).clone();
            raw = var_field!(origin.raw, DAE::StartOrigin::CONFIDENCE).clone();
            0
        }
        Some(mut origin @ DAE::StartOrigin::TYPE_CONFIDENCE { .. }) => {
            actual = var_field!(origin.level, DAE::StartOrigin::TYPE_CONFIDENCE).clone();
            1
        }
        Some(DAE::StartOrigin::BINDING_ORIGIN { .. }) => 2,
        Some(DAE::StartOrigin::TYPE_ORIGIN { .. }) => 3,
        Some(DAE::StartOrigin::UNDEFINED_ORIGIN { .. }) => 4,
        None => 5,
        _ => return Err("match: no arm matched"),
    });
    Ok((kind, actual, raw))
}

pub(crate) fn mergeNominalAttribute(
    mut inAVar: metamodelica::Ref<BackendDAE::Var>,
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut negate: bool,
) -> metamodelica::Ref<BackendDAE::Var> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    outVar = 'mc: {
        let __mc_input = (inAVar, inVar.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, var) => {
                    let mut var1: metamodelica::Ref<BackendDAE::Var>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut esum: metamodelica::Ref<DAE::Exp>;
                    let mut eaverage: metamodelica::Ref<DAE::Exp>;
                    e = varNominalValue(metamodelica::AsArg::as_arg(&v))?;
                    e1 = varNominalValue(metamodelica::AsArg::as_arg(&var))?;
                    e_1 = if (negate) {Expression::negate(e.clone())?} else {e.clone()};
                    esum = Expression::makeSum(list![e_1.clone(), e1.clone()])?;
                    eaverage = Expression::expDiv(esum.clone(), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))?;
                    (eaverage, _) = ExpressionSimplify::simplify(eaverage.clone())?;
                    var1 = setVarNominalValue(var.clone(), eaverage.clone())?;
                    Ok(var1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, var) => {
                    let mut var1: metamodelica::Ref<BackendDAE::Var>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    e = varNominalValue(metamodelica::AsArg::as_arg(&v))?;
                    e_1 = if (negate) {Expression::negate(e.clone())?} else {e.clone()};
                    var1 = setVarNominalValue(var.clone(), e_1.clone())?;
                    Ok(var1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    Ok(inVar.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outVar
}

pub(crate) fn mergeMinMaxAttribute(
    mut inAVar: metamodelica::Ref<BackendDAE::Var>,
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut negate: bool,
) -> metamodelica::Ref<BackendDAE::Var> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    outVar = 'mc: {
        let __mc_input = (inAVar, inVar.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { values: attr, .. }, var @ Deref @ BackendDAE::Var { values: attr1, .. }) => {
                    let mut var1: metamodelica::Ref<BackendDAE::Var>;
                    let mut min1: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut min2: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut max1: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut max2: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
                    (min1, max1) = DAEUtil::getMinMaxValues(attr.clone());
                    (min2, max2) = DAEUtil::getMinMaxValues(attr1.clone());
                    cr = varCref(metamodelica::AsArg::as_arg(&v));
                    cr1 = varCref(metamodelica::AsArg::as_arg(&var));
                    (min1, max1) = mergeMinMax(negate, min1.clone(), min2.clone(), max1.clone(), max2.clone(), &cr, &cr1)?;
                    var1 = setVarMinMax(var.clone(), min1.clone(), max1.clone())?;
                    Ok(var1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inVar.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outVar
}

pub(crate) fn getMinMaxAttribute(
    mut inVar: &metamodelica::Ref<BackendDAE::Var>,
) -> (Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>) {
    let mut outMin: Option<metamodelica::Ref<DAE::Exp>>;
    let mut outMax: Option<metamodelica::Ref<DAE::Exp>>;
    (outMin, outMax) = DAEUtil::getMinMaxValues(inVar.values.clone());
    (outMin, outMax)
}

fn mergeMinMax(
    mut negate: bool,
    mut inMin1: Option<metamodelica::Ref<DAE::Exp>>,
    mut inMin2: Option<metamodelica::Ref<DAE::Exp>>,
    mut inMax1: Option<metamodelica::Ref<DAE::Exp>>,
    mut inMax2: Option<metamodelica::Ref<DAE::Exp>>,
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut cr1: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(Option<metamodelica::Ref<DAE::Exp>>, Option<metamodelica::Ref<DAE::Exp>>)> {
    let mut outMin: Option<metamodelica::Ref<DAE::Exp>>;
    let mut outMax: Option<metamodelica::Ref<DAE::Exp>>;
    outMin = if (negate) {
        Util::applyOption(inMin1, &Expression::negate)?
    } else {
        inMin1
    };
    outMax = if (negate) {
        Util::applyOption(inMax1, &Expression::negate)?
    } else {
        inMax1
    };
    outMin = mergeMin(outMin, inMin2)?;
    outMax = mergeMax(outMax, inMax2)?;
    checkMinMax(outMin.clone(), outMax.clone(), cr, cr1, negate);
    Ok((outMin, outMax))
}

fn checkMinMax(
    mut inMin: Option<metamodelica::Ref<DAE::Exp>>,
    mut inMax: Option<metamodelica::Ref<DAE::Exp>>,
    mut cr1: &metamodelica::Ref<DAE::ComponentRef>,
    mut cr2: &metamodelica::Ref<DAE::ComponentRef>,
    mut negate: bool,
) -> () {
    let () = 'mc: {
        let __mc_input = (inMin, inMax);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(min), Some(max)) => {
                    let mut s: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    let mut s4: ArcStr;
                    let mut s5: ArcStr;
                    let mut rmin: metamodelica::Real;
                    let mut rmax: metamodelica::Real;
                    rmin = Expression::toReal(metamodelica::AsArg::as_arg(&min))?;
                    rmax = Expression::toReal(metamodelica::AsArg::as_arg(&max))?;
                    let true = (realGt(rmin, rmax)) else { return Err("pattern mismatch") };
                    s1 = ComponentReferenceBasics::printComponentRefStr(cr1)?;
                    s2 = if (negate) {literal!(" = -")} else {literal!(" = ")};
                    s3 = ComponentReferenceBasics::printComponentRefStr(cr2)?;
                    s4 = ExpressionBasics::printExpStr(min.clone())?;
                    s5 = ExpressionBasics::printExpStr(max.clone())?;
                    s = stringAppendList(list![literal!("Alias variables "), s1.clone(), s2.clone(), s3.clone(), literal!(" with invalid limits min "), s4.clone(), literal!(" > max "), s5.clone()]);
                    Error::addMessage(Error::COMPILER_WARNING.clone(), list![s.clone()])?;
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
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    ()
}

fn mergeMin(
    mut inMin1: Option<metamodelica::Ref<DAE::Exp>>,
    mut inMin2: Option<metamodelica::Ref<DAE::Exp>>,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut outMin: Option<metamodelica::Ref<DAE::Exp>>;
    outMin = (::match_deref::match_deref! { match &((inMin1.clone(), inMin2.clone())) {
        (Some(min1), Some(min2)) => {
            let mut min: metamodelica::Ref<DAE::Exp>;
            min = Expression::expMaxScalar(min1.clone(), min2.clone())?;
            (min, _) = ExpressionSimplify::simplify(min)?;
            if (referenceEq(&*(&*min),&*(min1.clone()))) {inMin1} else if (referenceEq(&*(&*min),&*(min2.clone()))) {inMin2} else {Some(min)}
        },
        (None, _) => {
            inMin2
        },
        (_, None) => {
            inMin1
        },
        _ => {
            inMin1
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMin)
}

fn mergeMax(
    mut inMax1: Option<metamodelica::Ref<DAE::Exp>>,
    mut inMax2: Option<metamodelica::Ref<DAE::Exp>>,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut outMax: Option<metamodelica::Ref<DAE::Exp>>;
    outMax = (::match_deref::match_deref! { match &((inMax1.clone(), inMax2.clone())) {
        (Some(max1), Some(max2)) => {
            let mut max: metamodelica::Ref<DAE::Exp>;
            max = Expression::expMinScalar(max1.clone(), max2.clone())?;
            (max, _) = ExpressionSimplify::simplify(max)?;
            if (referenceEq(&*(&*max),&*(max1.clone()))) {inMax1} else if (referenceEq(&*(&*max),&*(max2.clone()))) {inMax2} else {Some(max)}
        },
        (None, _) => {
            inMax2
        },
        (_, None) => {
            inMax1
        },
        _ => {
            inMax1
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMax)
}

pub(crate) fn calcAliasKey(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> Result<i32> {
    let mut i: i32;
    let mut b: bool;
    let mut d: i32;
    b = ComponentReference::isRecord(&inVar.varName);
    i = if (b) { -1 } else { 0 };
    b = ComponentReference::isArrayElement(&inVar.varName);
    i = intAdd(i, if (b) { -1 } else { 0 });
    b = isProtectedVar(inVar);
    i = intAdd(i, if (b) { 5 } else { 0 });
    b = isVarConnector(inVar);
    i = intAdd(i, if (b) { 1 } else { 0 });
    b = isDummyDerVar(inVar);
    i = intAdd(i, if (b) { 10 } else { 0 });
    b = selfGeneratedVar(&inVar.varName)?;
    i = intAdd(i, if (b) { 100 } else { 0 });
    d = ComponentReference::crefDepth(&inVar.varName)?;
    i = i + d;
    Ok(i)
}

pub(crate) fn selfGeneratedVar(mut inCref: &metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> {
    let mut b: bool = StringUtil::startsWith(ComponentReference::crefStr(inCref)?, literal!("$"));
    Ok(b)
}

pub(crate) fn varStateSelectPrioAlias(mut v: &metamodelica::Ref<BackendDAE::Var>) -> i32 {
    let mut prio: i32;
    let mut ss: DAE::StateSelect;
    let mut knownDer: bool;
    ss = varStateSelect(v);
    prio = stateSelectToInteger(ss);
    knownDer = varHasStateDerivative(v);
    prio = prio * 2;
    prio = if (knownDer) { prio + 1 } else { prio };
    prio
}

pub(crate) fn stateSelectToInteger(mut inStateSelect: DAE::StateSelect) -> i32 {
    let mut prio: i32;
    prio = (match inStateSelect {
        DAE::StateSelect::NEVER { .. } => -1,
        DAE::StateSelect::AVOID { .. } => 0,
        DAE::StateSelect::DEFAULT { .. } => 1,
        DAE::StateSelect::PREFER { .. } => 2,
        DAE::StateSelect::ALWAYS { .. } => 3,
    });
    prio
}

pub(crate) fn transformXToXd(mut inVar: metamodelica::Ref<BackendDAE::Var>) -> metamodelica::Ref<BackendDAE::Var> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    outVar = (match &*inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::STATE { .. },
            ..
        } => {
            outVar = inVar.clone();
            assign_field!(
                outVar.varName = ComponentReference::crefPrefixDer(inVar.varName.clone()),
                outVar.varKind = openmodelica_backend_types::BackendDAE::VarKind::STATE_DER
            );
            outVar
        }
        _ => inVar,
    });
    outVar
}

pub(crate) fn setStateIndex(
    mut v1: metamodelica::Ref<BackendDAE::Var>,
    mut idx: i32,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut v2: metamodelica::Ref<BackendDAE::Var> = v1.clone();
    let mut derName: Option<metamodelica::Ref<DAE::ComponentRef>>;
    let mut natural: bool;
    if isStateVar(&v1) {
        let BackendDAE::STATE {
            index: _,
            derName: __pa0,
            natural: __pa1,
        } = (getVarKind(&v1))
        else {
            return Err("pattern mismatch");
        };
        derName = metamodelica::Own::own(__pa0);
        natural = metamodelica::Own::own(__pa1);
        v2 = setVarKind(
            v1,
            BackendDAE::VarKind::STATE {
                index: idx,
                derName: derName,
                natural: natural,
            },
        )?;
    }
    Ok(v2)
}

pub(crate) fn isRecordVar(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> Result<bool> {
    let mut isRec: bool = ComponentReference::traverseCref(
        &inVar.varName,
        &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: bool| ComponentReference::crefIsRec(&__a0, __a1),
        false,
    )?;
    Ok(isRec)
}

pub(crate) fn varExp(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = Expression::crefToExp(inVar.varName.clone())?;
    Ok(outExp)
}

pub(crate) fn varExp2(mut inVar: &metamodelica::Ref<BackendDAE::Var>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (match &**inVar {
        BackendDAE::Var {
            varKind: BackendDAE::VarKind::STATE { index: 1, .. },
            ..
        } => {
            let mut exp: metamodelica::Ref<DAE::Exp>;
            exp = Expression::crefExp(inVar.varName.clone())?;
            Expression::expDer(exp)
        }
        _ => Expression::crefExp(inVar.varName.clone())?,
    });
    Ok(outExp)
}

pub(crate) fn scalarizeVariables(mut vars: BackendDAE::Variables) -> Result<BackendDAE::Variables> {
    let mut vars: BackendDAE::Variables = vars;
    let mut var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut new_var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    var_lst = varList(&vars)?;
    for mut var in &*var_lst {
        new_var_lst = scalarizeVar(var.clone(), new_var_lst)?;
    }
    vars = listVar(new_var_lst.reverse())?;
    Ok(vars)
}

pub(crate) fn scalarizeVar(
    mut var: metamodelica::Ref<BackendDAE::Var>,
    mut scalar_vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut scalar_vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = scalar_vars;
    let mut scalar_crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut scalar_var: metamodelica::Ref<BackendDAE::Var>;
    if Types::isArray(&var.varType) {
        scalar_crefs = ComponentReference::expandCref(&var.varName, false)?;
        for mut cref in &*scalar_crefs {
            scalar_var = copyVarNewName(cref.clone(), var.clone());
            assign_field!(scalar_var.varType = ComponentReference::crefTypeFull(metamodelica::AsArg::as_arg(&cref))?);
            scalar_vars = metamodelica::cons(scalar_var, scalar_vars);
        }
    } else {
        scalar_vars = metamodelica::cons(var, scalar_vars);
    }
    Ok(scalar_vars)
}
