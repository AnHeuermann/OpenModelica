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

use crate::NFAttributes as Attributes;
use crate::NFBackendExtension;
use crate::NFBinding as Binding;
use crate::NFBuiltinFuncs;
use crate::NFCall as Call;
use crate::NFClockKind;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFEquation::ScalarizeMode;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFOperator as Operator;
use crate::NFPrefixes::Purity;
use crate::NFPrefixes::Variability;
use crate::NFPrefixes::Visibility;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Global;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::MutableWeak;

// ============================================================
// Internal data types
// ============================================================
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Transition {
    pub from: i32,
    pub to: i32,
    pub condition: metamodelica::Ref<Expression::NFExpression>,
    pub immediate: bool,
    pub reset: bool,
    pub synchronize: bool,
    pub priority: i32,
}

impl metamodelica::gc::MMTrace for Transition {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.from, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.to, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.condition, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.immediate, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.reset, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.synchronize, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.priority, __mmv)?;
        Ok(())
    }
}
impl Default for Transition {
    fn default() -> Self {
        Self {
            from: Default::default(),
            to: Default::default(),
            condition: Default::default(),
            immediate: Default::default(),
            reset: Default::default(),
            synchronize: Default::default(),
            priority: Default::default(),
        }
    }
}

pub type TRANSITION = Transition;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FlatSmSemantics {
    /// Cref of the initial state (used as prefix for smOf vars)
    pub initStateRef: metamodelica::Ref<ComponentRef::NFComponentRef>,
    /// State crefs; index 1 = initial state
    pub smComps: metamodelica::Array<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    /// Transitions sorted by priority
    pub t: metamodelica::List<Transition>,
    /// Conditions sorted by priority
    pub c: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    /// SMS discrete variables
    pub vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    /// SMS parameters/constants
    pub knowns: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    /// SMS equations
    pub eqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    /// Propagation variables
    pub pvars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    /// Propagation equations
    pub peqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    /// Enclosing state if hierarchical SM
    pub enclosingState: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>,
}

impl metamodelica::gc::MMTrace for FlatSmSemantics {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.initStateRef, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.smComps, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.t, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.c, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.vars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.knowns, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.eqs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.pvars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.peqs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.enclosingState, __mmv)?;
        Ok(())
    }
}
impl Default for FlatSmSemantics {
    fn default() -> Self {
        Self {
            initStateRef: Default::default(),
            smComps: Default::default(),
            t: Default::default(),
            c: Default::default(),
            vars: Default::default(),
            knowns: Default::default(),
            eqs: Default::default(),
            pvars: Default::default(),
            peqs: Default::default(),
            enclosingState: Default::default(),
        }
    }
}

pub type FLAT_SM_SEMANTICS = FlatSmSemantics;

pub(crate) const SMS_PRE: &'static str = "smOf";

// ============================================================
// Public entry point
// ============================================================
pub(crate) fn flatten(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
) -> Result<metamodelica::Ref<FlatModel::NFFlatModel>> {
    pub(crate) type OuterVarList = metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<ComponentRef::NFComponentRef>,
    )>;

    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    let mut initStates: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut smGroups: metamodelica::List<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    let mut smEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut otherEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut resultEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut smVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    let mut resultVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    let mut allStateCrefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut outerVarMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<ComponentRef::NFComponentRef>,
            )>,
        >,
    >;
    let mut stateToSem: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, FlatSmSemantics>,
    >;
    let mut smGroupPairs: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    )>;
    let mut smGroupsSorted: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    )>;
    let mut sem: FlatSmSemantics;
    let mut initState: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut parentPrefix: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut stateCrefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut enclosingStateCrefOpt: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut enclosingSmSemOpt: Option<FlatSmSemantics>;
    if !(List::any(&flatModel.equations, &move |__a0: metamodelica::Ref<
        Equation::NFEquation,
    >| isTransitionOrInitialState(&__a0))?)
        && !(List::any(&flatModel.initialEquations, &move |__a0: metamodelica::Ref<
            Equation::NFEquation,
        >| {
            isTransitionOrInitialState(&__a0)
        })?)
    {
        return Ok(flatModel);
    }
    (initStates, smGroups) = groupStateMachines(flatModel.equations.clone(), flatModel.initialEquations.clone())?;
    if (initStates).is_empty() {
        return Ok(flatModel);
    }
    allStateCrefs = List::flatten(smGroups.clone())?;
    otherEqs = List::filterOnFalse(flatModel.equations.clone(), &move |__a0: metamodelica::Ref<
        Equation::NFEquation,
    >| isTransitionOrInitialState(&__a0))?;
    otherEqs = List::filterOnFalse(
        otherEqs,
        &({
            let __pe_b1 = allStateCrefs;
            move |__pe_a0| isOuterStateEquation(&__pe_a0, &__pe_b1)
        }),
    )?;
    outerVarMap = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0))
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                  __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                ComponentRef::isEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    smGroupPairs = List::zip(initStates, smGroups);
    smGroupsSorted = List::sort(
        smGroupPairs,
        (std::sync::Arc::new(
            move |__a0: (
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            ),
                  __a1: (
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            )|
                  -> metamodelica::Result<_> { ::std::result::Result::Ok(smGroupDepthLt(&__a0, &__a1)) },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        (
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                        ),
                        (
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                        ),
                    ) -> Result<bool>
                    + 'static,
            >),
    )?;
    stateToSem = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0))
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                  __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                ComponentRef::isEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    smVars = metamodelica::nil();
    smEqs = metamodelica::nil();
    for mut smPair in &*smGroupsSorted {
        (initState, stateCrefs) = smPair.clone();
        parentPrefix = ComponentRef::rest(&initState)?;
        if ComponentRef::isEmpty(&parentPrefix) {
            enclosingStateCrefOpt = None;
            enclosingSmSemOpt = None;
        } else {
            enclosingSmSemOpt = UnorderedMap::get(parentPrefix.clone(), stateToSem.clone())?;
            enclosingStateCrefOpt = if ((enclosingSmSemOpt).is_some()) {
                Some(parentPrefix)
            } else {
                None
            };
        }
        (smEqs, smVars, sem) = flatSmToDataFlow(
            initState,
            stateCrefs.clone(),
            flatModel.equations.clone(),
            flatModel.variables.clone(),
            enclosingStateCrefOpt,
            enclosingSmSemOpt,
            smEqs,
            smVars,
            outerVarMap.clone(),
        )?;
        for mut sc in &*stateCrefs {
            UnorderedMap::addUnique(sc.clone(), sem.clone(), stateToSem.clone())?;
        }
    }
    for mut outerVarCref in &*UnorderedMap::keyList(outerVarMap.clone()) {
        (smEqs, smVars) = generateMergeEquation(
            outerVarCref.clone(),
            outerVarMap.clone(),
            &flatModel.variables,
            smEqs,
            smVars,
        )?;
    }
    resultEqs = listAppend(
        ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
            for mut eq in (smEqs).into_iter().cloned() {
                let __x = subsActiveStateInEq(eq.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
            for mut eq in (otherEqs).into_iter().cloned() {
                let __x = subsActiveStateInEq(eq.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    );
    resultVars = listAppend(smVars, flatModel.variables.clone());
    assign_field!(
        flatModel.equations = resultEqs,
        flatModel.initialEquations =
            List::filterOnFalse(flatModel.initialEquations.clone(), &move |__a0: metamodelica::Ref<
                Equation::NFEquation,
            >| {
                isTransitionOrInitialState(&__a0)
            })?,
        flatModel.variables = resultVars
    );
    execStat(&(literal!("NFStateMachineFlatten.flatten")))?;
    Ok(flatModel)
}

// ============================================================
// SM group detection
// ============================================================
fn groupStateMachines(
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut initialEquations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    metamodelica::List<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
)> {
    let mut initStates: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
    let mut smGroups: metamodelica::List<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
        metamodelica::nil();
    let mut allFroms: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
    let mut allTos: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
    let mut allInits: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
    let mut cr1: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut cr2: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut group: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    for mut eq in &*listAppend(equations, initialEquations) {
        let () = (::match_deref::match_deref! { match &(eq.clone()) {
            Deref @ Equation::NORETCALL { exp: Deref @ Expression::CALL { call: eqCall }, .. } => {
                let mut fname: ArcStr;
                fname = Call::functionNameLast(metamodelica::AsArg::as_arg(&eqCall))?;
                if stringEq(&fname, &(literal!("transition"))) {
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::firstN(Call::arguments(metamodelica::AsArg::as_arg(&eqCall))?, 2)?) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::CREF { cref: __pa0, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::CREF { cref: __pa1, .. }, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cr1 = metamodelica::Own::own(__pa0);
                    cr2 = metamodelica::Own::own(__pa1);
                    allFroms = metamodelica::cons(cr1, allFroms);
                    allTos = metamodelica::cons(cr2, allTos);
                } else if stringEq(&fname, &(literal!("initialState"))) {
                    let __pa3 = ::match_deref::match_deref! { match &(List::firstN(Call::arguments(metamodelica::AsArg::as_arg(&eqCall))?, 1)?) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::CREF { cref: __pa3, .. }, tail: Deref @ metamodelica::ListNode::Nil } => __pa3.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cr1 = metamodelica::Own::own(__pa3);
                    allInits = metamodelica::cons(cr1, allInits);
                }
                ()
            },
            _ => {
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    for mut initCref in &*allInits {
        group = collectReachableStates(initCref.clone(), &allFroms, &allTos)?;
        initStates = metamodelica::cons(initCref.clone(), initStates);
        smGroups = metamodelica::cons(group, smGroups);
    }
    initStates = initStates.reverse();
    smGroups = smGroups.reverse();
    Ok((initStates, smGroups))
}

fn collectReachableStates(
    mut initCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut froms: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut tos: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
    let mut states: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut queue: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = list![initCref.clone()];
    let mut visited: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
    let mut cur: metamodelica::Ref<ComponentRef::NFComponentRef>;
    states = metamodelica::nil();
    while !((queue).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(queue) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cur = metamodelica::Own::own(__pa0);
        queue = metamodelica::Own::own(__pa1);
        if !(List::isMemberOnTrue(cur.clone(), &visited, &move |__a0: metamodelica::Ref<
            ComponentRef::NFComponentRef,
        >,
                                                                __a1: metamodelica::Ref<
            ComponentRef::NFComponentRef,
        >| ComponentRef::isEqual(&__a0, &__a1))?)
        {
            visited = metamodelica::cons(cur.clone(), visited);
            states = metamodelica::cons(cur.clone(), states);
            for mut i in 1..=((froms).len() as i32) {
                if ComponentRef::isEqual(&((froms).get(i)?), &cur)? {
                    queue = metamodelica::cons((tos).get(i)?, queue);
                }
                if ComponentRef::isEqual(&((tos).get(i)?), &cur)? {
                    queue = metamodelica::cons((froms).get(i)?, queue);
                }
            }
        }
    }
    states = List::sort(
        states,
        (std::sync::Arc::new({
            let __pe_b2 = initCref;
            move |__pe_a0, __pe_a1| statePriorityGt(&__pe_a0, &__pe_a1, &__pe_b2)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >),
    )?;
    Ok(states)
}

fn statePriorityGt(
    mut cr1: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut cr2: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut initCref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<bool> {
    let mut gt: bool;
    if ComponentRef::isEqual(cr2, initCref)? {
        gt = true;
    } else if ComponentRef::isEqual(cr1, initCref)? {
        gt = false;
    } else {
        gt = ComponentRef::toString(cr1)? > ComponentRef::toString(cr2)?;
    }
    Ok(gt)
}

// ============================================================
// Flat SM to data-flow transformation
// ============================================================
fn smGroupDepthLt(
    mut g1: &(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    ),
    mut g2: &(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    ),
) -> bool {
    let mut lt: bool;
    let mut c1: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut c2: metamodelica::Ref<ComponentRef::NFComponentRef>;
    (c1, _) = g1.clone();
    (c2, _) = g2.clone();
    lt = ComponentRef::depth(&c1) < ComponentRef::depth(&c2);
    lt
}

fn flatSmToDataFlow(
    mut initStateCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut stateCrefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut allEquations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut allVariables: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut enclosingStateCrefOpt: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut enclosingSmSemOpt: Option<FlatSmSemantics>,
    mut accEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut accVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut outerVarMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<ComponentRef::NFComponentRef>,
            )>,
        >,
    >,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    FlatSmSemantics,
)> {
    let mut accEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = accEqs;
    let mut accVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = accVars;
    let mut outSem: FlatSmSemantics;
    let mut transitionEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut initialStateEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut sem: FlatSmSemantics;
    let mut semWithProp: FlatSmSemantics;
    let mut semFinal: FlatSmSemantics;
    let mut parentPrefix: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut varCrefStrings: metamodelica::List<ArcStr>;
    transitionEqs = List::filterOnTrue(
        allEquations.clone(),
        (std::sync::Arc::new({
            let __pe_b1 = stateCrefs.clone();
            move |__pe_a0| isTransitionForGroup(&__pe_a0, &__pe_b1)
        })
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Equation::NFEquation>) -> Result<bool> + 'static>),
    )?;
    initialStateEqs = List::filterOnTrue(
        allEquations.clone(),
        (std::sync::Arc::new({
            let __pe_b1 = initStateCref.clone();
            move |__pe_a0| isInitialStateForGroup(&__pe_a0, &__pe_b1)
        })
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Equation::NFEquation>) -> Result<bool> + 'static>),
    )?;
    sem = basicFlatSmSemantics(initStateCref, stateCrefs.clone(), &transitionEqs)?;
    semWithProp = addPropagationEquations(sem, enclosingStateCrefOpt.clone(), enclosingSmSemOpt)?;
    semFinal = elabXInStateOps(semWithProp, enclosingStateCrefOpt)?;
    parentPrefix = ComponentRef::rest(&((stateCrefs).head().cloned()?))?;
    if !(ComponentRef::isEmpty(&parentPrefix)) {
        varCrefStrings = ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut v in (allVariables.clone()).into_iter().cloned() {
                let __x = ComponentRef::toString(&(v.name.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        semFinal.eqs = List::map(
            semFinal.eqs.clone(),
            &({
                let __pe_b1 = (std::sync::Arc::new({
                    let __pe_b1 = parentPrefix;
                    let __pe_b2 = varCrefStrings;
                    move |__pe_a0| qualifyOuterVarExpr(__pe_a0, &__pe_b1, &__pe_b2)
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >);
                move |__pe_a0| Equation::mapExp(__pe_a0, &*__pe_b1)
            }),
        )?;
    }
    accVars = List::flatten(list![
        accVars,
        semFinal.vars.clone(),
        semFinal.knowns.clone(),
        semFinal.pvars.clone()
    ])?;
    accEqs = List::flatten(list![accEqs, semFinal.eqs.clone(), semFinal.peqs.clone()])?;
    for mut stateCref in &*stateCrefs {
        (accEqs, accVars) = smCompToDataFlow(
            stateCref.clone(),
            &semFinal,
            allEquations.clone(),
            allVariables.clone(),
            accEqs,
            accVars,
            outerVarMap.clone(),
        )?;
    }
    outSem = semFinal;
    Ok((accEqs, accVars, outSem))
}

fn qualifyOuterVarExpr(
    mut e: metamodelica::Ref<Expression::NFExpression>,
    mut parentPrefix: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut varCrefStrings: &metamodelica::List<ArcStr>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut e: metamodelica::Ref<Expression::NFExpression> = e;
    e = Expression::map(
        e,
        (std::sync::Arc::new({
            let __pe_b1 = parentPrefix.clone();
            let __pe_b2 = varCrefStrings.clone();
            move |__pe_a0| qualifyOuterVarCref(__pe_a0, &__pe_b1, __pe_b2.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(e)
}

fn qualifyOuterVarCref(
    mut e: metamodelica::Ref<Expression::NFExpression>,
    mut parentPrefix: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut varCrefStrings: metamodelica::List<ArcStr>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut e: metamodelica::Ref<Expression::NFExpression> = e;
    let mut qualCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let () = (match &*e {
        Expression::CREF { cref: __e_cref, .. } if (ComponentRef::isSimple(metamodelica::AsArg::as_arg(&__e_cref))) => {
            qualCref = ComponentRef::append(__e_cref.clone(), parentPrefix)?;
            if listMember(ComponentRef::toString(&qualCref)?, varCrefStrings) {
                e = metamodelica::Ref::new(Expression::NFExpression::CREF {
                    ty: var_field!((*e).ty, Expression::NFExpression::CREF).clone(),
                    cref: qualCref,
                });
            }
            ()
        }
        _ => (),
    });
    Ok(e)
}

// ============================================================
// State machine component to data-flow
// ============================================================
fn smCompToDataFlow(
    mut stateCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut sem: &FlatSmSemantics,
    mut allEquations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut allVariables: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut accEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut accVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut outerVarMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<ComponentRef::NFComponentRef>,
            )>,
        >,
    >,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
)> {
    let mut accEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = accEqs;
    let mut accVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = accVars;
    let mut stateEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut stateVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    let mut crToStart: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >;
    let mut transformedEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut extraVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    stateEqs = List::filterOnTrue(
        allEquations,
        (std::sync::Arc::new({
            let __pe_b1 = stateCref.clone();
            move |__pe_a0| isEquationOfState(&__pe_a0, &__pe_b1)
        })
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Equation::NFEquation>) -> Result<bool> + 'static>),
    )?;
    stateVars = List::filterOnTrue(
        allVariables.clone(),
        (std::sync::Arc::new({
            let __pe_b1 = stateCref.clone();
            move |__pe_a0| isVariableOfState(&__pe_a0, &__pe_b1)
        })
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Variable::NFVariable>) -> Result<bool> + 'static>),
    )?;
    crToStart = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0))
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                  __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                ComponentRef::isEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    for mut v in &*stateVars {
        if List::any(
            &stateEqs,
            &({
                let __pe_b1 = v.name.clone();
                move |__pe_a0| equationHasPrevious(&__pe_a0, &__pe_b1)
            }),
        )? {
            UnorderedMap::addUnique(
                v.name.clone(),
                getStartValue(metamodelica::AsArg::as_arg(&v))?,
                crToStart.clone(),
            )?;
        }
    }
    transformedEqs = metamodelica::nil();
    extraVars = metamodelica::nil();
    for mut eq in &*stateEqs {
        (transformedEqs, extraVars) = addStateActivationAndReset(
            eq.clone(),
            stateCref.clone(),
            sem,
            crToStart.clone(),
            transformedEqs,
            extraVars,
            outerVarMap.clone(),
        )?;
    }
    accEqs = listAppend(transformedEqs.reverse(), accEqs);
    accVars = listAppend(extraVars.reverse(), accVars);
    addHierarchicalPassThroughs(stateCref, sem, &allVariables, outerVarMap)?;
    Ok((accEqs, accVars))
}

fn addHierarchicalPassThroughs(
    mut stateCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut sem: &FlatSmSemantics,
    mut allVariables: &metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut outerVarMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<ComponentRef::NFComponentRef>,
            )>,
        >,
    >,
) -> Result<()> {
    let mut stateStr: ArcStr;
    let mut leafName: ArcStr;
    let mut activeRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut topVarCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut topVar: metamodelica::Ref<Variable::NFVariable>;
    stateStr = ComponentRef::toString(&stateCref)?;
    activeRef = qCref(
        literal!("active"),
        crate::NFType::interned_BOOLEAN(),
        metamodelica::nil(),
        stateCref,
    )?;
    for mut v in &**allVariables {
        if !(ComponentRef::isSimple(&v.name))
            && stringEqual(&(ComponentRef::toString(&(ComponentRef::rest(&v.name)?))?), &stateStr)
        {
            leafName = ComponentRef::firstName(&v.name, false)?;
            if '__try0: {
                topVar = unwrap_break_err!(List::find(allVariables, &({ let __pe_b1 = leafName.clone(); move |__pe_a0| isSimpleVarNamed(&__pe_a0, &__pe_b1) })), '__try0);
                topVarCref = topVar.name.clone();
                if !(unwrap_break_err!(UnorderedMap::contains(topVarCref.clone(), outerVarMap.clone()), '__try0)) {
                    unwrap_break_err!(UnorderedMap::add(topVarCref.clone(), list![(activeRef.clone(), v.name.clone())], outerVarMap.clone()), '__try0);
                }
                Ok::<(), &'static str>(())
            }.is_err() {
            }
        }
    }
    Ok(())
}

fn isSimpleVarNamed(mut v: &metamodelica::Ref<Variable::NFVariable>, mut name: &ArcStr) -> Result<bool> {
    let mut res: bool;
    res = ComponentRef::isSimple(&v.name) && stringEqual(&(ComponentRef::firstName(&v.name, false)?), &name);
    Ok(res)
}

// ============================================================
// addStateActivationAndReset
// ============================================================
fn addStateActivationAndReset(
    mut inEq: metamodelica::Ref<Equation::NFEquation>,
    mut stateCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut sem: &FlatSmSemantics,
    mut crToStart: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut accEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut accVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut outerVarMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<ComponentRef::NFComponentRef>,
            )>,
        >,
    >,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
)> {
    let mut accEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = accEqs;
    let mut accVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = accVars;
    let () = (match &*inEq {
        Equation::EQUALITY { .. } => {
            (accEqs, accVars) =
                addStateActivationAndReset1(inEq, stateCref, sem, crToStart, accEqs, accVars, outerVarMap)?;
            ()
        }
        Equation::WHEN { .. } => {
            (accEqs, accVars) =
                transformWhenBranchesAndAccumulate(&inEq, stateCref, sem, crToStart, outerVarMap, accEqs, accVars)?;
            ()
        }
        _ => {
            accEqs = metamodelica::cons(inEq, accEqs);
            ()
        }
    });
    Ok((accEqs, accVars))
}

fn transformWhenBranchesAndAccumulate(
    mut whenEq: &metamodelica::Ref<Equation::NFEquation>,
    mut stateCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut sem: &FlatSmSemantics,
    mut crToStart: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut outerVarMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<ComponentRef::NFComponentRef>,
            )>,
        >,
    >,
    mut accEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut accVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
)> {
    let mut accEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = accEqs;
    let mut accVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = accVars;
    let mut branches: metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>>;
    let mut firstBranch: metamodelica::Ref<Equation::Branch::Branch>;
    let mut branchCond: metamodelica::Ref<Expression::NFExpression>;
    let mut outEq: metamodelica::Ref<Equation::NFEquation>;
    let mut extraVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    let mut innerEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut innerVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    let __pa0 = ::match_deref::match_deref! { match &((*whenEq)) {
        Deref @ Equation::WHEN { branches: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    branches = metamodelica::Own::own(__pa0);
    firstBranch = (branches).head().cloned()?;
    let __pa1 = ::match_deref::match_deref! { match &(firstBranch) {
        Deref @ Equation::Branch::BRANCH { condition: __pa1, .. } => __pa1.clone(),
        _ => return Err("pattern mismatch"),
    } };
    branchCond = metamodelica::Own::own(__pa1);
    if Type::isClock(&(Expression::typeOf(branchCond)))? {
        (innerEqs, innerVars) = transformWhenInnerAsPlain(whenEq, stateCref, sem, crToStart, outerVarMap)?;
        accEqs = listAppend(innerEqs, accEqs);
        accVars = listAppend(innerVars, accVars);
    } else {
        (outEq, extraVars) = transformWhenBranches(whenEq, stateCref, sem, crToStart, outerVarMap)?;
        accEqs = metamodelica::cons(outEq, accEqs);
        accVars = listAppend(extraVars, accVars);
    }
    Ok((accEqs, accVars))
}

fn transformWhenInnerAsPlain(
    mut whenEq: &metamodelica::Ref<Equation::NFEquation>,
    mut stateCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut sem: &FlatSmSemantics,
    mut crToStart: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut outerVarMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<ComponentRef::NFComponentRef>,
            )>,
        >,
    >,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
)> {
    let mut outEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    let mut outVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
    let mut branches: metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>>;
    let mut branchBody: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut transformedBody: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut branchVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    let __pa0 = ::match_deref::match_deref! { match &((*whenEq)) {
        Deref @ Equation::WHEN { branches: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    branches = metamodelica::Own::own(__pa0);
    for mut branch in &*branches {
        let () = (match &*branch.clone() {
            Equation::Branch::BRANCH {
                body: __esc_branchBody, ..
            } => {
                branchBody = (*__esc_branchBody).clone();
                transformedBody = metamodelica::nil();
                branchVars = metamodelica::nil();
                for mut eq in &*branchBody.clone() {
                    (transformedBody, branchVars) = addStateActivationAndReset(
                        eq.clone(),
                        stateCref.clone(),
                        sem,
                        crToStart.clone(),
                        transformedBody,
                        branchVars,
                        outerVarMap.clone(),
                    )?;
                }
                outEqs = listAppend(transformedBody.reverse(), outEqs);
                outVars = listAppend(branchVars, outVars);
                ()
            }
            _ => (),
        });
    }
    Ok((outEqs, outVars))
}

fn transformWhenBranches(
    mut whenEq: &metamodelica::Ref<Equation::NFEquation>,
    mut stateCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut sem: &FlatSmSemantics,
    mut crToStart: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut outerVarMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<ComponentRef::NFComponentRef>,
            )>,
        >,
    >,
) -> Result<(
    metamodelica::Ref<Equation::NFEquation>,
    metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
)> {
    let mut outEq: metamodelica::Ref<Equation::NFEquation>;
    let mut extraVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
    let mut branches: metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>>;
    let mut newBranches: metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>>;
    let mut transformedBody: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut branchVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    let mut whenScope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let mut whenSource: metamodelica::Ref<DAE::ElementSource>;
    let mut branchCond: metamodelica::Ref<Expression::NFExpression>;
    let mut branchCondVar: Variability;
    let mut branchBody: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*whenEq)) {
        Deref @ Equation::WHEN { branches: __pa0, scope: __pa1, source: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    branches = metamodelica::Own::own(__pa0);
    whenScope = metamodelica::Own::own(__pa1);
    whenSource = metamodelica::Own::own(__pa2);
    newBranches = metamodelica::nil();
    for mut branch in &*branches {
        let mut branch = branch.clone();
        branch = (match &*branch {
            Equation::Branch::BRANCH {
                condition: __esc_branchCond,
                conditionVar: __esc_branchCondVar,
                body: __esc_branchBody,
            } => {
                branchCond = (*__esc_branchCond).clone();
                branchCondVar = (*__esc_branchCondVar).clone();
                branchBody = (*__esc_branchBody).clone();
                transformedBody = metamodelica::nil();
                branchVars = metamodelica::nil();
                for mut eq in &*branchBody.clone() {
                    (transformedBody, branchVars) = addStateActivationAndReset(
                        eq.clone(),
                        stateCref.clone(),
                        sem,
                        crToStart.clone(),
                        transformedBody,
                        branchVars,
                        outerVarMap.clone(),
                    )?;
                }
                extraVars = listAppend(branchVars, extraVars);
                metamodelica::Ref::new(Equation::Branch::Branch::BRANCH {
                    condition: branchCond.clone(),
                    conditionVar: branchCondVar.clone(),
                    body: transformedBody.reverse(),
                })
            }
            _ => branch,
        });
        newBranches = metamodelica::cons(branch, newBranches);
    }
    outEq = metamodelica::Ref::new(Equation::NFEquation::WHEN {
        branches: newBranches.reverse(),
        scope: whenScope,
        source: whenSource,
    });
    Ok((outEq, extraVars))
}

fn addStateActivationAndReset1(
    mut inEq: metamodelica::Ref<Equation::NFEquation>,
    mut stateCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut sem: &FlatSmSemantics,
    mut crToStart: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut accEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut accVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut outerVarMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<ComponentRef::NFComponentRef>,
            )>,
        >,
    >,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
)> {
    let mut accEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = accEqs;
    let mut accVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = accVars;
    let mut lhs: metamodelica::Ref<Expression::NFExpression>;
    let mut rhs: metamodelica::Ref<Expression::NFExpression>;
    let mut lhsCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut perStateVarCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut stateActiveCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut lhsTy: metamodelica::Ref<Type::NFType>;
    let mut eqScope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let mut eqSource: metamodelica::Ref<DAE::ElementSource>;
    let mut stateVarCrefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut hasStateVarOnLHS: bool;
    let mut isOuterOutput: bool;
    let mut newRhs: metamodelica::Ref<Expression::NFExpression>;
    let mut perStateVarExp: metamodelica::Ref<Expression::NFExpression>;
    let mut eq1: metamodelica::Ref<Equation::NFEquation>;
    let mut eq2: metamodelica::Ref<Equation::NFEquation>;
    let mut perStateVar: metamodelica::Ref<Variable::NFVariable>;
    let mut prevList: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<ComponentRef::NFComponentRef>,
    )>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(inEq.clone()) {
        Deref @ Equation::EQUALITY { lhs: __pa0, rhs: __pa1, ty: __pa2, scope: __pa3, source: __pa4, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    lhs = metamodelica::Own::own(__pa0);
    rhs = metamodelica::Own::own(__pa1);
    lhsTy = metamodelica::Own::own(__pa2);
    eqScope = metamodelica::Own::own(__pa3);
    eqSource = metamodelica::Own::own(__pa4);
    stateVarCrefs = UnorderedMap::keyList(crToStart.clone());
    match '__try5: {
        let (__pa6, __pa7) = ::match_deref::match_deref! { match &(lhs.clone()) {
            Deref @ Expression::CREF { ty: __pa6, cref: __pa7 } => (__pa6.clone(), __pa7.clone()),
            _ => break '__try5 Err::<_, _>("pattern mismatch"),
        } };
        lhsTy = metamodelica::Own::own(__pa6);
        lhsCref = metamodelica::Own::own(__pa7);
        (newRhs, _) = unwrap_break_err!(Expression::mapFold(rhs.clone(), (std::sync::Arc::new({ let __pe_b1 = stateVarCrefs.clone(); move |__pe_a0, __pe_a2| Ok(subsPreviousCrefs(__pe_a0, &__pe_b1, __pe_a2)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, bool) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> + 'static>), false), '__try5);
        eq1 = metamodelica::Ref::new(Equation::NFEquation::EQUALITY {
            lhs: lhs.clone(),
            rhs: newRhs.clone(),
            ty: lhsTy.clone(),
            scope: eqScope.clone(),
            source: eqSource.clone(),
            scalarizeMode: ScalarizeMode::NO_PREFERENCE.clone(),
        });
        isOuterOutput = !(unwrap_break_err!(crefHasPrefix(&stateCref, lhsCref.clone()), '__try5))
            && stringEqual(
                &(unwrap_break_err!(NFInstNode::InstNode::name(&(unwrap_break_err!(NFInstNode::InstNode::fromCell(eqScope.clone()), '__try5))), '__try5)),
                &(unwrap_break_err!(ComponentRef::firstName(&stateCref, false), '__try5)),
            );
        if isOuterOutput {
            perStateVarCref = unwrap_break_err!(ComponentRef::prefixCref(metamodelica::Ref::new(InstNode::InstNode::NAME_NODE { name: unwrap_break_err!(ComponentRef::firstName(&lhsCref, false), '__try5) }), lhsTy.clone(), metamodelica::nil(), stateCref.clone()), '__try5);
            perStateVar = makeVarWithStart(
                perStateVarCref.clone(),
                lhsTy.clone(),
                Variability::DISCRETE.clone(),
                getDefaultStart(&lhsTy),
            );
            perStateVarExp = makeCrefExp(perStateVarCref.clone(), lhsTy.clone());
            eq1 = metamodelica::Ref::new(Equation::NFEquation::EQUALITY {
                lhs: perStateVarExp.clone(),
                rhs: newRhs.clone(),
                ty: lhsTy.clone(),
                scope: eqScope.clone(),
                source: eqSource.clone(),
                scalarizeMode: ScalarizeMode::NO_PREFERENCE.clone(),
            });
            eq1 = unwrap_break_err!(wrapInStateActivationConditional(&eq1, stateCref.clone(), false), '__try5);
            accEqs = metamodelica::cons(eq1.clone(), accEqs.clone());
            accVars = metamodelica::cons(perStateVar.clone(), accVars.clone());
            stateActiveCref = unwrap_break_err!(qCref(literal!("active"), crate::NFType::interned_BOOLEAN(), metamodelica::nil(), stateCref.clone()), '__try5);
            prevList = unwrap_break_err!(UnorderedMap::getOrDefault(lhsCref.clone(), outerVarMap.clone(), metamodelica::nil()), '__try5);
            unwrap_break_err!(UnorderedMap::add(lhsCref.clone(), metamodelica::cons((stateActiveCref.clone(), perStateVarCref.clone()), prevList.clone()), outerVarMap.clone()), '__try5);
        } else {
            hasStateVarOnLHS = false;
            for mut svc in &*stateVarCrefs {
                hasStateVarOnLHS =
                    unwrap_break_err!(ComponentRef::isEqual(metamodelica::AsArg::as_arg(&svc), &lhsCref), '__try5);
                if hasStateVarOnLHS {
                    break;
                }
            }
            if hasStateVarOnLHS {
                eq1 = unwrap_break_err!(wrapInStateActivationConditional(&eq1, stateCref.clone(), true), '__try5);
                eq2 = unwrap_break_err!(createResetEquation(lhsCref.clone(), lhsTy.clone(), stateCref.clone(), sem, crToStart.clone()), '__try5);
                accEqs = metamodelica::cons(eq1.clone(), metamodelica::cons(eq2.clone(), accEqs.clone()));
                accVars = metamodelica::cons(
                    makeVar(
                        unwrap_break_err!(ComponentRef::prefixCref(metamodelica::Ref::new(InstNode::InstNode::NAME_NODE { name: { let mut __mm_s = String::new(); __mm_s.push_str(&*unwrap_break_err!(ComponentRef::firstName(&lhsCref, false), '__try5)); __mm_s.push_str(&*literal!("_previous")); ArcStr::from(__mm_s) } }), lhsTy.clone(), metamodelica::nil(), unwrap_break_err!(ComponentRef::rest(&lhsCref), '__try5)), '__try5),
                        lhsTy.clone(),
                        Variability::CONTINUOUS.clone(),
                    ),
                    accVars.clone(),
                );
            } else {
                accEqs = metamodelica::cons(
                    unwrap_break_err!(wrapInStateActivationConditional(&eq1, stateCref.clone(), false), '__try5),
                    accEqs.clone(),
                );
            }
        }
        Ok::<_, &'static str>((accEqs.clone(),))
    } {
        Ok((__try5_o0,)) => {
            accEqs = __try5_o0;
        }
        Err(_) => {
            accEqs = metamodelica::cons(inEq.clone(), accEqs.clone());
        }
    }
    Ok((accEqs, accVars))
}

fn equationHasPrevious(
    mut eq: &metamodelica::Ref<Equation::NFEquation>,
    mut varCref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<bool> {
    let mut found: bool;
    found = Equation::containsExp(
        eq,
        &({
            let __pe_b1: Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static,
            > = (std::sync::Arc::new({
                let __pe_b1 = varCref.clone();
                move |__pe_a0| isPreviousOfCref(&__pe_a0, &__pe_b1)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<bool> + 'static,
                >);
            move |__pe_a0| Expression::contains(__pe_a0, &*__pe_b1)
        }),
    )?;
    Ok(found)
}

fn isPreviousOfCref(
    mut e: &metamodelica::Ref<Expression::NFExpression>,
    mut varCref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<bool> {
    let mut res: bool;
    let mut expCall: metamodelica::Ref<Call::NFCall>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut argCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    res = (match &**e {
        Expression::CALL { call: expCall }
            if (stringEq(
                &(Call::functionNameLast(metamodelica::AsArg::as_arg(&expCall))?),
                &(literal!("previous")),
            )) =>
        {
            args = Call::arguments(metamodelica::AsArg::as_arg(&expCall))?;
            res = false;
            if ((args).len() as i32) == 1 {
                res = (match &*((args).head().cloned()?) {
                    Expression::CREF {
                        cref: __esc_argCref, ..
                    } => {
                        argCref = (*__esc_argCref).clone();
                        ComponentRef::isEqual(metamodelica::AsArg::as_arg(&argCref), varCref)?
                    }
                    _ => false,
                });
            }
            res
        }
        _ => false,
    });
    Ok(res)
}

fn getDefaultStart(mut ty: &metamodelica::Ref<Type::NFType>) -> metamodelica::Ref<Expression::NFExpression> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (match &**ty {
        Type::INTEGER => metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
        Type::REAL => metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: metamodelica::OrderedFloat(0.0_f64),
        }),
        Type::BOOLEAN => metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }),
        Type::STRING => metamodelica::Ref::new(Expression::NFExpression::STRING { value: literal!("") }),
        _ => metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
    });
    result
}

// ============================================================
// basicFlatSmSemantics
// ============================================================
fn basicFlatSmSemantics(
    mut initStateCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut stateCrefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut transitionEqs: &metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<FlatSmSemantics> {
    let mut sem: FlatSmSemantics;
    let mut preRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut nStates: i32;
    let mut nTransitions: i32;
    let mut i: i32;
    let mut t: metamodelica::List<Transition>;
    let mut cExps: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
    let mut knowns: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
    let mut eqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    let mut nStatesRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut activeRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut resetRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut selectedStateRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut selectedResetRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut firedRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut activeStateRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut activeResetRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut nextStateRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut nextResetRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut stateMachineInFinalStateRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut tArrayBool: metamodelica::Ref<Type::NFType>;
    let mut tArrayInt: metamodelica::Ref<Type::NFType>;
    let mut activeResetStatesRefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
        metamodelica::nil();
    let mut nextResetStatesRefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
        metamodelica::nil();
    let mut finalStatesRefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
    let mut cRefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
    let mut cImmediateRefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
    let mut tTArrayBool: metamodelica::Ref<Type::NFType>;
    let mut tTArrayInt: metamodelica::Ref<Type::NFType>;
    let mut tFromRefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
    let mut tToRefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
    let mut tImmediateRefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
    let mut tResetRefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
    let mut tSynchronizeRefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
    let mut tPriorityRefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
    let mut rhs: metamodelica::Ref<Expression::NFExpression>;
    let mut expCond: metamodelica::Ref<Expression::NFExpression>;
    let mut expThen: metamodelica::Ref<Expression::NFExpression>;
    let mut expElse: metamodelica::Ref<Expression::NFExpression>;
    let mut exp1: metamodelica::Ref<Expression::NFExpression>;
    let mut exp2: metamodelica::Ref<Expression::NFExpression>;
    let mut expIf: metamodelica::Ref<Expression::NFExpression>;
    let mut expLst: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut immediateVal: bool;
    let mut tDim: metamodelica::Ref<Dimension::NFDimension>;
    let mut nStatesDim: metamodelica::Ref<Dimension::NFDimension>;
    preRef = makeSMSPrefix(initStateCref.clone())?;
    (t, cExps) = createTandC(&stateCrefs, transitionEqs)?;
    nStates = ((stateCrefs).len() as i32);
    nTransitions = ((t).len() as i32);
    tDim = metamodelica::Ref::new(Dimension::NFDimension::INTEGER {
        size: nTransitions,
        var: Variability::STRUCTURAL_PARAMETER.clone(),
    });
    nStatesDim = metamodelica::Ref::new(Dimension::NFDimension::INTEGER {
        size: nStates,
        var: Variability::STRUCTURAL_PARAMETER.clone(),
    });
    tTArrayBool = metamodelica::Ref::new(Type::NFType::ARRAY {
        elementType: crate::NFType::interned_BOOLEAN(),
        dimensions: list![tDim.clone()],
    });
    tTArrayInt = metamodelica::Ref::new(Type::NFType::ARRAY {
        elementType: crate::NFType::interned_INTEGER(),
        dimensions: list![tDim],
    });
    tArrayBool = metamodelica::Ref::new(Type::NFType::ARRAY {
        elementType: crate::NFType::interned_BOOLEAN(),
        dimensions: list![nStatesDim.clone()],
    });
    tArrayInt = metamodelica::Ref::new(Type::NFType::ARRAY {
        elementType: crate::NFType::interned_INTEGER(),
        dimensions: list![nStatesDim],
    });
    nStatesRef = qCref(
        literal!("nState"),
        crate::NFType::interned_INTEGER(),
        metamodelica::nil(),
        preRef.clone(),
    )?;
    knowns = metamodelica::cons(
        makeVarWithBinding(
            nStatesRef,
            crate::NFType::interned_INTEGER(),
            Variability::STRUCTURAL_PARAMETER.clone(),
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: nStates }),
        ),
        knowns,
    );
    i = 0;
    for mut tr in &*t {
        i = i + 1;
        tFromRefs = metamodelica::cons(
            qCref(
                literal!("tFrom"),
                tTArrayInt.clone(),
                list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                    index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i })
                })],
                preRef.clone(),
            )?,
            tFromRefs,
        );
        knowns = metamodelica::cons(
            makeVarWithBinding(
                (tFromRefs).head().cloned()?,
                crate::NFType::interned_INTEGER(),
                Variability::STRUCTURAL_PARAMETER.clone(),
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: tr.from.clone() }),
            ),
            knowns,
        );
        tToRefs = metamodelica::cons(
            qCref(
                literal!("tTo"),
                tTArrayInt.clone(),
                list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                    index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i })
                })],
                preRef.clone(),
            )?,
            tToRefs,
        );
        knowns = metamodelica::cons(
            makeVarWithBinding(
                (tToRefs).head().cloned()?,
                crate::NFType::interned_INTEGER(),
                Variability::STRUCTURAL_PARAMETER.clone(),
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: tr.to.clone() }),
            ),
            knowns,
        );
        tImmediateRefs = metamodelica::cons(
            qCref(
                literal!("tImmediate"),
                tTArrayBool.clone(),
                list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                    index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i })
                })],
                preRef.clone(),
            )?,
            tImmediateRefs,
        );
        knowns = metamodelica::cons(
            makeVarWithBinding(
                (tImmediateRefs).head().cloned()?,
                crate::NFType::interned_BOOLEAN(),
                Variability::STRUCTURAL_PARAMETER.clone(),
                metamodelica::Ref::new(Expression::NFExpression::BOOLEAN {
                    value: tr.immediate.clone(),
                }),
            ),
            knowns,
        );
        tResetRefs = metamodelica::cons(
            qCref(
                literal!("tReset"),
                tTArrayBool.clone(),
                list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                    index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i })
                })],
                preRef.clone(),
            )?,
            tResetRefs,
        );
        knowns = metamodelica::cons(
            makeVarWithBinding(
                (tResetRefs).head().cloned()?,
                crate::NFType::interned_BOOLEAN(),
                Variability::STRUCTURAL_PARAMETER.clone(),
                metamodelica::Ref::new(Expression::NFExpression::BOOLEAN {
                    value: tr.reset.clone(),
                }),
            ),
            knowns,
        );
        tSynchronizeRefs = metamodelica::cons(
            qCref(
                literal!("tSynchronize"),
                tTArrayBool.clone(),
                list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                    index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i })
                })],
                preRef.clone(),
            )?,
            tSynchronizeRefs,
        );
        knowns = metamodelica::cons(
            makeVarWithBinding(
                (tSynchronizeRefs).head().cloned()?,
                crate::NFType::interned_BOOLEAN(),
                Variability::STRUCTURAL_PARAMETER.clone(),
                metamodelica::Ref::new(Expression::NFExpression::BOOLEAN {
                    value: tr.synchronize.clone(),
                }),
            ),
            knowns,
        );
        tPriorityRefs = metamodelica::cons(
            qCref(
                literal!("tPriority"),
                tTArrayInt.clone(),
                list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                    index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i })
                })],
                preRef.clone(),
            )?,
            tPriorityRefs,
        );
        knowns = metamodelica::cons(
            makeVarWithBinding(
                (tPriorityRefs).head().cloned()?,
                crate::NFType::interned_INTEGER(),
                Variability::STRUCTURAL_PARAMETER.clone(),
                metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                    value: tr.priority.clone(),
                }),
            ),
            knowns,
        );
    }
    tFromRefs = tFromRefs.reverse();
    tToRefs = tToRefs.reverse();
    tImmediateRefs = tImmediateRefs.reverse();
    tResetRefs = tResetRefs.reverse();
    tSynchronizeRefs = tSynchronizeRefs.reverse();
    tPriorityRefs = tPriorityRefs.reverse();
    i = 0;
    for mut cExp in &*cExps {
        i = i + 1;
        cImmediateRefs = metamodelica::cons(
            qCref(
                literal!("cImmediate"),
                tTArrayBool.clone(),
                list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                    index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i })
                })],
                preRef.clone(),
            )?,
            cImmediateRefs,
        );
        cRefs = metamodelica::cons(
            qCref(
                literal!("c"),
                tTArrayBool.clone(),
                list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                    index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i })
                })],
                preRef.clone(),
            )?,
            cRefs,
        );
        vars = metamodelica::cons(
            makeVarWithStart(
                (cImmediateRefs).head().cloned()?,
                crate::NFType::interned_BOOLEAN(),
                Variability::DISCRETE.clone(),
                metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }),
            ),
            vars,
        );
        vars = metamodelica::cons(
            makeVar(
                (cRefs).head().cloned()?,
                crate::NFType::interned_BOOLEAN(),
                Variability::DISCRETE.clone(),
            ),
            vars,
        );
    }
    cImmediateRefs = cImmediateRefs.reverse();
    cRefs = cRefs.reverse();
    activeRef = qCref(
        literal!("active"),
        crate::NFType::interned_BOOLEAN(),
        metamodelica::nil(),
        preRef.clone(),
    )?;
    vars = metamodelica::cons(
        makeVar(
            activeRef.clone(),
            crate::NFType::interned_BOOLEAN(),
            Variability::DISCRETE.clone(),
        ),
        vars,
    );
    resetRef = qCref(
        literal!("reset"),
        crate::NFType::interned_BOOLEAN(),
        metamodelica::nil(),
        preRef.clone(),
    )?;
    vars = metamodelica::cons(
        makeVar(
            resetRef.clone(),
            crate::NFType::interned_BOOLEAN(),
            Variability::DISCRETE.clone(),
        ),
        vars,
    );
    selectedStateRef = qCref(
        literal!("selectedState"),
        crate::NFType::interned_INTEGER(),
        metamodelica::nil(),
        preRef.clone(),
    )?;
    vars = metamodelica::cons(
        makeVar(
            selectedStateRef.clone(),
            crate::NFType::interned_INTEGER(),
            Variability::DISCRETE.clone(),
        ),
        vars,
    );
    selectedResetRef = qCref(
        literal!("selectedReset"),
        crate::NFType::interned_BOOLEAN(),
        metamodelica::nil(),
        preRef.clone(),
    )?;
    vars = metamodelica::cons(
        makeVar(
            selectedResetRef.clone(),
            crate::NFType::interned_BOOLEAN(),
            Variability::DISCRETE.clone(),
        ),
        vars,
    );
    firedRef = qCref(
        literal!("fired"),
        crate::NFType::interned_INTEGER(),
        metamodelica::nil(),
        preRef.clone(),
    )?;
    vars = metamodelica::cons(
        makeVar(
            firedRef.clone(),
            crate::NFType::interned_INTEGER(),
            Variability::DISCRETE.clone(),
        ),
        vars,
    );
    activeStateRef = qCref(
        literal!("activeState"),
        crate::NFType::interned_INTEGER(),
        metamodelica::nil(),
        preRef.clone(),
    )?;
    vars = metamodelica::cons(
        makeVar(
            activeStateRef.clone(),
            crate::NFType::interned_INTEGER(),
            Variability::DISCRETE.clone(),
        ),
        vars,
    );
    activeResetRef = qCref(
        literal!("activeReset"),
        crate::NFType::interned_BOOLEAN(),
        metamodelica::nil(),
        preRef.clone(),
    )?;
    vars = metamodelica::cons(
        makeVar(
            activeResetRef.clone(),
            crate::NFType::interned_BOOLEAN(),
            Variability::DISCRETE.clone(),
        ),
        vars,
    );
    nextStateRef = qCref(
        literal!("nextState"),
        crate::NFType::interned_INTEGER(),
        metamodelica::nil(),
        preRef.clone(),
    )?;
    vars = metamodelica::cons(
        makeVarWithStart(
            nextStateRef.clone(),
            crate::NFType::interned_INTEGER(),
            Variability::DISCRETE.clone(),
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
        ),
        vars,
    );
    nextResetRef = qCref(
        literal!("nextReset"),
        crate::NFType::interned_BOOLEAN(),
        metamodelica::nil(),
        preRef.clone(),
    )?;
    vars = metamodelica::cons(
        makeVarWithStart(
            nextResetRef.clone(),
            crate::NFType::interned_BOOLEAN(),
            Variability::DISCRETE.clone(),
            metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }),
        ),
        vars,
    );
    for mut j in 1..=nStates {
        activeResetStatesRefs = metamodelica::cons(
            qCref(
                literal!("activeResetStates"),
                tArrayBool.clone(),
                list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                    index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: j })
                })],
                preRef.clone(),
            )?,
            activeResetStatesRefs,
        );
        vars = metamodelica::cons(
            makeVar(
                (activeResetStatesRefs).head().cloned()?,
                crate::NFType::interned_BOOLEAN(),
                Variability::DISCRETE.clone(),
            ),
            vars,
        );
        nextResetStatesRefs = metamodelica::cons(
            qCref(
                literal!("nextResetStates"),
                tArrayBool.clone(),
                list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                    index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: j })
                })],
                preRef.clone(),
            )?,
            nextResetStatesRefs,
        );
        vars = metamodelica::cons(
            makeVarWithStart(
                (nextResetStatesRefs).head().cloned()?,
                crate::NFType::interned_BOOLEAN(),
                Variability::DISCRETE.clone(),
                metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }),
            ),
            vars,
        );
        finalStatesRefs = metamodelica::cons(
            qCref(
                literal!("finalStates"),
                tArrayBool.clone(),
                list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                    index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: j })
                })],
                preRef.clone(),
            )?,
            finalStatesRefs,
        );
        vars = metamodelica::cons(
            makeVar(
                (finalStatesRefs).head().cloned()?,
                crate::NFType::interned_BOOLEAN(),
                Variability::DISCRETE.clone(),
            ),
            vars,
        );
    }
    activeResetStatesRefs = activeResetStatesRefs.reverse();
    nextResetStatesRefs = nextResetStatesRefs.reverse();
    finalStatesRefs = finalStatesRefs.reverse();
    stateMachineInFinalStateRef = qCref(
        literal!("stateMachineInFinalState"),
        crate::NFType::interned_BOOLEAN(),
        metamodelica::nil(),
        preRef.clone(),
    )?;
    vars = metamodelica::cons(
        makeVar(
            stateMachineInFinalStateRef.clone(),
            crate::NFType::interned_BOOLEAN(),
            Variability::DISCRETE.clone(),
        ),
        vars,
    );
    i = 0;
    for mut cExp in &*cExps {
        i = i + 1;
        eqs = metamodelica::cons(
            makeEq(
                makeCrefExp((cImmediateRefs).get(i)?, crate::NFType::interned_BOOLEAN()),
                cExp.clone(),
                crate::NFType::interned_BOOLEAN(),
            ),
            eqs,
        );
        let Transition { immediate: __pa0, .. } = (t).get(i)?;
        immediateVal = metamodelica::Own::own(__pa0);
        rhs = if (immediateVal) {
            makeCrefExp((cImmediateRefs).get(i)?, crate::NFType::interned_BOOLEAN())
        } else {
            makePreviousCall(
                makeCrefExp((cImmediateRefs).get(i)?, crate::NFType::interned_BOOLEAN()),
                crate::NFType::interned_BOOLEAN(),
            )
        };
        eqs = metamodelica::cons(
            makeEq(
                makeCrefExp((cRefs).get(i)?, crate::NFType::interned_BOOLEAN()),
                rhs,
                crate::NFType::interned_BOOLEAN(),
            ),
            eqs,
        );
    }
    eqs = metamodelica::cons(
        makeEq(
            makeCrefExp(selectedStateRef.clone(), crate::NFType::interned_INTEGER()),
            makeIfExp(
                makeCrefExp(resetRef.clone(), crate::NFType::interned_BOOLEAN()),
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }),
                makePreviousCall(
                    makeCrefExp(nextStateRef.clone(), crate::NFType::interned_INTEGER()),
                    crate::NFType::interned_INTEGER(),
                ),
                crate::NFType::interned_INTEGER(),
            ),
            crate::NFType::interned_INTEGER(),
        ),
        eqs,
    );
    eqs = metamodelica::cons(
        makeEq(
            makeCrefExp(selectedResetRef.clone(), crate::NFType::interned_BOOLEAN()),
            makeIfExp(
                makeCrefExp(resetRef.clone(), crate::NFType::interned_BOOLEAN()),
                metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true }),
                makePreviousCall(
                    makeCrefExp(nextResetRef.clone(), crate::NFType::interned_BOOLEAN()),
                    crate::NFType::interned_BOOLEAN(),
                ),
                crate::NFType::interned_BOOLEAN(),
            ),
            crate::NFType::interned_BOOLEAN(),
        ),
        eqs,
    );
    expLst = metamodelica::nil();
    for mut j in 1..=nTransitions {
        expCond = makeRelationEq(
            makeCrefExp((tFromRefs).get(j)?, crate::NFType::interned_INTEGER()),
            makeCrefExp(selectedStateRef.clone(), crate::NFType::interned_INTEGER()),
            crate::NFType::interned_INTEGER(),
        );
        expIf = makeIfExp(
            expCond,
            makeCrefExp((cRefs).get(j)?, crate::NFType::interned_BOOLEAN()),
            metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }),
            crate::NFType::interned_BOOLEAN(),
        );
        expLst = metamodelica::cons(
            makeIfExp(
                expIf,
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: j }),
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
                crate::NFType::interned_INTEGER(),
            ),
            expLst,
        );
    }
    expLst = expLst.reverse();
    rhs = if (((expLst).len() as i32) > 1) {
        makeMaxIntArrCall(expLst)
    } else if (((expLst).len() as i32) == 1) {
        (expLst).head().cloned()?
    } else {
        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 })
    };
    eqs = metamodelica::cons(
        makeEq(
            makeCrefExp(firedRef.clone(), crate::NFType::interned_INTEGER()),
            rhs,
            crate::NFType::interned_INTEGER(),
        ),
        eqs,
    );
    exp1 = makeRelationGt(
        makeCrefExp(firedRef.clone(), crate::NFType::interned_INTEGER()),
        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
        crate::NFType::interned_INTEGER(),
    );
    exp2 = makeCrefExp(
        qCref(
            literal!("tTo"),
            tTArrayInt,
            list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                index: makeCrefExp(firedRef.clone(), crate::NFType::interned_INTEGER())
            })],
            preRef.clone(),
        )?,
        crate::NFType::interned_INTEGER(),
    );
    expElse = makeIfExp(
        exp1,
        exp2,
        makeCrefExp(selectedStateRef, crate::NFType::interned_INTEGER()),
        crate::NFType::interned_INTEGER(),
    );
    eqs = metamodelica::cons(
        makeEq(
            makeCrefExp(activeStateRef.clone(), crate::NFType::interned_INTEGER()),
            makeIfExp(
                makeCrefExp(resetRef.clone(), crate::NFType::interned_BOOLEAN()),
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }),
                expElse,
                crate::NFType::interned_INTEGER(),
            ),
            crate::NFType::interned_INTEGER(),
        ),
        eqs,
    );
    exp1 = makeRelationGt(
        makeCrefExp(firedRef.clone(), crate::NFType::interned_INTEGER()),
        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
        crate::NFType::interned_INTEGER(),
    );
    exp2 = makeCrefExp(
        qCref(
            literal!("tReset"),
            tTArrayBool,
            list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                index: makeCrefExp(firedRef, crate::NFType::interned_INTEGER())
            })],
            preRef.clone(),
        )?,
        crate::NFType::interned_BOOLEAN(),
    );
    expElse = makeIfExp(
        exp1,
        exp2,
        makeCrefExp(selectedResetRef, crate::NFType::interned_BOOLEAN()),
        crate::NFType::interned_BOOLEAN(),
    );
    eqs = metamodelica::cons(
        makeEq(
            makeCrefExp(activeResetRef, crate::NFType::interned_BOOLEAN()),
            makeIfExp(
                makeCrefExp(resetRef.clone(), crate::NFType::interned_BOOLEAN()),
                metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true }),
                expElse,
                crate::NFType::interned_BOOLEAN(),
            ),
            crate::NFType::interned_BOOLEAN(),
        ),
        eqs,
    );
    eqs = metamodelica::cons(
        makeEq(
            makeCrefExp(nextStateRef.clone(), crate::NFType::interned_INTEGER()),
            makeIfExp(
                makeCrefExp(activeRef.clone(), crate::NFType::interned_BOOLEAN()),
                makeCrefExp(activeStateRef.clone(), crate::NFType::interned_INTEGER()),
                makePreviousCall(
                    makeCrefExp(nextStateRef, crate::NFType::interned_INTEGER()),
                    crate::NFType::interned_INTEGER(),
                ),
                crate::NFType::interned_INTEGER(),
            ),
            crate::NFType::interned_INTEGER(),
        ),
        eqs,
    );
    eqs = metamodelica::cons(
        makeEq(
            makeCrefExp(nextResetRef.clone(), crate::NFType::interned_BOOLEAN()),
            makeIfExp(
                makeCrefExp(activeRef.clone(), crate::NFType::interned_BOOLEAN()),
                metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }),
                makePreviousCall(
                    makeCrefExp(nextResetRef, crate::NFType::interned_BOOLEAN()),
                    crate::NFType::interned_BOOLEAN(),
                ),
                crate::NFType::interned_BOOLEAN(),
            ),
            crate::NFType::interned_BOOLEAN(),
        ),
        eqs,
    );
    for mut j in 1..=nStates {
        eqs = metamodelica::cons(
            makeEq(
                makeCrefExp((activeResetStatesRefs).get(j)?, crate::NFType::interned_BOOLEAN()),
                makeIfExp(
                    makeCrefExp(resetRef.clone(), crate::NFType::interned_BOOLEAN()),
                    metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true }),
                    makePreviousCall(
                        makeCrefExp((nextResetStatesRefs).get(j)?, crate::NFType::interned_BOOLEAN()),
                        crate::NFType::interned_BOOLEAN(),
                    ),
                    crate::NFType::interned_BOOLEAN(),
                ),
                crate::NFType::interned_BOOLEAN(),
            ),
            eqs,
        );
    }
    for mut j in 1..=nStates {
        exp1 = makeRelationEq(
            makeCrefExp(activeStateRef.clone(), crate::NFType::interned_INTEGER()),
            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: j }),
            crate::NFType::interned_INTEGER(),
        );
        expThen = makeIfExp(
            exp1,
            metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }),
            makeCrefExp((activeResetStatesRefs).get(j)?, crate::NFType::interned_BOOLEAN()),
            crate::NFType::interned_BOOLEAN(),
        );
        expElse = makePreviousCall(
            makeCrefExp((nextResetStatesRefs).get(j)?, crate::NFType::interned_BOOLEAN()),
            crate::NFType::interned_BOOLEAN(),
        );
        eqs = metamodelica::cons(
            makeEq(
                makeCrefExp((nextResetStatesRefs).get(j)?, crate::NFType::interned_BOOLEAN()),
                makeIfExp(
                    makeCrefExp(activeRef.clone(), crate::NFType::interned_BOOLEAN()),
                    expThen,
                    expElse,
                    crate::NFType::interned_BOOLEAN(),
                ),
                crate::NFType::interned_BOOLEAN(),
            ),
            eqs,
        );
    }
    for mut j in 1..=nStates {
        expLst = metamodelica::nil();
        for mut k in 1..=nTransitions {
            expCond = makeRelationEq(
                makeCrefExp((tFromRefs).get(k)?, crate::NFType::interned_INTEGER()),
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: j }),
                crate::NFType::interned_INTEGER(),
            );
            expLst = metamodelica::cons(
                makeIfExp(
                    expCond,
                    metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }),
                    metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
                    crate::NFType::interned_INTEGER(),
                ),
                expLst,
            );
        }
        expLst = expLst.reverse();
        rhs = if (((expLst).len() as i32) > 1) {
            makeRelationEq(
                makeMaxIntArrCall(expLst),
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
                crate::NFType::interned_INTEGER(),
            )
        } else if (((expLst).len() as i32) == 1) {
            makeRelationEq(
                (expLst).head().cloned()?,
                metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
                crate::NFType::interned_INTEGER(),
            )
        } else {
            metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true })
        };
        eqs = metamodelica::cons(
            makeEq(
                makeCrefExp((finalStatesRefs).get(j)?, crate::NFType::interned_BOOLEAN()),
                rhs,
                crate::NFType::interned_BOOLEAN(),
            ),
            eqs,
        );
    }
    eqs = metamodelica::cons(
        makeEq(
            makeCrefExp(stateMachineInFinalStateRef, crate::NFType::interned_BOOLEAN()),
            makeCrefExp(
                qCref(
                    literal!("finalStates"),
                    tArrayBool,
                    list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                        index: makeCrefExp(activeStateRef, crate::NFType::interned_INTEGER())
                    })],
                    preRef,
                )?,
                crate::NFType::interned_BOOLEAN(),
            ),
            crate::NFType::interned_BOOLEAN(),
        ),
        eqs,
    );
    sem = FlatSmSemantics {
        initStateRef: initStateCref,
        smComps: metamodelica::arrayFromVec(stateCrefs.into_iter().cloned().collect()),
        t: t,
        c: cExps,
        vars: vars,
        knowns: knowns,
        eqs: eqs,
        pvars: metamodelica::nil(),
        peqs: metamodelica::nil(),
        enclosingState: None,
    };
    Ok(sem)
}

// ============================================================
// addPropagationEquations
// ============================================================
fn addPropagationEquations(
    mut inSem: FlatSmSemantics,
    mut enclosingStateCrefOpt: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut enclosingSmSemOpt: Option<FlatSmSemantics>,
) -> Result<FlatSmSemantics> {
    let mut outSem: FlatSmSemantics = inSem.clone();
    let mut preRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut initStateRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut activeRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut resetRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut initRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut pvars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
    let mut peqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    let mut nStates: i32;
    let mut posOfEnclosing: i32;
    let mut tArrayBool: metamodelica::Ref<Type::NFType>;
    let mut enclosingStateCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut enclosingPreRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut enclosingActiveResetStateRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut enclosingActiveResetRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut enclosingActiveStateRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut enclosingInitStateRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut enclosingSem: FlatSmSemantics;
    let mut enclosingComps: metamodelica::Array<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut stateRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut activePlotRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut activePlotVar: metamodelica::Ref<Variable::NFVariable>;
    let mut ticksVar: metamodelica::Ref<Variable::NFVariable>;
    let mut timeEnteredVar: metamodelica::Ref<Variable::NFVariable>;
    let mut timeInVar: metamodelica::Ref<Variable::NFVariable>;
    let mut activePlotEq: metamodelica::Ref<Equation::NFEquation>;
    let mut ticksEq: metamodelica::Ref<Equation::NFEquation>;
    let mut timeEnteredEq: metamodelica::Ref<Equation::NFEquation>;
    let mut timeInEq: metamodelica::Ref<Equation::NFEquation>;
    initStateRef = inSem.initStateRef.clone();
    preRef = makeSMSPrefix(initStateRef)?;
    activeRef = qCref(
        literal!("active"),
        crate::NFType::interned_BOOLEAN(),
        metamodelica::nil(),
        preRef.clone(),
    )?;
    resetRef = qCref(
        literal!("reset"),
        crate::NFType::interned_BOOLEAN(),
        metamodelica::nil(),
        preRef.clone(),
    )?;
    nStates = metamodelica::arrayLength(inSem.smComps.clone());
    tArrayBool = metamodelica::Ref::new(Type::NFType::ARRAY {
        elementType: crate::NFType::interned_BOOLEAN(),
        dimensions: list![metamodelica::Ref::new(Dimension::NFDimension::INTEGER {
            size: nStates,
            var: Variability::STRUCTURAL_PARAMETER.clone()
        })],
    });
    if (enclosingSmSemOpt).is_none() {
        initRef = qCref(
            literal!("init"),
            crate::NFType::interned_BOOLEAN(),
            metamodelica::nil(),
            preRef.clone(),
        )?;
        pvars = metamodelica::cons(
            makeVarWithStart(
                initRef.clone(),
                crate::NFType::interned_BOOLEAN(),
                Variability::DISCRETE.clone(),
                metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true }),
            ),
            pvars,
        );
        peqs = metamodelica::cons(
            makeEq(
                makeCrefExp(initRef.clone(), crate::NFType::interned_BOOLEAN()),
                metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }),
                crate::NFType::interned_BOOLEAN(),
            ),
            peqs,
        );
        peqs = metamodelica::cons(
            makeEq(
                makeCrefExp(resetRef, crate::NFType::interned_BOOLEAN()),
                makePreviousCall(
                    makeCrefExp(initRef, crate::NFType::interned_BOOLEAN()),
                    crate::NFType::interned_BOOLEAN(),
                ),
                crate::NFType::interned_BOOLEAN(),
            ),
            peqs,
        );
        peqs = metamodelica::cons(
            makeEq(
                makeCrefExp(activeRef, crate::NFType::interned_BOOLEAN()),
                metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true }),
                crate::NFType::interned_BOOLEAN(),
            ),
            peqs,
        );
    } else {
        let __pa0 = ::match_deref::match_deref! { match &(enclosingStateCrefOpt.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        enclosingStateCref = metamodelica::Own::own(__pa0);
        let __pa1 = ::match_deref::match_deref! { match &(enclosingSmSemOpt) {
            Some(__pa1) => __pa1.clone(),
            _ => return Err("pattern mismatch"),
        } };
        enclosingSem = metamodelica::Own::own(__pa1);
        enclosingComps = enclosingSem.smComps.clone();
        enclosingInitStateRef = metamodelica::arrayGet(enclosingComps.clone(), 1)?;
        enclosingPreRef = makeSMSPrefix(enclosingInitStateRef)?;
        posOfEnclosing = 1;
        let __range2 = &*enclosingComps
            .clone()
            .borrow()
            .iter()
            .cloned()
            .collect::<metamodelica::List<_>>();
        for mut sc in __range2 {
            if ComponentRef::isEqual(metamodelica::AsArg::as_arg(&sc), &enclosingStateCref)? {
                break;
            }
            posOfEnclosing = posOfEnclosing + 1;
        }
        enclosingActiveStateRef = qCref(
            literal!("activeState"),
            crate::NFType::interned_INTEGER(),
            metamodelica::nil(),
            enclosingPreRef.clone(),
        )?;
        enclosingActiveResetRef = qCref(
            literal!("activeReset"),
            crate::NFType::interned_BOOLEAN(),
            metamodelica::nil(),
            enclosingPreRef.clone(),
        )?;
        enclosingActiveResetStateRef = qCref(
            literal!("activeResetStates"),
            tArrayBool,
            list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: posOfEnclosing })
            })],
            enclosingPreRef,
        )?;
        peqs = metamodelica::cons(
            makeEq(
                makeCrefExp(resetRef, crate::NFType::interned_BOOLEAN()),
                metamodelica::Ref::new(Expression::NFExpression::LBINARY {
                    exp1: makeCrefExp(enclosingActiveResetStateRef, crate::NFType::interned_BOOLEAN()),
                    operator: Operator::makeOr(crate::NFType::interned_BOOLEAN()),
                    exp2: metamodelica::Ref::new(Expression::NFExpression::LBINARY {
                        exp1: makeCrefExp(enclosingActiveResetRef, crate::NFType::interned_BOOLEAN()),
                        operator: Operator::makeAnd(crate::NFType::interned_BOOLEAN()),
                        exp2: makeRelationEq(
                            makeCrefExp(enclosingActiveStateRef.clone(), crate::NFType::interned_INTEGER()),
                            metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: posOfEnclosing }),
                            crate::NFType::interned_INTEGER(),
                        ),
                    }),
                }),
                crate::NFType::interned_BOOLEAN(),
            ),
            peqs,
        );
        peqs = metamodelica::cons(
            makeEq(
                makeCrefExp(activeRef, crate::NFType::interned_BOOLEAN()),
                makeRelationEq(
                    makeCrefExp(enclosingActiveStateRef, crate::NFType::interned_INTEGER()),
                    metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: posOfEnclosing }),
                    crate::NFType::interned_INTEGER(),
                ),
                crate::NFType::interned_BOOLEAN(),
            ),
            peqs,
        );
    }
    for mut j in 1..=nStates {
        stateRef = metamodelica::arrayGet(inSem.smComps.clone(), j)?;
        (activePlotVar, activePlotEq) = createActiveIndicator(stateRef.clone(), preRef.clone(), j)?;
        pvars = metamodelica::cons(activePlotVar.clone(), pvars);
        peqs = metamodelica::cons(activePlotEq, peqs);
        activePlotRef = activePlotVar.name.clone();
        (ticksVar, ticksEq) = createTicksInStateIndicator(stateRef.clone(), activePlotRef.clone())?;
        pvars = metamodelica::cons(ticksVar, pvars);
        peqs = metamodelica::cons(ticksEq, peqs);
        (timeEnteredVar, timeEnteredEq) = createTimeEnteredStateIndicator(stateRef.clone(), activePlotRef.clone())?;
        (timeInVar, timeInEq) = createTimeInStateIndicator(stateRef, activePlotRef, &timeEnteredVar)?;
        pvars = metamodelica::cons(timeEnteredVar, metamodelica::cons(timeInVar, pvars));
        peqs = metamodelica::cons(timeEnteredEq, metamodelica::cons(timeInEq, peqs));
    }
    outSem.pvars = pvars;
    outSem.peqs = peqs;
    outSem.enclosingState = enclosingStateCrefOpt;
    Ok(outSem)
}

// ============================================================
// elabXInStateOps
// ============================================================
fn elabXInStateOps(
    mut sem: FlatSmSemantics,
    mut enclosingStateCrefOpt: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>,
) -> Result<FlatSmSemantics> {
    let mut sem: FlatSmSemantics = sem;
    let mut tElab: metamodelica::List<Transition> = metamodelica::nil();
    let mut cElab: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
    let mut i: i32;
    let mut stateRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut substTickExp: metamodelica::Ref<Expression::NFExpression>;
    let mut substTimeExp: metamodelica::Ref<Expression::NFExpression>;
    let mut c3: metamodelica::Ref<Expression::NFExpression>;
    let mut c4: metamodelica::Ref<Expression::NFExpression>;
    let mut found: bool;
    let mut curT: Transition;
    let mut curFrom: i32;
    let mut curTo: i32;
    let mut curPriority: i32;
    let mut curImmediate: bool;
    let mut curReset: bool;
    let mut curSynchronize: bool;
    i = 0;
    for mut tc in &*List::zip(sem.t.clone(), sem.c.clone()) {
        i = i + 1;
        (_, c3) = tc.clone();
        curT = (sem.t).get(i)?;
        let Transition {
            from: __pa0,
            to: __pa1,
            immediate: __pa2,
            reset: __pa3,
            synchronize: __pa4,
            priority: __pa5,
            ..
        } = curT;
        curFrom = metamodelica::Own::own(__pa0);
        curTo = metamodelica::Own::own(__pa1);
        curImmediate = metamodelica::Own::own(__pa2);
        curReset = metamodelica::Own::own(__pa3);
        curSynchronize = metamodelica::Own::own(__pa4);
        curPriority = metamodelica::Own::own(__pa5);
        stateRef = metamodelica::arrayGet(sem.smComps.clone(), curFrom)?;
        substTickExp = makeCrefExp(
            qCref(
                literal!("$ticksInState"),
                crate::NFType::interned_INTEGER(),
                metamodelica::nil(),
                stateRef.clone(),
            )?,
            crate::NFType::interned_INTEGER(),
        );
        (c4, found) = subsXInState(c3, &(literal!("ticksInState")), &substTickExp)?;
        if found && (enclosingStateCrefOpt).is_some() {
            Error::addCompilerError(literal!(
                "Found 'ticksInState()' within a state of a hierarchical state machine."
            ))?;
            return Err("fail");
        }
        if found {
            sem.eqs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
                for mut eq in (sem.eqs.clone()).into_iter().cloned() {
                    let __x = smeqsSubsXInState(
                        eq.clone(),
                        metamodelica::arrayGet(sem.smComps.clone(), 1)?,
                        i,
                        ((sem.t).len() as i32),
                        &substTickExp,
                        &(literal!("ticksInState")),
                    )?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
        }
        substTimeExp = makeCrefExp(
            qCref(
                literal!("$timeInState"),
                crate::NFType::interned_REAL(),
                metamodelica::nil(),
                stateRef,
            )?,
            crate::NFType::interned_REAL(),
        );
        (c4, found) = subsXInState(c4, &(literal!("timeInState")), &substTimeExp)?;
        if found && (enclosingStateCrefOpt).is_some() {
            Error::addCompilerError(literal!(
                "Found 'timeInState()' within a state of a hierarchical state machine."
            ))?;
            return Err("fail");
        }
        if found {
            sem.eqs = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
                for mut eq in (sem.eqs.clone()).into_iter().cloned() {
                    let __x = smeqsSubsXInState(
                        eq.clone(),
                        metamodelica::arrayGet(sem.smComps.clone(), 1)?,
                        i,
                        ((sem.t).len() as i32),
                        &substTimeExp,
                        &(literal!("timeInState")),
                    )?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
        }
        tElab = metamodelica::cons(
            Transition {
                from: curFrom,
                to: curTo,
                condition: c4.clone(),
                immediate: curImmediate,
                reset: curReset,
                synchronize: curSynchronize,
                priority: curPriority,
            },
            tElab,
        );
        cElab = metamodelica::cons(c4, cElab);
    }
    sem.t = tElab.reverse();
    sem.c = cElab.reverse();
    Ok(sem)
}

fn subsXInState(
    mut inExp: metamodelica::Ref<Expression::NFExpression>,
    mut funcName: &ArcStr,
    mut substExp: &metamodelica::Ref<Expression::NFExpression>,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut found: bool = false;
    (outExp, found) = Expression::mapFold(
        inExp,
        (std::sync::Arc::new({
            let __pe_b1 = funcName.clone();
            let __pe_b2 = substExp.clone();
            move |__pe_a0, __pe_a3| Ok(subsXInStateHelper(__pe_a0, &__pe_b1, __pe_b2.clone(), __pe_a3))
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                        bool,
                    ) -> Result<(metamodelica::Ref<Expression::NFExpression>, bool)>
                    + 'static,
            >),
        false,
    )?;
    Ok((outExp, found))
}

fn subsXInStateHelper(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut funcName: &ArcStr,
    mut substExp: metamodelica::Ref<Expression::NFExpression>,
    mut found: bool,
) -> (metamodelica::Ref<Expression::NFExpression>, bool) {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut found: bool = found;
    let mut expCall: metamodelica::Ref<Call::NFCall>;
    if '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ Expression::CALL { call: __pa1 } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        expCall = metamodelica::Own::own(__pa1);
        if !(stringEq(
            &(unwrap_break_err!(Call::functionNameLast(&expCall), '__try0)),
            &funcName,
        )) {
            break '__try0 Err::<_, _>("fail");
        }
        if !((unwrap_break_err!(Call::arguments(&expCall), '__try0)).is_empty()) {
            break '__try0 Err::<_, _>("fail");
        }
        exp = substExp.clone();
        found = true;
        Ok::<(), &'static str>(())
    }
    .is_err()
    {}
    (exp, found)
}

fn smeqsSubsXInState(
    mut eq: metamodelica::Ref<Equation::NFEquation>,
    mut initStateComp: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut i: i32,
    mut nTransitions: i32,
    mut substExp: &metamodelica::Ref<Expression::NFExpression>,
    mut xInState: &ArcStr,
) -> Result<metamodelica::Ref<Equation::NFEquation>> {
    let mut outEq: metamodelica::Ref<Equation::NFEquation> = eq.clone();
    let mut preRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut lhsRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut cRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut tArrayBool: metamodelica::Ref<Type::NFType>;
    let mut lhs: metamodelica::Ref<Expression::NFExpression>;
    let mut rhs: metamodelica::Ref<Expression::NFExpression>;
    let mut newRhs: metamodelica::Ref<Expression::NFExpression>;
    outEq = (match &*eq {
        Equation::EQUALITY {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            scope: __eq_scope,
            source: __eq_source,
            ty: __eq_ty,
            ..
        } => {
            preRef = makeSMSPrefix(initStateComp)?;
            tArrayBool = metamodelica::Ref::new(Type::NFType::ARRAY {
                elementType: crate::NFType::interned_BOOLEAN(),
                dimensions: list![metamodelica::Ref::new(Dimension::NFDimension::INTEGER {
                    size: nTransitions,
                    var: Variability::STRUCTURAL_PARAMETER.clone()
                })],
            });
            cRef = qCref(
                literal!("cImmediate"),
                tArrayBool,
                list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                    index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i })
                })],
                preRef,
            )?;
            let __pa0 = ::match_deref::match_deref! { match &(__eq_lhs.clone()) {
                Deref @ Expression::CREF { cref: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            lhsRef = metamodelica::Own::own(__pa0);
            if ComponentRef::isEqual(&cRef, &lhsRef)? {
                (newRhs, _) = subsXInState(__eq_rhs.clone(), xInState, substExp)?;
                outEq = metamodelica::Ref::new(Equation::NFEquation::EQUALITY {
                    lhs: __eq_lhs.clone(),
                    rhs: newRhs,
                    ty: __eq_ty.clone(),
                    scope: __eq_scope.clone(),
                    source: __eq_source.clone(),
                    scalarizeMode: ScalarizeMode::NO_PREFERENCE.clone(),
                });
            }
            outEq
        }
        _ => eq,
    });
    Ok(outEq)
}

// ============================================================
// State indicator helpers
// ============================================================
fn createActiveIndicator(
    mut stateRef: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut preRef: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut i: i32,
) -> Result<(
    metamodelica::Ref<Variable::NFVariable>,
    metamodelica::Ref<Equation::NFEquation>,
)> {
    let mut activePlotVar: metamodelica::Ref<Variable::NFVariable>;
    let mut eqn: metamodelica::Ref<Equation::NFEquation>;
    let mut activePlotRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut activeRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut activeStateRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut andExp: metamodelica::Ref<Expression::NFExpression>;
    let mut eqExp: metamodelica::Ref<Expression::NFExpression>;
    activePlotRef = qCref(
        literal!("active"),
        crate::NFType::interned_BOOLEAN(),
        metamodelica::nil(),
        stateRef,
    )?;
    activePlotVar = makeVarWithStart(
        activePlotRef.clone(),
        crate::NFType::interned_BOOLEAN(),
        Variability::DISCRETE.clone(),
        metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }),
    );
    activeRef = qCref(
        literal!("active"),
        crate::NFType::interned_BOOLEAN(),
        metamodelica::nil(),
        preRef.clone(),
    )?;
    activeStateRef = qCref(
        literal!("activeState"),
        crate::NFType::interned_INTEGER(),
        metamodelica::nil(),
        preRef,
    )?;
    eqExp = makeRelationEq(
        makeCrefExp(activeStateRef, crate::NFType::interned_INTEGER()),
        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i }),
        crate::NFType::interned_INTEGER(),
    );
    andExp = metamodelica::Ref::new(Expression::NFExpression::LBINARY {
        exp1: makeCrefExp(activeRef, crate::NFType::interned_BOOLEAN()),
        operator: Operator::makeAnd(crate::NFType::interned_BOOLEAN()),
        exp2: eqExp,
    });
    eqn = makeEq(
        makeCrefExp(activePlotRef, crate::NFType::interned_BOOLEAN()),
        andExp,
        crate::NFType::interned_BOOLEAN(),
    );
    Ok((activePlotVar, eqn))
}

fn createTicksInStateIndicator(
    mut stateRef: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut stateActiveRef: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<(
    metamodelica::Ref<Variable::NFVariable>,
    metamodelica::Ref<Equation::NFEquation>,
)> {
    let mut ticksVar: metamodelica::Ref<Variable::NFVariable>;
    let mut ticksEq: metamodelica::Ref<Equation::NFEquation>;
    let mut ticksRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut ticksExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expCond: metamodelica::Ref<Expression::NFExpression>;
    let mut expThen: metamodelica::Ref<Expression::NFExpression>;
    let mut expElse: metamodelica::Ref<Expression::NFExpression>;
    ticksRef = qCref(
        literal!("$ticksInState"),
        crate::NFType::interned_INTEGER(),
        metamodelica::nil(),
        stateRef,
    )?;
    ticksVar = makeVarWithStart(
        ticksRef.clone(),
        crate::NFType::interned_INTEGER(),
        Variability::DISCRETE.clone(),
        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
    );
    ticksExp = makeCrefExp(ticksRef, crate::NFType::interned_INTEGER());
    expCond = metamodelica::Ref::new(Expression::NFExpression::LUNARY {
        operator: Operator::makeNot(crate::NFType::interned_BOOLEAN()),
        exp: makeCrefExp(stateActiveRef, crate::NFType::interned_BOOLEAN()),
    });
    expThen = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 });
    expElse = metamodelica::Ref::new(Expression::NFExpression::BINARY {
        exp1: makePreviousCall(ticksExp.clone(), crate::NFType::interned_INTEGER()),
        operator: Operator::makeAdd(crate::NFType::interned_INTEGER()),
        exp2: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }),
    });
    ticksEq = makeEq(
        ticksExp,
        makeIfExp(expCond, expThen, expElse, crate::NFType::interned_INTEGER()),
        crate::NFType::interned_INTEGER(),
    );
    Ok((ticksVar, ticksEq))
}

fn createTimeEnteredStateIndicator(
    mut stateRef: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut stateActiveRef: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<(
    metamodelica::Ref<Variable::NFVariable>,
    metamodelica::Ref<Equation::NFEquation>,
)> {
    let mut timeEnteredVar: metamodelica::Ref<Variable::NFVariable>;
    let mut timeEnteredEq: metamodelica::Ref<Equation::NFEquation>;
    let mut timeEnteredRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut timeEnteredExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expCond: metamodelica::Ref<Expression::NFExpression>;
    let mut expThen: metamodelica::Ref<Expression::NFExpression>;
    let mut expElse: metamodelica::Ref<Expression::NFExpression>;
    let mut activeExp: metamodelica::Ref<Expression::NFExpression>;
    timeEnteredRef = qCref(
        literal!("$timeEnteredState"),
        crate::NFType::interned_REAL(),
        metamodelica::nil(),
        stateRef,
    )?;
    timeEnteredVar = makeVarWithStart(
        timeEnteredRef.clone(),
        crate::NFType::interned_REAL(),
        Variability::CONTINUOUS.clone(),
        metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: metamodelica::OrderedFloat(0.0_f64),
        }),
    );
    timeEnteredExp = makeCrefExp(timeEnteredRef, crate::NFType::interned_REAL());
    activeExp = makeCrefExp(stateActiveRef, crate::NFType::interned_BOOLEAN());
    expCond = metamodelica::Ref::new(Expression::NFExpression::LBINARY {
        exp1: makeRelationEq(
            makePreviousCall(activeExp.clone(), crate::NFType::interned_BOOLEAN()),
            metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }),
            crate::NFType::interned_BOOLEAN(),
        ),
        operator: Operator::makeAnd(crate::NFType::interned_BOOLEAN()),
        exp2: makeRelationEq(
            activeExp,
            metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true }),
            crate::NFType::interned_BOOLEAN(),
        ),
    });
    expThen = makeSampleTimeCall()?;
    expElse = makePreviousCall(timeEnteredExp.clone(), crate::NFType::interned_REAL());
    timeEnteredEq = makeEq(
        timeEnteredExp,
        makeIfExp(expCond, expThen, expElse, crate::NFType::interned_REAL()),
        crate::NFType::interned_REAL(),
    );
    Ok((timeEnteredVar, timeEnteredEq))
}

fn createTimeInStateIndicator(
    mut stateRef: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut stateActiveRef: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut timeEnteredVar: &metamodelica::Ref<Variable::NFVariable>,
) -> Result<(
    metamodelica::Ref<Variable::NFVariable>,
    metamodelica::Ref<Equation::NFEquation>,
)> {
    let mut timeInVar: metamodelica::Ref<Variable::NFVariable>;
    let mut timeInEq: metamodelica::Ref<Equation::NFEquation>;
    let mut timeInRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut timeInExp: metamodelica::Ref<Expression::NFExpression>;
    let mut expCond: metamodelica::Ref<Expression::NFExpression>;
    let mut expThen: metamodelica::Ref<Expression::NFExpression>;
    let mut expElse: metamodelica::Ref<Expression::NFExpression>;
    let mut timeEnteredExp: metamodelica::Ref<Expression::NFExpression>;
    timeInRef = qCref(
        literal!("$timeInState"),
        crate::NFType::interned_REAL(),
        metamodelica::nil(),
        stateRef,
    )?;
    timeInVar = makeVarWithStart(
        timeInRef.clone(),
        crate::NFType::interned_REAL(),
        Variability::CONTINUOUS.clone(),
        metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: metamodelica::OrderedFloat(0.0_f64),
        }),
    );
    timeInExp = makeCrefExp(timeInRef, crate::NFType::interned_REAL());
    timeEnteredExp = makeCrefExp(timeEnteredVar.name.clone(), crate::NFType::interned_REAL());
    expCond = makeCrefExp(stateActiveRef, crate::NFType::interned_BOOLEAN());
    expThen = metamodelica::Ref::new(Expression::NFExpression::BINARY {
        exp1: makeSampleTimeCall()?,
        operator: Operator::makeSub(crate::NFType::interned_REAL()),
        exp2: timeEnteredExp,
    });
    expElse = metamodelica::Ref::new(Expression::NFExpression::REAL {
        value: metamodelica::OrderedFloat(0.0_f64),
    });
    timeInEq = makeEq(
        timeInExp,
        makeIfExp(expCond, expThen, expElse, crate::NFType::interned_REAL()),
        crate::NFType::interned_REAL(),
    );
    Ok((timeInVar, timeInEq))
}

// ============================================================
// Reset and activation wrapping
// ============================================================
fn wrapInStateActivationConditional(
    mut inEq: &metamodelica::Ref<Equation::NFEquation>,
    mut stateCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut isResetEquation: bool,
) -> Result<metamodelica::Ref<Equation::NFEquation>> {
    let mut outEq: metamodelica::Ref<Equation::NFEquation>;
    let mut lhs: metamodelica::Ref<Expression::NFExpression>;
    let mut rhs: metamodelica::Ref<Expression::NFExpression>;
    let mut activeRef: metamodelica::Ref<Expression::NFExpression>;
    let mut expElse: metamodelica::Ref<Expression::NFExpression>;
    let mut lhsCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut eqScope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let mut eqSource: metamodelica::Ref<DAE::ElementSource>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &((*inEq)) {
        Deref @ Equation::EQUALITY { lhs: __pa0, rhs: __pa1, ty: __pa2, scope: __pa3, source: __pa4, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    lhs = metamodelica::Own::own(__pa0);
    rhs = metamodelica::Own::own(__pa1);
    ty = metamodelica::Own::own(__pa2);
    eqScope = metamodelica::Own::own(__pa3);
    eqSource = metamodelica::Own::own(__pa4);
    let (__pa5, __pa6) = ::match_deref::match_deref! { match &(lhs.clone()) {
        Deref @ Expression::CREF { ty: __pa5, cref: __pa6 } => (__pa5.clone(), __pa6.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa5);
    lhsCref = metamodelica::Own::own(__pa6);
    activeRef = makeCrefExp(
        qCref(
            literal!("active"),
            crate::NFType::interned_BOOLEAN(),
            metamodelica::nil(),
            stateCref,
        )?,
        crate::NFType::interned_BOOLEAN(),
    );
    if isResetEquation {
        expElse = makeCrefExp(
            ComponentRef::prefixCref(
                metamodelica::Ref::new(InstNode::InstNode::NAME_NODE {
                    name: {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*ComponentRef::firstName(&lhsCref, false)?);
                        __mm_s.push_str(&*literal!("_previous"));
                        ArcStr::from(__mm_s)
                    },
                }),
                ty.clone(),
                metamodelica::nil(),
                ComponentRef::rest(&lhsCref)?,
            )?,
            ty.clone(),
        );
    } else {
        expElse = makePreviousCall(lhs.clone(), ty.clone());
    }
    outEq = metamodelica::Ref::new(Equation::NFEquation::EQUALITY {
        lhs: lhs,
        rhs: makeIfExp(activeRef, rhs, expElse, ty.clone()),
        ty: ty,
        scope: eqScope,
        source: eqSource,
        scalarizeMode: ScalarizeMode::NO_PREFERENCE.clone(),
    });
    Ok(outEq)
}

fn createResetEquation(
    mut lhsCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut lhsTy: metamodelica::Ref<Type::NFType>,
    mut stateCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut sem: &FlatSmSemantics,
    mut crToStart: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
) -> Result<metamodelica::Ref<Equation::NFEquation>> {
    let mut outEq: metamodelica::Ref<Equation::NFEquation>;
    let mut preRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut initStateRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut activeExp: metamodelica::Ref<Expression::NFExpression>;
    let mut activeResetExp: metamodelica::Ref<Expression::NFExpression>;
    let mut activeResetStatesExp: metamodelica::Ref<Expression::NFExpression>;
    let mut orExp: metamodelica::Ref<Expression::NFExpression>;
    let mut andExp: metamodelica::Ref<Expression::NFExpression>;
    let mut prevExp: metamodelica::Ref<Expression::NFExpression>;
    let mut startExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ifExp: metamodelica::Ref<Expression::NFExpression>;
    let mut lhsPrevExp: metamodelica::Ref<Expression::NFExpression>;
    let mut i: i32;
    let mut nStates: i32;
    let mut tArrayBool: metamodelica::Ref<Type::NFType>;
    initStateRef = metamodelica::arrayGet(sem.smComps.clone(), 1)?;
    preRef = makeSMSPrefix(initStateRef)?;
    i = 1;
    let __range0 = &*sem
        .smComps
        .clone()
        .borrow()
        .iter()
        .cloned()
        .collect::<metamodelica::List<_>>();
    for mut sc in __range0 {
        if ComponentRef::isEqual(metamodelica::AsArg::as_arg(&sc), &stateCref)? {
            break;
        }
        i = i + 1;
    }
    nStates = metamodelica::arrayLength(sem.smComps.clone());
    tArrayBool = metamodelica::Ref::new(Type::NFType::ARRAY {
        elementType: crate::NFType::interned_BOOLEAN(),
        dimensions: list![metamodelica::Ref::new(Dimension::NFDimension::INTEGER {
            size: nStates,
            var: Variability::STRUCTURAL_PARAMETER.clone()
        })],
    });
    activeResetExp = makeCrefExp(
        qCref(
            literal!("activeReset"),
            crate::NFType::interned_BOOLEAN(),
            metamodelica::nil(),
            preRef.clone(),
        )?,
        crate::NFType::interned_BOOLEAN(),
    );
    activeResetStatesExp = makeCrefExp(
        qCref(
            literal!("activeResetStates"),
            tArrayBool,
            list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: i })
            })],
            preRef,
        )?,
        crate::NFType::interned_BOOLEAN(),
    );
    orExp = metamodelica::Ref::new(Expression::NFExpression::LBINARY {
        exp1: activeResetExp,
        operator: Operator::makeOr(crate::NFType::interned_BOOLEAN()),
        exp2: activeResetStatesExp,
    });
    activeExp = makeCrefExp(
        qCref(
            literal!("active"),
            crate::NFType::interned_BOOLEAN(),
            metamodelica::nil(),
            stateCref,
        )?,
        crate::NFType::interned_BOOLEAN(),
    );
    andExp = metamodelica::Ref::new(Expression::NFExpression::LBINARY {
        exp1: activeExp,
        operator: Operator::makeAnd(crate::NFType::interned_BOOLEAN()),
        exp2: orExp,
    });
    prevExp = makePreviousCall(makeCrefExp(lhsCref.clone(), lhsTy.clone()), lhsTy.clone());
    startExp = UnorderedMap::getOrDefault(
        lhsCref.clone(),
        crToStart,
        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
    )?;
    ifExp = makeIfExp(andExp, startExp, prevExp, lhsTy.clone());
    lhsPrevExp = makeCrefExp(
        ComponentRef::prefixCref(
            metamodelica::Ref::new(InstNode::InstNode::NAME_NODE {
                name: {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*ComponentRef::firstName(&lhsCref, false)?);
                    __mm_s.push_str(&*literal!("_previous"));
                    ArcStr::from(__mm_s)
                },
            }),
            lhsTy.clone(),
            metamodelica::nil(),
            ComponentRef::rest(&lhsCref)?,
        )?,
        lhsTy.clone(),
    );
    outEq = makeEq(lhsPrevExp, ifExp, lhsTy);
    Ok(outEq)
}

// ============================================================
// Expression substitution helpers
// ============================================================
fn subsActiveStateInEq(
    mut eq: metamodelica::Ref<Equation::NFEquation>,
) -> Result<metamodelica::Ref<Equation::NFEquation>> {
    let mut eq: metamodelica::Ref<Equation::NFEquation> = eq;
    eq = Equation::mapExp(eq, &subsActiveStateInExp)?;
    Ok(eq)
}

fn subsActiveStateInExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = Expression::map(
        exp,
        (std::sync::Arc::new(fnptr!(
            subsActiveStateHelper,
            metamodelica::Ref<Expression::NFExpression>
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(exp)
}

fn subsActiveStateHelper(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> metamodelica::Ref<Expression::NFExpression> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut expCall: metamodelica::Ref<Call::NFCall>;
    let mut argCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut newExp: metamodelica::Ref<Expression::NFExpression>;
    if '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ Expression::CALL { call: __pa1 } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        expCall = metamodelica::Own::own(__pa1);
        if !(stringEq(&(unwrap_break_err!(Call::functionNameLast(&expCall), '__try0)), &(literal!("activeState")))) {
            break '__try0 Err::<_, _>("fail");
        }
        let __pa2 = ::match_deref::match_deref! { match &(unwrap_break_err!(Call::arguments(&expCall), '__try0)) {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::CREF { cref: __pa2, .. }, tail: Deref @ metamodelica::ListNode::Nil } => __pa2.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        argCref = metamodelica::Own::own(__pa2);
        newExp = makeCrefExp(unwrap_break_err!(qCref(literal!("active"), crate::NFType::interned_BOOLEAN(), metamodelica::nil(), argCref.clone()), '__try0), crate::NFType::interned_BOOLEAN());
        exp = newExp.clone();
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    exp
}

fn subsPreviousCrefs(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut stateVarCrefs: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut found: bool,
) -> (metamodelica::Ref<Expression::NFExpression>, bool) {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut found: bool = found;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut arg1: metamodelica::Ref<Expression::NFExpression>;
    let mut argTy: metamodelica::Ref<Type::NFType>;
    let mut argCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut expCall: metamodelica::Ref<Call::NFCall>;
    let mut newExp: metamodelica::Ref<Expression::NFExpression>;
    if '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ Expression::CALL { call: __pa1 } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        expCall = metamodelica::Own::own(__pa1);
        if !(stringEq(&(unwrap_break_err!(Call::functionNameLast(&expCall), '__try0)), &(literal!("previous")))) {
            break '__try0 Err::<_, _>("fail");
        }
        args = unwrap_break_err!(Call::arguments(&expCall), '__try0);
        if ((args).len() as i32) != 1 {
            break '__try0 Err::<_, _>("fail");
        }
        arg1 = unwrap_break_err!((args).head().cloned(), '__try0);
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(arg1.clone()) {
            Deref @ Expression::CREF { ty: __pa2, cref: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        argTy = metamodelica::Own::own(__pa2);
        argCref = metamodelica::Own::own(__pa3);
        for mut svc in &**stateVarCrefs {
            if unwrap_break_err!(ComponentRef::isEqual(metamodelica::AsArg::as_arg(&svc), &argCref), '__try0) {
                newExp = makeCrefExp(unwrap_break_err!(ComponentRef::prefixCref(metamodelica::Ref::new(InstNode::InstNode::NAME_NODE { name: { let mut __mm_s = String::new(); __mm_s.push_str(&*unwrap_break_err!(ComponentRef::firstName(&argCref, false), '__try0)); __mm_s.push_str(&*literal!("_previous")); ArcStr::from(__mm_s) } }), argTy.clone(), metamodelica::nil(), unwrap_break_err!(ComponentRef::rest(&argCref), '__try0)), '__try0), argTy.clone());
                exp = newExp.clone();
                found = true;
                break;
            }
        }
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    (exp, found)
}

// ============================================================
// createTandC
// ============================================================
fn createTandC(
    mut stateCrefs: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut transitionEqs: &metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<(
    metamodelica::List<Transition>,
    metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
)> {
    let mut t: metamodelica::List<Transition>;
    let mut c: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut transitions: metamodelica::List<Transition>;
    transitions = List::filterMap(
        transitionEqs,
        &({
            let __pe_b1 = stateCrefs.clone();
            move |__pe_a0| extractTransition(&__pe_a0, &__pe_b1)
        }),
    );
    t = List::sort(
        transitions,
        (std::sync::Arc::new(move |__a0: Transition, __a1: Transition| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(priorityGt(&__a0, &__a1))
        }) as std::sync::Arc<dyn ::std::ops::Fn(Transition, Transition) -> Result<bool> + 'static>),
    )?;
    c = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut tr in (t.clone()).into_iter().cloned() {
            let __x = tr.condition.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((t, c))
}

fn extractTransition(
    mut eq: &metamodelica::Ref<Equation::NFEquation>,
    mut stateCrefs: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
) -> Result<Transition> {
    let mut trans: Transition;
    let mut crFrom: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut crTo: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut imm: bool = true;
    let mut rst: bool = true;
    let mut syn: bool = false;
    let mut prio: i32 = 1;
    let mut from: i32;
    let mut to: i32;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut eqCall: metamodelica::Ref<Call::NFCall>;
    let __pa0 = ::match_deref::match_deref! { match &((*eq)) {
        Deref @ Equation::NORETCALL { exp: Deref @ Expression::CALL { call: __pa0 }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    eqCall = metamodelica::Own::own(__pa0);
    if !(stringEq(&(Call::functionNameLast(&eqCall)?), &(literal!("transition")))) {
        return Err("fail");
    }
    args = Call::arguments(&eqCall)?;
    let __pa2 = ::match_deref::match_deref! { match &((args).get(1)?) {
        Deref @ Expression::CREF { cref: __pa2, .. } => __pa2.clone(),
        _ => return Err("pattern mismatch"),
    } };
    crFrom = metamodelica::Own::own(__pa2);
    let __pa3 = ::match_deref::match_deref! { match &((args).get(2)?) {
        Deref @ Expression::CREF { cref: __pa3, .. } => __pa3.clone(),
        _ => return Err("pattern mismatch"),
    } };
    crTo = metamodelica::Own::own(__pa3);
    cond = (args).get(3)?;
    if ((args).len() as i32) >= 4 {
        let __pa4 = ::match_deref::match_deref! { match &((args).get(4)?) {
            Deref @ Expression::BOOLEAN { value: __pa4 } => __pa4.clone(),
            _ => return Err("pattern mismatch"),
        } };
        imm = metamodelica::Own::own(__pa4);
    }
    if ((args).len() as i32) >= 5 {
        let __pa5 = ::match_deref::match_deref! { match &((args).get(5)?) {
            Deref @ Expression::BOOLEAN { value: __pa5 } => __pa5.clone(),
            _ => return Err("pattern mismatch"),
        } };
        rst = metamodelica::Own::own(__pa5);
    }
    if ((args).len() as i32) >= 6 {
        let __pa6 = ::match_deref::match_deref! { match &((args).get(6)?) {
            Deref @ Expression::BOOLEAN { value: __pa6 } => __pa6.clone(),
            _ => return Err("pattern mismatch"),
        } };
        syn = metamodelica::Own::own(__pa6);
    }
    if ((args).len() as i32) >= 7 {
        let __pa7 = ::match_deref::match_deref! { match &((args).get(7)?) {
            Deref @ Expression::INTEGER { value: __pa7 } => __pa7.clone(),
            _ => return Err("pattern mismatch"),
        } };
        prio = metamodelica::Own::own(__pa7);
    }
    from = 1;
    for mut sc in &**stateCrefs {
        if ComponentRef::isEqual(metamodelica::AsArg::as_arg(&sc), &crFrom)? {
            break;
        }
        from = from + 1;
    }
    to = 1;
    for mut sc in &**stateCrefs {
        if ComponentRef::isEqual(metamodelica::AsArg::as_arg(&sc), &crTo)? {
            break;
        }
        to = to + 1;
    }
    trans = Transition {
        from: from,
        to: to,
        condition: cond,
        immediate: imm,
        reset: rst,
        synchronize: syn,
        priority: prio,
    };
    Ok(trans)
}

fn priorityGt(mut t1: &Transition, mut t2: &Transition) -> bool {
    let mut gt: bool;
    gt = t1.priority.clone() > t2.priority.clone();
    gt
}

// ============================================================
// Predicate helpers
// ============================================================
fn isTransitionOrInitialState(mut eq: &metamodelica::Ref<Equation::NFEquation>) -> Result<bool> {
    let mut res: bool = false;
    let () = (::match_deref::match_deref! { match eq {
        Deref @ Equation::NORETCALL { exp: Deref @ Expression::CALL { call: eqCall }, .. } => {
            res = (::match_deref::match_deref! { match &(Call::functionNameLast(metamodelica::AsArg::as_arg(&eqCall))?) {
        Deref @ "transition" => true,
        Deref @ "initialState" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

fn isTransitionForGroup(
    mut eq: &metamodelica::Ref<Equation::NFEquation>,
    mut stateCrefs: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
) -> Result<bool> {
    let mut res: bool = false;
    let mut cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let () = (::match_deref::match_deref! { match eq {
        Deref @ Equation::NORETCALL { exp: Deref @ Expression::CALL { call: eqCall }, .. } if (stringEq(&(Call::functionNameLast(metamodelica::AsArg::as_arg(&eqCall))?), &(literal!("transition")))) => {
            let __pa0 = ::match_deref::match_deref! { match &(((Call::arguments(metamodelica::AsArg::as_arg(&eqCall))?)).head().cloned()?) {
                Deref @ Expression::CREF { cref: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa0);
            for mut sc in &**stateCrefs {
                if ComponentRef::isEqual(&cr, metamodelica::AsArg::as_arg(&sc))? {
                    res = true;
                    break;
                }
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

fn isInitialStateForGroup(
    mut eq: &metamodelica::Ref<Equation::NFEquation>,
    mut initStateCref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<bool> {
    let mut res: bool = false;
    let mut cr: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let () = (::match_deref::match_deref! { match eq {
        Deref @ Equation::NORETCALL { exp: Deref @ Expression::CALL { call: eqCall }, .. } if (stringEq(&(Call::functionNameLast(metamodelica::AsArg::as_arg(&eqCall))?), &(literal!("initialState")))) => {
            let __pa0 = ::match_deref::match_deref! { match &(((Call::arguments(metamodelica::AsArg::as_arg(&eqCall))?)).head().cloned()?) {
                Deref @ Expression::CREF { cref: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa0);
            res = ComponentRef::isEqual(&cr, initStateCref)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

fn isEquationOfState(
    mut eq: &metamodelica::Ref<Equation::NFEquation>,
    mut stateCref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<bool> {
    let mut res: bool = false;
    let mut eqScope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let mut stateName: ArcStr;
    stateName = ComponentRef::firstName(stateCref, false)?;
    let () = (match &**eq {
        Equation::EQUALITY {
            scope: __esc_eqScope, ..
        } => {
            eqScope = (*__esc_eqScope).clone();
            res = stringEqual(
                &(NFInstNode::InstNode::name(&(NFInstNode::InstNode::fromCell(eqScope.clone())?))?),
                &stateName,
            );
            ()
        }
        Equation::WHEN {
            scope: __esc_eqScope, ..
        } => {
            eqScope = (*__esc_eqScope).clone();
            res = stringEqual(
                &(NFInstNode::InstNode::name(&(NFInstNode::InstNode::fromCell(eqScope.clone())?))?),
                &stateName,
            );
            ()
        }
        _ => (),
    });
    Ok(res)
}

fn isVariableOfState(
    mut var: &metamodelica::Ref<Variable::NFVariable>,
    mut stateCref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<bool> {
    let mut res: bool;
    res = crefHasPrefix(stateCref, var.name.clone())?;
    Ok(res)
}

fn isOuterStateEquation(
    mut eq: &metamodelica::Ref<Equation::NFEquation>,
    mut stateCrefs: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
) -> Result<bool> {
    let mut res: bool = false;
    let mut eqScope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let mut scopeName: ArcStr;
    let () = (match &**eq {
        Equation::EQUALITY {
            scope: __esc_eqScope, ..
        } => {
            eqScope = (*__esc_eqScope).clone();
            scopeName = NFInstNode::InstNode::name(&(NFInstNode::InstNode::fromCell(eqScope.clone())?))?;
            for mut stateCref in &**stateCrefs {
                if stringEqual(
                    &scopeName,
                    &(ComponentRef::firstName(metamodelica::AsArg::as_arg(&stateCref), false)?),
                ) {
                    res = true;
                    return Ok(res);
                }
            }
            ()
        }
        Equation::WHEN {
            scope: __esc_eqScope, ..
        } => {
            eqScope = (*__esc_eqScope).clone();
            scopeName = NFInstNode::InstNode::name(&(NFInstNode::InstNode::fromCell(eqScope.clone())?))?;
            for mut stateCref in &**stateCrefs {
                if stringEqual(
                    &scopeName,
                    &(ComponentRef::firstName(metamodelica::AsArg::as_arg(&stateCref), false)?),
                ) {
                    res = true;
                    return Ok(res);
                }
            }
            ()
        }
        _ => (),
    });
    Ok(res)
}

fn generateMergeEquation(
    mut outerVarCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut outerVarMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<ComponentRef::NFComponentRef>,
            )>,
        >,
    >,
    mut allVariables: &metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut accEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    mut accVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
    metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
)> {
    let mut accEqs: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = accEqs;
    let mut accVars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = accVars;
    let mut stateEntries: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<ComponentRef::NFComponentRef>,
    )>;
    let mut mergeRhs: metamodelica::Ref<Expression::NFExpression>;
    let mut outerVarExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut activeRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut perStateVarRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    stateEntries = UnorderedMap::getOrDefault(outerVarCref.clone(), outerVarMap, metamodelica::nil())?;
    if (stateEntries).is_empty() {
        return Ok((accEqs, accVars));
    }
    ty = crate::NFType::interned_INTEGER();
    for mut v in &**allVariables {
        if ComponentRef::isEqual(&v.name, &outerVarCref)? {
            ty = v.ty.clone();
            break;
        }
    }
    outerVarExp = makeCrefExp(outerVarCref, ty.clone());
    mergeRhs = makePreviousCall(outerVarExp.clone(), ty.clone());
    for mut entry in &*stateEntries {
        (activeRef, perStateVarRef) = entry.clone();
        mergeRhs = makeIfExp(
            makeCrefExp(activeRef, crate::NFType::interned_BOOLEAN()),
            makeCrefExp(perStateVarRef, ty.clone()),
            mergeRhs,
            ty.clone(),
        );
    }
    src = ElementSource::createElementSource(
        Absyn::dummyInfo.clone(),
        None,
        &(openmodelica_frontend_types::DAE::Prefix::NOPRE),
        (DAE::emptyCref().clone(), DAE::emptyCref().clone()),
    );
    accEqs = metamodelica::cons(
        metamodelica::Ref::new(Equation::NFEquation::EQUALITY {
            lhs: outerVarExp,
            rhs: mergeRhs,
            ty: ty,
            scope: NFInstNode::NO_SCOPE().clone(),
            source: src,
            scalarizeMode: ScalarizeMode::NO_PREFERENCE.clone(),
        }),
        accEqs,
    );
    Ok((accEqs, accVars))
}

// ============================================================
// ComponentRef utilities
// ============================================================
fn qCref(
    mut name: ArcStr,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut prefixCr: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    cref = ComponentRef::fromNode(
        metamodelica::Ref::new(InstNode::InstNode::NAME_NODE { name: name }),
        ty,
        subs,
        ComponentRef::Origin::CREF.clone(),
    )?;
    cref = ComponentRef::prepend(prefixCr, cref)?;
    Ok(cref)
}

fn makeSMSPrefix(
    mut initStateCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut preRef: metamodelica::Ref<ComponentRef::NFComponentRef>;
    preRef = ComponentRef::fromNode(
        metamodelica::Ref::new(InstNode::InstNode::NAME_NODE {
            name: arcstr::literal!(SMS_PRE),
        }),
        crate::NFType::interned_UNKNOWN(),
        metamodelica::nil(),
        ComponentRef::Origin::CREF.clone(),
    )?;
    preRef = ComponentRef::append(initStateCref, &preRef)?;
    Ok(preRef)
}

// ============================================================
// Variable creation helpers
// ============================================================
fn makeVar(
    mut name: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut var: Variability,
) -> metamodelica::Ref<Variable::NFVariable> {
    let mut v: metamodelica::Ref<Variable::NFVariable>;
    let mut attr: metamodelica::Ref<Attributes::NFAttributes>;
    attr = Attributes::DEFAULT_ATTR().clone();
    assign_field!(attr.variability = var);
    v = metamodelica::Ref::new(Variable::NFVariable {
        name: name,
        ty: ty,
        binding: Binding::EMPTY_BINDING().clone(),
        visibility: Visibility::PUBLIC.clone(),
        attributes: attr,
        typeAttributes: metamodelica::nil(),
        children: metamodelica::nil(),
        comment: metamodelica::Ref::new(SCode::Comment {
            annotation_: None,
            comment: None,
        }),
        info: Absyn::dummyInfo.clone(),
        backendinfo: NFBackendExtension::DUMMY_BACKEND_INFO().clone(),
    });
    v
}

fn makeVarWithStart(
    mut name: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut var: Variability,
    mut startExp: metamodelica::Ref<Expression::NFExpression>,
) -> metamodelica::Ref<Variable::NFVariable> {
    let mut v: metamodelica::Ref<Variable::NFVariable>;
    v = makeVar(name, ty, var);
    assign_field!(
        v.typeAttributes = list![
            (
                literal!("start"),
                Binding::makeFlat(
                    startExp,
                    Variability::CONSTANT.clone(),
                    Binding::Source::GENERATED.clone(),
                    Binding::NO_CONFIDENCE.clone()
                )
            ),
            (
                literal!("fixed"),
                Binding::makeFlat(
                    metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true }),
                    Variability::CONSTANT.clone(),
                    Binding::Source::GENERATED.clone(),
                    Binding::NO_CONFIDENCE.clone()
                )
            )
        ]
    );
    v
}

fn makeVarWithBinding(
    mut name: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut var: Variability,
    mut bindExp: metamodelica::Ref<Expression::NFExpression>,
) -> metamodelica::Ref<Variable::NFVariable> {
    let mut v: metamodelica::Ref<Variable::NFVariable>;
    v = makeVar(name, ty, var);
    assign_field!(
        v.binding = Binding::makeFlat(
            bindExp,
            var,
            Binding::Source::GENERATED.clone(),
            Binding::NO_CONFIDENCE.clone()
        )
    );
    v
}

// ============================================================
// Equation creation helpers
// ============================================================
fn makeEq(
    mut lhs: metamodelica::Ref<Expression::NFExpression>,
    mut rhs: metamodelica::Ref<Expression::NFExpression>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> metamodelica::Ref<Equation::NFEquation> {
    let mut eq: metamodelica::Ref<Equation::NFEquation>;
    eq = metamodelica::Ref::new(Equation::NFEquation::EQUALITY {
        lhs: lhs,
        rhs: rhs,
        ty: ty,
        scope: NFInstNode::NO_SCOPE().clone(),
        source: DAE::emptyElementSource().clone(),
        scalarizeMode: ScalarizeMode::NO_PREFERENCE.clone(),
    });
    eq
}

// ============================================================
// Expression creation helpers
// ============================================================
fn makeCrefExp(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> metamodelica::Ref<Expression::NFExpression> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = metamodelica::Ref::new(Expression::NFExpression::CREF { ty: ty, cref: cref });
    exp
}

fn makeIfExp(
    mut cond: metamodelica::Ref<Expression::NFExpression>,
    mut thenExp: metamodelica::Ref<Expression::NFExpression>,
    mut elseExp: metamodelica::Ref<Expression::NFExpression>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> metamodelica::Ref<Expression::NFExpression> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = metamodelica::Ref::new(Expression::NFExpression::IF {
        ty: ty,
        condition: cond,
        trueBranch: thenExp,
        falseBranch: elseExp,
    });
    exp
}

fn makePreviousCall(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> metamodelica::Ref<Expression::NFExpression> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(
            NFBuiltinFuncs::PREVIOUS().clone(),
            list![exp],
            Variability::DISCRETE.clone(),
            Purity::IMPURE.clone(),
            ty,
        ),
    });
    result
}

fn makeInitialCall() -> metamodelica::Ref<Expression::NFExpression> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(
            NFBuiltinFuncs::INITIAL().clone(),
            metamodelica::nil(),
            Variability::DISCRETE.clone(),
            Purity::IMPURE.clone(),
            crate::NFType::interned_BOOLEAN(),
        ),
    });
    result
}

fn makeMaxIntArrCall(
    mut exps: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> metamodelica::Ref<Expression::NFExpression> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut arrTy: metamodelica::Ref<Type::NFType>;
    arrTy = metamodelica::Ref::new(Type::NFType::ARRAY {
        elementType: crate::NFType::interned_INTEGER(),
        dimensions: list![metamodelica::Ref::new(Dimension::NFDimension::INTEGER {
            size: ((exps).len() as i32),
            var: Variability::STRUCTURAL_PARAMETER.clone()
        })],
    });
    result = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(
            NFBuiltinFuncs::MAX_INT_ARR().clone(),
            list![metamodelica::Ref::new(Expression::NFExpression::ARRAY {
                ty: arrTy,
                elements: metamodelica::arrayFromVec(exps.into_iter().cloned().collect()),
                literal: true
            })],
            Variability::DISCRETE.clone(),
            Purity::PURE.clone(),
            crate::NFType::interned_INTEGER(),
        ),
    });
    result
}

fn makeSampleTimeCall() -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    let mut timeExp: metamodelica::Ref<Expression::NFExpression>;
    let mut clockExp: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    ty = crate::NFType::interned_REAL();
    timeExp = metamodelica::Ref::new(Expression::NFExpression::CREF {
        ty: ty.clone(),
        cref: ComponentRef::prefixCref(
            metamodelica::Ref::new(InstNode::InstNode::NAME_NODE { name: literal!("time") }),
            ty.clone(),
            metamodelica::nil(),
            crate::NFComponentRef::interned_EMPTY(),
        )?,
    });
    clockExp = metamodelica::Ref::new(Expression::NFExpression::CLKCONST {
        clk: metamodelica::Ref::new(NFClockKind::NFClockKind::INFERRED_CLOCK {
            idx: System::tmpTickIndex(Global::inferredClock_index.clone()),
        }),
    });
    result = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(
            NFBuiltinFuncs::SAMPLE_CLOCKED().clone(),
            list![timeExp, clockExp],
            Variability::CONTINUOUS.clone(),
            Purity::IMPURE.clone(),
            ty,
        ),
    });
    Ok(result)
}

fn makeRelationEq(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> metamodelica::Ref<Expression::NFExpression> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = metamodelica::Ref::new(Expression::NFExpression::RELATION {
        exp1: exp1,
        operator: Operator::makeEqual(ty),
        exp2: exp2,
        index: 0,
    });
    result
}

fn makeRelationGt(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> metamodelica::Ref<Expression::NFExpression> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = metamodelica::Ref::new(Expression::NFExpression::RELATION {
        exp1: exp1,
        operator: Operator::makeGreater(ty),
        exp2: exp2,
        index: 0,
    });
    result
}

// ============================================================
// Start value helpers
// ============================================================
fn getStartValue(
    mut var: &metamodelica::Ref<Variable::NFVariable>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut startExp: metamodelica::Ref<Expression::NFExpression>;
    let mut attrName: ArcStr;
    let mut attrBinding: metamodelica::Ref<Binding::NFBinding>;
    let mut startOpt: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    for mut attr in &*var.typeAttributes.clone() {
        (attrName, attrBinding) = attr.clone();
        if metamodelica::stringEq(&attrName, &(literal!("start"))) {
            startOpt = Binding::typedExp(&attrBinding);
            if (startOpt).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(startOpt) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                startExp = metamodelica::Own::own(__pa0);
                return Ok(startExp);
            }
        }
    }
    ty = var.ty.clone();
    startExp = (match &*ty {
        Type::INTEGER => metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
        Type::REAL => metamodelica::Ref::new(Expression::NFExpression::REAL {
            value: metamodelica::OrderedFloat(0.0_f64),
        }),
        Type::BOOLEAN => metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }),
        Type::STRING => metamodelica::Ref::new(Expression::NFExpression::STRING { value: literal!("") }),
        _ => metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
    });
    Ok(startExp)
}

// ============================================================
// ComponentRef prefix check
// ============================================================
fn crefHasPrefix<'__b>(
    mut prefix: &'__b metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<bool> {
    '__tco: loop {
        if ComponentRef::isEqual(prefix, &cref)? {
            return Ok(true);
        } else if ComponentRef::isEmpty(&cref) {
            return Ok(false);
        } else {
            {
                (prefix, cref) = (prefix, ComponentRef::rest(&cref)?);
                continue '__tco;
            }
        }
    }
}
