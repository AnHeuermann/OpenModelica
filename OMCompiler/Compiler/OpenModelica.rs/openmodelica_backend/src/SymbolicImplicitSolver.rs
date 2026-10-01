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
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util_datatypes_basic::List;

pub(crate) fn symSolver(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<Option<BackendDAE::InlineData>> {
    let mut inlineData: Option<BackendDAE::InlineData>;
    if Flags::getConfigEnum(Flags::SYM_SOLVER.clone())? > 0 {
        inlineData = Some(symSolverWork(inDAE)?);
    } else {
        inlineData = None;
    }
    Ok(inlineData)
}

fn symSolverWork(mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>) -> Result<BackendDAE::InlineData> {
    let mut inlineData: BackendDAE::InlineData;
    let mut osystlst: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut syst_: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut tmpv: metamodelica::Ref<BackendDAE::Var>;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut localInline: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut knownVariables: BackendDAE::Variables;
    let mut saveKnGlobalVars: BackendDAE::Variables;
    let mut inlineBDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut execbool: bool;
    localInline = BackendDAEUtil::copyEqSystems(&inDAE.eqs);
    knownVariables = BackendVariable::emptyVars(BackendDAEUtil::daeSize(inDAE)?);
    inlineData = BackendDAE::InlineData {
        inlineSystems: localInline,
        knownVariables: knownVariables,
    };
    cref = ComponentReferenceBasics::makeCrefIdent(
        arcstr::literal!(BackendDAE::symSolverDT),
        DAE::T_REAL_DEFAULT().clone(),
        metamodelica::nil(),
    );
    tmpv = BackendVariable::makeVar(cref)?;
    tmpv = BackendVariable::setBindExp(
        tmpv,
        Some(metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        })),
    );
    inlineData.knownVariables = BackendVariable::addVars(&(list![tmpv]), inlineData.knownVariables.clone())?;
    knownVariables = inlineData.knownVariables.clone();
    for mut syst in &*inlineData.inlineSystems.clone() {
        (syst_, knownVariables) = symSolverUpdateSyst(syst.clone(), knownVariables)?;
        osystlst = metamodelica::cons(syst_, osystlst);
    }
    inlineData.knownVariables = knownVariables.clone();
    shared = inDAE.shared.clone();
    saveKnGlobalVars = shared.globalKnownVars.clone();
    knownVariables = BackendVariable::addVariables(shared.globalKnownVars.clone(), knownVariables)?;
    assign_field!(
        shared.globalKnownVars = knownVariables,
        shared.backendDAEType = openmodelica_backend_types::BackendDAE::BackendDAEType::INLINESYSTEM
    );
    inlineBDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: osystlst,
        shared: shared.clone(),
    });
    execbool = FlagsUtil::disableDebug(Flags::EXEC_STAT.clone())?;
    if Flags::isSet(Flags::DUMP_INLINE_SOLVER.clone())? {
        BackendDump::bltdump(literal!("Generated inline system:"), &inlineBDAE)?;
    }
    inlineBDAE = BackendDAEUtil::getSolvedSystemforJacobians(
        inlineBDAE,
        &(list![
            literal!("removeEqualRHS"),
            literal!("removeSimpleEquations"),
            literal!("evalFunc")
        ]),
        None,
        None,
        &(list![
            literal!("inlineArrayEqn"),
            literal!("constantLinearSystem"),
            literal!("solveSimpleEquations"),
            literal!("tearingSystem"),
            literal!("calculateStrongComponentJacobians"),
            literal!("removeConstants"),
            literal!("simplifyTimeIndepFuncCalls")
        ]),
    )?;
    FlagsUtil::set(Flags::EXEC_STAT.clone(), execbool)?;
    if Flags::isSet(Flags::DUMP_INLINE_SOLVER.clone())? {
        BackendDump::bltdump(literal!("Final inline systems:"), &inlineBDAE)?;
    }
    if Flags::isSet(Flags::DUMP_BACKENDDAE_INFO.clone())?
        || Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())?
        || Flags::isSet(Flags::DUMP_DISCRETEVARS_INFO.clone())?
    {
        BackendDump::dumpCompShort(&inlineBDAE)?;
    }
    let __arc1 = inlineBDAE;
    let BackendDAE::DAE { eqs: __pa0, shared: _ } = &*__arc1;
    localInline = metamodelica::Own::own(__pa0);
    inlineData.inlineSystems = localInline;
    assign_field!(shared.globalKnownVars = saveKnGlobalVars);
    Ok(inlineData)
}

fn symSolverUpdateSyst(
    mut iSyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inKnVars: BackendDAE::Variables,
) -> Result<(metamodelica::Ref<BackendDAE::EqSystem>, BackendDAE::Variables)> {
    let mut oSyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oKnVars: BackendDAE::Variables = inKnVars.clone();
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut vars: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    oSyst = (::match_deref::match_deref! { match &(iSyst) {
        syst @ Deref @ BackendDAE::EqSystem { orderedVars: __esc_vars, orderedEqs: __esc_eqns, .. } => {
            vars = (*__esc_vars).clone();
            eqns = (*__esc_eqns).clone();
            let mut syst = (*syst).clone();
            crlst = metamodelica::nil();
            for mut i in 1..=ExpandableArray::getLastUsedIndex(eqns.clone()) {
                if ExpandableArray::occupied(i, eqns.clone()) {
                    eqn = ExpandableArray::get(i, eqns.clone())?;
                    let (__pa0, (__pa1, _)) = BackendEquation::traverseExpsOfEquation(eqn, (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>, __a1: (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, BackendDAE::Variables)| symSolverUpdateEqn(__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, BackendDAE::Variables)) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, BackendDAE::Variables))> + 'static>), (crlst, syst.orderedVars.clone()))?;
                    eqn = metamodelica::Own::own(__pa0);
                    crlst = metamodelica::Own::own(__pa1);
                    ExpandableArray::update(i, eqn, eqns.clone())?;
                }
            }
            (vars, oKnVars) = symSolverState(vars.clone(), inKnVars, &crlst)?;
            assign_field!(
                syst.orderedVars = vars.clone(),
                syst.orderedEqs = eqns.clone()
            );
            BackendDAEUtil::clearEqSyst(metamodelica::AsArg::as_arg(&syst))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oSyst, oKnVars))
}

// function changes every state variable to algebraic variable
fn symSolverState(
    mut vars: BackendDAE::Variables,
    mut knvars: BackendDAE::Variables,
    mut crlst: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(BackendDAE::Variables, BackendDAE::Variables)> {
    let mut ovars: BackendDAE::Variables = vars;
    let mut oknvars: BackendDAE::Variables = knvars;
    let mut idx: i32;
    let mut oldCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    for mut cref in &**crlst {
        (var, idx) = BackendVariable::getVar2(metamodelica::AsArg::as_arg(&cref), &ovars)?;
        ovars =
            BackendVariable::setVarKindForVar(idx, openmodelica_backend_types::BackendDAE::VarKind::ALG_STATE, ovars)?;
        oldCref = ComponentReference::appendStringLastIdent(&(literal!("$Old")), metamodelica::AsArg::as_arg(&cref))?;
        var = BackendVariable::copyVarNewName(oldCref, var);
        var = BackendVariable::setVarKind(var, openmodelica_backend_types::BackendDAE::VarKind::ALG_STATE_OLD)?;
        oknvars = BackendVariable::addVars(&(list![var]), oknvars)?;
    }
    Ok((ovars, oknvars))
}

fn symSolverUpdateEqn(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTl: &(
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        BackendDAE::Variables,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        BackendDAE::Variables,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        BackendDAE::Variables,
    );
    let mut orderedVars: BackendDAE::Variables;
    let mut inTpl: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (inTpl, orderedVars) = inTl.clone();
    if Flags::getConfigEnum(Flags::SYM_SOLVER.clone())? > 1 {
        let (__pa0, (__pa1, __pa2)) =
            Expression::traverseExpTopDown(inExp, &symSolverUpdateStates, (inTpl, orderedVars))?;
        outExp = metamodelica::Own::own(__pa0);
        inTpl = metamodelica::Own::own(__pa1);
        orderedVars = metamodelica::Own::own(__pa2);
    } else {
        (outExp, inTpl) = Expression::traverseExpTopDown(inExp, &symSolverUpdateDer, inTpl)?;
    }
    outTpl = (inTpl, orderedVars);
    Ok((outExp, outTpl))
}

fn symSolverUpdateStates(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTl: (
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        BackendDAE::Variables,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    (
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        BackendDAE::Variables,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool = true;
    let mut outTl: (
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        BackendDAE::Variables,
    );
    let mut inTpl: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut orderedVars: BackendDAE::Variables;
    (inTpl, orderedVars) = inTl.clone();
    (outExp, outTl) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1 @ Deref @ DAE::Exp::CREF { ty: tp, componentRef: cr }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut cr_lst = inTpl;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut e3: metamodelica::Ref<DAE::Exp>;
            e2 = Expression::crefExp(ComponentReference::appendStringLastIdent(&(literal!("$Old")), metamodelica::AsArg::as_arg(&cr))?)?;
            e3 = Expression::crefExp(ComponentReferenceBasics::makeCrefIdent(arcstr::literal!(BackendDAE::symSolverDT), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil()))?;
            cont = false;
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: e2 }), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: e3 }), (List::unionElt(cr.clone(), cr_lst), orderedVars))
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut cr_lst = inTpl;
            let mut e: metamodelica::Ref<DAE::Exp>;
            (e, cr_lst) = symSolverAppendStringToStates(cr.clone(), cr_lst, &orderedVars)?;
            (e, (cr_lst, orderedVars))
        },
        _ => {
            (inExp, inTl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTl))
}

fn symSolverAppendStringToStates(
    mut inCr: metamodelica::Ref<DAE::ComponentRef>,
    mut incr_lst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut orderedVars: &BackendDAE::Variables,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = Expression::crefExp(inCr.clone())?;
    let mut outcr_lst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = incr_lst.clone();
    if BackendVariable::isState(inCr.clone(), orderedVars) {
        outExp = Expression::crefExp(ComponentReference::appendStringLastIdent(&(literal!("$Old")), &inCr)?)?;
        outcr_lst = List::unionElt(inCr, incr_lst);
    }
    Ok((outExp, outcr_lst))
}

// function changes call "der" to difference quotient
fn symSolverUpdateDer(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool = true;
    let mut outTpl: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (outExp, outTpl) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1 @ Deref @ DAE::Exp::CREF { ty: tp, componentRef: cr }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut cr_lst = inTpl.clone();
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut e3: metamodelica::Ref<DAE::Exp>;
            e2 = Expression::crefExp(ComponentReference::appendStringLastIdent(&(literal!("$Old")), metamodelica::AsArg::as_arg(&cr))?)?;
            e3 = Expression::crefExp(ComponentReferenceBasics::makeCrefIdent(arcstr::literal!(BackendDAE::symSolverDT), DAE::T_REAL_DEFAULT().clone(), metamodelica::nil()))?;
            (metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: tp.clone() }, exp2: e2 }), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: e3 }), List::unionElt(cr.clone(), cr_lst))
        },
        _ => {
            (inExp, inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}
