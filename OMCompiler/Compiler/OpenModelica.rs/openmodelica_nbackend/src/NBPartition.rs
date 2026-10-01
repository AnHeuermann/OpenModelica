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

use crate::NBAdjacency as Adjacency;
use crate::NBEquation as BEquation;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointers;
use crate::NBJacobian as BJacobian;
use crate::NBMatching as Matching;
use crate::NBPartitioning;
use crate::NBPartitioning::BClock;
use crate::NBPartitioning::ClockedInfo;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use crate::NBackendDAE as Jacobian;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFClockKind as ClockKind;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::Pointer;

// NF imports
// Backend Imports
// Util imports
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum Kind {
    ODE = 1,
    ALG = 2,
    ODE_EVT = 3,
    ALG_EVT = 4,
    INI = 5,
    INI_0 = 6,
    DAE = 7,
    JAC = 8,
    CLK = 9,
}
impl PartialOrd for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Kind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for Kind {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for Kind {
    fn default() -> Self {
        Self::ODE
    }
}

pub mod Association {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub(crate) enum Association {
        CONTINUOUS {
            kind: Kind,
            /// Analytic jacobian for the integrator
            jacobian: Option<metamodelica::Ref<Jacobian::NBackendDAE>>,
            /// Analytic adjoint jacobian for the integrator
            jacobianAdjoint: Option<metamodelica::Ref<Jacobian::NBackendDAE>>,
            /// Analytic jacobian of Lagrange term (L), ODE (f), Path Constraints (g) for MOO
            LFG_jacobian: Option<metamodelica::Ref<Jacobian::NBackendDAE>>,
            /// Analytic jacobian of Mayer term (Mf), Final Constraints (rf) for MOO
            MRF_jacobian: Option<metamodelica::Ref<Jacobian::NBackendDAE>>,
            /// Analytic jacobian of Initial Constraints (r0) for MOO
            R0_jacobian: Option<metamodelica::Ref<Jacobian::NBackendDAE>>,
        },
        CLOCKED {
            clock: metamodelica::Ref<BClock::BClock>,
            baseClock: Option<metamodelica::Ref<BClock::BClock>>,
            /// dependencies of this clocked partition
            clock_deps: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<BClock::BClock>>>,
            holdEvents: bool,
        },
    }
    impl metamodelica::gc::MMTrace for Association {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                Association::CONTINUOUS {
                    kind,
                    jacobian,
                    jacobianAdjoint,
                    LFG_jacobian,
                    MRF_jacobian,
                    R0_jacobian,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(kind, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(jacobian, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(jacobianAdjoint, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(LFG_jacobian, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(MRF_jacobian, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(R0_jacobian, __mmv)?;
                    Ok(())
                }
                Association::CLOCKED {
                    clock,
                    baseClock,
                    clock_deps,
                    holdEvents,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(clock, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(baseClock, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(clock_deps, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(holdEvents, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Default for Association {
        fn default() -> Self {
            Self::CLOCKED {
                clock: Default::default(),
                baseClock: Default::default(),
                clock_deps: Default::default(),
                holdEvents: Default::default(),
            }
        }
    }
    pub(crate) use self::Association::{CLOCKED, CONTINUOUS};
    pub(crate) fn toStringShort(mut association: &metamodelica::Ref<Association>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = (match &**association {
            CONTINUOUS {
                kind: __association_kind,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Continuous "));
                __mm_s.push_str(&*Partition::kindToString(__association_kind.clone())?);
                ArcStr::from(__mm_s)
            }
            CLOCKED { .. } => literal!("Clocked"),
            _ => literal!("Unknown"),
        });
        Ok(r#str)
    }

    pub(crate) fn toString(mut association: &metamodelica::Ref<Association>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = (match &**association {
            CONTINUOUS {
                LFG_jacobian: __association_LFG_jacobian,
                MRF_jacobian: __association_MRF_jacobian,
                R0_jacobian: __association_R0_jacobian,
                jacobian: __association_jacobian,
                jacobianAdjoint: __association_jacobianAdjoint,
                kind: __association_kind,
            } => {
                if (__association_jacobian).is_some() {
                    r#str = BJacobian::toString(
                        &(__association_jacobian.clone().ok_or("pattern mismatch")?),
                        Partition::kindToString(__association_kind.clone())?,
                    )?;
                    if Flags::getConfigBool(Flags::MOO_DYNAMIC_OPTIMIZATION.clone())? {
                        r#str = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("\n"));
                            __mm_s.push_str(&*r#str);
                            __mm_s.push_str(&*BJacobian::toString(
                                &(__association_LFG_jacobian.clone().ok_or("pattern mismatch")?),
                                Partition::kindToString(__association_kind.clone())?,
                            )?);
                            ArcStr::from(__mm_s)
                        };
                        r#str = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("\n"));
                            __mm_s.push_str(&*r#str);
                            __mm_s.push_str(&*BJacobian::toString(
                                &(__association_MRF_jacobian.clone().ok_or("pattern mismatch")?),
                                Partition::kindToString(__association_kind.clone())?,
                            )?);
                            ArcStr::from(__mm_s)
                        };
                        r#str = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("\n"));
                            __mm_s.push_str(&*r#str);
                            __mm_s.push_str(&*BJacobian::toString(
                                &(__association_R0_jacobian.clone().ok_or("pattern mismatch")?),
                                Partition::kindToString(__association_kind.clone())?,
                            )?);
                            ArcStr::from(__mm_s)
                        };
                    }
                } else {
                    r#str = StringUtil::headline_1(&(literal!("No Jacobian")))?;
                }
                if (__association_jacobianAdjoint).is_some() {
                    r#str = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*BJacobian::toString(
                            &(__association_jacobianAdjoint.clone().ok_or("pattern mismatch")?),
                            {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*Partition::kindToString(__association_kind.clone())?);
                                __mm_s.push_str(&*literal!(" Adjoint"));
                                ArcStr::from(__mm_s)
                            },
                        )?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    };
                }
                r#str
            }
            CLOCKED {
                baseClock: __association_baseClock,
                clock: __association_clock,
                ..
            } => {
                r#str = BClock::toString(metamodelica::AsArg::as_arg(&__association_clock))?;
                if (__association_baseClock).is_some() {
                    r#str = StringUtil::headline_1(
                        &({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("Sub clock: "));
                            __mm_s.push_str(&*r#str);
                            __mm_s.push_str(&*literal!(" of base clock "));
                            __mm_s.push_str(&*BClock::toString(
                                &(__association_baseClock.clone().ok_or("pattern mismatch")?),
                            )?);
                            ArcStr::from(__mm_s)
                        }),
                    )?;
                } else {
                    r#str = StringUtil::headline_1(
                        &({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("Base clock: "));
                            __mm_s.push_str(&*r#str);
                            ArcStr::from(__mm_s)
                        }),
                    )?;
                }
                r#str
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBPartition.Association.toString"));
                        __mm_s.push_str(&*literal!(" failed. Unknown partition association in match."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(r#str)
    }

    pub(crate) fn create(
        mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
        mut kind: Kind,
        mut info: &metamodelica::Ref<ClockedInfo::ClockedInfo>,
        mut infer_del: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    ) -> Result<metamodelica::Ref<Association>> {
        let mut association: metamodelica::Ref<Association>;
        let mut clock_ptr: Pointer::Pointer<
            Option<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<BClock::BClock>,
            )>,
        > = Pointer::create(None);
        let mut infer_ptr: Pointer::Pointer<Option<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
            Pointer::create(None);
        let mut failed_set: metamodelica::Ref<
            UnorderedSet::UnorderedSet<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<BClock::BClock>,
            )>,
        > = UnorderedSet::new(
            (std::sync::Arc::new(hashClockTpl)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            (
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                                metamodelica::Ref<BClock::BClock>,
                            ),
                        ) -> Result<i32>
                        + 'static,
                >),
            (std::sync::Arc::new(isEqualClockTpl)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            (
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                                metamodelica::Ref<BClock::BClock>,
                            ),
                            (
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                                metamodelica::Ref<BClock::BClock>,
                            ),
                        ) -> Result<bool>
                        + 'static,
                >),
            13,
        );
        let mut clock_deps: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<BClock::BClock>>> =
            UnorderedSet::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<BClock::BClock>| BClock::hash(&__a0))
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BClock::BClock>) -> Result<i32> + 'static>),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<BClock::BClock>, __a1: metamodelica::Ref<BClock::BClock>| {
                        BClock::isEqual(&__a0, &__a1)
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<BClock::BClock>,
                                metamodelica::Ref<BClock::BClock>,
                            ) -> Result<bool>
                            + 'static,
                    >),
                13,
            );
        let mut clock_tpl: Option<(
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<BClock::BClock>,
        )>;
        let mut infer: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut base_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut clock: metamodelica::Ref<BClock::BClock>;
        BEquation::EquationPointers::mapExp(
            equations,
            (std::sync::Arc::new({
                let __pe_b1 = info.clone();
                let __pe_b2 = clock_ptr.clone();
                let __pe_b3 = infer_ptr.clone();
                let __pe_b4 = failed_set.clone();
                let __pe_b5 = clock_deps.clone();
                let __pe_b6 = infer_del;
                move |__pe_a0| {
                    expClocked(
                        __pe_a0,
                        &__pe_b1,
                        __pe_b2.clone(),
                        __pe_b3.clone(),
                        __pe_b4.clone(),
                        __pe_b5.clone(),
                        __pe_b6.clone(),
                    )
                }
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
            None,
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<Expression::NFExpression>,
                      __a1: Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
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
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?;
        clock_tpl = Pointer::access(clock_ptr);
        infer = Pointer::access(infer_ptr);
        if (clock_tpl).is_some() {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(clock_tpl) {
                Some((__pa0, __pa1)) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            name = metamodelica::Own::own(__pa0);
            clock = metamodelica::Own::own(__pa1);
            if !(UnorderedSet::isEmpty(failed_set.clone())) {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBPartition.Association.create"));
                        __mm_s.push_str(&*literal!(
                            " failed because there are non-identical clocks in the same partition:\n"
                        ));
                        __mm_s.push_str(&*literal!("### First clock found:\n"));
                        __mm_s.push_str(&*clockTplString((name.clone(), clock.clone()))?);
                        __mm_s.push_str(&*literal!("\n### Conflicting clocks:\n"));
                        __mm_s.push_str(&*UnorderedSet::toString(failed_set, &clockTplString, literal!("\n"))?);
                        __mm_s.push_str(&*literal!("."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
            if BClock::isBaseClock(&clock) {
                if BClock::isInferredClock(&clock) {
                    if (infer).is_none() {
                        clock = NBPartitioning::DEFAULT_BASE_CLOCK().clone();
                        UnorderedMap::add(name, clock.clone(), info.baseClocks.clone())?;
                    } else {
                        clock = metamodelica::Ref::new(BClock::BClock::INFERRED_CLOCK {
                            base_ref: infer.ok_or("pattern mismatch")?,
                        });
                    }
                }
                association = metamodelica::Ref::new(Association::CLOCKED {
                    clock: clock,
                    baseClock: None,
                    clock_deps: clock_deps,
                    holdEvents: false,
                });
            } else {
                base_name = UnorderedMap::getSafe(
                    name,
                    info.subToBase.clone(),
                    metamodelica::sourceInfo!("NBackEnd/Classes/NBPartition.mo"),
                )?;
                association = metamodelica::Ref::new(Association::CLOCKED {
                    clock: clock,
                    baseClock: Some(UnorderedMap::getSafe(
                        base_name,
                        info.baseClocks.clone(),
                        metamodelica::sourceInfo!("NBackEnd/Classes/NBPartition.mo"),
                    )?),
                    clock_deps: clock_deps,
                    holdEvents: false,
                });
            }
        } else {
            association = metamodelica::Ref::new(Association::CONTINUOUS {
                kind: kind,
                jacobian: None,
                jacobianAdjoint: None,
                LFG_jacobian: None,
                MRF_jacobian: None,
                R0_jacobian: None,
            });
        }
        Ok(association)
    }

    pub(crate) fn merge(
        mut ass1: metamodelica::Ref<Association>,
        mut ass2: &metamodelica::Ref<Association>,
        mut strict: bool,
    ) -> Result<metamodelica::Ref<Association>> {
        let mut ass1: metamodelica::Ref<Association> = ass1;
        ass1 = (::match_deref::match_deref! { match &((ass1.clone(), ass2.clone())) {
            (Deref @ CONTINUOUS { jacobian: Some(jac1 @ Deref @ Jacobian::JACOBIAN { .. }), .. }, Deref @ CONTINUOUS { jacobian: Some(jac2), .. }) if (var_field!((*ass1).kind, Association::CONTINUOUS).clone() == var_field!((**ass2).kind, Association::CONTINUOUS).clone() || !(strict)) => {
                assign_variant_field!(ass1 => Association::CONTINUOUS; jacobian = Some(BJacobian::combine(&(list![jac1.clone(), jac2.clone()]), var_field!((**jac1).name, Jacobian::NBackendDAE::JACOBIAN).clone())?));
                ass1.clone()
            },
            (Deref @ CONTINUOUS { .. }, Deref @ CONTINUOUS { .. }) if (var_field!((*ass1).kind, Association::CONTINUOUS).clone() == var_field!((**ass2).kind, Association::CONTINUOUS).clone() || !(strict)) => {
                ass1.clone()
            },
            (Deref @ CLOCKED { .. }, Deref @ CLOCKED { .. }) if (!(strict) || BClock::isEqual(var_field!((*ass1).clock, Association::CLOCKED), var_field!((**ass2).clock, Association::CLOCKED))? && Util::optionEqual(var_field!((*ass1).baseClock, Association::CLOCKED).clone(), var_field!((**ass2).baseClock, Association::CLOCKED).clone(), &move |__a0: metamodelica::Ref<BClock::BClock>, __a1: metamodelica::Ref<BClock::BClock>| BClock::isEqual(&__a0, &__a1))?) => {
                assign_variant_field!(ass1 => Association::CLOCKED;
                    clock_deps = UnorderedSet::union(var_field!((*ass1).clock_deps, Association::CLOCKED).clone(), var_field!((**ass2).clock_deps, Association::CLOCKED).clone())?,
                    holdEvents = var_field!((*ass1).holdEvents, Association::CLOCKED).clone() || var_field!((**ass2).holdEvents, Association::CLOCKED).clone()
                );
                ass1.clone()
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartition.Association.merge")); __mm_s.push_str(&*literal!(" failed. Cannot merge\n")); __mm_s.push_str(&*toString(&ass1)?); __mm_s.push_str(&*literal!(" and\n")); __mm_s.push_str(&*toString(ass2)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(ass1)
    }

    pub(crate) fn isClocked(mut association: &metamodelica::Ref<Association>) -> bool {
        let mut b: bool;
        b = (match &**association {
            CLOCKED { .. } => true,
            _ => false,
        });
        b
    }

    pub type ClockTpl = (
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<BClock::BClock>,
    );

    pub(crate) fn clockTplString(mut tpl: ClockTpl) -> Result<ArcStr> {
        let mut r#str: ArcStr = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*ComponentRef::toString(&(Util::tuple21(tpl.clone())))?);
            __mm_s.push_str(&*literal!(" = "));
            __mm_s.push_str(&*BClock::toString(&(Util::tuple22(tpl.clone())))?);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn hashClockTpl(mut tpl: ClockTpl) -> Result<i32> {
        let mut hash: i32;
        hash = ComponentRef::hash(&(Util::tuple21(tpl.clone())))?;
        hash = stringHashDjb2Continue(&(BClock::toString(&(Util::tuple22(tpl)))?), hash);
        Ok(hash)
    }

    pub(crate) fn isEqualClockTpl(mut tpl1: ClockTpl, mut tpl2: ClockTpl) -> Result<bool> {
        let mut b: bool = ComponentRef::isEqual(&(Util::tuple21(tpl1.clone())), &(Util::tuple21(tpl2.clone())))?
            && BClock::isEqual(&(Util::tuple22(tpl1.clone())), &(Util::tuple22(tpl2.clone())))?;
        Ok(b)
    }

    fn expClocked(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut info: &metamodelica::Ref<ClockedInfo::ClockedInfo>,
        mut clock_ptr: Pointer::Pointer<
            Option<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<BClock::BClock>,
            )>,
        >,
        mut infer_ptr: Pointer::Pointer<Option<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        mut failed_set: metamodelica::Ref<
            UnorderedSet::UnorderedSet<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<BClock::BClock>,
            )>,
        >,
        mut clock_deps: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<BClock::BClock>>>,
        mut infer_del: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        exp = (::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ Expression::CREF { cref: __exp_cref, .. } if (BVariable::isClockOrClocked(BVariable::getVarPointer(metamodelica::AsArg::as_arg(&__exp_cref), metamodelica::sourceInfo!("NBackEnd/Classes/NBPartition.mo"))?)) => {
                let mut clock_opt: Option<metamodelica::Ref<BClock::BClock>>;
                if UnorderedMap::contains(__exp_cref.clone(), info.baseClocks.clone())? {
                    clock_opt = Some(UnorderedMap::getSafe(__exp_cref.clone(), info.baseClocks.clone(), metamodelica::sourceInfo!("NBackEnd/Classes/NBPartition.mo"))?);
                } else if UnorderedMap::contains(__exp_cref.clone(), info.subClocks.clone())? {
                    clock_opt = Some(UnorderedMap::getSafe(__exp_cref.clone(), info.subClocks.clone(), metamodelica::sourceInfo!("NBackEnd/Classes/NBPartition.mo"))?);
                } else {
                    clock_opt = None;
                }
                let () = (::match_deref::match_deref! { match &((clock_opt, Pointer::access(clock_ptr.clone()))) {
            (Some(Deref @ BClock::BASE_CLOCK { .. }), Some((name, Deref @ BClock::SUB_CLOCK { .. }))) => {
                removeInferredClock(name.clone(), __exp_cref.clone(), info, infer_del)?;
                ()
            },
            (Some(new @ Deref @ BClock::SUB_CLOCK { .. }), Some((_, Deref @ BClock::BASE_CLOCK { .. }))) => {
                Pointer::update(clock_ptr, Some((__exp_cref.clone(), new.clone())));
                ()
            },
            (Some(new), Some((_, Deref @ BClock::BASE_CLOCK { clock: Deref @ ClockKind::INFERRED_CLOCK { .. } }))) => {
                Pointer::update(clock_ptr, Some((__exp_cref.clone(), new.clone())));
                ()
            },
            (Some(new), Some((_, old))) => {
                if BClock::isInferredClock(metamodelica::AsArg::as_arg(&old)) {
                    Pointer::update(clock_ptr, Some((__exp_cref.clone(), new.clone())));
                } else if !(BClock::isInferredClock(metamodelica::AsArg::as_arg(&new)) || BClock::isEqual(metamodelica::AsArg::as_arg(&new), metamodelica::AsArg::as_arg(&old))?) {
                    UnorderedSet::add((__exp_cref.clone(), new.clone()), failed_set)?;
                }
                ()
            },
            (Some(new), None) => {
                Pointer::update(clock_ptr, Some((__exp_cref.clone(), new.clone())));
                ()
            },
            _ => {
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                exp.clone()
            },
            Deref @ Expression::CALL { call: Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::CREF { cref: arg, .. }, tail: _ }, .. } } if (Expression::isClockOrSampleFunction(&exp)?) => {
                if UnorderedMap::contains(arg.clone(), info.subClocks.clone())? {
                    UnorderedSet::add(UnorderedMap::getSafe(arg.clone(), info.subClocks.clone(), metamodelica::sourceInfo!("NBackEnd/Classes/NBPartition.mo"))?, clock_deps)?;
                    Pointer::update(infer_ptr, Some(arg.clone()));
                }
                exp.clone()
            },
            _ => {
                Expression::mapShallow(exp.clone(), (std::sync::Arc::new({ let __pe_b1 = info.clone(); let __pe_b2 = clock_ptr; let __pe_b3 = infer_ptr; let __pe_b4 = failed_set; let __pe_b5 = clock_deps; let __pe_b6 = infer_del; move |__pe_a0| expClocked(__pe_a0, &__pe_b1, __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(exp)
    }

    fn removeInferredClock(
        mut name: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut new_name: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut info: &metamodelica::Ref<ClockedInfo::ClockedInfo>,
        mut infer_del: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    ) -> Result<()> {
        let mut base: metamodelica::Ref<BClock::BClock>;
        let mut base_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut sub_clock_names1: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut sub_clock_names2: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        base_name = UnorderedMap::getSafe(
            name,
            info.subToBase.clone(),
            metamodelica::sourceInfo!("NBackEnd/Classes/NBPartition.mo"),
        )?;
        base = UnorderedMap::getSafe(
            base_name.clone(),
            info.baseClocks.clone(),
            metamodelica::sourceInfo!("NBackEnd/Classes/NBPartition.mo"),
        )?;
        if BClock::isInferredClock(&base) {
            sub_clock_names1 = UnorderedMap::getSafe(
                base_name.clone(),
                info.baseToSub.clone(),
                metamodelica::sourceInfo!("NBackEnd/Classes/NBPartition.mo"),
            )?;
            for mut s_name in &*sub_clock_names1 {
                UnorderedMap::add(s_name.clone(), new_name.clone(), info.subToBase.clone())?;
            }
            sub_clock_names2 =
                UnorderedMap::getOrDefault(new_name.clone(), info.baseToSub.clone(), metamodelica::nil())?;
            UnorderedMap::add(
                new_name,
                listAppend(sub_clock_names1, sub_clock_names2),
                info.baseToSub.clone(),
            )?;
            UnorderedSet::add(base_name, infer_del)?;
        }
        Ok(())
    }
}

pub mod Partition {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct Partition {
        /// Partition index
        pub index: i32,
        /// Clocked/Continuous
        pub association: metamodelica::Ref<Association::Association>,
        /// Variable array of unknowns, subset of full variable array
        pub unknowns: metamodelica::Ref<VariablePointers::VariablePointers>,
        /// Variable array of unknowns in the case of dae mode
        pub daeUnknowns: Option<metamodelica::Ref<VariablePointers::VariablePointers>>,
        /// Equations array, subset of the full equation array
        pub equations: metamodelica::Ref<EquationPointers::EquationPointers>,
        /// Adjacency matrix with all additional information
        pub adjacencyMatrix: Option<metamodelica::Ref<Adjacency::Matrix::Matrix>>,
        /// Matching (see 2.5)
        pub matching: Option<metamodelica::Ref<Matching::NBMatching>>,
        /// Strong Components
        pub strongComponents: Option<metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>>,
    }

    impl metamodelica::gc::MMTrace for Partition {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.association, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.unknowns, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.daeUnknowns, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.equations, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.adjacencyMatrix, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.matching, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.strongComponents, __mmv)?;
            Ok(())
        }
    }
    impl Default for Partition {
        fn default() -> Self {
            Self {
                index: Default::default(),
                association: Default::default(),
                unknowns: Default::default(),
                daeUnknowns: Default::default(),
                equations: Default::default(),
                adjacencyMatrix: Default::default(),
                matching: Default::default(),
                strongComponents: Default::default(),
            }
        }
    }

    pub type PARTITION = Partition;

    pub(crate) fn toString(mut partition: &metamodelica::Ref<Partition>, mut level: i32) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*StringUtil::headline_2(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("("));
                    __mm_s.push_str(&*intString(partition.index.clone()));
                    __mm_s.push_str(&*literal!(") "));
                    __mm_s.push_str(&*Association::toStringShort(&partition.association)?);
                    __mm_s.push_str(&*literal!(" Partition"));
                    ArcStr::from(__mm_s)
                }),
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
        r#str = (match partition.strongComponents.clone() {
            Some(mut comps) => {
                for mut i in 1..=metamodelica::arrayLength(comps.clone()) {
                    r#str = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*StrongComponent::toString(
                            &({
                                let __elt = (*metamodelica::index_checked(&comps.borrow(), i)?).clone();
                                __elt
                            }),
                            i,
                        )?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    };
                }
                r#str
            }
            _ => {
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*BVariable::VariablePointers::toString(
                        &partition.unknowns,
                        literal!("Unknown"),
                        None,
                        true,
                    )?);
                    __mm_s.push_str(&*literal!("\n"));
                    __mm_s.push_str(&*BEquation::EquationPointers::toString(
                        &partition.equations,
                        literal!(""),
                        None,
                        true,
                        None,
                    )?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                };
                r#str
            }
        });
        if level == 1 || level == 3 {
            if (partition.adjacencyMatrix).is_some() {
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*Adjacency::Matrix::toString(
                        &(partition.adjacencyMatrix.clone().ok_or("pattern mismatch")?),
                        literal!(""),
                    )?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                };
            }
            if (partition.matching).is_some() {
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*Matching::toString(
                        &(partition.matching.clone().ok_or("pattern mismatch")?),
                        literal!(""),
                    )?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                };
            }
        }
        if level == 2 {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*Association::toString(&partition.association)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        }
        Ok(r#str)
    }

    pub(crate) fn toStringList(
        mut partitions: &metamodelica::List<metamodelica::Ref<Partition>>,
        mut header: &ArcStr,
    ) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        if !((partitions).is_empty()) {
            if !metamodelica::stringEq(&header, &(literal!(""))) {
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*StringUtil::headline_1(header)?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                };
            }
            for mut part in &**partitions {
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*toString(metamodelica::AsArg::as_arg(&part), 0)?);
                    ArcStr::from(__mm_s)
                };
            }
        }
        Ok(r#str)
    }

    pub(crate) fn sort(mut partition: metamodelica::Ref<Partition>) -> Result<metamodelica::Ref<Partition>> {
        let mut partition: metamodelica::Ref<Partition> = partition;
        assign_field!(
            partition.unknowns = BVariable::VariablePointers::sort(partition.unknowns.clone())?,
            partition.equations = BEquation::EquationPointers::sort(partition.equations.clone())?
        );
        Ok(partition)
    }

    pub(crate) fn hasIndex(mut partition: &metamodelica::Ref<Partition>, mut index: i32) -> bool {
        let mut b: bool = partition.index.clone() == index;
        b
    }

    pub(crate) fn isEmpty(mut partition: &metamodelica::Ref<Partition>) -> Result<bool> {
        use arrayEmpty as isEmptyArr;

        let mut b: bool = BEquation::EquationPointers::size(&partition.equations) == 0
            || Util::applyOptionOrDefault(
                partition.strongComponents.clone(),
                &fnptr!(
                    isEmptyArr,
                    metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>
                ),
                false,
            )?;
        Ok(b)
    }

    pub(crate) fn isODEorDAE(mut part: &metamodelica::Ref<Partition>) -> bool {
        let mut b: bool;
        b = (match &*part.association.clone() {
            Association::CONTINUOUS { kind, .. } => {
                kind.clone() == Kind::ODE.clone()
                    || kind.clone() == Kind::ODE_EVT.clone()
                    || kind.clone() == Kind::DAE.clone()
            }
            _ => false,
        });
        b
    }

    pub(crate) fn isClocked(mut part: &metamodelica::Ref<Partition>) -> bool {
        let mut b: bool;
        b = (match &*part.association.clone() {
            Association::CLOCKED { .. } => true,
            _ => false,
        });
        b
    }

    pub(crate) fn categorize(
        mut partition: metamodelica::Ref<Partition>,
        mut ode: DoubleEnded::MutableList<metamodelica::Ref<Partition>>,
        mut alg: DoubleEnded::MutableList<metamodelica::Ref<Partition>>,
        mut ode_evt: DoubleEnded::MutableList<metamodelica::Ref<Partition>>,
        mut alg_evt: DoubleEnded::MutableList<metamodelica::Ref<Partition>>,
        mut clocked: DoubleEnded::MutableList<metamodelica::Ref<Partition>>,
    ) -> Result<()> {
        fn isAlgebraicContinuous(mut part: &metamodelica::Ref<Partition>) -> Result<(bool, bool)> {
            let mut alg: bool = true;
            let mut con: bool = true;
            for mut var in &*BVariable::VariablePointers::toList(&part.unknowns)? {
                alg = if (alg) {
                    !(BVariable::isStateDerivative(var.clone()))
                } else {
                    false
                };
                con = if (con) {
                    !(BVariable::isDiscrete(var.clone()))
                } else {
                    false
                };
                if !(alg || con) {
                    break;
                }
            }
            Ok((alg, con))
        }

        let mut algebraic: bool;
        let mut continuous: bool;
        let mut kind: Kind;
        let mut association: metamodelica::Ref<Association::Association>;
        (algebraic, continuous) = isAlgebraicContinuous(&partition)?;
        kind = (match (algebraic, continuous) {
            (true, true) => Kind::ALG.clone(),
            (false, true) => Kind::ODE.clone(),
            (true, false) => Kind::ALG_EVT.clone(),
            (false, false) => Kind::ODE_EVT.clone(),
            _ => return Err("fail"),
        });
        assign_field!(
            partition.association = (::match_deref::match_deref! { match &((kind, partition.association.clone())) {
                (_, Deref @ Association::CLOCKED { .. }) => {
                    DoubleEnded::push_back(clocked, partition.clone())?;
                    partition.association.clone()
                },
                (Kind::ALG, __esc_association @ Deref @ Association::CONTINUOUS { .. }) => {
                    association = (*__esc_association).clone();
                    assign_variant_field!(association => Association::Association::CONTINUOUS; kind = kind);
                    assign_field!(partition.association = association.clone());
                    DoubleEnded::push_back(alg, partition.clone())?;
                    association.clone()
                },
                (Kind::ODE, __esc_association @ Deref @ Association::CONTINUOUS { .. }) => {
                    association = (*__esc_association).clone();
                    assign_variant_field!(association => Association::Association::CONTINUOUS; kind = kind);
                    assign_field!(partition.association = association.clone());
                    DoubleEnded::push_back(ode, partition.clone())?;
                    association.clone()
                },
                (Kind::ALG_EVT, __esc_association @ Deref @ Association::CONTINUOUS { .. }) => {
                    association = (*__esc_association).clone();
                    assign_variant_field!(association => Association::Association::CONTINUOUS; kind = kind);
                    assign_field!(partition.association = association.clone());
                    DoubleEnded::push_back(alg_evt, partition.clone())?;
                    association.clone()
                },
                (Kind::ODE_EVT, __esc_association @ Deref @ Association::CONTINUOUS { .. }) => {
                    association = (*__esc_association).clone();
                    assign_variant_field!(association => Association::Association::CONTINUOUS; kind = kind);
                    assign_field!(partition.association = association.clone());
                    DoubleEnded::push_back(ode_evt, partition.clone())?;
                    association.clone()
                },
                _ => return Err("fail"),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        );
        Ok(())
    }

    pub(crate) fn setIndex(
        mut part: metamodelica::Ref<Partition>,
        mut index: Pointer::Pointer<i32>,
    ) -> Result<metamodelica::Ref<Partition>> {
        let mut part: metamodelica::Ref<Partition> = part;
        let mut clock_idx: i32 = Pointer::access(index.clone());
        assign_field!(part.index = clock_idx);
        if isClocked(&part) {
            assign_field!(
                part.equations = BEquation::EquationPointers::map(
                    part.equations.clone(),
                    &({
                        let __pe_b1 = EquationKind::CLOCKED.clone();
                        let __pe_b2 = Some(clock_idx);
                        move |__pe_a0| BEquation::Equation::setKind(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                    })
                )?
            );
        }
        Pointer::update(index, clock_idx + 1);
        Ok(part)
    }

    pub(crate) fn setKind(
        mut part: metamodelica::Ref<Partition>,
        mut kind: Kind,
    ) -> Result<metamodelica::Ref<Partition>> {
        let mut part: metamodelica::Ref<Partition> = part;
        assign_field!(
            part.association = (::match_deref::match_deref! { match &(part.association.clone()) {
                ass @ Deref @ Association::CONTINUOUS { .. } => {
                    let mut ass = (*ass).clone();
                    assign_variant_field!(ass => Association::Association::CONTINUOUS; kind = kind);
                    ass.clone()
                },
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartition.Partition.setKind")); __mm_s.push_str(&*literal!(" failed. Cannot set kind for non-continuous partition:\n")); __mm_s.push_str(&*toString(&part, 0)?); ArcStr::from(__mm_s) }])?;
                    return Err("fail")
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        );
        Ok(part)
    }

    pub(crate) fn getJacobian(
        mut part: &metamodelica::Ref<Partition>,
    ) -> Option<metamodelica::Ref<Jacobian::NBackendDAE>> {
        let mut jac: Option<metamodelica::Ref<Jacobian::NBackendDAE>>;
        jac = (match &*part.association.clone() {
            Association::CONTINUOUS {
                jacobian: __esc_jac, ..
            } => {
                jac = (*__esc_jac).clone();
                jac.clone()
            }
            _ => None,
        });
        jac
    }

    pub(crate) fn getJacobianAdjoint(
        mut part: &metamodelica::Ref<Partition>,
    ) -> Option<metamodelica::Ref<Jacobian::NBackendDAE>> {
        let mut jac: Option<metamodelica::Ref<Jacobian::NBackendDAE>>;
        jac = (match &*part.association.clone() {
            Association::CONTINUOUS {
                jacobianAdjoint: __esc_jac,
                ..
            } => {
                jac = (*__esc_jac).clone();
                jac.clone()
            }
            _ => None,
        });
        jac
    }

    pub(crate) fn getJacobianLfg(
        mut part: &metamodelica::Ref<Partition>,
    ) -> Option<metamodelica::Ref<Jacobian::NBackendDAE>> {
        let mut jac: Option<metamodelica::Ref<Jacobian::NBackendDAE>>;
        jac = (match &*part.association.clone() {
            Association::CONTINUOUS {
                LFG_jacobian: __esc_jac,
                ..
            } => {
                jac = (*__esc_jac).clone();
                jac.clone()
            }
            _ => None,
        });
        jac
    }

    pub(crate) fn getJacobianMrf(
        mut part: &metamodelica::Ref<Partition>,
    ) -> Option<metamodelica::Ref<Jacobian::NBackendDAE>> {
        let mut jac: Option<metamodelica::Ref<Jacobian::NBackendDAE>>;
        jac = (match &*part.association.clone() {
            Association::CONTINUOUS {
                MRF_jacobian: __esc_jac,
                ..
            } => {
                jac = (*__esc_jac).clone();
                jac.clone()
            }
            _ => None,
        });
        jac
    }

    pub(crate) fn getJacobianR0(
        mut part: &metamodelica::Ref<Partition>,
    ) -> Option<metamodelica::Ref<Jacobian::NBackendDAE>> {
        let mut jac: Option<metamodelica::Ref<Jacobian::NBackendDAE>>;
        jac = (match &*part.association.clone() {
            Association::CONTINUOUS {
                R0_jacobian: __esc_jac, ..
            } => {
                jac = (*__esc_jac).clone();
                jac.clone()
            }
            _ => None,
        });
        jac
    }

    pub(crate) fn getKind(mut part: &metamodelica::Ref<Partition>) -> Kind {
        let mut kind: Kind;
        kind = (match &*part.association.clone() {
            Association::CONTINUOUS { kind: __esc_kind, .. } => {
                kind = (*__esc_kind).clone();
                kind.clone()
            }
            _ => Kind::CLK.clone(),
        });
        kind
    }

    pub(crate) fn getClocks(
        mut part: &metamodelica::Ref<Partition>,
    ) -> Result<(
        metamodelica::Ref<BClock::BClock>,
        Option<metamodelica::Ref<BClock::BClock>>,
        bool,
    )> {
        let mut clock: metamodelica::Ref<BClock::BClock>;
        let mut baseClock: Option<metamodelica::Ref<BClock::BClock>>;
        let mut holdEvents: bool;
        (clock, baseClock, holdEvents) = (match &*part.association.clone() {
            Association::CLOCKED {
                clock: __esc_clock,
                baseClock: __esc_baseClock,
                holdEvents: __esc_holdEvents,
                ..
            } => {
                clock = (*__esc_clock).clone();
                baseClock = (*__esc_baseClock).clone();
                holdEvents = (*__esc_holdEvents).clone();
                (clock.clone(), baseClock.clone(), holdEvents.clone())
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBPartition.Partition.getClocks"));
                        __mm_s.push_str(&*literal!(" failed. Cannot get clocks for continuous partition:\n"));
                        __mm_s.push_str(&*toString(part, 0)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok((clock, baseClock, holdEvents))
    }

    pub(crate) fn setClocks(
        mut part: metamodelica::Ref<Partition>,
        mut clock: metamodelica::Ref<BClock::BClock>,
        mut baseClock: Option<metamodelica::Ref<BClock::BClock>>,
    ) -> Result<metamodelica::Ref<Partition>> {
        let mut part: metamodelica::Ref<Partition> = part;
        part = (::match_deref::match_deref! { match &(part.association.clone()) {
            association @ Deref @ Association::CLOCKED { .. } => {
                let mut association = (*association).clone();
                assign_variant_field!(association => Association::Association::CLOCKED;
                    clock = clock,
                    baseClock = baseClock
                );
                assign_field!(part.association = association.clone());
                part
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartition.Partition.setClocks")); __mm_s.push_str(&*literal!(" failed. Cannot set clocks for continuous partition:\n")); __mm_s.push_str(&*toString(&part, 0)?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(part)
    }

    pub(crate) fn getClockDependencies(
        mut part: &metamodelica::Ref<Partition>,
    ) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<BClock::BClock>>>> {
        let mut clock_deps: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<BClock::BClock>>>;
        clock_deps = (match &*part.association.clone() {
            Association::CLOCKED {
                clock_deps: __esc_clock_deps,
                ..
            } => {
                clock_deps = (*__esc_clock_deps).clone();
                clock_deps.clone()
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBPartition.Partition.getClockDependencies"));
                        __mm_s.push_str(&*literal!(
                            " failed. Cannot get clock dependencies for continuous partition:\n"
                        ));
                        __mm_s.push_str(&*toString(part, 0)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(clock_deps)
    }

    pub(crate) fn getLoopResiduals(
        mut part: &metamodelica::Ref<Partition>,
    ) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
        let mut residuals: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            metamodelica::nil();
        if (part.strongComponents).is_some() {
            let __range0 = part
                .strongComponents
                .clone()
                .ok_or("pattern mismatch")?
                .borrow()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            for mut comp in __range0 {
                residuals = listAppend(StrongComponent::getLoopResiduals(&comp)?, residuals);
            }
        }
        Ok(residuals)
    }

    pub(crate) fn mapEqn(
        mut partition: metamodelica::Ref<Partition>,
        mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Equation::Equation>) -> Result<metamodelica::Ref<Equation::Equation>>,
    ) -> Result<metamodelica::Ref<Partition>> {
        pub type MapFunc = std::sync::Arc<
            dyn ::std::ops::Fn(metamodelica::Ref<Equation::Equation>) -> Result<metamodelica::Ref<Equation::Equation>>
                + 'static,
        >;

        let mut partition: metamodelica::Ref<Partition> = partition;
        assign_field!(partition.equations = BEquation::EquationPointers::map(partition.equations.clone(), func)?);
        Ok(partition)
    }

    pub(crate) fn mapExp(
        mut partition: metamodelica::Ref<Partition>,
        mut func: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >,
    ) -> Result<metamodelica::Ref<Partition>> {
        pub type MapFunc = std::sync::Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >;

        let mut partition: metamodelica::Ref<Partition> = partition;
        assign_field!(
            partition.equations = BEquation::EquationPointers::mapExp(
                partition.equations.clone(),
                func.clone(),
                None,
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
                    >)
            )?
        );
        Ok(partition)
    }

    pub(crate) fn mapStrongComponents(
        mut partition: metamodelica::Ref<Partition>,
        mut func: &dyn ::std::ops::Fn(
            metamodelica::Ref<StrongComponent::NBStrongComponent>,
        ) -> Result<metamodelica::Ref<StrongComponent::NBStrongComponent>>,
    ) -> Result<metamodelica::Ref<Partition>> {
        pub type MapFunc = std::sync::Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<StrongComponent::NBStrongComponent>,
                ) -> Result<metamodelica::Ref<StrongComponent::NBStrongComponent>>
                + 'static,
        >;

        let mut partition: metamodelica::Ref<Partition> = partition;
        let mut comps: metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
        if (partition.strongComponents).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(partition.strongComponents.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            comps = metamodelica::Own::own(__pa0);
            for mut i in 1..=metamodelica::arrayLength(comps.clone()) {
                {
                    let __cell1 = func(
                        ({
                            let __elt = (*metamodelica::index_checked(&comps.borrow(), i)?).clone();
                            __elt
                        }),
                    )?;
                    let __idx1 = i;
                    *metamodelica::index_mut_checked(&mut comps.clone().borrow_mut(), __idx1)? = __cell1;
                }
            }
            assign_field!(partition.strongComponents = Some(comps.clone()));
        }
        Ok(partition)
    }

    pub(crate) fn kindToString(mut kind: Kind) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        r#str = (match kind {
            Kind::ODE => literal!("ODE"),
            Kind::ALG => literal!("ALG"),
            Kind::ODE_EVT => literal!("ODE_EVT"),
            Kind::ALG_EVT => literal!("ALG_EVT"),
            Kind::INI => literal!("INI"),
            Kind::INI_0 => literal!("INI_0"),
            Kind::DAE { .. } => literal!("DAE"),
            Kind::JAC => literal!("JAC"),
            Kind::CLK => literal!("CLK"),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBPartition.Partition.kindToString"));
                        __mm_s.push_str(&*literal!(" failed. Unknown partition kind in match."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(r#str)
    }

    pub(crate) fn kindToInteger(mut kind: Kind) -> Result<i32> {
        let mut i: i32;
        i = (match kind {
            Kind::ODE => 0,
            Kind::ALG => 1,
            Kind::ODE_EVT => 2,
            Kind::ALG_EVT => 3,
            Kind::INI => 4,
            Kind::INI_0 => 5,
            Kind::DAE { .. } => 6,
            Kind::JAC => 7,
            Kind::CLK => 8,
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBPartition.Partition.kindToInteger"));
                        __mm_s.push_str(&*literal!(" failed. Unknown partition kind in match."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(i)
    }

    pub(crate) fn clone(
        mut par: metamodelica::Ref<Partition>,
        mut shallow: bool,
    ) -> Result<metamodelica::Ref<Partition>> {
        let mut par: metamodelica::Ref<Partition> = par;
        assign_field!(par.equations = BEquation::EquationPointers::clone(&par.equations, shallow)?);
        if !(shallow) {
            assign_field!(
                par.adjacencyMatrix = None,
                par.matching = None,
                par.strongComponents = None
            );
            assign_field!(
                par.association = (::match_deref::match_deref! { match &(par.association.clone()) {
                    association @ Deref @ Association::CONTINUOUS { .. } => {
                        let mut association = (*association).clone();
                        assign_variant_field!(association => Association::Association::CONTINUOUS; jacobian = None);
                        association.clone()
                    },
                    _ => {
                        par.association.clone()
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } })
            );
        }
        Ok(par)
    }

    pub(crate) fn removeAlias(mut par: metamodelica::Ref<Partition>) -> Result<metamodelica::Ref<Partition>> {
        let mut par: metamodelica::Ref<Partition> = par;
        let mut comps: metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
        if (par.strongComponents).is_some() {
            comps = par.strongComponents.clone().ok_or("pattern mismatch")?;
            for mut i in 1..=metamodelica::arrayLength(comps.clone()) {
                {
                    let __cell0 = StrongComponent::removeAlias(
                        ({
                            let __elt = (*metamodelica::index_checked(&comps.borrow(), i)?).clone();
                            __elt
                        }),
                    );
                    let __idx0 = i;
                    *metamodelica::index_mut_checked(&mut comps.clone().borrow_mut(), __idx0)? = __cell0;
                }
            }
        }
        Ok(par)
    }

    pub(crate) fn updateHeldVars(
        mut par: metamodelica::Ref<Partition>,
        mut held_crefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    ) -> Result<metamodelica::Ref<Partition>> {
        let mut par: metamodelica::Ref<Partition> = par;
        assign_field!(
            par.association = (::match_deref::match_deref! { match &(par.association.clone()) {
                association @ Deref @ Association::CLOCKED { .. } => {
                    let mut association = (*association).clone();
                    assign_variant_field!(association => Association::Association::CLOCKED; holdEvents = !(UnorderedSet::isDisjoint(held_crefs, UnorderedMap::keySet(par.unknowns.map.clone())?)?));
                    association.clone()
                },
                _ => {
                    par.association.clone()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        );
        Ok(par)
    }

    pub(crate) fn merge(
        mut part1: metamodelica::Ref<Partition>,
        mut part2: &metamodelica::Ref<Partition>,
        mut strict: bool,
    ) -> Result<metamodelica::Ref<Partition>> {
        let mut part1: metamodelica::Ref<Partition> = part1;
        if (part1.daeUnknowns).is_some() || (part2.daeUnknowns).is_some() {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBPartition.Partition.merge"));
                    __mm_s.push_str(&*literal!(" failed. Cannot merge DAE-Mode partitions."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        } else if (part1.strongComponents).is_some() || (part2.strongComponents).is_some() {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBPartition.Partition.merge"));
                    __mm_s.push_str(&*literal!(" failed. Should not merge sorted partitions."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        } else if (part1.matching).is_some() || (part2.matching).is_some() {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBPartition.Partition.merge"));
                    __mm_s.push_str(&*literal!(" failed. Should not merge matched partitions."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        } else if (part1.adjacencyMatrix).is_some() || (part2.adjacencyMatrix).is_some() {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBPartition.Partition.merge"));
                    __mm_s.push_str(&*literal!(
                        " failed. Should not merge partitions with adjacency matrix."
                    ));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
        assign_field!(
            part1.association = Association::merge(part1.association.clone(), &part2.association, strict)?,
            part1.unknowns = BVariable::VariablePointers::addList(
                &(BVariable::VariablePointers::toList(&part2.unknowns)?),
                part1.unknowns.clone()
            )?,
            part1.equations = BEquation::EquationPointers::addList(
                &(BEquation::EquationPointers::toList(&part2.equations)?),
                part1.equations.clone()
            )?
        );
        Ok(part1)
    }
}

pub(crate) fn kindIsInitial(mut kind: Kind) -> bool {
    let mut b: bool = kind == Kind::INI.clone() || kind == Kind::INI_0.clone();
    b
}
