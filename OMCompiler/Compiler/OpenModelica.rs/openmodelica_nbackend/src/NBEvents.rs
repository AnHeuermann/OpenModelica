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

use crate::NBBackendUtil as BackendUtil;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::Frame;
use crate::NBEquation::IfEquationBody;
use crate::NBEquation::Iterator;
use crate::NBEquation::WhenEquationBody;
use crate::NBModule as Module;
use crate::NBSolve as Solve;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use crate::NSimGenericCall::SimIterator;
use crate::NSimStrongComponent::Block;
use openmodelica_ast::Absyn::Path;
use openmodelica_backend_types::BackendDAE as OldBackendDAE;
use openmodelica_backend_types::BackendDAE::SimIterator as OldSimIterator;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFAlgorithm as Algorithm;
use openmodelica_nf_frontend::NFBuiltin as Builtin;
use openmodelica_nf_frontend::NFBuiltinFuncs;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFClockKind as ClockKind;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFPrefixes as Prefixes;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_simcode_types::SimCode as OldSimCode;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// OF imports
// NF
// OB
// New Backend
// SimCode
// Old Simcode
// Util
// =========================================================================
//                      MAIN ROUTINE, PLEASE DO NOT CHANGE
// =========================================================================
pub(crate) fn main(
    mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::NBackendDAE>> {
    let mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE> = bdae;
    let mut func: Module::eventsInterface;
    func = getModule()?;
    bdae = (match &*bdae {
        BackendDAE::MAIN {
            eqData: __bdae_eqData,
            eventInfo: __bdae_eventInfo,
            funcMap: __bdae_funcMap,
            varData: __bdae_varData,
            ..
        } => {
            let mut varData: metamodelica::Ref<VarData::VarData>;
            let mut eqData: metamodelica::Ref<EqData::EqData>;
            let mut eventInfo: metamodelica::Ref<EventInfo::EventInfo>;
            (varData, eqData, eventInfo) = func(
                __bdae_varData.clone(),
                __bdae_eqData.clone(),
                __bdae_eventInfo.clone(),
                __bdae_funcMap.clone(),
            )?;
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                varData = varData,
                eqData = eqData,
                eventInfo = eventInfo
            );
            bdae
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBEvents.main"));
                    __mm_s.push_str(&*literal!(" failed."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(bdae)
}

pub(crate) fn getModule() -> Result<
    Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<VarData::VarData>,
                metamodelica::Ref<EqData::EqData>,
                metamodelica::Ref<EventInfo::EventInfo>,
                metamodelica::Ref<
                    UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
                >,
            ) -> Result<(
                metamodelica::Ref<VarData::VarData>,
                metamodelica::Ref<EqData::EqData>,
                metamodelica::Ref<EventInfo::EventInfo>,
            )> + 'static,
    >,
> {
    let mut func: Module::eventsInterface;
    let mut flag: ArcStr = literal!("default");
    func = (::match_deref::match_deref! { match &(flag) {
        Deref @ "default" => (std::sync::Arc::new(eventsDefault) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>, metamodelica::Ref<EventInfo::EventInfo>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>) -> Result<(metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>, metamodelica::Ref<EventInfo::EventInfo>)> + 'static>),
        _ => return Err("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(func)
}

// =========================================================================
//                    TYPES, UNIONTYPES AND MEMBER FUNCTIONS
// =========================================================================
pub mod EventInfo {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct EventInfo {
        /// tracks compact time events (SINGLE or SAMPLE)
        pub time_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<TimeEvent::TimeEvent>>>,
        /// tracks full time events of the form $TEV_11 = ...
        pub time_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<Condition::Condition>,
                metamodelica::Ref<CompositeEvent::CompositeEvent>,
            >,
        >,
        /// tracks full state events of the form $SEV_4 = ...
        pub state_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<Condition::Condition>,
                metamodelica::Ref<StateEvent::StateEvent>,
            >,
        >,
        /// stores the number of math function that trigger events e.g. floor, ceil, integer, ...
        pub numberMathEvents: i32,
        /// stores all spatial distribution calls
        pub spatial_lst: metamodelica::List<metamodelica::Ref<SpatialDistribution::SpatialDistribution>>,
    }

    impl metamodelica::gc::MMTrace for EventInfo {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.time_set, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.time_map, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.state_map, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numberMathEvents, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.spatial_lst, __mmv)?;
            Ok(())
        }
    }
    impl Default for EventInfo {
        fn default() -> Self {
            Self {
                time_set: Default::default(),
                time_map: Default::default(),
                state_map: Default::default(),
                numberMathEvents: Default::default(),
                spatial_lst: Default::default(),
            }
        }
    }

    pub type EVENT_INFO = EventInfo;

    pub(crate) fn toString(mut eventInfo: &metamodelica::Ref<EventInfo>) -> Result<ArcStr> {
        fn tplString<
            T1: Clone + 'static + metamodelica::gc::MMTrace,
            T2: Clone + 'static + metamodelica::gc::MMTrace,
        >(
            mut tpl: (T1, T2),
            mut f1: &dyn ::std::ops::Fn(T1) -> Result<ArcStr>,
            mut f2: &dyn ::std::ops::Fn(T2) -> Result<ArcStr>,
        ) -> Result<ArcStr> {
            type F1<T1: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T1) -> Result<ArcStr> + 'static>;

            type F2<T2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T2) -> Result<ArcStr> + 'static>;

            let mut r#str: ArcStr;
            let mut t1: T1;
            let mut t2: T2;
            (t1, t2) = tpl;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*f2(t2)?);
                __mm_s.push_str(&*literal!(" = "));
                __mm_s.push_str(&*f1(t1)?);
                ArcStr::from(__mm_s)
            };
            Ok(r#str)
        }

        let mut r#str: ArcStr = literal!("");
        let mut tev_lst: metamodelica::List<metamodelica::Ref<TimeEvent::TimeEvent>>;
        let mut cev_lst: metamodelica::List<(
            metamodelica::Ref<Condition::Condition>,
            metamodelica::Ref<CompositeEvent::CompositeEvent>,
        )>;
        let mut sev_lst: metamodelica::List<(
            metamodelica::Ref<Condition::Condition>,
            metamodelica::Ref<StateEvent::StateEvent>,
        )>;
        if !(isEmpty(eventInfo)) {
            (tev_lst, cev_lst, sev_lst) = toLists(eventInfo)?;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*StringUtil::headline_2(&(literal!("Event Info")))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*StringUtil::headline_4(&(literal!("Time Events")))?);
                __mm_s.push_str(&*List::toString(
                    tev_lst,
                    &({
                        let __pe_b1 = true;
                        move |__pe_a0| TimeEvent::toString(&__pe_a0, __pe_b1.clone())
                    }),
                    List::Style::NEWLINE.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*StringUtil::headline_4(&(literal!("Composite Events")))?);
                __mm_s.push_str(&*List::toString(
                    cev_lst,
                    &({
                        let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<ArcStr> + 'static> =
                            (std::sync::Arc::new(move |__a0: metamodelica::Ref<Condition::Condition>| {
                                Condition::toString(&__a0)
                            })
                                as std::sync::Arc<
                                    dyn ::std::ops::Fn(metamodelica::Ref<Condition::Condition>) -> Result<ArcStr>
                                        + 'static,
                                >);
                        let __pe_b2: Arc<dyn ::std::ops::Fn(_) -> Result<ArcStr> + 'static> =
                            (std::sync::Arc::new(move |__a0: metamodelica::Ref<CompositeEvent::CompositeEvent>| {
                                CompositeEvent::toString(&__a0)
                            })
                                as std::sync::Arc<
                                    dyn ::std::ops::Fn(
                                            metamodelica::Ref<CompositeEvent::CompositeEvent>,
                                        ) -> Result<ArcStr>
                                        + 'static,
                                >);
                        move |__pe_a0| tplString(__pe_a0, &*__pe_b1, &*__pe_b2)
                    }),
                    List::Style::NEWLINE.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*StringUtil::headline_4(&(literal!("State Events")))?);
                __mm_s.push_str(&*List::toString(
                    sev_lst,
                    &({
                        let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<ArcStr> + 'static> =
                            (std::sync::Arc::new(move |__a0: metamodelica::Ref<Condition::Condition>| {
                                Condition::toString(&__a0)
                            })
                                as std::sync::Arc<
                                    dyn ::std::ops::Fn(metamodelica::Ref<Condition::Condition>) -> Result<ArcStr>
                                        + 'static,
                                >);
                        let __pe_b2: Arc<dyn ::std::ops::Fn(_) -> Result<ArcStr> + 'static> =
                            (std::sync::Arc::new(move |__a0: metamodelica::Ref<StateEvent::StateEvent>| {
                                StateEvent::toString(&__a0)
                            })
                                as std::sync::Arc<
                                    dyn ::std::ops::Fn(metamodelica::Ref<StateEvent::StateEvent>) -> Result<ArcStr>
                                        + 'static,
                                >);
                        move |__pe_a0| tplString(__pe_a0, &*__pe_b1, &*__pe_b2)
                    }),
                    List::Style::NEWLINE.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            };
        }
        Ok(r#str)
    }

    pub(crate) fn toLists(
        mut eventInfo: &metamodelica::Ref<EventInfo>,
    ) -> Result<(
        metamodelica::List<metamodelica::Ref<TimeEvent::TimeEvent>>,
        metamodelica::List<(
            metamodelica::Ref<Condition::Condition>,
            metamodelica::Ref<CompositeEvent::CompositeEvent>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<Condition::Condition>,
            metamodelica::Ref<StateEvent::StateEvent>,
        )>,
    )> {
        let mut tev_lst: metamodelica::List<metamodelica::Ref<TimeEvent::TimeEvent>>;
        let mut cev_lst: metamodelica::List<(
            metamodelica::Ref<Condition::Condition>,
            metamodelica::Ref<CompositeEvent::CompositeEvent>,
        )>;
        let mut sev_lst: metamodelica::List<(
            metamodelica::Ref<Condition::Condition>,
            metamodelica::Ref<StateEvent::StateEvent>,
        )>;
        tev_lst = List::sort(
            UnorderedSet::toList(eventInfo.time_set.clone()),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<TimeEvent::TimeEvent>,
                      __a1: metamodelica::Ref<TimeEvent::TimeEvent>|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(TimeEvent::indexGt(&__a0, &__a1))
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<TimeEvent::TimeEvent>,
                            metamodelica::Ref<TimeEvent::TimeEvent>,
                        ) -> Result<bool>
                        + 'static,
                >),
        )?;
        cev_lst = List::sort(
            UnorderedMap::toList(eventInfo.time_map.clone()),
            (std::sync::Arc::new(
                move |__a0: (
                    metamodelica::Ref<Condition::Condition>,
                    metamodelica::Ref<CompositeEvent::CompositeEvent>,
                ),
                      __a1: (
                    metamodelica::Ref<Condition::Condition>,
                    metamodelica::Ref<CompositeEvent::CompositeEvent>,
                )|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(CompositeEvent::indexGt(&__a0, &__a1))
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            (
                                metamodelica::Ref<Condition::Condition>,
                                metamodelica::Ref<CompositeEvent::CompositeEvent>,
                            ),
                            (
                                metamodelica::Ref<Condition::Condition>,
                                metamodelica::Ref<CompositeEvent::CompositeEvent>,
                            ),
                        ) -> Result<bool>
                        + 'static,
                >),
        )?;
        sev_lst = List::sort(
            UnorderedMap::toList(eventInfo.state_map.clone()),
            (std::sync::Arc::new(
                move |__a0: (
                    metamodelica::Ref<Condition::Condition>,
                    metamodelica::Ref<StateEvent::StateEvent>,
                ),
                      __a1: (
                    metamodelica::Ref<Condition::Condition>,
                    metamodelica::Ref<StateEvent::StateEvent>,
                )|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(StateEvent::indexGt(&__a0, &__a1))
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            (
                                metamodelica::Ref<Condition::Condition>,
                                metamodelica::Ref<StateEvent::StateEvent>,
                            ),
                            (
                                metamodelica::Ref<Condition::Condition>,
                                metamodelica::Ref<StateEvent::StateEvent>,
                            ),
                        ) -> Result<bool>
                        + 'static,
                >),
        )?;
        Ok((tev_lst, cev_lst, sev_lst))
    }

    pub(crate) fn create(
        mut bucket: &metamodelica::Ref<Bucket>,
        mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
        mut idx: Pointer::Pointer<i32>,
        mut spatial_lst: metamodelica::List<metamodelica::Ref<SpatialDistribution::SpatialDistribution>>,
    ) -> Result<(
        metamodelica::Ref<EventInfo>,
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    )> {
        let mut eventInfo: metamodelica::Ref<EventInfo>;
        let mut auxiliary_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            metamodelica::nil();
        let mut auxiliary_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
            metamodelica::nil();
        let mut cond: metamodelica::Ref<Condition::Condition>;
        let mut cev: metamodelica::Ref<CompositeEvent::CompositeEvent>;
        let mut sev: metamodelica::Ref<StateEvent::StateEvent>;
        for mut tpl in &*UnorderedMap::toList(bucket.time_map.clone()) {
            (cond, cev) = tpl.clone();
            (auxiliary_vars, auxiliary_eqns) = createAux(
                &cond,
                cev.auxiliary.clone(),
                variables,
                idx.clone(),
                auxiliary_vars,
                auxiliary_eqns,
            )?;
        }
        for mut tpl in &*UnorderedMap::toList(bucket.state_map.clone()) {
            (cond, sev) = tpl.clone();
            (auxiliary_vars, auxiliary_eqns) = createAux(
                &cond,
                sev.auxiliary.clone(),
                variables,
                idx.clone(),
                auxiliary_vars,
                auxiliary_eqns,
            )?;
        }
        eventInfo = metamodelica::Ref::new(EventInfo {
            time_set: bucket.time_set.clone(),
            time_map: bucket.time_map.clone(),
            state_map: bucket.state_map.clone(),
            numberMathEvents: 0,
            spatial_lst: spatial_lst,
        });
        if Flags::isSet(Flags::DUMP_EVENTS.clone())? {
            metamodelica::print(toString(&eventInfo)?);
            metamodelica::print(List::toStringCustom(
                auxiliary_eqns.clone(),
                &({
                    let __pe_b1 = literal!("  ");
                    move |__pe_a0| BEquation::Equation::pointerToString(__pe_a0, __pe_b1.clone())
                }),
                StringUtil::headline_4(&(literal!("Event Equations")))?,
                literal!(""),
                literal!("\n"),
                literal!("\n\n"),
                true,
                0,
            )?);
        }
        Ok((eventInfo, auxiliary_vars, auxiliary_eqns))
    }

    pub(crate) fn createAux(
        mut cond: &metamodelica::Ref<Condition::Condition>,
        mut aux_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
        mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
        mut idx: Pointer::Pointer<i32>,
        mut auxiliary_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        mut auxiliary_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    ) -> Result<(
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    )> {
        let mut auxiliary_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            auxiliary_vars;
        let mut auxiliary_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
            auxiliary_eqns;
        let mut lhs_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut aux_eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
        if cond.stmt_index.clone() == 0 {
            lhs_cref = ComponentRef::mapSubscripts(
                BVariable::getVarName(aux_var.clone()),
                &({
                    let __pe_b1: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    > = (std::sync::Arc::new({
                        let __pe_b1 = variables.clone();
                        let __pe_b2 = true;
                        move |__pe_a0| BackendDAE::lowerComponentReferenceExp(__pe_a0, &__pe_b1, __pe_b2.clone())
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >);
                    move |__pe_a0| Subscript::mapExp(__pe_a0, __pe_b1.clone())
                }),
                false,
            )?;
            aux_eqn = BEquation::Equation::makeAssignment(
                Expression::fromCref(lhs_cref, false)?,
                cond.exp.clone(),
                idx,
                &(literal!("EVT")),
                cond.iter.clone(),
                BEquation::default(EquationKind::DISCRETE.clone(), false, None, None),
            )?;
            auxiliary_eqns = metamodelica::cons(aux_eqn, auxiliary_eqns);
        }
        BVariable::setVarName(
            aux_var.clone(),
            ComponentRef::stripSubscriptsAll(&(BVariable::getVarName(aux_var.clone()))),
        );
        auxiliary_vars = metamodelica::cons(aux_var, auxiliary_vars);
        Ok((auxiliary_vars, auxiliary_eqns))
    }

    pub(crate) fn createAuxStatements(
        mut new_stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
        mut bucket_ptr: Pointer::Pointer<metamodelica::Ref<Bucket>>,
        mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    ) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
        let mut new_stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = new_stmts;
        let mut bucket: metamodelica::Ref<Bucket> = Pointer::access(bucket_ptr.clone());
        let mut new_stmt: metamodelica::Ref<Statement::NFStatement>;
        let mut cond: metamodelica::Ref<Condition::Condition>;
        let mut aux: metamodelica::Ref<ComponentRef::NFComponentRef>;
        if (bucket.aux_stmts).is_some() {
            for mut tpl in &*Util::getOption(bucket.aux_stmts.clone())? {
                (cond, aux) = tpl.clone();
                aux = ComponentRef::mapSubscripts(
                    aux,
                    &({
                        let __pe_b1: Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        > = (std::sync::Arc::new({
                            let __pe_b1 = variables.clone();
                            let __pe_b2 = true;
                            move |__pe_a0| BackendDAE::lowerComponentReferenceExp(__pe_a0, &__pe_b1, __pe_b2.clone())
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<Expression::NFExpression>,
                                    )
                                        -> Result<metamodelica::Ref<Expression::NFExpression>>
                                    + 'static,
                            >);
                        move |__pe_a0| Subscript::mapExp(__pe_a0, __pe_b1.clone())
                    }),
                    false,
                )?;
                new_stmt = Statement::makeAssignment(
                    Expression::fromCref(aux.clone(), false)?,
                    cond.exp.clone(),
                    ComponentRef::getSubscriptedType(&aux, false)?,
                    DAE::emptyElementSource().clone(),
                );
                new_stmts = metamodelica::cons(new_stmt, new_stmts);
            }
            assign_field!(bucket.aux_stmts = None);
            Pointer::update(bucket_ptr, bucket);
        }
        Ok(new_stmts)
    }

    pub(crate) fn empty() -> metamodelica::Ref<EventInfo> {
        let mut eventInfo: metamodelica::Ref<EventInfo>;
        eventInfo = metamodelica::Ref::new(EventInfo {
            time_set: UnorderedSet::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<TimeEvent::TimeEvent>| TimeEvent::hash(&__a0))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<TimeEvent::TimeEvent>) -> Result<i32> + 'static,
                    >),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<TimeEvent::TimeEvent>,
                          __a1: metamodelica::Ref<TimeEvent::TimeEvent>| {
                        TimeEvent::isEqual(&__a0, &__a1)
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<TimeEvent::TimeEvent>,
                                metamodelica::Ref<TimeEvent::TimeEvent>,
                            ) -> Result<bool>
                            + 'static,
                    >),
                13,
            ),
            time_map: UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<Condition::Condition>| Condition::hash(&__a0))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Condition::Condition>) -> Result<i32> + 'static,
                    >),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Condition::Condition>,
                          __a1: metamodelica::Ref<Condition::Condition>| {
                        Condition::isEqual(&__a0, &__a1)
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Condition::Condition>,
                                metamodelica::Ref<Condition::Condition>,
                            ) -> Result<bool>
                            + 'static,
                    >),
                1,
            ),
            state_map: UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<Condition::Condition>| Condition::hash(&__a0))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Condition::Condition>) -> Result<i32> + 'static,
                    >),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Condition::Condition>,
                          __a1: metamodelica::Ref<Condition::Condition>| {
                        Condition::isEqual(&__a0, &__a1)
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Condition::Condition>,
                                metamodelica::Ref<Condition::Condition>,
                            ) -> Result<bool>
                            + 'static,
                    >),
                1,
            ),
            numberMathEvents: 0,
            spatial_lst: metamodelica::nil(),
        });
        eventInfo
    }

    pub(crate) fn isEmpty(mut eventInfo: &metamodelica::Ref<EventInfo>) -> bool {
        let mut b: bool;
        b = UnorderedSet::isEmpty(eventInfo.time_set.clone())
            && UnorderedMap::isEmpty(eventInfo.time_map.clone())
            && UnorderedMap::isEmpty(eventInfo.state_map.clone())
            && eventInfo.numberMathEvents.clone() == 0;
        b
    }

    pub(crate) fn convert(
        mut eventInfo: &metamodelica::Ref<EventInfo>,
        mut equation_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Block::Block>,
            >,
        >,
    ) -> Result<(
        metamodelica::List<OldBackendDAE::ZeroCrossing>,
        metamodelica::List<OldBackendDAE::ZeroCrossing>,
        metamodelica::List<OldBackendDAE::TimeEvent>,
        OldSimCode::SpatialDistributionInfo,
    )> {
        let mut zeroCrossings: metamodelica::List<OldBackendDAE::ZeroCrossing>;
        let mut relations: metamodelica::List<OldBackendDAE::ZeroCrossing>;
        let mut timeEvents: metamodelica::List<OldBackendDAE::TimeEvent>;
        let mut spatialInfo: OldSimCode::SpatialDistributionInfo;
        let mut tev_lst: metamodelica::List<metamodelica::Ref<TimeEvent::TimeEvent>>;
        let mut cev_lst: metamodelica::List<(
            metamodelica::Ref<Condition::Condition>,
            metamodelica::Ref<CompositeEvent::CompositeEvent>,
        )>;
        let mut sev_lst: metamodelica::List<(
            metamodelica::Ref<Condition::Condition>,
            metamodelica::Ref<StateEvent::StateEvent>,
        )>;
        (tev_lst, cev_lst, sev_lst) = toLists(eventInfo)?;
        zeroCrossings = ({
            let mut __acc: metamodelica::List<OldBackendDAE::ZeroCrossing> = metamodelica::nil();
            for mut sev_tpl in (sev_lst).into_iter().cloned() {
                let __x = StateEvent::convert(&(sev_tpl.clone()), equation_map.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        relations = zeroCrossings.clone();
        timeEvents = ({
            let mut __acc: metamodelica::List<OldBackendDAE::TimeEvent> = metamodelica::nil();
            for mut tev in (tev_lst).into_iter().cloned() {
                let __x = TimeEvent::convert(&(tev.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        if (eventInfo.spatial_lst).is_empty() {
            spatialInfo = OldSimCode::SpatialDistributionInfo {
                spatialDistributions: metamodelica::nil(),
                maxIndex: 0,
            };
        } else {
            spatialInfo = OldSimCode::SpatialDistributionInfo {
                spatialDistributions: ({
                    let mut __acc: metamodelica::List<OldSimCode::SpatialDistribution> = metamodelica::nil();
                    for mut sd in (eventInfo.spatial_lst.clone()).into_iter().cloned() {
                        let __x = SpatialDistribution::convert(&(sd.clone()))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                maxIndex: ((eventInfo.spatial_lst).len() as i32) - 1,
            };
        }
        Ok((zeroCrossings, relations, timeEvents, spatialInfo))
    }
}

pub mod TimeEvent {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub(crate) enum TimeEvent {
        /// e.g. time > 0.5
        SINGLE {
            /// unique sample index
            index: i32,
            /// single point in time that triggers it
            trigger: metamodelica::Ref<Expression::NFExpression>,
            /// potential iterator
            iter: metamodelica::Ref<Iterator::Iterator>,
        },
        /// e.g. sample(1, 1)
        SAMPLE {
            /// unique sample index
            index: i32,
            /// first trigger point
            start: metamodelica::Ref<Expression::NFExpression>,
            /// equidistant intervals
            interval: metamodelica::Ref<Expression::NFExpression>,
            /// potential iterator
            iter: metamodelica::Ref<Iterator::Iterator>,
        },
    }
    impl metamodelica::gc::MMTrace for TimeEvent {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                TimeEvent::SINGLE { index, trigger, iter } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(trigger, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(iter, __mmv)?;
                    Ok(())
                }
                TimeEvent::SAMPLE {
                    index,
                    start,
                    interval,
                    iter,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(start, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(interval, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(iter, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Default for TimeEvent {
        fn default() -> Self {
            Self::SINGLE {
                index: Default::default(),
                trigger: Default::default(),
                iter: Default::default(),
            }
        }
    }
    pub(crate) use self::TimeEvent::{SAMPLE, SINGLE};
    pub(crate) fn toString(mut timeEvent: &metamodelica::Ref<TimeEvent>, mut printIndex: bool) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        let mut iter: metamodelica::Ref<Iterator::Iterator>;
        (r#str, iter) = (match &**timeEvent {
            SINGLE {
                iter: __timeEvent_iter,
                trigger: __timeEvent_trigger,
                ..
            } => (
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("time > "));
                    __mm_s.push_str(&*Expression::toString(__timeEvent_trigger.clone())?);
                    ArcStr::from(__mm_s)
                },
                __timeEvent_iter.clone(),
            ),
            SAMPLE {
                index: __timeEvent_index,
                interval: __timeEvent_interval,
                iter: __timeEvent_iter,
                start: __timeEvent_start,
            } => (
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("sample("));
                    __mm_s.push_str(&*intString(__timeEvent_index.clone()));
                    __mm_s.push_str(&*literal!(", "));
                    __mm_s.push_str(&*Expression::toString(__timeEvent_start.clone())?);
                    __mm_s.push_str(&*literal!(", "));
                    __mm_s.push_str(&*Expression::toString(__timeEvent_interval.clone())?);
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                },
                __timeEvent_iter.clone(),
            ),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEvents.TimeEvent.toString"));
                        __mm_s.push_str(&*literal!(" failed."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        if !(BEquation::Iterator::isEmpty(&iter)) {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!(" for {"));
                __mm_s.push_str(&*BEquation::Iterator::toString(&iter)?);
                __mm_s.push_str(&*literal!("}"));
                ArcStr::from(__mm_s)
            };
        }
        if printIndex {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(getIndex(timeEvent)));
                __mm_s.push_str(&*literal!(") "));
                __mm_s.push_str(&*r#str);
                ArcStr::from(__mm_s)
            };
        }
        Ok(r#str)
    }

    pub(crate) fn toStringList(mut events_lst: metamodelica::List<metamodelica::Ref<TimeEvent>>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = StringUtil::headline_4(&(literal!("Time Events")))?;
        if (events_lst).is_empty() {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("\t<No Time Events>\n"));
                ArcStr::from(__mm_s)
            };
        } else {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*stringDelimitList(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut te in (events_lst).into_iter().cloned() {
                            let __x = toString(&(te.clone()), true)?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    literal!("\n"),
                ));
                ArcStr::from(__mm_s)
            };
        }
        Ok(r#str)
    }

    pub(crate) fn hash(mut tev: &metamodelica::Ref<TimeEvent>) -> Result<i32> {
        let mut h: i32 = stringHashDjb2(&(toString(tev, false)?));
        Ok(h)
    }

    pub(crate) fn isEqual(
        mut tev1: &metamodelica::Ref<TimeEvent>,
        mut tev2: &metamodelica::Ref<TimeEvent>,
    ) -> Result<bool> {
        let mut b: bool;
        b = (::match_deref::match_deref! { match (tev1, tev2) {
            (Deref @ SINGLE { .. }, Deref @ SINGLE { .. }) => Expression::isEqual(var_field!((**tev1).trigger, TimeEvent::SINGLE).clone(), var_field!((**tev2).trigger, TimeEvent::SINGLE).clone())?,
            (Deref @ SAMPLE { .. }, Deref @ SAMPLE { .. }) => Expression::isEqual(var_field!((**tev1).start, TimeEvent::SAMPLE).clone(), var_field!((**tev2).start, TimeEvent::SAMPLE).clone())? && Expression::isEqual(var_field!((**tev1).interval, TimeEvent::SAMPLE).clone(), var_field!((**tev2).interval, TimeEvent::SAMPLE).clone())?,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(b)
    }

    pub(crate) fn indexGt(mut tev1: &metamodelica::Ref<TimeEvent>, mut tev2: &metamodelica::Ref<TimeEvent>) -> bool {
        let mut b: bool = getIndex(tev1) > getIndex(tev2);
        b
    }

    pub(crate) fn create(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut bucket: metamodelica::Ref<Bucket>,
        mut iter: metamodelica::Ref<Iterator::Iterator>,
        mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
        mut funcMap: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
        >,
        mut createEqn: bool,
    ) -> Result<(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::Ref<Bucket>,
        bool,
    )> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        let mut bucket: metamodelica::Ref<Bucket> = bucket;
        let mut failed: bool = false;
        (exp, bucket, failed) = (match &*exp {
            Expression::LBINARY {
                exp1: __exp_exp1,
                exp2: __exp_exp2,
                operator: __exp_operator,
            } if (Operator::getMathClassification(metamodelica::AsArg::as_arg(&__exp_operator))?
                == Operator::MathClassification::LOGICAL.clone()) =>
            {
                let mut exp1: metamodelica::Ref<Expression::NFExpression>;
                let mut exp2: metamodelica::Ref<Expression::NFExpression>;
                let mut b1: bool;
                let mut b2: bool;
                (exp1, bucket, b1) = create(
                    __exp_exp1.clone(),
                    bucket,
                    iter.clone(),
                    eqn.clone(),
                    funcMap.clone(),
                    createEqn,
                )?;
                (exp2, bucket, b2) = create(__exp_exp2.clone(), bucket, iter.clone(), eqn, funcMap, createEqn)?;
                failed = b1 || b2;
                if !(failed) {
                    assign_variant_field!(exp => Expression::NFExpression::LBINARY;
                        exp1 = exp1,
                        exp2 = exp2
                    );
                }
                (exp, bucket, failed)
            }
            _ => createSingleOrSample(exp, bucket, iter.clone(), eqn, funcMap)?,
        });
        if !(failed) {
            (exp, bucket) = CompositeEvent::add(exp, iter, bucket, createEqn)?;
        }
        Ok((exp, bucket, failed))
    }

    pub(crate) fn createSingleOrSample(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut bucket: metamodelica::Ref<Bucket>,
        mut iter: metamodelica::Ref<Iterator::Iterator>,
        mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
        mut funcMap: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
        >,
    ) -> Result<(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::Ref<Bucket>,
        bool,
    )> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        let mut bucket: metamodelica::Ref<Bucket> = bucket;
        let mut failed: bool;
        (exp, failed) = ({
            let mut containsTime: Pointer::Pointer<bool> = Pointer::create(false);
            (match &*exp.clone() {
                Expression::CALL { call: __exp_call } => {
                    let mut call: metamodelica::Ref<Call::NFCall>;
                    (call, bucket, failed, _) = createSample(__exp_call.clone(), bucket, iter)?;
                    assign_variant_field!(exp => Expression::NFExpression::CALL; call = call);
                    (exp, failed)
                }
                Expression::RELATION {
                    exp1: __exp_exp1,
                    exp2: __exp_exp2,
                    operator: __exp_operator,
                    ..
                } if (Operator::getMathClassification(metamodelica::AsArg::as_arg(&__exp_operator))?
                    == Operator::MathClassification::RELATION.clone()) =>
                {
                    let mut tmpEqn: metamodelica::Ref<Equation::Equation>;
                    let mut status: Solve::Status;
                    let mut can_trigger: bool;
                    let mut invert: Solve::RelationInversion;
                    let mut trigger: metamodelica::Ref<Expression::NFExpression>;
                    let mut new_exp: metamodelica::Ref<Expression::NFExpression>;
                    let mut timeEvent: metamodelica::Ref<TimeEvent>;
                    let mut time_op: metamodelica::Ref<Operator::NFOperator>;
                    tmpEqn = Pointer::access(BEquation::Equation::makeAssignment(
                        __exp_exp1.clone(),
                        __exp_exp2.clone(),
                        Pointer::create(0),
                        &(arcstr::literal!(BVariable::TEMPORARY_STR)),
                        crate::NBEquation::Iterator::interned_EMPTY(),
                        BEquation::default(EquationKind::UNKNOWN.clone(), false, None, None),
                    )?);
                    BEquation::Equation::map(
                        tmpEqn.clone(),
                        (std::sync::Arc::new({
                            let __pe_b1 = containsTime.clone();
                            move |__pe_a0| containsTimeTraverseExp(__pe_a0, __pe_b1.clone())
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<Expression::NFExpression>,
                                    )
                                        -> Result<metamodelica::Ref<Expression::NFExpression>>
                                    + 'static,
                            >),
                        Some(
                            (std::sync::Arc::new({
                                let __pe_b1 = containsTime.clone();
                                move |__pe_a0| containsTimeTraverseCref(__pe_a0, __pe_b1.clone())
                            })
                                as std::sync::Arc<
                                    dyn ::std::ops::Fn(
                                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                                        )
                                            -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                                        + 'static,
                                >),
                        ),
                        (std::sync::Arc::new(Expression::map)
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<Expression::NFExpression>,
                                        Arc<
                                            dyn ::std::ops::Fn(
                                                    metamodelica::Ref<Expression::NFExpression>,
                                                )
                                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                                + 'static,
                                        >,
                                    )
                                        -> Result<metamodelica::Ref<Expression::NFExpression>>
                                    + 'static,
                            >),
                    )?;
                    if Pointer::access(containsTime) {
                        (tmpEqn, status, invert) = Solve::solveBody(tmpEqn, Builtin::TIME_CREF().clone(), funcMap)?;
                        if status == Solve::Status::EXPLICIT.clone()
                            && invert != Solve::RelationInversion::UNKNOWN.clone()
                        {
                            let __pa0 = ::match_deref::match_deref! { match &(BEquation::Equation::getRHS(tmpEqn)?) {
                                Some(__pa0) => __pa0.clone(),
                                _ => return Err("pattern mismatch"),
                            } };
                            trigger = metamodelica::Own::own(__pa0);
                            time_op = if (invert == Solve::RelationInversion::TRUE.clone()) {
                                Operator::invert(__exp_operator.clone())?
                            } else {
                                __exp_operator.clone()
                            };
                            if BEquation::Equation::isWhenEquation(eqn.clone())? {
                                can_trigger = (match time_op.op.clone() {
                                    Operator::Op::GREATER => true,
                                    Operator::Op::GREATEREQ => true,
                                    _ => false,
                                });
                                if can_trigger {
                                    timeEvent = getOrAdd(
                                        metamodelica::Ref::new(TimeEvent::SINGLE {
                                            index: UnorderedSet::size(bucket.time_set.clone()),
                                            trigger: trigger.clone(),
                                            iter: iter,
                                        }),
                                        bucket.time_set.clone(),
                                    )?;
                                    new_exp = metamodelica::Ref::new(Expression::NFExpression::CALL {
                                        call: Call::makeTypedCall(
                                            NFBuiltinFuncs::SAMPLE().clone(),
                                            list![
                                                metamodelica::Ref::new(Expression::NFExpression::INTEGER {
                                                    value: getIndex(&timeEvent) + 1
                                                }),
                                                trigger,
                                                Expression::makeMaxValue(
                                                    &(openmodelica_nf_frontend::NFType::interned_REAL())
                                                )?
                                            ],
                                            Prefixes::Variability::DISCRETE.clone(),
                                            Prefixes::Purity::PURE.clone(),
                                            NFBuiltinFuncs::SAMPLE().returnType.clone(),
                                        ),
                                    });
                                } else {
                                    new_exp =
                                        metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false });
                                }
                                failed = false;
                            } else if BEquation::Equation::isAlgorithm(eqn) {
                                timeEvent = getOrAdd(
                                    metamodelica::Ref::new(TimeEvent::SINGLE {
                                        index: UnorderedSet::size(bucket.time_set.clone()),
                                        trigger: trigger,
                                        iter: iter,
                                    }),
                                    bucket.time_set.clone(),
                                )?;
                                failed = false;
                                new_exp = exp;
                            } else {
                                failed = true;
                                new_exp = exp;
                            }
                        } else {
                            failed = true;
                            new_exp = exp;
                        }
                    } else {
                        failed = true;
                        new_exp = exp;
                    }
                    (new_exp, failed)
                }
                _ => (exp, true),
            })
        });
        Ok((exp, bucket, failed))
    }

    pub(crate) fn createSample(
        mut call: metamodelica::Ref<Call::NFCall>,
        mut bucket: metamodelica::Ref<Bucket>,
        mut iter: metamodelica::Ref<Iterator::Iterator>,
    ) -> Result<(metamodelica::Ref<Call::NFCall>, metamodelica::Ref<Bucket>, bool, bool)> {
        let mut call: metamodelica::Ref<Call::NFCall> = call;
        let mut bucket: metamodelica::Ref<Bucket> = bucket;
        let mut failed: bool;
        let mut clocked: bool;
        (failed, clocked) = (::match_deref::match_deref! { match &((AbsynUtil::pathLastIdent(&(Call::functionName(&call)?)), Call::arguments(&call)?)) {
            (Deref @ "sample", Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: clock, tail: Deref @ metamodelica::ListNode::Nil } }) if (Type::isClock(&(Expression::typeOf(clock.clone())))?) => {
                (false, true)
            },
            (Deref @ "sample", Deref @ metamodelica::ListNode::Cons { head: start, tail: Deref @ metamodelica::ListNode::Cons { head: interval, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                let mut timeEvent: metamodelica::Ref<TimeEvent>;
                timeEvent = getOrAdd(metamodelica::Ref::new(TimeEvent::SAMPLE { index: UnorderedSet::size(bucket.time_set.clone()), start: start.clone(), interval: interval.clone(), iter: iter }), bucket.time_set.clone())?;
                call = Call::setArguments(call, list![metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: getIndex(&timeEvent) + 1 }), start.clone(), interval.clone()])?;
                (false, false)
            },
            (Deref @ "sample", _) => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEvents.TimeEvent.createSample")); __mm_s.push_str(&*literal!(" failed for sample operator: ")); __mm_s.push_str(&*Call::toString(&call)?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => {
                (true, false)
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok((call, bucket, failed, clocked))
    }

    pub(crate) fn createSampleTraverse(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut bucket: metamodelica::Ref<Bucket>,
        mut iter: metamodelica::Ref<Iterator::Iterator>,
        mut clocked: Pointer::Pointer<bool>,
    ) -> Result<(metamodelica::Ref<Expression::NFExpression>, metamodelica::Ref<Bucket>)> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        let mut bucket: metamodelica::Ref<Bucket> = bucket;
        let mut c: bool;
        exp = (match &*exp {
            Expression::CALL { call } => {
                let mut call = (*call).clone();
                (call, bucket, _, c) = createSample(call.clone(), bucket, iter)?;
                assign_variant_field!(exp => Expression::NFExpression::CALL; call = call.clone());
                if c {
                    Pointer::update(clocked, c);
                }
                exp
            }
            _ => exp,
        });
        Ok((exp, bucket))
    }

    pub(crate) fn getOrAdd(
        mut timeEvent: metamodelica::Ref<TimeEvent>,
        mut time_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<TimeEvent>>>,
    ) -> Result<metamodelica::Ref<TimeEvent>> {
        let mut result: metamodelica::Ref<TimeEvent>;
        result = (::match_deref::match_deref! { match &(UnorderedSet::get(timeEvent.clone(), time_set.clone())?) {
            Some(__esc_result) => {
                result = (*__esc_result).clone();
                result.clone()
            },
            _ => {
                UnorderedSet::add(timeEvent.clone(), time_set)?;
                timeEvent
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(result)
    }

    pub(crate) fn getIndex(mut timeEvent: &metamodelica::Ref<TimeEvent>) -> i32 {
        let mut index: i32;
        index = (match &**timeEvent {
            SINGLE {
                index: __timeEvent_index,
                ..
            } => __timeEvent_index.clone(),
            SAMPLE {
                index: __timeEvent_index,
                ..
            } => __timeEvent_index.clone(),
        });
        index
    }

    pub(crate) fn setIndex(
        mut timeEvent: metamodelica::Ref<TimeEvent>,
        mut index: i32,
    ) -> metamodelica::Ref<TimeEvent> {
        let mut timeEvent: metamodelica::Ref<TimeEvent> = timeEvent;
        timeEvent = (match &*timeEvent {
            SINGLE { .. } => {
                assign_variant_field!(timeEvent => TimeEvent::SINGLE; index = index);
                timeEvent
            }
            SAMPLE { .. } => {
                assign_variant_field!(timeEvent => TimeEvent::SAMPLE; index = index);
                timeEvent
            }
            _ => timeEvent,
        });
        timeEvent
    }

    pub(crate) fn convert(mut timeEvent: &metamodelica::Ref<TimeEvent>) -> Result<OldBackendDAE::TimeEvent> {
        let mut oldTimeEvent: OldBackendDAE::TimeEvent;
        oldTimeEvent = (match &**timeEvent {
            SINGLE {
                index: __timeEvent_index,
                iter: __timeEvent_iter,
                trigger: __timeEvent_trigger,
            } => OldBackendDAE::TimeEvent::SAMPLE_TIME_EVENT {
                index: __timeEvent_index.clone(),
                startExp: Expression::toDAE(__timeEvent_trigger.clone(), false)?,
                intervalExp: Expression::toDAE(
                    Expression::makeMaxValue(&(openmodelica_nf_frontend::NFType::interned_REAL()))?,
                    false,
                )?,
                iter: convertEventIterator(metamodelica::AsArg::as_arg(&__timeEvent_iter))?,
            },
            SAMPLE {
                index: __timeEvent_index,
                interval: __timeEvent_interval,
                iter: __timeEvent_iter,
                start: __timeEvent_start,
            } => OldBackendDAE::TimeEvent::SAMPLE_TIME_EVENT {
                index: __timeEvent_index.clone(),
                startExp: Expression::toDAE(__timeEvent_start.clone(), false)?,
                intervalExp: Expression::toDAE(__timeEvent_interval.clone(), false)?,
                iter: convertEventIterator(metamodelica::AsArg::as_arg(&__timeEvent_iter))?,
            },
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBEvents.TimeEvent.convert"));
                        __mm_s.push_str(&*literal!(" failed."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(oldTimeEvent)
    }
}

pub mod StateEvent {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct StateEvent {
        /// index for simcode
        pub index: i32,
        /// auxiliary variable representing the relation
        pub auxiliary: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
        /// equations where the function occurs
        pub eqns:
            metamodelica::Ref<UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    }

    impl metamodelica::gc::MMTrace for StateEvent {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.auxiliary, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.eqns, __mmv)?;
            Ok(())
        }
    }
    impl Default for StateEvent {
        fn default() -> Self {
            Self {
                index: Default::default(),
                auxiliary: Default::default(),
                eqns: Default::default(),
            }
        }
    }

    pub type STATE_EVENT = StateEvent;

    pub(crate) fn toString(mut sev: &metamodelica::Ref<StateEvent>) -> Result<ArcStr> {
        let mut r#str: ArcStr = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*intString(sev.index.clone()));
            __mm_s.push_str(&*literal!(") "));
            __mm_s.push_str(&*BVariable::toString(
                &(Pointer::access(sev.auxiliary.clone())),
                literal!(""),
            )?);
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn toStringList(mut events_lst: metamodelica::List<metamodelica::Ref<StateEvent>>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = StringUtil::headline_4(&(literal!("State Events")))?;
        if (events_lst).is_empty() {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("\t<No State Events>\n"));
                ArcStr::from(__mm_s)
            };
        } else {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*stringDelimitList(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut te in (events_lst).into_iter().cloned() {
                            let __x = toString(&(te.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    literal!("\n"),
                ));
                ArcStr::from(__mm_s)
            };
        }
        Ok(r#str)
    }

    pub(crate) fn indexGt(
        mut tpl1: &(metamodelica::Ref<Condition::Condition>, metamodelica::Ref<StateEvent>),
        mut tpl2: &(metamodelica::Ref<Condition::Condition>, metamodelica::Ref<StateEvent>),
    ) -> bool {
        let mut b: bool;
        let mut sev1: metamodelica::Ref<StateEvent>;
        let mut sev2: metamodelica::Ref<StateEvent>;
        (_, sev1) = tpl1.clone();
        (_, sev2) = tpl2.clone();
        b = sev1.index.clone() > sev2.index.clone();
        b
    }

    pub(crate) fn fromStatement(
        mut stmt: metamodelica::Ref<Statement::NFStatement>,
        mut bucket_ptr: Pointer::Pointer<metamodelica::Ref<Bucket>>,
        mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
        mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
        mut funcMap: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
        >,
        mut frames: &metamodelica::List<(
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
            Option<metamodelica::Ref<Iterator::Iterator>>,
        )>,
    ) -> Result<metamodelica::Ref<Statement::NFStatement>> {
        let mut stmt: metamodelica::Ref<Statement::NFStatement> = stmt;
        stmt = (::match_deref::match_deref! { match &(stmt.clone()) {
            Deref @ Statement::ASSERT { .. } => {
                stmt
            },
            Deref @ Statement::FOR { range: Some(range), body: __stmt_body, iterator: __stmt_iterator, .. } => {
                let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut new_frames: metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Expression::NFExpression>, Option<metamodelica::Ref<Iterator::Iterator>>)>;
                let mut new_stmt: metamodelica::Ref<Statement::NFStatement>;
                let mut new_stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
                new_stmts = metamodelica::nil();
                name = ComponentRef::fromNode(__stmt_iterator.clone(), openmodelica_nf_frontend::NFType::interned_INTEGER(), metamodelica::nil(), ComponentRef::Origin::CREF.clone())?;
                name = BackendDAE::lowerComponentReference(name, variables, true)?;
                new_frames = metamodelica::cons((name, range.clone(), None), frames.clone());
                for mut elem in &*__stmt_body.clone() {
                    new_stmt = fromStatement(elem.clone(), bucket_ptr.clone(), eqn.clone(), variables, funcMap.clone(), &new_frames)?;
                    new_stmts = metamodelica::cons(new_stmt, new_stmts);
                    new_stmts = EventInfo::createAuxStatements(new_stmts, bucket_ptr.clone(), variables)?;
                }
                assign_variant_field!(stmt => Statement::NFStatement::FOR; body = new_stmts.reverse());
                stmt
            },
            _ => {
                let mut iter: metamodelica::Ref<Iterator::Iterator>;
                iter = BEquation::Iterator::fromFrames(frames.clone().reverse());
                stmt = Statement::mapExp(stmt, &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = (std::sync::Arc::new({ let __pe_b1 = bucket_ptr; let __pe_b2 = iter; let __pe_b3 = eqn; let __pe_b4 = funcMap; let __pe_b5 = false; move |__pe_a0| collectEventsTraverse(__pe_a0, __pe_b1.clone(), &__pe_b2, __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>); move |__pe_a0| Expression::fakeMap(__pe_a0, &*__pe_b1) }))?;
                stmt
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(stmt)
    }

    pub(crate) fn create(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut bucket: metamodelica::Ref<Bucket>,
        mut iter: metamodelica::Ref<Iterator::Iterator>,
        mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
        mut createEqn: bool,
    ) -> Result<(metamodelica::Ref<Expression::NFExpression>, metamodelica::Ref<Bucket>)> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        let mut bucket: metamodelica::Ref<Bucket> = bucket;
        let mut condition: metamodelica::Ref<Condition::Condition>;
        let mut sev_opt: Option<metamodelica::Ref<StateEvent>>;
        let mut sev: metamodelica::Ref<StateEvent>;
        let mut aux_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
        let mut aux_cref: metamodelica::Ref<ComponentRef::NFComponentRef> =
            openmodelica_nf_frontend::NFComponentRef::interned_EMPTY();
        let mut clocked: Pointer::Pointer<bool> = Pointer::create(false);
        (exp, bucket) = Expression::mapFold(
            exp,
            (std::sync::Arc::new({
                let __pe_b2 = iter.clone();
                let __pe_b3 = clocked.clone();
                move |__pe_a0, __pe_a1| {
                    TimeEvent::createSampleTraverse(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone())
                }
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                            metamodelica::Ref<Bucket>,
                        ) -> Result<(
                            metamodelica::Ref<Expression::NFExpression>,
                            metamodelica::Ref<Bucket>,
                        )> + 'static,
                >),
            bucket,
        )?;
        if createEqn {
            condition = metamodelica::Ref::new(Condition::Condition {
                exp: exp.clone(),
                iter: iter.clone(),
                stmt_index: 0,
            });
        } else {
            condition = metamodelica::Ref::new(Condition::Condition {
                exp: exp.clone(),
                iter: iter.clone(),
                stmt_index: bucket.stmt_index.clone(),
            });
            assign_field!(bucket.stmt_index = bucket.stmt_index.clone() + 1);
        }
        sev_opt = UnorderedMap::get(condition.clone(), bucket.state_map.clone())?;
        if (sev_opt).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(sev_opt) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            sev = metamodelica::Own::own(__pa0);
            UnorderedSet::add(eqn, sev.eqns.clone())?;
            UnorderedMap::add(condition.clone(), sev.clone(), bucket.state_map.clone())?;
            aux_cref = BVariable::getVarName(sev.auxiliary.clone());
            exp = Expression::fromCref(aux_cref.clone(), false)?;
        } else if !(Pointer::access(clocked.clone())) {
            (aux_var, aux_cref) = BVariable::makeEventVar(
                &(arcstr::literal!(BVariable::STATE_EVENT_STR)),
                UnorderedMap::size(bucket.state_map.clone()),
                Expression::typeOf(exp),
                &iter,
            )?;
            exp = Expression::fromCref(aux_cref.clone(), false)?;
            sev = metamodelica::Ref::new(StateEvent {
                index: bucket.relation_index.clone(),
                auxiliary: aux_var,
                eqns: UnorderedSet::fromList(
                    &(list![eqn]),
                    (std::sync::Arc::new(BEquation::Equation::hash)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<i32>
                                + 'static,
                        >),
                    (std::sync::Arc::new(BEquation::Equation::equalName)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
                                    Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
                                ) -> Result<bool>
                                + 'static,
                        >),
                )?,
            });
            assign_field!(bucket.relation_index = bucket.relation_index.clone() + Condition::size(&condition)?);
            condition = Condition::setRelationIndex(condition, sev.index.clone());
            UnorderedMap::add(condition.clone(), sev, bucket.state_map.clone())?;
        }
        if !(createEqn || Pointer::access(clocked)) {
            assign_field!(
                bucket.aux_stmts = Some(metamodelica::cons(
                    (condition, aux_cref),
                    Util::getOptionOrDefault(bucket.aux_stmts.clone(), metamodelica::nil())
                ))
            );
        }
        Ok((exp, bucket))
    }

    pub(crate) fn asubTuple(
        mut iter: &metamodelica::Ref<Iterator::Iterator>,
    ) -> Result<Option<(metamodelica::Ref<DAE::Exp>, i32, i32)>> {
        let mut asub: Option<(metamodelica::Ref<DAE::Exp>, i32, i32)>;
        asub = (::match_deref::match_deref! { match iter {
            Deref @ BEquation::Iterator::SINGLE { range: Deref @ Expression::RANGE { start: start @ Deref @ Expression::INTEGER { .. }, step: Some(step_exp @ Deref @ Expression::INTEGER { .. }), .. }, name: __iter_name, .. } => {
                Some((Expression::toDAE(Expression::fromCref(__iter_name.clone(), false)?, false)?, var_field!((**start).value, Expression::NFExpression::INTEGER).clone(), var_field!((**step_exp).value, Expression::NFExpression::INTEGER).clone()))
            },
            Deref @ BEquation::Iterator::SINGLE { range: Deref @ Expression::RANGE { start: start @ Deref @ Expression::INTEGER { .. }, step: None, .. }, name: __iter_name, .. } => {
                Some((Expression::toDAE(Expression::fromCref(__iter_name.clone(), false)?, false)?, var_field!((**start).value, Expression::NFExpression::INTEGER).clone(), 1))
            },
            _ => {
                None
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(asub)
    }

    pub(crate) fn convert(
        mut sev_tpl: &(metamodelica::Ref<Condition::Condition>, metamodelica::Ref<StateEvent>),
        mut equation_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Block::Block>,
            >,
        >,
    ) -> Result<OldBackendDAE::ZeroCrossing> {
        let mut oldZc: OldBackendDAE::ZeroCrossing;
        let mut cond: metamodelica::Ref<Condition::Condition>;
        let mut sev: metamodelica::Ref<StateEvent>;
        let mut iter: Option<metamodelica::List<OldSimIterator>>;
        let mut eqn_names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut eqn_indices: metamodelica::List<i32>;
        let mut relExp: metamodelica::Ref<DAE::Exp>;
        (cond, sev) = sev_tpl.clone();
        iter = convertEventIterator(&cond.iter)?;
        eqn_names = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
            for mut eqn in (UnorderedSet::toList(sev.eqns.clone())).into_iter().cloned() {
                if !(!(BEquation::Equation::isDummy(&(Pointer::access(eqn.clone()))))) {
                    continue;
                }
                let __x = BEquation::Equation::getEqnName(eqn.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        eqn_indices = ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut name in (eqn_names).into_iter().cloned() {
                if !(UnorderedMap::contains(name.clone(), equation_map.clone())?) {
                    continue;
                }
                let __x = Block::getIndex(
                    &(UnorderedMap::getSafe(
                        name.clone(),
                        equation_map.clone(),
                        metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBEvents.mo"),
                    )?),
                )?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        relExp = Expression::toDAE(cond.exp.clone(), false)?;
        relExp = (match &*relExp {
            DAE::Exp::RELATION {
                exp1: __relExp_exp1,
                exp2: __relExp_exp2,
                index: __relExp_index,
                operator: __relExp_operator,
                ..
            } => metamodelica::Ref::new(DAE::Exp::RELATION {
                exp1: __relExp_exp1.clone(),
                operator: __relExp_operator.clone(),
                exp2: __relExp_exp2.clone(),
                index: __relExp_index.clone(),
                optionExpisASUB: asubTuple(&cond.iter)?,
            }),
            _ => relExp,
        });
        oldZc = OldBackendDAE::ZeroCrossing {
            index: sev.index.clone(),
            relation_: relExp,
            occurEquLst: eqn_indices,
            iter: iter,
        };
        Ok(oldZc)
    }
}

pub mod CompositeEvent {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct CompositeEvent {
        pub index: i32,
        pub auxiliary: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    }

    impl metamodelica::gc::MMTrace for CompositeEvent {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.auxiliary, __mmv)?;
            Ok(())
        }
    }
    impl Default for CompositeEvent {
        fn default() -> Self {
            Self {
                index: Default::default(),
                auxiliary: Default::default(),
            }
        }
    }

    pub type COMPOSITE_EVENT = CompositeEvent;

    pub(crate) fn toString(mut cev: &metamodelica::Ref<CompositeEvent>) -> Result<ArcStr> {
        let mut r#str: ArcStr = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*intString(cev.index.clone()));
            __mm_s.push_str(&*literal!(") "));
            __mm_s.push_str(&*BVariable::pointerToString(cev.auxiliary.clone())?);
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn indexGt(
        mut tpl1: &(
            metamodelica::Ref<Condition::Condition>,
            metamodelica::Ref<CompositeEvent>,
        ),
        mut tpl2: &(
            metamodelica::Ref<Condition::Condition>,
            metamodelica::Ref<CompositeEvent>,
        ),
    ) -> bool {
        let mut b: bool;
        let mut cev1: metamodelica::Ref<CompositeEvent>;
        let mut cev2: metamodelica::Ref<CompositeEvent>;
        (_, cev1) = tpl1.clone();
        (_, cev2) = tpl2.clone();
        b = cev1.index.clone() > cev2.index.clone();
        b
    }

    pub(crate) fn create(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut bucket: metamodelica::Ref<Bucket>,
        mut iter: metamodelica::Ref<Iterator::Iterator>,
        mut createEqn: bool,
    ) -> Result<(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::Ref<Bucket>,
        bool,
    )> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        let mut bucket: metamodelica::Ref<Bucket> = bucket;
        let mut failed: bool = false;
        (exp, bucket, failed) = (::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ Expression::LBINARY { exp1: exp1 @ Deref @ Expression::CALL { call }, operator: Deref @ Operator::OPERATOR { op: Operator::Op::AND, .. }, exp2: __exp_exp2 } if (BackendUtil::isOnlyTimeDependent(exp1.clone())?) => {
                let mut exp2: metamodelica::Ref<Expression::NFExpression>;
                let mut exp1 = (*exp1).clone();
                let mut call = (*call).clone();
                (call, exp2, bucket, failed) = checkDirectComposite(call.clone(), __exp_exp2.clone(), bucket, iter.clone(), createEqn)?;
                if !(failed) {
                    assign_variant_field!(exp1 => Expression::NFExpression::CALL; call = call.clone());
                    assign_variant_field!(exp => Expression::NFExpression::LBINARY; exp1 = exp1.clone());
                    if !(referenceEq(&*(&*exp2),&*(var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone()))) {
                        assign_variant_field!(exp => Expression::NFExpression::LBINARY; exp2 = exp2.clone());
                    }
                }
                (exp, bucket, failed)
            },
            Deref @ Expression::LBINARY { exp2: exp2 @ Deref @ Expression::CALL { call }, operator: Deref @ Operator::OPERATOR { op: Operator::Op::AND, .. }, exp1: __exp_exp1 } if (BackendUtil::isOnlyTimeDependent(exp2.clone())?) => {
                let mut exp1: metamodelica::Ref<Expression::NFExpression>;
                let mut exp2 = (*exp2).clone();
                let mut call = (*call).clone();
                (call, exp1, bucket, failed) = checkDirectComposite(call.clone(), __exp_exp1.clone(), bucket, iter.clone(), createEqn)?;
                if !(failed) {
                    assign_variant_field!(exp2 => Expression::NFExpression::CALL; call = call.clone());
                    assign_variant_field!(exp => Expression::NFExpression::LBINARY; exp2 = exp2.clone());
                    if !(referenceEq(&*(&*exp1),&*(var_field!((*exp).exp1, Expression::NFExpression::LBINARY).clone()))) {
                        assign_variant_field!(exp => Expression::NFExpression::LBINARY; exp1 = exp1.clone());
                    }
                }
                (exp, bucket, failed)
            },
            Deref @ Expression::LBINARY { operator: Deref @ Operator::OPERATOR { op: Operator::Op::AND, .. }, exp1: __exp_exp1, .. } => {
                let mut exp1: metamodelica::Ref<Expression::NFExpression>;
                let mut exp2: metamodelica::Ref<Expression::NFExpression>;
                (exp1, bucket, failed) = create(__exp_exp1.clone(), bucket, iter.clone(), createEqn)?;
                if !(failed) {
                    assign_variant_field!(exp => Expression::NFExpression::LBINARY; exp1 = exp1.clone());
                    (exp2, bucket, failed) = create(var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone(), bucket, iter.clone(), createEqn)?;
                    if !(failed) {
                        assign_variant_field!(exp => Expression::NFExpression::LBINARY; exp2 = exp2.clone());
                    }
                    failed = false;
                } else {
                    (exp2, bucket, failed) = create(var_field!((*exp).exp2, Expression::NFExpression::LBINARY).clone(), bucket, iter.clone(), createEqn)?;
                    if !(failed) {
                        assign_variant_field!(exp => Expression::NFExpression::LBINARY; exp2 = exp2.clone());
                    }
                }
                (exp, bucket, failed)
            },
            _ => {
                (exp, bucket, true)
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        if !(failed) {
            (exp, bucket) = add(exp, iter, bucket, createEqn)?;
        }
        Ok((exp, bucket, failed))
    }

    pub(crate) fn checkDirectComposite(
        mut call: metamodelica::Ref<Call::NFCall>,
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut bucket: metamodelica::Ref<Bucket>,
        mut iter: metamodelica::Ref<Iterator::Iterator>,
        mut createEqn: bool,
    ) -> Result<(
        metamodelica::Ref<Call::NFCall>,
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::Ref<Bucket>,
        bool,
    )> {
        let mut call: metamodelica::Ref<Call::NFCall> = call;
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        let mut bucket: metamodelica::Ref<Bucket> = bucket;
        let mut failed: bool;
        let mut failed2: bool;
        (call, bucket, failed, _) = TimeEvent::createSample(call, bucket, iter.clone())?;
        if !(failed) {
            (exp, bucket, failed2) = create(exp, bucket, iter, createEqn)?;
            if !(failed2) {}
        }
        Ok((call, exp, bucket, failed))
    }

    pub(crate) fn add(
        mut cond: metamodelica::Ref<Expression::NFExpression>,
        mut iter: metamodelica::Ref<Iterator::Iterator>,
        mut bucket: metamodelica::Ref<Bucket>,
        mut createEqn: bool,
    ) -> Result<(metamodelica::Ref<Expression::NFExpression>, metamodelica::Ref<Bucket>)> {
        let mut exp: metamodelica::Ref<Expression::NFExpression>;
        let mut bucket: metamodelica::Ref<Bucket> = bucket;
        let mut condition: metamodelica::Ref<Condition::Condition>;
        let mut cev_opt: Option<metamodelica::Ref<CompositeEvent>>;
        let mut cev: metamodelica::Ref<CompositeEvent>;
        let mut aux_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
        let mut aux_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
        if createEqn {
            condition = metamodelica::Ref::new(Condition::Condition {
                exp: cond,
                iter: iter.clone(),
                stmt_index: 0,
            });
        } else {
            condition = metamodelica::Ref::new(Condition::Condition {
                exp: cond,
                iter: iter.clone(),
                stmt_index: bucket.stmt_index.clone(),
            });
            assign_field!(bucket.stmt_index = bucket.stmt_index.clone() + 1);
        }
        cev_opt = UnorderedMap::get(condition.clone(), bucket.time_map.clone())?;
        if (cev_opt).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(cev_opt) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cev = metamodelica::Own::own(__pa0);
            aux_cref = BVariable::getVarName(cev.auxiliary.clone());
            exp = Expression::fromCref(aux_cref.clone(), false)?;
        } else {
            (aux_var, aux_cref) = BVariable::makeEventVar(
                &(arcstr::literal!(BVariable::TIME_EVENT_STR)),
                UnorderedMap::size(bucket.time_map.clone()),
                Expression::typeOf(condition.exp.clone()),
                &iter,
            )?;
            exp = Expression::fromCref(aux_cref.clone(), false)?;
            cev = metamodelica::Ref::new(CompositeEvent {
                index: UnorderedMap::size(bucket.time_map.clone()),
                auxiliary: aux_var,
            });
            UnorderedMap::add(condition.clone(), cev, bucket.time_map.clone())?;
        }
        if !(createEqn) {
            assign_field!(
                bucket.aux_stmts = Some(metamodelica::cons(
                    (condition, aux_cref),
                    Util::getOptionOrDefault(bucket.aux_stmts.clone(), metamodelica::nil())
                ))
            );
        }
        Ok((exp, bucket))
    }
}

pub mod Condition {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct Condition {
        pub exp: metamodelica::Ref<Expression::NFExpression>,
        pub iter: metamodelica::Ref<Iterator::Iterator>,
        pub stmt_index: i32,
    }

    impl metamodelica::gc::MMTrace for Condition {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.exp, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.iter, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.stmt_index, __mmv)?;
            Ok(())
        }
    }
    impl Default for Condition {
        fn default() -> Self {
            Self {
                exp: Default::default(),
                iter: Default::default(),
                stmt_index: Default::default(),
            }
        }
    }

    pub type CONDITION = Condition;

    pub(crate) fn toString(mut cond: &metamodelica::Ref<Condition>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = Expression::toString(cond.exp.clone())?;
        if !(BEquation::Iterator::isEmpty(&cond.iter)) {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!(" for {"));
                __mm_s.push_str(&*BEquation::Iterator::toString(&cond.iter)?);
                __mm_s.push_str(&*literal!("}"));
                ArcStr::from(__mm_s)
            };
        }
        if !(cond.stmt_index.clone() == 0) {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(cond.stmt_index.clone()));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            };
        }
        Ok(r#str)
    }

    pub(crate) fn hash(mut cond: &metamodelica::Ref<Condition>) -> Result<i32> {
        let mut h: i32 = stringHashDjb2(&(toString(cond)?));
        Ok(h)
    }

    pub(crate) fn isEqual(
        mut cond1: &metamodelica::Ref<Condition>,
        mut cond2: &metamodelica::Ref<Condition>,
    ) -> Result<bool> {
        let mut b: bool = Expression::isEqual(cond1.exp.clone(), cond2.exp.clone())?
            && BEquation::Iterator::isEqual(&cond1.iter, &cond2.iter)?
            && cond1.stmt_index.clone() == cond2.stmt_index.clone();
        Ok(b)
    }

    pub(crate) fn size(mut cond: &metamodelica::Ref<Condition>) -> Result<i32> {
        let mut s: i32 = BEquation::Iterator::size(&cond.iter, false)?;
        Ok(s)
    }

    pub(crate) fn setRelationIndex(
        mut cond: metamodelica::Ref<Condition>,
        mut index: i32,
    ) -> metamodelica::Ref<Condition> {
        let mut cond: metamodelica::Ref<Condition> = cond;
        assign_field!(
            cond.exp = (::match_deref::match_deref! { match &(cond.exp.clone()) {
                exp @ Deref @ Expression::RELATION { .. } => {
                    let mut exp = (*exp).clone();
                    assign_variant_field!(exp => Expression::NFExpression::RELATION; index = index);
                    exp.clone()
                },
                _ => {
                    cond.exp.clone()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        );
        cond
    }
}

pub(crate) fn convertEventIterator(
    mut iter: &metamodelica::Ref<Iterator::Iterator>,
) -> Result<Option<metamodelica::List<OldSimIterator>>> {
    let mut sim_iter: Option<metamodelica::List<OldSimIterator>>;
    sim_iter = if (BEquation::Iterator::isEmpty(iter)) {
        None
    } else {
        Some(
            ({
                let mut __acc: metamodelica::List<OldSimIterator> = metamodelica::nil();
                for mut it in (SimIterator::fromIterator(iter)?).into_iter().cloned() {
                    let __x = SimIterator::convert(&(it.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        )
    };
    Ok(sim_iter)
}

pub mod SpatialDistribution {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct SpatialDistribution {
        /// uniqueIndex
        pub index: i32,
        /// input 0
        pub in0: metamodelica::Ref<Expression::NFExpression>,
        /// input 1
        pub in1: metamodelica::Ref<Expression::NFExpression>,
        /// current pos
        pub pos: metamodelica::Ref<Expression::NFExpression>,
        /// flow direction
        pub dir: metamodelica::Ref<Expression::NFExpression>,
        /// initial grid points
        pub initPnts: metamodelica::Ref<Expression::NFExpression>,
        /// initial grid values
        pub initVals: metamodelica::Ref<Expression::NFExpression>,
        /// number of initial points
        pub initSize: i32,
        /// guard condition of the enclosing if-branch, if any
        pub condition: Option<metamodelica::Ref<Expression::NFExpression>>,
    }

    impl metamodelica::gc::MMTrace for SpatialDistribution {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.in0, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.in1, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.pos, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.dir, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.initPnts, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.initVals, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.initSize, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.condition, __mmv)?;
            Ok(())
        }
    }
    impl Default for SpatialDistribution {
        fn default() -> Self {
            Self {
                index: Default::default(),
                in0: Default::default(),
                in1: Default::default(),
                pos: Default::default(),
                dir: Default::default(),
                initPnts: Default::default(),
                initVals: Default::default(),
                initSize: Default::default(),
                condition: Default::default(),
            }
        }
    }

    pub type SPATIAL_DISTRIBUTION = SpatialDistribution;

    pub(crate) fn collect(
        mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
        mut condition: Option<metamodelica::Ref<Expression::NFExpression>>,
        mut spatial_lst: Pointer::Pointer<metamodelica::List<metamodelica::Ref<SpatialDistribution>>>,
    ) -> Result<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> {
        let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>> = eqn_ptr;
        let mut eqn: metamodelica::Ref<Equation::Equation> = Pointer::access(eqn_ptr.clone());
        let mut new_eqn: metamodelica::Ref<Equation::Equation>;
        new_eqn = (match &*eqn {
            BEquation::Equation::IF_EQUATION { body: __eqn_body, .. } => {
                assign_variant_field!(eqn => Equation::Equation::IF_EQUATION; body = collectIfBody(__eqn_body.clone(), condition, spatial_lst)?);
                eqn.clone()
            }
            _ => BEquation::Equation::map(
                eqn.clone(),
                (std::sync::Arc::new({
                    let __pe_b1 = condition;
                    let __pe_b2 = spatial_lst;
                    move |__pe_a0| collectExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
                None,
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Expression::NFExpression>,
                          __a1: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >| Expression::fakeMap(__a0, metamodelica::arc_ref(&__a1)),
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                                Arc<
                                    dyn ::std::ops::Fn(
                                            metamodelica::Ref<Expression::NFExpression>,
                                        )
                                            -> Result<metamodelica::Ref<Expression::NFExpression>>
                                        + 'static,
                                >,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?,
        });
        if !(referenceEq(&*(eqn), &*(&*new_eqn))) {
            Pointer::update(eqn_ptr.clone(), new_eqn);
        }
        Ok(eqn_ptr)
    }

    pub(crate) fn collectIfBody(
        mut body: metamodelica::Ref<IfEquationBody::IfEquationBody>,
        mut condition: Option<metamodelica::Ref<Expression::NFExpression>>,
        mut spatial_lst: Pointer::Pointer<metamodelica::List<metamodelica::Ref<SpatialDistribution>>>,
    ) -> Result<metamodelica::Ref<IfEquationBody::IfEquationBody>> {
        let mut body: metamodelica::Ref<IfEquationBody::IfEquationBody> = body;
        let mut cond_true: metamodelica::Ref<Expression::NFExpression>;
        let mut cond_false: metamodelica::Ref<Expression::NFExpression>;
        (cond_true, cond_false) = updateCondition(condition, body.condition.clone());
        assign_field!(
            body.then_eqns = ({
                let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
                    metamodelica::nil();
                for mut eqn in (body.then_eqns.clone()).into_iter().cloned() {
                    let __x = collect(eqn.clone(), Some(cond_true.clone()), spatial_lst.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            body.else_if = Util::applyOption(
                body.else_if.clone(),
                &({
                    let __pe_b1 = Some(cond_false);
                    let __pe_b2 = spatial_lst;
                    move |__pe_a0| collectIfBody(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                })
            )?
        );
        Ok(body)
    }

    pub(crate) fn collectExp(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut condition: Option<metamodelica::Ref<Expression::NFExpression>>,
        mut spatial_lst: Pointer::Pointer<metamodelica::List<metamodelica::Ref<SpatialDistribution>>>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        exp = (::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ Expression::IF { condition: __exp_condition, trueBranch: __exp_trueBranch, .. } => {
                let mut cond_true: metamodelica::Ref<Expression::NFExpression>;
                let mut cond_false: metamodelica::Ref<Expression::NFExpression>;
                (cond_true, cond_false) = updateCondition(condition, __exp_condition.clone());
                assign_variant_field!(exp => Expression::NFExpression::IF;
                    trueBranch = Expression::fakeMap(__exp_trueBranch.clone(), &({ let __pe_b1 = Some(cond_true); let __pe_b2 = spatial_lst.clone(); move |__pe_a0| collectExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?,
                    falseBranch = Expression::fakeMap(var_field!((*exp).falseBranch, Expression::NFExpression::IF).clone(), &({ let __pe_b1 = Some(cond_false); let __pe_b2 = spatial_lst; move |__pe_a0| collectExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?
                );
                exp
            },
            Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::ListNode::Cons { head: in0, tail: Deref @ metamodelica::ListNode::Cons { head: in1, tail: Deref @ metamodelica::ListNode::Cons { head: pos, tail: Deref @ metamodelica::ListNode::Cons { head: dir, tail: Deref @ metamodelica::ListNode::Cons { head: initPnts @ Deref @ Expression::ARRAY { .. }, tail: Deref @ metamodelica::ListNode::Cons { head: initVals, tail: Deref @ metamodelica::ListNode::Nil } } } } } }, .. } } if (metamodelica::stringEq(&(AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL)), literal!("."), true, false)?), &(literal!("spatialDistribution")))) => {
                let mut slst: metamodelica::List<metamodelica::Ref<SpatialDistribution>>;
                let mut index: i32;
                slst = Pointer::access(spatial_lst.clone());
                index = ((slst).len() as i32);
                assign_variant_field!(exp => Expression::NFExpression::CALL; call = Call::setArguments(call.clone(), metamodelica::cons(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: index }), list![in0.clone(), in1.clone(), pos.clone(), dir.clone(), initPnts.clone(), initVals.clone()]))?);
                Pointer::update(spatial_lst, metamodelica::cons(metamodelica::Ref::new(SpatialDistribution { index: index, in0: in0.clone(), in1: in1.clone(), pos: pos.clone(), dir: dir.clone(), initPnts: initPnts.clone(), initVals: initVals.clone(), initSize: metamodelica::arrayLength(var_field!((**initPnts).elements, Expression::NFExpression::ARRAY).clone()), condition: condition }), slst));
                exp
            },
            _ => {
                Expression::mapShallow(exp, (std::sync::Arc::new({ let __pe_b1 = condition; let __pe_b2 = spatial_lst; move |__pe_a0| collectExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(exp)
    }

    pub(crate) fn updateCondition(
        mut condition: Option<metamodelica::Ref<Expression::NFExpression>>,
        mut new_cond: metamodelica::Ref<Expression::NFExpression>,
    ) -> (
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::Ref<Expression::NFExpression>,
    ) {
        let mut cond_true: metamodelica::Ref<Expression::NFExpression>;
        let mut cond_false: metamodelica::Ref<Expression::NFExpression>;
        let mut cond: metamodelica::Ref<Expression::NFExpression>;
        (cond_true, cond_false) = (::match_deref::match_deref! { match &(condition) {
            Some(__esc_cond) => {
                cond = (*__esc_cond).clone();
                (metamodelica::Ref::new(Expression::NFExpression::LBINARY { exp1: cond.clone(), operator: Operator::makeAnd(openmodelica_nf_frontend::NFType::interned_BOOLEAN()), exp2: new_cond.clone() }), metamodelica::Ref::new(Expression::NFExpression::LBINARY { exp1: cond.clone(), operator: Operator::makeAnd(openmodelica_nf_frontend::NFType::interned_BOOLEAN()), exp2: Expression::logicNegate(new_cond) }))
            },
            _ => (new_cond.clone(), Expression::logicNegate(new_cond)),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        (cond_true, cond_false)
    }

    pub(crate) fn convert(mut sd: &metamodelica::Ref<SpatialDistribution>) -> Result<OldSimCode::SpatialDistribution> {
        let mut osd: OldSimCode::SpatialDistribution;
        osd = OldSimCode::SpatialDistribution {
            index: sd.index.clone(),
            in0: Expression::toDAE(sd.in0.clone(), false)?,
            in1: Expression::toDAE(sd.in1.clone(), false)?,
            pos: Expression::toDAE(sd.pos.clone(), false)?,
            dir: Expression::toDAE(sd.dir.clone(), false)?,
            initPnts: Expression::toDAE(sd.initPnts.clone(), false)?,
            initVals: Expression::toDAE(sd.initVals.clone(), false)?,
            initSize: sd.initSize.clone(),
            condition: if ((sd.condition).is_some()) {
                Some(Expression::toDAE(Util::getOption(sd.condition.clone())?, false)?)
            } else {
                None
            },
        };
        Ok(osd)
    }
}

// =========================================================================
//                    PROTECTED UNIONTYPES AND FUNCTIONS
// =========================================================================
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Bucket {
    /// tracks compact time events (SINGLE or SAMPLE)
    pub time_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<TimeEvent::TimeEvent>>>,
    /// tracks full time events of the form $TEV_11 = ...
    pub time_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<Condition::Condition>,
            metamodelica::Ref<CompositeEvent::CompositeEvent>,
        >,
    >,
    /// tracks full state events of the form $SEV_4 = ...
    pub state_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Condition::Condition>, metamodelica::Ref<StateEvent::StateEvent>>,
    >,
    /// optional statement conditions in algorithms
    pub aux_stmts: Option<
        metamodelica::List<(
            metamodelica::Ref<Condition::Condition>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        )>,
    >,
    /// index to be used for unique statement auxiliaries
    pub stmt_index: i32,
    /// next free storedRelations[] slot; unlike state_map's
    ///        size (one entry per distinct, possibly for-loop-wrapped condition), this is incremented by
    ///        Condition.size(condition) -- the condition's scalar iteration count -- so a for-loop-wrapped
    ///        relation (e.g. v_abc[i] > a for i in 1:3) reserves one storedRelations slot per iteration
    ///        instead of all iterations colliding on a single shared slot (see StateEvent.create/convert)
    pub relation_index: i32,
}

impl metamodelica::gc::MMTrace for Bucket {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.time_set, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.time_map, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.state_map, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.aux_stmts, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stmt_index, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.relation_index, __mmv)?;
        Ok(())
    }
}
impl Default for Bucket {
    fn default() -> Self {
        Self {
            time_set: Default::default(),
            time_map: Default::default(),
            state_map: Default::default(),
            aux_stmts: Default::default(),
            stmt_index: Default::default(),
            relation_index: Default::default(),
        }
    }
}

pub type BUCKET = Bucket;

fn eventsDefault(
    mut varData: metamodelica::Ref<VarData::VarData>,
    mut eqData: metamodelica::Ref<EqData::EqData>,
    mut eventInfo: metamodelica::Ref<EventInfo::EventInfo>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
) -> Result<(
    metamodelica::Ref<VarData::VarData>,
    metamodelica::Ref<EqData::EqData>,
    metamodelica::Ref<EventInfo::EventInfo>,
)> {
    let mut varData: metamodelica::Ref<VarData::VarData> = varData;
    let mut eqData: metamodelica::Ref<EqData::EqData> = eqData;
    let mut eventInfo: metamodelica::Ref<EventInfo::EventInfo> = eventInfo;
    let mut bucket: metamodelica::Ref<Bucket> = metamodelica::Ref::new(Bucket {
        time_set: UnorderedSet::new(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<TimeEvent::TimeEvent>| TimeEvent::hash(&__a0))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<TimeEvent::TimeEvent>) -> Result<i32> + 'static,
                >),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<TimeEvent::TimeEvent>, __a1: metamodelica::Ref<TimeEvent::TimeEvent>| {
                    TimeEvent::isEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<TimeEvent::TimeEvent>,
                            metamodelica::Ref<TimeEvent::TimeEvent>,
                        ) -> Result<bool>
                        + 'static,
                >),
            13,
        ),
        time_map: UnorderedMap::new(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<Condition::Condition>| Condition::hash(&__a0))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<Condition::Condition>) -> Result<i32> + 'static,
                >),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<Condition::Condition>, __a1: metamodelica::Ref<Condition::Condition>| {
                    Condition::isEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Condition::Condition>,
                            metamodelica::Ref<Condition::Condition>,
                        ) -> Result<bool>
                        + 'static,
                >),
            1,
        ),
        state_map: UnorderedMap::new(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<Condition::Condition>| Condition::hash(&__a0))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<Condition::Condition>) -> Result<i32> + 'static,
                >),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<Condition::Condition>, __a1: metamodelica::Ref<Condition::Condition>| {
                    Condition::isEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Condition::Condition>,
                            metamodelica::Ref<Condition::Condition>,
                        ) -> Result<bool>
                        + 'static,
                >),
            1,
        ),
        aux_stmts: None,
        stmt_index: 1,
        relation_index: 0,
    });
    let mut bucket_ptr: Pointer::Pointer<metamodelica::Ref<Bucket>>;
    let mut auxiliary_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut auxiliary_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut wc_cnt: Pointer::Pointer<i32> = Pointer::create(0);
    let mut wc_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut wc_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut spatial_lst: Pointer::Pointer<
        metamodelica::List<metamodelica::Ref<SpatialDistribution::SpatialDistribution>>,
    > = Pointer::create(metamodelica::nil());
    eventInfo = (::match_deref::match_deref! { match &((varData.clone(), eqData.clone())) {
        (Deref @ BVariable::VarData::VAR_DATA_SIM { .. }, Deref @ BEquation::EqData::EQ_DATA_SIM { .. }) => {
            bucket_ptr = Pointer::create(bucket);
            BEquation::EquationPointers::mapPtr(var_field!((*eqData).simulation, EqData::EqData::EQ_DATA_SIM), &({ let __pe_b1 = bucket_ptr.clone(); let __pe_b2 = var_field!((*varData).variables, VarData::VarData::VAR_DATA_SIM).clone(); let __pe_b3 = funcMap.clone(); move |__pe_a0| collectEvents(__pe_a0, __pe_b1.clone(), &__pe_b2, __pe_b3.clone()) }))?;
            BEquation::EquationPointers::mapPtr(var_field!((*eqData).clocked, EqData::EqData::EQ_DATA_SIM), &({ let __pe_b1 = bucket_ptr.clone(); let __pe_b2 = var_field!((*varData).variables, VarData::VarData::VAR_DATA_SIM).clone(); let __pe_b3 = funcMap.clone(); move |__pe_a0| collectEvents(__pe_a0, __pe_b1.clone(), &__pe_b2, __pe_b3.clone()) }))?;
            BEquation::EquationPointers::mapPtr(var_field!((*eqData).removed, EqData::EqData::EQ_DATA_SIM), &({ let __pe_b1 = bucket_ptr.clone(); let __pe_b2 = var_field!((*varData).variables, VarData::VarData::VAR_DATA_SIM).clone(); let __pe_b3 = funcMap; move |__pe_a0| collectEvents(__pe_a0, __pe_b1.clone(), &__pe_b2, __pe_b3.clone()) }))?;
            BEquation::EquationPointers::mapPtr(var_field!((*eqData).simulation, EqData::EqData::EQ_DATA_SIM), &({ let __pe_b1 = None; let __pe_b2 = spatial_lst.clone(); move |__pe_a0| SpatialDistribution::collect(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
            bucket = Pointer::access(bucket_ptr);
            (eventInfo, auxiliary_vars, auxiliary_eqns) = EventInfo::create(&bucket, var_field!((*varData).variables, VarData::VarData::VAR_DATA_SIM), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), Pointer::access(spatial_lst))?;
            (wc_vars, wc_eqns) = simplifyWhenConditions(&(var_field!((*eqData).simulation, EqData::EqData::EQ_DATA_SIM).clone()), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), wc_cnt.clone())?;
            auxiliary_vars = listAppend(wc_vars, auxiliary_vars);
            auxiliary_eqns = listAppend(wc_eqns, auxiliary_eqns);
            (wc_vars, wc_eqns) = simplifyWhenConditions(&(var_field!((*eqData).clocked, EqData::EqData::EQ_DATA_SIM).clone()), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), wc_cnt.clone())?;
            auxiliary_vars = listAppend(wc_vars, auxiliary_vars);
            auxiliary_eqns = listAppend(wc_eqns, auxiliary_eqns);
            (wc_vars, wc_eqns) = simplifyWhenConditions(&(var_field!((*eqData).removed, EqData::EqData::EQ_DATA_SIM).clone()), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), wc_cnt)?;
            auxiliary_vars = listAppend(wc_vars, auxiliary_vars);
            auxiliary_eqns = listAppend(wc_eqns, auxiliary_eqns);
            assign_variant_field!(varData => VarData::VarData::VAR_DATA_SIM;
                variables = BVariable::VariablePointers::addList(&auxiliary_vars, var_field!((*varData).variables, VarData::VarData::VAR_DATA_SIM).clone())?,
                unknowns = BVariable::VariablePointers::addList(&auxiliary_vars, var_field!((*varData).unknowns, VarData::VarData::VAR_DATA_SIM).clone())?,
                initials = BVariable::VariablePointers::addList(&auxiliary_vars, var_field!((*varData).initials, VarData::VarData::VAR_DATA_SIM).clone())?,
                discretes = BVariable::VariablePointers::addList(&auxiliary_vars, var_field!((*varData).discretes, VarData::VarData::VAR_DATA_SIM).clone())?
            );
            assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM;
                equations = BEquation::EquationPointers::addList(&auxiliary_eqns, var_field!((*eqData).equations, EqData::EqData::EQ_DATA_SIM).clone())?,
                simulation = BEquation::EquationPointers::addList(&auxiliary_eqns, var_field!((*eqData).simulation, EqData::EqData::EQ_DATA_SIM).clone())?,
                initials = BEquation::EquationPointers::addList(&auxiliary_eqns, var_field!((*eqData).initials, EqData::EqData::EQ_DATA_SIM).clone())?,
                discretes = BEquation::EquationPointers::addList(&auxiliary_eqns, var_field!((*eqData).discretes, EqData::EqData::EQ_DATA_SIM).clone())?
            );
            eventInfo
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBEvents.eventsDefault")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((varData, eqData, eventInfo))
}

fn collectEvents(
    mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
    mut bucket_ptr: Pointer::Pointer<metamodelica::Ref<Bucket>>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
) -> Result<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> {
    let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>> = eqn_ptr;
    let mut eqn: metamodelica::Ref<Equation::Equation> = Pointer::access(eqn_ptr.clone());
    let mut body_eqn: metamodelica::Ref<Equation::Equation>;
    let mut iter: metamodelica::Ref<Iterator::Iterator>;
    let mut createEqn: bool = !(BEquation::Equation::isAlgorithm(eqn_ptr.clone()));
    let mut collector: BEquation::MapFuncExp;
    let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>;
    let mut new_stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    iter = BEquation::Equation::getForIterator(&eqn);
    collector = (std::sync::Arc::new({
        let __pe_b1 = bucket_ptr.clone();
        let __pe_b2 = iter;
        let __pe_b3 = eqn_ptr.clone();
        let __pe_b4 = funcMap.clone();
        let __pe_b5 = createEqn;
        move |__pe_a0| {
            collectEventsTraverse(
                __pe_a0,
                __pe_b1.clone(),
                &__pe_b2,
                __pe_b3.clone(),
                __pe_b4.clone(),
                __pe_b5.clone(),
            )
        }
    })
        as std::sync::Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >);
    eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::ALGORITHM { alg: __esc_alg, .. } => {
            alg = (*__esc_alg).clone();
            new_stmts = metamodelica::nil();
            for mut stmt in &*alg.statements.clone() {
                let mut stmt = stmt.clone();
                stmt = StateEvent::fromStatement(stmt, bucket_ptr.clone(), eqn_ptr.clone(), variables, funcMap.clone(), &(metamodelica::nil()))?;
                new_stmts = EventInfo::createAuxStatements(new_stmts, bucket_ptr.clone(), variables)?;
                new_stmts = metamodelica::cons(stmt, new_stmts);
            }
            assign_field!(alg.statements = new_stmts.reverse());
            assign_variant_field!(eqn => Equation::Equation::ALGORITHM; alg = Algorithm::setInputsOutputs(alg.clone())?);
            assign_variant_field!(eqn => Equation::Equation::ALGORITHM; size = ({
        let mut __acc: i32 = 0;
        for mut out in (var_field!((*eqn).alg, Equation::Equation::ALGORITHM).outputs.clone()).into_iter().cloned() {
            let __x = ComponentRef::size(&(out.clone()), true, false)?;
            __acc += __x;
        }
        __acc
    }));
            eqn
        },
        Deref @ BEquation::Equation::WHEN_EQUATION { body: __eqn_body, .. } => {
            assign_variant_field!(eqn => Equation::Equation::WHEN_EQUATION; body = BEquation::WhenEquationBody::mapCondition(__eqn_body.clone(), collector.clone(), None, (std::sync::Arc::new(move |__a0: metamodelica::Ref<Expression::NFExpression>, __a1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>| Expression::fakeMap(__a0, metamodelica::arc_ref(&__a1))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?);
            eqn
        },
        Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: __esc_body_eqn @ Deref @ BEquation::Equation::WHEN_EQUATION { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            body_eqn = (*__esc_body_eqn).clone();
            assign_variant_field!(body_eqn => Equation::Equation::WHEN_EQUATION; body = BEquation::WhenEquationBody::mapCondition(var_field!((*body_eqn).body, Equation::Equation::WHEN_EQUATION).clone(), collector.clone(), None, (std::sync::Arc::new(move |__a0: metamodelica::Ref<Expression::NFExpression>, __a1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>| Expression::fakeMap(__a0, metamodelica::arc_ref(&__a1))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?);
            assign_variant_field!(eqn => Equation::Equation::FOR_EQUATION; body = list![body_eqn.clone()]);
            eqn
        },
        Deref @ BEquation::Equation::IF_EQUATION { body: __eqn_body, .. } => {
            assign_variant_field!(eqn => Equation::Equation::IF_EQUATION; body = BEquation::IfEquationBody::mapEqnExpCref(__eqn_body.clone(), &({ let __pe_b1 = bucket_ptr; let __pe_b2 = variables.clone(); let __pe_b3 = funcMap; move |__pe_a0| collectEvents(__pe_a0, __pe_b1.clone(), &__pe_b2, __pe_b3.clone()) }), collector.clone(), None, &Expression::mapReverse)?);
            eqn
        },
        _ => BEquation::Equation::map(eqn, collector.clone(), None, (std::sync::Arc::new(move |__a0: metamodelica::Ref<Expression::NFExpression>, __a1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>| Expression::fakeMap(__a0, metamodelica::arc_ref(&__a1))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !(referenceEq(&*(&*eqn), &*(Pointer::access(eqn_ptr.clone())))) {
        Pointer::update(eqn_ptr.clone(), eqn);
    }
    Ok(eqn_ptr)
}

fn collectEventsTraverse(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut bucket_ptr: Pointer::Pointer<metamodelica::Ref<Bucket>>,
    mut iter: &metamodelica::Ref<Iterator::Iterator>,
    mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut createEqn: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::LUNARY { .. } => {
            let mut bucket: metamodelica::Ref<Bucket>;
            (exp, bucket) = collectEventsCondition(exp, Pointer::access(bucket_ptr.clone()), iter.clone(), eqn, funcMap, createEqn)?;
            Pointer::update(bucket_ptr, bucket);
            exp.clone()
        },
        Deref @ Expression::LBINARY { .. } => {
            let mut bucket: metamodelica::Ref<Bucket>;
            (exp, bucket) = collectEventsCondition(exp, Pointer::access(bucket_ptr.clone()), iter.clone(), eqn, funcMap, createEqn)?;
            Pointer::update(bucket_ptr, bucket);
            exp.clone()
        },
        Deref @ Expression::RELATION { .. } => {
            let mut bucket: metamodelica::Ref<Bucket>;
            (exp, bucket) = collectEventsCondition(exp, Pointer::access(bucket_ptr.clone()), iter.clone(), eqn, funcMap, createEqn)?;
            Pointer::update(bucket_ptr, bucket);
            exp.clone()
        },
        Deref @ Expression::CALL { call: __exp_call } if (Call::isNamed(metamodelica::AsArg::as_arg(&__exp_call), &(literal!("sample")))?) => {
            let mut bucket: metamodelica::Ref<Bucket>;
            (exp, bucket) = collectEventsCondition(exp, Pointer::access(bucket_ptr.clone()), iter.clone(), eqn, funcMap, createEqn)?;
            Pointer::update(bucket_ptr, bucket);
            exp.clone()
        },
        Deref @ Expression::CLKCONST { clk: clk @ Deref @ ClockKind::EVENT_CLOCK { condition, .. } } => {
            let mut clk = (*clk).clone();
            assign_variant_field!(clk => ClockKind::NFClockKind::EVENT_CLOCK; condition = collectEventsTraverse(condition.clone(), bucket_ptr, iter, eqn, funcMap, createEqn)?);
            assign_variant_field!(exp => Expression::NFExpression::CLKCONST; clk = clk.clone());
            exp.clone()
        },
        Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } } if (Call::isNamed(var_field!((*exp).call, Expression::NFExpression::CALL), &(literal!("pre")))?) => {
            let mut bucket: metamodelica::Ref<Bucket>;
            (exp, bucket) = CompositeEvent::add(exp, iter.clone(), Pointer::access(bucket_ptr.clone()), createEqn)?;
            Pointer::update(bucket_ptr, bucket);
            exp.clone()
        },
        Deref @ Expression::CREF { cref: __exp_cref, .. } if (BVariable::isPrevious(BVariable::getVarPointer(metamodelica::AsArg::as_arg(&__exp_cref), metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBEvents.mo"))?)) => {
            let mut bucket: metamodelica::Ref<Bucket>;
            (exp, bucket) = CompositeEvent::add(exp, iter.clone(), Pointer::access(bucket_ptr.clone()), createEqn)?;
            Pointer::update(bucket_ptr, bucket);
            exp.clone()
        },
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_REDUCTION { .. } } => {
            let mut new_frames: metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Expression::NFExpression>, Option<metamodelica::Ref<Iterator::Iterator>>)>;
            let mut call = (*call).clone();
            new_frames = ({
        let mut __acc: metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Expression::NFExpression>, Option<_>)> = metamodelica::nil();
        for mut tpl in (var_field!((*call).iters, Call::NFCall::TYPED_REDUCTION).clone()).into_iter().cloned() {
            let __x = (ComponentRef::fromNode(Util::tuple21(tpl.clone()), openmodelica_nf_frontend::NFType::interned_INTEGER(), metamodelica::nil(), ComponentRef::Origin::CREF.clone())?, Util::tuple22(tpl.clone()), None);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            assign_variant_field!(call => Call::NFCall::TYPED_REDUCTION; exp = collectEventsTraverse(var_field!((*call).exp, Call::NFCall::TYPED_REDUCTION).clone(), bucket_ptr, &(BEquation::Iterator::addFrames(iter.clone(), new_frames)), eqn, funcMap, createEqn)?);
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = call.clone());
            exp.clone()
        },
        Deref @ Expression::CALL { call: __exp_call } if (Call::isNamed(metamodelica::AsArg::as_arg(&__exp_call), &(literal!("noEvent")))?) => {
            exp.clone()
        },
        Deref @ Expression::CREF { .. } => {
            exp.clone()
        },
        _ => {
            Expression::mapShallow(exp.clone(), (std::sync::Arc::new({ let __pe_b1 = bucket_ptr; let __pe_b2 = iter.clone(); let __pe_b3 = eqn; let __pe_b4 = funcMap; let __pe_b5 = createEqn; move |__pe_a0| collectEventsTraverse(__pe_a0, __pe_b1.clone(), &__pe_b2, __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn collectEventsCondition(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut bucket: metamodelica::Ref<Bucket>,
    mut iter: metamodelica::Ref<Iterator::Iterator>,
    mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut createEqn: bool,
) -> Result<(metamodelica::Ref<Expression::NFExpression>, metamodelica::Ref<Bucket>)> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut bucket: metamodelica::Ref<Bucket> = bucket;
    let mut failed: bool = true;
    let mut original_exp: metamodelica::Ref<Expression::NFExpression>;
    if BackendUtil::isOnlyTimeDependent(exp.clone())? {
        original_exp = exp.clone();
        (exp, bucket, failed) = TimeEvent::create(exp, bucket, iter.clone(), eqn.clone(), funcMap, createEqn)?;
        if !(failed) {
            let _ = (match &*original_exp {
                Expression::RELATION { .. } => {
                    (_, bucket) = StateEvent::create(original_exp, bucket, iter.clone(), eqn.clone(), createEqn)?;
                    ()
                }
                _ => (),
            });
        }
    } else {
        if !(BackendUtil::containsContinuousVar(exp.clone())?) {
            return Ok((exp, bucket));
        }
        (exp, bucket, failed) = CompositeEvent::create(exp, bucket, iter.clone(), createEqn)?;
    }
    if failed {
        (exp, bucket) = StateEvent::create(exp, bucket, iter, eqn, createEqn)?;
    }
    Ok((exp, bucket))
}

fn simplifyWhenConditions(
    mut equations: &metamodelica::Ref<EquationPointers::EquationPointers>,
    mut idx: Pointer::Pointer<i32>,
    mut cnt: Pointer::Pointer<i32>,
) -> Result<(
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
)> {
    let mut new_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut new_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> = metamodelica::nil();
    let mut vars_ptr: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> =
        Pointer::create(metamodelica::nil());
    let mut eqns_ptr: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>> =
        Pointer::create(metamodelica::nil());
    BEquation::EquationPointers::mapPtr(
        equations,
        &({
            let __pe_b1 = idx;
            let __pe_b2 = cnt;
            let __pe_b3 = vars_ptr.clone();
            let __pe_b4 = eqns_ptr.clone();
            move |__pe_a0| {
                simplifyWhenConditionEqn(
                    __pe_a0,
                    __pe_b1.clone(),
                    __pe_b2.clone(),
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                )
            }
        }),
    )?;
    new_vars = Pointer::access(vars_ptr);
    new_eqns = Pointer::access(eqns_ptr);
    Ok((new_vars, new_eqns))
}

fn simplifyWhenConditionEqn(
    mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
    mut idx: Pointer::Pointer<i32>,
    mut cnt: Pointer::Pointer<i32>,
    mut vars_ptr: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut eqns_ptr: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
) -> Result<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> {
    let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>> = eqn_ptr;
    let mut eqn: metamodelica::Ref<Equation::Equation> = Pointer::access(eqn_ptr.clone());
    let mut body_eqn: metamodelica::Ref<Equation::Equation>;
    let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>;
    eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::WHEN_EQUATION { body: __eqn_body, .. } => {
            assign_variant_field!(eqn => Equation::Equation::WHEN_EQUATION; body = simplifyWhenConditionBody(__eqn_body.clone(), idx, cnt, vars_ptr, eqns_ptr)?);
            eqn
        },
        Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: __esc_body_eqn @ Deref @ BEquation::Equation::WHEN_EQUATION { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            body_eqn = (*__esc_body_eqn).clone();
            assign_variant_field!(body_eqn => Equation::Equation::WHEN_EQUATION; body = simplifyWhenConditionBody(var_field!((*body_eqn).body, Equation::Equation::WHEN_EQUATION).clone(), idx, cnt, vars_ptr, eqns_ptr)?);
            assign_variant_field!(eqn => Equation::Equation::FOR_EQUATION; body = list![body_eqn.clone()]);
            eqn
        },
        Deref @ BEquation::Equation::ALGORITHM { alg: __esc_alg, .. } => {
            alg = (*__esc_alg).clone();
            assign_field!(alg.statements = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
        for mut stmt in (alg.statements.clone()).into_iter().cloned() {
            let __x = simplifyWhenConditionStmt(stmt.clone(), idx.clone(), cnt.clone(), vars_ptr.clone(), eqns_ptr.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            assign_variant_field!(eqn => Equation::Equation::ALGORITHM; alg = Algorithm::setInputsOutputs(alg.clone())?);
            eqn
        },
        _ => eqn,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !(referenceEq(&*(&*eqn), &*(Pointer::access(eqn_ptr.clone())))) {
        Pointer::update(eqn_ptr.clone(), eqn);
    }
    Ok(eqn_ptr)
}

fn simplifyWhenConditionStmt(
    mut stmt: metamodelica::Ref<Statement::NFStatement>,
    mut idx: Pointer::Pointer<i32>,
    mut cnt: Pointer::Pointer<i32>,
    mut vars_ptr: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut eqns_ptr: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
) -> Result<metamodelica::Ref<Statement::NFStatement>> {
    let mut stmt: metamodelica::Ref<Statement::NFStatement> = stmt;
    stmt = ({
        let mut branches: metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
        )> = metamodelica::nil();
        (match &*stmt {
            Statement::WHEN {
                branches: __stmt_branches,
                ..
            } => {
                let mut cond: metamodelica::Ref<Expression::NFExpression>;
                let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
                for mut branch in &*__stmt_branches.clone() {
                    (cond, body) = branch.clone();
                    cond =
                        simplifyWhenConditionExp(cond, idx.clone(), cnt.clone(), vars_ptr.clone(), eqns_ptr.clone())?;
                    branches = metamodelica::cons((cond, body), branches);
                }
                assign_variant_field!(stmt => Statement::NFStatement::WHEN; branches = branches.reverse());
                stmt
            }
            Statement::IF {
                branches: __stmt_branches,
                ..
            } => {
                let mut cond: metamodelica::Ref<Expression::NFExpression>;
                let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
                for mut branch in &*__stmt_branches.clone() {
                    (cond, body) = branch.clone();
                    body = ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> =
                            metamodelica::nil();
                        for mut s in (body).into_iter().cloned() {
                            let __x = simplifyWhenConditionStmt(
                                s.clone(),
                                idx.clone(),
                                cnt.clone(),
                                vars_ptr.clone(),
                                eqns_ptr.clone(),
                            )?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    });
                    branches = metamodelica::cons((cond, body), branches);
                }
                assign_variant_field!(stmt => Statement::NFStatement::IF; branches = branches.reverse());
                stmt
            }
            _ => stmt,
        })
    });
    Ok(stmt)
}

fn simplifyWhenConditionBody(
    mut body: metamodelica::Ref<WhenEquationBody::WhenEquationBody>,
    mut idx: Pointer::Pointer<i32>,
    mut cnt: Pointer::Pointer<i32>,
    mut vars_ptr: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut eqns_ptr: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
) -> Result<metamodelica::Ref<WhenEquationBody::WhenEquationBody>> {
    let mut body: metamodelica::Ref<WhenEquationBody::WhenEquationBody> = body;
    assign_field!(
        body.condition = simplifyWhenConditionExp(
            body.condition.clone(),
            idx.clone(),
            cnt.clone(),
            vars_ptr.clone(),
            eqns_ptr.clone()
        )?,
        body.else_when = Util::applyOption(
            body.else_when.clone(),
            &({
                let __pe_b1 = idx;
                let __pe_b2 = cnt;
                let __pe_b3 = vars_ptr;
                let __pe_b4 = eqns_ptr;
                move |__pe_a0| {
                    simplifyWhenConditionBody(
                        __pe_a0,
                        __pe_b1.clone(),
                        __pe_b2.clone(),
                        __pe_b3.clone(),
                        __pe_b4.clone(),
                    )
                }
            })
        )?
    );
    Ok(body)
}

fn simplifyWhenConditionExp(
    mut cond: metamodelica::Ref<Expression::NFExpression>,
    mut idx: Pointer::Pointer<i32>,
    mut cnt: Pointer::Pointer<i32>,
    mut vars_ptr: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut eqns_ptr: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut cond: metamodelica::Ref<Expression::NFExpression> = cond;
    let mut aux_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut aux_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut aux_eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut i: i32;
    cond = (match &*cond {
        Expression::CREF { .. } => cond,
        Expression::ARRAY { .. } => {
            assign_variant_field!(cond => Expression::NFExpression::ARRAY; elements = Array::map(var_field!((*cond).elements, Expression::NFExpression::ARRAY).clone(), &({ let __pe_b1 = idx; let __pe_b2 = cnt; let __pe_b3 = vars_ptr; let __pe_b4 = eqns_ptr; move |__pe_a0| simplifyWhenConditionExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }))?);
            cond
        }
        Expression::CALL { call: __cond_call }
            if (Call::isNamed(metamodelica::AsArg::as_arg(&__cond_call), &(literal!("initial")))?) =>
        {
            cond
        }
        _ => {
            i = Pointer::access(cnt.clone());
            Pointer::update(cnt, i + 1);
            (aux_var, aux_cref) = BVariable::makeEventVar(
                &(arcstr::literal!(BVariable::WHEN_CONDITION_STR)),
                i,
                Expression::typeOf(cond.clone()),
                &(crate::NBEquation::Iterator::interned_EMPTY()),
            )?;
            aux_eqn = BEquation::Equation::makeAssignment(
                Expression::fromCref(aux_cref.clone(), false)?,
                cond,
                idx,
                &(literal!("WC")),
                crate::NBEquation::Iterator::interned_EMPTY(),
                BEquation::default(EquationKind::DISCRETE.clone(), false, None, None),
            )?;
            Pointer::update(vars_ptr.clone(), metamodelica::cons(aux_var, Pointer::access(vars_ptr)));
            Pointer::update(eqns_ptr.clone(), metamodelica::cons(aux_eqn, Pointer::access(eqns_ptr)));
            cond = Expression::fromCref(aux_cref, false)?;
            cond
        }
    });
    Ok(cond)
}

fn containsTimeTraverseExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut b: Pointer::Pointer<bool>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    if !(Pointer::access(b.clone())) && Expression::isTime(&exp)? {
        Pointer::update(b, true);
    }
    Ok(exp)
}

fn containsTimeTraverseCref(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut b: Pointer::Pointer<bool>,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    if !(Pointer::access(b.clone())) && ComponentRef::isTime(&cref)? {
        Pointer::update(b, true);
    }
    Ok(cref)
}
