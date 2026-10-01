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
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_error::ErrorTypes;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::HashTable;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::MMath;
use openmodelica_util::StringUtil;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// clock partitioning
//
// =============================================================================
pub(crate) fn clockPartitioning(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    outDAE = (::match_deref::match_deref! { match inDAE {
        Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: syst, tail: Deref @ metamodelica::ListNode::Nil }, shared } => {
            clockPartitioning1(syst.clone(), shared.clone())?
        },
        _ => {
            let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
            let mut shared: metamodelica::Ref<BackendDAE::Shared>;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendDAEOptimize::collapseIndependentBlocks(inDAE)?) {
                Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, shared: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            syst = metamodelica::Own::own(__pa0);
            shared = metamodelica::Own::own(__pa1);
            clockPartitioning1(syst, shared)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDAE)
}

pub(crate) fn synchronousFeatures(
    mut inDAE: metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut contSysts: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut clockedSysts: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    (clockedSysts, contSysts) = List::splitOnTrue(&inDAE.eqs, &move |__a0: metamodelica::Ref<
        BackendDAE::EqSystem,
    >|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(BackendDAEUtil::isClockedSyst(&__a0))
    })?;
    if !((clockedSysts).is_empty()) {
        shared = inDAE.shared.clone();
        (clockedSysts, shared) = treatClockedStates(clockedSysts, shared)?;
        systs = listAppend(contSysts, clockedSysts);
        outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
            eqs: systs.clone(),
            shared: shared.clone(),
        });
        if Flags::isSet(Flags::DUMP_SYNCHRONOUS.clone())? {
            metamodelica::print(literal!("synchronous features post-phase: synchronousFeatures\n\n"));
            BackendDump::dumpEqSystems(&systs, &(literal!("clock partitioning")))?;
            BackendDump::dumpBasePartitions(shared.partitionsInfo.basePartitions.clone(), &(literal!("Base clocks")))?;
            BackendDump::dumpSubPartitions(shared.partitionsInfo.subPartitions.clone(), &(literal!("Sub clocks")))?;
        }
    } else {
        outDAE = inDAE;
    }
    Ok(outDAE)
}

pub(crate) fn contPartitioning(
    mut inDAE: &metamodelica::Ref<BackendDAE::BackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut clockedSysts: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut clockedSysts1: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut unpartRemEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    (clockedSysts, systs) = List::splitOnTrue(
        &inDAE.eqs,
        &move |__a0: metamodelica::Ref<BackendDAE::EqSystem>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(BackendDAEUtil::isClockedSyst(&__a0))
        },
    )?;
    shared = inDAE.shared.clone();
    if !((systs).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(BackendDAEOptimize::collapseIndependentBlocks(&(metamodelica::Ref::new(BackendDAE::BackendDAE { eqs: systs, shared: shared })))?) {
            Deref @ BackendDAE::BackendDAE { eqs: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, shared: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        syst = metamodelica::Own::own(__pa0);
        shared = metamodelica::Own::own(__pa1);
        (systs, clockedSysts1, unpartRemEqs) = baseClockPartitioning(syst, &shared)?;
        assert!(
            (clockedSysts1).is_empty(),
            "{}",
            &*literal!("Get clocked system in SynchronousFeatures.addContVarsEqs")
        );
        assign_field!(shared.removedEqs = BackendEquation::addList(&unpartRemEqs, shared.removedEqs.clone())?);
    }
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: listAppend(systs, clockedSysts),
        shared: shared,
    });
    Ok(outDAE)
}

fn clockPartitioning1(
    mut inSyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outDAE: metamodelica::Ref<BackendDAE::BackendDAE>;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut contSysts: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut clockedSysts: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared> = inShared.clone();
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut holdComps: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut unpartRemEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    syst = substitutePartitionOpExps(inSyst, inShared)?;
    (contSysts, clockedSysts, unpartRemEqs) = baseClockPartitioning(syst, &shared)?;
    (contSysts, holdComps) = removeHoldExpsSyst(&contSysts)?;
    (clockedSysts, shared) = subClockPartitioning1(&clockedSysts, shared, &holdComps)?;
    unpartRemEqs = createBoolClockWhenClauses(&shared, unpartRemEqs)?;
    assign_field!(shared.removedEqs = BackendEquation::addList(&unpartRemEqs, shared.removedEqs.clone())?);
    systs = listAppend(contSysts, clockedSysts.clone());
    outDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
        eqs: systs.clone(),
        shared: shared.clone(),
    });
    if !((clockedSysts).is_empty()) {
        if Flags::isSet(Flags::DUMP_SYNCHRONOUS.clone())? {
            metamodelica::print(literal!("synchronous features pre-phase: synchronousFeatures\n\n"));
            BackendDump::dumpEqSystems(&systs, &(literal!("clock partitioning")))?;
            BackendDump::dumpBasePartitions(shared.partitionsInfo.basePartitions.clone(), &(literal!("Base clocks")))?;
            BackendDump::dumpSubPartitions(shared.partitionsInfo.subPartitions.clone(), &(literal!("Sub clocks")))?;
        }
    }
    Ok(outDAE)
}

fn createBoolClockWhenClauses(
    mut inShared: &metamodelica::Ref<BackendDAE::Shared>,
    mut inRemovedEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    let mut outRemovedEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = inRemovedEqs;
    let mut basePartition: BackendDAE::BasePartition;
    for mut i in 1..=metamodelica::arrayLength(inShared.partitionsInfo.basePartitions.clone()) {
        basePartition = ({
            let __elt = (*metamodelica::index_checked(&inShared.partitionsInfo.basePartitions.borrow(), i)?).clone();
            __elt
        });
        outRemovedEqs = (match &*basePartition.clock.clone() {
            DAE::ClockKind::EVENT_CLOCK {
                condition: c,
                startInterval: _,
            } => {
                let mut e: metamodelica::Ref<DAE::Exp>;
                let mut whenEq: metamodelica::Ref<BackendDAE::WhenEquation>;
                let mut eq: metamodelica::Ref<BackendDAE::Equation>;
                e = metamodelica::Ref::new(DAE::Exp::CALL {
                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: literal!("$_clkfire"),
                    }),
                    expLst: list![metamodelica::Ref::new(DAE::Exp::ICONST { integer: i })],
                    attr: DAE::callAttrBuiltinOther().clone(),
                });
                whenEq = metamodelica::Ref::new(BackendDAE::WhenEquation {
                    condition: c.clone(),
                    whenStmtLst: list![BackendDAE::WhenOperator::NORETCALL {
                        exp: e,
                        source: DAE::emptyElementSource().clone()
                    }],
                    elsewhenPart: None,
                });
                eq = metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION {
                    size: 0,
                    whenEquation: whenEq,
                    source: DAE::emptyElementSource().clone(),
                    attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(),
                });
                metamodelica::cons(eq, outRemovedEqs)
            }
            _ => outRemovedEqs,
        });
    }
    Ok(outRemovedEqs)
}

pub(crate) fn getBoolClockWhenClauses(
    mut eq: metamodelica::Ref<BackendDAE::Equation>,
    mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> (
    metamodelica::Ref<BackendDAE::Equation>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) {
    let mut eq: metamodelica::Ref<BackendDAE::Equation> = eq;
    let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = eqLst;
    if hasBoolClockWhenClause(&eq) {
        eqLst = metamodelica::cons(eq.clone(), eqLst);
    }
    (eq, eqLst)
}

fn hasBoolClockWhenClause(mut eqn: &metamodelica::Ref<BackendDAE::Equation>) -> bool {
    let mut hasBool: bool = false;
    let () = (::match_deref::match_deref! { match eqn {
        Deref @ BackendDAE::Equation::WHEN_EQUATION { size: 0, whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::ListNode::Cons { head: BackendDAE::WhenOperator::NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "$_clkfire" }, .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. } => {
            hasBool = true;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasBool
}

fn treatClockedStates(
    mut inSysts: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut outSysts: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut shared: metamodelica::Ref<BackendDAE::Shared> = inShared;
    outSysts = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
        for mut syst in (inSysts).into_iter().cloned() {
            let __x = ({
                let mut lstEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
                let mut derVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
                (match &*syst.clone() {
                    BackendDAE::EqSystem { orderedEqs: eqs, .. } => {
                        let mut idx: i32;
                        let mut subPartition: BackendDAE::SubPartition;
                        let mut solverMethod: ArcStr;
                        let mut eq: metamodelica::Ref<BackendDAE::Equation>;
                        let mut var: metamodelica::Ref<BackendDAE::Var>;
                        let mut exp: metamodelica::Ref<DAE::Exp>;
                        let mut exp2: metamodelica::Ref<DAE::Exp>;
                        let mut ty: metamodelica::Ref<DAE::Type>;
                        let mut eqs = (*eqs).clone();
                        let BackendDAE::CLOCKED_PARTITION { subPartIdx: __pa0 } = (syst.partitionKind.clone()) else {
                            return Err("pattern mismatch");
                        };
                        idx = metamodelica::Own::own(__pa0);
                        subPartition = ({
                            let __elt =
                                (*metamodelica::index_checked(&shared.partitionsInfo.subPartitions.borrow(), idx)?)
                                    .clone();
                            __elt
                        });
                        solverMethod = BackendDump::optionString(getSubClockSolverOpt(&subPartition.clock));
                        if StringUtil::startsWith(solverMethod.clone(), literal!("Explicit")) {
                            if !metamodelica::stringEq(&solverMethod, &(literal!("ExplicitEuler"))) {
                                Error::addMessage(
                                    Error::CLOCK_SOLVERMETHOD.clone(),
                                    list![literal!("ExplicitEuler"), solverMethod.clone()],
                                )?;
                                solverMethod = literal!("ExplicitEuler");
                            }
                        } else if ((solverMethod).len() as i32) > 0
                            && !metamodelica::stringEq(&solverMethod, &(literal!("ImplicitEuler")))
                            && !metamodelica::stringEq(&solverMethod, &(literal!("SemiImplicitEuler")))
                            && !metamodelica::stringEq(&solverMethod, &(literal!("ImplicitTrapezoid")))
                        {
                            Error::addMessage(
                                Error::CLOCK_SOLVERMETHOD.clone(),
                                list![literal!("ImplicitEuler"), solverMethod.clone()],
                            )?;
                            solverMethod = literal!("ImplicitEuler");
                        }
                        for mut i in 1..=BackendEquation::getNumberOfEquations(eqs.clone()) {
                            eq = BackendEquation::get(eqs.clone(), i)?;
                            let (__pa1, (__pa2, _)) = BackendEquation::traverseExpsOfEquation(
                                eq.clone(),
                                (std::sync::Arc::new(getDerVars1)
                                    as std::sync::Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::Exp>,
                                                (
                                                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                                                    Option<ArcStr>,
                                                ),
                                            ) -> Result<(
                                                metamodelica::Ref<DAE::Exp>,
                                                (
                                                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                                                    Option<ArcStr>,
                                                ),
                                            )> + 'static,
                                    >),
                                (derVars.clone(), BackendEquation::getForEquationIterIdent(&eq)),
                            )?;
                            eq = metamodelica::Own::own(__pa1);
                            derVars = metamodelica::Own::own(__pa2);
                            lstEqs = metamodelica::cons(eq.clone(), lstEqs.clone());
                        }
                        for mut derVar in &*derVars {
                            var = ((BackendVariable::getVar(derVar.clone(), &(syst.orderedVars.clone()))?).0).get(1)?;
                            var = metamodelica::Ref::new(BackendDAE::Var {
                                varName: ComponentReference::crefPrefixDer(derVar.clone()),
                                varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE,
                                varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
                                varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
                                varType: var.varType.clone(),
                                bindExp: None,
                                tplExp: None,
                                arryDim: var.arryDim.clone(),
                                source: DAE::emptyElementSource().clone(),
                                values: None,
                                tearingSelectOption: None,
                                hideResult: None,
                                comment: None,
                                connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(
                                ),
                                innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER,
                                unreplaceable: false,
                                initNonlinear: false,
                                encrypted: false,
                            });
                            assign_field!(
                                syst.orderedVars = BackendVariable::addVar(var.clone(), syst.orderedVars.clone())?
                            );
                        }
                        for mut derVar in &*derVars {
                            let mut derVar = derVar.clone();
                            var = ((BackendVariable::getVar(derVar.clone(), &(syst.orderedVars.clone()))?).0).get(1)?;
                            ty = var.varType.clone();
                            derVar = (match &*var.varType.clone() {
                                DAE::Type::T_ARRAY { ty: __esc_ty, .. } => {
                                    ty = (*__esc_ty).clone();
                                    ComponentReference::crefApplySubs(
                                        &derVar,
                                        &(list![metamodelica::Ref::new(DAE::Subscript::INDEX {
                                            exp: metamodelica::Ref::new(DAE::Exp::CREF {
                                                componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                                                    ident: literal!("i"),
                                                    identType: DAE::T_INTEGER_DEFAULT().clone(),
                                                    subscriptLst: metamodelica::nil()
                                                }),
                                                ty: DAE::T_INTEGER_DEFAULT().clone()
                                            })
                                        })]),
                                    )?
                                }
                                _ => derVar.clone(),
                            });
                            exp = metamodelica::Ref::new(DAE::Exp::CALL {
                                path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("der") }),
                                expLst: list![metamodelica::Ref::new(DAE::Exp::CREF {
                                    componentRef: derVar.clone(),
                                    ty: ty.clone()
                                })],
                                attr: DAE::callAttrBuiltinImpureReal().clone(),
                            });
                            (exp, _) = substituteFiniteDifference(exp.clone(), metamodelica::nil());
                            exp2 = metamodelica::Ref::new(DAE::Exp::CREF {
                                componentRef: ComponentReference::crefPrefixDer(derVar.clone()),
                                ty: ty.clone(),
                            });
                            if metamodelica::stringEq(&solverMethod, &(literal!("ExplicitEuler"))) {
                                exp2 = metamodelica::Ref::new(DAE::Exp::CALL {
                                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                                        name: literal!("previous"),
                                    }),
                                    expLst: list![exp2.clone()],
                                    attr: DAE::callAttrBuiltinImpureReal().clone(),
                                });
                            } else if metamodelica::stringEq(&solverMethod, &(literal!("ImplicitTrapezoid"))) {
                                exp2 = metamodelica::Ref::new(DAE::Exp::BINARY {
                                    exp1: exp2.clone(),
                                    operator: DAE::Operator::ADD {
                                        ty: DAE::T_REAL_DEFAULT().clone(),
                                    },
                                    exp2: metamodelica::Ref::new(DAE::Exp::CALL {
                                        path: metamodelica::Ref::new(Absyn::Path::IDENT {
                                            name: literal!("previous"),
                                        }),
                                        expLst: list![exp2.clone()],
                                        attr: DAE::callAttrBuiltinImpureReal().clone(),
                                    }),
                                });
                                exp2 = metamodelica::Ref::new(DAE::Exp::BINARY {
                                    exp1: metamodelica::Ref::new(DAE::Exp::RCONST {
                                        real: metamodelica::OrderedFloat(0.5_f64),
                                    }),
                                    operator: DAE::Operator::MUL {
                                        ty: DAE::T_REAL_DEFAULT().clone(),
                                    },
                                    exp2: exp2.clone(),
                                });
                            }
                            exp2 = metamodelica::Ref::new(DAE::Exp::IFEXP {
                                expCond: metamodelica::Ref::new(DAE::Exp::CALL {
                                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                                        name: literal!("firstTick"),
                                    }),
                                    expLst: metamodelica::nil(),
                                    attr: DAE::callAttrBuiltinImpureBool().clone(),
                                }),
                                expThen: metamodelica::Ref::new(DAE::Exp::RCONST {
                                    real: metamodelica::OrderedFloat((0) as f64),
                                }),
                                expElse: exp2.clone(),
                            });
                            eq = (::match_deref::match_deref! { match &(var.varType.clone()) {
                                Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                                    metamodelica::Ref::new(BackendDAE::Equation::FOR_EQUATION { iter: metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: literal!("i"), identType: DAE::T_INTEGER_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_INTEGER_DEFAULT().clone() }), start: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }), stop: DAEUtil::dimExp(dim.clone())?, body: metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: exp.clone(), scalar: exp2.clone(), source: var.source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), source: var.source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() })
                                },
                                _ => {
                                    metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: exp.clone(), scalar: exp2.clone(), source: var.source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() })
                                },
                                _ => unreachable!("match_deref! exhaustiveness placeholder"),
                            } });
                            lstEqs = metamodelica::cons(eq.clone(), lstEqs.clone());
                        }
                        assign_field!(syst.orderedEqs = BackendEquation::listEquation(&(lstEqs.clone().reverse()))?);
                        if metamodelica::stringEq(&solverMethod, &(literal!("SemiImplicitEuler"))) {
                            for mut i in 1..=BackendEquation::getNumberOfEquations(eqs.clone()) {
                                eq = BackendEquation::get(eqs.clone(), i)?;
                                (eq, _) = BackendEquation::traverseExpsOfEquation(
                                    eq.clone(),
                                    (std::sync::Arc::new(shiftDerVars1)
                                        as std::sync::Arc<
                                            dyn ::std::ops::Fn(
                                                    metamodelica::Ref<DAE::Exp>,
                                                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                                                )
                                                    -> Result<(
                                                    metamodelica::Ref<DAE::Exp>,
                                                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                                                )> + 'static,
                                        >),
                                    derVars.clone(),
                                )?;
                                eqs = BackendEquation::setAtIndex(eqs.clone(), i, eq.clone())?;
                            }
                        }
                        shared = markClockedStates(&(syst.clone()), shared.clone(), &derVars)?;
                        BackendDAEUtil::clearEqSyst(&(syst.clone()))
                    }
                })
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((outSysts, shared))
}

fn getDerVars1(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inDerVars: (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, Option<ArcStr>),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, Option<ArcStr>),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outDerVars: (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, Option<ArcStr>);
    (outExp, outDerVars) = Expression::traverseExpBottomUp(inExp, &getDerVars, inDerVars)?;
    Ok((outExp, outDerVars))
}

fn getDerVars(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inDerVars: (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, Option<ArcStr>),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, Option<ArcStr>),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outDerVars: (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, Option<ArcStr>) = inDerVars.clone();
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: x, ty }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut derVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut optForIter: Option<ArcStr>;
            let mut forIter: ArcStr;
            let mut der_x: metamodelica::Ref<DAE::Exp>;
            let mut x = (*x).clone();
            der_x = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: ComponentReference::crefPrefixDer(x.clone()), ty: ty.clone() });
            (derVars, optForIter) = inDerVars;
            let () = (match optForIter.clone() {
        Some(mut __esc_forIter) => {
            forIter = __esc_forIter.clone();
            x = ComponentReference::crefStripIterSub(metamodelica::AsArg::as_arg(&x), &forIter);
            ()
        },
        _ => (),
    });
            if !(ComponentReferenceBasics::crefInLst(x.clone(), &derVars)?) {
                derVars = metamodelica::cons(x.clone(), derVars);
            }
            outDerVars = (derVars, optForIter);
            der_x
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outDerVars))
}

fn shiftDerVars1(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inDerVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outDerVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (outExp, outDerVars) = Expression::traverseExpBottomUp(inExp, &shiftDerVars, inDerVars)?;
    Ok((outExp, outDerVars))
}

fn shiftDerVars(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inDerVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outDerVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = inDerVars.clone();
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: x, .. } if (ComponentReferenceBasics::crefInLst(x.clone(), &inDerVars)?) => {
            let mut exp: metamodelica::Ref<DAE::Exp>;
            exp = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("previous") }), expLst: list![inExp], attr: DAE::callAttrBuiltinImpureReal().clone() });
            exp
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst, .. }, tail: Deref @ metamodelica::ListNode::Nil }, attr: attr @ Deref @ DAE::CallAttributes { .. } } => {
            let mut exp: metamodelica::Ref<DAE::Exp>;
            exp = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("der") }), expLst: expLst.clone(), attr: attr.clone() });
            exp
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst, .. }, tail: Deref @ metamodelica::ListNode::Nil }, attr: attr @ Deref @ DAE::CallAttributes { .. } } => {
            let mut exp: metamodelica::Ref<DAE::Exp>;
            exp = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("previous") }), expLst: expLst.clone(), attr: attr.clone() });
            exp
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outDerVars))
}

fn substituteFiniteDifference1(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inDerVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outDerVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (outExp, outDerVars) = Expression::traverseExpBottomUp(
        inExp,
        &fnptr!(
            substituteFiniteDifference,
            metamodelica::Ref<DAE::Exp>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>
        ),
        inDerVars,
    )?;
    Ok((outExp, outDerVars))
}

fn substituteFiniteDifference(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inDerVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> (
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outDerVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (outExp, outDerVars) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: expLst @ Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: x, .. }, tail: Deref @ metamodelica::ListNode::Nil }, attr: attr @ Deref @ DAE::CallAttributes { ty, .. } } => {
            let mut exp: metamodelica::Ref<DAE::Exp>;
            exp = metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("previous") }), expLst: expLst.clone(), attr: attr.clone() });
            exp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::CREF { componentRef: x.clone(), ty: ty.clone() }), operator: DAE::Operator::SUB { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: exp });
            exp = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: exp, operator: DAE::Operator::DIV { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("interval") }), expLst: metamodelica::nil(), attr: DAE::callAttrBuiltinImpureReal().clone() }) });
            (exp, metamodelica::cons(x.clone(), inDerVars))
        },
        _ => {
            (inExp, inDerVars)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outDerVars)
}

fn markClockedStates(
    mut inSyst: &metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut derVars: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::Ref<BackendDAE::Shared>> {
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    let mut prevVars: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut isPrevVarArr: metamodelica::Array<bool>;
    let mut isDerVarArr: metamodelica::Array<bool>;
    let mut varIxs: metamodelica::List<i32>;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut idx: i32;
    let mut subPartition: BackendDAE::SubPartition;
    let BackendDAE::CLOCKED_PARTITION { subPartIdx: __pa0 } = (inSyst.partitionKind.clone()) else {
        return Err("pattern mismatch");
    };
    idx = metamodelica::Own::own(__pa0);
    subPartition = ({
        let __elt = (*metamodelica::index_checked(&outShared.partitionsInfo.subPartitions.borrow(), idx)?).clone();
        __elt
    });
    isPrevVarArr = arrayCreate(BackendVariable::varsSize(&inSyst.orderedVars), false);
    isDerVarArr = arrayCreate(BackendVariable::varsSize(&inSyst.orderedVars), false);
    for mut cr in &**derVars {
        varIxs = getVarIxs(cr.clone(), &inSyst.orderedVars);
        for mut idx in &*varIxs {
            let mut idx = idx.clone();
            metamodelica::arrayUpdate(isDerVarArr.clone(), idx, true)?;
        }
    }
    for mut i in 1..=BackendEquation::getNumberOfEquations(inSyst.orderedEqs.clone()) {
        eq = BackendEquation::get(inSyst.orderedEqs.clone(), i)?;
        let (_, (__pa1, _)) = BackendEquation::traverseExpsOfEquation(
            eq.clone(),
            (std::sync::Arc::new(collectPrevVars)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, Option<ArcStr>),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, Option<ArcStr>),
                        )> + 'static,
                >),
            (prevVars, BackendEquation::getForEquationIterIdent(&eq)),
        )?;
        prevVars = metamodelica::Own::own(__pa1);
    }
    for mut i in 1..=BackendEquation::getNumberOfEquations(inSyst.removedEqs.clone()) {
        eq = BackendEquation::get(inSyst.removedEqs.clone(), i)?;
        let (_, (__pa2, _)) = BackendEquation::traverseExpsOfEquation(
            eq.clone(),
            (std::sync::Arc::new(collectPrevVars)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, Option<ArcStr>),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, Option<ArcStr>),
                        )> + 'static,
                >),
            (prevVars, BackendEquation::getForEquationIterIdent(&eq)),
        )?;
        prevVars = metamodelica::Own::own(__pa2);
    }
    if !(Flags::isSet(Flags::NF_SCALARIZE.clone())?) {
        prevVars = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
            for mut cr in (prevVars).into_iter().cloned() {
                let __x = ComponentReferenceBasics::crefStripLastSubs(&(cr.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
    }
    for mut cr in &*prevVars {
        varIxs = getVarIxs(cr.clone(), &inSyst.orderedVars);
        for mut idx in &*varIxs {
            let mut idx = idx.clone();
            metamodelica::arrayUpdate(isPrevVarArr.clone(), idx, true)?;
        }
    }
    prevVars = metamodelica::nil();
    for mut i in 1..=metamodelica::arrayLength(isPrevVarArr.clone()) {
        if ({
            let __elt = (*metamodelica::index_checked(&isPrevVarArr.borrow(), i)?).clone();
            __elt
        }) {
            var = BackendVariable::getVarAt(&inSyst.orderedVars, i)?;
            var = BackendVariable::setVarKind(
                var.clone(),
                BackendDAE::VarKind::CLOCKED_STATE {
                    previousName: ComponentReference::crefPrefixPrevious(var.varName.clone()),
                    isStartFixed: ({
                        let __elt = (*metamodelica::index_checked(&isDerVarArr.borrow(), i)?).clone();
                        __elt
                    }),
                },
            )?;
            var = BackendVariable::setVarFixed(var, true)?;
            BackendVariable::setVarAt(inSyst.orderedVars.clone(), i, var.clone())?;
            prevVars = metamodelica::cons(var.varName.clone(), prevVars);
        }
    }
    subPartition.prevVars = prevVars;
    metamodelica::arrayUpdate(outShared.partitionsInfo.subPartitions.clone(), idx, subPartition)?;
    Ok(outShared)
}

fn collectPrevVars(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inPrevVars: (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, Option<ArcStr>),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, Option<ArcStr>),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outPrevVars: (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, Option<ArcStr>);
    (outExp, outPrevVars) = Expression::traverseExpBottomUp(
        inExp,
        &fnptr!(
            collectPrevVars1,
            metamodelica::Ref<DAE::Exp>,
            (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, Option<ArcStr>)
        ),
        inPrevVars,
    )?;
    Ok((outExp, outPrevVars))
}

fn collectPrevVars1(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inPrevVars: (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, Option<ArcStr>),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, Option<ArcStr>),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut outPrevVars: (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, Option<ArcStr>);
    outPrevVars = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut inPrevCompRefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut inForIter: Option<ArcStr>;
            let mut forIter: ArcStr;
            let mut cr = (*cr).clone();
            (inPrevCompRefs, inForIter) = inPrevVars;
            let () = (match inForIter.clone() {
        Some(mut __esc_forIter) => {
            forIter = __esc_forIter.clone();
            cr = ComponentReference::crefStripIterSub(metamodelica::AsArg::as_arg(&cr), &forIter);
            ()
        },
        _ => (),
    });
            (metamodelica::cons(cr.clone(), inPrevCompRefs), inForIter)
        },
        _ => {
            inPrevVars
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outPrevVars)
}

fn subClockPartitioning1(
    mut inSysts: &metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
    mut inHoldComps: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    metamodelica::Ref<BackendDAE::Shared>,
)> {
    let mut outSysts: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut outShared: metamodelica::Ref<BackendDAE::Shared> = inShared;
    let mut baseClock: metamodelica::Ref<DAE::ClockKind>;
    let mut varsPartition: (
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
    let mut i: i32;
    let mut j: i32;
    let mut n: i32;
    let mut nBaseClocks: i32;
    let mut cr: metamodelica::Ref<DAE::ComponentRef> = metamodelica::Ref::new(DAE::ComponentRef::WILD);
    let mut hasHoldOperator: metamodelica::Array<bool>;
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut lstSubClocks1: metamodelica::List<BackendDAE::SubClock>;
    let mut lstSubClocks: metamodelica::List<BackendDAE::SubClock> = metamodelica::nil();
    let mut partitionsInfo: BackendDAE::PartitionsInfo;
    let mut basePartitions: metamodelica::Array<BackendDAE::BasePartition>;
    let mut subPartitions: metamodelica::Array<BackendDAE::SubPartition>;
    nBaseClocks = ((inSysts).len() as i32);
    basePartitions = arrayCreate(
        nBaseClocks,
        BackendDAE::BasePartition {
            clock: openmodelica_frontend_types::DAE::ClockKind::interned_INFERRED_CLOCK(),
            nSubClocks: 0,
        },
    );
    varsPartition = HashTable::emptyHashTable();
    i = 0;
    j = 1;
    for mut syst in &**inSysts {
        (systs, baseClock, lstSubClocks1) = subClockPartitioning(syst.clone(), &outShared, i)?;
        n = ((systs).len() as i32);
        metamodelica::arrayUpdate(
            basePartitions.clone(),
            j,
            BackendDAE::BasePartition {
                clock: baseClock,
                nSubClocks: n,
            },
        )?;
        outSysts = List::append_reverse(&systs, outSysts);
        lstSubClocks = List::append_reverse(&lstSubClocks1, lstSubClocks);
        i = i + n;
        j = j + 1;
    }
    outSysts = metamodelica::Dangerous::listReverseInPlace(outSysts);
    lstSubClocks = metamodelica::Dangerous::listReverseInPlace(lstSubClocks);
    hasHoldOperator = arrayCreate(((lstSubClocks).len() as i32), false);
    i = 1;
    for mut syst in &*outSysts {
        for mut j in 1..=BackendVariable::varsSize(&syst.orderedVars) {
            let __arc1 = BackendVariable::getVarAt(&syst.orderedVars, j)?;
            let BackendDAE::VAR { varName: __pa0, .. } = &*__arc1;
            cr = metamodelica::Own::own(__pa0);
            varsPartition = BaseHashTable::add((cr, i), varsPartition)?;
        }
        i = i + 1;
    }
    for mut cr in &**inHoldComps {
        let mut cr = cr.clone();
        i = BaseHashTable::get(cr, &varsPartition)?;
        metamodelica::arrayUpdate(hasHoldOperator.clone(), i, true)?;
    }
    i = 1;
    subPartitions = arrayCreate(
        ((lstSubClocks).len() as i32),
        BackendDAE::SubPartition {
            clock: BackendDAE::DEFAULT_SUBCLOCK.clone(),
            holdEvents: false,
            prevVars: metamodelica::nil(),
        },
    );
    for mut subclock in &*lstSubClocks {
        metamodelica::arrayUpdate(
            subPartitions.clone(),
            i,
            BackendDAE::SubPartition {
                clock: subclock.clone(),
                holdEvents: ({
                    let __elt = (*metamodelica::index_checked(&hasHoldOperator.borrow(), i)?).clone();
                    __elt
                }),
                prevVars: metamodelica::nil(),
            },
        )?;
        i = i + 1;
    }
    partitionsInfo = outShared.partitionsInfo.clone();
    partitionsInfo.basePartitions = basePartitions.clone();
    partitionsInfo.subPartitions = subPartitions.clone();
    assign_field!(outShared.partitionsInfo = partitionsInfo);
    Ok((outSysts, outShared))
}

fn removeHoldExpsSyst(
    mut inSysts: &metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut outSysts: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut outHoldComps: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    for mut syst1 in &**inSysts {
        let mut syst1 = syst1.clone();
        syst1 = (::match_deref::match_deref! { match &(syst1) {
            syst @ Deref @ BackendDAE::EqSystem { orderedEqs: eqs, .. } => {
                let mut lstEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut i: i32 = 0;
                let mut eq: metamodelica::Ref<BackendDAE::Equation>;
                let mut syst = (*syst).clone();
                lstEqs = metamodelica::nil();
                for mut i in 1..=BackendEquation::getNumberOfEquations(eqs.clone()) {
                    eq = BackendEquation::get(eqs.clone(), i)?;
                    (eq, outHoldComps) = BackendEquation::traverseExpsOfEquation(eq, (std::sync::Arc::new(removeHoldExp1) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)> + 'static>), outHoldComps)?;
                    lstEqs = metamodelica::cons(eq, lstEqs);
                }
                assign_field!(syst.orderedEqs = BackendEquation::listEquation(&(lstEqs.reverse()))?);
                syst.clone()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        outSysts = metamodelica::cons(BackendDAEUtil::clearEqSyst(&syst1), outSysts);
    }
    Ok((outSysts, outHoldComps))
}

fn removeHoldExp1(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inComps: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outComps: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (outExp, outComps) = Expression::traverseExpBottomUp(inExp, &removeHoldExp, inComps)?;
    Ok((outExp, outComps))
}

fn removeHoldExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inComps: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outComps: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (outExp, outComps) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "hold" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, attr: _ } => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let __pa0 = ::match_deref::match_deref! { match &(e.clone()) {
                Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa0);
            (substGetPartition(e.clone())?, metamodelica::cons(cr, inComps))
        },
        _ => {
            (inExp, inComps)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outComps))
}

fn getSubPartitionAdjacency(
    mut numPartitions: i32,
    mut baseClockEq: i32,
    mut subPartitionInterfaceEqs: &metamodelica::List<i32>,
    mut eqPartMap: metamodelica::Array<i32>,
    mut varPartMap: metamodelica::Array<i32>,
    mut clockedVarsMask: metamodelica::Array<bool>,
    mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut vars: &BackendDAE::Variables,
) -> Result<(
    metamodelica::Array<metamodelica::List<(i32, BackendDAE::SubClock)>>,
    metamodelica::Array<i32>,
)> {
    let __ab_clockedVarsMask = clockedVarsMask.borrow();
    let mut partAdjacency: metamodelica::Array<metamodelica::List<(i32, BackendDAE::SubClock)>>;
    let mut order: metamodelica::Array<i32>;
    let mut infered: bool;
    let mut part: i32;
    let mut part1: i32;
    let mut part2: i32;
    let mut var1: i32;
    let mut var2: i32;
    let mut partLst: metamodelica::List<i32>;
    let mut orderLst: metamodelica::List<i32>;
    let mut subClk1: BackendDAE::SubClock;
    let mut subClk2: BackendDAE::SubClock;
    let mut partitionParents: metamodelica::Array<i32>;
    let mut partitionParentsVisited: metamodelica::Array<bool>;
    let mut partitionInterfacesClockVars: metamodelica::Array<bool>;
    partAdjacency = arrayCreate(numPartitions, metamodelica::nil());
    partitionParents = arrayCreate(numPartitions, -1);
    partitionInterfacesClockVars = arrayCreate(numPartitions, false);
    for mut subPartEq in &**subPartitionInterfaceEqs {
        (infered, part1, var1, subClk1, part2, var2, subClk2) = getConnectedSubPartitions(
            &(BackendEquation::get(eqs.clone(), subPartEq.clone())?),
            varPartMap.clone(),
            vars,
        )?;
        if part1 != 0 && part2 != 0 {
            addPartAdjacencyEdge(part1, subClk1, part2, subClk2, partAdjacency.clone())?;
        }
        if ({
            let __elt = (*metamodelica::index_checked(&partitionParents.borrow(), part2)?).clone();
            __elt
        }) == part1
            && ({
                let __elt = (*metamodelica::index_checked(&partitionInterfacesClockVars.borrow(), part2)?).clone();
                __elt
            })
        {
            {
                let __cell0 = -1;
                let __idx0 = part2;
                *metamodelica::index_mut_checked(&mut partitionParents.clone().borrow_mut(), __idx0)? = __cell0;
            }
        }
        {
            let __cell1 = !((*metamodelica::index_checked(&__ab_clockedVarsMask, var1)?).clone()
                && (*metamodelica::index_checked(&__ab_clockedVarsMask, var2)?).clone());
            let __idx1 = part1;
            *metamodelica::index_mut_checked(&mut partitionInterfacesClockVars.clone().borrow_mut(), __idx1)? = __cell1;
        }
        if ({
            let __elt = (*metamodelica::index_checked(&partitionParents.borrow(), part2)?).clone();
            __elt
        }) != part1
        {
            {
                let __cell2 = part2;
                let __idx2 = part1;
                *metamodelica::index_mut_checked(&mut partitionParents.clone().borrow_mut(), __idx2)? = __cell2;
            }
        }
    }
    partLst = List::intRange(numPartitions);
    partitionParentsVisited = arrayCreate(numPartitions, false);
    orderLst = metamodelica::nil();
    while !((partLst).is_empty()) {
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(partLst) {
            Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
            _ => return Err("pattern mismatch"),
        } };
        part = metamodelica::Own::own(__pa3);
        partLst = metamodelica::Own::own(__pa4);
        if !({
            let __elt = (*metamodelica::index_checked(&partitionParentsVisited.borrow(), part)?).clone();
            __elt
        }) {
            if ({
                let __elt = (*metamodelica::index_checked(&partitionParents.borrow(), part)?).clone();
                __elt
            }) == -1
                || ({
                    let __elt = (*metamodelica::index_checked(&partitionParents.borrow(), part)?).clone();
                    __elt
                }) == part
            {
                orderLst = metamodelica::cons(part, orderLst);
                {
                    let __cell5 = true;
                    let __idx5 = part;
                    *metamodelica::index_mut_checked(&mut partitionParentsVisited.clone().borrow_mut(), __idx5)? =
                        __cell5;
                }
            } else if ({
                let __elt = (*metamodelica::index_checked(
                    &partitionParentsVisited.borrow(),
                    ({
                        let __elt = (*metamodelica::index_checked(&partitionParents.borrow(), part)?).clone();
                        __elt
                    }),
                )?)
                .clone();
                __elt
            }) {
                orderLst = metamodelica::cons(part, orderLst);
                {
                    let __cell6 = true;
                    let __idx6 = part;
                    *metamodelica::index_mut_checked(&mut partitionParentsVisited.clone().borrow_mut(), __idx6)? =
                        __cell6;
                }
            } else {
                partLst = metamodelica::cons(part, partLst);
                partLst = metamodelica::cons(
                    ({
                        let __elt = (*metamodelica::index_checked(&partitionParents.borrow(), part)?).clone();
                        __elt
                    }),
                    partLst,
                );
            }
        }
    }
    order = metamodelica::arrayFromVec(orderLst.reverse().into_iter().cloned().collect());
    Ok((partAdjacency, order))
}

fn getSubClockForClkConstructor(
    mut refClock: &metamodelica::Ref<DAE::ClockKind>,
    mut clk: &metamodelica::Ref<DAE::ClockKind>,
) -> Result<BackendDAE::SubClock> {
    let mut subClk: BackendDAE::SubClock;
    subClk = (::match_deref::match_deref! { match (refClock, clk) {
        (Deref @ DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: Deref @ DAE::Exp::ICONST { integer: i1 }, resolution: Deref @ DAE::Exp::ICONST { integer: i2 } }, Deref @ DAE::ClockKind::INFERRED_CLOCK { .. }) => {
            BackendDAE::SubClock::SUBCLOCK { factor: MMath::Rational { nom: i2.clone(), denom: i1.clone() }, shift: MMath::RAT0.clone(), solver: None }
        },
        (Deref @ DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: Deref @ DAE::Exp::ICONST { integer: i1 }, resolution: Deref @ DAE::Exp::ICONST { integer: i2 } }, Deref @ DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: Deref @ DAE::Exp::ICONST { integer: i3 }, resolution: Deref @ DAE::Exp::ICONST { integer: i4 } }) => {
            BackendDAE::SubClock::SUBCLOCK { factor: MMath::divRational(MMath::Rational { nom: i2.clone(), denom: i1.clone() }, MMath::Rational { nom: i4.clone(), denom: i3.clone() })?, shift: MMath::RAT0.clone(), solver: None }
        },
        (Deref @ DAE::ClockKind::REAL_CLOCK { interval: Deref @ DAE::Exp::RCONST { real: r1 } }, Deref @ DAE::ClockKind::INFERRED_CLOCK { .. }) => {
            BackendDAE::SubClock::SUBCLOCK { factor: MMath::Rational { nom: 1, denom: ((metamodelica::real_div_checked(metamodelica::OrderedFloat(1.0_f64), r1.clone())?).0.floor() as i32) }, shift: MMath::RAT0.clone(), solver: None }
        },
        (Deref @ DAE::ClockKind::REAL_CLOCK { interval: Deref @ DAE::Exp::RCONST { real: r1 } }, Deref @ DAE::ClockKind::REAL_CLOCK { interval: Deref @ DAE::Exp::RCONST { real: r2 } }) => {
            BackendDAE::SubClock::SUBCLOCK { factor: MMath::divRational(MMath::Rational { nom: 1, denom: ((metamodelica::real_div_checked(metamodelica::OrderedFloat(1.0_f64), r1.clone())?).0.floor() as i32) }, MMath::Rational { nom: 1, denom: ((metamodelica::real_div_checked(metamodelica::OrderedFloat(1.0_f64), r2.clone())?).0.floor() as i32) })?, shift: MMath::RAT0.clone(), solver: None }
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SynchronousFeatures.getSubClockForClkConstructor")); __mm_s.push_str(&*literal!(" failed.\n")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/SynchronousFeatures.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(subClk)
}

fn setSolverSubClock(
    mut baseClkIn: metamodelica::Ref<DAE::ClockKind>,
    mut inSubClock: BackendDAE::SubClock,
) -> (metamodelica::Ref<DAE::ClockKind>, BackendDAE::SubClock) {
    let mut baseClkOut: metamodelica::Ref<DAE::ClockKind>;
    let mut outSubClock: BackendDAE::SubClock;
    (baseClkOut, outSubClock) = (::match_deref::match_deref! { match &(baseClkIn.clone()) {
        Deref @ DAE::ClockKind::SOLVER_CLOCK { c: Deref @ DAE::Exp::CLKCONST { clk }, solverMethod: Deref @ DAE::Exp::SCONST { string: solver } } => {
            outSubClock = setSubClockSolver(inSubClock, if (metamodelica::stringEq(&solver, &(literal!("")))) {None} else {Some(solver.clone())});
            (clk.clone(), outSubClock)
        },
        _ => {
            (baseClkIn, inSubClock)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (baseClkOut, outSubClock)
}

fn findSubClocks(
    mut numPartitions: i32,
    mut baseClockEq: i32,
    mut baseClk: metamodelica::Ref<DAE::ClockKind>,
    mut baseClockConstructors: &metamodelica::List<i32>,
    mut subPartitionInterfaceEqs: &metamodelica::List<i32>,
    mut eqPartMap: metamodelica::Array<i32>,
    mut varPartMap: metamodelica::Array<i32>,
    mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut partAdjacency: metamodelica::Array<metamodelica::List<(i32, BackendDAE::SubClock)>>,
) -> Result<(
    metamodelica::Ref<DAE::ClockKind>,
    metamodelica::Array<BackendDAE::SubClock>,
)> {
    let mut baseClkOut: metamodelica::Ref<DAE::ClockKind>;
    let mut outSubClocks: metamodelica::Array<BackendDAE::SubClock>;
    let mut part1: i32;
    let mut part2: i32;
    let mut partLst: metamodelica::List<i32>;
    let mut subClk1: BackendDAE::SubClock;
    let mut subClk2: BackendDAE::SubClock;
    let mut clk: metamodelica::Ref<DAE::ClockKind>;
    let mut partIsAssigned: metamodelica::Array<bool>;
    let mut adjParts: metamodelica::List<(i32, BackendDAE::SubClock)>;
    outSubClocks = arrayCreate(numPartitions, BackendDAE::DEFAULT_SUBCLOCK.clone());
    partIsAssigned = arrayCreate(numPartitions, false);
    for mut clockEq in &**baseClockConstructors {
        if !(intEq(baseClockEq, clockEq.clone())) && !(intEq(baseClockEq, -1)) {
            part1 = metamodelica::arrayGet(eqPartMap.clone(), clockEq.clone())?;
            clk = getBaseClock(&(BackendEquation::get(eqs.clone(), clockEq.clone())?));
            if !(isInferedBaseClock(&clk)) {
                subClk1 = getSubClockForClkConstructor(&baseClk, &clk)?;
                metamodelica::arrayUpdate(outSubClocks.clone(), part1, subClk1)?;
                metamodelica::arrayUpdate(partIsAssigned.clone(), part1, true)?;
            }
        }
    }
    if isInferedBaseClock(&baseClk) {
        baseClkOut = baseClk;
        partLst = List::intRange(numPartitions);
    } else {
        part1 = metamodelica::arrayGet(eqPartMap.clone(), baseClockEq)?;
        partLst = metamodelica::cons(part1, List::intRange(numPartitions));
        (baseClkOut, subClk1) = setSolverSubClock(
            baseClk,
            ({
                let __elt = (*metamodelica::index_checked(&outSubClocks.borrow(), part1)?).clone();
                __elt
            }),
        );
        metamodelica::arrayUpdate(outSubClocks.clone(), part1, subClk1)?;
        metamodelica::arrayUpdate(partIsAssigned.clone(), part1, true)?;
    }
    while !((partLst).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(partLst) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        part1 = metamodelica::Own::own(__pa0);
        partLst = metamodelica::Own::own(__pa1);
        adjParts = metamodelica::arrayGet(partAdjacency.clone(), part1)?;
        for mut adjPart in &*adjParts {
            part2 = Util::tuple21(adjPart.clone());
            if !(metamodelica::arrayGet(partIsAssigned.clone(), part2)?) {
                subClk1 = metamodelica::arrayGet(outSubClocks.clone(), part1)?;
                subClk2 = Util::tuple22(adjPart.clone());
                subClk2 = computeAbsoluteSubClock(&subClk1, subClk2)?;
                if !(isInferedSubClock(&subClk2)) {
                    metamodelica::arrayUpdate(outSubClocks.clone(), part2, subClk2)?;
                    metamodelica::arrayUpdate(partIsAssigned.clone(), part2, true)?;
                    partLst = metamodelica::cons(part2, partLst);
                }
            }
        }
    }
    Ok((baseClkOut, outSubClocks))
}

fn computeAbsoluteSubClock(
    mut preClock: &BackendDAE::SubClock,
    mut subSeqClock: BackendDAE::SubClock,
) -> Result<BackendDAE::SubClock> {
    let mut subClk: BackendDAE::SubClock = BackendDAE::DEFAULT_SUBCLOCK.clone();
    subClk = (match (preClock.clone(), subSeqClock.clone()) {
        (
            BackendDAE::SubClock::SUBCLOCK {
                factor: mut f1,
                shift: mut s1,
                solver: mut solver1,
            },
            BackendDAE::SubClock::SUBCLOCK {
                factor: mut f2,
                shift: mut s2,
                solver: mut solver2,
            },
        ) => {
            solver1 = mergeSolver(solver1.clone(), solver2.clone())?;
            BackendDAE::SubClock::SUBCLOCK {
                factor: MMath::divRational(f1.clone(), f2.clone())?,
                shift: MMath::addRational(MMath::multRational(s1.clone(), f2.clone())?, s2.clone())?,
                solver: solver1.clone(),
            }
        }
        (
            BackendDAE::SubClock::SUBCLOCK {
                factor: _,
                shift: _,
                solver: _,
            },
            BackendDAE::SubClock::INFERED_SUBCLOCK { .. },
        ) => subSeqClock,
        _ => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("SynchronousFeatures.computeAbsoluteSubClock"));
                    __mm_s.push_str(&*literal!(" failed.\n"));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("BackEnd/SynchronousFeatures.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(subClk)
}

fn mergeSolver(mut solver1: Option<ArcStr>, mut solver2: Option<ArcStr>) -> Result<Option<ArcStr>> {
    let mut sOut: Option<ArcStr>;
    sOut = (match (solver1, solver2) {
        (None, Some(mut s2)) => Some(s2),
        (Some(mut s1), None) => Some(s1),
        (Some(mut s1), Some(mut s2)) => {
            if !(stringEq(&s1, &s2)) {
                Error::addCompilerNotification({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Infered sub clock partitions have different solvers:"));
                    __mm_s.push_str(&*s1);
                    __mm_s.push_str(&*literal!(" <->"));
                    __mm_s.push_str(&*s2);
                    __mm_s.push_str(&*literal!(".\n"));
                    ArcStr::from(__mm_s)
                })?;
            }
            Some(s1)
        }
        _ => None,
    });
    Ok(sOut)
}

fn addPartAdjacencyEdge(
    mut part1: i32,
    mut sub1: BackendDAE::SubClock,
    mut part2: i32,
    mut sub2: BackendDAE::SubClock,
    mut partAdjacency: metamodelica::Array<metamodelica::List<(i32, BackendDAE::SubClock)>>,
) -> Result<()> {
    let mut partEdges: metamodelica::List<(i32, BackendDAE::SubClock)>;
    if intGt(part1, 0) && intGt(part2, 0) {
        partEdges = metamodelica::arrayGet(partAdjacency.clone(), part1)?;
        for mut edge in &*partEdges {
            if intEq(Util::tuple21(edge.clone()), part2) {}
        }
        metamodelica::arrayUpdate(
            partAdjacency.clone(),
            part1,
            metamodelica::cons((part2, sub1), partEdges),
        )?;
        partEdges = metamodelica::arrayGet(partAdjacency.clone(), part2)?;
        metamodelica::arrayUpdate(
            partAdjacency.clone(),
            part2,
            metamodelica::cons((part1, sub2), partEdges),
        )?;
    }
    Ok(())
}

fn setSubClockFactor(mut subClk: BackendDAE::SubClock, mut factor: MMath::Rational) -> BackendDAE::SubClock {
    let mut subClkOut: BackendDAE::SubClock;
    subClkOut = (match subClk.clone() {
        BackendDAE::SubClock::SUBCLOCK {
            factor: _,
            shift: mut shift,
            solver: mut solver,
        } => BackendDAE::SubClock::SUBCLOCK {
            factor: factor,
            shift: shift.clone(),
            solver: solver.clone(),
        },
        _ => subClk,
    });
    subClkOut
}

fn getSubClockFactor(mut subClk: &BackendDAE::SubClock) -> MMath::Rational {
    let mut factor: MMath::Rational;
    factor = (match subClk.clone() {
        BackendDAE::SubClock::SUBCLOCK {
            factor: mut __esc_factor,
            shift: _,
            solver: _,
        } => {
            factor = __esc_factor.clone();
            factor
        }
        _ => MMath::RAT1.clone(),
    });
    factor
}

fn getSubClockShift(mut subClk: &BackendDAE::SubClock) -> MMath::Rational {
    let mut shift: MMath::Rational;
    shift = (match subClk.clone() {
        BackendDAE::SubClock::SUBCLOCK {
            factor: _,
            shift: mut __esc_shift,
            solver: _,
        } => {
            shift = __esc_shift.clone();
            shift
        }
        _ => MMath::RAT0.clone(),
    });
    shift
}

fn getSubClockSolverOpt(mut subClk: &BackendDAE::SubClock) -> Option<ArcStr> {
    let mut solver: Option<ArcStr>;
    solver = (match subClk.clone() {
        BackendDAE::SubClock::SUBCLOCK {
            factor: _,
            shift: _,
            solver: mut __esc_solver,
        } => {
            solver = __esc_solver.clone();
            solver
        }
        _ => None,
    });
    solver
}

fn setSubClockShift(mut subClk: BackendDAE::SubClock, mut shift: MMath::Rational) -> BackendDAE::SubClock {
    let mut subClkOut: BackendDAE::SubClock;
    subClkOut = (match subClk.clone() {
        BackendDAE::SubClock::SUBCLOCK {
            factor: mut factor,
            shift: _,
            solver: mut solver,
        } => BackendDAE::SubClock::SUBCLOCK {
            factor: factor.clone(),
            shift: shift,
            solver: solver.clone(),
        },
        _ => subClk,
    });
    subClkOut
}

fn setSubClockSolver(mut subClk: BackendDAE::SubClock, mut solver: Option<ArcStr>) -> BackendDAE::SubClock {
    let mut subClkOut: BackendDAE::SubClock;
    subClkOut = (match subClk.clone() {
        BackendDAE::SubClock::SUBCLOCK {
            factor: mut factor,
            shift: mut shift,
            solver: _,
        } => BackendDAE::SubClock::SUBCLOCK {
            factor: factor.clone(),
            shift: shift.clone(),
            solver: solver,
        },
        _ => subClk,
    });
    subClkOut
}

fn getConnectedSubPartitions(
    mut eq: &metamodelica::Ref<BackendDAE::Equation>,
    mut varPartMap: metamodelica::Array<i32>,
    mut vars: &BackendDAE::Variables,
) -> Result<(bool, i32, i32, BackendDAE::SubClock, i32, i32, BackendDAE::SubClock)> {
    let __ab_varPartMap = varPartMap.borrow();
    let mut infered: bool = false;
    let mut part1: i32;
    let mut var1: i32 = -1;
    let mut sub1: BackendDAE::SubClock;
    let mut part2: i32;
    let mut var2: i32 = -1;
    let mut sub2: BackendDAE::SubClock;
    sub1 = BackendDAE::DEFAULT_SUBCLOCK.clone();
    sub2 = BackendDAE::DEFAULT_SUBCLOCK.clone();
    (part1, var1, part2, var2) = (::match_deref::match_deref! { match eq {
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "superSample" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref2, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: factor }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, .. } => {
            let mut v1: i32;
            let mut v2: i32;
            let mut p1: i32;
            let mut p2: i32;
            infered = intEq(factor.clone(), 0);
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars)?) {
                (_, Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            v1 = metamodelica::Own::own(__pa0);
            p1 = (*metamodelica::index_checked(&__ab_varPartMap, v1)?).clone();
            let __pa2 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref2.clone(), vars)?) {
                (_, Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil }) => __pa2.clone(),
                _ => return Err("pattern mismatch"),
            } };
            v2 = metamodelica::Own::own(__pa2);
            p2 = (*metamodelica::index_checked(&__ab_varPartMap, v2)?).clone();
            if infered {
                sub1 = openmodelica_backend_types::BackendDAE::SubClock::INFERED_SUBCLOCK;
                sub2 = openmodelica_backend_types::BackendDAE::SubClock::INFERED_SUBCLOCK;
            } else {
                sub1 = setSubClockFactor(sub1, MMath::divRational(MMath::RAT1.clone(), MMath::Rational { nom: factor.clone(), denom: 1 })?);
                sub2 = setSubClockFactor(sub2, MMath::Rational { nom: factor.clone(), denom: 1 });
            }
            (p1, v1, p2, v2)
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "subSample" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref2, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: factor }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, .. } => {
            let mut v1: i32;
            let mut v2: i32;
            let mut p1: i32;
            let mut p2: i32;
            infered = intEq(factor.clone(), 0);
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars)?) {
                (_, Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            v1 = metamodelica::Own::own(__pa0);
            p1 = (*metamodelica::index_checked(&__ab_varPartMap, v1)?).clone();
            let __pa2 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref2.clone(), vars)?) {
                (_, Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil }) => __pa2.clone(),
                _ => return Err("pattern mismatch"),
            } };
            v2 = metamodelica::Own::own(__pa2);
            p2 = (*metamodelica::index_checked(&__ab_varPartMap, v2)?).clone();
            if infered {
                sub1 = openmodelica_backend_types::BackendDAE::SubClock::INFERED_SUBCLOCK;
                sub2 = openmodelica_backend_types::BackendDAE::SubClock::INFERED_SUBCLOCK;
            } else {
                sub1 = setSubClockFactor(sub1, MMath::Rational { nom: factor.clone(), denom: 1 });
                sub2 = setSubClockFactor(sub2, MMath::divRational(MMath::RAT1.clone(), MMath::Rational { nom: factor.clone(), denom: 1 })?);
            }
            (p1, v1, p2, v2)
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "shiftSample" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref2, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: counter }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: resolution }, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, .. } => {
            let mut v1: i32;
            let mut v2: i32;
            let mut p1: i32;
            let mut p2: i32;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars)?) {
                (_, Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            v1 = metamodelica::Own::own(__pa0);
            p1 = (*metamodelica::index_checked(&__ab_varPartMap, v1)?).clone();
            let __pa2 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref2.clone(), vars)?) {
                (_, Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil }) => __pa2.clone(),
                _ => return Err("pattern mismatch"),
            } };
            v2 = metamodelica::Own::own(__pa2);
            p2 = (*metamodelica::index_checked(&__ab_varPartMap, v2)?).clone();
            sub1 = setSubClockShift(sub1, MMath::subRational(MMath::RAT0.clone(), MMath::Rational { nom: counter.clone(), denom: resolution.clone() })?);
            sub2 = setSubClockShift(sub2, MMath::Rational { nom: counter.clone(), denom: resolution.clone() });
            (p1, v1, p2, v2)
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "backSample" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref2, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: counter }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: resolution }, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, .. } => {
            let mut v1: i32;
            let mut v2: i32;
            let mut p1: i32;
            let mut p2: i32;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars)?) {
                (_, Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            v1 = metamodelica::Own::own(__pa0);
            p1 = (*metamodelica::index_checked(&__ab_varPartMap, v1)?).clone();
            let __pa2 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref2.clone(), vars)?) {
                (_, Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil }) => __pa2.clone(),
                _ => return Err("pattern mismatch"),
            } };
            v2 = metamodelica::Own::own(__pa2);
            p2 = (*metamodelica::index_checked(&__ab_varPartMap, v2)?).clone();
            sub1 = setSubClockShift(sub1, MMath::Rational { nom: counter.clone(), denom: resolution.clone() });
            sub2 = setSubClockShift(sub2, MMath::subRational(MMath::RAT0.clone(), MMath::Rational { nom: counter.clone(), denom: resolution.clone() })?);
            (p1, v1, p2, v2)
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::SOLVER_CLOCK { c: Deref @ DAE::Exp::CREF { componentRef: cref2, .. }, solverMethod: Deref @ DAE::Exp::SCONST { string: solver } } }, .. } => {
            let mut v1: i32;
            let mut v2: i32;
            let mut p1: i32;
            let mut p2: i32;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars)?) {
                (_, Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            v1 = metamodelica::Own::own(__pa0);
            p1 = (*metamodelica::index_checked(&__ab_varPartMap, v1)?).clone();
            let __pa2 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref2.clone(), vars)?) {
                (_, Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil }) => __pa2.clone(),
                _ => return Err("pattern mismatch"),
            } };
            v2 = metamodelica::Own::own(__pa2);
            p2 = (*metamodelica::index_checked(&__ab_varPartMap, v2)?).clone();
            sub1 = setSubClockSolver(sub1, Some(solver.clone()));
            sub2 = setSubClockSolver(sub2, Some(solver.clone()));
            (p1, v1, p2, v2)
        },
        _ => {
            (-1, -1, -1, -1)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((infered, part1, var1, sub1, part2, var2, sub2))
}

fn chooseBaseClock(
    mut clockEqs: &metamodelica::List<i32>,
    mut numPartitions: i32,
    mut eqPartMap: metamodelica::Array<i32>,
    mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
) -> Result<(metamodelica::Ref<DAE::ClockKind>, i32)> {
    let mut outBaseClock: metamodelica::Ref<DAE::ClockKind> =
        openmodelica_frontend_types::DAE::ClockKind::interned_INFERRED_CLOCK();
    let mut baseClockEqIdx: i32 = -1;
    let mut subClkPartMap: metamodelica::Array<BackendDAE::SubClock>;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    subClkPartMap = arrayCreate(numPartitions, BackendDAE::DEFAULT_SUBCLOCK.clone());
    for mut clockEq in &**clockEqs {
        eq = BackendEquation::get(eqs.clone(), clockEq.clone())?;
        if isBaseClockEq(&eq) {
            outBaseClock = getBaseClock(&eq);
            baseClockEqIdx = clockEq.clone();
        }
    }
    Ok((outBaseClock, baseClockEqIdx))
}

fn isBaseClockEq(mut eq: &metamodelica::Ref<BackendDAE::Equation>) -> bool {
    let mut isBaseClock: bool;
    isBaseClock = (::match_deref::match_deref! { match eq {
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { .. }, scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::INFERRED_CLOCK { .. } }, .. } => {
            false
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { .. }, scalar: Deref @ DAE::Exp::CLKCONST { .. }, .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isBaseClock
}

fn getBaseClock(mut eq: &metamodelica::Ref<BackendDAE::Equation>) -> metamodelica::Ref<DAE::ClockKind> {
    let mut baseClk: metamodelica::Ref<DAE::ClockKind>;
    baseClk = (::match_deref::match_deref! { match eq {
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { .. }, scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::INFERRED_CLOCK { .. } }, .. } => {
            openmodelica_frontend_types::DAE::ClockKind::interned_INFERRED_CLOCK()
        },
        Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { .. }, scalar: Deref @ DAE::Exp::CLKCONST { clk }, .. } => {
            clk.clone()
        },
        _ => {
            openmodelica_frontend_types::DAE::ClockKind::interned_INFERRED_CLOCK()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    baseClk
}

fn removeEdge(
    mut eq: i32,
    mut var: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<()> {
    let mut row: metamodelica::List<i32>;
    row = metamodelica::arrayGet(m.clone(), eq)?;
    (row, _) = List::deleteMemberOnTrue(var, row, &fnptr!(intEq, i32, i32))?;
    metamodelica::arrayUpdate(m.clone(), eq, row)?;
    row = metamodelica::arrayGet(mT.clone(), var)?;
    (row, _) = List::deleteMemberOnTrue(eq, row, &fnptr!(intEq, i32, i32))?;
    metamodelica::arrayUpdate(mT.clone(), var, row)?;
    Ok(())
}

fn findBaseClockInterfaces(
    mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut vars: &BackendDAE::Variables,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut clockEqs: metamodelica::List<i32> = metamodelica::nil();
    let mut subClockInterfaceEqIdxs: metamodelica::List<i32> = metamodelica::nil();
    let mut subClockInterfaceEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut eqIdx: i32 = 0;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    for mut eqIdx in 1..=BackendEquation::getNumberOfEquations(eqs.clone()) {
        eq = BackendEquation::get(eqs.clone(), eqIdx)?;
        (clockEqs, subClockInterfaceEqIdxs, subClockInterfaceEqs) = findBaseClockInterfaces1(
            eq,
            eqIdx,
            eqs.clone(),
            vars,
            m.clone(),
            mT.clone(),
            clockEqs,
            subClockInterfaceEqIdxs,
            subClockInterfaceEqs,
        )?;
    }
    Ok((clockEqs, subClockInterfaceEqIdxs, subClockInterfaceEqs))
}

fn findBaseClockInterfaces1(
    mut eq: metamodelica::Ref<BackendDAE::Equation>,
    mut eqIdx: i32,
    mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut vars: &BackendDAE::Variables,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut clockEqsIn: metamodelica::List<i32>,
    mut subClockInterfaceEqIdxsIn: metamodelica::List<i32>,
    mut subClockInterfaceEqsIn: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<(
    metamodelica::List<i32>,
    metamodelica::List<i32>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut clockEqsOut: metamodelica::List<i32>;
    let mut subClockInterfaceEqIdxsOut: metamodelica::List<i32>;
    let mut subClockInterfaceEqsOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    (clockEqsOut, subClockInterfaceEqIdxsOut, subClockInterfaceEqsOut) = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::INFERRED_CLOCK { .. } }, .. } => {
            (metamodelica::cons(eqIdx, clockEqsIn), subClockInterfaceEqIdxsIn, subClockInterfaceEqsIn)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: _, .. } }, .. } => {
            (metamodelica::cons(eqIdx, clockEqsIn), subClockInterfaceEqIdxsIn, subClockInterfaceEqsIn)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::REAL_CLOCK { interval: _ } }, .. } => {
            (metamodelica::cons(eqIdx, clockEqsIn), subClockInterfaceEqIdxsIn, subClockInterfaceEqsIn)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::EVENT_CLOCK { condition: _, .. } }, .. } => {
            (metamodelica::cons(eqIdx, clockEqsIn), subClockInterfaceEqIdxsIn, subClockInterfaceEqsIn)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::SOLVER_CLOCK { c: Deref @ DAE::Exp::CREF { componentRef: _, .. }, solverMethod: _ } }, .. } => {
            (clockEqsIn, metamodelica::cons(eqIdx, subClockInterfaceEqIdxsIn), metamodelica::cons(eq, subClockInterfaceEqsIn))
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::SOLVER_CLOCK { c: Deref @ DAE::Exp::CLKCONST { clk: _ }, solverMethod: _ } }, .. } => {
            (metamodelica::cons(eqIdx, clockEqsIn), subClockInterfaceEqIdxsIn, subClockInterfaceEqsIn)
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "superSample" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, .. } => {
            let mut varIdx: i32;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars)?) {
                (_, Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            varIdx = metamodelica::Own::own(__pa0);
            removeEdge(eqIdx, varIdx, m.clone(), mT.clone())?;
            (clockEqsIn, metamodelica::cons(eqIdx, subClockInterfaceEqIdxsIn), metamodelica::cons(eq, subClockInterfaceEqsIn))
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "subSample" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, .. } => {
            let mut varIdx: i32;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars)?) {
                (_, Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            varIdx = metamodelica::Own::own(__pa0);
            removeEdge(eqIdx, varIdx, m.clone(), mT.clone())?;
            (clockEqsIn, metamodelica::cons(eqIdx, subClockInterfaceEqIdxsIn), metamodelica::cons(eq, subClockInterfaceEqsIn))
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "shiftSample" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, .. } => {
            let mut varIdx: i32;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars)?) {
                (_, Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            varIdx = metamodelica::Own::own(__pa0);
            removeEdge(eqIdx, varIdx, m.clone(), mT.clone())?;
            (clockEqsIn, metamodelica::cons(eqIdx, subClockInterfaceEqIdxsIn), metamodelica::cons(eq, subClockInterfaceEqsIn))
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "shiftSample" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, .. } => {
            let mut varIdx: i32;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars)?) {
                (_, Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            varIdx = metamodelica::Own::own(__pa0);
            removeEdge(eqIdx, varIdx, m.clone(), mT.clone())?;
            (clockEqsIn, metamodelica::cons(eqIdx, subClockInterfaceEqIdxsIn), metamodelica::cons(eq, subClockInterfaceEqsIn))
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "backSample" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, .. } => {
            let mut varIdx: i32;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars)?) {
                (_, Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            varIdx = metamodelica::Own::own(__pa0);
            removeEdge(eqIdx, varIdx, m.clone(), mT.clone())?;
            (clockEqsIn, metamodelica::cons(eqIdx, subClockInterfaceEqIdxsIn), metamodelica::cons(eq, subClockInterfaceEqsIn))
        },
        Deref @ BackendDAE::Equation::EQUATION { scalar: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "backSample" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref1, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, .. } => {
            let mut varIdx: i32;
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(cref1.clone(), vars)?) {
                (_, Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            varIdx = metamodelica::Own::own(__pa0);
            removeEdge(eqIdx, varIdx, m.clone(), mT.clone())?;
            (clockEqsIn, metamodelica::cons(eqIdx, subClockInterfaceEqIdxsIn), metamodelica::cons(eq, subClockInterfaceEqsIn))
        },
        Deref @ BackendDAE::Equation::EQUATION { .. } => {
            (clockEqsIn, subClockInterfaceEqIdxsIn, subClockInterfaceEqsIn)
        },
        _ => {
            (clockEqsIn, subClockInterfaceEqIdxsIn, subClockInterfaceEqsIn)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((clockEqsOut, subClockInterfaceEqIdxsOut, subClockInterfaceEqsOut))
}

fn findHighestWhenPrefixIdx(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut idxIn: i32,
) -> Result<(metamodelica::Ref<BackendDAE::Var>, i32)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var> = inVar.clone();
    let mut idxOut: i32 = idxIn;
    let mut name: metamodelica::Ref<DAE::ComponentRef>;
    let mut chars: metamodelica::List<ArcStr>;
    let mut chars1: metamodelica::List<ArcStr>;
    let mut chars2: metamodelica::List<ArcStr>;
    name = inVar.varName.clone();
    chars = stringListStringChar(ComponentReference::crefStr(&name)?);
    if intGt(((chars).len() as i32), 9) {
        (chars1, chars2) = List::split(chars, 8)?;
        if stringEq(
            &(stringDelimitList(chars1, literal!(""))),
            &arcstr::literal!(BackendDAE::WHENCLK_PRREFIX),
        ) {
            idxOut = intMax(idxIn, stringInt(stringDelimitList(chars2, literal!("")))?);
        }
    }
    Ok((outVar, idxOut))
}

fn replaceSampledClocks(
    mut eqsIn: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut varsIn: BackendDAE::Variables,
) -> Result<(
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    BackendDAE::Variables,
)> {
    let mut eqsOut: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut varsOut: BackendDAE::Variables;
    let mut prefIdx: i32;
    let mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut newEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut newVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    prefIdx = BackendVariable::traverseBackendDAEVars(
        varsIn.clone(),
        (std::sync::Arc::new(findHighestWhenPrefixIdx)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        i32,
                    ) -> Result<(metamodelica::Ref<BackendDAE::Var>, i32)>
                    + 'static,
            >),
        1,
    )?;
    let (__pa0, (_, _, __pa1, __pa2)) = BackendEquation::traverseEquationArray_WithUpdate(
        eqsIn,
        &replaceSampledClocks1,
        (varsIn.clone(), prefIdx + 1, metamodelica::nil(), metamodelica::nil()),
    )?;
    eqs = metamodelica::Own::own(__pa0);
    newEqs = metamodelica::Own::own(__pa1);
    newVars = metamodelica::Own::own(__pa2);
    eqsOut = BackendEquation::addList(&newEqs, eqs)?;
    varsOut = BackendVariable::addVars(&newVars, varsIn)?;
    Ok((eqsOut, varsOut))
}

fn replaceSampledClocks1(
    mut eqIn: metamodelica::Ref<BackendDAE::Equation>,
    mut tplIn: (
        BackendDAE::Variables,
        i32,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    ),
) -> Result<(
    metamodelica::Ref<BackendDAE::Equation>,
    (
        BackendDAE::Variables,
        i32,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    ),
)> {
    let mut eqOut: metamodelica::Ref<BackendDAE::Equation>;
    let mut tplOut: (
        BackendDAE::Variables,
        i32,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    );
    (eqOut, tplOut) = (::match_deref::match_deref! { match &((eqIn.clone(), tplIn.clone())) {
        (Deref @ BackendDAE::Equation::EQUATION { exp: e1, scalar: e2, source, attr: BackendDAE::EquationAttributes { kind: BackendDAE::EquationKind::DYNAMIC_EQUATION { .. }, .. } }, (vars, suffixIdx0, newEqs, newVars)) => {
            let mut suffixIdx: i32;
            let mut attr: BackendDAE::EquationAttributes;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            let mut newEqs = (*newEqs).clone();
            let mut newVars = (*newVars).clone();
            let (__pa0, (__pa1, __pa2, __pa3)) = Expression::traverseExpTopDown(e1.clone(), &replaceSampledClocks2, (newEqs.clone(), newVars.clone(), suffixIdx0.clone()))?;
            e1 = metamodelica::Own::own(__pa0);
            newEqs = metamodelica::Own::own(__pa1);
            newVars = metamodelica::Own::own(__pa2);
            suffixIdx = metamodelica::Own::own(__pa3);
            let (__pa4, (__pa5, __pa6, __pa7)) = Expression::traverseExpTopDown(e2.clone(), &replaceSampledClocks2, (newEqs.clone(), newVars.clone(), suffixIdx))?;
            e2 = metamodelica::Own::own(__pa4);
            newEqs = metamodelica::Own::own(__pa5);
            newVars = metamodelica::Own::own(__pa6);
            suffixIdx = metamodelica::Own::own(__pa7);
            if intEq(suffixIdx - suffixIdx0.clone(), 1) {
                attr = BackendEquation::defaultClockedEqAttr(suffixIdx0.clone());
            } else {
                attr = BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone();
            }
            (metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: e1.clone(), scalar: e2.clone(), source: source.clone(), attr: attr }), (vars.clone(), suffixIdx, newEqs.clone(), newVars.clone()))
        },
        (Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize, left: e1, right: e2, source, attr: BackendDAE::EquationAttributes { kind: BackendDAE::EquationKind::DYNAMIC_EQUATION { .. }, .. }, recordSize }, (vars, suffixIdx0, newEqs, newVars)) => {
            let mut suffixIdx: i32;
            let mut attr: BackendDAE::EquationAttributes;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            let mut newEqs = (*newEqs).clone();
            let mut newVars = (*newVars).clone();
            let (__pa0, (__pa1, __pa2, __pa3)) = Expression::traverseExpTopDown(e1.clone(), &replaceSampledClocks2, (newEqs.clone(), newVars.clone(), suffixIdx0.clone()))?;
            e1 = metamodelica::Own::own(__pa0);
            newEqs = metamodelica::Own::own(__pa1);
            newVars = metamodelica::Own::own(__pa2);
            suffixIdx = metamodelica::Own::own(__pa3);
            let (__pa4, (__pa5, __pa6, __pa7)) = Expression::traverseExpTopDown(e2.clone(), &replaceSampledClocks2, (newEqs.clone(), newVars.clone(), suffixIdx))?;
            e2 = metamodelica::Own::own(__pa4);
            newEqs = metamodelica::Own::own(__pa5);
            newVars = metamodelica::Own::own(__pa6);
            suffixIdx = metamodelica::Own::own(__pa7);
            if intEq(suffixIdx - suffixIdx0.clone(), 1) {
                attr = BackendEquation::defaultClockedEqAttr(suffixIdx0.clone());
            } else {
                attr = BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone();
            }
            (metamodelica::Ref::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: dimSize.clone(), left: e1.clone(), right: e2.clone(), source: source.clone(), attr: attr, recordSize: recordSize.clone() }), (vars.clone(), suffixIdx, newEqs.clone(), newVars.clone()))
        },
        _ => {
            (eqIn, tplIn)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((eqOut, tplOut))
}

fn replaceSampledClocks2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut tplIn: (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        i32,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        i32,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut tplOut: (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        i32,
    );
    (outExp, cont, tplOut) = (::match_deref::match_deref! { match &((inExp.clone(), tplIn.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, expLst: Deref @ metamodelica::ListNode::Cons { head: varExp @ Deref @ DAE::Exp::CREF { componentRef: _, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: clk @ Deref @ DAE::Exp::CLKCONST { clk: _ }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, (newEqs, newVars, suffixIdx)) => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut addEq: metamodelica::Ref<BackendDAE::Equation>;
            let mut addVar: metamodelica::Ref<BackendDAE::Var>;
            cr = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: { let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(BackendDAE::WHENCLK_PRREFIX)); __mm_s.push_str(&*intString(suffixIdx.clone())); ArcStr::from(__mm_s) }, identType: DAE::T_CLOCK_DEFAULT().clone(), subscriptLst: metamodelica::nil() });
            addVar = BackendVariable::makeVar(cr.clone())?;
            addEq = metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: Expression::crefToExp(cr)?, scalar: clk.clone(), source: DAE::emptyElementSource().clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
            (substGetPartition(varExp.clone())?, false, (metamodelica::cons(addEq, newEqs.clone()), metamodelica::cons(addVar, newVars.clone()), suffixIdx.clone() + 1))
        },
        _ => {
            (inExp, true, tplIn)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, tplOut))
}

fn subClockPartitioning(
    mut inEqSystem: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: &metamodelica::Ref<BackendDAE::Shared>,
    mut off: i32,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    metamodelica::Ref<DAE::ClockKind>,
    metamodelica::List<BackendDAE::SubClock>,
)> {
    let mut outSysts: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut outBaseClock: metamodelica::Ref<DAE::ClockKind>;
    let mut outSubClocks: metamodelica::List<BackendDAE::SubClock>;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut remEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut clockEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut clockVars: BackendDAE::Variables;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut mT: metamodelica::Array<metamodelica::List<i32>>;
    let mut rm: metamodelica::Array<metamodelica::List<i32>>;
    let mut rmT: metamodelica::Array<metamodelica::List<i32>>;
    let mut partitionsCnt: i32;
    let mut remEqPartMap: metamodelica::Array<i32>;
    let mut newClockEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut newClockVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut contPartitions: metamodelica::Array<Option<bool>>;
    let mut subclksCnt: metamodelica::Array<i32>;
    let mut order: metamodelica::Array<i32>;
    let mut subclocks: metamodelica::Array<BackendDAE::SubClock>;
    let mut clockedEqsMask: metamodelica::Array<bool>;
    let mut clockedVarsMask: metamodelica::Array<bool>;
    let mut usedVars: metamodelica::Array<bool>;
    let mut usedRemovedVars: metamodelica::Array<bool>;
    let mut baseClockEqIdx: i32;
    let mut eqIdx: i32 = 0;
    let mut varIdx: i32 = 0;
    let mut baseClockEquations: metamodelica::List<i32>;
    let mut subClockInterfaceEqIdxs: metamodelica::List<i32>;
    let mut subClockInterfaceEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut varPartMap: metamodelica::Array<i32>;
    let mut eqPartMap: metamodelica::Array<i32>;
    let mut partAdjacency: metamodelica::Array<metamodelica::List<(i32, BackendDAE::SubClock)>>;
    let mut sys: metamodelica::Ref<BackendDAE::EqSystem>;
    funcs = BackendDAEUtil::getFunctions(inShared);
    let __arc3 = inEqSystem.clone();
    let BackendDAE::EQSYSTEM {
        orderedVars: __pa0,
        orderedEqs: __pa1,
        removedEqs: __pa2,
        ..
    } = &*__arc3;
    vars = metamodelica::Own::own(__pa0);
    eqs = metamodelica::Own::own(__pa1);
    remEqs = metamodelica::Own::own(__pa2);
    (eqs, vars) = replaceSampledClocks(eqs, vars)?;
    sys = BackendDAEUtil::setEqSystVars(inEqSystem.clone(), vars.clone());
    sys = BackendDAEUtil::setEqSystEqs(sys, eqs.clone());
    (sys, m, mT) = BackendDAEUtil::getAdjacencyMatrix(
        sys,
        openmodelica_backend_types::BackendDAE::IndexType::SUBCLOCK_IDX,
        Some(funcs.clone()),
        BackendDAEUtil::isInitializationDAE(inShared),
    )?;
    (baseClockEquations, subClockInterfaceEqIdxs, subClockInterfaceEqs) =
        findBaseClockInterfaces(eqs.clone(), &vars, m.clone(), mT.clone())?;
    (clockEqs, clockedEqsMask) = splitClockEqs(eqs.clone())?;
    (clockVars, clockedVarsMask) = splitClockVars(&vars)?;
    (rm, rmT) = BackendDAEUtil::removedAdjacencyMatrix(
        &sys,
        openmodelica_backend_types::BackendDAE::IndexType::SUBCLOCK_IDX,
        Some(funcs.clone()),
        BackendDAEUtil::isInitializationDAE(inShared),
    )?;
    remEqPartMap = arrayCreate(metamodelica::arrayLength(rm.clone()), 0);
    eqPartMap = arrayCreate(metamodelica::arrayLength(m.clone()), 0);
    varPartMap = arrayCreate(metamodelica::arrayLength(mT.clone()), 0);
    usedRemovedVars = arrayCreate(metamodelica::arrayLength(rmT.clone()), false);
    usedVars = arrayCreate(metamodelica::arrayLength(mT.clone()), false);
    partitionsCnt = partitionIndependentBlocksMasked(
        m.clone(),
        mT.clone(),
        rm.clone(),
        rmT.clone(),
        arrayCreate(BackendEquation::getNumberOfEquations(eqs.clone()), true),
        eqPartMap.clone(),
        varPartMap.clone(),
        remEqPartMap.clone(),
        usedVars.clone(),
        usedRemovedVars.clone(),
    )?;
    (outBaseClock, baseClockEqIdx) =
        chooseBaseClock(&baseClockEquations, partitionsCnt, eqPartMap.clone(), eqs.clone())?;
    (partAdjacency, order) = getSubPartitionAdjacency(
        partitionsCnt,
        baseClockEqIdx,
        &subClockInterfaceEqIdxs,
        eqPartMap.clone(),
        varPartMap.clone(),
        clockedVarsMask.clone(),
        eqs.clone(),
        &vars,
    )?;
    (m, mT) = BackendDAEUtil::adjacencyMatrixMasked(
        &inEqSystem,
        openmodelica_backend_types::BackendDAE::IndexType::SUBCLOCK_IDX,
        clockedEqsMask.clone(),
        Some(funcs),
        BackendDAEUtil::isInitializationDAE(inShared),
    )?;
    (newClockEqs, newClockVars, contPartitions, subclksCnt) = collectSubclkInfo(
        eqs.clone(),
        inEqSystem.removedEqs.clone(),
        partitionsCnt,
        eqPartMap.clone(),
        remEqPartMap.clone(),
        vars.clone(),
        mT.clone(),
    )?;
    (outBaseClock, subclocks) = findSubClocks(
        partitionsCnt,
        baseClockEqIdx,
        outBaseClock,
        &baseClockEquations,
        &subClockInterfaceEqIdxs,
        eqPartMap.clone(),
        varPartMap.clone(),
        eqs.clone(),
        partAdjacency.clone(),
    )?;
    for mut eqIdx in 1..=metamodelica::arrayLength(clockedEqsMask.clone()) {
        if !(metamodelica::arrayGet(clockedEqsMask.clone(), eqIdx)?) {
            metamodelica::arrayUpdate(eqPartMap.clone(), eqIdx, 0)?;
        }
    }
    for mut varIdx in 1..=metamodelica::arrayLength(clockedVarsMask.clone()) {
        if !(metamodelica::arrayGet(clockedVarsMask.clone(), varIdx)?) {
            metamodelica::arrayUpdate(varPartMap.clone(), varIdx, 0)?;
        }
    }
    (outSysts, outSubClocks) = orderSubPartitions(
        partitionsCnt,
        subclocks.clone(),
        order.clone(),
        eqPartMap.clone(),
        varPartMap.clone(),
        remEqPartMap.clone(),
        eqs,
        &vars,
        remEqs,
        inShared,
        off,
    )?;
    Ok((outSysts, outBaseClock, outSubClocks))
}

fn orderSubPartitions(
    mut numParts: i32,
    mut subclocks: metamodelica::Array<BackendDAE::SubClock>,
    mut order: metamodelica::Array<i32>,
    mut eqPartMap: metamodelica::Array<i32>,
    mut varPartMap: metamodelica::Array<i32>,
    mut remEqPartMap: metamodelica::Array<i32>,
    mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut vars: &BackendDAE::Variables,
    mut remEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut shared: &metamodelica::Ref<BackendDAE::Shared>,
    mut partitionOffset: i32,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    metamodelica::List<BackendDAE::SubClock>,
)> {
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut subClksOut: metamodelica::List<BackendDAE::SubClock> = metamodelica::nil();
    let mut considerRemovedEqs: bool;
    let mut part: i32 = 0;
    let mut mergedParts: metamodelica::List<i32>;
    let mut partVarMap: metamodelica::Array<metamodelica::List<i32>>;
    let mut partEqMap: metamodelica::Array<metamodelica::List<i32>>;
    let mut partRemEqMap: metamodelica::Array<metamodelica::List<i32>>;
    let mut sys: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut clk: BackendDAE::SubClock;
    let mut clk2: BackendDAE::SubClock;
    let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut remEqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut mergedOrder: metamodelica::List<metamodelica::List<i32>>;
    considerRemovedEqs = intGe(metamodelica::arrayLength(remEqPartMap.clone()), 1);
    partVarMap = arrayCreate(numParts, metamodelica::nil());
    for mut varIdx in 1..=metamodelica::arrayLength(varPartMap.clone()) {
        part = metamodelica::arrayGet(varPartMap.clone(), varIdx)?;
        if part > 0 {
            metamodelica::arrayUpdate(
                partVarMap.clone(),
                part,
                listAppend(
                    ({
                        let __elt = (*metamodelica::index_checked(&partVarMap.borrow(), part)?).clone();
                        __elt
                    }),
                    list![varIdx],
                ),
            )?;
        }
    }
    partEqMap = arrayCreate(numParts, metamodelica::nil());
    for mut eqIdx in 1..=metamodelica::arrayLength(eqPartMap.clone()) {
        part = metamodelica::arrayGet(eqPartMap.clone(), eqIdx)?;
        if part > 0 {
            metamodelica::arrayUpdate(
                partEqMap.clone(),
                part,
                listAppend(
                    ({
                        let __elt = (*metamodelica::index_checked(&partEqMap.borrow(), part)?).clone();
                        __elt
                    }),
                    list![eqIdx],
                ),
            )?;
        }
    }
    partRemEqMap = arrayCreate(numParts, metamodelica::nil());
    if considerRemovedEqs {
        for mut reqIdx in 1..=metamodelica::arrayLength(partRemEqMap.clone()) {
            part = metamodelica::arrayGet(remEqPartMap.clone(), reqIdx)?;
            if part > 0 {
                metamodelica::arrayUpdate(
                    partRemEqMap.clone(),
                    part,
                    listAppend(
                        ({
                            let __elt = (*metamodelica::index_checked(&partRemEqMap.borrow(), part)?).clone();
                            __elt
                        }),
                        list![reqIdx],
                    ),
                )?;
            }
        }
    }
    mergedOrder = metamodelica::nil();
    mergedParts = metamodelica::nil();
    clk = metamodelica::arrayGet(
        subclocks.clone(),
        ({
            let __elt = (*metamodelica::index_checked(&order.borrow(), 1)?).clone();
            __elt
        }),
    )?;
    let __range0 = order.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut part in __range0 {
        clk2 = metamodelica::arrayGet(subclocks.clone(), part)?;
        if subClkEqual(&clk, &clk2)? {
            mergedParts = metamodelica::cons(part, mergedParts);
        } else {
            mergedOrder = metamodelica::cons(mergedParts.reverse(), mergedOrder);
            mergedParts = list![part];
            clk = metamodelica::arrayGet(subclocks.clone(), part)?;
        }
    }
    mergedOrder = metamodelica::cons(mergedParts.reverse(), mergedOrder);
    mergedOrder = mergedOrder.reverse();
    part = 1;
    for mut mergedParts in &*mergedOrder {
        let mut mergedParts = mergedParts.clone();
        eqLst = metamodelica::nil();
        varLst = metamodelica::nil();
        remEqLst = metamodelica::nil();
        for mut partIdx in &*mergedParts {
            for mut e in &*metamodelica::arrayGet(partEqMap.clone(), partIdx.clone())? {
                eqLst = metamodelica::cons(BackendEquation::get(eqs.clone(), e.clone())?, eqLst);
            }
            for mut v in &*metamodelica::arrayGet(partVarMap.clone(), partIdx.clone())? {
                varLst = metamodelica::cons(BackendVariable::getVarAt(vars, v.clone())?, varLst);
            }
            for mut r in &*metamodelica::arrayGet(partRemEqMap.clone(), partIdx.clone())? {
                remEqLst = metamodelica::cons(BackendEquation::get(remEqs.clone(), r.clone())?, remEqLst);
            }
            clk = metamodelica::arrayGet(subclocks.clone(), partIdx.clone())?;
        }
        if !((eqLst).is_empty()) || !((remEqLst).is_empty()) {
            (sys, _) = createEqSystem(eqLst.reverse(), varLst.reverse(), &remEqLst, (true, true))?;
            assign_field!(
                sys.partitionKind = BackendDAE::BaseClockPartitionKind::CLOCKED_PARTITION {
                    subPartIdx: partitionOffset + part
                }
            );
            subClksOut = metamodelica::cons(clk.clone(), subClksOut);
            systs = metamodelica::cons(sys, systs);
            part = part + 1;
        }
    }
    systs = systs.reverse();
    subClksOut = subClksOut.reverse();
    Ok((systs, subClksOut))
}

fn isInferedSubClock(mut subClk: &BackendDAE::SubClock) -> bool {
    let mut isInfered: bool;
    isInfered = (match subClk.clone() {
        BackendDAE::SubClock::INFERED_SUBCLOCK { .. } => true,
        _ => false,
    });
    isInfered
}

fn isInferedBaseClock(mut subClk: &metamodelica::Ref<DAE::ClockKind>) -> bool {
    let mut isInfered: bool;
    isInfered = (match &**subClk {
        DAE::ClockKind::INFERRED_CLOCK { .. } => true,
        _ => false,
    });
    isInfered
}

fn setFactor(mut oldVal: MMath::Rational, mut newVal: MMath::Rational) -> Result<MMath::Rational> {
    let mut outVal: MMath::Rational;
    outVal = (match (oldVal, newVal) {
        (MMath::Rational { nom: 1, denom: 1 }, _) => newVal,
        (_, MMath::Rational { nom: 1, denom: 1 }) => oldVal,
        _ => {
            if !(MMath::equals(oldVal, newVal)?) {
                Error::addMessage(
                    Error::SUBCLOCK_CONFLICT.clone(),
                    list![
                        literal!("factor"),
                        MMath::rationalString(oldVal),
                        MMath::rationalString(newVal)
                    ],
                )?;
                return Err("fail");
            }
            newVal
        }
    });
    Ok(outVal)
}

fn setShift(mut oldVal: MMath::Rational, mut newVal: MMath::Rational) -> Result<MMath::Rational> {
    let mut outVal: MMath::Rational;
    outVal = (match (oldVal, newVal) {
        (MMath::Rational { nom: 0, denom: _ }, _) => newVal,
        (_, MMath::Rational { nom: 0, denom: _ }) => oldVal,
        _ => {
            if !(MMath::equals(oldVal, newVal)?) {
                Error::addMessage(
                    Error::SUBCLOCK_CONFLICT.clone(),
                    list![
                        literal!("shift"),
                        MMath::rationalString(oldVal),
                        MMath::rationalString(newVal)
                    ],
                )?;
                return Err("fail");
            }
            newVal
        }
    });
    Ok(outVal)
}

fn collectSubclkInfoExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: &(
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::Array<Option<bool>>,
        SourceInfo,
        metamodelica::Array<i32>,
        i32,
        metamodelica::Array<i32>,
        BackendDAE::Variables,
        metamodelica::Array<metamodelica::List<i32>>,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::Array<Option<bool>>,
        SourceInfo,
        metamodelica::Array<i32>,
        i32,
        metamodelica::Array<i32>,
        BackendDAE::Variables,
        metamodelica::Array<metamodelica::List<i32>>,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::Array<Option<bool>>,
        SourceInfo,
        metamodelica::Array<i32>,
        i32,
        metamodelica::Array<i32>,
        BackendDAE::Variables,
        metamodelica::Array<metamodelica::List<i32>>,
    );
    let mut newEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut newVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut contPartitions: metamodelica::Array<Option<bool>>;
    let mut partitionIdx: i32;
    let mut partitions: metamodelica::Array<i32>;
    let mut vars: BackendDAE::Variables;
    let mut mT: metamodelica::Array<metamodelica::List<i32>>;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut attr: metamodelica::Ref<DAE::CallAttributes>;
    let mut clksCnt: metamodelica::Array<i32>;
    let mut clkCnt: i32;
    let mut source: SourceInfo;
    (
        newEqs,
        newVars,
        contPartitions,
        source,
        clksCnt,
        partitionIdx,
        partitions,
        vars,
        mT,
    ) = inTpl.clone();
    clkCnt = metamodelica::arrayGet(clksCnt.clone(), partitionIdx)?;
    (outExp, newEqs, newVars, clkCnt) = (match &*inExp {
        DAE::Exp::CALL {
            path: __esc_path,
            expLst: __esc_expLst,
            attr: __esc_attr,
        } => {
            path = (*__esc_path).clone();
            expLst = (*__esc_expLst).clone();
            attr = (*__esc_attr).clone();
            collectSubclkInfoCall(
                path.clone(),
                expLst.clone(),
                attr.clone(),
                newEqs,
                newVars,
                contPartitions.clone(),
                partitionIdx,
                clkCnt,
                partitions.clone(),
                &vars,
                mT.clone(),
                &source,
            )?
        }
        _ => (inExp, newEqs, newVars, clkCnt),
    });
    metamodelica::arrayUpdate(clksCnt.clone(), partitionIdx, clkCnt)?;
    outTpl = (
        newEqs,
        newVars,
        contPartitions.clone(),
        source,
        clksCnt.clone(),
        partitionIdx,
        partitions.clone(),
        vars,
        mT.clone(),
    );
    Ok((outExp, outTpl))
}

fn createSubClockVar(
    mut inPartitionIdx: i32,
    mut inClkCnt: i32,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inAttr: metamodelica::Ref<DAE::CallAttributes>,
    mut inPartitions: metamodelica::Array<i32>,
    mut inVars: &BackendDAE::Variables,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    metamodelica::Ref<BackendDAE::Equation>,
)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outEq: metamodelica::Ref<BackendDAE::Equation>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut varIxs: metamodelica::List<i32>;
    let mut i: i32;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut subclk: metamodelica::Ref<DAE::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &((inExpLst).head().cloned()?) {
        Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cr = metamodelica::Own::own(__pa0);
    (_, varIxs) = BackendVariable::getVar(cr, inVars)?;
    i = (varIxs).head().cloned()?;
    i = (metamodelica::arrayGet(mT.clone(), i)?).head().cloned()?;
    i = metamodelica::arrayGet(inPartitions.clone(), i)?;
    subclk = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: getSubClkName(i, 1, DAE::T_CLOCK_DEFAULT().clone()),
        ty: DAE::T_CLOCK_DEFAULT().clone(),
    });
    e = metamodelica::Ref::new(DAE::Exp::CALL {
        path: inPath,
        expLst: metamodelica::cons(subclk, (inExpLst).rest()?),
        attr: inAttr,
    });
    (outVar, outEq) = createSubClock(inPartitionIdx, inClkCnt, e)?;
    Ok((outVar, outEq))
}

fn setContClockedPartition(
    mut inIsContClockedPartition: bool,
    mut inPartitionIdx: i32,
    mut inContPartitions: metamodelica::Array<Option<bool>>,
    mut source: &SourceInfo,
) -> Result<()> {
    let mut isContClockedPartition: Option<bool>;
    let mut isContClockedPrevPartition: bool;
    isContClockedPartition = metamodelica::arrayGet(inContPartitions.clone(), inPartitionIdx)?;
    isContClockedPartition = (match isContClockedPartition {
        None => Some(inIsContClockedPartition),
        Some(mut __esc_isContClockedPrevPartition) => {
            isContClockedPrevPartition = __esc_isContClockedPrevPartition.clone();
            Some(inIsContClockedPartition || isContClockedPrevPartition)
        }
    });
    metamodelica::arrayUpdate(inContPartitions.clone(), inPartitionIdx, isContClockedPartition)?;
    Ok(())
}

fn collectSubclkInfoCall(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inAttr: metamodelica::Ref<DAE::CallAttributes>,
    mut inNewEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inContPartitions: metamodelica::Array<Option<bool>>,
    mut inPartitionIdx: i32,
    mut inClkCnt: i32,
    mut inPartitions: metamodelica::Array<i32>,
    mut inVars: &BackendDAE::Variables,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut source: &SourceInfo,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    i32,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outNewEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut outClkCnt: i32;
    (outExp, outNewEqs, outNewVars, outClkCnt) = (::match_deref::match_deref! { match &((inPath.clone(), ((inExpLst).len() as i32))) {
        (Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, _) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (metamodelica::Ref::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, _) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (metamodelica::Ref::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "spatialDistribution" }, _) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (metamodelica::Ref::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, _) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (metamodelica::Ref::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "terminal" }, _) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (metamodelica::Ref::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, _) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (metamodelica::Ref::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, 3) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (metamodelica::Ref::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, _) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (metamodelica::Ref::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, _) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (metamodelica::Ref::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, _) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (metamodelica::Ref::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "reinit" }, _) => {
            setContClockedPartition(true, inPartitionIdx, inContPartitions.clone(), source)?;
            (metamodelica::Ref::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, _) => {
            setContClockedPartition(false, inPartitionIdx, inContPartitions.clone(), source)?;
            (metamodelica::Ref::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "firstTick" }, _) => {
            setContClockedPartition(false, inPartitionIdx, inContPartitions.clone(), source)?;
            (metamodelica::Ref::new(DAE::Exp::CALL { path: inPath, expLst: metamodelica::nil(), attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "interval" }, _) => {
            setContClockedPartition(false, inPartitionIdx, inContPartitions.clone(), source)?;
            (metamodelica::Ref::new(DAE::Exp::CALL { path: inPath, expLst: metamodelica::nil(), attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, 2) => {
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            let mut eq: metamodelica::Ref<BackendDAE::Equation>;
            (var, eq) = createSubClock(inPartitionIdx, inClkCnt, (inExpLst).get(2)?)?;
            (substGetPartition((inExpLst).get(1)?)?, metamodelica::cons(eq, inNewEqs), metamodelica::cons(var, inNewVars), inClkCnt + 1)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "subSample" }, 2) => {
            (substGetPartition((inExpLst).get(1)?)?, inNewEqs, inNewVars, inClkCnt + 1)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "superSample" }, 2) => {
            (substGetPartition((inExpLst).get(1)?)?, inNewEqs, inNewVars, inClkCnt + 1)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "shiftSample" }, 3) => {
            (substGetPartition((inExpLst).get(1)?)?, inNewEqs, inNewVars, inClkCnt + 1)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "backSample" }, 3) => {
            (substGetPartition((inExpLst).get(1)?)?, inNewEqs, inNewVars, inClkCnt + 1)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "noClock" }, 1) => {
            (substGetPartition((inExpLst).get(1)?)?, inNewEqs, inNewVars, inClkCnt)
        },
        _ => {
            (metamodelica::Ref::new(DAE::Exp::CALL { path: inPath, expLst: inExpLst, attr: inAttr }), inNewEqs, inNewVars, inClkCnt)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outNewEqs, outNewVars, outClkCnt))
}

fn createSubClockVarFactor(
    mut inPartitionIdx: i32,
    mut inClkCnt: i32,
    mut inPath: &metamodelica::Ref<Absyn::Path>,
    mut inExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inAttr: &metamodelica::Ref<DAE::CallAttributes>,
    mut inPartitions: metamodelica::Array<i32>,
    mut inVars: &BackendDAE::Variables,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut inNewEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    i32,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outNewEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = inNewEqs;
    let mut outNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = inNewVars;
    let mut outClkCnt: i32 = inClkCnt;
    outExp = substGetPartition((inExpLst).head().cloned()?)?;
    Ok((outExp, outNewEqs, outNewVars, outClkCnt))
}

fn substGetPartition(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut attrs: metamodelica::Ref<DAE::CallAttributes>;
    attrs = metamodelica::Ref::new(DAE::CallAttributes {
        ty: Expression::r#typeof(inExp.clone())?,
        tuple_: false,
        builtin: true,
        isImpure: true,
        isFunctionPointerCall: false,
        inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE,
        tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL,
        noReturn: DAE::NoReturn::RETURNS.clone(),
    });
    outExp = metamodelica::Ref::new(DAE::Exp::CALL {
        path: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("$getPart"),
        }),
        expLst: list![inExp],
        attr: attrs,
    });
    Ok(outExp)
}

fn getSubClkName(
    mut inPartitionIdx: i32,
    mut inClkIdx: i32,
    mut inTy: metamodelica::Ref<DAE::Type>,
) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut name: ArcStr;
    name = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("$subclk"));
        __mm_s.push_str(&*intString(inPartitionIdx));
        __mm_s.push_str(&*literal!("_"));
        __mm_s.push_str(&*intString(inClkIdx));
        ArcStr::from(__mm_s)
    };
    outRef = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
        ident: name,
        identType: inTy,
        subscriptLst: metamodelica::nil(),
    });
    outRef
}

fn createSubClock(
    mut inPartitionIdx: i32,
    mut inCnt: i32,
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    metamodelica::Ref<BackendDAE::Equation>,
)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outEq: metamodelica::Ref<BackendDAE::Equation>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    ty = DAE::T_CLOCK_DEFAULT().clone();
    cr = getSubClkName(inPartitionIdx, inCnt, ty.clone());
    (outVar, outEq) = createEqVarPair(cr, ty, inExp)?;
    Ok((outVar, outEq))
}

fn collectSubclkInfo(
    mut inEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inRemovedEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inPartitionCnt: i32,
    mut inPartitions: metamodelica::Array<i32>,
    mut inReqsPartitions: metamodelica::Array<i32>,
    mut inVars: BackendDAE::Variables,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::Array<Option<bool>>,
    metamodelica::Array<i32>,
)> {
    let mut outNewEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut outContPartitions: metamodelica::Array<Option<bool>>;
    let mut oClksCnt: metamodelica::Array<i32>;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut cnt: i32;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    let mut partitionsWhenClocks: metamodelica::Array<metamodelica::List<i32>>;
    outContPartitions = arrayCreate(inPartitionCnt, None);
    partitionsWhenClocks = arrayCreate(inPartitionCnt, metamodelica::nil());
    oClksCnt = arrayCreate(inPartitionCnt, 1);
    (outNewEqs, outNewVars) = collectEquationArrayClocks(
        inEqs,
        inPartitionCnt,
        inPartitions.clone(),
        partitionsWhenClocks.clone(),
        oClksCnt.clone(),
        outContPartitions.clone(),
        inVars.clone(),
        mT.clone(),
        metamodelica::nil(),
        metamodelica::nil(),
    )?;
    (outNewEqs, outNewVars) = collectEquationArrayClocks(
        inRemovedEqs,
        inPartitionCnt,
        inReqsPartitions.clone(),
        partitionsWhenClocks.clone(),
        oClksCnt.clone(),
        outContPartitions.clone(),
        inVars,
        mT.clone(),
        outNewEqs,
        outNewVars,
    )?;
    for mut i in 1..=inPartitionCnt {
        for mut j in &*metamodelica::arrayGet(partitionsWhenClocks.clone(), i)? {
            let mut j = j.clone();
            cnt = metamodelica::arrayGet(oClksCnt.clone(), i)?;
            cr = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*arcstr::literal!(BackendDAE::WHENCLK_PRREFIX));
                    __mm_s.push_str(&*intString(j));
                    ArcStr::from(__mm_s)
                },
                identType: DAE::T_CLOCK_DEFAULT().clone(),
                subscriptLst: metamodelica::nil(),
            });
            (var, eq) = createSubClock(
                i,
                cnt,
                metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: cr,
                    ty: DAE::T_CLOCK_DEFAULT().clone(),
                }),
            )?;
            outNewEqs = metamodelica::cons(eq, outNewEqs);
            outNewVars = metamodelica::cons(var, outNewVars);
            metamodelica::arrayUpdate(oClksCnt.clone(), i, cnt + 1)?;
        }
        if metamodelica::arrayGet(oClksCnt.clone(), i)? == 1 {
            (var, eq) = createSubClock(
                i,
                1,
                metamodelica::Ref::new(DAE::Exp::CLKCONST {
                    clk: openmodelica_frontend_types::DAE::ClockKind::interned_INFERRED_CLOCK(),
                }),
            )?;
            outNewEqs = metamodelica::cons(eq, outNewEqs);
            outNewVars = metamodelica::cons(var, outNewVars);
            metamodelica::arrayUpdate(oClksCnt.clone(), i, 2)?;
        }
    }
    Ok((outNewEqs, outNewVars, outContPartitions, oClksCnt))
}

fn collectEquationArrayClocks(
    mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut partitionsCnt: i32,
    mut partitions: metamodelica::Array<i32>,
    mut partitionsWhenClocks: metamodelica::Array<metamodelica::List<i32>>,
    mut clksCnt: metamodelica::Array<i32>,
    mut contPartitions: metamodelica::Array<Option<bool>>,
    mut inVars: BackendDAE::Variables,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut inNewEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
)> {
    let mut outNewEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = inNewEqs;
    let mut outNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = inNewVars;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    let mut eqAttr: BackendDAE::EquationAttributes;
    let mut partitionIdx: i32;
    let mut source: SourceInfo;
    for mut i in 1..=BackendEquation::getNumberOfEquations(eqs.clone()) {
        eq = BackendEquation::get(eqs.clone(), i)?;
        partitionIdx = metamodelica::arrayGet(partitions.clone(), i)?;
        let __arc1 = BackendEquation::equationSource(&eq)?;
        let DAE::SOURCE { info: __pa0, .. } = &*__arc1;
        source = metamodelica::Own::own(__pa0);
        if partitionIdx != 0 {
            eqAttr = BackendEquation::getEquationAttributes(&eq)?;
            eqAttr = (match eqAttr {
                BackendDAE::EquationAttributes {
                    kind: BackendDAE::EquationKind::CLOCKED_EQUATION { clk: mut whenIdx },
                    ..
                } => {
                    let mut partitionsWhenClocksLst: metamodelica::List<i32>;
                    partitionsWhenClocksLst = ({
                        let __elt =
                            (*metamodelica::index_checked(&partitionsWhenClocks.borrow(), partitionIdx)?).clone();
                        __elt
                    });
                    if whenIdx.clone() != 0 && List::notMember(whenIdx.clone(), partitionsWhenClocksLst.clone()) {
                        metamodelica::arrayUpdate(
                            partitionsWhenClocks.clone(),
                            partitionIdx,
                            metamodelica::cons(whenIdx.clone(), partitionsWhenClocksLst),
                        )?;
                    }
                    eqAttr.kind = openmodelica_backend_types::BackendDAE::EquationKind::DYNAMIC_EQUATION;
                    eqAttr
                }
                _ => eqAttr,
            });
            eq = BackendEquation::setEquationAttributes(&eq, eqAttr)?;
            let (__pa2, (__pa3, __pa4, _, _, _, _, _, _, _)) = BackendEquation::traverseExpsOfEquation(
                eq,
                (std::sync::Arc::new(collectSubclkInfoExp1)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                (
                                    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                                    metamodelica::Array<Option<bool>>,
                                    SourceInfo,
                                    metamodelica::Array<i32>,
                                    i32,
                                    metamodelica::Array<i32>,
                                    BackendDAE::Variables,
                                    metamodelica::Array<metamodelica::List<i32>>,
                                ),
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                (
                                    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                                    metamodelica::Array<Option<bool>>,
                                    SourceInfo,
                                    metamodelica::Array<i32>,
                                    i32,
                                    metamodelica::Array<i32>,
                                    BackendDAE::Variables,
                                    metamodelica::Array<metamodelica::List<i32>>,
                                ),
                            )> + 'static,
                    >),
                (
                    outNewEqs,
                    outNewVars,
                    contPartitions.clone(),
                    source,
                    clksCnt.clone(),
                    partitionIdx,
                    partitions.clone(),
                    inVars.clone(),
                    mT.clone(),
                ),
            )?;
            eq = metamodelica::Own::own(__pa2);
            outNewEqs = metamodelica::Own::own(__pa3);
            outNewVars = metamodelica::Own::own(__pa4);
            BackendEquation::setAtIndex(eqs.clone(), i, eq)?;
        }
    }
    Ok((outNewEqs, outNewVars))
}

fn collectSubclkInfoExp1(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::Array<Option<bool>>,
        SourceInfo,
        metamodelica::Array<i32>,
        i32,
        metamodelica::Array<i32>,
        BackendDAE::Variables,
        metamodelica::Array<metamodelica::List<i32>>,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::Array<Option<bool>>,
        SourceInfo,
        metamodelica::Array<i32>,
        i32,
        metamodelica::Array<i32>,
        BackendDAE::Variables,
        metamodelica::Array<metamodelica::List<i32>>,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        metamodelica::Array<Option<bool>>,
        SourceInfo,
        metamodelica::Array<i32>,
        i32,
        metamodelica::Array<i32>,
        BackendDAE::Variables,
        metamodelica::Array<metamodelica::List<i32>>,
    );
    (outExp, outTpl) = Expression::traverseExpBottomUp(
        inExp,
        &move |__a0: metamodelica::Ref<DAE::Exp>,
               __a1: (
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::Array<Option<bool>>,
            SourceInfo,
            metamodelica::Array<i32>,
            i32,
            metamodelica::Array<i32>,
            BackendDAE::Variables,
            metamodelica::Array<metamodelica::List<i32>>,
        )| collectSubclkInfoExp(__a0, &__a1),
        inTpl,
    )?;
    Ok((outExp, outTpl))
}

fn splitClockEqs(
    mut inEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
) -> Result<(
    metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    metamodelica::Array<bool>,
)> {
    let mut outClockEqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut outClockEqsMask: metamodelica::Array<bool>;
    let mut clockEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    let mut i: i32 = 0;
    outClockEqsMask = arrayCreate(BackendEquation::getNumberOfEquations(inEqs.clone()), true);
    for mut i in 1..=BackendEquation::getNumberOfEquations(inEqs.clone()) {
        eq = BackendEquation::get(inEqs.clone(), i)?;
        if isClockEquation(eq.clone())? {
            clockEqs = metamodelica::cons(eq, clockEqs);
            metamodelica::arrayUpdate(outClockEqsMask.clone(), i, false)?;
        }
    }
    outClockEqs = BackendEquation::listEquation(&clockEqs)?;
    Ok((outClockEqs, outClockEqsMask))
}

fn splitClockVars(mut inVars: &BackendDAE::Variables) -> Result<(BackendDAE::Variables, metamodelica::Array<bool>)> {
    let mut outClockVars: BackendDAE::Variables;
    let mut outClockVarsMask: metamodelica::Array<bool>;
    let mut clockVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut var: metamodelica::Ref<BackendDAE::Var>;
    outClockVarsMask = arrayCreate(BackendVariable::varsSize(inVars), true);
    for mut i in 1..=BackendVariable::varsSize(inVars) {
        var = BackendVariable::getVarAt(inVars, i)?;
        if Types::isClockOrSubTypeClock(var.varType.clone()) {
            clockVars = metamodelica::cons(var, clockVars);
            metamodelica::arrayUpdate(outClockVarsMask.clone(), i, false)?;
        }
    }
    outClockVars = BackendVariable::listVar(clockVars)?;
    Ok((outClockVars, outClockVarsMask))
}

fn substitutePartitionOpExps(
    mut inSyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<metamodelica::Ref<BackendDAE::EqSystem>> {
    let mut outSyst: metamodelica::Ref<BackendDAE::EqSystem> = inSyst.clone();
    let mut newEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut newVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut cnt: i32 = 1;
    for mut eq in &*BackendEquation::equationList(inSyst.orderedEqs.clone())? {
        let mut eq = eq.clone();
        let (__pa0, (__pa1, __pa2, __pa3, _)) = BackendEquation::traverseExpsOfEquation(
            eq,
            (std::sync::Arc::new(substitutePartitionOpExp)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                                i32,
                                metamodelica::Ref<BackendDAE::Shared>,
                            ),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (
                                metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
                                metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
                                i32,
                                metamodelica::Ref<BackendDAE::Shared>,
                            ),
                        )> + 'static,
                >),
            (newEqs, newVars, cnt, inShared.clone()),
        )?;
        eq = metamodelica::Own::own(__pa0);
        newEqs = metamodelica::Own::own(__pa1);
        newVars = metamodelica::Own::own(__pa2);
        cnt = metamodelica::Own::own(__pa3);
        newEqs = metamodelica::cons(eq, newEqs);
    }
    assign_field!(
        outSyst.orderedEqs = BackendEquation::listEquation(&(newEqs.reverse()))?,
        outSyst.orderedVars = BackendVariable::addVars(&newVars, inSyst.orderedVars.clone())?
    );
    outSyst = BackendDAEUtil::clearEqSyst(&outSyst);
    Ok(outSyst)
}

fn substitutePartitionOpExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        i32,
        metamodelica::Ref<BackendDAE::Shared>,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        i32,
        metamodelica::Ref<BackendDAE::Shared>,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        i32,
        metamodelica::Ref<BackendDAE::Shared>,
    );
    (outExp, outTpl) = Expression::traverseExpBottomUp(inExp, &substitutePartitionOpExp1, inTpl)?;
    Ok((outExp, outTpl))
}

fn substitutePartitionOpExp1(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        i32,
        metamodelica::Ref<BackendDAE::Shared>,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        i32,
        metamodelica::Ref<BackendDAE::Shared>,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        i32,
        metamodelica::Ref<BackendDAE::Shared>,
    );
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut shared: metamodelica::Ref<BackendDAE::Shared>;
    let mut attr: metamodelica::Ref<DAE::CallAttributes>;
    let mut clk: metamodelica::Ref<DAE::ClockKind>;
    let mut cnt: i32;
    let mut newEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut newVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    (newEqs, newVars, cnt, shared) = inTpl.clone();
    (outExp, outTpl) = (match &*inExp {
        DAE::Exp::CLKCONST { clk: __esc_clk } => {
            clk = (*__esc_clk).clone();
            (clk, newEqs, newVars, cnt) = substClock(clk.clone(), newEqs, newVars, cnt, &shared)?;
            (
                metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: clk.clone() }),
                (newEqs, newVars, cnt, shared),
            )
        }
        DAE::Exp::CALL {
            path: __esc_path,
            expLst: __esc_exps,
            attr: __esc_attr,
        } => {
            path = (*__esc_path).clone();
            exps = (*__esc_exps).clone();
            attr = (*__esc_attr).clone();
            substituteExpsCall(path.clone(), exps.clone(), attr.clone(), newEqs, newVars, cnt, shared)?
        }
        _ => (inExp, inTpl),
    });
    Ok((outExp, outTpl))
}

fn substClock(
    mut inClk: metamodelica::Ref<DAE::ClockKind>,
    mut inNewEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inCnt: i32,
    mut inShared: &metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<DAE::ClockKind>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    i32,
)> {
    let mut outClk: metamodelica::Ref<DAE::ClockKind>;
    let mut outNewEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut outCnt: i32;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut i: metamodelica::Ref<DAE::Exp>;
    let mut f: metamodelica::Ref<DAE::Exp>;
    let mut cnt: i32;
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    (outClk, outNewEqs, outNewVars, outCnt) = (match &*inClk {
        DAE::ClockKind::EVENT_CLOCK {
            condition: __esc_e,
            startInterval: __esc_f,
        } => {
            e = (*__esc_e).clone();
            f = (*__esc_f).clone();
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(substExp(list![e.clone()], inNewEqs, inNewVars, inCnt)?) {
                (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, __pa1, __pa2, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e = metamodelica::Own::own(__pa0);
            eqs = metamodelica::Own::own(__pa1);
            vars = metamodelica::Own::own(__pa2);
            cnt = metamodelica::Own::own(__pa3);
            (
                metamodelica::Ref::new(DAE::ClockKind::EVENT_CLOCK {
                    condition: e.clone(),
                    startInterval: f.clone(),
                }),
                eqs,
                vars,
                cnt,
            )
        }
        DAE::ClockKind::REAL_CLOCK { interval: __esc_e } => {
            e = (*__esc_e).clone();
            (e, eqs, vars, cnt) = substClockExp(e.clone(), inNewEqs, inNewVars, inCnt, inShared)?;
            (
                metamodelica::Ref::new(DAE::ClockKind::REAL_CLOCK { interval: e.clone() }),
                eqs,
                vars,
                cnt,
            )
        }
        DAE::ClockKind::RATIONAL_CLOCK {
            intervalCounter: __esc_e,
            resolution: __esc_i,
        } => {
            e = (*__esc_e).clone();
            i = (*__esc_i).clone();
            (e, eqs, vars, cnt) = substClockExp(e.clone(), inNewEqs, inNewVars, inCnt, inShared)?;
            (
                metamodelica::Ref::new(DAE::ClockKind::RATIONAL_CLOCK {
                    intervalCounter: e.clone(),
                    resolution: i.clone(),
                }),
                eqs,
                vars,
                cnt,
            )
        }
        _ => (inClk, inNewEqs, inNewVars, inCnt),
    });
    Ok((outClk, outNewEqs, outNewVars, outCnt))
}

fn isKnownOrConstantExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inKnownVars: BackendDAE::Variables,
) -> Result<bool> {
    let mut outKnown: bool;
    let (_, (__pa0, _)) = Expression::traverseExpTopDown(
        inExp,
        &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: (bool, BackendDAE::Variables)| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(isKnownOrConstantExp_traverser(__a0, &__a1))
        },
        (true, inKnownVars),
    )?;
    outKnown = metamodelica::Own::own(__pa0);
    Ok(outKnown)
}

fn isKnownOrConstantExp_traverser(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: &(bool, BackendDAE::Variables),
) -> (metamodelica::Ref<DAE::Exp>, bool, (bool, BackendDAE::Variables)) {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut outContinue: bool;
    let mut outTpl: (bool, BackendDAE::Variables);
    let mut globalKnownVars: BackendDAE::Variables;
    let mut isKnown: bool;
    (isKnown, globalKnownVars) = inTpl.clone();
    isKnown = (match &*inExp {
        DAE::Exp::CALL { .. } => false,
        DAE::Exp::CREF { componentRef, .. } => BackendVariable::containsCref(componentRef.clone(), &globalKnownVars),
        _ => isKnown,
    });
    outTpl = (isKnown, globalKnownVars);
    outContinue = isKnown;
    (outExp, outContinue, outTpl)
}

fn substClockExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inNewEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inCnt: i32,
    mut inShared: &metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    i32,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outNewEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outNewVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut outCnt: i32;
    if isKnownOrConstantExp(inExp.clone(), inShared.globalKnownVars.clone())? {
        outExp = inExp;
        outNewEqs = inNewEqs;
        outNewVars = inNewVars;
        outCnt = inCnt;
    } else {
        let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(substExp(list![inExp], inNewEqs, inNewVars, inCnt)?) {
            (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, __pa1, __pa2, __pa3) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        outExp = metamodelica::Own::own(__pa0);
        outNewEqs = metamodelica::Own::own(__pa1);
        outNewVars = metamodelica::Own::own(__pa2);
        outCnt = metamodelica::Own::own(__pa3);
    }
    Ok((outExp, outNewEqs, outNewVars, outCnt))
}

fn substituteExpsCall(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inAttr: metamodelica::Ref<DAE::CallAttributes>,
    mut inEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inCnt: i32,
    mut inShared: metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        i32,
        metamodelica::Ref<BackendDAE::Shared>,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        i32,
        metamodelica::Ref<BackendDAE::Shared>,
    );
    let mut replace: bool;
    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut eqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut cnt: i32;
    replace = (::match_deref::match_deref! { match &((&*inPath, ((inExps).len() as i32))) {
        (Deref @ Absyn::Path::IDENT { name: Deref @ "hold" }, 1) => true,
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, 2) => true,
        (Deref @ Absyn::Path::IDENT { name: Deref @ "subSample" }, 2) => true,
        (Deref @ Absyn::Path::IDENT { name: Deref @ "superSample" }, 2) => true,
        (Deref @ Absyn::Path::IDENT { name: Deref @ "shiftSample" }, 3) => true,
        (Deref @ Absyn::Path::IDENT { name: Deref @ "backSample" }, 3) => true,
        (Deref @ Absyn::Path::IDENT { name: Deref @ "noClock" }, 1) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exps, eqs, vars, cnt) = if (replace) {
        substExp(inExps, inEqs, inVars, inCnt)?
    } else {
        (inExps, inEqs, inVars, inCnt)
    };
    outExp = metamodelica::Ref::new(DAE::Exp::CALL {
        path: inPath,
        expLst: exps,
        attr: inAttr,
    });
    outTpl = (eqs, vars, cnt, inShared);
    Ok((outExp, outTpl))
}

fn createVar(
    mut inComp: metamodelica::Ref<DAE::ComponentRef>,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    outVar = metamodelica::Ref::new(BackendDAE::Var {
        varName: inComp,
        varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE,
        varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
        varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
        varType: inType.clone(),
        bindExp: None,
        tplExp: None,
        arryDim: metamodelica::nil(),
        source: DAE::emptyElementSource().clone(),
        values: DAEUtil::setProtectedAttr(DAEUtil::getEmptyVarAttr(&inType), true)?,
        tearingSelectOption: Some(openmodelica_backend_types::BackendDAE::TearingSelect::DEFAULT),
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

fn createEqVarPair(
    mut inComp: metamodelica::Ref<DAE::ComponentRef>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    metamodelica::Ref<BackendDAE::Equation>,
)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut outEq: metamodelica::Ref<BackendDAE::Equation>;
    outVar = createVar(inComp.clone(), inType.clone())?;
    outEq = metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
        exp: metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: inComp,
            ty: inType,
        }),
        scalar: inExp,
        source: DAE::emptyElementSource().clone(),
        attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(),
    });
    Ok((outVar, outEq))
}

fn substExp(
    mut inExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inCnt: i32,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    i32,
)> {
    let mut outTpl: (
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        i32,
    );
    let mut create: bool;
    let mut e: metamodelica::Ref<DAE::Exp>;
    e = (inExps).head().cloned()?;
    create = (match &*e {
        DAE::Exp::CREF { .. } => false,
        DAE::Exp::RCONST { .. } => false,
        DAE::Exp::SCONST { .. } => false,
        DAE::Exp::BCONST { .. } => false,
        DAE::Exp::ENUM_LITERAL { .. } => false,
        DAE::Exp::CLKCONST { .. } => true,
        _ => true,
    });
    outTpl = (match create {
        true => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut eq: metamodelica::Ref<BackendDAE::Equation>;
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            ty = Expression::r#typeof(e.clone())?;
            cr = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("$var"));
                    __mm_s.push_str(&*intString(inCnt));
                    ArcStr::from(__mm_s)
                },
                identType: ty.clone(),
                subscriptLst: metamodelica::nil(),
            });
            (var, eq) = createEqVarPair(cr.clone(), ty.clone(), e)?;
            (
                metamodelica::cons(
                    metamodelica::Ref::new(DAE::Exp::CREF {
                        componentRef: cr,
                        ty: ty,
                    }),
                    (inExps).rest()?,
                ),
                metamodelica::cons(eq, inEqs),
                metamodelica::cons(var, inVars),
                inCnt + 1,
            )
        }
        false => (inExps, inEqs, inVars, inCnt),
    });
    Ok(outTpl)
}

fn getVarIxs(
    mut inComp: metamodelica::Ref<DAE::ComponentRef>,
    mut inVariables: &BackendDAE::Variables,
) -> metamodelica::List<i32> {
    let mut outIntegerLst: metamodelica::List<i32>;
    outIntegerLst = 'mc: {
        let __mc_input = &*inComp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ixs: metamodelica::List<i32>;
                    (_, ixs) = BackendVariable::getVar(inComp.clone(), inVariables)?;
                    Ok(ixs.clone())
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
    outIntegerLst
}

fn baseClockPartitioning(
    mut inSyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inShared: &metamodelica::Ref<BackendDAE::Shared>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut outContSysts: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut outClockedSysts: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut outUnpartRemEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut vars: BackendDAE::Variables;
    let mut eqs: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut m: metamodelica::Array<metamodelica::List<i32>>;
    let mut mT: metamodelica::Array<metamodelica::List<i32>>;
    let mut rm: metamodelica::Array<metamodelica::List<i32>>;
    let mut rmT: metamodelica::Array<metamodelica::List<i32>>;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>;
    let mut partitionCnt: i32;
    let mut i: i32 = 0;
    let mut j: i32;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut varIxs: metamodelica::List<i32>;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut eqPartMap: metamodelica::Array<i32>;
    let mut varPartMap: metamodelica::Array<i32>;
    let mut reqsPartition: metamodelica::Array<i32>;
    let mut varsPartition: metamodelica::Array<bool>;
    let mut rvarsPartition: metamodelica::Array<bool>;
    let mut eq: metamodelica::Ref<BackendDAE::Equation> = metamodelica::Ref::new(BackendDAE::Equation::DUMMY_EQUATION);
    let mut refsInfo: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, bool)>;
    let mut refInfo: (metamodelica::Ref<DAE::ComponentRef>, bool) =
        (metamodelica::Ref::new(DAE::ComponentRef::WILD), false);
    let mut partitionType: Option<bool>;
    let mut isClocked: bool;
    let mut isInitial: bool;
    let mut clockedEqs: metamodelica::Array<Option<bool>>;
    let mut clockedVars: metamodelica::Array<Option<bool>>;
    let mut clockedPartitions: metamodelica::Array<Option<bool>>;
    let mut info: SourceInfo;
    funcs = BackendDAEUtil::getFunctions(inShared);
    isInitial = BackendDAEUtil::isInitializationDAE(inShared);
    (syst, m, mT) = BackendDAEUtil::getAdjacencyMatrixfromOption(
        inSyst.clone(),
        openmodelica_backend_types::BackendDAE::IndexType::BASECLOCK_IDX,
        Some(funcs.clone()),
        isInitial,
    )?;
    (rm, rmT) = BackendDAEUtil::removedAdjacencyMatrix(
        &inSyst,
        openmodelica_backend_types::BackendDAE::IndexType::BASECLOCK_IDX,
        Some(funcs.clone()),
        isInitial,
    )?;
    let __arc2 = syst.clone();
    let BackendDAE::EQSYSTEM {
        orderedVars: __pa0,
        orderedEqs: __pa1,
        ..
    } = &*__arc2;
    vars = metamodelica::Own::own(__pa0);
    eqs = metamodelica::Own::own(__pa1);
    eqPartMap = arrayCreate(metamodelica::arrayLength(m.clone()), 0);
    varPartMap = arrayCreate(metamodelica::arrayLength(mT.clone()), 0);
    reqsPartition = arrayCreate(metamodelica::arrayLength(rm.clone()), 0);
    varsPartition = arrayCreate(metamodelica::arrayLength(mT.clone()), false);
    rvarsPartition = arrayCreate(metamodelica::arrayLength(rmT.clone()), false);
    partitionCnt = partitionIndependentBlocks0(
        m.clone(),
        mT.clone(),
        rm.clone(),
        rmT.clone(),
        eqPartMap.clone(),
        varPartMap.clone(),
        reqsPartition.clone(),
        varsPartition.clone(),
        rvarsPartition.clone(),
    )?;
    if partitionCnt > 1 {
        (systs, outUnpartRemEqs, _) = partitionIndependentBlocksSplitBlocks(
            partitionCnt,
            syst,
            eqPartMap.clone(),
            reqsPartition.clone(),
            mT.clone(),
            rmT.clone(),
            false,
            funcs,
            BackendDAEUtil::isInitializationDAE(inShared),
        )?;
    } else {
        (systs, outUnpartRemEqs) = (list![syst], metamodelica::nil());
    }
    clockedEqs = arrayCreate(BackendEquation::getNumberOfEquations(eqs.clone()), None);
    clockedVars = arrayCreate(BackendVariable::varsSize(&vars), None);
    clockedPartitions = arrayCreate(if (partitionCnt > 0) { partitionCnt } else { 1 }, None);
    j = 0;
    for mut eq in &*BackendEquation::equationList(eqs.clone())? {
        let mut eq = eq.clone();
        j = j + 1;
        (partitionType, refsInfo) = detectEqPartition(eq.clone())?;
        info = BackendEquation::equationInfo(&eq)?;
        metamodelica::arrayUpdate(
            clockedEqs.clone(),
            j,
            setClockedPartition(
                partitionType,
                metamodelica::arrayGet(clockedEqs.clone(), j)?,
                None,
                &info,
            )?,
        )?;
        for mut refInfo in &*refsInfo {
            let mut refInfo = refInfo.clone();
            (cr, isClocked) = refInfo;
            varIxs = getVarIxs(cr.clone(), &vars);
            for mut i in &*varIxs {
                let mut i = i.clone();
                metamodelica::arrayUpdate(
                    clockedVars.clone(),
                    i,
                    setClockedPartition(
                        Some(isClocked),
                        metamodelica::arrayGet(clockedVars.clone(), i)?,
                        Some(cr.clone()),
                        &info,
                    )?,
                )?;
            }
        }
    }
    for mut i in 1..=metamodelica::arrayLength(clockedVars.clone()) {
        partitionType = metamodelica::arrayGet(clockedVars.clone(), i)?;
        cr = BackendVariable::varCref(&(BackendVariable::getVarAt(&vars, i)?));
        for mut j in &*metamodelica::arrayGet(mT.clone(), i)? {
            let mut j = j.clone();
            info = BackendEquation::equationInfo(&(BackendEquation::get(eqs.clone(), j)?))?;
            metamodelica::arrayUpdate(
                clockedEqs.clone(),
                j,
                setClockedPartition(
                    partitionType.clone(),
                    metamodelica::arrayGet(clockedEqs.clone(), j)?,
                    Some(cr.clone()),
                    &info,
                )?,
            )?;
        }
    }
    for mut i in 1..=metamodelica::arrayLength(clockedEqs.clone()) {
        partitionType = metamodelica::arrayGet(clockedEqs.clone(), i)?;
        info = BackendEquation::equationInfo(&(BackendEquation::get(eqs.clone(), i)?))?;
        j = metamodelica::arrayGet(eqPartMap.clone(), i)?;
        metamodelica::arrayUpdate(
            clockedPartitions.clone(),
            j,
            setClockedPartition(
                partitionType,
                metamodelica::arrayGet(clockedPartitions.clone(), j)?,
                None,
                &info,
            )?,
        )?;
    }
    i = 1;
    for mut syst in &*systs {
        let mut syst = syst.clone();
        (outContSysts, outClockedSysts) = (match metamodelica::arrayGet(clockedPartitions.clone(), i)? {
            Some(false) => (
                metamodelica::cons(
                    setSystPartition(
                        syst,
                        openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::CONTINUOUS_TIME_PARTITION,
                    ),
                    outContSysts,
                ),
                outClockedSysts,
            ),
            None => (
                metamodelica::cons(
                    setSystPartition(
                        syst,
                        openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNSPECIFIED_PARTITION,
                    ),
                    outContSysts,
                ),
                outClockedSysts,
            ),
            Some(true) => (outContSysts, metamodelica::cons(syst, outClockedSysts)),
            _ => return Err("match: no arm matched"),
        });
        i = i + 1;
    }
    Ok((outContSysts, outClockedSysts, outUnpartRemEqs))
}

fn isClockExp(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut out: bool;
    out = Types::isClockOrSubTypeClock(Expression::r#typeof(inExp)?);
    Ok(out)
}

fn isClockEquation(mut inEq: metamodelica::Ref<BackendDAE::Equation>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inEq.clone()) {
            Deref @ BackendDAE::Equation::EQUATION { scalar: e, .. } => {
                return Ok(isClockExp(e.clone())?)
            },
            Deref @ BackendDAE::Equation::ARRAY_EQUATION { right: e, .. } => {
                return Ok(isClockExp(e.clone())?)
            },
            Deref @ BackendDAE::Equation::FOR_EQUATION { body: eq, .. } => {
                { inEq = eq.clone(); continue '__tco; }
            },
            Deref @ BackendDAE::Equation::SOLVED_EQUATION { exp: e, .. } => {
                return Ok(isClockExp(e.clone())?)
            },
            Deref @ BackendDAE::Equation::RESIDUAL_EQUATION { exp: e, .. } => {
                return Ok(isClockExp(e.clone())?)
            },
            Deref @ BackendDAE::Equation::ALGORITHM { .. } => {
                return Ok(false)
            },
            Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::ListNode::Cons { head: BackendDAE::WhenOperator::ASSIGN { right: e, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. } => {
                let mut info: SourceInfo;
                if isClockExp(e.clone())? {
                    let __arc1 = BackendEquation::equationSource(&inEq)?;
                    let DAE::SOURCE { info: __pa0, .. } = &*__arc1;
                    info = metamodelica::Own::own(__pa0);
                    Error::addSourceMessageAndFail(&(Error::INVALID_CLOCK_EQUATION.clone()), metamodelica::nil(), &info)?;
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                }
                return Ok(false)
            },
            Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::ListNode::Cons { head: BackendDAE::WhenOperator::REINIT { value: e, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. } => {
                let mut info: SourceInfo;
                if isClockExp(e.clone())? {
                    let __arc1 = BackendEquation::equationSource(&inEq)?;
                    let DAE::SOURCE { info: __pa0, .. } = &*__arc1;
                    info = metamodelica::Own::own(__pa0);
                    Error::addSourceMessageAndFail(&(Error::INVALID_CLOCK_EQUATION.clone()), metamodelica::nil(), &info)?;
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                }
                return Ok(false)
            },
            Deref @ BackendDAE::Equation::COMPLEX_EQUATION { right: e, .. } => {
                return Ok(isClockExp(e.clone())?)
            },
            Deref @ BackendDAE::Equation::IF_EQUATION { eqnstrue: trueEqs, eqnsfalse: falseEqs, .. } => {
                let mut listEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
                let mut eq: metamodelica::Ref<BackendDAE::Equation> = metamodelica::Ref::new(BackendDAE::Equation::DUMMY_EQUATION);
                let mut info: SourceInfo;
                for mut listEqs in &*trueEqs.clone() {
                    let mut listEqs = listEqs.clone();
                    for mut eq in &*listEqs {
                        let mut eq = eq.clone();
                        if isClockEquation(eq.clone())? {
                            let __arc1 = BackendEquation::equationSource(&eq)?;
                            let DAE::SOURCE { info: __pa0, .. } = &*__arc1;
                            info = metamodelica::Own::own(__pa0);
                            Error::addSourceMessageAndFail(&(Error::INVALID_CLOCK_EQUATION.clone()), metamodelica::nil(), &info)?;
                            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                        }
                    }
                }
                for mut eq in &*falseEqs.clone() {
                    let mut eq = eq.clone();
                    if isClockEquation(eq.clone())? {
                        let __arc3 = BackendEquation::equationSource(&eq)?;
                        let DAE::SOURCE { info: __pa2, .. } = &*__arc3;
                        info = metamodelica::Own::own(__pa2);
                        Error::addSourceMessageAndFail(&(Error::INVALID_CLOCK_EQUATION.clone()), metamodelica::nil(), &info)?;
                        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                    }
                }
                return Ok(false)
            },
            _ => {
                Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("SynchronousFeatures.isClockEquation")); __mm_s.push_str(&*literal!(" failed.\n")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("BackEnd/SynchronousFeatures.mo"))?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn detectEqPartition(
    mut inEq: metamodelica::Ref<BackendDAE::Equation>,
) -> Result<(
    Option<bool>,
    metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, bool)>,
)> {
    let mut outPartitionType: Option<bool>;
    let mut refsInfo: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, bool)>;
    let mut partitionType: Option<bool>;
    let mut isClockEq: bool;
    let mut info: SourceInfo;
    partitionType = (match BackendEquation::getEquationAttributes(&inEq)? {
        BackendDAE::EquationAttributes {
            kind: BackendDAE::EquationKind::CLOCKED_EQUATION { .. },
            ..
        } => Some(true),
        _ => None,
    });
    info = BackendEquation::equationInfo(&inEq)?;
    let (_, (__pa0, __pa1, _)) = BackendEquation::traverseExpsOfEquation(
        inEq.clone(),
        (std::sync::Arc::new(detectEqPartitionExp)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::Exp>,
                        (
                            Option<bool>,
                            metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, bool)>,
                            SourceInfo,
                        ),
                    ) -> Result<(
                        metamodelica::Ref<DAE::Exp>,
                        (
                            Option<bool>,
                            metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, bool)>,
                            SourceInfo,
                        ),
                    )> + 'static,
            >),
        (partitionType, metamodelica::nil(), info.clone()),
    )?;
    partitionType = metamodelica::Own::own(__pa0);
    refsInfo = metamodelica::Own::own(__pa1);
    isClockEq = isClockEquation(inEq)?;
    outPartitionType = if (isClockEq) {
        setClockedPartition(Some(true), partitionType, None, &info)?
    } else {
        partitionType
    };
    Ok((outPartitionType, refsInfo))
}

fn printPartitionType(mut isClockedPartition: Option<bool>) -> ArcStr {
    let mut out: ArcStr;
    out = (match isClockedPartition {
        Some(false) => literal!("CONT_PARTITION"),
        Some(true) => literal!("CLOCKED_PARTITION"),
        _ => literal!("UNSPECIFIED_PARTITION"),
    });
    out
}

fn detectEqPartitionExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (
        Option<bool>,
        metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, bool)>,
        SourceInfo,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        Option<bool>,
        metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, bool)>,
        SourceInfo,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (
        Option<bool>,
        metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, bool)>,
        SourceInfo,
    );
    (outExp, outTpl) = Expression::traverseExpTopDown(
        inExp,
        &move |__a0: metamodelica::Ref<DAE::Exp>,
               __a1: (
            Option<bool>,
            metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, bool)>,
            SourceInfo,
        )| detectEqPartitionExp1(__a0, &__a1),
        inTpl,
    )?;
    Ok((outExp, outTpl))
}

fn detectEqPartitionExp1(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: &(
        Option<bool>,
        metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, bool)>,
        SourceInfo,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    (
        Option<bool>,
        metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, bool)>,
        SourceInfo,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut cont: bool;
    let mut outTpl: (
        Option<bool>,
        metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, bool)>,
        SourceInfo,
    );
    let mut refs: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, bool)>;
    let mut partition: Option<bool>;
    let mut info: SourceInfo;
    (partition, refs, info) = inTpl.clone();
    (partition, refs, cont) = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::EVENT_CLOCK { condition: e, startInterval: _ } } => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let __pa0 = ::match_deref::match_deref! { match &(e.clone()) {
                Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa0);
            (partition, metamodelica::cons((cr, false), refs), false)
        },
        Deref @ DAE::Exp::CALL { path, expLst: exps, .. } => {
            detectEqPartitionCall(metamodelica::AsArg::as_arg(&path), metamodelica::AsArg::as_arg(&exps), refs, partition, &info)?
        },
        _ => {
            (partition, refs, true)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTpl = (partition, refs, info);
    Ok((outExp, cont, outTpl))
}

fn detectEqPartitionCall(
    mut inPath: &metamodelica::Ref<Absyn::Path>,
    mut inExps: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inRefs: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, bool)>,
    mut inPartition: Option<bool>,
    mut info: &SourceInfo,
) -> Result<(
    Option<bool>,
    metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, bool)>,
    bool,
)> {
    let mut outPartition: Option<bool>;
    let mut outRefs: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, bool)>;
    let mut cont: bool;
    (outPartition, outRefs, cont) = (::match_deref::match_deref! { match (inPath, inExps) {
        (Deref @ Absyn::Path::IDENT { name: Deref @ "hold" }, Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }) => {
            detectEqPartitionCall1(false, true, inPartition, metamodelica::AsArg::as_arg(&e), inRefs, info)?
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            detectEqPartitionCall1(true, false, inPartition, metamodelica::AsArg::as_arg(&e), inRefs, info)?
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "subSample" }, Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            detectEqPartitionCall1(true, true, inPartition, metamodelica::AsArg::as_arg(&e), inRefs, info)?
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "superSample" }, Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            detectEqPartitionCall1(true, true, inPartition, metamodelica::AsArg::as_arg(&e), inRefs, info)?
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "shiftSample" }, Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
            detectEqPartitionCall1(true, true, inPartition, metamodelica::AsArg::as_arg(&e), inRefs, info)?
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "backSample" }, Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
            detectEqPartitionCall1(true, true, inPartition, metamodelica::AsArg::as_arg(&e), inRefs, info)?
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "noClock" }, Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }) => {
            detectEqPartitionCall1(true, true, inPartition, metamodelica::AsArg::as_arg(&e), inRefs, info)?
        },
        _ => {
            (inPartition, inRefs, true)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outPartition, outRefs, cont))
}

fn detectEqPartitionCall1(
    mut expClocked: bool,
    mut refClocked: bool,
    mut inPartition: Option<bool>,
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inRefs: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, bool)>,
    mut info: &SourceInfo,
) -> Result<(
    Option<bool>,
    metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, bool)>,
    bool,
)> {
    let mut outPartition: Option<bool>;
    let mut outRefs: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, bool)>;
    let mut cont: bool = false;
    (outPartition, outRefs) = (match &**inExp {
        DAE::Exp::CREF {
            componentRef: cr,
            ty: _,
        } => (
            setClockedPartition(Some(expClocked), inPartition, None, info)?,
            metamodelica::cons((cr.clone(), refClocked), inRefs),
        ),
        _ => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("SynchronousFeatures.detectEqPartitionCall1"));
                    __mm_s.push_str(&*literal!(" failed.\n"));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("BackEnd/SynchronousFeatures.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok((outPartition, outRefs, cont))
}

fn setSystPartition(
    mut inSyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut inPartitionKind: BackendDAE::BaseClockPartitionKind,
) -> metamodelica::Ref<BackendDAE::EqSystem> {
    let mut outSyst: metamodelica::Ref<BackendDAE::EqSystem>;
    outSyst = (::match_deref::match_deref! { match &(inSyst) {
        syst @ Deref @ BackendDAE::EqSystem { .. } => {
            let mut syst = (*syst).clone();
            assign_field!(syst.partitionKind = inPartitionKind);
            syst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outSyst
}

fn getPartitionConflictError(
    mut inComp: Option<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(ErrorTypes::Message, metamodelica::List<ArcStr>)> {
    let mut msg: ErrorTypes::Message;
    let mut tokens: metamodelica::List<ArcStr>;
    (msg, tokens) = (::match_deref::match_deref! { match &(inComp) {
        Some(cr) => {
            (Error::CONT_CLOCKED_PARTITION_CONFLICT_VAR.clone(), list![ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?])
        },
        _ => {
            (Error::CONT_CLOCKED_PARTITION_CONFLICT_EQ.clone(), metamodelica::nil())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((msg, tokens))
}

fn setClockedPartition(
    mut inNewPartitionType: Option<bool>,
    mut inOldPartitionType: Option<bool>,
    mut inComp: Option<metamodelica::Ref<DAE::ComponentRef>>,
    mut info: &SourceInfo,
) -> Result<Option<bool>> {
    let mut outPartitionType: Option<bool>;
    outPartitionType = (match (inOldPartitionType.clone(), inNewPartitionType.clone()) {
        (None, _) => inNewPartitionType,
        (_, None) => inOldPartitionType,
        (Some(mut oldVal), Some(mut newVal)) if (oldVal == newVal) => inNewPartitionType,
        _ => {
            let mut msg: ErrorTypes::Message;
            let mut tokens: metamodelica::List<ArcStr>;
            (msg, tokens) = getPartitionConflictError(inComp)?;
            Error::addSourceMessage(&msg, tokens, info)?;
            return Err("fail");
        }
    });
    Ok(outPartitionType)
}

pub(crate) fn partitionIndependentBlocks0(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rm: metamodelica::Array<metamodelica::List<i32>>,
    mut rmT: metamodelica::Array<metamodelica::List<i32>>,
    mut eqPartMap: metamodelica::Array<i32>,
    mut varPartMap: metamodelica::Array<i32>,
    mut rixs: metamodelica::Array<i32>,
    mut vars: metamodelica::Array<bool>,
    mut rvars: metamodelica::Array<bool>,
) -> Result<i32> {
    let mut on: i32 = 0;
    for mut i in ({
        let __s = metamodelica::arrayLength(m.clone());
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        on = if (partitionIndependentBlocksWork(
            i,
            false,
            on + 1,
            m.clone(),
            mT.clone(),
            rm.clone(),
            rmT.clone(),
            eqPartMap.clone(),
            varPartMap.clone(),
            rixs.clone(),
            vars.clone(),
            rvars.clone(),
        )?) {
            on + 1
        } else {
            on
        };
    }
    for mut i in ({
        let __s = metamodelica::arrayLength(rm.clone());
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        on = if (partitionIndependentBlocksWork(
            i,
            true,
            on + 1,
            m.clone(),
            mT.clone(),
            rm.clone(),
            rmT.clone(),
            eqPartMap.clone(),
            varPartMap.clone(),
            rixs.clone(),
            vars.clone(),
            rvars.clone(),
        )?) {
            on + 1
        } else {
            on
        };
    }
    Ok(on)
}

fn partitionIndependentBlocks(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut eqPartMap: metamodelica::Array<i32>,
    mut varPartMap: metamodelica::Array<i32>,
) -> Result<i32> {
    let mut on: i32 = 0;
    for mut eq in ({
        let __s = metamodelica::arrayLength(m.clone());
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("check eq "));
            __mm_s.push_str(&*intString(eq));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        if !(intEq(metamodelica::arrayGet(eqPartMap.clone(), eq)?, -2)) {
            on = if (partitionIndependentBlocks2(
                eq,
                on + 1,
                m.clone(),
                mT.clone(),
                eqPartMap.clone(),
                varPartMap.clone(),
            )?) {
                on + 1
            } else {
                on
            };
        }
    }
    Ok(on)
}

fn partitionIndependentBlocks2(
    mut eqIdx: i32,
    mut partIdx: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut eqPartMap: metamodelica::Array<i32>,
    mut varPartMap: metamodelica::Array<i32>,
) -> Result<bool> {
    let mut ochange: bool;
    ochange = metamodelica::arrayGet(eqPartMap.clone(), eqIdx)? == -1;
    if ochange {
        metamodelica::arrayUpdate(eqPartMap.clone(), eqIdx, partIdx)?;
        for mut var in &*metamodelica::arrayGet(m.clone(), eqIdx)? {
            if !(intGt(metamodelica::arrayGet(varPartMap.clone(), intAbs(var.clone()))?, 0)) {
                metamodelica::arrayUpdate(varPartMap.clone(), intAbs(var.clone()), partIdx)?;
                for mut newEq in &*metamodelica::arrayGet(mT.clone(), intAbs(var.clone()))? {
                    partitionIndependentBlocks2(
                        intAbs(newEq.clone()),
                        partIdx,
                        m.clone(),
                        mT.clone(),
                        eqPartMap.clone(),
                        varPartMap.clone(),
                    )?;
                }
            }
        }
    }
    Ok(ochange)
}

fn partitionIndependentBlocksMasked(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rm: metamodelica::Array<metamodelica::List<i32>>,
    mut rmT: metamodelica::Array<metamodelica::List<i32>>,
    mut mask: metamodelica::Array<bool>,
    mut eqPartMap: metamodelica::Array<i32>,
    mut varPartMap: metamodelica::Array<i32>,
    mut remEqPartMap: metamodelica::Array<i32>,
    mut vars: metamodelica::Array<bool>,
    mut rvars: metamodelica::Array<bool>,
) -> Result<i32> {
    let __ab_mask = mask.borrow();
    let mut on: i32;
    on = 0;
    for mut i in ({
        let __s = metamodelica::arrayLength(m.clone());
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        if (*metamodelica::index_checked(&__ab_mask, i)?).clone() {
            if partitionIndependentBlocksWork(
                i,
                false,
                on + 1,
                m.clone(),
                mT.clone(),
                rm.clone(),
                rmT.clone(),
                eqPartMap.clone(),
                varPartMap.clone(),
                remEqPartMap.clone(),
                vars.clone(),
                rvars.clone(),
            )? {
                on = on + 1;
            }
        }
    }
    for mut i in ({
        let __s = metamodelica::arrayLength(rm.clone());
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        if partitionIndependentBlocksWork(
            i,
            true,
            on + 1,
            m.clone(),
            mT.clone(),
            rm.clone(),
            rmT.clone(),
            eqPartMap.clone(),
            varPartMap.clone(),
            remEqPartMap.clone(),
            vars.clone(),
            rvars.clone(),
        )? {
            on = on + 1;
        }
    }
    Ok(on)
}

fn partitionIndependentBlocksWork(
    mut idx: i32,
    mut isRemovedIdx: bool,
    mut partIdx: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rm: metamodelica::Array<metamodelica::List<i32>>,
    mut rmT: metamodelica::Array<metamodelica::List<i32>>,
    mut eqPartMap: metamodelica::Array<i32>,
    mut varPartMap: metamodelica::Array<i32>,
    mut rixs: metamodelica::Array<i32>,
    mut vars: metamodelica::Array<bool>,
    mut rvars: metamodelica::Array<bool>,
) -> Result<bool> {
    let mut ochange: bool;
    let mut eqIdx: i32;
    let mut rmIdx: i32;
    let mut workListEq: metamodelica::List<i32> = metamodelica::nil();
    let mut workListRm: metamodelica::List<i32> = metamodelica::nil();
    ochange = false;
    if isRemovedIdx {
        if metamodelica::arrayGet(rixs.clone(), idx)? == 0 {
            metamodelica::arrayUpdate(rixs.clone(), idx, partIdx)?;
            workListRm = list![idx];
            ochange = true;
        }
    } else {
        if metamodelica::arrayGet(eqPartMap.clone(), idx)? == 0 {
            metamodelica::arrayUpdate(eqPartMap.clone(), idx, partIdx)?;
            workListEq = list![idx];
            ochange = true;
        }
    }
    if !(ochange) {
        return Ok(ochange);
    }
    while !((workListEq).is_empty() && (workListRm).is_empty()) {
        if !((workListEq).is_empty()) {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(workListEq) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            eqIdx = metamodelica::Own::own(__pa0);
            workListEq = metamodelica::Own::own(__pa1);
            for mut varIdx in &*metamodelica::arrayGet(m.clone(), eqIdx)? {
                if !(metamodelica::arrayGet(vars.clone(), intAbs(varIdx.clone()))?) {
                    metamodelica::arrayUpdate(vars.clone(), intAbs(varIdx.clone()), true)?;
                    metamodelica::arrayUpdate(varPartMap.clone(), intAbs(varIdx.clone()), partIdx)?;
                    for mut nextEqIdx in &*metamodelica::arrayGet(mT.clone(), intAbs(varIdx.clone()))? {
                        if metamodelica::arrayGet(eqPartMap.clone(), intAbs(nextEqIdx.clone()))? == 0 {
                            workListEq = metamodelica::cons(intAbs(nextEqIdx.clone()), workListEq);
                            metamodelica::arrayUpdate(eqPartMap.clone(), intAbs(nextEqIdx.clone()), partIdx)?;
                        }
                    }
                    for mut nextEqIdx in &*metamodelica::arrayGet(rmT.clone(), intAbs(varIdx.clone()))? {
                        if metamodelica::arrayGet(rixs.clone(), intAbs(nextEqIdx.clone()))? == 0 {
                            workListRm = metamodelica::cons(intAbs(nextEqIdx.clone()), workListRm);
                            metamodelica::arrayUpdate(rixs.clone(), intAbs(nextEqIdx.clone()), partIdx)?;
                        }
                    }
                }
            }
        } else {
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(workListRm) {
                Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            rmIdx = metamodelica::Own::own(__pa2);
            workListRm = metamodelica::Own::own(__pa3);
            for mut varIdx in &*metamodelica::arrayGet(rm.clone(), rmIdx)? {
                if !(metamodelica::arrayGet(rvars.clone(), intAbs(varIdx.clone()))?) {
                    metamodelica::arrayUpdate(rvars.clone(), intAbs(varIdx.clone()), true)?;
                    for mut nextEqIdx in &*metamodelica::arrayGet(mT.clone(), intAbs(varIdx.clone()))? {
                        if metamodelica::arrayGet(eqPartMap.clone(), intAbs(nextEqIdx.clone()))? == 0 {
                            workListEq = metamodelica::cons(intAbs(nextEqIdx.clone()), workListEq);
                            metamodelica::arrayUpdate(eqPartMap.clone(), intAbs(nextEqIdx.clone()), partIdx)?;
                        }
                    }
                    for mut nextEqIdx in &*metamodelica::arrayGet(rmT.clone(), intAbs(varIdx.clone()))? {
                        if metamodelica::arrayGet(rixs.clone(), intAbs(nextEqIdx.clone()))? == 0 {
                            workListRm = metamodelica::cons(intAbs(nextEqIdx.clone()), workListRm);
                            metamodelica::arrayUpdate(rixs.clone(), intAbs(nextEqIdx.clone()), partIdx)?;
                        }
                    }
                }
            }
        }
    }
    Ok(ochange)
}

pub(crate) fn partitionIndependentBlocksSplitBlocks(
    mut n: i32,
    mut inSyst: metamodelica::Ref<BackendDAE::EqSystem>,
    mut ixs: metamodelica::Array<i32>,
    mut rixs: metamodelica::Array<i32>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut rmT: metamodelica::Array<metamodelica::List<i32>>,
    mut throwNoError: bool,
    mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut isInitial: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::Array<i32>,
)> {
    let __ab_mT = mT.borrow();
    let __ab_rmT = rmT.borrow();
    let mut systs: metamodelica::List<metamodelica::Ref<BackendDAE::EqSystem>> = metamodelica::nil();
    let mut unpartRemovedEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut varPartMap: metamodelica::Array<i32>;
    let mut ea: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut rea: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut va: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>;
    let mut i1: i32;
    let mut i2: i32;
    let mut b: bool;
    let mut b1: bool = true;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut varsPartition: metamodelica::Array<i32>;
    let mut lstVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    ea = arrayCreate(n, metamodelica::nil());
    rea = arrayCreate(n, metamodelica::nil());
    va = arrayCreate(n, metamodelica::nil());
    varPartMap = arrayCreate(n, -1);
    i1 = BackendEquation::equationArraySize(inSyst.orderedEqs.clone())?;
    i2 = BackendVariable::varsSize(&inSyst.orderedVars);
    if i1 != i2 && !(throwNoError) {
        Error::addSourceMessage(
            &(if (i1 > i2) {
                Error::OVERDET_EQN_SYSTEM.clone()
            } else {
                Error::UNDERDET_EQN_SYSTEM.clone()
            }),
            list![
                ArcStr::from(::std::format!("{}", i1)),
                ArcStr::from(::std::format!("{}", i2))
            ],
            &(Absyn::dummyInfo.clone()),
        )?;
        BackendDAEUtil::checkAdjacencyMatrixSolvability(inSyst.clone(), funcs, isInitial)?;
        return Err("fail");
    }
    partitionEquations(inSyst.orderedEqs.clone(), ixs.clone(), ea.clone())?;
    unpartRemovedEqs = partitionEquations(inSyst.removedEqs.clone(), rixs.clone(), rea.clone())?;
    varsPartition = arrayCreate(BackendVariable::varsSize(&inSyst.orderedVars), 0);
    for mut i in 1..=BackendVariable::varsSize(&inSyst.orderedVars) {
        setVarPartition(
            varsPartition.clone(),
            i,
            &(*metamodelica::index_checked(&__ab_mT, i)?),
            ixs.clone(),
        )?;
        setVarPartition(
            varsPartition.clone(),
            i,
            &(*metamodelica::index_checked(&__ab_rmT, i)?),
            rixs.clone(),
        )?;
    }
    for mut i in ({
        let __s = metamodelica::arrayLength(varsPartition.clone());
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        if ({
            let __elt = (*metamodelica::index_checked(&varsPartition.borrow(), i)?).clone();
            __elt
        }) != 0
        {
            lstVars = ({
                let __elt = (*metamodelica::index_checked(
                    &va.borrow(),
                    ({
                        let __elt = (*metamodelica::index_checked(&varsPartition.borrow(), i)?).clone();
                        __elt
                    }),
                )?)
                .clone();
                __elt
            });
            metamodelica::arrayUpdate(
                va.clone(),
                ({
                    let __elt = (*metamodelica::index_checked(&varsPartition.borrow(), i)?).clone();
                    __elt
                }),
                metamodelica::cons(BackendVariable::getVarAt(&inSyst.orderedVars, i)?, lstVars),
            )?;
        }
    }
    for mut i in 1..=n {
        let (__pa0, (__pa1, _)) = createEqSystem(
            ({
                let __elt = (*metamodelica::index_checked(&ea.borrow(), i)?).clone();
                __elt
            }),
            ({
                let __elt = (*metamodelica::index_checked(&va.borrow(), i)?).clone();
                __elt
            }),
            &({
                let __elt = (*metamodelica::index_checked(&rea.borrow(), i)?).clone();
                __elt
            }),
            (true, throwNoError),
        )?;
        syst = metamodelica::Own::own(__pa0);
        b = metamodelica::Own::own(__pa1);
        systs = metamodelica::cons(syst, systs);
        b1 = b1 && b;
    }
    let true = (throwNoError || b1) else {
        return Err("pattern mismatch");
    };
    systs = systs.reverse();
    Ok((systs, unpartRemovedEqs, varPartMap))
}

fn setVarPartition(
    mut varsPartition: metamodelica::Array<i32>,
    mut i: i32,
    mut eqsIxs: &metamodelica::List<i32>,
    mut eqsPartitions: metamodelica::Array<i32>,
) -> Result<()> {
    let __ab_eqsPartitions = eqsPartitions.borrow();
    let mut partitionIdx: i32;
    for mut eq in &**eqsIxs {
        partitionIdx = (*metamodelica::index_checked(&__ab_eqsPartitions, eq.clone())?).clone();
        if partitionIdx != 0 {
            assert!(
                ({
                    let __elt = (*metamodelica::index_checked(&varsPartition.borrow(), i)?).clone();
                    __elt
                }) == 0
                    || ({
                        let __elt = (*metamodelica::index_checked(&varsPartition.borrow(), i)?).clone();
                        __elt
                    }) == partitionIdx,
                "{}",
                &*literal!("SynchronousFeatures.setVarPartition failed")
            );
            metamodelica::arrayUpdate(varsPartition.clone(), i, partitionIdx)?;
        }
    }
    Ok(())
}

fn createEqSystem(
    mut el: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut vl: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut rel: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut iTpl: (bool, bool),
) -> Result<(metamodelica::Ref<BackendDAE::EqSystem>, (bool, bool))> {
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem>;
    let mut oTpl: (bool, bool);
    let mut arr: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut remArr: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>;
    let mut vars: BackendDAE::Variables;
    let mut i1: i32;
    let mut i2: i32;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    let mut s3: ArcStr;
    let mut s4: ArcStr;
    let mut crs: metamodelica::List<ArcStr>;
    let mut success: bool;
    let mut throwNoError: bool;
    (success, throwNoError) = iTpl;
    vars = BackendVariable::listVar1(&vl)?;
    arr = BackendEquation::listEquation(&el)?;
    remArr = BackendEquation::listEquation(rel)?;
    i1 = BackendEquation::equationArraySize(arr.clone())?;
    i2 = BackendVariable::varsSize(&vars);
    if i1 != i2 && !(throwNoError) {
        s1 = intString(i1);
        s2 = intString(i2);
        crs = List::mapMap(
            vl,
            &move |__a0: metamodelica::Ref<BackendDAE::Var>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(BackendVariable::varCref(&__a0))
            },
            &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::printComponentRefStr(&__a0),
        )?;
        s3 = stringDelimitList(crs, literal!("\n"));
        s4 = BackendDump::dumpEqnsStr(el)?;
        Error::addSourceMessage(
            &(Error::IMBALANCED_EQUATIONS.clone()),
            list![s1, s2, s3, s4],
            &(Absyn::dummyInfo.clone()),
        )?;
        return Err("fail");
    }
    syst = BackendDAEUtil::createEqSystem(
        vars,
        arr,
        metamodelica::nil(),
        openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
        remArr,
    );
    success = success && i1 == i2;
    oTpl = (success, throwNoError);
    Ok((syst, oTpl))
}

fn partitionEquations(
    mut arr: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>>,
    mut ixs: metamodelica::Array<i32>,
    mut ea: metamodelica::Array<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    let __ab_ixs = ixs.borrow();
    let mut restEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut ix: i32;
    let mut lst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    for mut i in ({
        let __s = BackendEquation::getNumberOfEquations(arr.clone());
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        ix = (*metamodelica::index_checked(&__ab_ixs, i)?).clone();
        eq = BackendEquation::get(arr.clone(), i)?;
        if ix == 0 {
            restEqs = metamodelica::cons(eq, restEqs);
        } else {
            lst = ({
                let __elt = (*metamodelica::index_checked(&ea.borrow(), ix)?).clone();
                __elt
            });
            lst = metamodelica::cons(eq, lst);
            metamodelica::arrayUpdate(ea.clone(), ix, lst)?;
        }
    }
    Ok(restEqs)
}

fn subClkEqual(mut sc1: &BackendDAE::SubClock, mut sc2: &BackendDAE::SubClock) -> Result<bool> {
    let mut isEqual: bool;
    isEqual = (match (sc1.clone(), sc2.clone()) {
        (BackendDAE::SubClock::INFERED_SUBCLOCK { .. }, BackendDAE::SubClock::INFERED_SUBCLOCK { .. }) => true,
        (BackendDAE::SubClock::SUBCLOCK { .. }, BackendDAE::SubClock::SUBCLOCK { .. }) => {
            MMath::equals(
                var_field!(sc1.factor, BackendDAE::SubClock::SUBCLOCK).clone(),
                var_field!(sc2.factor, BackendDAE::SubClock::SUBCLOCK).clone(),
            )? && MMath::equals(
                var_field!(sc1.shift, BackendDAE::SubClock::SUBCLOCK).clone(),
                var_field!(sc2.shift, BackendDAE::SubClock::SUBCLOCK).clone(),
            )? && Util::optionEqual(
                var_field!(sc1.solver, BackendDAE::SubClock::SUBCLOCK).clone(),
                var_field!(sc2.solver, BackendDAE::SubClock::SUBCLOCK).clone(),
                &fnptr!(stringEqual, ArcStr, ArcStr),
            )?
        }
        _ => false,
    });
    Ok(isEqual)
}

fn subClockTreeString(mut treeIn: metamodelica::Array<(BackendDAE::SubClock, i32)>) -> ArcStr {
    let mut sOut: ArcStr = literal!("");
    let mut tpl: (BackendDAE::SubClock, i32) = (BackendDAE::SubClock::INFERED_SUBCLOCK, 0);
    let mut subClock: BackendDAE::SubClock;
    let mut i: i32;
    let mut idx: i32 = 1;
    let __range0 = treeIn.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut tpl in __range0 {
        (subClock, i) = tpl;
        sOut = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*intString(idx));
            __mm_s.push_str(&*literal!(": ["));
            __mm_s.push_str(&*intString(i));
            __mm_s.push_str(&*literal!("]:  "));
            __mm_s.push_str(&*BackendDump::subClockString(&subClock));
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*sOut);
            ArcStr::from(__mm_s)
        };
        idx = idx + 1;
    }
    sOut
}
