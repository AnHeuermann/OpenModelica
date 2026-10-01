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
use crate::BackendVariable;
use crate::Differentiate;
use crate::ExpressionSolve;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Config;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util_datatypes_basic::List;

pub(crate) fn createDynamicOptimization(
    mut dae: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut dae: metamodelica::Ref<BackendDAE::BackendDAE> = dae;
    let mut vars: BackendDAE::Variables;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    shared = dae.shared.clone();
    let __pa0 = ::match_deref::match_deref! { match &(dae.eqs.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    syst = metamodelica::Own::own(__pa0);
    let __arc4 = syst.clone();
    let BackendDAE::EQSYSTEM {
        orderedVars: __pa2,
        orderedEqs: __pa3,
        ..
    } = &*__arc4;
    vars = metamodelica::Own::own(__pa2);
    eqns = metamodelica::Own::own(__pa3);
    (vars, eqns, shared) = addOptimizationVarsEqns(vars, eqns, shared)?;
    assign_field!(syst.orderedVars = vars, syst.orderedEqs = eqns);
    assign_field!(dae.eqs = list![syst], dae.shared = shared);
    Ok(dae)
}

fn addOptimizationVarsEqns(
    mut vars: BackendDAE::Variables,
    mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut shared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    BackendDAE::Variables,
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut vars: BackendDAE::Variables = vars;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> = eqns;
    let mut shared: metamodelica::Ref<BackendDAE::Shared> = shared;
    let mut mayer: Option<metamodelica::Ref<DAE::Exp>>;
    let mut lagrange: Option<metamodelica::Ref<DAE::Exp>>;
    let mut startTimeE: Option<metamodelica::Ref<DAE::Exp>>;
    let mut finalTimeE: Option<metamodelica::Ref<DAE::Exp>>;
    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut eqnsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut classAttrs: metamodelica::List<metamodelica::Ref<DAE::ClassAttributes>>;
    let mut constraints: metamodelica::List<metamodelica::Ref<DAE::Constraint>>;
    let mut globalKnownVars: BackendDAE::Variables;
    let mut inOptimicaFlag: bool = Config::acceptOptimicaGrammar()?;
    let mut inDynOptimization: bool = Flags::getConfigBool(Flags::GENERATE_DYN_OPTIMIZATION_PROBLEM.clone())?;
    let debug: bool = false;
    classAttrs = shared.classAttrs.clone();
    constraints = shared.constraints.clone();
    globalKnownVars = shared.globalKnownVars.clone();
    eqnsLst = metamodelica::nil();
    if !(inOptimicaFlag || inDynOptimization) {
        metamodelica::print(literal!(
            "Something going wrong for postOptModul=createDynamicOptimization. Check your flags. You need -g=DynOpt or -g=Optimica!\n"
        ));
        return Err("fail");
    }
    FlagsUtil::setConfigEnum(Flags::GRAMMAR.clone(), Flags::OPTIMICA.clone())?;
    (mayer, lagrange, startTimeE, finalTimeE) = getOptimicaArgs(&classAttrs);
    varlst = BackendVariable::varList(&globalKnownVars)?;
    addTimeGrid(varlst.clone(), globalKnownVars.clone())?;
    varlst = listAppend(varlst, BackendVariable::varList(&vars)?);
    (vars, eqnsLst, mayer) = joinObjectFun(
        &(makeObject(
            arcstr::literal!(BackendDAE::optimizationMayerTermName),
            &findMayerTerm,
            varlst.clone(),
            mayer,
        )?),
        vars,
        eqnsLst,
    )?;
    (vars, eqnsLst, lagrange) = joinObjectFun(
        &(makeObject(
            arcstr::literal!(BackendDAE::optimizationLagrangeTermName),
            &findLagrangeTerm,
            varlst.clone(),
            lagrange,
        )?),
        vars,
        eqnsLst,
    )?;
    (vars, eqnsLst) = joinConstraints(
        &constraints,
        &(literal!("$con$")),
        openmodelica_backend_types::BackendDAE::VarKind::OPT_CONSTR,
        globalKnownVars.clone(),
        varlst.clone(),
        vars,
        eqnsLst,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendVariable::hasConTermAnno(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>),
    )?;
    (vars, eqnsLst) = joinConstraints(
        &(metamodelica::nil()),
        &(literal!("$finalCon$")),
        openmodelica_backend_types::BackendDAE::VarKind::OPT_FCONSTR,
        globalKnownVars,
        varlst,
        vars,
        eqnsLst,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendVariable::hasFinalConTermAnno(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>),
    )?;
    FlagsUtil::setConfigBool(Flags::GENERATE_SYMBOLIC_LINEARIZATION.clone(), true)?;
    assign_field!(
        shared.classAttrs = list![metamodelica::Ref::new(DAE::ClassAttributes {
            objetiveE: mayer,
            objectiveIntegrandE: lagrange,
            startTimeE: startTimeE,
            finalTimeE: finalTimeE
        })]
    );
    if debug {
        metamodelica::print(literal!("\neqs"));
        BackendDump::printEquationList(&eqnsLst)?;
    }
    eqns = BackendEquation::addList(&eqnsLst, eqns)?;
    Ok((vars, eqns, shared))
}

fn getOptimicaArgs(
    mut inClassAttr: &metamodelica::List<metamodelica::Ref<DAE::ClassAttributes>>,
) -> (
    Option<metamodelica::Ref<DAE::Exp>>,
    Option<metamodelica::Ref<DAE::Exp>>,
    Option<metamodelica::Ref<DAE::Exp>>,
    Option<metamodelica::Ref<DAE::Exp>>,
) {
    let mut mayer: Option<metamodelica::Ref<DAE::Exp>>;
    let mut lagrange: Option<metamodelica::Ref<DAE::Exp>>;
    let mut startTimeE: Option<metamodelica::Ref<DAE::Exp>>;
    let mut finalTimeE: Option<metamodelica::Ref<DAE::Exp>>;
    (mayer, lagrange, startTimeE, finalTimeE) = (::match_deref::match_deref! { match inClassAttr {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::ClassAttributes { objetiveE: mayer_, objectiveIntegrandE: lagrange_, startTimeE: startTimeE_, finalTimeE: finalTimeE_ }, tail: Deref @ metamodelica::ListNode::Nil } => {
            (mayer_.clone(), lagrange_.clone(), startTimeE_.clone(), finalTimeE_.clone())
        },
        _ => {
            (None, None, None, None)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (mayer, lagrange, startTimeE, finalTimeE)
}

fn addTimeGrid(
    mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut iv: BackendDAE::Variables,
) -> Result<BackendDAE::Variables> {
    let mut ov: BackendDAE::Variables = iv;
    let mut tG: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = findTimeGrid(varlst.clone())?;
    let mut ind: metamodelica::List<i32>;
    if !((tG).is_empty()) {
        ind = BackendVariable::getVarIndexFromVars(&tG, &ov);
        for mut i in &*ind {
            ov = BackendVariable::setVarKindForVar(
                i.clone(),
                openmodelica_backend_types::BackendDAE::VarKind::OPT_TGRID,
                ov,
            )?;
        }
    }
    Ok(ov)
}

fn joinConstraints(
    mut inConstraint: &metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    mut name: &ArcStr,
    mut conKind: BackendDAE::VarKind,
    mut globalKnownVars: BackendDAE::Variables,
    mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut vars: BackendDAE::Variables,
    mut e: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut findCon: Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>,
) -> Result<(
    BackendDAE::Variables,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>;

    let mut ovars: BackendDAE::Variables;
    let mut oe: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut constraints: metamodelica::List<metamodelica::Ref<DAE::Constraint>>;
    constraints = addConstraints(varlst, inConstraint, findCon.clone())?;
    (ovars, oe) = addOptimizationVarsEqns2(&constraints, 1, vars, e, globalKnownVars, name, conKind)?;
    Ok((ovars, oe))
}

fn joinObjectFun(
    mut obj: &(
        metamodelica::Ref<BackendDAE::Var>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        Option<metamodelica::Ref<DAE::Exp>>,
    ),
    mut vars: BackendDAE::Variables,
    mut e: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<(
    BackendDAE::Variables,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    Option<metamodelica::Ref<DAE::Exp>>,
)> {
    let mut ovars: BackendDAE::Variables;
    let mut oe: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut objExp: Option<metamodelica::Ref<DAE::Exp>>;
    (ovars, oe, objExp) = (::match_deref::match_deref! { match &(obj) {
        (_, Deref @ metamodelica::ListNode::Nil, _) => {
            (vars, e, None)
        },
        (v, e_, e1) => {
            (BackendVariable::addNewVar(v.clone(), vars)?, listAppend(e_.clone(), e), e1.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((ovars, oe, objExp))
}

fn makeObject(
    mut name: ArcStr,
    mut findObj: &dyn ::std::ops::Fn(
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    ) -> Result<Option<metamodelica::Ref<DAE::Exp>>>,
    mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut optimicaExp: Option<metamodelica::Ref<DAE::Exp>>,
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    Option<metamodelica::Ref<DAE::Exp>>,
)> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            ) -> Result<Option<metamodelica::Ref<DAE::Exp>>>
            + 'static,
    >;

    let mut outTpl: (
        metamodelica::Ref<BackendDAE::Var>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        Option<metamodelica::Ref<DAE::Exp>>,
    );
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut annoObj: Option<metamodelica::Ref<DAE::Exp>>;
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    let mut e: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    (cr, v) = makeVar(name);
    annoObj = findObj(varlst)?;
    annoObj = mergeObjectVars(annoObj, optimicaExp)?;
    e = BackendEquation::generateSolvedEqnsfromOption(
        cr,
        annoObj.clone(),
        DAE::emptyElementSource().clone(),
        BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone(),
    );
    outTpl = (v, e, annoObj);
    Ok(outTpl)
}

fn makeVar(mut name: ArcStr) -> (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<BackendDAE::Var>) {
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut v: metamodelica::Ref<BackendDAE::Var>;
    cr = ComponentReferenceBasics::makeCrefIdent(name, DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
    v = metamodelica::Ref::new(BackendDAE::Var {
        varName: cr.clone(),
        varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE,
        varDirection: openmodelica_frontend_types::DAE::VarDirection::OUTPUT,
        varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
        varType: DAE::T_REAL_DEFAULT().clone(),
        bindExp: None,
        tplExp: None,
        arryDim: metamodelica::nil(),
        source: DAE::emptyElementSource().clone(),
        values: None,
        tearingSelectOption: Some(openmodelica_backend_types::BackendDAE::TearingSelect::AVOID),
        hideResult: None,
        comment: None,
        connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
        innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER,
        unreplaceable: false,
        initNonlinear: false,
        encrypted: false,
    });
    (cr, v)
}

fn addOptimizationVarsEqns1(
    mut constraintLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inI: i32,
    mut inVars: BackendDAE::Variables,
    mut inEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut globalKnownVars: BackendDAE::Variables,
    mut prefConCrefName: &ArcStr,
    mut conKind: BackendDAE::VarKind,
) -> Result<(
    BackendDAE::Variables,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut outVars: BackendDAE::Variables = inVars;
    let mut outEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = inEqns;
    let mut i: i32 = inI;
    let mut dummyVar: metamodelica::Ref<BackendDAE::Var>;
    let mut conEqn: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut conCrefName: ArcStr;
    for mut elem in &**constraintLst {
        match '__try0: {
            conCrefName = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*prefConCrefName);
                __mm_s.push_str(&*unwrap_break_err!(ComponentReferenceBasics::printComponentRefStr(&(unwrap_break_err!(Expression::expCref(metamodelica::AsArg::as_arg(&elem)), '__try0))), '__try0));
                ArcStr::from(__mm_s)
            };
            Ok::<_, &'static str>((conCrefName.clone(),))
        } {
            Ok((__try0_o0,)) => {
                conCrefName = __try0_o0;
            }
            Err(_) => {
                conCrefName = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*prefConCrefName);
                    __mm_s.push_str(&*intString(i));
                    ArcStr::from(__mm_s)
                };
                i = i + 1;
            }
        }
        (conEqn, dummyVar) = BackendEquation::generateResidualFromRelation(
            conCrefName.clone(),
            elem.clone(),
            DAE::emptyElementSource().clone(),
            &outVars,
            globalKnownVars.clone(),
            conKind.clone(),
        )?;
        outVars = BackendVariable::addNewVar(dummyVar, outVars)?;
        outEqns = listAppend(conEqn, outEqns);
    }
    Ok((outVars, outEqns))
}

fn addOptimizationVarsEqns2(
    mut inConstraint: &metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    mut inI: i32,
    mut inVars: BackendDAE::Variables,
    mut inEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut globalKnownVars: BackendDAE::Variables,
    mut prefConCrefName: &ArcStr,
    mut conKind: BackendDAE::VarKind,
) -> Result<(
    BackendDAE::Variables,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut outVars: BackendDAE::Variables;
    let mut outEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    (outVars, outEqns) = (::match_deref::match_deref! { match inConstraint {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Constraint::CONSTRAINT_EXPS { constraintLst }, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut e: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut v: BackendDAE::Variables;
            (v, e) = addOptimizationVarsEqns1(metamodelica::AsArg::as_arg(&constraintLst), inI, inVars, inEqns, globalKnownVars, prefConCrefName, conKind)?;
            (v, e)
        },
        _ => {
            (inVars, inEqns)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outVars, outEqns))
}

fn findMayerTerm(
    mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut mayer: Option<metamodelica::Ref<DAE::Exp>> = findObjTerm(
        varlst.clone(),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendVariable::hasMayerTermAnno(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>),
    )?;
    Ok(mayer)
}

fn findLagrangeTerm(
    mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut lagrange: Option<metamodelica::Ref<DAE::Exp>> = findObjTerm(
        varlst.clone(),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendVariable::hasLagrangeTermAnno(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>),
    )?;
    Ok(lagrange)
}

fn findTimeGrid(
    mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut timeGrids: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = List::select(
        varlst.clone(),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendVariable::hasTimeGridAnno(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>),
    )?;
    Ok(timeGrids)
}

fn findObjTerm(
    mut InVarlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut findObjTermFun: Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>;

    let mut objeExp: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut nom: metamodelica::Ref<DAE::Exp>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> =
        List::select(InVarlst.clone(), findObjTermFun.clone())?;
    for mut v in &*varlst {
        nom = BackendVariable::getVarNominalValue(metamodelica::AsArg::as_arg(&v));
        cr = BackendVariable::varCref(metamodelica::AsArg::as_arg(&v));
        e = metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: cr,
            ty: DAE::T_REAL_DEFAULT().clone(),
        });
        e = Expression::expDiv(e, nom)?;
        objeExp = mergeObjectVars(objeExp, Some(e))?;
    }
    Ok(objeExp)
}

fn mergeObjectVars(
    mut inmayer1: Option<metamodelica::Ref<DAE::Exp>>,
    mut inmayer2: Option<metamodelica::Ref<DAE::Exp>>,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut mayer: Option<metamodelica::Ref<DAE::Exp>>;
    mayer = (::match_deref::match_deref! { match &((inmayer1.clone(), inmayer2.clone())) {
        (Some(e1), Some(e2)) => {
            let mut e3: metamodelica::Ref<DAE::Exp>;
            e3 = Expression::expAdd(e1.clone(), e2.clone())?;
            Some(e3)
        },
        (None, Some(_)) => {
            inmayer2
        },
        (_, None) => {
            inmayer1
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(mayer)
}

fn addConstraints(
    mut InVarlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inConstraint: &metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    mut findCon: Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Constraint>>> {
    pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>;

    let mut outConstraint: metamodelica::List<metamodelica::Ref<DAE::Constraint>>;
    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut constraintLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    constraintLst = (::match_deref::match_deref! { match inConstraint {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Constraint::CONSTRAINT_EXPS { constraintLst: constraintLst_ }, tail: Deref @ metamodelica::ListNode::Nil } => {
            constraintLst_.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    varlst = List::select(InVarlst, findCon.clone())?;
    constraintLst = addConstraints2(constraintLst, &varlst);
    outConstraint = list![metamodelica::Ref::new(DAE::Constraint::CONSTRAINT_EXPS {
        constraintLst: constraintLst
    })];
    Ok(outConstraint)
}

fn addConstraints2(
    mut inConstraintLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inVarlst: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> metamodelica::List<metamodelica::Ref<DAE::Exp>> {
    let mut outConstraintLst: metamodelica::List<metamodelica::Ref<DAE::Exp>> = inConstraintLst;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    for mut v in &**inVarlst {
        cr = BackendVariable::varCref(metamodelica::AsArg::as_arg(&v));
        e = metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: cr,
            ty: DAE::T_REAL_DEFAULT().clone(),
        });
        outConstraintLst = metamodelica::cons(e, outConstraintLst);
    }
    outConstraintLst
}

// =============================================================================
// section for preOptModule >>inputDerivativesForDynOpt<<
//
// check for derivatives of inputs and replace (only for dyn. optimization)
// =============================================================================
pub(crate) fn inputDerivativesForDynOpt(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    if Config::acceptOptimicaGrammar()? || Flags::getConfigBool(Flags::GENERATE_DYN_OPTIMIZATION_PROBLEM.clone())? {
        (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(
            &inDAE,
            &fnptr!(
                inputDerivativesForDynOptWork,
                metamodelica::Ref<BackendDAE::EqSystem>,
                metamodelica::Ref<BackendDAE::Shared>,
                bool
            ),
            false,
        )?;
    } else {
        outDAE = inDAE;
    }
    Ok(outDAE)
}

fn inputDerivativesForDynOptWork(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inChanged: bool,
) -> (
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    bool,
) {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared;
    let mut outChanged: bool;
    (osyst, outChanged) = ({
        let mut idercr: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
        let mut icr: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
        let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
        'mc: {
            let __mc_input = &*isyst;
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ BackendDAE::EqSystem { orderedEqs, .. } => {
                        let mut vars: BackendDAE::Variables;
                        let mut outShared: metamodelica::Ref<BackendDAE::Shared> = outShared.clone();
                        vars = BackendVariable::daeGlobalKnownVars(&outShared);
                        (_, idercr, icr, varLst) = BackendDAEUtil::traverseBackendDAEExpsEqns(orderedEqs.clone(), (std::sync::Arc::new(traverserinputDerivativesForDynOpt) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>)) -> Result<(metamodelica::Ref<DAE::Exp>, (BackendDAE::Variables, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, metamodelica::List<metamodelica::Ref<BackendDAE::Var>>))> + 'static>), (vars.clone(), idercr.clone(), icr.clone(), varLst.clone()))?;
                        if (idercr).is_empty() {
                            return Err("fail");
                        }
                        varLst = BackendVariable::setVarsKind(varLst.clone(), openmodelica_backend_types::BackendDAE::VarKind::OPT_INPUT_WITH_DER)?;
                        for mut v in &*varLst {
                            outShared = BackendVariable::addGlobalKnownVarDAE(v.clone(), outShared.clone())?;
                        }
                        varLst = List::map(idercr.clone(), &BackendVariable::makeVar)?;
                        varLst = List::map1(varLst.clone(), &fnptr!(BackendVariable::setVarDirection, metamodelica::Ref<BackendDAE::Var>, DAE::VarDirection), openmodelica_frontend_types::DAE::VarDirection::INPUT)?;
                        for mut v in &*varLst {
                            let mut v = v.clone();
                            v = BackendVariable::setVarKind(v.clone(), openmodelica_backend_types::BackendDAE::VarKind::OPT_INPUT_DER)?;
                            outShared = BackendVariable::addGlobalKnownVarDAE(v.clone(), outShared.clone())?;
                        }
                        Ok(((isyst.clone(), true), outShared.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                outShared = __wb0;
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        Ok((isyst.clone(), inChanged))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            panic!("matchcontinue: no arm matched")
        }
    });
    (osyst, outShared, outChanged)
}

fn traverserinputDerivativesForDynOpt(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut itpl: (
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    ),
)> {
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut tpl: (
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    );
    (e, tpl) = Expression::traverseExpTopDown(
        inExp,
        &fnptr!(
            traverserExpinputDerivativesForDynOpt,
            metamodelica::Ref<DAE::Exp>,
            (
                BackendDAE::Variables,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<BackendDAE::Var>>
            )
        ),
        itpl,
    )?;
    Ok((e, tpl))
}

fn traverserExpinputDerivativesForDynOpt(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut tpl: (
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    bool,
    (
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (
        BackendDAE::Variables,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    );
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (&*inExp, &tpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (vars, lst, lst1, varLst)) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    (var, _) = BackendVariable::getVarSingle(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&vars))?;
                    let true = (BackendVariable::isVarOnTopLevelAndInput(&var)) else { return Err("pattern mismatch") };
                    var = BackendVariable::setHideResult(var.clone(), Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: true })));
                    cr1 = ComponentReference::prependStringCref(literal!("$TMP$DER$P"), metamodelica::AsArg::as_arg(&cr))?;
                    e = Expression::crefExp(cr1.clone())?;
                    Ok((e.clone(), true, (vars.clone(), List::unionElt(cr1.clone(), lst.clone()), List::unionElt(cr.clone(), lst1.clone()), List::unionElt(var.clone(), varLst.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), true, tpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, cont, outTpl)
}

// =============================================================================
// section for postOptModule >>extendDynamicOptimization<<
//
// transform loops from DAE in constraints for optimizer
// - bigger NLP
// - don't solve loop in each step
// - cheaper jacobians
// =============================================================================
pub(crate) fn removeLoops(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    if !(metamodelica::stringEq(&(Flags::getConfigString(Flags::LOOP2CON.clone())?), &(literal!("none")))) {
        (outDAE, _) = BackendDAEUtil::mapEqSystemAndFold(&inDAE, &findLoops, false)?;
    } else {
        outDAE = inDAE;
    }
    Ok(outDAE)
}

fn findLoops(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inChanged: bool,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    bool,
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut outShared: metamodelica::Ref<BackendDAE::Shared>;
    let mut outChanged: bool;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let __pa0 = ::match_deref::match_deref! { match &(isyst.clone()) {
        Deref @ BackendDAE::EqSystem { matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa0, .. }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    comps = metamodelica::Own::own(__pa0);
    (osyst, outShared, outChanged) = findLoops1(isyst, inShared, &comps, inChanged)?;
    Ok((osyst, outShared, outChanged))
}

fn findLoops1(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut inComps: &metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>,
    mut inchanged: bool,
) -> Result<(
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
    bool,
)> {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem> = isyst;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared> = ishared;
    let mut changed: bool = inchanged;
    let mut l2p_all: bool =
        metamodelica::stringEq(&(Flags::getConfigString(Flags::LOOP2CON.clone())?), &(literal!("all")));
    let mut l2p_nl: bool;
    let mut l2p_l: bool;
    if l2p_all {
        l2p_l = true;
    } else {
        l2p_nl = metamodelica::stringEq(
            &(Flags::getConfigString(Flags::LOOP2CON.clone())?),
            &(literal!("noLin")),
        );
        l2p_l = !(l2p_nl);
    }
    for mut comp in &**inComps {
        (osyst, oshared) = removeLoopsWork(osyst, oshared, metamodelica::AsArg::as_arg(&comp), l2p_all, l2p_l);
    }
    Ok((osyst, oshared, changed))
}

fn removeLoopsWork(
    mut isyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
    mut icomp: &metamodelica::Ref<BackendDAE::StrongComponent>,
    mut l2p_all: bool,
    mut l2p_l: bool,
) -> (
    metamodelica::Ref<BackendDAE::EqSystem>,
    metamodelica::Ref<BackendDAE::Shared>,
) {
    let mut osyst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oshared: metamodelica::Ref<BackendDAE::Shared>;
    (osyst, oshared) = 'mc: {
        let __mc_input = (isyst.clone(), ishared.clone(), &**icomp);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. }, shared, Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { eqns: eindex, vars: vindx, jacType, .. }) => {
                    if !((l2p_all || if (l2p_l) {isConstOrlinear(jacType.clone())} else {!(isConstOrlinear(jacType.clone()))})) { return Err("guard") }
                    let mut syst = (*syst).clone();
                    let mut vars = (*vars).clone();
                    let mut eqns = (*eqns).clone();
                    let mut shared = (*shared).clone();
                    (eqns, vars, shared) = res2Con(eqns.clone(), vars.clone(), eindex.clone(), vindx.clone(), shared.clone())?;
                    assign_field!(
                        syst.orderedEqs = eqns.clone(),
                        syst.orderedVars = vars.clone()
                    );
                    Ok((BackendDAEUtil::clearEqSyst(metamodelica::AsArg::as_arg(&syst)), shared.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. }, shared, Deref @ BackendDAE::StrongComponent::TORNSYSTEM { strictTearingSet: BackendDAE::TearingSet { residualequations: eindex, tearingvars: vindx, .. }, linear, .. }) => {
                    if !((l2p_all || if (l2p_l) {linear.clone()} else {!(linear.clone())})) { return Err("guard") }
                    let mut syst = (*syst).clone();
                    let mut vars = (*vars).clone();
                    let mut eqns = (*eqns).clone();
                    let mut shared = (*shared).clone();
                    (eqns, vars, shared) = res2Con(eqns.clone(), vars.clone(), eindex.clone(), vindx.clone(), shared.clone())?;
                    assign_field!(
                        syst.orderedEqs = eqns.clone(),
                        syst.orderedVars = vars.clone()
                    );
                    Ok((BackendDAEUtil::clearEqSyst(metamodelica::AsArg::as_arg(&syst)), shared.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (syst @ Deref @ BackendDAE::EqSystem { orderedVars: vars, orderedEqs: eqns, .. }, shared @ Deref @ BackendDAE::Shared { functionTree: funcs, .. }, Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: eindex_, var: vindx_ }) => {
                    if !((l2p_all || !(l2p_l))) { return Err("guard") }
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut varexp: metamodelica::Ref<DAE::Exp>;
                    let mut v: metamodelica::Ref<BackendDAE::Var>;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut syst = (*syst).clone();
                    let mut vars = (*vars).clone();
                    let mut eqns = (*eqns).clone();
                    let mut shared = (*shared).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendEquation::get(eqns.clone(), eindex_.clone())?) {
                        Deref @ BackendDAE::Equation::EQUATION { exp: __pa0, scalar: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1 = metamodelica::Own::own(__pa0);
                    e2 = metamodelica::Own::own(__pa1);
                    let __arc4 = BackendVariable::getVarAt(metamodelica::AsArg::as_arg(&vars), vindx_.clone())?;
                    let __pa3 = (__arc4).clone();
                    let BackendDAE::VAR { varName: __pa2, .. } = &*__arc4;
                    cr = metamodelica::Own::own(__pa2);
                    v = metamodelica::Own::own(__pa3);
                    varexp = Expression::crefExp(cr.clone())?;
                    varexp = if (BackendVariable::isStateVar(&v)) {Expression::expDer(varexp.clone())} else {varexp.clone()};
                    if '__try5: {
                        unwrap_break_err!(ExpressionSolve::solve2(e1.clone(), e2.clone(), varexp.clone(), Some(funcs.clone()), None, true, false), '__try5);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    (eqns, vars, shared) = res2Con(eqns.clone(), vars.clone(), list![eindex_.clone()], list![vindx_.clone()], shared.clone())?;
                    assign_field!(
                        syst.orderedEqs = eqns.clone(),
                        syst.orderedVars = vars.clone()
                    );
                    Ok((BackendDAEUtil::clearEqSyst(metamodelica::AsArg::as_arg(&syst)), shared.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((isyst.clone(), ishared.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (osyst, oshared)
}

fn isConstOrlinear(mut jacType: BackendDAE::JacobianType) -> bool {
    let mut b: bool;
    b = (match jacType {
        BackendDAE::JacobianType::JAC_CONSTANT { .. } => true,
        BackendDAE::JacobianType::JAC_LINEAR { .. } => true,
        _ => false,
    });
    b
}

fn res2Con(
    mut ieqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut ivars: BackendDAE::Variables,
    mut eindex: metamodelica::List<i32>,
    mut vindx: metamodelica::List<i32>,
    mut ishared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    BackendDAE::Variables,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut oeqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> =
        ieqns.clone();
    let mut ovars: BackendDAE::Variables = ivars.clone();
    let mut oshared: metamodelica::Ref<BackendDAE::Shared> = ishared;
    let mut eqn_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> =
        BackendEquation::getList(eindex.clone(), ieqns.clone())?;
    let mut eqn: metamodelica::Ref<BackendDAE::Equation>;
    let mut var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = List::map1r(
        vindx.clone(),
        &move |__a0: BackendDAE::Variables, __a1: i32| BackendVariable::getVarAt(&__a0, __a1),
        ivars.clone(),
    )?;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut var_: metamodelica::Ref<BackendDAE::Var> =
        <metamodelica::Ref<BackendDAE::Var> as ::std::default::Default>::default();
    let mut cr_lst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = List::map(
        var_lst.clone(),
        &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(BackendVariable::varCref(&__a0))
        },
    )?;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut cr_var: metamodelica::Ref<DAE::ComponentRef>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut res: metamodelica::Ref<DAE::Exp>;
    let mut ind_e: i32;
    let mut ind_v: i32;
    let mut ind_lst_v: metamodelica::List<i32> = List::map(vindx.clone(), &fnptr!(intAbs, i32))?;
    let mut ind_lst_e: metamodelica::List<i32> = eindex.clone();
    let mut globalKnownVars: BackendDAE::Variables;
    let __arc1 = oshared.clone();
    let BackendDAE::SHARED {
        globalKnownVars: __pa0, ..
    } = &*__arc1;
    globalKnownVars = metamodelica::Own::own(__pa0);
    for mut var_ in &*var_lst {
        let mut var_ = var_.clone();
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(cr_lst) {
            Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cr_var = metamodelica::Own::own(__pa2);
        cr_lst = metamodelica::Own::own(__pa3);
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(eqn_lst) {
            Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => return Err("pattern mismatch"),
        } };
        eqn = metamodelica::Own::own(__pa4);
        eqn_lst = metamodelica::Own::own(__pa5);
        let (__pa6, __pa7) = ::match_deref::match_deref! { match &(ind_lst_e) {
            Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: __pa7 } => (__pa6.clone(), __pa7.clone()),
            _ => return Err("pattern mismatch"),
        } };
        ind_e = metamodelica::Own::own(__pa6);
        ind_lst_e = metamodelica::Own::own(__pa7);
        let (__pa8, __pa9) = ::match_deref::match_deref! { match &(ind_lst_v) {
            Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: __pa9 } => (__pa8.clone(), __pa9.clone()),
            _ => return Err("pattern mismatch"),
        } };
        ind_v = metamodelica::Own::own(__pa8);
        ind_lst_v = metamodelica::Own::own(__pa9);
        cr = ComponentReferenceBasics::makeCrefIdent(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("$EqCon$"));
                __mm_s.push_str(&*ComponentReference::crefModelicaStr(&cr_var));
                ArcStr::from(__mm_s)
            },
            DAE::T_REAL_DEFAULT().clone(),
            metamodelica::nil(),
        );
        e = Expression::crefExp(cr.clone())?;
        var = BackendVariable::makeVar(cr)?;
        var = BackendVariable::setVarMinMax(
            var,
            Some(metamodelica::Ref::new(DAE::Exp::RCONST {
                real: metamodelica::OrderedFloat(0.0_f64),
            })),
            Some(metamodelica::Ref::new(DAE::Exp::RCONST {
                real: metamodelica::OrderedFloat(0.0_f64),
            })),
        )?;
        var = BackendVariable::setVarKind(var, openmodelica_backend_types::BackendDAE::VarKind::OPT_CONSTR)?;
        var = BackendVariable::setVarDirection(var, openmodelica_frontend_types::DAE::VarDirection::OUTPUT);
        ovars = BackendVariable::addNewVar(var, ovars)?;
        res = BackendDAEOptimize::makeEquationToResidualExp(&eqn)?;
        res = Expression::createResidualExp(res.clone(), Expression::makeConstZeroE(res)?)?;
        oeqns = BackendEquation::setAtIndex(
            oeqns,
            ind_e,
            metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
                exp: e,
                scalar: res,
                source: DAE::emptyElementSource().clone(),
                attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone(),
            }),
        )?;
        (cr, var) = makeVar({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("$"));
            __mm_s.push_str(&*ComponentReference::crefModelicaStr(&cr_var));
            ArcStr::from(__mm_s)
        });
        var = BackendVariable::setVarDirection(var, openmodelica_frontend_types::DAE::VarDirection::INPUT);
        e = Expression::crefExp(cr_var.clone())?;
        if BackendVariable::isStateVar(&var_) {
            e = Expression::expDer(e);
            var = BackendVariable::setVarKind(
                var,
                BackendDAE::VarKind::OPT_LOOP_INPUT {
                    replaceExp: ComponentReference::crefPrefixDer(cr_var),
                },
            )?;
        } else {
            var = BackendVariable::mergeAliasVars(var, var_, false, globalKnownVars.clone())?;
            var = BackendVariable::setVarKind(var, BackendDAE::VarKind::OPT_LOOP_INPUT { replaceExp: cr_var })?;
        }
        oshared = BackendVariable::addGlobalKnownVarDAE(var, oshared)?;
        oeqns = BackendEquation::add(
            metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
                exp: e,
                scalar: Expression::crefExp(cr)?,
                source: DAE::emptyElementSource().clone(),
                attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone(),
            }),
            oeqns,
        )?;
    }
    Ok((oeqns, ovars, oshared))
}

// =============================================================================
// section for postOptModule >>simplifyConstraints<<
//
// simplify nonlinear constraints if possible in  box constraints
// =============================================================================
pub(crate) fn simplifyConstraints(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut systlst: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut new_systlst: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut eqn_: metamodelica::Ref<BackendDAE::Equation> =
        openmodelica_backend_types::BackendDAE::Equation::interned_DUMMY_EQUATION();
    let mut var_: metamodelica::Ref<BackendDAE::Var>;
    let mut var_con: metamodelica::Ref<BackendDAE::Var>;
    let mut comps: metamodelica::List<metamodelica::Ref<BackendDAE::StrongComponent>>;
    let mut eindex: i32;
    let mut vindx: i32;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut var_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut var_lst_opt: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut var_lst1: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp> = metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 });
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut c: metamodelica::Ref<DAE::Exp>;
    let mut vars: BackendDAE::Variables;
    let mut globalKnownVars: BackendDAE::Variables;
    let mut eqns: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut oMax_con: Option<metamodelica::Ref<DAE::Exp>>;
    let mut oMin_con: Option<metamodelica::Ref<DAE::Exp>>;
    let mut max_con: metamodelica::Ref<DAE::Exp>;
    let mut min_con: metamodelica::Ref<DAE::Exp>;
    let mut zero: metamodelica::Ref<DAE::Exp>;
    let mut con2: metamodelica::Ref<DAE::Exp>;
    let mut z: metamodelica::Ref<DAE::Exp>;
    let mut der_e: metamodelica::Ref<DAE::Exp>;
    let mut b1: bool;
    let mut b2: bool;
    let mut b: bool;
    let mut b3: bool;
    let mut b4: bool;
    let mut tp: metamodelica::Ref<DAE::Type>;
    if Flags::getConfigBool(Flags::GENERATE_DYN_OPTIMIZATION_PROBLEM.clone())? {
        let __arc2 = inDAE;
        let BackendDAE::DAE {
            eqs: __pa0,
            shared: __pa1,
        } = &*__arc2;
        systlst = metamodelica::Own::own(__pa0);
        shared = metamodelica::Own::own(__pa1);
        let __arc5 = shared.clone();
        let BackendDAE::SHARED {
            functionTree: __pa3,
            globalKnownVars: __pa4,
            ..
        } = &*__arc5;
        funcs = metamodelica::Own::own(__pa3);
        globalKnownVars = metamodelica::Own::own(__pa4);
        for mut syst in &*systlst {
            let (__pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(syst.clone()) {
                Deref @ BackendDAE::EqSystem { orderedVars: __pa6, orderedEqs: __pa7, matching: Deref @ BackendDAE::Matching::MATCHING { comps: __pa8, .. }, .. } => (__pa6.clone(), __pa7.clone(), __pa8.clone()),
                _ => return Err("pattern mismatch"),
            } };
            vars = metamodelica::Own::own(__pa6);
            eqns = metamodelica::Own::own(__pa7);
            comps = metamodelica::Own::own(__pa8);
            b = false;
            '__loop10: for mut comp in &*comps {
                if (match &*comp.clone() {
                    BackendDAE::StrongComponent::SINGLEEQUATION { .. } => true,
                    _ => false,
                }) {
                    let (__pa11, __pa12) = ::match_deref::match_deref! { match &(comp.clone()) {
                        Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { eqn: __pa11, var: __pa12 } => (__pa11.clone(), __pa12.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    eindex = metamodelica::Own::own(__pa11);
                    vindx = metamodelica::Own::own(__pa12);
                    var_con = BackendVariable::getVarAt(&vars, vindx)?;
                    b3 = BackendVariable::isRealOptimizeConstraintsVars(&var_con);
                    if b3 {
                        if '__try13: {
                            let (__pa16, __pa14, __pa15) = ::match_deref::match_deref! { match &(unwrap_break_err!(BackendEquation::get(eqns.clone(), eindex), '__try13)) {
                                __pa16 @ Deref @ BackendDAE::Equation::EQUATION { exp: __pa14, scalar: __pa15, .. } => (__pa16.clone(), __pa14.clone(), __pa15.clone()),
                                _ => break '__try13 Err::<_, _>("pattern mismatch"),
                            } };
                            e1 = metamodelica::Own::own(__pa14);
                            e2 = metamodelica::Own::own(__pa15);
                            eqn_ = metamodelica::Own::own(__pa16);
                            let true = (unwrap_break_err!(ExpressionBasics::expEqual(&e1, unwrap_break_err!(BackendVariable::varExp(&var_con), '__try13)), '__try13)) else { break '__try13 Err::<_, _>("pattern mismatch") };
                            Ok::<(), &'static str>(())
                        }.is_err() {
                            b3 = false;
                        }
                    }
                    if b3 {
                        var_lst = BackendEquation::equationsLstVars(&(list![eqn_.clone()]), vars.clone())?;
                        var_lst_opt = ({
                            let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
                            for mut vv in (var_lst).into_iter().cloned() {
                                if !(BackendVariable::isStateVar(&(vv.clone()))) {
                                    continue;
                                }
                                let __x = vv.clone();
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        });
                        b3 = ((var_lst_opt).len() as i32) == 1;
                        var_lst = BackendEquation::equationsLstVars(&(list![eqn_.clone()]), globalKnownVars.clone())?;
                        var_lst_opt = listAppend(
                            var_lst_opt,
                            ({
                                let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> =
                                    metamodelica::nil();
                                for mut vv in (var_lst.clone()).into_iter().cloned() {
                                    if !(BackendVariable::isInput(&(vv.clone()))) {
                                        continue;
                                    }
                                    let __x = vv.clone();
                                    __acc = cons(__x, __acc);
                                }
                                __acc.reverse()
                            }),
                        );
                        if ((var_lst_opt).len() as i32) == 1 {
                            let __pa17 = ::match_deref::match_deref! { match &(var_lst_opt) {
                                Deref @ metamodelica::ListNode::Cons { head: __pa17, tail: Deref @ metamodelica::ListNode::Nil } => __pa17.clone(),
                                _ => return Err("pattern mismatch"),
                            } };
                            var_ = metamodelica::Own::own(__pa17);
                            let __arc20 = var_.clone();
                            let BackendDAE::VAR { varName: __pa19, .. } = &*__arc20;
                            cr = metamodelica::Own::own(__pa19);
                            e = Expression::crefExp(cr.clone())?;
                            tp = Expression::r#typeof(e.clone())?;
                            zero = Expression::makeConstZero(&tp);
                            if '__try21: {
                                der_e = unwrap_break_err!(Differentiate::differentiateExpSolve(e2.clone(), cr.clone(), Some(funcs.clone())), '__try21);
                                (der_e, _) = unwrap_break_err!(ExpressionSimplify::simplify(der_e.clone()), '__try21);
                                if unwrap_break_err!(Expression::isZero(&e), '__try21) {
                                    continue '__loop10;
                                }
                                (z, _) = unwrap_break_err!(Expression::makeZeroExpression(&(Expression::arrayDimension(&tp))), '__try21);
                                (c, _) = unwrap_break_err!(Expression::replaceExp(e2.clone(), e.clone(), z.clone()), '__try21);
                                (c, _) = unwrap_break_err!(ExpressionSimplify::simplify(c.clone()), '__try21);
                                var_lst = unwrap_break_err!(BackendEquation::expressionVars(der_e.clone(), globalKnownVars.clone()), '__try21);
                                if b3 {
                                    var_lst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
        for mut vv in (var_lst.clone()).into_iter().cloned() {
            if !(!(BackendVariable::isParam(&(vv.clone())))) { continue; }
            let __x = vv.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                                }
                                var_lst = listAppend(unwrap_break_err!(BackendEquation::expressionVars(der_e.clone(), vars.clone()), '__try21), var_lst.clone());
                                var_lst1 = unwrap_break_err!(BackendEquation::expressionVars(c.clone(), globalKnownVars.clone()), '__try21);
                                if b3 {
                                    var_lst1 = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
        for mut vv in (var_lst1.clone()).into_iter().cloned() {
            if !(!(BackendVariable::isParam(&(vv.clone())))) { continue; }
            let __x = vv.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                                }
                                var_lst1 = listAppend(unwrap_break_err!(BackendEquation::expressionVars(c.clone(), vars.clone()), '__try21), var_lst1.clone());
                                var_lst = listAppend(var_lst1.clone(), var_lst.clone());
                                b4 = unwrap_break_err!(Expression::expHasCref(der_e.clone(), DAE::crefTime().clone()), '__try21) || unwrap_break_err!(Expression::expHasCref(c.clone(), DAE::crefTime().clone()), '__try21);
                                if (var_lst).is_empty() && !(b4) {
                                    (oMin_con, oMax_con) = BackendVariable::getMinMaxAttribute(&var_con);
                                    b1 = (oMin_con).is_some();
                                    b2 = (oMax_con).is_some();
                                    con2 = Expression::makeNoEvent(metamodelica::Ref::new(DAE::Exp::RELATION { exp1: der_e.clone(), operator: DAE::Operator::LESS { ty: tp.clone() }, exp2: zero.clone(), index: -1, optionExpisASUB: None }));
                                    if b1 {
                                        let __pa22 = ::match_deref::match_deref! { match &(oMin_con.clone()) {
                                            Some(__pa22) => __pa22.clone(),
                                            _ => break '__try21 Err::<_, _>("pattern mismatch"),
                                        } };
                                        min_con = metamodelica::Own::own(__pa22);
                                        min_con = unwrap_break_err!(Expression::makeDiv(unwrap_break_err!(Expression::expSub(min_con.clone(), c.clone()), '__try21), der_e.clone()), '__try21);
                                    } else {
                                        min_con = metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(-1e64_f64) });
                                    }
                                    if b2 {
                                        let __pa23 = ::match_deref::match_deref! { match &(oMax_con.clone()) {
                                            Some(__pa23) => __pa23.clone(),
                                            _ => break '__try21 Err::<_, _>("pattern mismatch"),
                                        } };
                                        max_con = metamodelica::Own::own(__pa23);
                                        max_con = unwrap_break_err!(Expression::makeDiv(unwrap_break_err!(Expression::expSub(max_con.clone(), c.clone()), '__try21), der_e.clone()), '__try21);
                                    } else {
                                        max_con = metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1e64_f64) });
                                    }
                                    oMin_con = Some(metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: con2.clone(), expThen: max_con.clone(), expElse: min_con.clone() }));
                                    oMax_con = Some(metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: con2.clone(), expThen: min_con.clone(), expElse: max_con.clone() }));
                                    oMin_con = unwrap_break_err!(ExpressionSimplify::simplify1o(oMin_con.clone()), '__try21);
                                    oMax_con = unwrap_break_err!(ExpressionSimplify::simplify1o(oMax_con.clone()), '__try21);
                                    var_con = unwrap_break_err!(BackendVariable::setVarMinMax(var_con.clone(), oMin_con.clone(), oMax_con.clone()), '__try21);
                                    var_ = BackendVariable::mergeMinMaxAttribute(var_con.clone(), var_.clone(), false);
                                    var_con = unwrap_break_err!(BackendVariable::setVarKind(var_con.clone(), openmodelica_backend_types::BackendDAE::VarKind::VARIABLE), '__try21);
                                    vars = unwrap_break_err!(BackendVariable::setVarAt(vars.clone(), vindx, var_con.clone()), '__try21);
                                    match '__try24: {
                                        (_, vindx) = unwrap_break_err!(BackendVariable::getVarSingle(&cr, &vars), '__try24);
                                        vars = unwrap_break_err!(BackendVariable::setVarAt(vars.clone(), vindx, var_.clone()), '__try24);
                                        Ok::<_, &'static str>((vindx.clone(),))
                                    } {
                                        Ok((__try24_o0,)) => {
                                            vindx = __try24_o0;
                                        }
                                        Err(_) => {
                                            (_, vindx) = unwrap_break_err!(BackendVariable::getVarSingle(&cr, &globalKnownVars), '__try21);
                                            globalKnownVars = unwrap_break_err!(BackendVariable::setVarAt(globalKnownVars.clone(), vindx, var_.clone()), '__try21);
                                        }
                                    }
                                    b = true;
                                }
                                Ok::<(), &'static str>(())
                            }.is_err() {
                            }
                        }
                    }
                }
            }
            if b {
                new_systlst = metamodelica::cons(
                    BackendDAEUtil::clearEqSyst(metamodelica::AsArg::as_arg(&syst)),
                    new_systlst,
                );
            } else {
                new_systlst = metamodelica::cons(syst.clone(), new_systlst);
            }
        }
        shared = BackendDAEUtil::setSharedGlobalKnownVars(shared, globalKnownVars);
        outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
            eqs: new_systlst,
            shared: shared,
        });
    } else {
        outDAE = inDAE;
    }
    Ok(outDAE)
}

// =============================================================================
// section for postOptModule >>reduceDynamicOptimization<<
//
// remove eqs which not need for the calculations of cost and constraints
// =============================================================================
pub(crate) fn reduceDynamicOptimization(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut opt_varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut conVarsList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut fconVarsList: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut objMayer: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut objLagrange: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut systlst: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut newsyst: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut v: BackendDAE::Variables;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let __arc2 = &(*inDAE);
    let BackendDAE::DAE {
        eqs: __pa0,
        shared: __pa1,
    } = &**__arc2;
    systlst = metamodelica::Own::own(__pa0);
    shared = metamodelica::Own::own(__pa1);
    shared = BackendVariable::removeAliasVars(shared);
    for mut syst in &*systlst {
        let mut syst = syst.clone();
        syst = BackendEquation::removeRemovedEqs(syst);
        let __arc4 = syst.clone();
        let BackendDAE::EQSYSTEM { orderedVars: __pa3, .. } = &*__arc4;
        v = metamodelica::Own::own(__pa3);
        varlst = BackendVariable::varList(&v)?;
        conVarsList = List::select(
            varlst.clone(),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(BackendVariable::isRealOptimizeConstraintsVars(&__a0))
                },
            )
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>),
        )?;
        fconVarsList = List::select(
            varlst,
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(BackendVariable::isRealOptimizeFinalConstraintsVars(&__a0))
                },
            )
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BackendDAE::Var>) -> Result<bool> + 'static>),
        )?;
        objMayer = checkObjectIsSet(&v, arcstr::literal!(BackendDAE::optimizationMayerTermName));
        objLagrange = checkObjectIsSet(&v, arcstr::literal!(BackendDAE::optimizationLagrangeTermName));
        opt_varlst = listAppend(conVarsList, listAppend(fconVarsList, listAppend(objMayer, objLagrange)));
        if !((opt_varlst).is_empty()) {
            newsyst = metamodelica::cons(
                BackendDAEUtil::tryReduceEqSystem(syst, &shared, opt_varlst, false),
                newsyst,
            );
        }
    }
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: newsyst,
        shared: shared,
    });
    Ok(outDAE)
}

pub(crate) fn checkObjectIsSet(
    mut inVars: &BackendDAE::Variables,
    mut CrefName: ArcStr,
) -> metamodelica::List<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut leftcref: metamodelica::Ref<DAE::ComponentRef>;
    leftcref = ComponentReferenceBasics::makeCrefIdent(CrefName, DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
    match '__try0: {
        (outVars, _) = unwrap_break_err!(BackendVariable::getVar(leftcref.clone(), inVars), '__try0);
        Ok::<_, &'static str>((outVars.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outVars = __try0_o0;
        }
        Err(_) => {
            outVars = metamodelica::nil();
        }
    }
    outVars
}
