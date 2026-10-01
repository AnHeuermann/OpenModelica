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

use crate::NBEquation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationKind;
use crate::NBEquation::WhenStatement;
use crate::NBPartitioning::BClock;
use crate::NSimCode::SimCodeIndices;
use crate::NSimStrongComponent::Block;
use crate::NSimVar::SimVar;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFBuiltinFuncs as BuiltinFuncs;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFClockKind as ClockKind;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFPrefixes as Prefixes;
use openmodelica_simcode_types::SimCode as OldSimCode;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_util::Error;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util_datatypes_basic::List;

/// file:        NSimPartition.mo
/// package:     NSimPartition
/// description: This file contains the data types and functions for clocked partitions
///              in simulation code phase.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NSimPartition {
    BASE_PARTITION {
        baseClock: metamodelica::Ref<BClock::BClock>,
        subPartitions: metamodelica::List<metamodelica::Ref<NSimPartition>>,
    },
    SUB_PARTITION {
        variables: metamodelica::List<(metamodelica::Ref<SimVar::SimVar>, bool)>,
        equations: metamodelica::List<metamodelica::Ref<Block::Block>>,
        removedEquations: metamodelica::List<metamodelica::Ref<Block::Block>>,
        subClock: metamodelica::Ref<BClock::BClock>,
        holdEvents: bool,
    },
}
impl metamodelica::gc::MMTrace for NSimPartition {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NSimPartition::BASE_PARTITION {
                baseClock,
                subPartitions,
            } => {
                metamodelica::gc::MMTrace::mm_accept(baseClock, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(subPartitions, __mmv)?;
                Ok(())
            }
            NSimPartition::SUB_PARTITION {
                variables,
                equations,
                removedEquations,
                subClock,
                holdEvents,
            } => {
                metamodelica::gc::MMTrace::mm_accept(variables, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(equations, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(removedEquations, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(subClock, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(holdEvents, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for NSimPartition {
    fn default() -> Self {
        Self::BASE_PARTITION {
            baseClock: Default::default(),
            subPartitions: Default::default(),
        }
    }
}
pub use self::NSimPartition::{BASE_PARTITION, SUB_PARTITION};
pub(crate) fn createSubPartition(
    mut subClock: metamodelica::Ref<BClock::BClock>,
    mut equations: metamodelica::List<metamodelica::Ref<Block::Block>>,
    mut variables: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
    mut holdEvents: bool,
) -> metamodelica::Ref<NSimPartition> {
    let mut part: metamodelica::Ref<NSimPartition>;
    part = metamodelica::Ref::new(NSimPartition::SUB_PARTITION {
        variables: ({
            let mut __acc: metamodelica::List<(metamodelica::Ref<SimVar::SimVar>, bool)> = metamodelica::nil();
            for mut v in (variables).into_iter().cloned() {
                let __x = (v.clone(), true);
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        equations: equations,
        removedEquations: metamodelica::nil(),
        subClock: subClock,
        holdEvents: holdEvents,
    });
    part
}

pub(crate) fn createBasePartitions(
    mut clock_collector: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<BClock::BClock>,
            metamodelica::List<metamodelica::Ref<NSimPartition>>,
        >,
    >,
    mut simCodeIndices: SimCodeIndices,
) -> (
    metamodelica::List<metamodelica::Ref<NSimPartition>>,
    metamodelica::List<metamodelica::Ref<Block::Block>>,
    SimCodeIndices,
) {
    let mut baseParts: metamodelica::List<metamodelica::Ref<NSimPartition>> = metamodelica::nil();
    let mut eventClocks: metamodelica::List<metamodelica::Ref<Block::Block>> = metamodelica::nil();
    let mut simCodeIndices: SimCodeIndices = simCodeIndices;
    let mut baseClock: metamodelica::Ref<BClock::BClock>;
    let mut subClocks: metamodelica::List<metamodelica::Ref<NSimPartition>>;
    let mut clock_idx: i32 = 1;
    for mut tpl in &*UnorderedMap::toList(clock_collector) {
        (baseClock, subClocks) = tpl.clone();
        if !(BClock::isInferredClock(&baseClock)) {
            baseParts = metamodelica::cons(
                metamodelica::Ref::new(NSimPartition::BASE_PARTITION {
                    baseClock: baseClock,
                    subPartitions: subClocks,
                }),
                baseParts,
            );
        }
    }
    for mut base in &*baseParts {
        let () = (::match_deref::match_deref! { match &(base.clone()) {
            Deref @ BASE_PARTITION { baseClock: Deref @ BClock::BASE_CLOCK { clock: Deref @ ClockKind::EVENT_CLOCK { condition: Deref @ Expression::CREF { cref: cond, .. }, .. } }, .. } => {
                let mut source: metamodelica::Ref<DAE::ElementSource>;
                let mut fire: metamodelica::Ref<Expression::NFExpression>;
                let mut stmt: metamodelica::Ref<WhenStatement::WhenStatement>;
                let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>;
                let mut blck: metamodelica::Ref<Block::Block>;
                source = DAE::emptyElementSource().clone();
                fire = metamodelica::Ref::new(Expression::NFExpression::CALL { call: Call::makeTypedCall(BuiltinFuncs::CLOCK_FIRE().clone(), list![metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: clock_idx })], Prefixes::Variability::CONSTANT.clone(), Prefixes::Purity::PURE.clone(), BuiltinFuncs::CLOCK_FIRE().returnType.clone()) });
                stmt = metamodelica::Ref::new(WhenStatement::WhenStatement::NORETCALL { exp: fire, source: source.clone() });
                attr = NBEquation::default(EquationKind::EMPTY.clone(), false, None, None);
                blck = metamodelica::Ref::new(Block::Block::WHEN { index: simCodeIndices.equationIndex.clone(), initialCall: false, conditions: list![cond.clone()], when_stmts: list![stmt], else_when: None, source: source, attr: attr });
                eventClocks = metamodelica::cons(blck, eventClocks);
                simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
                ()
            },
            _ => {
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        clock_idx = clock_idx + 1;
    }
    (baseParts, eventClocks, simCodeIndices)
}

pub(crate) fn getClock(mut part: &metamodelica::Ref<NSimPartition>) -> Result<metamodelica::Ref<BClock::BClock>> {
    let mut clock: metamodelica::Ref<BClock::BClock>;
    clock = (match &**part {
        BASE_PARTITION {
            baseClock: __part_baseClock,
            ..
        } => __part_baseClock.clone(),
        SUB_PARTITION {
            subClock: __part_subClock,
            ..
        } => __part_subClock.clone(),
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NSimPartition.getClock"));
                    __mm_s.push_str(&*literal!(" failed for unknown partition:\n"));
                    __mm_s.push_str(&*toString(part, literal!(""))?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(clock)
}

pub(crate) fn listToString(
    mut parts: &metamodelica::List<metamodelica::Ref<NSimPartition>>,
    mut r#str: ArcStr,
    mut header: &ArcStr,
) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    let mut indent: ArcStr = r#str.clone();
    r#str = if (!metamodelica::stringEq(&header, &(literal!("")))) {
        StringUtil::headline_3(header)?
    } else {
        literal!("")
    };
    for mut part in &**parts {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*toString(metamodelica::AsArg::as_arg(&part), indent.clone())?);
            ArcStr::from(__mm_s)
        };
    }
    Ok(r#str)
}

pub(crate) fn toString(mut part: &metamodelica::Ref<NSimPartition>, mut r#str: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    r#str = (match &**part {
        BASE_PARTITION {
            baseClock: __part_baseClock,
            subPartitions: __part_subPartitions,
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[BASE] Partition "));
            __mm_s.push_str(&*BClock::toString(metamodelica::AsArg::as_arg(&__part_baseClock))?);
            __mm_s.push_str(&*List::toString(
                __part_subPartitions.clone(),
                &({
                    let __pe_b1 = r#str;
                    move |__pe_a0| toString(&__pe_a0, __pe_b1.clone())
                }),
                List::Style::NEWLINE.clone(),
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        }
        SUB_PARTITION {
            equations: __part_equations,
            subClock: __part_subClock,
            ..
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("[SUB-] Partition "));
            __mm_s.push_str(&*BClock::toString(metamodelica::AsArg::as_arg(&__part_subClock))?);
            __mm_s.push_str(&*List::toString(
                __part_equations.clone(),
                &({
                    let __pe_b1 = r#str;
                    move |__pe_a0| Block::toString(&__pe_a0, __pe_b1.clone())
                }),
                List::Style::NEWLINE.clone(),
            )?);
            ArcStr::from(__mm_s)
        }
        _ => literal!("[ERR-]"),
    });
    Ok(r#str)
}

pub(crate) fn toStringShort(mut part: &metamodelica::Ref<NSimPartition>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match &**part {
        BASE_PARTITION {
            baseClock: __part_baseClock,
            ..
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[BASE] Partition "));
            __mm_s.push_str(&*BClock::toString(metamodelica::AsArg::as_arg(&__part_baseClock))?);
            ArcStr::from(__mm_s)
        }
        SUB_PARTITION {
            subClock: __part_subClock,
            ..
        } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[SUB-] Partition "));
            __mm_s.push_str(&*BClock::toString(metamodelica::AsArg::as_arg(&__part_subClock))?);
            ArcStr::from(__mm_s)
        }
        _ => literal!("[ERR-]"),
    });
    Ok(r#str)
}

pub(crate) fn convertBase(mut part: &metamodelica::Ref<NSimPartition>) -> Result<OldSimCode::ClockedPartition> {
    let mut oldPart: OldSimCode::ClockedPartition;
    oldPart = (match &**part {
        BASE_PARTITION {
            baseClock: __part_baseClock,
            subPartitions: __part_subPartitions,
        } => OldSimCode::ClockedPartition {
            baseClock: BClock::convertBase(metamodelica::AsArg::as_arg(&__part_baseClock))?,
            subPartitions: ({
                let mut __acc: metamodelica::List<OldSimCode::SubPartition> = metamodelica::nil();
                for mut sub in (__part_subPartitions.clone()).into_iter().cloned() {
                    let __x = convertSub(&(sub.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        },
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NSimPartition.convertBase"));
                    __mm_s.push_str(&*literal!(" failed for non-base partition:\n"));
                    __mm_s.push_str(&*toString(part, literal!(""))?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(oldPart)
}

pub(crate) fn convertSub(mut part: &metamodelica::Ref<NSimPartition>) -> Result<OldSimCode::SubPartition> {
    let mut oldPart: OldSimCode::SubPartition;
    oldPart = (match &**part {
        SUB_PARTITION {
            equations: __part_equations,
            holdEvents: __part_holdEvents,
            removedEquations: __part_removedEquations,
            subClock: __part_subClock,
            variables: __part_variables,
        } => OldSimCode::SubPartition {
            vars: ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<SimCodeVar::SimVar>, bool)> = metamodelica::nil();
                for mut tpl in (__part_variables.clone()).into_iter().cloned() {
                    let __x = SimVar::convertTpl(&(tpl.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            equations: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<OldSimCode::SimEqSystem>> = metamodelica::nil();
                for mut blck in (__part_equations.clone()).into_iter().cloned() {
                    let __x = Block::convert(&(blck.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            removedEquations: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<OldSimCode::SimEqSystem>> = metamodelica::nil();
                for mut blck in (__part_removedEquations.clone()).into_iter().cloned() {
                    let __x = Block::convert(&(blck.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            subClock: BClock::convertSub(metamodelica::AsArg::as_arg(&__part_subClock))?,
            holdEvents: __part_holdEvents.clone(),
        },
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NSimPartition.convertSub"));
                    __mm_s.push_str(&*literal!(" failed for non-base partition:\n"));
                    __mm_s.push_str(&*toString(part, literal!(""))?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(oldPart)
}
