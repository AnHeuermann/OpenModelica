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
use crate::NBBackendUtil;
use crate::NBCausalize as Causalize;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::WhenEquationBody;
use crate::NBEquation::WhenStatement;
use crate::NBMatching as Matching;
use crate::NBModule as Module;
use crate::NBPartition as Partition;
use crate::NBSorting as Sorting;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_backend_types::BackendDAE as OldBackendDAE;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE as OldDAE;
use openmodelica_nf_frontend::NFBackendExtension::BackendInfo;
use openmodelica_nf_frontend::NFBackendExtension::VariableKind;
use openmodelica_nf_frontend::NFBuiltinFuncs;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFClockKind as ClockKind;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFPrefixes;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Rational;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;
use openmodelica_util_datatypes_basic::PointerWeak;

// NF
// Backend
// Util
// Old imports
pub mod BClock {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum BClock {
        BASE_CLOCK {
            clock: metamodelica::Ref<ClockKind::NFClockKind>,
        },
        SUB_CLOCK {
            factor: metamodelica::Ref<Rational::Rational>,
            shift: metamodelica::Ref<Rational::Rational>,
            solver: Option<ArcStr>,
        },
        INFERRED_CLOCK {
            base_ref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        },
    }
    impl metamodelica::gc::MMTrace for BClock {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                BClock::BASE_CLOCK { clock } => {
                    metamodelica::gc::MMTrace::mm_accept(clock, __mmv)?;
                    Ok(())
                }
                BClock::SUB_CLOCK { factor, shift, solver } => {
                    metamodelica::gc::MMTrace::mm_accept(factor, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(shift, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(solver, __mmv)?;
                    Ok(())
                }
                BClock::INFERRED_CLOCK { base_ref } => {
                    metamodelica::gc::MMTrace::mm_accept(base_ref, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Default for BClock {
        fn default() -> Self {
            Self::BASE_CLOCK {
                clock: Default::default(),
            }
        }
    }
    pub use self::BClock::{BASE_CLOCK, INFERRED_CLOCK, SUB_CLOCK};
    pub(crate) fn toString(mut clock: &metamodelica::Ref<BClock>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = (match &**clock {
            BASE_CLOCK { clock: __clock_clock } => {
                ClockKind::toDebugString(metamodelica::AsArg::as_arg(&__clock_clock))?
            }
            SUB_CLOCK {
                factor: __clock_factor,
                shift: __clock_shift,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("SUB_CLOCK("));
                __mm_s.push_str(&*Rational::toString(metamodelica::AsArg::as_arg(&__clock_factor)));
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*Rational::toString(metamodelica::AsArg::as_arg(&__clock_shift)));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
            INFERRED_CLOCK {
                base_ref: __clock_base_ref,
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("INFERRED_CLOCK("));
                __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(
                    &__clock_base_ref,
                ))?);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
            _ => literal!("UNKNOWN_CLOCK()"),
        });
        Ok(r#str)
    }

    pub(crate) fn hash(mut clock: &metamodelica::Ref<BClock>) -> Result<i32> {
        let mut i: i32 = stringHashDjb2(&(toString(clock)?));
        Ok(i)
    }

    pub(crate) fn isEqual(
        mut clock1: &metamodelica::Ref<BClock>,
        mut clock2: &metamodelica::Ref<BClock>,
    ) -> Result<bool> {
        let mut b: bool;
        b = (::match_deref::match_deref! { match (clock1, clock2) {
            (Deref @ BASE_CLOCK { .. }, Deref @ BASE_CLOCK { .. }) => ClockKind::compare(var_field!((**clock1).clock, BClock::BASE_CLOCK).clone(), var_field!((**clock2).clock, BClock::BASE_CLOCK).clone())? == 0,
            (Deref @ SUB_CLOCK { .. }, Deref @ SUB_CLOCK { .. }) => Rational::isEqual(var_field!((**clock1).factor, BClock::SUB_CLOCK), var_field!((**clock2).factor, BClock::SUB_CLOCK)) && Rational::isEqual(var_field!((**clock1).shift, BClock::SUB_CLOCK), var_field!((**clock2).shift, BClock::SUB_CLOCK)) && Util::optionEqual(var_field!((**clock1).solver, BClock::SUB_CLOCK).clone(), var_field!((**clock2).solver, BClock::SUB_CLOCK).clone(), &fnptr!(stringEq, ArcStr, ArcStr))?,
            (Deref @ INFERRED_CLOCK { .. }, Deref @ INFERRED_CLOCK { .. }) => ComponentRef::isEqual(var_field!((**clock1).base_ref, BClock::INFERRED_CLOCK), var_field!((**clock2).base_ref, BClock::INFERRED_CLOCK))?,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(b)
    }

    pub(crate) fn add(
        mut eqn: metamodelica::Ref<Equation::Equation>,
        mut info: &metamodelica::Ref<ClockedInfo::ClockedInfo>,
    ) -> Result<()> {
        let () = (::match_deref::match_deref! { match &((BEquation::Equation::getLHS(eqn.clone())?, BEquation::Equation::getRHS(eqn)?)) {
            (Some(Deref @ Expression::CREF { cref: clock_name, .. }), Some(exp)) if (Expression::isClockOrSampleFunction(metamodelica::AsArg::as_arg(&exp))?) => {
                create(clock_name.clone(), metamodelica::AsArg::as_arg(&exp), info)?;
                ()
            },
            (Some(exp), Some(Deref @ Expression::CREF { cref: clock_name, .. })) if (Expression::isClockOrSampleFunction(metamodelica::AsArg::as_arg(&exp))?) => {
                create(clock_name.clone(), metamodelica::AsArg::as_arg(&exp), info)?;
                ()
            },
            _ => {
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(())
    }

    pub(crate) fn isBaseClock(mut clock: &metamodelica::Ref<BClock>) -> bool {
        let mut b: bool;
        b = (match &**clock {
            BASE_CLOCK { .. } => true,
            _ => false,
        });
        b
    }

    pub(crate) fn isInferredClock(mut clock: &metamodelica::Ref<BClock>) -> bool {
        let mut b: bool;
        b = (::match_deref::match_deref! { match clock {
            Deref @ BASE_CLOCK { clock: Deref @ ClockKind::INFERRED_CLOCK { .. } } => true,
            Deref @ INFERRED_CLOCK { .. } => true,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        b
    }

    pub(crate) fn isEventClock(mut clock: &metamodelica::Ref<BClock>) -> bool {
        let mut b: bool;
        b = (::match_deref::match_deref! { match clock {
            Deref @ BASE_CLOCK { clock: Deref @ ClockKind::EVENT_CLOCK { .. } } => true,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        b
    }

    pub(crate) fn baseClockInferrence(
        mut clock: metamodelica::Ref<BClock>,
        mut base_clock_inferrence: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<BClock>>,
        >,
    ) -> Result<metamodelica::Ref<BClock>> {
        '__tco: loop {
            ::match_deref::match_deref! { match &(clock.clone()) {
                Deref @ INFERRED_CLOCK { base_ref: __clock_base_ref } => {
                    let mut base_clock: metamodelica::Ref<BClock>;
                    base_clock = UnorderedMap::getSafe(__clock_base_ref.clone(), base_clock_inferrence.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?;
                    { (clock, base_clock_inferrence) = (base_clock, base_clock_inferrence); continue '__tco; }
                },
                Deref @ BASE_CLOCK { clock: Deref @ ClockKind::INFERRED_CLOCK { .. } } => {
                    return Ok(DEFAULT_BASE_CLOCK().clone())
                },
                _ => {
                    return Ok(clock)
                },
                _ => return Err("match: no arm matched"),
            } }
        }
    }

    pub(crate) fn convertBase(mut clock: &metamodelica::Ref<BClock>) -> Result<metamodelica::Ref<OldDAE::ClockKind>> {
        let mut oldClock: metamodelica::Ref<OldDAE::ClockKind>;
        oldClock = (match &**clock {
            BASE_CLOCK { clock: __clock_clock } => ClockKind::toDAE(metamodelica::AsArg::as_arg(&__clock_clock))?,
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBPartitioning.BClock.convertBase"));
                        __mm_s.push_str(&*literal!(" failed for non-base clock: "));
                        __mm_s.push_str(&*toString(clock)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(oldClock)
    }

    pub(crate) fn convertSub(mut clock: &metamodelica::Ref<BClock>) -> Result<OldBackendDAE::SubClock> {
        let mut oldClock: OldBackendDAE::SubClock;
        oldClock = (match &**clock {
            SUB_CLOCK {
                factor: __clock_factor,
                shift: __clock_shift,
                solver: __clock_solver,
            } => OldBackendDAE::SubClock::SUBCLOCK {
                factor: NBBackendUtil::convertRational(metamodelica::AsArg::as_arg(&__clock_factor)),
                shift: NBBackendUtil::convertRational(metamodelica::AsArg::as_arg(&__clock_shift)),
                solver: __clock_solver.clone(),
            },
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBPartitioning.BClock.convertSub"));
                        __mm_s.push_str(&*literal!(" failed for non-sub clock: "));
                        __mm_s.push_str(&*toString(clock)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(oldClock)
    }

    pub(crate) fn toExp(mut clock: &metamodelica::Ref<BClock>) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut exp: metamodelica::Ref<Expression::NFExpression>;
        exp = (match &**clock {
            BASE_CLOCK { clock: __clock_clock } => metamodelica::Ref::new(Expression::NFExpression::CLKCONST {
                clk: __clock_clock.clone(),
            }),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBPartitioning.BClock.toExp"));
                        __mm_s.push_str(&*literal!(" failed for non-base clock: "));
                        __mm_s.push_str(&*toString(clock)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(exp)
    }

    fn create(
        mut clock_name: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut exp: &metamodelica::Ref<Expression::NFExpression>,
        mut info: &metamodelica::Ref<ClockedInfo::ClockedInfo>,
    ) -> Result<()> {
        let mut clock: metamodelica::Ref<BClock>;
        let mut baseClock: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut clock_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
        match '__try0: {
            (clock, baseClock) = unwrap_break_err!(fromExp(exp), '__try0);
            if (baseClock).is_some() {
                unwrap_break_err!(UnorderedMap::add(clock_name.clone(), clock.clone(), info.subClocks.clone()), '__try0);
                unwrap_break_err!(UnorderedMap::add(clock_name.clone(), unwrap_break_err!(Util::getOption(baseClock.clone()), '__try0), info.subToBase.clone()), '__try0);
            } else {
                unwrap_break_err!(UnorderedMap::add(clock_name.clone(), clock.clone(), info.baseClocks.clone()), '__try0);
            }
            clock_var = unwrap_break_err!(BVariable::getVarPointer(&clock_name, metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo")), '__try0);
            if !(BVariable::isClockOrClocked(clock_var.clone())) {
                BVariable::setVarKind(
                    clock_var.clone(),
                    openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_CLOCKED(),
                );
            }
            Ok::<_, &'static str>((baseClock.clone(), clock.clone(), clock_var.clone()))
        } {
            Ok((__try0_o0, __try0_o1, __try0_o2)) => {
                baseClock = __try0_o0;
                clock = __try0_o1;
                clock_var = __try0_o2;
            }
            Err(__try0_err) => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBPartitioning.BClock.create"));
                        __mm_s.push_str(&*literal!(" failed for "));
                        __mm_s.push_str(&*ComponentRef::toString(&clock_name)?);
                        __mm_s.push_str(&*literal!("."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err(__try0_err);
            }
        }
        Ok(())
    }

    fn fromExp(
        mut exp: &metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<(
        metamodelica::Ref<BClock>,
        Option<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    )> {
        let mut subClock: metamodelica::Ref<BClock>;
        let mut baseClock: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        (subClock, baseClock) = (::match_deref::match_deref! { match exp {
            Deref @ Expression::CLKCONST { clk: __exp_clk } => {
                (metamodelica::Ref::new(BClock::BASE_CLOCK { clock: __exp_clk.clone() }), None)
            },
            Deref @ Expression::CREF { cref: __exp_cref, .. } => {
                (DEFAULT_SUB_CLOCK().clone(), Some(__exp_cref.clone()))
            },
            Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
                (baseClock, subClock) = (::match_deref::match_deref! { match &((AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL)), literal!("."), true, false)?, Call::arguments(metamodelica::AsArg::as_arg(&call))?)) {
            (Deref @ "sample", Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                (subClock, baseClock) = fromExp(metamodelica::AsArg::as_arg(&e))?;
                (baseClock, subClock)
            },
            (Deref @ "subSample", Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                (subClock, baseClock) = fromExp(metamodelica::AsArg::as_arg(&e))?;
                subClock = updateSubClock(subClock, &(metamodelica::Ref::new(BClock::SUB_CLOCK { factor: metamodelica::Ref::new(Rational::Rational { n: i1.clone(), d: 1 }), shift: Rational::ZERO.clone(), solver: None })))?;
                (baseClock, subClock)
            },
            (Deref @ "superSample", Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                (subClock, baseClock) = fromExp(metamodelica::AsArg::as_arg(&e))?;
                subClock = updateSubClock(subClock, &(metamodelica::Ref::new(BClock::SUB_CLOCK { factor: metamodelica::Ref::new(Rational::Rational { n: 1, d: i1.clone() }), shift: Rational::ZERO.clone(), solver: None })))?;
                (baseClock, subClock)
            },
            (Deref @ "shiftSample", Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                (subClock, baseClock) = fromExp(metamodelica::AsArg::as_arg(&e))?;
                subClock = updateSubClock(subClock, &(metamodelica::Ref::new(BClock::SUB_CLOCK { factor: Rational::ONE.clone(), shift: metamodelica::Ref::new(Rational::Rational { n: i1.clone(), d: 1 }), solver: None })))?;
                (baseClock, subClock)
            },
            (Deref @ "shiftSample", Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: i2 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                (subClock, baseClock) = fromExp(metamodelica::AsArg::as_arg(&e))?;
                subClock = updateSubClock(subClock, &(metamodelica::Ref::new(BClock::SUB_CLOCK { factor: Rational::ONE.clone(), shift: metamodelica::Ref::new(Rational::Rational { n: i1.clone(), d: i2.clone() }), solver: None })))?;
                (baseClock, subClock)
            },
            (Deref @ "backSample", Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
                (subClock, baseClock) = fromExp(metamodelica::AsArg::as_arg(&e))?;
                subClock = updateSubClock(subClock, &(metamodelica::Ref::new(BClock::SUB_CLOCK { factor: Rational::ONE.clone(), shift: metamodelica::Ref::new(Rational::Rational { n: -(i1.clone()), d: 1 }), solver: None })))?;
                (baseClock, subClock)
            },
            (Deref @ "backSample", Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: i1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::INTEGER { value: i2 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
                (subClock, baseClock) = fromExp(metamodelica::AsArg::as_arg(&e))?;
                subClock = updateSubClock(subClock, &(metamodelica::Ref::new(BClock::SUB_CLOCK { factor: Rational::ONE.clone(), shift: metamodelica::Ref::new(Rational::Rational { n: -(i1.clone()), d: i2.clone() }), solver: None })))?;
                (baseClock, subClock)
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.BClock.fromExp")); __mm_s.push_str(&*literal!(" failed for exp with unhandled call: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                (subClock, baseClock)
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.BClock.fromExp")); __mm_s.push_str(&*literal!(" failed for exp with unhandled expression kind: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok((subClock, baseClock))
    }

    pub(crate) fn updateSubClock(
        mut dest: metamodelica::Ref<BClock>,
        mut src: &metamodelica::Ref<BClock>,
    ) -> Result<metamodelica::Ref<BClock>> {
        let mut dest: metamodelica::Ref<BClock> = dest;
        dest = (::match_deref::match_deref! { match &((dest.clone(), src.clone())) {
            (Deref @ SUB_CLOCK { .. }, Deref @ SUB_CLOCK { .. }) => {
                assign_variant_field!(dest => BClock::SUB_CLOCK;
                    shift = Rational::add(&(var_field!((*dest).shift, BClock::SUB_CLOCK).clone()), &(Rational::mul(var_field!((**src).shift, BClock::SUB_CLOCK), var_field!((*dest).factor, BClock::SUB_CLOCK)))),
                    factor = Rational::mul(var_field!((*dest).factor, BClock::SUB_CLOCK), var_field!((**src).factor, BClock::SUB_CLOCK))
                );
                dest
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.BClock.updateSubClock")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*toString(&dest)?); __mm_s.push_str(&*literal!(" and ")); __mm_s.push_str(&*toString(src)?); __mm_s.push_str(&*literal!(" because of incorrect clock types.")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(dest)
    }
}

thread_local! { static __DEFAULT_BASE_CLOCK_TLS: metamodelica::Ref<BClock::BClock> = metamodelica::Ref::new(BClock::BClock::BASE_CLOCK { clock: metamodelica::Ref::new(ClockKind::NFClockKind::REAL_CLOCK { interval: metamodelica::Ref::new(Expression::NFExpression::REAL { value: metamodelica::OrderedFloat(1.0_f64) }) }) }); }
pub(crate) fn DEFAULT_BASE_CLOCK() -> metamodelica::Ref<BClock::BClock> {
    __DEFAULT_BASE_CLOCK_TLS.with(|__t| __t.clone())
}

thread_local! { static __DEFAULT_SUB_CLOCK_TLS: metamodelica::Ref<BClock::BClock> = metamodelica::Ref::new(BClock::BClock::SUB_CLOCK { factor: Rational::ONE.clone(), shift: Rational::ZERO.clone(), solver: None }); }
pub(crate) fn DEFAULT_SUB_CLOCK() -> metamodelica::Ref<BClock::BClock> {
    __DEFAULT_SUB_CLOCK_TLS.with(|__t| __t.clone())
}

pub type CrefLst = metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;

pub mod ClockedInfo {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct ClockedInfo {
        pub baseClocks: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<BClock::BClock>,
            >,
        >,
        pub subClocks: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<BClock::BClock>,
            >,
        >,
        pub subToBase: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<ComponentRef::NFComponentRef>,
            >,
        >,
        pub baseToSub: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            >,
        >,
    }

    impl metamodelica::gc::MMTrace for ClockedInfo {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.baseClocks, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.subClocks, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.subToBase, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.baseToSub, __mmv)?;
            Ok(())
        }
    }
    pub type CLOCKED_INFO = ClockedInfo;

    pub(crate) fn new() -> metamodelica::Ref<ClockedInfo> {
        let mut info: metamodelica::Ref<ClockedInfo> = metamodelica::Ref::new(ClockedInfo {
            baseClocks: UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
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
            ),
            subClocks: UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
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
            ),
            subToBase: UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
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
            ),
            baseToSub: UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
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
            ),
        });
        info
    }

    pub(crate) fn toString(mut info: &metamodelica::Ref<ClockedInfo>) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        if !(isEmpty(info)) {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*StringUtil::headline_2(&(literal!("Clocked Info")))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*StringUtil::headline_3(&(literal!("Base Clocks")))?);
                __mm_s.push_str(&*UnorderedMap::toString(
                    info.baseClocks.clone(),
                    &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                    &move |__a0: metamodelica::Ref<BClock::BClock>| BClock::toString(&__a0),
                    literal!("\n"),
                    &(literal!(", ")),
                )?);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*StringUtil::headline_3(&(literal!("Sub Clocks")))?);
                __mm_s.push_str(&*UnorderedMap::toString(
                    info.subClocks.clone(),
                    &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                    &move |__a0: metamodelica::Ref<BClock::BClock>| BClock::toString(&__a0),
                    literal!("\n"),
                    &(literal!(", ")),
                )?);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*StringUtil::headline_3(&(literal!("Sub to Base Clocks")))?);
                __mm_s.push_str(&*UnorderedMap::toString(
                    info.subToBase.clone(),
                    &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                    &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                    literal!("\n"),
                    &(literal!(", ")),
                )?);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*StringUtil::headline_3(&(literal!("Base to Sub Clocks")))?);
                __mm_s.push_str(&*UnorderedMap::toString(
                    info.baseToSub.clone(),
                    &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                    &ComponentRef::listToString,
                    literal!("\n"),
                    &(literal!(", ")),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        }
        Ok(r#str)
    }

    pub(crate) fn isEmpty(mut info: &metamodelica::Ref<ClockedInfo>) -> bool {
        let mut b: bool = UnorderedMap::isEmpty(info.baseClocks.clone());
        b
    }

    pub(crate) fn resolveSubClocks(
        mut info: &metamodelica::Ref<ClockedInfo>,
        mut clock_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<ComponentRef::NFComponentRef>,
            >,
        >,
    ) -> Result<()> {
        for mut cref in &*UnorderedMap::keyList(clock_map.clone()) {
            resolveImplicitSubClock(cref.clone(), info, clock_map.clone())?;
        }
        for mut sub_clock in &*UnorderedMap::keyList(info.subClocks.clone()) {
            resolveSubClock(sub_clock.clone(), info, clock_map.clone())?;
        }
        for mut sub_clock in &*UnorderedMap::keyList(info.subClocks.clone()) {
            addSubClock(sub_clock.clone(), info)?;
        }
        Ok(())
    }

    pub(crate) fn baseClockCount(mut info: &metamodelica::Ref<ClockedInfo>, mut countInferred: bool) -> Result<i32> {
        let mut count: i32 = UnorderedMap::size(info.baseClocks.clone());
        if !(countInferred) {
            count = count
                - List::count(
                    &(UnorderedMap::valueList(info.baseClocks.clone())),
                    &move |__a0: metamodelica::Ref<BClock::BClock>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(BClock::isInferredClock(&__a0))
                    },
                )?;
        }
        Ok(count)
    }

    pub(crate) fn subClockCount(mut info: &metamodelica::Ref<ClockedInfo>) -> i32 {
        let mut count: i32 = UnorderedMap::size(info.subClocks.clone());
        count
    }

    fn resolveImplicitSubClock(
        mut key: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut info: &metamodelica::Ref<ClockedInfo>,
        mut clock_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<ComponentRef::NFComponentRef>,
            >,
        >,
    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
        let mut clock: metamodelica::Ref<ComponentRef::NFComponentRef> = key.clone();
        if UnorderedMap::contains(key.clone(), clock_map.clone())? {
            clock = UnorderedMap::getSafe(
                key.clone(),
                clock_map.clone(),
                metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"),
            )?;
            if !(UnorderedMap::contains(clock.clone(), info.subClocks.clone())?
                || UnorderedMap::contains(clock.clone(), info.baseClocks.clone())?)
            {
                clock = resolveImplicitSubClock(clock, info, clock_map.clone())?;
                UnorderedMap::add(key, clock.clone(), clock_map)?;
            }
        }
        Ok(clock)
    }

    fn resolveSubClock(
        mut clock_name: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut info: &metamodelica::Ref<ClockedInfo>,
        mut clock_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<ComponentRef::NFComponentRef>,
            >,
        >,
    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
        let mut base_clock: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut implicit_clock: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut parent_clock: metamodelica::Ref<ComponentRef::NFComponentRef> = UnorderedMap::getSafe(
            clock_name.clone(),
            info.subToBase.clone(),
            metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"),
        )?;
        let mut implicit_clock_opt: Option<metamodelica::Ref<ComponentRef::NFComponentRef>> = None;
        let mut dest: metamodelica::Ref<BClock::BClock>;
        let mut src: metamodelica::Ref<BClock::BClock>;
        if UnorderedMap::contains(parent_clock.clone(), info.baseClocks.clone())? {
            base_clock = parent_clock;
        } else {
            if !(UnorderedMap::contains(parent_clock.clone(), info.subClocks.clone())?) {
                implicit_clock_opt = Some(parent_clock.clone());
                parent_clock = UnorderedMap::getSafe(
                    parent_clock,
                    clock_map.clone(),
                    metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"),
                )?;
            }
            base_clock = resolveSubClock(parent_clock.clone(), info, clock_map)?;
            dest = UnorderedMap::getSafe(
                parent_clock,
                info.subClocks.clone(),
                metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"),
            )?;
            src = UnorderedMap::getSafe(
                clock_name.clone(),
                info.subClocks.clone(),
                metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"),
            )?;
            UnorderedMap::add(
                clock_name.clone(),
                BClock::updateSubClock(dest.clone(), &src)?,
                info.subClocks.clone(),
            )?;
            UnorderedMap::add(clock_name, base_clock.clone(), info.subToBase.clone())?;
            if (implicit_clock_opt).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(implicit_clock_opt) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                implicit_clock = metamodelica::Own::own(__pa0);
                UnorderedMap::add(implicit_clock.clone(), dest, info.subClocks.clone())?;
                UnorderedMap::add(implicit_clock, base_clock.clone(), info.subToBase.clone())?;
            }
        }
        Ok(base_clock)
    }

    fn addSubClock(
        mut clock_name: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut info: &metamodelica::Ref<ClockedInfo>,
    ) -> Result<()> {
        let mut base_clock: metamodelica::Ref<ComponentRef::NFComponentRef> = UnorderedMap::getSafe(
            clock_name.clone(),
            info.subToBase.clone(),
            metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"),
        )?;
        let mut current_clocks: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        current_clocks = UnorderedMap::getOrDefault(base_clock.clone(), info.baseToSub.clone(), metamodelica::nil())?;
        UnorderedMap::add(
            base_clock,
            metamodelica::cons(clock_name, current_clocks),
            info.baseToSub.clone(),
        )?;
        Ok(())
    }
}

// =========================================================================
//                      MAIN ROUTINE, PLEASE DO NOT CHANGE
// =========================================================================
pub(crate) fn main(
    mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE>,
    mut kind: Partition::Kind,
) -> Result<metamodelica::Ref<BackendDAE::NBackendDAE>> {
    let mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE> = bdae;
    let mut func: Module::partitioningInterface;
    func = getModule()?;
    bdae = (::match_deref::match_deref! { match &((kind, bdae.clone())) {
        (Partition::Kind::ODE, Deref @ BackendDAE::MAIN { varData: Deref @ BVariable::VarData::VAR_DATA_SIM { unknowns: variables, clocks, .. }, eqData: Deref @ BEquation::EqData::EQ_DATA_SIM { simulation: equations, clocked, .. }, .. }) => {
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN; ode = func(kind, variables.clone(), equations.clone(), clocks.clone(), clocked.clone(), var_field!((*bdae).clockedInfo, BackendDAE::NBackendDAE::MAIN).clone())?);
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                ode = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>> = metamodelica::nil();
        for mut sys in (var_field!((*bdae).ode, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
            if !(!(Partition::Partition::isEmpty(&(sys.clone()))?)) { continue; }
            let __x = sys.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
                varData = BVariable::VarData::removeTypedCheck(var_field!((*bdae).varData, BackendDAE::NBackendDAE::MAIN).clone(), &fnptr!(BVariable::isClock, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>), VarData::VarType::DISCRETE.clone())?,
                eqData = BEquation::EqData::removeTypedCheck(var_field!((*bdae).eqData, BackendDAE::NBackendDAE::MAIN).clone(), &BEquation::Equation::isTypeClock, EqData::EqType::DISCRETE.clone())?
            );
            bdae
        },
        (_, Deref @ BackendDAE::MAIN { varData: Deref @ BVariable::VarData::VAR_DATA_SIM { initials: variables, clocks, .. }, eqData: Deref @ BEquation::EqData::EQ_DATA_SIM { initials: equations, clocked, .. }, .. }) if (Partition::kindIsInitial(kind)) => {
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN; init = partitioningNone(kind, variables.clone(), equations.clone(), clocks.clone(), clocked.clone(), var_field!((*bdae).clockedInfo, BackendDAE::NBackendDAE::MAIN).clone())?);
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN; init = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>> = metamodelica::nil();
        for mut sys in (var_field!((*bdae).init, BackendDAE::NBackendDAE::MAIN).clone()).into_iter().cloned() {
            if !(!(Partition::Partition::isEmpty(&(sys.clone()))?)) { continue; }
            let __x = sys.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            bdae
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.main")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(bdae)
}

pub(crate) fn getModule() -> Result<
    Arc<
        dyn ::std::ops::Fn(
                Partition::Kind,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<EquationPointers::EquationPointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<EquationPointers::EquationPointers>,
                metamodelica::Ref<ClockedInfo::ClockedInfo>,
            )
                -> Result<metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>>
            + 'static,
    >,
> {
    let mut func: Module::partitioningInterface;
    let mut flag: ArcStr = literal!("clocked");
    func = (::match_deref::match_deref! { match &(flag) {
        Deref @ "default" => (std::sync::Arc::new(partitioningClocked) as std::sync::Arc<dyn ::std::ops::Fn(Partition::Kind, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, metamodelica::Ref<ClockedInfo::ClockedInfo>) -> Result<metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>> + 'static>),
        Deref @ "clocked" => (std::sync::Arc::new(partitioningClocked) as std::sync::Arc<dyn ::std::ops::Fn(Partition::Kind, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, metamodelica::Ref<ClockedInfo::ClockedInfo>) -> Result<metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>> + 'static>),
        Deref @ "none" => (std::sync::Arc::new(partitioningNone) as std::sync::Arc<dyn ::std::ops::Fn(Partition::Kind, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, metamodelica::Ref<ClockedInfo::ClockedInfo>) -> Result<metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>> + 'static>),
        _ => return Err("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(func)
}

pub(crate) fn categorize(
    mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE>,
) -> Result<metamodelica::Ref<BackendDAE::NBackendDAE>> {
    let mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE> = bdae;
    bdae = ({
        let mut ode: DoubleEnded::MutableList<metamodelica::Ref<Partition::Partition::Partition>> =
            DoubleEnded::fromList(&(metamodelica::nil()))?;
        let mut alg: DoubleEnded::MutableList<metamodelica::Ref<Partition::Partition::Partition>> =
            DoubleEnded::fromList(&(metamodelica::nil()))?;
        let mut ode_evt: DoubleEnded::MutableList<metamodelica::Ref<Partition::Partition::Partition>> =
            DoubleEnded::fromList(&(metamodelica::nil()))?;
        let mut alg_evt: DoubleEnded::MutableList<metamodelica::Ref<Partition::Partition::Partition>> =
            DoubleEnded::fromList(&(metamodelica::nil()))?;
        let mut clocked: DoubleEnded::MutableList<metamodelica::Ref<Partition::Partition::Partition>> =
            DoubleEnded::fromList(&(metamodelica::nil()))?;
        (match &*bdae {
            BackendDAE::MAIN { ode: __bdae_ode, .. } => {
                for mut syst in &*__bdae_ode.clone() {
                    Partition::Partition::categorize(
                        syst.clone(),
                        ode.clone(),
                        alg.clone(),
                        ode_evt.clone(),
                        alg_evt.clone(),
                        clocked.clone(),
                    )?;
                }
                assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                    ode = DoubleEnded::toListAndClear(ode, metamodelica::nil())?,
                    algebraic = DoubleEnded::toListAndClear(alg, metamodelica::nil())?,
                    ode_event = DoubleEnded::toListAndClear(ode_evt, metamodelica::nil())?,
                    alg_event = DoubleEnded::toListAndClear(alg_evt, metamodelica::nil())?,
                    clocked = DoubleEnded::toListAndClear(clocked, metamodelica::nil())?
                );
                bdae
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBPartitioning.categorize"));
                        __mm_s.push_str(&*literal!(" failed."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        })
    });
    Ok(bdae)
}

pub(crate) fn extractClocksEqn(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut clck_coll: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<BClock::BClock>, metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >,
    mut infr_coll: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<BClock::BClock>, metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >,
    mut new_clocks: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut new_infers: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut idx: Pointer::Pointer<i32>,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    eqn = (match &*eqn {
        BEquation::Equation::WHEN_EQUATION { body: __eqn_body, .. } => {
            assign_variant_field!(eqn => Equation::Equation::WHEN_EQUATION; body = Util::getOption(extractClocksWhenCond(Some(__eqn_body.clone()), clck_coll.clone(), infr_coll.clone(), new_clocks.clone(), new_infers.clone(), idx.clone())?)?);
            eqn
        }
        _ => eqn,
    });
    eqn = BEquation::Equation::map(
        eqn,
        (std::sync::Arc::new({
            let __pe_b1 = clck_coll;
            let __pe_b2 = infr_coll;
            let __pe_b3 = new_clocks;
            let __pe_b4 = new_infers;
            let __pe_b5 = idx;
            let __pe_b6 = false;
            move |__pe_a0| {
                extractClocks(
                    __pe_a0,
                    __pe_b1.clone(),
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
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(eqn)
}

pub(crate) fn extractClocksWhenCond(
    mut body_opt: Option<metamodelica::Ref<WhenEquationBody::WhenEquationBody>>,
    mut clck_coll: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<BClock::BClock>, metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >,
    mut infr_coll: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<BClock::BClock>, metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >,
    mut new_clocks: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut new_infers: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut idx: Pointer::Pointer<i32>,
) -> Result<Option<metamodelica::Ref<WhenEquationBody::WhenEquationBody>>> {
    let mut body_opt: Option<metamodelica::Ref<WhenEquationBody::WhenEquationBody>> = body_opt;
    body_opt = (::match_deref::match_deref! { match &(body_opt.clone()) {
        Some(body) => {
            let mut body = (*body).clone();
            assign_field!(
                body.condition = Expression::map(body.condition.clone(), (std::sync::Arc::new({ let __pe_b1 = clck_coll.clone(); let __pe_b2 = infr_coll.clone(); let __pe_b3 = new_clocks.clone(); let __pe_b4 = new_infers.clone(); let __pe_b5 = idx.clone(); let __pe_b6 = true; move |__pe_a0| extractClocks(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?,
                body.else_when = extractClocksWhenCond(body.else_when.clone(), clck_coll, infr_coll, new_clocks, new_infers, idx)?
            );
            Some(body.clone())
        },
        _ => {
            body_opt
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(body_opt)
}

pub(crate) fn extractClocks(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut clck_coll: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<BClock::BClock>, metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >,
    mut infr_coll: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<BClock::BClock>, metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >,
    mut new_clocks: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut new_infers: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut idx: Pointer::Pointer<i32>,
    mut when_cond: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (match &*exp {
        Expression::CLKCONST { clk: __exp_clk }
            if (when_cond || !(ClockKind::isInferred(metamodelica::AsArg::as_arg(&__exp_clk)))) =>
        {
            let mut clock: metamodelica::Ref<BClock::BClock>;
            let mut clock_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
            let mut clock_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
            clock = metamodelica::Ref::new(BClock::BClock::BASE_CLOCK {
                clock: __exp_clk.clone(),
            });
            if UnorderedMap::contains(clock.clone(), clck_coll.clone())? {
                clock_name = UnorderedMap::getSafe(
                    clock,
                    clck_coll,
                    metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"),
                )?;
            } else if UnorderedMap::contains(clock.clone(), infr_coll.clone())? {
                clock_name = UnorderedMap::getSafe(
                    clock,
                    infr_coll,
                    metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"),
                )?;
            } else {
                (clock_var, clock_name) =
                    BVariable::makeClockVar(Pointer::access(idx.clone()), Expression::typeOf(exp))?;
                if BClock::isInferredClock(&clock) {
                    UnorderedMap::add(clock, clock_name.clone(), infr_coll)?;
                    Pointer::update(
                        new_infers.clone(),
                        metamodelica::cons(clock_var, Pointer::access(new_infers)),
                    );
                } else {
                    UnorderedMap::add(clock, clock_name.clone(), clck_coll)?;
                    Pointer::update(
                        new_clocks.clone(),
                        metamodelica::cons(clock_var, Pointer::access(new_clocks)),
                    );
                }
                Pointer::update(idx.clone(), Pointer::access(idx) + 1);
            }
            Expression::fromCref(clock_name, false)?
        }
        _ => exp,
    });
    Ok(exp)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum ClusterElementType {
    EQUATION = 1,
    VARIABLE = 2,
}
impl PartialOrd for ClusterElementType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ClusterElementType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for ClusterElementType {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub mod Cluster {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct Cluster {
        /// set of all variables in this cluster
        pub variables: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        /// set of all equations in this cluster
        pub eqn_idnts: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    }

    impl metamodelica::gc::MMTrace for Cluster {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.variables, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.eqn_idnts, __mmv)?;
            Ok(())
        }
    }
    impl Default for Cluster {
        fn default() -> Self {
            Self {
                variables: Default::default(),
                eqn_idnts: Default::default(),
            }
        }
    }

    pub type CLUSTER = Cluster;

    pub(crate) fn toString(mut cluster: &metamodelica::Ref<Cluster>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("### Cluster Variables:\n"));
            __mm_s.push_str(&*UnorderedSet::toString(
                cluster.variables.clone(),
                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                literal!("\n"),
            )?);
            __mm_s.push_str(&*literal!("\n### Cluster Equation Identifiers:\n"));
            __mm_s.push_str(&*UnorderedSet::toString(
                cluster.eqn_idnts.clone(),
                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                literal!("\n"),
            )?);
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn addElement(
        mut cluster_opt: Option<metamodelica::Ref<Cluster>>,
        mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut ty: ClusterElementType,
    ) -> Result<metamodelica::Ref<Cluster>> {
        let mut cluster: metamodelica::Ref<Cluster>;
        cluster = (::match_deref::match_deref! { match &(cluster_opt) {
            Some(__esc_cluster) => {
                cluster = (*__esc_cluster).clone();
                cluster.clone()
            },
            _ => metamodelica::Ref::new(Cluster { variables: UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13), eqn_idnts: UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13) }),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        cluster = (match ty {
            ClusterElementType::VARIABLE => {
                UnorderedSet::add(cref, cluster.variables.clone())?;
                cluster
            }
            ClusterElementType::EQUATION { .. } => {
                UnorderedSet::add(cref, cluster.eqn_idnts.clone())?;
                cluster
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBPartitioning.Cluster.addElement"));
                        __mm_s.push_str(&*literal!(" failed for "));
                        __mm_s.push_str(&*ComponentRef::toString(&cref)?);
                        __mm_s.push_str(&*literal!(" because of unknown cluster element type."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(cluster)
    }

    pub(crate) fn addToClockMap(
        mut cluster: &metamodelica::Ref<Cluster>,
        mut equations: &metamodelica::Ref<EquationPointers::EquationPointers>,
        mut info: &metamodelica::Ref<ClockedInfo::ClockedInfo>,
        mut clock_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<ComponentRef::NFComponentRef>,
            >,
        >,
    ) -> Result<()> {
        fn findClock(
            mut exp: metamodelica::Ref<Expression::NFExpression>,
            mut info: &metamodelica::Ref<ClockedInfo::ClockedInfo>,
            mut clock_ptr: Pointer::Pointer<Option<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
            let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
            let mut clock_opt: Option<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                Pointer::access(clock_ptr.clone());
            exp = (::match_deref::match_deref! { match &((exp.clone(), clock_opt)) {
                (_, Some(_)) => exp.clone(),
                (Deref @ Expression::CREF { .. }, None) if (BVariable::isClockOrClocked(BVariable::getVarPointer(var_field!((*exp).cref, Expression::NFExpression::CREF), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?)) => {
                    Pointer::update(clock_ptr, Some(var_field!((*exp).cref, Expression::NFExpression::CREF).clone()));
                    exp.clone()
                },
                (Deref @ Expression::CALL { .. }, _) if (Expression::isClockOrSampleFunction(&exp)?) => exp.clone(),
                _ => Expression::mapShallow(exp.clone(), (std::sync::Arc::new({ let __pe_b1 = info.clone(); let __pe_b2 = clock_ptr; move |__pe_a0| findClock(__pe_a0, &__pe_b1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            Ok(exp)
        }

        let mut clock_ptr: Pointer::Pointer<Option<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
            Pointer::create(None);
        let mut clock_opt: Option<metamodelica::Ref<ComponentRef::NFComponentRef>> = None;
        let mut clock: metamodelica::Ref<ComponentRef::NFComponentRef>;
        for mut eqn_name in &*UnorderedSet::toList(cluster.eqn_idnts.clone()) {
            BEquation::Equation::map(
                Pointer::access(BEquation::EquationPointers::getEqnByName(equations, eqn_name.clone())?),
                (std::sync::Arc::new({
                    let __pe_b1 = info.clone();
                    let __pe_b2 = clock_ptr.clone();
                    move |__pe_a0| findClock(__pe_a0, &__pe_b1, __pe_b2.clone())
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
            )?;
            clock_opt = Pointer::access(clock_ptr.clone());
            if (clock_opt).is_some() {
                break;
            }
        }
        if (clock_opt).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(clock_opt) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            clock = metamodelica::Own::own(__pa0);
            for mut var_name in &*UnorderedSet::toList(cluster.variables.clone()) {
                UnorderedMap::add(var_name.clone(), clock.clone(), clock_map.clone())?;
            }
        }
        Ok(())
    }

    pub(crate) fn toPartition(
        mut cluster: &metamodelica::Ref<Cluster>,
        mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
        mut equations: &metamodelica::Ref<EquationPointers::EquationPointers>,
        mut kind: Partition::Kind,
        mut info: &metamodelica::Ref<ClockedInfo::ClockedInfo>,
        mut held_crefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        mut infer_del: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    ) -> Result<metamodelica::Ref<Partition::Partition::Partition>> {
        let mut partition: metamodelica::Ref<Partition::Partition::Partition>;
        let mut cvars: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
            UnorderedSet::toList(cluster.variables.clone());
        let mut cidnt: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
            UnorderedSet::toList(cluster.eqn_idnts.clone());
        let mut association: metamodelica::Ref<Partition::Association::Association>;
        let mut var_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
        let mut filtered_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
        let mut eqn_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
        let mut partVariables: metamodelica::Ref<VariablePointers::VariablePointers>;
        let mut partEquations: metamodelica::Ref<EquationPointers::EquationPointers>;
        let mut inferred_clocks: metamodelica::Ref<
            UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        > = UnorderedSet::new(
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
            13,
        );
        var_lst = ({
            let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                metamodelica::nil();
            for mut cref in (cvars).into_iter().cloned() {
                let __x = BVariable::getVarPointer(
                    &(cref.clone()),
                    metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"),
                )?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        filtered_vars = ({
            let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                metamodelica::nil();
            for mut var in (var_lst).into_iter().cloned() {
                if !(BVariable::VariablePointers::contains(var.clone(), variables)?) {
                    continue;
                }
                let __x = var.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        eqn_lst = ({
            let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
                metamodelica::nil();
            for mut name in (cidnt).into_iter().cloned() {
                let __x = BEquation::EquationPointers::getEqnByName(equations, name.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        partVariables = BVariable::VariablePointers::fromList(&filtered_vars, false)?;
        partEquations = BEquation::EquationPointers::fromList(&eqn_lst)?;
        association = Partition::Association::create(partEquations.clone(), kind, info, infer_del.clone())?;
        partEquations = BEquation::EquationPointers::mapExp(
            partEquations,
            (std::sync::Arc::new({
                let __pe_b1 = held_crefs;
                move |__pe_a0| replaceClockedFunctions(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
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
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?;
        if Partition::Association::isClocked(&association) {
            partVariables = BVariable::VariablePointers::mapRemovePtr(
                partVariables,
                &({
                    let __pe_b1 = inferred_clocks.clone();
                    move |__pe_a0| collectInferredClock(__pe_a0, __pe_b1.clone())
                }),
            )?;
            partEquations = BEquation::EquationPointers::mapRemovePtr(
                partEquations,
                &({
                    let __pe_b1 = inferred_clocks.clone();
                    move |__pe_a0| removeInferredClock(__pe_a0, __pe_b1.clone())
                }),
            )?;
            partEquations = BEquation::EquationPointers::map(partEquations, &replaceClockedWhen)?;
            partVariables = BVariable::VariablePointers::mapPtr(
                partVariables,
                &({
                    let __pe_b1 = openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_CLOCKED();
                    move |__pe_a0| Ok(BVariable::setVarKind(__pe_a0, __pe_b1.clone()))
                }),
            )?;
            if BEquation::EquationPointers::size(&partEquations) == 0 {
                UnorderedSet::merge(infer_del, inferred_clocks)?;
            }
        }
        partition = metamodelica::Ref::new(Partition::Partition::Partition {
            index: 0,
            association: association,
            unknowns: partVariables,
            daeUnknowns: None,
            equations: partEquations,
            adjacencyMatrix: None,
            matching: None,
            strongComponents: None,
        });
        Ok(partition)
    }

    fn collectInferredClock(
        mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
        mut inferred_clocks: metamodelica::Ref<
            UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    ) -> Result<bool> {
        let mut delete: bool = BVariable::isClock(var.clone());
        if delete {
            UnorderedSet::add(BVariable::getVarName(var), inferred_clocks)?;
        }
        Ok(delete)
    }

    fn removeInferredClock(
        mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
        mut inferred_clocks: metamodelica::Ref<
            UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    ) -> Result<bool> {
        let mut delete: bool;
        delete = (::match_deref::match_deref! { match &(Pointer::access(eqn)) {
            Deref @ BEquation::Equation::SCALAR_EQUATION { lhs: Deref @ Expression::CREF { cref: lhs, .. }, .. } => {
                UnorderedSet::contains(lhs.clone(), inferred_clocks)?
            },
            _ => {
                false
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(delete)
    }
}

// Perhaps this deserves its own place in Util/*.mo
pub mod DisjointSetForest {
    use super::*;
    /// Custom implementation of disjoint-set data structure with constant number of elements.
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct DisjointSetForest {
        pub parent: Pointer::Pointer<metamodelica::Array<i32>>,
        pub rank: Pointer::Pointer<metamodelica::Array<i32>>,
    }

    impl metamodelica::gc::MMTrace for DisjointSetForest {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.parent, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.rank, __mmv)?;
            Ok(())
        }
    }
    impl Default for DisjointSetForest {
        fn default() -> Self {
            Self {
                parent: Default::default(),
                rank: Default::default(),
            }
        }
    }

    pub type FOREST = DisjointSetForest;

    pub(crate) fn new(mut n: i32) -> metamodelica::Ref<DisjointSetForest> {
        let mut dsf: metamodelica::Ref<DisjointSetForest>;
        dsf = metamodelica::Ref::new(DisjointSetForest {
            parent: Pointer::create(metamodelica::arrayFromVec(
                ({
                    let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                    for mut i in (1..=n).into_iter() {
                        let __x = i.clone();
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })
                .into_iter()
                .cloned()
                .collect(),
            )),
            rank: Pointer::create(arrayCreate(n, 0)),
        });
        dsf
    }

    pub(crate) fn find(mut dsf: &metamodelica::Ref<DisjointSetForest>, mut index: i32) -> Result<i32> {
        let mut index: i32 = index;
        let mut parent: metamodelica::Array<i32> = Pointer::access(dsf.parent.clone());
        while index
            != ({
                let __elt = (*metamodelica::index_checked(&parent.borrow(), index)?).clone();
                __elt
            })
        {
            {
                let __cell0 = ({
                    let __elt = (*metamodelica::index_checked(
                        &parent.borrow(),
                        ({
                            let __elt = (*metamodelica::index_checked(&parent.borrow(), index)?).clone();
                            __elt
                        }),
                    )?)
                    .clone();
                    __elt
                });
                let __idx0 = index;
                *metamodelica::index_mut_checked(&mut parent.clone().borrow_mut(), __idx0)? = __cell0;
            }
            index = ({
                let __elt = (*metamodelica::index_checked(&parent.borrow(), index)?).clone();
                __elt
            });
        }
        Pointer::update(dsf.parent.clone(), parent.clone());
        Ok(index)
    }

    pub(crate) fn unite(
        mut dsf: &metamodelica::Ref<DisjointSetForest>,
        mut indices: metamodelica::List<i32>,
    ) -> Result<i32> {
        let mut root: i32;
        let mut roots: metamodelica::List<i32> = ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut i in (indices.clone()).into_iter().cloned() {
                let __x = find(dsf, i.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        let mut parent: metamodelica::Array<i32> = Pointer::access(dsf.parent.clone());
        let mut rank: metamodelica::Array<i32> = Pointer::access(dsf.rank.clone());
        let mut maxRank: i32;
        let mut tied: bool = false;
        root = (roots).head().cloned()?;
        maxRank = ({
            let __elt = (*metamodelica::index_checked(&rank.borrow(), root)?).clone();
            __elt
        });
        for mut r in &*(roots).rest()? {
            if r.clone() != root {
                if ({
                    let __elt = (*metamodelica::index_checked(&rank.borrow(), r.clone())?).clone();
                    __elt
                }) > maxRank
                {
                    root = r.clone();
                    maxRank = ({
                        let __elt = (*metamodelica::index_checked(&rank.borrow(), root)?).clone();
                        __elt
                    });
                    tied = false;
                } else if ({
                    let __elt = (*metamodelica::index_checked(&rank.borrow(), r.clone())?).clone();
                    __elt
                }) == maxRank
                {
                    tied = true;
                }
            }
        }
        for mut r in &*roots {
            {
                let __cell0 = root;
                let __idx0 = find(dsf, r.clone())?;
                *metamodelica::index_mut_checked(&mut parent.clone().borrow_mut(), __idx0)? = __cell0;
            }
        }
        if tied {
            {
                let __cell1 = ({
                    let __elt = (*metamodelica::index_checked(&rank.borrow(), root)?).clone();
                    __elt
                }) + 1;
                let __idx1 = root;
                *metamodelica::index_mut_checked(&mut rank.clone().borrow_mut(), __idx1)? = __cell1;
            }
        }
        Pointer::update(dsf.parent.clone(), parent.clone());
        Pointer::update(dsf.rank.clone(), rank.clone());
        Ok(root)
    }
}

fn partitioningNone(
    mut kind: Partition::Kind,
    mut variables: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut clocks: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut clocked: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut info: metamodelica::Ref<ClockedInfo::ClockedInfo>,
) -> Result<metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>> {
    let mut partitions: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>;
    let mut clone_vars: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut clone_eqns: metamodelica::Ref<EquationPointers::EquationPointers>;
    clone_vars = BVariable::VariablePointers::clone(&variables, true)?;
    clone_eqns = BEquation::EquationPointers::clone(&equations, true)?;
    partitions = list![metamodelica::Ref::new(Partition::Partition::Partition {
        index: 1,
        association: metamodelica::Ref::new(Partition::Association::Association::CONTINUOUS {
            kind: kind,
            jacobian: None,
            jacobianAdjoint: None,
            LFG_jacobian: None,
            MRF_jacobian: None,
            R0_jacobian: None
        }),
        unknowns: clone_vars,
        daeUnknowns: None,
        equations: clone_eqns,
        adjacencyMatrix: None,
        matching: None,
        strongComponents: None
    })];
    Ok(partitions)
}

fn partitioningClocked(
    mut kind: Partition::Kind,
    mut variables: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut clocks: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut clocked: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut info: metamodelica::Ref<ClockedInfo::ClockedInfo>,
) -> Result<metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>> {
    let mut partitions: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>;
    let mut eqn_dsf: metamodelica::Ref<DisjointSetForest::DisjointSetForest> =
        DisjointSetForest::new(ExpandableArray::getLastUsedIndex(equations.eqArr.clone()));
    let mut var_map: metamodelica::Array<i32> =
        arrayCreate(ExpandableArray::getLastUsedIndex(variables.varArr.clone()), -1);
    let mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut var_crefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    let mut var_indices: metamodelica::List<i32>;
    let mut part_idx: i32;
    let mut cluster_map: metamodelica::Ref<UnorderedMap::UnorderedMap<i32, metamodelica::Ref<Cluster::Cluster>>> =
        UnorderedMap::new(
            std::sync::Arc::new(fnptr!(Util::id, _)),
            (std::sync::Arc::new(fnptr!(intEq, i32, i32))
                as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            1,
        );
    let mut name_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut marked_vars: metamodelica::Array<bool>;
    let mut single_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut held_crefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
        UnorderedSet::new(
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
            13,
        );
    let mut clock_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    > = UnorderedMap::new(
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
    let mut infer_del: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
        UnorderedSet::new(
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
            13,
        );
    let mut index: Pointer::Pointer<i32> = Pointer::create(1);
    for mut eq_idx in &*UnorderedMap::valueList(clocked.map.clone()) {
        if eq_idx.clone() > 0 {
            eqn = BEquation::EquationPointers::getEqnAt(&clocked, eq_idx.clone())?;
            BClock::add(Pointer::access(eqn), &info)?;
        }
    }
    for mut eq_idx in &*UnorderedMap::valueList(equations.map.clone()) {
        if eq_idx.clone() > 0 {
            eqn = BEquation::EquationPointers::getEqnAt(&equations, eq_idx.clone())?;
            BClock::add(Pointer::access(eqn.clone()), &info)?;
            var_crefs = UnorderedSet::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
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
                13,
            );
            BEquation::Equation::map(
                Pointer::access(eqn),
                (std::sync::Arc::new({
                    let __pe_b1 = var_crefs.clone();
                    move |__pe_a0| collectPartitioningCrefs(__pe_a0, __pe_b1.clone())
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
            )?;
            var_indices = ({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut cref in (UnorderedSet::toList(var_crefs)).into_iter().cloned() {
                    let __x = BVariable::VariablePointers::getVarIndex(&variables, cref.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            var_indices = ({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut i in (var_indices).into_iter().cloned() {
                    if !(i.clone() > 0) {
                        continue;
                    }
                    let __x = i.clone();
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            part_idx = DisjointSetForest::unite(
                &eqn_dsf,
                metamodelica::cons(
                    eq_idx.clone(),
                    ({
                        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                        for mut j in (var_indices.clone()).into_iter().cloned() {
                            if !(({
                                let __elt = (*metamodelica::index_checked(&var_map.borrow(), j.clone())?).clone();
                                __elt
                            }) > 0)
                            {
                                continue;
                            }
                            let __x = ({
                                let __elt = (*metamodelica::index_checked(&var_map.borrow(), j.clone())?).clone();
                                __elt
                            });
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                ),
            )?;
            for mut i in &*var_indices {
                {
                    let __cell0 = part_idx;
                    let __idx0 = i.clone();
                    *metamodelica::index_mut_checked(&mut var_map.clone().borrow_mut(), __idx0)? = __cell0;
                }
            }
        }
    }
    marked_vars = metamodelica::arrayFromVec(
        ({
            let mut __acc: metamodelica::List<bool> = metamodelica::nil();
            for mut var_idx in (UnorderedMap::valueList(variables.map.clone())).into_iter().cloned() {
                let __x = ({
                    let __elt = (*metamodelica::index_checked(&var_map.borrow(), var_idx.clone())?).clone();
                    __elt
                }) < 0;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
        .into_iter()
        .cloned()
        .collect(),
    );
    single_vars = ({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            metamodelica::nil();
        for mut var_ptr in (BVariable::VariablePointers::getMarkedVars(&variables, marked_vars.clone())?)
            .into_iter()
            .cloned()
        {
            let __x = var_ptr.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if !((single_vars).is_empty()) {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NBPartitioning.partitioningClocked"));
                __mm_s.push_str(&*literal!(" ("));
                __mm_s.push_str(&*Partition::Partition::kindToString(kind)?);
                __mm_s.push_str(&*literal!(
                    ") failed because the following variables could not be assigned to a partition:\n  {"
                ));
                __mm_s.push_str(&*stringDelimitList(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut var_ptr in (single_vars).into_iter().cloned() {
                            let __x = BVariable::toString(&(Pointer::access(var_ptr.clone())), literal!(""))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    literal!("\n"),
                ));
                __mm_s.push_str(&*literal!("}"));
                ArcStr::from(__mm_s)
            }],
        )?;
        return Err("fail");
    }
    for mut eq_idx in &*UnorderedMap::valueList(equations.map.clone()) {
        if eq_idx.clone() > 0 {
            eqn = BEquation::EquationPointers::getEqnAt(&equations, eq_idx.clone())?;
            name_cref = BEquation::Equation::getEqnName(eqn)?;
            part_idx = DisjointSetForest::find(&eqn_dsf, eq_idx.clone())?;
            UnorderedMap::addUpdate(
                part_idx,
                &({
                    let __pe_b1 = name_cref;
                    let __pe_b2 = ClusterElementType::EQUATION.clone();
                    move |__pe_a0| Cluster::addElement(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                }),
                cluster_map.clone(),
            )?;
        }
    }
    for mut var_idx in &*UnorderedMap::valueList(variables.map.clone()) {
        if var_idx.clone() > 0 {
            var = BVariable::VariablePointers::getVarAt(&variables, var_idx.clone())?;
            name_cref = BVariable::getVarName(var);
            part_idx = DisjointSetForest::find(
                &eqn_dsf,
                ({
                    let __elt = (*metamodelica::index_checked(&var_map.borrow(), var_idx.clone())?).clone();
                    __elt
                }),
            )?;
            UnorderedMap::addUpdate(
                part_idx,
                &({
                    let __pe_b1 = name_cref;
                    let __pe_b2 = ClusterElementType::VARIABLE.clone();
                    move |__pe_a0| Cluster::addElement(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                }),
                cluster_map.clone(),
            )?;
        }
    }
    for mut cluster in &*UnorderedMap::valueList(cluster_map.clone()) {
        Cluster::addToClockMap(
            metamodelica::AsArg::as_arg(&cluster),
            &equations,
            &info,
            clock_map.clone(),
        )?;
    }
    ClockedInfo::resolveSubClocks(&info, clock_map)?;
    partitions = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>> = metamodelica::nil();
        for mut cl in (UnorderedMap::valueList(cluster_map)).into_iter().cloned() {
            let __x = Cluster::toPartition(
                &(cl.clone()),
                &variables,
                &equations,
                kind,
                &info,
                held_crefs.clone(),
                infer_del.clone(),
            )?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    partitions = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>> = metamodelica::nil();
        for mut part in (partitions).into_iter().cloned() {
            let __x = Partition::Partition::updateHeldVars(part.clone(), held_crefs.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    partitions = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>> = metamodelica::nil();
        for mut partition in (sortAndMergeClockedPartitions(partitions, &info)?).into_iter().cloned() {
            if !(!(Partition::Partition::isEmpty(&(partition.clone()))?)) {
                continue;
            }
            let __x = Partition::Partition::setIndex(partition.clone(), index.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    for mut unused_infer in &*UnorderedSet::toList(infer_del) {
        UnorderedMap::remove(unused_infer.clone(), info.baseClocks.clone())?;
        UnorderedMap::remove(unused_infer.clone(), info.baseToSub.clone())?;
    }
    if Flags::isSet(Flags::DUMP_SYNCHRONOUS.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*StringUtil::headline_1(
                &(literal!("[dumpSynchronous] Partitioning result:")),
            )?);
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*List::toString(
                partitions.clone(),
                &({
                    let __pe_b1 = 2;
                    move |__pe_a0| Partition::Partition::toString(&__pe_a0, __pe_b1.clone())
                }),
                List::Style::NEWLINE.clone(),
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print(ClockedInfo::toString(&info)?);
    }
    Ok(partitions)
}

fn sortAndMergeClockedPartitions(
    mut partitions: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>,
    mut info: &metamodelica::Ref<ClockedInfo::ClockedInfo>,
) -> Result<metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>> {
    pub(crate) type SubMap = metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<BClock::BClock>,
            metamodelica::Ref<Partition::Partition::Partition>,
        >,
    >;

    let mut partitions: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>> = partitions;
    let mut clocked_partitions: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>;
    let mut new_clocked: metamodelica::List<metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>> =
        metamodelica::nil();
    let mut clock_collector: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<BClock::BClock>,
            metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<BClock::BClock>,
                    metamodelica::Ref<Partition::Partition::Partition>,
                >,
            >,
        >,
    > = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<BClock::BClock>| BClock::hash(&__a0))
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BClock::BClock>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<BClock::BClock>, __a1: metamodelica::Ref<BClock::BClock>| {
                BClock::isEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<BClock::BClock>, metamodelica::Ref<BClock::BClock>) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    let mut base_clock_inferrence: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<BClock::BClock>>,
    > = UnorderedMap::new(
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
    let mut clock: metamodelica::Ref<BClock::BClock>;
    let mut baseClock: metamodelica::Ref<BClock::BClock> =
        <metamodelica::Ref<BClock::BClock> as ::std::default::Default>::default();
    let mut subClock: metamodelica::Ref<BClock::BClock>;
    let mut baseClock_opt: Option<metamodelica::Ref<BClock::BClock>>;
    let mut subClockMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<BClock::BClock>,
            metamodelica::Ref<Partition::Partition::Partition>,
        >,
    >;
    let mut new_part: metamodelica::Ref<Partition::Partition::Partition>;
    (clocked_partitions, partitions) = List::splitOnTrue(&partitions, &move |__a0: metamodelica::Ref<
        Partition::Partition::Partition,
    >|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(Partition::Partition::isClocked(&__a0))
    })?;
    for mut baseClock in &*UnorderedMap::valueList(info.baseClocks.clone()) {
        let mut baseClock = baseClock.clone();
        UnorderedMap::add(
            baseClock,
            UnorderedMap::new(
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
                1,
            ),
            clock_collector.clone(),
        )?;
    }
    for mut partition in &*clocked_partitions {
        (clock, baseClock_opt, _) = Partition::Partition::getClocks(metamodelica::AsArg::as_arg(&partition))?;
        clock = (::match_deref::match_deref! { match &(baseClock_opt) {
            Some(__esc_clock) => {
                clock = (*__esc_clock).clone();
                clock.clone()
            },
            _ => clock,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        for mut var in &*BVariable::VariablePointers::toList(&partition.unknowns)? {
            UnorderedMap::add(
                BVariable::getVarName(var.clone()),
                clock.clone(),
                base_clock_inferrence.clone(),
            )?;
        }
    }
    for mut partition in &*clocked_partitions {
        let mut partition = partition.clone();
        (clock, baseClock_opt, _) = Partition::Partition::getClocks(&partition)?;
        if (baseClock_opt).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(baseClock_opt) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            baseClock = metamodelica::Own::own(__pa0);
            baseClock = BClock::baseClockInferrence(baseClock, base_clock_inferrence.clone())?;
            subClock = clock;
        } else {
            baseClock = BClock::baseClockInferrence(clock, base_clock_inferrence.clone())?;
            subClock = DEFAULT_SUB_CLOCK().clone();
        }
        partition = Partition::Partition::setClocks(partition, subClock.clone(), Some(baseClock.clone()))?;
        subClockMap = UnorderedMap::getSafe(
            baseClock,
            clock_collector.clone(),
            metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"),
        )?;
        new_part = (::match_deref::match_deref! { match &(UnorderedMap::get(subClock.clone(), subClockMap.clone())?) {
            Some(__esc_new_part) => {
                new_part = (*__esc_new_part).clone();
                Partition::Partition::merge(new_part.clone(), &partition, true)?
            },
            _ => partition,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        UnorderedMap::add(subClock, new_part.clone(), subClockMap)?;
    }
    for mut tpl in &*UnorderedMap::toList(clock_collector) {
        (baseClock, subClockMap) = tpl.clone();
        new_clocked = metamodelica::cons(
            sortClockedPartitions(UnorderedMap::valueList(subClockMap))?,
            new_clocked,
        );
    }
    partitions = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>> = metamodelica::nil();
        for mut partition in (metamodelica::cons(partitions, new_clocked).reverse())
            .into_iter()
            .cloned()
        {
            let __x = partition.clone();
            __acc = __x.append(&__acc);
        }
        __acc
    });
    Ok(partitions)
}

fn sortClockedPartitions(
    mut unsorted: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>,
) -> Result<metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>> {
    let mut sorted: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>> = metamodelica::nil();
    let mut n: i32 = ((unsorted).len() as i32);
    let mut partitions: metamodelica::Array<metamodelica::Ref<Partition::Partition::Partition>> =
        metamodelica::arrayFromVec(unsorted.clone().reverse().into_iter().cloned().collect());
    let mut m: Adjacency::IntMatrix::Builder = Adjacency::IntMatrix::newBuilder(n, false);
    let mut matching: metamodelica::Ref<Matching::NBMatching> = Matching::trivial(n);
    let mut index_map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<BClock::BClock>, i32>> =
        UnorderedMap::new(
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
            1,
        );
    let mut partition_order: metamodelica::List<metamodelica::List<i32>>;
    let mut j: i32;
    for mut i in 1..=n {
        UnorderedMap::add(
            (Partition::Partition::getClocks(
                &({
                    let __elt = (*metamodelica::index_checked(&partitions.borrow(), i)?).clone();
                    __elt
                }),
            )?)
            .0,
            i,
            index_map.clone(),
        )?;
    }
    for mut i in 1..=n {
        let __range0 = &*UnorderedSet::toList(Partition::Partition::getClockDependencies(
            &({
                let __elt = (*metamodelica::index_checked(&partitions.borrow(), i)?).clone();
                __elt
            }),
        )?);
        for mut clock in __range0 {
            j = UnorderedMap::getSafe(
                clock.clone(),
                index_map.clone(),
                metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"),
            )?;
            Adjacency::IntMatrix::builderAdd(&m, i, j);
        }
    }
    partition_order = Sorting::tarjanScalar(&(Adjacency::IntMatrix::fromBuilder(&m, n)?), &matching)?;
    for mut comp in &*partition_order.reverse() {
        let mut comp = comp.clone();
        sorted = (::match_deref::match_deref! { match &(comp.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: __esc_j, tail: Deref @ metamodelica::ListNode::Nil } => {
                j = (*__esc_j).clone();
                metamodelica::cons(({let __elt = (*metamodelica::index_checked(&partitions.borrow(), j.clone())?).clone(); __elt}), sorted)
            },
            _ => {
                let mut var_clock_map: metamodelica::Ref<UnorderedMap::UnorderedMap<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>, metamodelica::Ref<BClock::BClock>>>;
                let mut part: metamodelica::Ref<Partition::Partition::Partition>;
                let mut sub_comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
                let mut sub_comp_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
                let mut sub_comp_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
                let mut collector: Option<(metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>, metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>, metamodelica::Ref<BClock::BClock>)>;
                let mut var_clocks: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<BClock::BClock>>>;
                let mut baseClock: Option<metamodelica::Ref<BClock::BClock>>;
                let mut vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
                let mut eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
                let mut clock: metamodelica::Ref<BClock::BClock>;
                let mut new_clock: metamodelica::Ref<BClock::BClock>;
                var_clock_map = UnorderedMap::new((std::sync::Arc::new(BVariable::hash) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<i32> + 'static>), (std::sync::Arc::new(BVariable::equalName) as std::sync::Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool> + 'static>), 1);
                for mut i in &*comp {
                    part = ({let __elt = (*metamodelica::index_checked(&partitions.borrow(), i.clone())?).clone(); __elt});
                    for mut var in &*BVariable::VariablePointers::toList(&part.unknowns)? {
                        UnorderedMap::add(var.clone(), (Partition::Partition::getClocks(&part)?).0, var_clock_map.clone())?;
                    }
                }
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(comp) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                j = metamodelica::Own::own(__pa0);
                comp = metamodelica::Own::own(__pa1);
                part = ({let __elt = (*metamodelica::index_checked(&partitions.borrow(), j)?).clone(); __elt});
                for mut i in &*comp {
                    part = Partition::Partition::merge(part, &({let __elt = (*metamodelica::index_checked(&partitions.borrow(), i.clone())?).clone(); __elt}), false)?;
                }
                (_, baseClock, _) = Partition::Partition::getClocks(&part)?;
                (_, sub_comps) = Causalize::simple(&(part.unknowns.clone()), &(part.equations.clone()), Partition::Partition::getKind(&part), Adjacency::MatrixStrictness::MATCHING.clone(), &(crate::NBEquation::Iterator::interned_EMPTY()))?;
                collector = None;
                for mut sub_comp in &*sub_comps.reverse() {
                    sub_comp_vars = StrongComponent::getVariables(sub_comp.clone())?;
                    sub_comp_eqns = StrongComponent::getEquations(sub_comp.clone())?;
                    var_clocks = UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<BClock::BClock>| BClock::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BClock::BClock>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<BClock::BClock>, __a1: metamodelica::Ref<BClock::BClock>| BClock::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BClock::BClock>, metamodelica::Ref<BClock::BClock>) -> Result<bool> + 'static>), 13);
                    for mut var in &*sub_comp_vars {
                        UnorderedSet::add(UnorderedMap::getSafe(var.clone(), var_clock_map.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?, var_clocks.clone())?;
                    }
                    collector = (::match_deref::match_deref! { match &((collector, UnorderedSet::toList(var_clocks.clone()))) {
            (None, Deref @ metamodelica::ListNode::Cons { head: __esc_new_clock, tail: Deref @ metamodelica::ListNode::Nil }) => {
                new_clock = (*__esc_new_clock).clone();
                Some((sub_comp_vars, sub_comp_eqns, new_clock.clone()))
            },
            (Some((__esc_vars, __esc_eqns, __esc_clock)), Deref @ metamodelica::ListNode::Cons { head: __esc_new_clock, tail: Deref @ metamodelica::ListNode::Nil }) => {
                vars = (*__esc_vars).clone();
                eqns = (*__esc_eqns).clone();
                clock = (*__esc_clock).clone();
                new_clock = (*__esc_new_clock).clone();
                if BClock::isEqual(metamodelica::AsArg::as_arg(&clock), metamodelica::AsArg::as_arg(&new_clock))? {
                    collector = Some((listAppend(sub_comp_vars, vars.clone()), listAppend(sub_comp_eqns, eqns.clone()), clock.clone()));
                } else {
                    part = metamodelica::Ref::new(Partition::Partition::Partition { index: 0, association: metamodelica::Ref::new(Partition::Association::Association::CLOCKED { clock: clock.clone(), baseClock: baseClock.clone(), clock_deps: UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<BClock::BClock>| BClock::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BClock::BClock>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<BClock::BClock>, __a1: metamodelica::Ref<BClock::BClock>| BClock::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BClock::BClock>, metamodelica::Ref<BClock::BClock>) -> Result<bool> + 'static>), 13), holdEvents: false }), unknowns: BVariable::VariablePointers::fromList(metamodelica::AsArg::as_arg(&vars), false)?, daeUnknowns: None, equations: BEquation::EquationPointers::fromList(metamodelica::AsArg::as_arg(&eqns))?, adjacencyMatrix: None, matching: None, strongComponents: None });
                    sorted = metamodelica::cons(part, sorted);
                    collector = Some((sub_comp_vars, sub_comp_eqns, new_clock.clone()));
                }
                collector
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.sortClockedPartitions")); __mm_s.push_str(&*literal!(" failed for sub-partitions with cyclic dependency that could not be resolved:\n")); __mm_s.push_str(&*literal!("There are contradicting sub-clocks: ")); __mm_s.push_str(&*List::toString(UnorderedSet::toList(var_clocks), &move |__a0: metamodelica::Ref<BClock::BClock>| BClock::toString(&__a0), List::Style::FLAT_CURLY.clone())?); __mm_s.push_str(&*literal!(" in strong component:\n")); __mm_s.push_str(&*StrongComponent::toString(metamodelica::AsArg::as_arg(&sub_comp), -1)?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                }
                if (collector).is_some() {
                    let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(collector) {
                        Some((__pa2, __pa3, __pa4)) => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    vars = metamodelica::Own::own(__pa2);
                    eqns = metamodelica::Own::own(__pa3);
                    clock = metamodelica::Own::own(__pa4);
                    part = metamodelica::Ref::new(Partition::Partition::Partition { index: 0, association: metamodelica::Ref::new(Partition::Association::Association::CLOCKED { clock: clock, baseClock: baseClock, clock_deps: UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<BClock::BClock>| BClock::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BClock::BClock>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<BClock::BClock>, __a1: metamodelica::Ref<BClock::BClock>| BClock::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BClock::BClock>, metamodelica::Ref<BClock::BClock>) -> Result<bool> + 'static>), 13), holdEvents: false }), unknowns: BVariable::VariablePointers::fromList(&vars, false)?, daeUnknowns: None, equations: BEquation::EquationPointers::fromList(&eqns)?, adjacencyMatrix: None, matching: None, strongComponents: None });
                    sorted = metamodelica::cons(part, sorted);
                }
                sorted
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(sorted)
}

fn collectPartitioningCrefs(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut var_crefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
            let mut newExp: metamodelica::Ref<Expression::NFExpression>;
            let mut arg: metamodelica::Ref<Expression::NFExpression>;
            newExp = (::match_deref::match_deref! { match &(AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL)), literal!("."), true, false)?) {
        Deref @ "subSample" => exp,
        Deref @ "superSample" => exp,
        Deref @ "shiftSample" => exp,
        Deref @ "backSample" => exp,
        Deref @ "previous" => exp,
        Deref @ "hold" => exp,
        Deref @ "sample" => {
            arg = (::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL))?) {
        Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_arg, tail: Deref @ metamodelica::ListNode::Nil } } => {
            arg = (*__esc_arg).clone();
            arg.clone()
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_arg, tail: Deref @ metamodelica::ListNode::Nil } } } => {
            arg = (*__esc_arg).clone();
            arg.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.collectPartitioningCrefs")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            Expression::mapShallow(arg, (std::sync::Arc::new({ let __pe_b1 = var_crefs; move |__pe_a0| collectPartitioningCrefs(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?
        },
        _ => Expression::mapShallow(exp, (std::sync::Arc::new({ let __pe_b1 = var_crefs; move |__pe_a0| collectPartitioningCrefs(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            newExp
        },
        Deref @ Expression::CREF { cref: __exp_cref, .. } => {
            let mut children: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
            let mut stripped: metamodelica::Ref<ComponentRef::NFComponentRef>;
            children = (::match_deref::match_deref! { match &(BVariable::getVar(metamodelica::AsArg::as_arg(&__exp_cref), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { varKind: Deref @ VariableKind::RECORD { children: children_vars, .. }, .. }, .. } => {
            ({
        let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        for mut var in (children_vars.clone()).into_iter().cloned() {
            let __x = BVariable::getVarName(PointerWeak::upgrade(var.clone())?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        _ => {
            list![__exp_cref.clone()]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            for mut child in &*children {
                stripped = ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&child));
                if !(BVariable::checkCref(&stripped, &fnptr!(BVariable::isParamOrConst, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>), metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"))?) {
                    addCrefToSet(stripped, var_crefs.clone())?;
                }
            }
            exp
        },
        _ => {
            Expression::mapShallow(exp, (std::sync::Arc::new({ let __pe_b1 = var_crefs; move |__pe_a0| collectPartitioningCrefs(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn addCrefToSet(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<()> {
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>> = BVariable::getVarPointer(
        &cref,
        metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBPartitioning.mo"),
    )?;
    if BVariable::isState(var_ptr.clone()) {
        UnorderedSet::add(
            BVariable::getPartnerCref(
                &cref,
                &fnptr!(
                    BVariable::getVarDer,
                    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
                ),
                false,
            )?,
            set,
        )?;
    } else if BVariable::isPrevious(var_ptr) {
        UnorderedSet::add(
            BVariable::getPartnerCref(
                &cref,
                &fnptr!(
                    BVariable::getVarPre,
                    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
                ),
                false,
            )?,
            set,
        )?;
    } else {
        UnorderedSet::add(cref, set)?;
    }
    Ok(())
}

fn replaceClockedFunctions(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut held_crefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    pub(crate) fn replaceSample(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut call: &metamodelica::Ref<Call::NFCall>,
        mut basic: bool,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        let mut arg1: metamodelica::Ref<Expression::NFExpression>;
        let mut arg2: metamodelica::Ref<Expression::NFExpression>;
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &((::match_deref::match_deref! { match &(Call::arguments(call)?) {
            Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_arg2, tail: Deref @ metamodelica::ListNode::Nil } } => {
                arg1 = (*__esc_arg1).clone();
                arg2 = (*__esc_arg2).clone();
                list![arg1.clone(), arg2.clone()]
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_arg2, tail: Deref @ metamodelica::ListNode::Nil } } } if (basic) => {
                arg1 = (*__esc_arg1).clone();
                arg2 = (*__esc_arg2).clone();
                list![arg1.clone(), arg2.clone()]
            },
            Deref @ metamodelica::ListNode::Cons { head: __esc_arg1, tail: Deref @ metamodelica::ListNode::Cons { head: __esc_arg2, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } } => {
                arg1 = (*__esc_arg1).clone();
                arg2 = (*__esc_arg2).clone();
                list![arg1.clone(), arg2.clone()]
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.replaceClockedFunctions.replaceSample")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
        arg1 = metamodelica::Own::own(__pa0);
        arg2 = metamodelica::Own::own(__pa1);
        if basic {
            exp = if (Type::isClock(&(Expression::typeOf(arg2)))?) {
                replaceClockedFunctionExp(arg1)?
            } else {
                exp
            };
        } else {
            exp = replaceClockedFunctionExp(arg1)?;
        }
        Ok(exp)
    }

    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
            let mut newExp: metamodelica::Ref<Expression::NFExpression>;
            let mut arg: metamodelica::Ref<Expression::NFExpression>;
            newExp = (::match_deref::match_deref! { match &(AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL)), literal!("."), true, false)?) {
        Deref @ "sample" => replaceSample(exp, metamodelica::AsArg::as_arg(&call), true)?,
        Deref @ "subSample" => replaceSample(exp, metamodelica::AsArg::as_arg(&call), false)?,
        Deref @ "superSample" => replaceSample(exp, metamodelica::AsArg::as_arg(&call), false)?,
        Deref @ "shiftSample" => replaceSample(exp, metamodelica::AsArg::as_arg(&call), false)?,
        Deref @ "backSample" => replaceSample(exp, metamodelica::AsArg::as_arg(&call), false)?,
        Deref @ "hold" => {
            arg = (::match_deref::match_deref! { match &(Call::arguments(var_field!((*exp).call, Expression::NFExpression::CALL))?) {
        Deref @ metamodelica::ListNode::Cons { head: __esc_arg @ Deref @ Expression::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
            arg = (*__esc_arg).clone();
            UnorderedSet::add(var_field!((*arg).cref, Expression::NFExpression::CREF).clone(), held_crefs)?;
            arg.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBPartitioning.replaceClockedFunctions")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Expression::toString(exp)?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            replaceClockedFunctionExp(arg)?
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            newExp
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn replaceClockedFunctionExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut func: metamodelica::Ref<Function::Function>;
    func = (match &*(Expression::typeOf(exp.clone())) {
        Type::REAL => NFBuiltinFuncs::GET_PART_REAL().clone(),
        Type::INTEGER => NFBuiltinFuncs::GET_PART_INT().clone(),
        Type::BOOLEAN => NFBuiltinFuncs::GET_PART_BOOL().clone(),
        Type::CLOCK => NFBuiltinFuncs::GET_PART_CLOCK().clone(),
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBPartitioning.replaceClockedFunctionExp"));
                    __mm_s.push_str(&*literal!(" failed. "));
                    __mm_s.push_str(&*Expression::toString(exp.clone())?);
                    __mm_s.push_str(&*literal!(" is of type "));
                    __mm_s.push_str(&*Type::toString(&(Expression::typeOf(exp.clone())))?);
                    __mm_s.push_str(&*literal!(", only real, integer, boolean and clock are allowed."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    exp = metamodelica::Ref::new(Expression::NFExpression::CALL {
        call: Call::makeTypedCall(
            func.clone(),
            list![exp.clone()],
            Expression::variability(exp)?,
            NFPrefixes::Purity::PURE.clone(),
            func.returnType.clone(),
        ),
    });
    Ok(exp)
}

fn replaceClockedWhen(mut eqn: metamodelica::Ref<Equation::Equation>) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    eqn = (::match_deref::match_deref! { match &(eqn.clone()) {
        Deref @ BEquation::Equation::WHEN_EQUATION { body: Deref @ WhenEquationBody::WHEN_EQUATION_BODY { condition: cond, when_stmts: Deref @ metamodelica::ListNode::Cons { head: stmt, tail: Deref @ metamodelica::ListNode::Nil }, else_when: None }, attr: __eqn_attr, .. } if (Type::isClock(&(Expression::typeOf(cond.clone())))?) => {
            BEquation::WhenStatement::toEquation(metamodelica::AsArg::as_arg(&stmt), __eqn_attr.clone(), false)?
        },
        _ => {
            eqn
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqn)
}
