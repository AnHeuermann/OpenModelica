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

use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend::UnitAbsyn;
use openmodelica_frontend::UnitAbsynBuilder;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::HashTable;
use openmodelica_frontend_dump::HashTableCrIListArray;
use openmodelica_frontend_dump::HashTableCrILst;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_script_util::UnitParserExt;
use openmodelica_simcode_types::AvlTreeCRToInt;
use openmodelica_simcode_types::HashTableCrefSimVar;
use openmodelica_simcode_types::SimCode;
use openmodelica_simcode_types::SimCodeFunction;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_simcode_util::SimCodeFunctionUtil;
use openmodelica_simcode_util::SimCodeUtilShared;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

fn compareEqSystems(
    mut eq1: &metamodelica::Ref<SimCode::SimEqSystem>,
    mut eq2: &metamodelica::Ref<SimCode::SimEqSystem>,
) -> Result<bool> {
    let mut b: bool;
    b = simEqSystemIndex(eq1)? > simEqSystemIndex(eq2)?;
    Ok(b)
}

pub fn sortEqSystems(
    mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
) -> Result<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>> {
    let mut outEqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    outEqs = List::flatten(
        ({
            let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>> =
                metamodelica::nil();
            for mut eq in (eqs).into_iter().cloned() {
                let __x = expandEntwined(eq.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    )?;
    outEqs = List::sort(
        outEqs,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<SimCode::SimEqSystem>, __a1: metamodelica::Ref<SimCode::SimEqSystem>| {
                compareEqSystems(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<SimCode::SimEqSystem>,
                        metamodelica::Ref<SimCode::SimEqSystem>,
                    ) -> Result<bool>
                    + 'static,
            >),
    )?;
    Ok(outEqs)
}

fn expandEntwined(
    mut eq: metamodelica::Ref<SimCode::SimEqSystem>,
) -> metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>> {
    let mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    eqs = (match &*eq.clone() {
        SimCode::SimEqSystem::SES_ENTWINED_ASSIGN {
            single_calls: __eq_single_calls,
            ..
        } => metamodelica::cons(eq, __eq_single_calls.clone()),
        _ => list![eq],
    });
    eqs
}

pub fn getClockIndex(
    mut simVar: &metamodelica::Ref<SimCodeVar::SimVar>,
    mut simCode: &metamodelica::Ref<SimCode::SimCode>,
) -> Result<Option<i32>> {
    let mut clockIndex: Option<i32>;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut clkHT: (
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
    cref = getSimVarCompRef(simVar);
    clockIndex = (match &**simCode {
        SimCode::SimCode {
            crefToClockIndexHT: __esc_clkHT,
            ..
        } => {
            clkHT = (*__esc_clkHT).clone();
            if (BaseHashTable::hasKey(cref.clone(), &(clkHT.clone()))?) {
                Some(BaseHashTable::get(cref, &(clkHT.clone()))?)
            } else {
                None
            }
        }
    });
    Ok(clockIndex)
}

pub fn getSimVarCompRef(mut inVar: &metamodelica::Ref<SimCodeVar::SimVar>) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outComp: metamodelica::Ref<DAE::ComponentRef>;
    outComp = inVar.name.clone();
    outComp
}

pub fn getSubPartitions(
    mut inPartitions: metamodelica::List<SimCode::ClockedPartition>,
) -> Result<metamodelica::List<SimCode::SubPartition>> {
    let mut outSubPartitions: metamodelica::List<SimCode::SubPartition>;
    outSubPartitions = List::flatten(List::map(
        inPartitions,
        &move |__a0: SimCode::ClockedPartition| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(getSubPartition(&__a0))
        },
    )?)?;
    Ok(outSubPartitions)
}

pub(crate) fn getSubPartition(
    mut inPartition: &SimCode::ClockedPartition,
) -> metamodelica::List<SimCode::SubPartition> {
    let mut outSubPartitions: metamodelica::List<SimCode::SubPartition>;
    outSubPartitions = inPartition.subPartitions.clone();
    outSubPartitions
}

pub fn getClockedEquations(
    mut inSubPartitions: &metamodelica::List<SimCode::SubPartition>,
) -> metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>> {
    let mut outEqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>> = metamodelica::nil();
    for mut part in &**inSubPartitions {
        outEqs = listAppend(part.equations.clone(), outEqs);
        outEqs = listAppend(part.removedEquations.clone(), outEqs);
    }
    outEqs
}

pub fn jacobianColumnsAreEmpty(mut columns: &metamodelica::List<metamodelica::Ref<SimCode::JacobianColumn>>) -> bool {
    let mut b: bool = true;
    for mut col in &**columns {
        if !((col.columnEqns).is_empty() && (col.constantEqns).is_empty()) {
            b = false;
            return b;
        }
    }
    b
}

pub fn stripAsubIfNoIter(mut exp: metamodelica::Ref<DAE::Exp>, mut hasIter: bool) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = if (hasIter) {
        exp
    } else {
        (::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ DAE::Exp::RELATION { optionExpisASUB: Some(_), exp1: __exp_exp1, exp2: __exp_exp2, index: __exp_index, operator: __exp_operator } => metamodelica::Ref::new(DAE::Exp::RELATION { exp1: __exp_exp1.clone(), operator: __exp_operator.clone(), exp2: __exp_exp2.clone(), index: __exp_index.clone(), optionExpisASUB: None }),
            Deref @ DAE::Exp::LBINARY { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator } => metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: stripAsubIfNoIter(__exp_exp1.clone(), hasIter), operator: __exp_operator.clone(), exp2: stripAsubIfNoIter(__exp_exp2.clone(), hasIter) }),
            Deref @ DAE::Exp::LUNARY { exp: __exp_exp, operator: __exp_operator } => metamodelica::Ref::new(DAE::Exp::LUNARY { operator: __exp_operator.clone(), exp: stripAsubIfNoIter(__exp_exp.clone(), hasIter) }),
            _ => exp,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    };
    outExp
}

pub fn dimsToAllIndexes(
    mut inDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut outIndexes: metamodelica::List<metamodelica::List<i32>>;
    let mut ilst: metamodelica::List<i32>;
    let mut lstlst: metamodelica::List<metamodelica::List<i32>>;
    ilst = Expression::dimensionsSizes(inDims)?;
    lstlst = List::map(ilst, &fnptr!(List::intRange, i32))?;
    outIndexes = dimsToAllIndexes1(&lstlst)?;
    Ok(outIndexes)
}

fn dimsToAllIndexes1(
    mut inDims: &metamodelica::List<metamodelica::List<i32>>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut oAllIndex: metamodelica::List<metamodelica::List<i32>>;
    oAllIndex = (::match_deref::match_deref! { match inDims {
        Deref @ metamodelica::ListNode::Cons { head: dims, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut indxes: metamodelica::List<metamodelica::List<i32>>;
            indxes = List::map(dims.clone(), &fnptr!(List::create, _))?;
            indxes
        },
        Deref @ metamodelica::ListNode::Cons { head: dims, tail: rest } => {
            let mut indxes: metamodelica::List<metamodelica::List<i32>>;
            indxes = dimsToAllIndexes1(rest)?;
            indxes = List::fold1(metamodelica::AsArg::as_arg(&dims), &dimsToAllIndexes2, indxes, metamodelica::nil())?;
            indxes
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(oAllIndex)
}

fn dimsToAllIndexes2(
    mut i: i32,
    mut iIndex: metamodelica::List<metamodelica::List<i32>>,
    mut iAllIndex: metamodelica::List<metamodelica::List<i32>>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut oAllIndex: metamodelica::List<metamodelica::List<i32>>;
    oAllIndex = List::map1(iIndex, &fnptr!(List::consr, _, _), i)?;
    oAllIndex = listAppend(iAllIndex, oAllIndex);
    Ok(oAllIndex)
}

pub fn getDefaultFmiInitialAttribute(
    mut variability: SimCodeVar::Variability,
    mut causality: SimCodeVar::Causality,
) -> SimCodeVar::Initial {
    let mut initial_: SimCodeVar::Initial;
    initial_ = (match (variability, causality) {
        (SimCodeVar::Variability::CONSTANT { .. }, SimCodeVar::Causality::OUTPUT { .. }) => {
            openmodelica_simcode_types::SimCodeVar::Initial::EXACT
        }
        (SimCodeVar::Variability::CONSTANT { .. }, SimCodeVar::Causality::LOCAL { .. }) => {
            openmodelica_simcode_types::SimCodeVar::Initial::EXACT
        }
        (SimCodeVar::Variability::FIXED { .. }, SimCodeVar::Causality::PARAMETER { .. }) => {
            openmodelica_simcode_types::SimCodeVar::Initial::EXACT
        }
        (SimCodeVar::Variability::FIXED { .. }, SimCodeVar::Causality::CALCULATED_PARAMETER { .. }) => {
            openmodelica_simcode_types::SimCodeVar::Initial::CALCULATED
        }
        (SimCodeVar::Variability::FIXED { .. }, SimCodeVar::Causality::LOCAL { .. }) => {
            openmodelica_simcode_types::SimCodeVar::Initial::CALCULATED
        }
        (SimCodeVar::Variability::TUNABLE { .. }, SimCodeVar::Causality::PARAMETER { .. }) => {
            openmodelica_simcode_types::SimCodeVar::Initial::EXACT
        }
        (SimCodeVar::Variability::TUNABLE { .. }, SimCodeVar::Causality::CALCULATED_PARAMETER { .. }) => {
            openmodelica_simcode_types::SimCodeVar::Initial::CALCULATED
        }
        (SimCodeVar::Variability::TUNABLE { .. }, SimCodeVar::Causality::LOCAL { .. }) => {
            openmodelica_simcode_types::SimCodeVar::Initial::CALCULATED
        }
        (SimCodeVar::Variability::DISCRETE { .. }, SimCodeVar::Causality::OUTPUT { .. }) => {
            openmodelica_simcode_types::SimCodeVar::Initial::CALCULATED
        }
        (SimCodeVar::Variability::DISCRETE { .. }, SimCodeVar::Causality::LOCAL { .. }) => {
            openmodelica_simcode_types::SimCodeVar::Initial::CALCULATED
        }
        (SimCodeVar::Variability::CONTINUOUS { .. }, SimCodeVar::Causality::OUTPUT { .. }) => {
            openmodelica_simcode_types::SimCodeVar::Initial::CALCULATED
        }
        (SimCodeVar::Variability::CONTINUOUS { .. }, SimCodeVar::Causality::LOCAL { .. }) => {
            openmodelica_simcode_types::SimCodeVar::Initial::CALCULATED
        }
        _ => openmodelica_simcode_types::SimCodeVar::Initial::NONE_INITIAL,
    });
    initial_
}

pub fn getFmiInitialAttributeStr(mut simVar: &metamodelica::Ref<SimCodeVar::SimVar>) -> Result<ArcStr> {
    let mut out_string: ArcStr = literal!("");
    let mut var_initial: SimCodeVar::Initial;
    let mut default_initial: SimCodeVar::Initial;
    if (simVar.initial_).is_none() {
        return Ok(out_string);
    }
    let __pa0 = ::match_deref::match_deref! { match &(simVar.initial_.clone()) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    var_initial = metamodelica::Own::own(__pa0);
    default_initial = getDefaultFmiInitialAttribute(
        simVar
            .variability
            .clone()
            .unwrap_or(openmodelica_simcode_types::SimCodeVar::Variability::CONTINUOUS),
        simVar
            .causality
            .clone()
            .unwrap_or(openmodelica_simcode_types::SimCodeVar::Causality::LOCAL),
    );
    if var_initial == default_initial && !(Flags::isSet(Flags::DUMP_FORCE_FMI_ATTRIBUTES.clone())?) {
        var_initial = openmodelica_simcode_types::SimCodeVar::Initial::NONE_INITIAL;
    }
    out_string = (match var_initial {
        SimCodeVar::Initial::EXACT => literal!("exact"),
        SimCodeVar::Initial::APPROX => literal!("approx"),
        SimCodeVar::Initial::CALCULATED => literal!("calculated"),
        SimCodeVar::Initial::NONE_INITIAL => literal!(""),
    });
    Ok(out_string)
}

pub fn simGenericCallString(mut call: &SimCode::SimGenericCall) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match call.clone() {
        SimCode::SimGenericCall::SINGLE_GENERIC_CALL { .. } => {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("single generic call "));
                __mm_s.push_str(&*intString(
                    var_field!(call.index, SimCode::SimGenericCall::SINGLE_GENERIC_CALL).clone(),
                ));
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*List::toString(
                    var_field!(call.iters, SimCode::SimGenericCall::SINGLE_GENERIC_CALL).clone(),
                    &move |__a0: BackendDAE::SimIterator| simIteratorString(&__a0),
                    List::Style::FLAT_CURLY.clone(),
                )?);
                ArcStr::from(__mm_s)
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("\n  "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(
                    var_field!(call.lhs, SimCode::SimGenericCall::SINGLE_GENERIC_CALL).clone(),
                )?);
                __mm_s.push_str(&*literal!(" = "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(
                    var_field!(call.rhs, SimCode::SimGenericCall::SINGLE_GENERIC_CALL).clone(),
                )?);
                __mm_s.push_str(&*literal!(";"));
                ArcStr::from(__mm_s)
            };
            r#str
        }
        SimCode::SimGenericCall::IF_GENERIC_CALL { .. } => {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("if generic call "));
                __mm_s.push_str(&*intString(
                    var_field!(call.index, SimCode::SimGenericCall::IF_GENERIC_CALL).clone(),
                ));
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*List::toString(
                    var_field!(call.iters, SimCode::SimGenericCall::IF_GENERIC_CALL).clone(),
                    &move |__a0: BackendDAE::SimIterator| simIteratorString(&__a0),
                    List::Style::FLAT_CURLY.clone(),
                )?);
                ArcStr::from(__mm_s)
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*List::toString(
                    var_field!(call.branches, SimCode::SimGenericCall::IF_GENERIC_CALL).clone(),
                    &move |__a0: SimCode::SimBranch| simBranchString(&__a0),
                    List::Style::NEWLINE.clone(),
                )?);
                ArcStr::from(__mm_s)
            };
            r#str
        }
        SimCode::SimGenericCall::WHEN_GENERIC_CALL { .. } => {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("when generic call "));
                __mm_s.push_str(&*intString(
                    var_field!(call.index, SimCode::SimGenericCall::WHEN_GENERIC_CALL).clone(),
                ));
                __mm_s.push_str(&*literal!(" "));
                __mm_s.push_str(&*List::toString(
                    var_field!(call.iters, SimCode::SimGenericCall::WHEN_GENERIC_CALL).clone(),
                    &move |__a0: BackendDAE::SimIterator| simIteratorString(&__a0),
                    List::Style::FLAT_CURLY.clone(),
                )?);
                ArcStr::from(__mm_s)
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*List::toString(
                    var_field!(call.branches, SimCode::SimGenericCall::WHEN_GENERIC_CALL).clone(),
                    &move |__a0: SimCode::SimBranch| simBranchString(&__a0),
                    List::Style::NEWLINE.clone(),
                )?);
                ArcStr::from(__mm_s)
            };
            r#str
        }
        _ => literal!(""),
    });
    Ok(r#str)
}

pub(crate) fn simBranchString(mut branch: &SimCode::SimBranch) -> Result<ArcStr> {
    fn simBranchBodyString(mut tpl: (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)) -> Result<ArcStr> {
        let mut r#str: ArcStr = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*ExpressionBasics::printExpStr(Util::tuple21(tpl.clone()))?);
            __mm_s.push_str(&*literal!(" = "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(Util::tuple22(tpl.clone()))?);
            __mm_s.push_str(&*literal!(";"));
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    let mut r#str: ArcStr;
    r#str = (match branch.clone() {
        SimCode::SimBranch::SIM_BRANCH { .. } => {
            let mut b: bool;
            b = (var_field!(branch.condition, SimCode::SimBranch::SIM_BRANCH)).is_some();
            r#str = if (b) {
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("if "));
                    __mm_s.push_str(&*ExpressionBasics::printExpStr(
                        var_field!(branch.condition, SimCode::SimBranch::SIM_BRANCH)
                            .clone()
                            .ok_or("pattern mismatch")?,
                    )?);
                    __mm_s.push_str(&*literal!(" then\n"));
                    ArcStr::from(__mm_s)
                }
            } else {
                literal!("else\n")
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*List::toString(
                    var_field!(branch.body, SimCode::SimBranch::SIM_BRANCH).clone(),
                    &simBranchBodyString,
                    List::Style::NEWLINE_INDENT.clone(),
                )?);
                ArcStr::from(__mm_s)
            };
            r#str = if (b) {
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*literal!("end if;"));
                    ArcStr::from(__mm_s)
                }
            } else {
                r#str
            };
            r#str
        }
        SimCode::SimBranch::SIM_BRANCH_STMT { .. } => {
            let mut b: bool;
            b = (var_field!(branch.condition, SimCode::SimBranch::SIM_BRANCH_STMT)).is_some();
            r#str = if (b) {
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("if "));
                    __mm_s.push_str(&*ExpressionBasics::printExpStr(
                        var_field!(branch.condition, SimCode::SimBranch::SIM_BRANCH_STMT)
                            .clone()
                            .ok_or("pattern mismatch")?,
                    )?);
                    __mm_s.push_str(&*literal!(" then\n"));
                    ArcStr::from(__mm_s)
                }
            } else {
                literal!("else\n")
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*List::toString(
                    var_field!(branch.body, SimCode::SimBranch::SIM_BRANCH_STMT).clone(),
                    &fnptr!(DAEDump::ppStatementStr, metamodelica::Ref<DAE::Statement>),
                    List::Style::NEWLINE_INDENT.clone(),
                )?);
                ArcStr::from(__mm_s)
            };
            r#str = if (b) {
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*literal!("\nend if;"));
                    ArcStr::from(__mm_s)
                }
            } else {
                r#str
            };
            r#str
        }
        _ => {
            literal!("")
        }
    });
    Ok(r#str)
}

pub fn simVarString(mut inVar: &metamodelica::Ref<SimCodeVar::SimVar>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("index:"));
        __mm_s.push_str(&*intString(inVar.index.clone()));
        __mm_s.push_str(&*literal!(": "));
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&inVar.name)?);
        ArcStr::from(__mm_s)
    };
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*s);
        __mm_s.push_str(
            &*(match inVar.aliasvar.clone() {
                SimCodeVar::AliasVariable::NOALIAS { .. } => {
                    literal!(" (no alias) ")
                }
                SimCodeVar::AliasVariable::ALIAS { varName: ref cr } => {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(" (alias: "));
                    __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(
                        metamodelica::AsArg::as_arg(&cr),
                    )?);
                    __mm_s.push_str(&*literal!(") "));
                    ArcStr::from(__mm_s)
                }
                SimCodeVar::AliasVariable::NEGATEDALIAS { varName: ref cr } => {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(" (negated alias: "));
                    __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(
                        metamodelica::AsArg::as_arg(&cr),
                    )?);
                    __mm_s.push_str(&*literal!(") "));
                    ArcStr::from(__mm_s)
                }
            }),
        );
        ArcStr::from(__mm_s)
    };
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*s);
        __mm_s.push_str(&*if (inVar.isProtected.clone()) {
            literal!(" protected ")
        } else {
            literal!("")
        });
        ArcStr::from(__mm_s)
    };
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*s);
        __mm_s.push_str(&*if (inVar.hideResult.clone().unwrap_or(false)) {
            literal!(" hideResult ")
        } else {
            literal!("")
        });
        ArcStr::from(__mm_s)
    };
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*s);
        __mm_s.push_str(&*literal!(" initial: "));
        __mm_s.push_str(&*if ((inVar.initialValue).is_some()) {
            ExpressionDump::printOptExpStr(inVar.initialValue.clone())?
        } else {
            literal!("")
        });
        ArcStr::from(__mm_s)
    };
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*s);
        __mm_s.push_str(&*if ((inVar.arrayCref).is_some()) {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\tarrCref:"));
                __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(
                    &(inVar.arrayCref.clone().ok_or("pattern mismatch")?),
                )?);
                ArcStr::from(__mm_s)
            }
        } else {
            literal!("\tno arrCref")
        });
        ArcStr::from(__mm_s)
    };
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*s);
        __mm_s.push_str(&*literal!(" index:("));
        __mm_s.push_str(&*if ((inVar.variable_index).is_some()) {
            intString(inVar.variable_index.clone().ok_or("pattern mismatch")?)
        } else {
            literal!("")
        });
        __mm_s.push_str(&*literal!(")"));
        ArcStr::from(__mm_s)
    };
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*s);
        __mm_s.push_str(&*literal!(" ["));
        __mm_s.push_str(&*stringDelimitList(inVar.numArrayElement.clone(), literal!(",")));
        __mm_s.push_str(&*literal!("]"));
        ArcStr::from(__mm_s)
    };
    Ok(s)
}

pub fn setVariableIndexHelper(
    mut inVars: &metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    mut inIndex: i32,
    mut inFMIIndex: i32,
) -> Result<(metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>, i32, i32)> {
    let mut outVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut outIndex: i32;
    let mut outFMIIndex: i32;
    let (__pa0, (__pa1, __pa2)) = List::mapFold(inVars, &setVariableIndexHelper2, (inIndex, inFMIIndex))?;
    outVars = metamodelica::Own::own(__pa0);
    outIndex = metamodelica::Own::own(__pa1);
    outFMIIndex = metamodelica::Own::own(__pa2);
    Ok((outVars, outIndex, outFMIIndex))
}

fn setVariableIndexHelper2(
    mut var: metamodelica::Ref<SimCodeVar::SimVar>,
    mut tpl: (i32, i32),
) -> Result<(metamodelica::Ref<SimCodeVar::SimVar>, (i32, i32))> {
    let mut var: metamodelica::Ref<SimCodeVar::SimVar> = var;
    let mut tpl: (i32, i32) = tpl;
    let mut index: i32;
    let mut fmi_index: i32;
    (index, fmi_index) = tpl;
    assign_field!(var.variable_index = Some(index));
    index = index + SimCodeUtilShared::getNumElems(&var)?;
    if (var.exportVar).is_some() {
        assign_field!(var.fmi_index = Some(fmi_index));
        fmi_index = fmi_index + SimCodeUtilShared::getNumElems(&var)?;
    } else {
        assign_field!(var.fmi_index = None);
    }
    tpl = (index, fmi_index);
    Ok((var, tpl))
}

pub fn unitConversion(mut to: ArcStr, mut from: ArcStr) -> (bool, metamodelica::Real, metamodelica::Real) {
    let mut converts: bool = false;
    let mut factor: metamodelica::Real = metamodelica::OrderedFloat(1.0_f64);
    let mut offset: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut u1: UnitAbsyn::Unit;
    let mut u2: UnitAbsyn::Unit;
    let mut factor1: metamodelica::Real;
    let mut factor2: metamodelica::Real;
    let mut offset1: metamodelica::Real;
    let mut offset2: metamodelica::Real;
    if '__try0: {
        UnitParserExt::initSIUnits();
        (u1, factor1, offset1) =
            unwrap_break_err!(UnitAbsynBuilder::str2unitWithScaleFactor(to.clone(), None), '__try0);
        (u2, factor2, offset2) =
            unwrap_break_err!(UnitAbsynBuilder::str2unitWithScaleFactor(from.clone(), None), '__try0);
        let true = (u1.clone() == u2.clone()) else {
            break '__try0 Err::<_, _>("pattern mismatch");
        };
        factor = unwrap_break_err!(metamodelica::real_div_checked(factor2, factor1), '__try0);
        offset = unwrap_break_err!(metamodelica::real_div_checked((offset2 - offset1), factor1), '__try0);
        converts = true;
        Ok::<(), &'static str>(())
    }
    .is_err()
    {}
    (converts, factor, offset)
}

pub fn createCrefToSimVarHT(
    mut modelInfo: &SimCode::ModelInfo,
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<
            Option<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::Ref<SimCodeVar::SimVar>,
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
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<SimCodeVar::SimVar>) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut outHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<SimCodeVar::SimVar>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCrefSimVar::FuncHashCref,
            HashTableCrefSimVar::FuncCrefEqual,
            HashTableCrefSimVar::FuncCrefStr,
            HashTableCrefSimVar::FuncExpStr,
        ),
    );
    let mut size: i32;
    let mut varInfo: SimCode::VarInfo;
    let mut arraySimVars: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
        ),
        i32,
        (
            HashTableCrILst::FuncHashCref,
            HashTableCrILst::FuncCrefEqual,
            HashTableCrILst::FuncCrefStr,
            HashTableCrILst::FuncExpStr,
        ),
    );
    let mut vars: SimCodeVar::SimVars;
    match '__try0: {
        varInfo = modelInfo.varInfo.clone();
        vars = modelInfo.vars.clone();
        size = varInfo.numStateVars.clone()
            + varInfo.numAlgVars.clone()
            + varInfo.numIntAlgVars.clone()
            + varInfo.numBoolAlgVars.clone()
            + varInfo.numAlgAliasVars.clone()
            + varInfo.numIntAliasVars.clone()
            + varInfo.numBoolAliasVars.clone()
            + varInfo.numParams.clone()
            + varInfo.numIntParams.clone()
            + varInfo.numBoolParams.clone()
            + varInfo.numOutVars.clone()
            + varInfo.numInVars.clone()
            + varInfo.numOptimizeConstraints.clone()
            + varInfo.numOptimizeFinalConstraints.clone();
        size = intMax(size, 1023);
        outHT = HashTableCrefSimVar::emptyHashTableSized(size);
        arraySimVars = HashTableCrILst::emptyHashTableSized(size);
        outHT = unwrap_break_err!(List::fold(&vars.stateVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.derivativeVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.algVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        arraySimVars = unwrap_break_err!(List::fold(&vars.algVars, &move |__a0: metamodelica::Ref<SimCodeVar::SimVar>, __a1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>))| getArraySimVars(&__a0, __a1), arraySimVars.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.discreteAlgVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.intAlgVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.boolAlgVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.paramVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        arraySimVars = unwrap_break_err!(List::fold(&vars.paramVars, &move |__a0: metamodelica::Ref<SimCodeVar::SimVar>, __a1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>))| getArraySimVars(&__a0, __a1), arraySimVars.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.intParamVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.boolParamVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.aliasVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        arraySimVars = unwrap_break_err!(List::fold(&vars.aliasVars, &move |__a0: metamodelica::Ref<SimCodeVar::SimVar>, __a1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>))| getArraySimVars(&__a0, __a1), arraySimVars.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.intAliasVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.boolAliasVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.stringAlgVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.stringParamVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.stringAliasVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.extObjVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.constVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.intConstVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.boolConstVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.stringConstVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.sensitivityVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.jacobianVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.seedVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.realOptimizeConstraintsVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        outHT = unwrap_break_err!(List::fold(&vars.realOptimizeFinalConstraintsVars, &HashTableCrefSimVar::addSimVarToHashTable, outHT.clone()), '__try0);
        Ok::<_, &'static str>((
            arraySimVars.clone(),
            outHT.clone(),
            size.clone(),
            varInfo.clone(),
            vars.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
            arraySimVars = __try0_o0;
            outHT = __try0_o1;
            size = __try0_o2;
            varInfo = __try0_o3;
            vars = __try0_o4;
        }
        Err(__try0_err) => {
            Error::addInternalError(
                literal!("function createCrefToSimVarHT failed"),
                metamodelica::sourceInfo!("SimCode/SimCodeCodegenUtil.mo"),
            )?;
            return Err(__try0_err);
        }
    }
    Ok(outHT)
}

fn getArraySimVars(
    mut iSimVar: &metamodelica::Ref<SimCodeVar::SimVar>,
    mut iArrayMapping: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
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
        Arc<dyn ::std::ops::Fn(metamodelica::List<i32>) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut oArrayMapping: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
        ),
        i32,
        (
            HashTableCrILst::FuncHashCref,
            HashTableCrILst::FuncCrefEqual,
            HashTableCrILst::FuncCrefStr,
            HashTableCrILst::FuncExpStr,
        ),
    );
    let mut name: metamodelica::Ref<DAE::ComponentRef>;
    let mut arrayCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut tmpArrayMapping: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<i32>)>>,
        ),
        i32,
        (
            HashTableCrILst::FuncHashCref,
            HashTableCrILst::FuncCrefEqual,
            HashTableCrILst::FuncCrefStr,
            HashTableCrILst::FuncExpStr,
        ),
    ) = iArrayMapping.clone();
    let mut arrayVars: metamodelica::List<i32>;
    let mut index: i32;
    oArrayMapping = (::match_deref::match_deref! { match iSimVar {
        Deref @ SimCodeVar::SimVar { name: __esc_name, index: __esc_index, numArrayElement: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. } => {
            name = (*__esc_name).clone();
            index = (*__esc_index).clone();
            arrayCref = ComponentReferenceBasics::crefStripLastSubs(metamodelica::AsArg::as_arg(&name))?;
            if BaseHashTable::hasKey(arrayCref.clone(), &iArrayMapping)? {
                arrayVars = BaseHashTable::get(arrayCref.clone(), &iArrayMapping)?;
                tmpArrayMapping = BaseHashTable::add((arrayCref, metamodelica::cons(index.clone(), arrayVars)), tmpArrayMapping)?;
            } else {
                tmpArrayMapping = BaseHashTable::add((arrayCref, list![index.clone()]), tmpArrayMapping)?;
            }
            tmpArrayMapping
        },
        _ => iArrayMapping,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oArrayMapping)
}

pub(crate) fn functionInfo(mut r#fn: &metamodelica::Ref<SimCodeFunction::Function::Function>) -> Result<SourceInfo> {
    let mut info: SourceInfo;
    info = (match &**r#fn {
        SimCodeFunction::Function::FUNCTION { info: __esc_info, .. } => {
            info = (*__esc_info).clone();
            info.clone()
        }
        SimCodeFunction::Function::EXTERNAL_FUNCTION { info: __esc_info, .. } => {
            info = (*__esc_info).clone();
            info.clone()
        }
        SimCodeFunction::Function::RECORD_CONSTRUCTOR { info: __esc_info, .. } => {
            info = (*__esc_info).clone();
            info.clone()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(info)
}

pub fn eqInfo(mut eq: &metamodelica::Ref<SimCode::SimEqSystem>) -> Result<SourceInfo> {
    let mut info: SourceInfo;
    info = (::match_deref::match_deref! { match eq {
        Deref @ SimCode::SimEqSystem::SES_RESIDUAL { source: Deref @ DAE::ElementSource { info: __esc_info, .. }, .. } => {
            info = (*__esc_info).clone();
            info.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_FOR_RESIDUAL { source: Deref @ DAE::ElementSource { info: __esc_info, .. }, .. } => {
            info = (*__esc_info).clone();
            info.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_GENERIC_RESIDUAL { source: Deref @ DAE::ElementSource { info: __esc_info, .. }, .. } => {
            info = (*__esc_info).clone();
            info.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { source: Deref @ DAE::ElementSource { info: __esc_info, .. }, .. } => {
            info = (*__esc_info).clone();
            info.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { source: Deref @ DAE::ElementSource { info: __esc_info, .. }, .. } => {
            info = (*__esc_info).clone();
            info.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { source: Deref @ DAE::ElementSource { info: __esc_info, .. }, .. } => {
            info = (*__esc_info).clone();
            info.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_RESIZABLE_ASSIGN { source: Deref @ DAE::ElementSource { info: __esc_info, .. }, .. } => {
            info = (*__esc_info).clone();
            info.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_GENERIC_ASSIGN { source: Deref @ DAE::ElementSource { info: __esc_info, .. }, .. } => {
            info = (*__esc_info).clone();
            info.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_ENTWINED_ASSIGN { source: Deref @ DAE::ElementSource { info: __esc_info, .. }, .. } => {
            info = (*__esc_info).clone();
            info.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_WHEN { source: Deref @ DAE::ElementSource { info: __esc_info, .. }, .. } => {
            info = (*__esc_info).clone();
            info.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_FOR_LOOP { source: Deref @ DAE::ElementSource { info: __esc_info, .. }, .. } => {
            info = (*__esc_info).clone();
            info.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(info)
}

pub fn simEqSystemIndex(mut eq: &metamodelica::Ref<SimCode::SimEqSystem>) -> Result<i32> {
    let mut index: i32;
    index = (::match_deref::match_deref! { match eq {
        Deref @ SimCode::SimEqSystem::SES_RESIDUAL { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            index.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_FOR_RESIDUAL { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            index.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_GENERIC_RESIDUAL { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            index.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            index.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            index.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            index.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_RESIZABLE_ASSIGN { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            index.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_GENERIC_ASSIGN { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            index.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_ENTWINED_ASSIGN { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            index.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_IFEQUATION { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            index.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_ALGORITHM { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            index.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_INVERSE_ALGORITHM { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            index.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: Deref @ SimCode::LinearSystem { index: __esc_index, .. }, .. } => {
            index = (*__esc_index).clone();
            index.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: Deref @ SimCode::NonlinearSystem { index: __esc_index, .. }, .. } => {
            index = (*__esc_index).clone();
            index.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_MIXED { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            index.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_WHEN { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            index.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_FOR_LOOP { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            index.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_ALIAS { index: __esc_index, .. } => {
            index = (*__esc_index).clone();
            index.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("SimCodeUtil.simEqSystemIndex failed")])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(index)
}

pub fn countDynamicExternalFunctions<'__b>(
    mut inFncLst: &'__b metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>,
) -> i32 {
    '__tco: loop {
        ::match_deref::match_deref! { match inFncLst {
            Deref @ metamodelica::ListNode::Nil => {
                return 0
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { dynamicLoad: true, .. }, tail: rest } => {
                let mut i: i32;
                i = countDynamicExternalFunctions(rest);
                return intAdd(i, 1)
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                let mut i: i32;
                { inFncLst = rest; continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn getVarIndexListByMapping(
    mut iVarToArrayIndexMapping: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    (metamodelica::List<i32>, metamodelica::Array<i32>),
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
            Arc<dyn ::std::ops::Fn((metamodelica::List<i32>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iVarName: metamodelica::Ref<DAE::ComponentRef>,
    mut iColumnMajor: bool,
    mut iIndexForUndefinedReferences: ArcStr,
) -> Result<metamodelica::List<ArcStr>> {
    let mut oVarIndexList: metamodelica::List<ArcStr>;
    (oVarIndexList, _) = getVarIndexInfosByMapping(
        iVarToArrayIndexMapping,
        iVarName,
        iColumnMajor,
        iIndexForUndefinedReferences,
    )?;
    Ok(oVarIndexList)
}

pub fn getVarIndexHeadByMapping(
    mut iVarToArrayIndexMapping: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    (metamodelica::List<i32>, metamodelica::Array<i32>),
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
            Arc<dyn ::std::ops::Fn((metamodelica::List<i32>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iVarName: metamodelica::Ref<DAE::ComponentRef>,
    mut iColumnMajor: bool,
    mut iIndexForUndefinedReferences: ArcStr,
) -> Result<ArcStr> {
    let mut oVarIndex: ArcStr;
    let mut varIndexList: metamodelica::List<ArcStr>;
    (varIndexList, _) = getVarIndexInfosByMapping(
        iVarToArrayIndexMapping,
        iVarName,
        iColumnMajor,
        iIndexForUndefinedReferences,
    )?;
    oVarIndex = (varIndexList).head().cloned()?;
    Ok(oVarIndex)
}

pub(crate) fn getVarIndexByMapping(
    mut iVarToArrayIndexMapping: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    (metamodelica::List<i32>, metamodelica::Array<i32>),
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
            Arc<dyn ::std::ops::Fn((metamodelica::List<i32>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iVarName: metamodelica::Ref<DAE::ComponentRef>,
    mut iColumnMajor: bool,
    mut iIndexForUndefinedReferences: ArcStr,
) -> Result<ArcStr> {
    let mut oConcreteVarIndex: ArcStr;
    (_, oConcreteVarIndex) = getVarIndexInfosByMapping(
        iVarToArrayIndexMapping,
        iVarName,
        iColumnMajor,
        iIndexForUndefinedReferences,
    )?;
    Ok(oConcreteVarIndex)
}

pub fn providesDirectionalDerivative(mut inSimCode: &metamodelica::Ref<SimCode::SimCode>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match inSimCode {
        Deref @ SimCode::SimCode { modelStructure: Some(SimCode::FmiModelStructure { continuousPartialDerivatives: Some(_), .. }), .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn getVarIndexInfosByMapping(
    mut iVarToArrayIndexMapping: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    (metamodelica::List<i32>, metamodelica::Array<i32>),
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
            Arc<dyn ::std::ops::Fn((metamodelica::List<i32>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iVarName: metamodelica::Ref<DAE::ComponentRef>,
    mut iColumnMajor: bool,
    mut iIndexForUndefinedReferences: ArcStr,
) -> Result<(metamodelica::List<ArcStr>, ArcStr)> {
    let mut oVarIndexList: metamodelica::List<ArcStr>;
    let mut oConcreteVarIndex: ArcStr = literal!("");
    let mut varName: metamodelica::Ref<DAE::ComponentRef> = iVarName.clone();
    let mut arrayIdx: i32 = 0;
    let mut idx: i32;
    let mut arraySize: i32;
    let mut concreteVarIndex: i32;
    let mut varIndices: metamodelica::Array<i32>;
    let mut tmpVarIndexListNew: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut arraySubscripts: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let mut arrayDimensions: metamodelica::List<i32>;
    let mut arrayDimensionsReverse: metamodelica::List<i32> = metamodelica::nil();
    let mut toColumnMajor: bool;
    let mut isContiguous: bool;
    arraySubscripts = ComponentReference::crefLastSubs(&varName)?;
    varName = ComponentReferenceBasics::crefStripLastSubs(&varName)?;
    if BaseHashTable::hasKey(varName.clone(), iVarToArrayIndexMapping)? {
        (arrayDimensions, varIndices) = BaseHashTable::get(varName.clone(), iVarToArrayIndexMapping)?;
        isContiguous = metamodelica::arrayLength(varIndices.clone()) == 1;
        if isContiguous {
            arraySize = List::fold(&arrayDimensions, &fnptr!(intMul, i32, i32), 1)?;
        } else {
            arraySize = metamodelica::arrayLength(varIndices.clone());
        }
        concreteVarIndex = SimCodeUtilShared::getScalarElementIndex(&arraySubscripts, &arrayDimensions)?;
        toColumnMajor = iColumnMajor && ((arrayDimensions).len() as i32) > 1;
        if toColumnMajor {
            concreteVarIndex = convertIndexToColumnMajor(concreteVarIndex, &arrayDimensions)?;
            arrayDimensionsReverse = arrayDimensions.clone().reverse();
        }
        for mut arrayIdx in 0..=arraySize - 1 {
            idx = arraySize - arrayIdx;
            if toColumnMajor {
                idx = convertIndexToColumnMajor(idx, &arrayDimensionsReverse)?;
            }
            if isContiguous {
                idx = metamodelica::arrayGet(varIndices.clone(), 1)? + idx - 1;
            } else {
                idx = metamodelica::arrayGet(varIndices.clone(), idx)?;
            }
            if intLt(idx, 0) {
                tmpVarIndexListNew = metamodelica::cons(intString(intMul(idx, -1) - 1), tmpVarIndexListNew);
            } else {
                if intEq(idx, 0) {
                    tmpVarIndexListNew = metamodelica::cons(iIndexForUndefinedReferences.clone(), tmpVarIndexListNew);
                } else {
                    tmpVarIndexListNew = metamodelica::cons(intString(idx - 1), tmpVarIndexListNew);
                }
            }
        }
        if isVarIndexListConsecutive(iVarToArrayIndexMapping, iVarName)? && toColumnMajor {
            concreteVarIndex = convertIndexToColumnMajor(concreteVarIndex, &arrayDimensions)?;
        }
        oConcreteVarIndex = (tmpVarIndexListNew).get(concreteVarIndex)?;
    }
    if (tmpVarIndexListNew).is_empty() {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("GetVarIndexListByMapping: No Element for "));
                __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&varName)?);
                __mm_s.push_str(&*literal!(" found!"));
                ArcStr::from(__mm_s)
            }],
        )?;
        tmpVarIndexListNew = list![iIndexForUndefinedReferences.clone()];
        oConcreteVarIndex = iIndexForUndefinedReferences;
    }
    oVarIndexList = tmpVarIndexListNew;
    Ok((oVarIndexList, oConcreteVarIndex))
}

pub fn convertIndexToColumnMajor(mut idx: i32, mut arrayDimensions: &metamodelica::List<i32>) -> Result<i32> {
    let mut idxOut: i32;
    let mut idx0: i32;
    let mut ndim: i32;
    let mut length: i32;
    let mut idxi: i32;
    let mut fac: i32;
    ndim = ((arrayDimensions).len() as i32);
    length = List::fold(arrayDimensions, &fnptr!(intMul, i32, i32), 1)?;
    idx0 = idx - 1;
    idxOut = 1;
    fac = 1;
    for mut dimi in &**arrayDimensions {
        length = intDiv(length, dimi.clone());
        idxi = intDiv(idx0, length);
        idx0 = idx0 - idxi * length;
        idxOut = idxOut + idxi * fac;
        fac = fac * dimi.clone();
    }
    Ok(idxOut)
}

pub fn isVarIndexListConsecutive(
    mut iVarToArrayIndexMapping: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    (metamodelica::List<i32>, metamodelica::Array<i32>),
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
            Arc<dyn ::std::ops::Fn((metamodelica::List<i32>, metamodelica::Array<i32>)) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iVarName: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut oIsConsecutive: bool;
    let mut varName: metamodelica::Ref<DAE::ComponentRef> = iVarName;
    let mut arrayIdx: i32 = 0;
    let mut idx: i32;
    let mut arraySize: i32;
    let mut currentIndex: i32 = -1;
    let mut varIndices: metamodelica::Array<i32>;
    let mut consecutive: bool = true;
    varName = ComponentReferenceBasics::crefStripLastSubs(&varName)?;
    if BaseHashTable::hasKey(varName.clone(), iVarToArrayIndexMapping)? {
        (_, varIndices) = BaseHashTable::get(varName, iVarToArrayIndexMapping)?;
        arraySize = metamodelica::arrayLength(varIndices.clone());
        for mut arrayIdx in 0..=arraySize - 1 {
            idx = metamodelica::arrayGet(varIndices.clone(), arraySize - arrayIdx)?;
            if intLt(idx, 0) {
                if intEq(currentIndex, -1) {
                    currentIndex = intMul(idx, -1) - 1;
                } else {
                    consecutive = boolAnd(consecutive, intEq(currentIndex, intMul(idx, -1)));
                    currentIndex = intMul(idx, -1) - 1;
                }
            } else {
                if intEq(idx, 0) {
                    currentIndex = -2;
                    consecutive = false;
                } else {
                    if intEq(currentIndex, -1) {
                        currentIndex = idx - 1;
                    } else {
                        consecutive = boolAnd(consecutive, intEq(currentIndex, idx));
                        currentIndex = idx - 1;
                    }
                }
            }
        }
    }
    oIsConsecutive = consecutive;
    Ok(oIsConsecutive)
}

pub fn getEnumerationTypes(
    mut inVars: &SimCodeVar::SimVars,
) -> Result<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>> {
    let mut outVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    outVars = (match inVars.clone() {
        SimCodeVar::SimVars { .. } => {
            outVars = getEnumerationTypesHelper(&inVars.stateVars, metamodelica::nil())?;
            outVars = getEnumerationTypesHelper(&inVars.derivativeVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.algVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.discreteAlgVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.intAlgVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.boolAlgVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.inputVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.outputVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.aliasVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.intAliasVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.boolAliasVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.paramVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.intParamVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.boolParamVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.stringAlgVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.stringParamVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.stringAliasVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.extObjVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.constVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.intConstVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.boolConstVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.stringConstVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.sensitivityVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.jacobianVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.seedVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.realOptimizeConstraintsVars, outVars)?;
            outVars = getEnumerationTypesHelper(&inVars.realOptimizeFinalConstraintsVars, outVars)?;
            outVars.reverse()
        }
        _ => metamodelica::nil(),
    });
    Ok(outVars)
}

fn getEnumerationTypesHelper(
    mut inVars: &metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    mut inAccumVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
) -> Result<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>> {
    let mut outVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = inAccumVars;
    for mut var in &**inVars {
        let () = (match &*var.clone() {
            SimCodeVar::SimVar { .. } => {
                if Types::isEnumeration(&var.type_)
                    && !(List::exist1(
                        &outVars,
                        &move |__a0: metamodelica::Ref<SimCodeVar::SimVar>,
                               __a1: metamodelica::Ref<DAE::Type>|
                              -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(enumerationTypeExists(&__a0, &__a1))
                        },
                        var.type_.clone(),
                    )?)
                {
                    outVars = metamodelica::cons(var.clone(), outVars);
                }
                ()
            }
            _ => (),
        });
    }
    Ok(outVars)
}

fn enumerationTypeExists(
    mut var: &metamodelica::Ref<SimCodeVar::SimVar>,
    mut inType: &metamodelica::Ref<DAE::Type>,
) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match (var, inType) {
        (Deref @ SimCodeVar::SimVar { type_: ty @ Deref @ DAE::Type::T_ENUMERATION { .. }, .. }, Deref @ DAE::Type::T_ENUMERATION { .. }) => {
            AbsynUtil::pathEqual(var_field!((**ty).path, DAE::Type::T_ENUMERATION), var_field!((**inType).path, DAE::Type::T_ENUMERATION))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn getSimEqSysForIndex(
    mut idx: i32,
    mut allSimEqs: &metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
) -> Result<metamodelica::Ref<SimCode::SimEqSystem>> {
    let mut outSimEq: metamodelica::Ref<SimCode::SimEqSystem>;
    if let Ok(__iflet0) = List::getMemberOnTrue(idx, allSimEqs, &move |__a0: i32,
                                                                       __a1: metamodelica::Ref<
        SimCode::SimEqSystem,
    >| indexIsEqual(__a0, &__a1))
    {
        outSimEq = __iflet0;
    } else {
        metamodelica::print(literal!("getSimEqSysForIndex failed!\n"));
        return Err("fail");
    }
    Ok(outSimEq)
}

pub fn indexIsEqual(mut idx: i32, mut ses: &metamodelica::Ref<SimCode::SimEqSystem>) -> Result<bool> {
    let mut b: bool;
    let mut idx2: i32;
    idx2 = simEqSystemIndex(ses)?;
    b = intEq(idx, idx2);
    Ok(b)
}

pub fn getSimEqSystemCrefsLHS(
    mut simEqSys: &metamodelica::Ref<SimCode::SimEqSystem>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut crefsOut: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    crefsOut = (::match_deref::match_deref! { match simEqSys {
        Deref @ SimCode::SimEqSystem::SES_RESIDUAL { .. } => {
            metamodelica::print(literal!("implement SES_RESIDUAL in SimCodeUtil.getSimEqSystemCrefsLHS!\n"));
            metamodelica::nil()
        },
        Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { cref, .. } => {
            list![cref.clone()]
        },
        Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { cref, .. } => {
            list![cref.clone()]
        },
        Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { lhs, .. } => {
            list![Expression::expCref(lhs)?]
        },
        Deref @ SimCode::SimEqSystem::SES_IFEQUATION { .. } => {
            metamodelica::print(literal!("implement SES_IFEQUATION in SimCodeUtil.getSimEqSystemCrefsLHS!\n"));
            metamodelica::nil()
        },
        Deref @ SimCode::SimEqSystem::SES_ALGORITHM { .. } => {
            metamodelica::print(literal!("implement SES_ALGORITHM in SimCodeUtil.getSimEqSystemCrefsLHS!\n"));
            metamodelica::nil()
        },
        Deref @ SimCode::SimEqSystem::SES_INVERSE_ALGORITHM { .. } => {
            metamodelica::print(literal!("implement SES_INVERSE_ALGORITHM in SimCodeUtil.getSimEqSystemCrefsLHS!\n"));
            metamodelica::nil()
        },
        Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: Deref @ SimCode::LinearSystem { vars: simVars, residual, .. }, .. } => {
            let mut crefs2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            crefs2 = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
        for mut v in (simVars.clone()).into_iter().cloned() {
            let __x = v.name.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            listAppend(crefs2.clone(), crefs2)
        },
        Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: Deref @ SimCode::NonlinearSystem { crefs, .. }, .. } => {
            crefs.clone()
        },
        Deref @ SimCode::SimEqSystem::SES_MIXED { discVars: simVars, .. } => {
            ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
        for mut v in (simVars.clone()).into_iter().cloned() {
            let __x = v.name.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        Deref @ SimCode::SimEqSystem::SES_WHEN { whenStmtLst: Deref @ metamodelica::ListNode::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: lhs, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            crefs = Expression::getAllCrefs(lhs.clone())?;
            crefs
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(crefsOut)
}

pub fn getMaxSimEqSystemIndex(mut simCode: &metamodelica::Ref<SimCode::SimCode>) -> Result<i32> {
    let mut idxOut: i32 = 0;
    let mut allEquations: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut jacobianEquations: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut equationsForZeroCrossings: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut algorithmAndEquationAsserts: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut removedEquations: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut parameterEquations: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut maxValueEquations: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut minValueEquations: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut nominalValueEquations: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut startValueEquations: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut initialEquations: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut odeEquations: metamodelica::List<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>>;
    let mut algebraicEquations: metamodelica::List<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>>;
    let __arc13 = &(*simCode);
    let SimCode::SIMCODE {
        allEquations: __pa0,
        odeEquations: __pa1,
        algebraicEquations: __pa2,
        initialEquations: __pa3,
        startValueEquations: __pa4,
        nominalValueEquations: __pa5,
        minValueEquations: __pa6,
        maxValueEquations: __pa7,
        parameterEquations: __pa8,
        removedEquations: __pa9,
        algorithmAndEquationAsserts: __pa10,
        equationsForZeroCrossings: __pa11,
        jacobianEquations: __pa12,
        ..
    } = &**__arc13;
    allEquations = metamodelica::Own::own(__pa0);
    odeEquations = metamodelica::Own::own(__pa1);
    algebraicEquations = metamodelica::Own::own(__pa2);
    initialEquations = metamodelica::Own::own(__pa3);
    startValueEquations = metamodelica::Own::own(__pa4);
    nominalValueEquations = metamodelica::Own::own(__pa5);
    minValueEquations = metamodelica::Own::own(__pa6);
    maxValueEquations = metamodelica::Own::own(__pa7);
    parameterEquations = metamodelica::Own::own(__pa8);
    removedEquations = metamodelica::Own::own(__pa9);
    algorithmAndEquationAsserts = metamodelica::Own::own(__pa10);
    equationsForZeroCrossings = metamodelica::Own::own(__pa11);
    jacobianEquations = metamodelica::Own::own(__pa12);
    for mut eq in &*jacobianEquations {
        idxOut = intMax(idxOut, simEqSystemIndex(metamodelica::AsArg::as_arg(&eq))?);
    }
    for mut eq in &*equationsForZeroCrossings {
        idxOut = intMax(idxOut, simEqSystemIndex(metamodelica::AsArg::as_arg(&eq))?);
    }
    for mut eq in &*algorithmAndEquationAsserts {
        idxOut = intMax(idxOut, simEqSystemIndex(metamodelica::AsArg::as_arg(&eq))?);
    }
    for mut eq in &*removedEquations {
        idxOut = intMax(idxOut, simEqSystemIndex(metamodelica::AsArg::as_arg(&eq))?);
    }
    for mut eq in &*parameterEquations {
        idxOut = intMax(idxOut, simEqSystemIndex(metamodelica::AsArg::as_arg(&eq))?);
    }
    for mut eq in &*maxValueEquations {
        idxOut = intMax(idxOut, simEqSystemIndex(metamodelica::AsArg::as_arg(&eq))?);
    }
    for mut eq in &*minValueEquations {
        idxOut = intMax(idxOut, simEqSystemIndex(metamodelica::AsArg::as_arg(&eq))?);
    }
    for mut eq in &*nominalValueEquations {
        idxOut = intMax(idxOut, simEqSystemIndex(metamodelica::AsArg::as_arg(&eq))?);
    }
    for mut eq in &*nominalValueEquations {
        idxOut = intMax(idxOut, simEqSystemIndex(metamodelica::AsArg::as_arg(&eq))?);
    }
    for mut eq in &*startValueEquations {
        idxOut = intMax(idxOut, simEqSystemIndex(metamodelica::AsArg::as_arg(&eq))?);
    }
    for mut eq in &*initialEquations {
        idxOut = intMax(idxOut, simEqSystemIndex(metamodelica::AsArg::as_arg(&eq))?);
    }
    for mut eq in &*allEquations {
        idxOut = intMax(idxOut, simEqSystemIndex(metamodelica::AsArg::as_arg(&eq))?);
    }
    for mut eq in &*jacobianEquations {
        idxOut = intMax(idxOut, simEqSystemIndex(metamodelica::AsArg::as_arg(&eq))?);
    }
    for mut eq in &*jacobianEquations {
        idxOut = intMax(idxOut, simEqSystemIndex(metamodelica::AsArg::as_arg(&eq))?);
    }
    for mut eq in &*jacobianEquations {
        idxOut = intMax(idxOut, simEqSystemIndex(metamodelica::AsArg::as_arg(&eq))?);
    }
    Ok(idxOut)
}

pub(crate) fn getDaeEqsNotPartOfOdeSystem(
    mut iSimCode: &metamodelica::Ref<SimCode::SimCode>,
) -> Result<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>> {
    let mut oEqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut allEqs: metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>;
    let mut allEqIdxMapping: metamodelica::List<(i32, metamodelica::Ref<SimCode::SimEqSystem>)>;
    let mut allEquations: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut odeEquations: metamodelica::List<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>>;
    let mut highestIdx: i32;
    let mut tmpEqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let __arc2 = &(*iSimCode);
    let SimCode::SIMCODE {
        allEquations: __pa0,
        odeEquations: __pa1,
        ..
    } = &**__arc2;
    allEquations = metamodelica::Own::own(__pa0);
    odeEquations = metamodelica::Own::own(__pa1);
    (allEqIdxMapping, highestIdx) = List::fold(
        &allEquations,
        &move |__a0: metamodelica::Ref<SimCode::SimEqSystem>,
               __a1: (metamodelica::List<(i32, metamodelica::Ref<SimCode::SimEqSystem>)>, i32)| {
            getDaeEqsNotPartOfOdeSystem0(__a0, &__a1)
        },
        (metamodelica::nil(), 0),
    )?;
    allEqs = arrayCreate(highestIdx, None);
    allEqs = List::fold(
        &allEqIdxMapping,
        &move |__a0: (i32, metamodelica::Ref<SimCode::SimEqSystem>),
               __a1: metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>| {
            getDaeEqsNotPartOfOdeSystem1(&__a0, __a1)
        },
        allEqs.clone(),
    )?;
    allEqs = List::fold(
        &odeEquations,
        &move |__a0: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
               __a1: metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>| {
            getDaeEqsNotPartOfOdeSystem2(&__a0, __a1)
        },
        allEqs.clone(),
    )?;
    tmpEqs = metamodelica::nil();
    tmpEqs = Array::fold(
        allEqs.clone(),
        &fnptr!(
            getDaeEqsNotPartOfOdeSystem4,
            Option<metamodelica::Ref<SimCode::SimEqSystem>>,
            metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>
        ),
        tmpEqs,
    )?;
    oEqs = Dangerous::listReverseInPlace(tmpEqs);
    Ok(oEqs)
}

fn getDaeEqsNotPartOfOdeSystem0(
    mut iEqSystem: metamodelica::Ref<SimCode::SimEqSystem>,
    mut iMappingWithHighestIdx: &(metamodelica::List<(i32, metamodelica::Ref<SimCode::SimEqSystem>)>, i32),
) -> Result<(metamodelica::List<(i32, metamodelica::Ref<SimCode::SimEqSystem>)>, i32)> {
    let mut outMappingWithHighestIdx: (metamodelica::List<(i32, metamodelica::Ref<SimCode::SimEqSystem>)>, i32);
    let mut index: i32;
    let mut highestIdx: i32;
    let mut allEqIdxMapping: metamodelica::List<(i32, metamodelica::Ref<SimCode::SimEqSystem>)>;
    index = simEqSystemIndex(&iEqSystem)?;
    (allEqIdxMapping, highestIdx) = iMappingWithHighestIdx.clone();
    allEqIdxMapping = metamodelica::cons((index, iEqSystem), allEqIdxMapping);
    highestIdx = intMax(highestIdx, index);
    outMappingWithHighestIdx = (allEqIdxMapping, highestIdx);
    Ok(outMappingWithHighestIdx)
}

fn getDaeEqsNotPartOfOdeSystem1(
    mut iEqSystem: &(i32, metamodelica::Ref<SimCode::SimEqSystem>),
    mut iEqArray: metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>,
) -> Result<metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>> {
    let mut oEqArray: metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>;
    let mut eqSysIdx: i32;
    let mut eqSys: metamodelica::Ref<SimCode::SimEqSystem>;
    (eqSysIdx, eqSys) = iEqSystem.clone();
    oEqArray = metamodelica::arrayUpdate(iEqArray.clone(), eqSysIdx, Some(eqSys))?;
    Ok(oEqArray)
}

fn getDaeEqsNotPartOfOdeSystem2(
    mut iEqSystem: &metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
    mut iEqArray: metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>,
) -> Result<metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>> {
    let mut oEqArray: metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>;
    oEqArray = List::fold(
        iEqSystem,
        &move |__a0: metamodelica::Ref<SimCode::SimEqSystem>,
               __a1: metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>| {
            getDaeEqsNotPartOfOdeSystem3(&__a0, __a1)
        },
        iEqArray.clone(),
    )?;
    Ok(oEqArray)
}

fn getDaeEqsNotPartOfOdeSystem3(
    mut iEqSystem: &metamodelica::Ref<SimCode::SimEqSystem>,
    mut iEqArray: metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>,
) -> Result<metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>> {
    let mut oEqArray: metamodelica::Array<Option<metamodelica::Ref<SimCode::SimEqSystem>>>;
    let mut eqSysIdx: i32;
    eqSysIdx = simEqSystemIndex(iEqSystem)?;
    oEqArray = metamodelica::arrayUpdate(iEqArray.clone(), eqSysIdx, None)?;
    Ok(oEqArray)
}

fn getDaeEqsNotPartOfOdeSystem4(
    mut iEqSystemOpt: Option<metamodelica::Ref<SimCode::SimEqSystem>>,
    mut iResList: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
) -> metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>> {
    let mut oResList: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut eqSys: metamodelica::Ref<SimCode::SimEqSystem>;
    oResList = (::match_deref::match_deref! { match &(iEqSystemOpt) {
        Some(__esc_eqSys) => {
            eqSys = (*__esc_eqSys).clone();
            metamodelica::cons(eqSys.clone(), iResList)
        },
        _ => iResList,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oResList
}

pub fn getStateSimVarIndexFromIndex(
    mut inStateVars: &metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    mut inIndex: i32,
) -> Result<i32> {
    let mut outVariableIndex: i32;
    let mut stateVar: metamodelica::Ref<SimCodeVar::SimVar>;
    stateVar = (inStateVars.clone()).get(
        inIndex + 1
            - if (metamodelica::stringEq(&(Config::simCodeTarget()?), &(literal!("Cpp")))) {
                0
            } else {
                ((inStateVars).len() as i32)
            },
    )?;
    outVariableIndex = getVariableIndex(&stateVar);
    Ok(outVariableIndex)
}

pub fn getNumScalars(mut vars: &metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>) -> Result<i32> {
    let mut numScalars: i32;
    numScalars = List::applyAndFold(
        vars,
        &fnptr!(intAdd, i32, i32),
        &move |__a0: metamodelica::Ref<SimCodeVar::SimVar>| SimCodeUtilShared::getNumElems(&__a0),
        0,
    )?;
    Ok(numScalars)
}

pub fn numScalarElems(mut vars: &metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>) -> Result<i32> {
    let mut n: i32;
    n = getNumScalars(vars)?;
    Ok(n)
}

pub fn numScalarElemsBefore(
    mut vars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    mut n: i32,
) -> Result<i32> {
    let mut numScalars: i32 = 0;
    for mut v in &*List::firstN(vars, n)? {
        numScalars = numScalars + SimCodeUtilShared::getNumElems(metamodelica::AsArg::as_arg(&v))?;
    }
    Ok(numScalars)
}

pub fn numScalarElemsVar(mut var: &metamodelica::Ref<SimCodeVar::SimVar>) -> Result<i32> {
    let mut n: i32 = SimCodeUtilShared::getNumElems(var)?;
    Ok(n)
}

pub fn arrayElementSubscripts(mut var: &metamodelica::Ref<SimCodeVar::SimVar>) -> Result<metamodelica::List<ArcStr>> {
    let mut subscripts: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut dims: metamodelica::List<i32>;
    let mut acc: metamodelica::List<metamodelica::List<ArcStr>> = list![metamodelica::nil()];
    subscripts = (::match_deref::match_deref! { match var {
        Deref @ SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_ARRAY { .. }, .. } => {
            dims = ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut d in (var.numArrayElement.clone()).into_iter().cloned() {
            let __x = stringInt(d.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            for mut d in &*dims.reverse() {
                acc = List::flatten(({
        let mut __acc: metamodelica::List<metamodelica::List<metamodelica::List<ArcStr>>> = metamodelica::nil();
        for mut i in (1..=d.clone()).into_iter() {
            let __x = ({
        let mut __acc: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut rest in (acc.clone()).into_iter().cloned() {
            let __x = metamodelica::cons(intString(i.clone()), rest.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
            }
            ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut sub in (acc).into_iter().cloned() {
            let __x = stringDelimitList(sub.clone(), literal!(","));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(subscripts)
}

pub fn getFMI3ArrayStart(mut var: &metamodelica::Ref<SimCodeVar::SimVar>) -> Result<ArcStr> {
    let mut out: ArcStr = literal!("");
    let mut svals: metamodelica::List<ArcStr>;
    let mut n: i32;
    out = (::match_deref::match_deref! { match &(var.initialValue.clone()) {
        Some(e) => {
            svals = getFMIArrayStartValues(e.clone())?;
            n = SimCodeUtilShared::getNumElems(var)?;
            if ((svals).is_empty()) {literal!("")} else {if (intEq(((svals).len() as i32), 1)) {stringDelimitList(List::fill((svals).head().cloned()?, n), literal!(" "))} else {stringDelimitList(svals, literal!(" "))}}
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out)
}

pub fn getFMIArrayStartValues(mut e: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::List<ArcStr>> {
    '__tco: loop {
        match &*e {
            DAE::Exp::RCONST { real: r } => return Ok(list![realString(r.clone())]),
            DAE::Exp::ICONST { integer: i } => return Ok(list![intString(i.clone())]),
            DAE::Exp::BCONST { bool: b } => {
                return Ok(list![if (b.clone()) {
                    literal!("true")
                } else {
                    literal!("false")
                }]);
            }
            DAE::Exp::SCONST { string: s } => return Ok(list![s.clone()]),
            DAE::Exp::ARRAY { array: arr, .. } => {
                return Ok(List::flatten(
                    ({
                        let mut __acc: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
                        for mut el in (arr.clone()).into_iter().cloned() {
                            let __x = getFMIArrayStartValues(el.clone())?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                )?);
            }
            DAE::Exp::REDUCTION { expr: first, .. } => {
                e = first.clone();
                continue '__tco;
            }
            _ => return Ok(metamodelica::nil()),
        }
    }
}

pub fn getFMIScalarVRs(
    mut var: &metamodelica::Ref<SimCodeVar::SimVar>,
    mut simCode: &metamodelica::Ref<SimCode::SimCode>,
) -> Result<ArcStr> {
    let mut out: ArcStr;
    let mut base: i32;
    let mut n: i32;
    let mut refs: metamodelica::List<ArcStr> = metamodelica::nil();
    base = lookupVR(var.name.clone(), simCode)?;
    n = SimCodeUtilShared::getNumElems(var)?;
    for mut i in 0..=n - 1 {
        refs = metamodelica::cons(ArcStr::from(::std::format!("{}", base + i)), refs);
    }
    out = stringDelimitList(refs.reverse(), literal!(", "));
    Ok(out)
}

pub fn getScalarElements(
    mut var: metamodelica::Ref<SimCodeVar::SimVar>,
) -> Result<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>> {
    let mut elts: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut dims: metamodelica::List<i32>;
    let mut elt: metamodelica::Ref<SimCodeVar::SimVar>;
    let mut index: i32;
    let mut fmi_index: i32;
    elts = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_ARRAY { .. }, exportVar: None, .. } => metamodelica::nil(),
        Deref @ SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_ARRAY { .. }, variable_index: Some(__esc_index), fmi_index: Some(__esc_fmi_index), .. } => {
            index = (*__esc_index).clone();
            fmi_index = (*__esc_fmi_index).clone();
            dims = List::map(List::lastN(var.numArrayElement.clone(), ((var.numArrayElement).len() as i32))?, &stringInt)?;
            elt = var.clone();
            assign_field!(elt.type_ = Types::arrayElementType(&var.type_));
            elts = fillScalarElements(elt, &dims, 1, metamodelica::nil(), metamodelica::nil())?;
            (elts, _, _) = setVariableIndexHelper(&elts, index.clone(), fmi_index.clone())?;
            elts
        },
        _ => list![var],
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(elts)
}

fn fillScalarElements(
    mut eltIn: metamodelica::Ref<SimCodeVar::SimVar>,
    mut dims: &metamodelica::List<i32>,
    mut dimIdx: i32,
    mut subsIn: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut elts: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
) -> Result<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>> {
    let mut elts: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = elts;
    let mut elt: metamodelica::Ref<SimCodeVar::SimVar> = eltIn.clone();
    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    for mut i in ({
        let __s = (dims).get(dimIdx)?;
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        subs = metamodelica::cons(
            metamodelica::Ref::new(DAE::Subscript::INDEX {
                exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i }),
            }),
            subsIn.clone(),
        );
        if dimIdx < ((dims).len() as i32) {
            elts = fillScalarElements(eltIn.clone(), dims, dimIdx + 1, subs, elts)?;
        } else {
            subs = subs.reverse();
            assign_field!(
                elt.name = ComponentReference::crefSetLastSubs(&elt.name, &subs)?,
                elt.exportVar = Some(ComponentReference::crefSetLastSubs(
                    &(elt.exportVar.clone().ok_or("pattern mismatch")?),
                    &subs
                )?)
            );
            let () = (match &*elt {
                SimCodeVar::SimVar {
                    varKind:
                        BackendDAE::VarKind::CLOCKED_STATE {
                            previousName: cref,
                            isStartFixed: fixed,
                        },
                    ..
                } => {
                    assign_field!(
                        elt.varKind = BackendDAE::VarKind::CLOCKED_STATE {
                            previousName: ComponentReference::crefSetLastSubs(
                                metamodelica::AsArg::as_arg(&cref),
                                &subs
                            )?,
                            isStartFixed: fixed.clone()
                        }
                    );
                    ()
                }
                _ => (),
            });
            elts = metamodelica::cons(elt.clone(), elts);
        }
    }
    Ok(elts)
}

pub fn getVariableIndex(mut inVar: &metamodelica::Ref<SimCodeVar::SimVar>) -> i32 {
    let mut outVariableIndex: i32;
    outVariableIndex = (match &**inVar {
        SimCodeVar::SimVar {
            variable_index: Some(variableIndex),
            ..
        } => variableIndex.clone(),
        _ => 0,
    });
    outVariableIndex
}

pub fn getVariableFMIIndex(mut inVar: &metamodelica::Ref<SimCodeVar::SimVar>) -> i32 {
    let mut outVariableIndex: i32;
    outVariableIndex = (match &**inVar {
        SimCodeVar::SimVar {
            fmi_index: Some(variableIndex),
            ..
        } => variableIndex.clone(),
        _ => 0,
    });
    outVariableIndex
}

pub fn getValueReference(
    mut inSimVar: metamodelica::Ref<SimCodeVar::SimVar>,
    mut inSimCode: &metamodelica::Ref<SimCode::SimCode>,
    mut inElimNegAliases: bool,
) -> Result<ArcStr> {
    let mut outValueReference: ArcStr;
    outValueReference = (::match_deref::match_deref! { match &((inSimVar.clone(), inElimNegAliases, Config::simCodeTarget()?)) {
        (Deref @ SimCodeVar::SimVar { aliasvar: SimCodeVar::AliasVariable::NEGATEDALIAS { varName: _ }, .. }, false, _) => {
            getDefaultValueReference(&inSimVar, inSimCode.modelInfo.varInfo.clone())?
        },
        (_, _, _) if (stringEqual(&(Config::simCodeTarget()?), &(literal!("Cpp"))) || stringEqual(&(Config::simCodeTarget()?), &(literal!("omsic")))) => {
            let mut simVar: metamodelica::Ref<SimCodeVar::SimVar>;
            let mut cref: metamodelica::Ref<DAE::ComponentRef>;
            let mut valueReference: ArcStr;
            simVar = (::match_deref::match_deref! { match &(inSimVar.clone()) {
        Deref @ SimCodeVar::SimVar { aliasvar: SimCodeVar::AliasVariable::ALIAS { varName: __esc_cref }, .. } => {
            cref = (*__esc_cref).clone();
            cref2simvar(cref.clone(), inSimCode)?
        },
        Deref @ SimCodeVar::SimVar { aliasvar: SimCodeVar::AliasVariable::NEGATEDALIAS { varName: __esc_cref }, .. } => {
            cref = (*__esc_cref).clone();
            cref2simvar(cref.clone(), inSimCode)?
        },
        Deref @ SimCodeVar::SimVar { name: Deref @ DAE::ComponentRef::CREF_QUAL { ident: Deref @ "$PRE", componentRef, .. }, .. } => {
            cref2simvar(componentRef.clone(), inSimCode)?
        },
        _ => {
            inSimVar
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            valueReference = getVarIndexByMapping(&(inSimCode.varToArrayIndexMapping.clone()), simVar.name.clone(), true, literal!("-1"))?;
            if stringEqual(&valueReference, &(literal!("-1"))) {
                Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("invalid return value from getVarIndexByMapping for ")); __mm_s.push_str(&*simVarString(&simVar)?); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("SimCode/SimCodeCodegenUtil.mo"))?;
            }
            valueReference
        },
        (Deref @ SimCodeVar::SimVar { aliasvar: SimCodeVar::AliasVariable::ALIAS { varName: cref }, .. }, _, _) => {
            getDefaultValueReference(&(cref2simvar(cref.clone(), inSimCode)?), inSimCode.modelInfo.varInfo.clone())?
        },
        _ => {
            getDefaultValueReference(&inSimVar, inSimCode.modelInfo.varInfo.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outValueReference)
}

fn getDefaultValueReference(
    mut inSimVar: &metamodelica::Ref<SimCodeVar::SimVar>,
    mut inVarInfo: SimCode::VarInfo,
) -> Result<ArcStr> {
    let mut outDefaultValueReference: ArcStr;
    let mut reference: i32;
    let mut numReal: i32 = 2 * inVarInfo.numStateVars.clone()
        + inVarInfo.numAlgVars.clone()
        + inVarInfo.numDiscreteReal.clone()
        + inVarInfo.numParams.clone()
        + inVarInfo.numAlgAliasVars.clone();
    let mut numInteger: i32 =
        inVarInfo.numIntAlgVars.clone() + inVarInfo.numIntParams.clone() + inVarInfo.numIntAliasVars.clone();
    let mut numBoolean: i32 =
        inVarInfo.numBoolAlgVars.clone() + inVarInfo.numBoolParams.clone() + inVarInfo.numBoolAliasVars.clone();
    reference = getVariableIndex(inSimVar);
    if reference > numReal + numInteger + numBoolean {
        reference = reference - numReal - numInteger - numBoolean;
    } else if reference > numReal + numInteger {
        reference = reference - numReal - numInteger;
    } else if reference > numReal {
        reference = reference - numReal;
    } else if reference < 0 {
        Error::addInternalError(
            literal!("invalid return value from getVariableIndex"),
            metamodelica::sourceInfo!("SimCode/SimCodeCodegenUtil.mo"),
        )?;
    }
    outDefaultValueReference = ArcStr::from(::std::format!("{}", reference - 1));
    Ok(outDefaultValueReference)
}

pub(crate) fn getFMI3TypeOffset<'__b>(
    mut inType: &'__b metamodelica::Ref<DAE::Type>,
    mut inModelInfo: &'__b SimCode::ModelInfo,
) -> i32 {
    '__tco: loop {
        let mut vi: SimCode::VarInfo = inModelInfo.varInfo.clone();
        let mut numReal: i32 = 2 * vi.numStateVars.clone()
            + vi.numAlgVars.clone()
            + vi.numDiscreteReal.clone()
            + vi.numParams.clone()
            + vi.numAlgAliasVars.clone();
        let mut numInteger: i32 = vi.numIntAlgVars.clone() + vi.numIntParams.clone() + vi.numIntAliasVars.clone();
        let mut numBoolean: i32 = vi.numBoolAlgVars.clone() + vi.numBoolParams.clone() + vi.numBoolAliasVars.clone();
        let mut numString: i32 =
            vi.numStringAlgVars.clone() + vi.numStringParamVars.clone() + vi.numStringAliasVars.clone();
        match &**inType {
            DAE::Type::T_REAL { .. } => return 0,
            DAE::Type::T_INTEGER { .. } => return numReal,
            DAE::Type::T_ENUMERATION { .. } => return numReal,
            DAE::Type::T_BOOL { .. } => return numReal + numInteger,
            DAE::Type::T_STRING { .. } => return numReal + numInteger + numBoolean,
            DAE::Type::T_COMPLEX {
                complexClassType: ClassInf::State::EXTERNAL_OBJ { .. },
                ..
            } => return numReal + numInteger + numBoolean + numString,
            DAE::Type::T_ARRAY { ty: aty, .. } => {
                (inType, inModelInfo) = (aty, inModelInfo);
                continue '__tco;
            }
            _ => return 0,
        }
    }
}

pub fn getFMI2ValueReferenceOffsets(mut modelInfo: &SimCode::ModelInfo) -> metamodelica::List<i32> {
    let mut offsets: metamodelica::List<i32>;
    offsets = list![
        getFMI3TypeOffset(&(DAE::T_REAL_DEFAULT().clone()), modelInfo),
        getFMI3TypeOffset(&(DAE::T_INTEGER_DEFAULT().clone()), modelInfo),
        getFMI3TypeOffset(&(DAE::T_BOOL_DEFAULT().clone()), modelInfo),
        getFMI3TypeOffset(&(DAE::T_STRING_DEFAULT().clone()), modelInfo)
    ];
    offsets
}

pub fn getFMI3ValueReference(
    mut inSimVar: &metamodelica::Ref<SimCodeVar::SimVar>,
    mut inSimCode: &metamodelica::Ref<SimCode::SimCode>,
) -> Result<ArcStr> {
    let mut outValueReference: ArcStr;
    let mut offset: i32;
    let mut localRef: i32;
    offset = getFMI3TypeOffset(&inSimVar.type_, &inSimCode.modelInfo);
    localRef = lookupVR(inSimVar.name.clone(), inSimCode)?;
    outValueReference = ArcStr::from(::std::format!("{}", offset + localRef));
    Ok(outValueReference)
}

fn fmi3ModelVariableLists(
    mut vars: &SimCodeVar::SimVars,
) -> metamodelica::List<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>> {
    let mut allLists: metamodelica::List<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>;
    allLists = list![
        vars.stateVars.clone(),
        vars.derivativeVars.clone(),
        vars.algVars.clone(),
        vars.discreteAlgVars.clone(),
        vars.intAlgVars.clone(),
        vars.boolAlgVars.clone(),
        vars.stringAlgVars.clone(),
        vars.inputVars.clone(),
        vars.outputVars.clone(),
        vars.paramVars.clone(),
        vars.intParamVars.clone(),
        vars.boolParamVars.clone(),
        vars.stringParamVars.clone(),
        vars.aliasVars.clone(),
        vars.intAliasVars.clone(),
        vars.boolAliasVars.clone(),
        vars.stringAliasVars.clone()
    ];
    allLists
}

pub fn cacheFMI3ValueReferences(mut simCode: &metamodelica::Ref<SimCode::SimCode>) -> Result<ArcStr> {
    let mut dummy: ArcStr = literal!("");
    let mut allLists: metamodelica::List<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>> =
        fmi3ModelVariableLists(&simCode.modelInfo.vars);
    let mut table: metamodelica::Array<ArcStr>;
    let mut n: i32 = 0;
    let mut i: i32;
    for mut lst in &*allLists {
        for mut v in &*lst.clone() {
            n = intMax(n, getVariableFMIIndex(metamodelica::AsArg::as_arg(&v)));
        }
    }
    table = arrayCreate(n, literal!(""));
    for mut lst in &*allLists {
        for mut v in &*lst.clone() {
            i = getVariableFMIIndex(metamodelica::AsArg::as_arg(&v));
            if i > 0 && i <= n && stringEmpty(&(metamodelica::arrayGet(table.clone(), i)?)) {
                metamodelica::arrayUpdate(
                    table.clone(),
                    i,
                    getFMI3ValueReference(metamodelica::AsArg::as_arg(&v), simCode)?,
                )?;
            }
        }
    }
    {
        let __v = Some(table.clone());
        openmodelica_util::Globals::fmi3ValueReferenceCache.with(|__root| *__root.borrow_mut() = __v)
    };
    Ok(dummy)
}

pub fn clearFMI3ValueReferences() -> ArcStr {
    let mut dummy: ArcStr = literal!("");
    {
        let __v = None;
        openmodelica_util::Globals::fmi3ValueReferenceCache.with(|__root| *__root.borrow_mut() = __v)
    };
    dummy
}

pub fn fmi3UnknownDependencyAttributes(
    mut simCode: &metamodelica::Ref<SimCode::SimCode>,
    mut unknown: &SimCode::FmiUnknown,
) -> Result<ArcStr> {
    let mut attributes: ArcStr = literal!("");
    let mut cache: Option<metamodelica::Array<ArcStr>> =
        openmodelica_util::Globals::fmi3ValueReferenceCache.with(|__root| __root.borrow().clone());
    let mut vrs: metamodelica::List<ArcStr> = metamodelica::nil();
    if !((unknown.dependencies).is_empty()) {
        for mut d in &*unknown.dependencies.clone() {
            vrs = (match cache.clone() {
                Some(mut table)
                    if (d.clone() > 0
                        && d.clone() <= metamodelica::arrayLength(table.clone())
                        && !(stringEmpty(&(metamodelica::arrayGet(table.clone(), d.clone())?)))) =>
                {
                    metamodelica::cons(metamodelica::arrayGet(table.clone(), d.clone())?, vrs)
                }
                _ => metamodelica::cons(getFMI3ValueReferenceFromFMIIndex(simCode, d.clone())?, vrs),
            });
        }
        attributes = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(" dependencies=\""));
            __mm_s.push_str(&*stringDelimitList(vrs.reverse(), literal!(" ")));
            __mm_s.push_str(&*literal!("\""));
            ArcStr::from(__mm_s)
        };
    }
    if !((unknown.dependenciesKind).is_empty()) {
        attributes = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*attributes);
            __mm_s.push_str(&*literal!(" dependenciesKind=\""));
            __mm_s.push_str(&*stringDelimitList(unknown.dependenciesKind.clone(), literal!(" ")));
            __mm_s.push_str(&*literal!("\""));
            ArcStr::from(__mm_s)
        };
    }
    Ok(attributes)
}

pub fn fmiDependenciesString(mut dependencies: metamodelica::List<i32>) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(
        ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut d in (dependencies).into_iter().cloned() {
                let __x = intString(d.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        literal!(" "),
    );
    r#str
}

pub fn fmiDependenciesKindString(mut kinds: metamodelica::List<ArcStr>) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = stringDelimitList(kinds, literal!(" "));
    r#str
}

pub fn getFMI3ValueReferenceFromFMIIndex(
    mut inSimCode: &metamodelica::Ref<SimCode::SimCode>,
    mut inFMIIndex: i32,
) -> Result<ArcStr> {
    let mut outValueReference: ArcStr;
    let mut vars: SimCodeVar::SimVars = inSimCode.modelInfo.vars.clone();
    let mut cache: Option<metamodelica::Array<ArcStr>>;
    let mut table: metamodelica::Array<ArcStr>;
    let mut found: Option<metamodelica::Ref<SimCodeVar::SimVar>> = None;
    cache = openmodelica_util::Globals::fmi3ValueReferenceCache.with(|__root| __root.borrow().clone());
    if (cache).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(cache) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        table = metamodelica::Own::own(__pa0);
        if inFMIIndex > 0
            && inFMIIndex <= metamodelica::arrayLength(table.clone())
            && !(stringEmpty(&(metamodelica::arrayGet(table.clone(), inFMIIndex)?)))
        {
            outValueReference = metamodelica::arrayGet(table.clone(), inFMIIndex)?;
            return Ok(outValueReference);
        }
    }
    for mut lst in &*fmi3ModelVariableLists(&vars) {
        for mut v in &*lst.clone() {
            if intEq(getVariableFMIIndex(metamodelica::AsArg::as_arg(&v)), inFMIIndex) {
                found = Some(v.clone());
                break;
            }
        }
        if (found).is_some() {
            break;
        }
    }
    outValueReference = (::match_deref::match_deref! { match &(found.clone()) {
        Some(_) => getFMI3ValueReference(&(found.ok_or("pattern mismatch")?), inSimCode)?,
        _ => ArcStr::from(::std::format!("{}", inFMIIndex)),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outValueReference)
}

pub fn getFMI3TimeValueReference(mut inSimCode: &metamodelica::Ref<SimCode::SimCode>) -> Result<ArcStr> {
    let mut outValueReference: ArcStr;
    outValueReference = ArcStr::from(::std::format!(
        "{}",
        getFMI3ClockVROffset(&inSimCode.modelInfo)? + ((inSimCode.clockedPartitions).len() as i32)
    ));
    Ok(outValueReference)
}

pub const FMI_LS_DAE_VERSION: &'static str = "1.0.0-alpha.1";

pub fn fmiLsDaeVersion() -> ArcStr {
    let mut version: ArcStr = arcstr::literal!(FMI_LS_DAE_VERSION);
    version
}

pub fn getFMI3DaeModeValueReference(mut simCode: &metamodelica::Ref<SimCode::SimCode>) -> Result<ArcStr> {
    let mut vr: ArcStr;
    vr = ArcStr::from(::std::format!(
        "{}",
        stringInt(getFMI3TimeValueReference(simCode)?)? + simCode.modelInfo.varInfo.numZeroCrossings.clone() + 1
    ));
    Ok(vr)
}

pub fn fmi3DaeResiduals(
    mut simCode: &metamodelica::Ref<SimCode::SimCode>,
) -> Result<metamodelica::List<(ArcStr, ArcStr)>> {
    let mut residuals: metamodelica::List<(ArcStr, ArcStr)> = metamodelica::nil();
    let mut dmd: SimCode::DaeModeData;
    let mut rows: Option<metamodelica::List<metamodelica::List<i32>>>;
    let mut rest: metamodelica::List<metamodelica::List<i32>>;
    let mut cols: metamodelica::List<i32>;
    let mut stateVRs: metamodelica::Array<ArcStr>;
    let mut algebraicVRs: metamodelica::Array<ArcStr>;
    let mut numStates: i32;
    let mut daeModeVR: i32;
    let mut vr: ArcStr;
    let mut attributes: ArcStr;
    let mut acc: metamodelica::List<ArcStr>;
    let __pa0 = ::match_deref::match_deref! { match &(simCode.daeModeData.clone()) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    dmd = metamodelica::Own::own(__pa0);
    daeModeVR = stringInt(getFMI3DaeModeValueReference(simCode)?)?;
    rows = (::match_deref::match_deref! { match &(dmd.sparsityPattern.clone()) {
        Some(jm) => {
            Some(({
        let mut __acc: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
        for mut e in (jm.sparsityT.clone()).into_iter().cloned() {
            let __x = Util::tuple22(e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    numStates = numScalarElems(&simCode.modelInfo.vars.stateVars)?;
    stateVRs = metamodelica::arrayFromVec(
        ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut v in (simCode.modelInfo.vars.stateVars.clone()).into_iter().cloned() {
                let __x = getFMI3ValueReference(&(v.clone()), simCode)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
        .into_iter()
        .cloned()
        .collect(),
    );
    algebraicVRs = metamodelica::arrayFromVec(
        ({
            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
            for mut v in (dmd.algebraicVars.clone()).into_iter().cloned() {
                let __x = getFMI3ValueReference(&(v.clone()), simCode)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
        .into_iter()
        .cloned()
        .collect(),
    );
    for mut var in &*dmd.residualVars.clone() {
        attributes = literal!("");
        if (rows).is_some() {
            let __pa1 = ::match_deref::match_deref! { match &(rows.clone()) {
                Some(__pa1) => __pa1.clone(),
                _ => return Err("pattern mismatch"),
            } };
            rest = metamodelica::Own::own(__pa1);
            if (rest).is_empty() {
                cols = metamodelica::nil();
            } else {
                cols = (rest).head().cloned()?;
                rows = Some((rest).rest()?);
            }
            acc = metamodelica::nil();
            for mut c in &*cols {
                if c.clone() < numStates {
                    vr = metamodelica::arrayGet(stateVRs.clone(), c.clone() + 1)?;
                    acc = metamodelica::cons(
                        ArcStr::from(::std::format!("{}", stringInt(vr.clone())? + numStates)),
                        metamodelica::cons(vr, acc),
                    );
                } else {
                    acc = metamodelica::cons(
                        metamodelica::arrayGet(algebraicVRs.clone(), c.clone() - numStates + 1)?,
                        acc,
                    );
                }
            }
            acc = acc.reverse();
            attributes = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(" dependencies=\""));
                __mm_s.push_str(&*stringDelimitList(acc.clone(), literal!(" ")));
                __mm_s.push_str(&*literal!("\" dependenciesKind=\""));
                __mm_s.push_str(&*stringDelimitList(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut s in (acc).into_iter().cloned() {
                            let __x = literal!("dependent");
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    literal!(" "),
                ));
                __mm_s.push_str(&*literal!("\""));
                ArcStr::from(__mm_s)
            };
        }
        residuals = metamodelica::cons(
            (
                ArcStr::from(::std::format!("{}", daeModeVR + 1 + var.index.clone())),
                attributes,
            ),
            residuals,
        );
    }
    residuals = residuals.reverse();
    Ok(residuals)
}

pub(crate) fn getLocalValueReference(
    mut inSimVar: metamodelica::Ref<SimCodeVar::SimVar>,
    mut inSimCode: &metamodelica::Ref<SimCode::SimCode>,
    mut inCrefToSimVarHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<SimCodeVar::SimVar>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<SimCodeVar::SimVar>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inElimNegAliases: bool,
) -> Result<ArcStr> {
    let mut outValueReference: ArcStr;
    outValueReference = 'mc: {
        let __mc_input = (&*inSimVar, inCrefToSimVarHT);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SimCodeVar::SimVar { name: cref, .. }, crefToSimVarHT) => {
                    let mut valueReference: ArcStr;
                    valueReference = localCref2Index(cref.clone(), crefToSimVarHT.clone());
                    if stringEqual(&valueReference, &(literal!("-1"))) {
                        valueReference = getValueReference(inSimVar.clone(), inSimCode, inElimNegAliases)?;
                    }
                    Ok(valueReference.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("getLocalValueReference failed."), metamodelica::sourceInfo!("SimCode/SimCodeCodegenUtil.mo"))?;
                    Ok(literal!("ERROR: getLocalValueReference failed"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outValueReference)
}

fn getNLSysRHS(
    mut eqs: &metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
    mut res: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut unknowns: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    unknowns = 'mc: {
        let __mc_input = (&**eqs, &**res);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::SimEqSystem::SES_RESIDUAL { exp, .. }, tail: tail }, _) => {
                    Ok(getNLSysRHS(metamodelica::AsArg::as_arg(&tail), &(listAppend(res.clone(), Expression::getAllCrefs(exp.clone())?)))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::SimEqSystem::SES_FOR_RESIDUAL { exp, .. }, tail: tail }, _) => {
                    Ok(getNLSysRHS(metamodelica::AsArg::as_arg(&tail), &(listAppend(res.clone(), Expression::getAllCrefs(exp.clone())?)))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCode::SimEqSystem::SES_GENERIC_RESIDUAL { exp, .. }, tail: tail }, _) => {
                    Ok(getNLSysRHS(metamodelica::AsArg::as_arg(&tail), &(listAppend(res.clone(), Expression::getAllCrefs(exp.clone())?)))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    metamodelica::print(literal!("getNLSysRHS failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(unknowns)
}

fn computeDependenciesHelper(
    mut eqs: &metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
    mut unknowns: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut res: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
) -> Result<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>> {
    let mut deps: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    deps = 'mc: {
        let __mc_input = (&**eqs, res);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, r) => {
                    Ok(r.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: head @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { cref, exp, .. }, tail: tail }, r) => {
                    let mut new_unknowns: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let true = (List::isMemberOnTrue(cref.clone(), unknowns, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?) else { return Err("pattern mismatch") };
                    new_unknowns = Expression::getAllCrefs(exp.clone())?;
                    Ok(computeDependenciesHelper(metamodelica::AsArg::as_arg(&tail), &(listAppend(unknowns.clone(), new_unknowns.clone())), listAppend(r.clone(), list![head.clone()]))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: head @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { cref, exp, .. }, tail: tail }, r) => {
                    let mut new_unknowns: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let true = (List::isMemberOnTrue(cref.clone(), unknowns, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?) else { return Err("pattern mismatch") };
                    new_unknowns = Expression::getAllCrefs(exp.clone())?;
                    Ok(computeDependenciesHelper(metamodelica::AsArg::as_arg(&tail), &(listAppend(unknowns.clone(), new_unknowns.clone())), listAppend(r.clone(), list![head.clone()]))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: head @ Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: Deref @ SimCode::LinearSystem { beqs, .. }, .. }, tail: tail }, r) => {
                    let mut new_unknowns: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut linsys_unk: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    linsys_unk = getSimEqSystemCrefsLHS(metamodelica::AsArg::as_arg(&head))?;
                    let false = (((List::intersectionOnTrue(&linsys_unk, unknowns, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?)).is_empty()) else { return Err("pattern mismatch") };
                    new_unknowns = List::flatten(List::map(beqs.clone(), &Expression::getAllCrefs)?)?;
                    Ok(computeDependenciesHelper(metamodelica::AsArg::as_arg(&tail), &(listAppend(unknowns.clone(), new_unknowns.clone())), listAppend(r.clone(), list![head.clone()]))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: head @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: Deref @ SimCode::NonlinearSystem { crefs: nlsys_unk, eqs: nlsys_eqs, .. }, .. }, tail: tail }, r) => {
                    let mut new_unknowns: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let false = (((List::intersectionOnTrue(metamodelica::AsArg::as_arg(&nlsys_unk), unknowns, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?)).is_empty()) else { return Err("pattern mismatch") };
                    new_unknowns = getNLSysRHS(metamodelica::AsArg::as_arg(&nlsys_eqs), &(metamodelica::nil()))?;
                    Ok(computeDependenciesHelper(metamodelica::AsArg::as_arg(&tail), &(listAppend(unknowns.clone(), new_unknowns.clone())), listAppend(r.clone(), list![head.clone()]))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: tail }, r) => {
                    Ok(computeDependenciesHelper(metamodelica::AsArg::as_arg(&tail), unknowns, r.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(deps)
}

pub fn computeDependencies(
    mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
    mut cref: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>> {
    let mut deps: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    deps = (match &*cref {
        _ => computeDependenciesHelper(&(eqs.reverse()), &(list![cref]), metamodelica::nil())?.reverse(),
    });
    Ok(deps)
}

pub fn getSimEqSystemsByIndexLst(
    mut idcs: metamodelica::List<i32>,
    mut allSes: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
) -> Result<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>> {
    let mut sesOut: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    sesOut = List::map1(
        idcs,
        &move |__a0: i32, __a1: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>| {
            getSimEqSysForIndex(__a0, &__a1)
        },
        allSes,
    )?;
    Ok(sesOut)
}

pub fn getInputIndex(mut var: &metamodelica::Ref<SimCodeVar::SimVar>) -> Result<i32> {
    let mut inputIndex: i32;
    let mut v: metamodelica::Array<i32>;
    inputIndex = (match &**var {
        SimCodeVar::SimVar {
            inputIndex: Some(v), ..
        } if (metamodelica::arrayLength(v.clone()) == 1) => metamodelica::arrayGet(v.clone(), 1)?,
        SimCodeVar::SimVar {
            inputIndex: Some(_), ..
        } => {
            Error::addInternalError(
                literal!("Failed to SimCodeUtil.getInputIndex of variable"),
                metamodelica::sourceInfo!("SimCode/SimCodeCodegenUtil.mo"),
            )?;
            return Err("fail");
        }
        _ => -1,
    });
    Ok(inputIndex)
}

pub fn resetFunctionIndex() -> Result<()> {
    {
        let __v = DoubleEnded::fromList(&(metamodelica::nil()))?;
        openmodelica_util::Globals::codegenFunctionList.with(|__root| *__root.borrow_mut() = __v)
    };
    Ok(())
}

pub fn addFunctionIndex(mut prefix: &ArcStr, mut suffix: &ArcStr) -> Result<ArcStr> {
    let mut newName: ArcStr;
    let mut delst: DoubleEnded::MutableList<ArcStr>;
    delst = openmodelica_util::Globals::codegenFunctionList.with(|__root| __root.borrow().clone());
    newName = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*prefix);
        __mm_s.push_str(&*ArcStr::from(::std::format!("{}", DoubleEnded::length(delst.clone()))));
        __mm_s.push_str(&*suffix);
        ArcStr::from(__mm_s)
    };
    DoubleEnded::push_back(delst, newName.clone())?;
    Ok(newName)
}

pub fn nVariablesReal(mut varInfo: SimCode::VarInfo) -> i32 {
    let mut n: i32;
    n = 2 * varInfo.numStateVars.clone()
        + varInfo.numAlgVars.clone()
        + varInfo.numDiscreteReal.clone()
        + varInfo.numOptimizeConstraints.clone()
        + varInfo.numOptimizeFinalConstraints.clone();
    n
}

pub fn getSimCode() -> Result<metamodelica::Ref<SimCode::SimCode>> {
    let mut code: metamodelica::Ref<SimCode::SimCode>;
    let mut ocode: Option<metamodelica::Ref<SimCode::SimCode>>;
    ocode = crate::Globals::optionSimCode.with(|__root| __root.borrow().clone());
    code = (::match_deref::match_deref! { match &(ocode) {
        Some(c) => {
            c.clone()
        },
        _ => {
            Error::addInternalError(literal!("Tried to generate code that requires the SimCode structure, but this is not set (function context?)"), metamodelica::sourceInfo!("SimCode/SimCodeCodegenUtil.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(code)
}

pub fn isSimulationCodegen() -> bool {
    let mut simulation: bool;
    let mut ocode: Option<metamodelica::Ref<SimCode::SimCode>>;
    ocode = crate::Globals::optionSimCode.with(|__root| __root.borrow().clone());
    simulation = (::match_deref::match_deref! { match &(ocode) {
        Some(_) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    simulation
}

pub fn isContiguousArrayCref(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut context: &SimCodeFunction::Context,
) -> Result<bool> {
    let mut outContiguous: bool = true;
    let mut simCode: metamodelica::Ref<SimCode::SimCode> = getSimCode()?;
    let mut v: metamodelica::Ref<SimCodeVar::SimVar>;
    let mut next: i32 = -1;
    let mut param: bool;
    let mut firstParam: bool = false;
    let mut jacVar: bool;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    if !(simCode.scalarized.clone()) {
        return Ok(outContiguous);
    }
    crefs = ComponentReference::expandCref(inCref, true)?;
    (jacVar, outContiguous) = (::match_deref::match_deref! { match &((context, &*crefs)) {
        (SimCodeFunction::Context::JACOBIAN_CONTEXT { jacHT: Some(jacHT), .. }, Deref @ metamodelica::ListNode::Cons { head: cr, tail: _ }) if (isJacobianColumnCref(metamodelica::AsArg::as_arg(&cr)) || List::any(&crefs, &({ let __pe_b1 = jacHT.clone(); move |__pe_a0| BaseHashTable::hasKey(__pe_a0, &__pe_b1) }))?) => {
            (true, BaseHashTable::hasKey(ComponentReference::crefStripSubs(inCref)?, &(jacHT.clone()))?)
        },
        _ => {
            (false, true)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if jacVar {
        return Ok(outContiguous);
    }
    for mut cr in &*crefs {
        v = cref2simvar(cr.clone(), &simCode)?;
        if v.index.clone() < 0 {
            return Ok(outContiguous);
        }
        param = (match v.varKind.clone() {
            BackendDAE::VarKind::PARAM { .. } => true,
            _ => false,
        });
        if next == -1 {
            firstParam = param;
        }
        outContiguous = (match v.aliasvar.clone() {
            SimCodeVar::AliasVariable::NOALIAS { .. } => param == firstParam && (next == -1 || v.index.clone() == next),
            _ => false,
        });
        if !(outContiguous) {
            return Ok(outContiguous);
        }
        next = v.index.clone() + 1;
    }
    Ok(outContiguous)
}

pub fn contiguousSliceStart(
    mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut dims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Subscript>>> {
    let mut start: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    (start, _) = contiguousSlice(subs, dims)?;
    Ok(start)
}

pub fn contiguousSliceDims(
    mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut dims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<metamodelica::List<i32>> {
    let mut sliceDims: metamodelica::List<i32>;
    (_, sliceDims) = contiguousSlice(subs, dims)?;
    Ok(sliceDims)
}

fn contiguousSlice(
    mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut dims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    metamodelica::List<i32>,
)> {
    let mut start: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = metamodelica::nil();
    let mut sliceDims: metamodelica::List<i32> = metamodelica::nil();
    let mut i: i32;
    let mut j: i32;
    let mut d: i32;
    let mut step: Option<metamodelica::Ref<DAE::Exp>>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut sub: metamodelica::Ref<DAE::Subscript>;
    let mut rest: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = subs;
    let mut inBlock: bool = false;
    let mut sliced: bool = false;
    let mut ok: bool;
    for mut dim in &**dims {
        d = (match &*dim.clone() {
            DAE::Dimension::DIM_INTEGER { integer: __dim_integer } => __dim_integer.clone(),
            _ => -1,
        });
        if (rest).is_empty() {
            sub = openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM();
        } else {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            sub = metamodelica::Own::own(__pa0);
            rest = metamodelica::Own::own(__pa1);
        }
        ok = if (d < 1) {
            false
        } else {
            (::match_deref::match_deref! { match &(sub.clone()) {
                Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i } } if (!(inBlock) && i.clone() >= 1 && i.clone() <= d) => {
                    start = metamodelica::cons(sub, start);
                    true
                },
                Deref @ DAE::Subscript::INDEX { exp: e } if (!(inBlock) && !(Expression::isConst(e.clone())?) && Types::isInteger(&(Expression::r#typeof(e.clone())?))) => {
                    start = metamodelica::cons(sub, start);
                    true
                },
                Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::ICONST { integer: i }, step, stop: Deref @ DAE::Exp::ICONST { integer: j }, .. } } if (!(inBlock) && i.clone() >= 1 && j.clone() >= i.clone() && j.clone() <= d && Util::applyOptionOrDefault(step.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Expression::isConstOne(&__a0)) }, true)?) => {
                    inBlock = true;
                    sliced = true;
                    start = metamodelica::cons(metamodelica::Ref::new(DAE::Subscript::INDEX { exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() }) }), start);
                    sliceDims = metamodelica::cons(j.clone() - i.clone() + 1, sliceDims);
                    true
                },
                Deref @ DAE::Subscript::WHOLEDIM { .. } => {
                    inBlock = true;
                    start = metamodelica::cons(metamodelica::Ref::new(DAE::Subscript::INDEX { exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }) }), start);
                    sliceDims = metamodelica::cons(d, sliceDims);
                    true
                },
                _ => false,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        };
        if !(ok) {
            start = metamodelica::nil();
            sliceDims = metamodelica::nil();
            return Ok((start, sliceDims));
        }
        sliced = sliced || !(inBlock);
    }
    if !((rest).is_empty()) || !(sliced) || (sliceDims).is_empty() {
        start = metamodelica::nil();
        sliceDims = metamodelica::nil();
    } else {
        start = start.reverse();
        sliceDims = sliceDims.reverse();
    }
    Ok((start, sliceDims))
}

pub fn stackArrayLength(
    mut var: &metamodelica::Ref<SimCodeFunction::Variable::Variable>,
    mut r#fn: &metamodelica::Ref<SimCodeFunction::Function::Function>,
) -> Result<i32> {
    let mut n: i32 = 0;
    let mut outVars: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>> = (match &**r#fn {
        SimCodeFunction::Function::FUNCTION { .. } => {
            var_field!((**r#fn).outVars, SimCodeFunction::Function::Function::FUNCTION).clone()
        }
        _ => metamodelica::nil(),
    });
    let _ = (match &**var {
        SimCodeFunction::Variable::VARIABLE {
            parallelism: DAE::VarParallelism::NON_PARALLEL { .. },
            bind_from_outside: false,
            instDims: __var_instDims,
            name: __var_name,
            ty: __var_ty,
            value: __var_value,
            ..
        } => {
            if (__var_instDims).is_empty() {
                return Ok(n);
            }
            let _ = (::match_deref::match_deref! { match &(__var_value.clone()) {
                Some(Deref @ DAE::Exp::SHARED_LITERAL { .. }) => {
                    return Ok(n);
                    ()
                },
                _ => (),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            let _ = (match &*(Types::arrayElementType(metamodelica::AsArg::as_arg(&__var_ty))) {
                DAE::Type::T_REAL { .. } => (),
                DAE::Type::T_INTEGER { .. } => (),
                DAE::Type::T_BOOL { .. } => (),
                _ => {
                    return Ok(n);
                    ()
                }
            });
            for mut v in &*outVars {
                let _ = (match &*v.clone() {
                    SimCodeFunction::Variable::VARIABLE { name: __v_name, .. }
                        if (ComponentReferenceBasics::crefEqual(
                            metamodelica::AsArg::as_arg(&__v_name),
                            metamodelica::AsArg::as_arg(&__var_name),
                        )?) =>
                    {
                        return Ok(n);
                        ()
                    }
                    _ => (),
                });
            }
            n = 1;
            for mut d in &*__var_instDims.clone() {
                n = (match &*d.clone() {
                    DAE::Dimension::DIM_INTEGER { integer: __d_integer } if (__d_integer.clone() > 0) => {
                        n * __d_integer.clone()
                    }
                    _ => 0,
                });
            }
            if n > 256 {
                n = 0;
            }
            ()
        }
        _ => (),
    });
    Ok(n)
}

pub fn isJacobianColumnCref<'__b>(mut cr: &'__b metamodelica::Ref<DAE::ComponentRef>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match cr {
            Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: last, .. }, .. } => {
                return StringUtil::startsWith(id.clone(), arcstr::literal!(DAE::partialDerivativeNamePrefix)) && StringUtil::startsWith(last.clone(), literal!("dummyVar"))
            },
            Deref @ DAE::ComponentRef::CREF_QUAL { .. } => {
                { cr = var_field!((**cr).componentRef, DAE::ComponentRef::CREF_QUAL); continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn cref2simvar(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut simCode: &metamodelica::Ref<SimCode::SimCode>,
) -> Result<metamodelica::Ref<SimCodeVar::SimVar>> {
    let mut outSimVar: metamodelica::Ref<SimCodeVar::SimVar>;
    let mut crefToSimVarHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<SimCodeVar::SimVar>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCrefSimVar::FuncHashCref,
            HashTableCrefSimVar::FuncCrefEqual,
            HashTableCrefSimVar::FuncCrefStr,
            HashTableCrefSimVar::FuncExpStr,
        ),
    );
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut badcref: metamodelica::Ref<DAE::ComponentRef>;
    match '__try0: {
        let __arc2 = &(*simCode);
        let SimCode::SIMCODE {
            crefToSimVarHT: __pa1, ..
        } = &**__arc2;
        crefToSimVarHT = metamodelica::Own::own(__pa1);
        cref = if (simCode.scalarized.clone()) {
            inCref.clone()
        } else {
            unwrap_break_err!(ComponentReference::crefStripSubs(&inCref), '__try0)
        };
        outSimVar = unwrap_break_err!(simVarFromHT(cref.clone(), &crefToSimVarHT), '__try0);
        Ok::<_, &'static str>((outSimVar.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outSimVar = __try0_o0;
        }
        Err(_) => {
            badcref = ComponentReferenceBasics::makeCrefIdent(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("ERROR_cref2simvar_failed "));
                    __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&inCref)?);
                    ArcStr::from(__mm_s)
                },
                DAE::T_REAL_DEFAULT().clone(),
                metamodelica::nil(),
            );
            outSimVar = metamodelica::Ref::new(SimCodeVar::SimVar {
                name: badcref.clone(),
                varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE,
                comment: literal!(""),
                unit: literal!(""),
                displayUnit: literal!(""),
                index: -2,
                minValue: None,
                maxValue: None,
                initialValue: None,
                nominalValue: None,
                isFixed: false,
                type_: DAE::T_REAL_DEFAULT().clone(),
                isDiscrete: false,
                arrayCref: None,
                aliasvar: openmodelica_simcode_types::SimCodeVar::AliasVariable::NOALIAS,
                source: DAE::emptyElementSource().clone(),
                causality: Some(openmodelica_simcode_types::SimCodeVar::Causality::LOCAL),
                variable_index: None,
                fmi_index: None,
                numArrayElement: metamodelica::nil(),
                isValueChangeable: false,
                isProtected: true,
                hideResult: None,
                isEncrypted: false,
                inputIndex: None,
                initNonlinear: false,
                matrixName: None,
                variability: None,
                initial_: None,
                exportVar: Some(badcref.clone()),
                relativeQuantity: false,
                isConnectorFlow: false,
            });
        }
    }
    Ok(outSimVar)
}

pub fn simVarExactFromHT(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut crefToSimVarHT: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<SimCodeVar::SimVar>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<SimCodeVar::SimVar>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<Option<metamodelica::Ref<SimCodeVar::SimVar>>> {
    let mut outSimVar: Option<metamodelica::Ref<SimCodeVar::SimVar>>;
    outSimVar = if (BaseHashTable::hasKey(inCref.clone(), crefToSimVarHT)?) {
        Some(BaseHashTable::get(inCref, crefToSimVarHT)?)
    } else {
        None
    };
    Ok(outSimVar)
}

pub fn simVarFromHT(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut crefToSimVarHT: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<SimCodeVar::SimVar>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<SimCodeVar::SimVar>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<metamodelica::Ref<SimCodeVar::SimVar>> {
    let mut outSimVar: metamodelica::Ref<SimCodeVar::SimVar>;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut badcref: metamodelica::Ref<DAE::ComponentRef>;
    let mut sv: metamodelica::Ref<SimCodeVar::SimVar>;
    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    match '__try0: {
        if unwrap_break_err!(BaseHashTable::hasKey(inCref.clone(), crefToSimVarHT), '__try0) {
            sv = unwrap_break_err!(BaseHashTable::get(inCref.clone(), crefToSimVarHT), '__try0);
        } else {
            if unwrap_break_err!(Flags::isSet(Flags::NF_SCALARIZE.clone()), '__try0) {
                sv = unwrap_break_err!(BaseHashTable::get(unwrap_break_err!(ComponentReferenceBasics::crefStripLastSubs(&inCref), '__try0), crefToSimVarHT), '__try0);
                subs = unwrap_break_err!(ComponentReference::crefLastSubs(&inCref), '__try0);
                assign_field!(
                    sv.name = unwrap_break_err!(ComponentReference::crefSetLastSubs(&sv.name, &subs), '__try0)
                );
            } else {
                sv = unwrap_break_err!(BaseHashTable::get(unwrap_break_err!(ComponentReference::crefStripSubs(&inCref), '__try0), crefToSimVarHT), '__try0);
                subs = unwrap_break_err!(ComponentReferenceBasics::crefSubs(&inCref), '__try0);
                assign_field!(
                    sv.name = unwrap_break_err!(ComponentReference::crefApplySubs(&(unwrap_break_err!(ComponentReference::crefStripSubs(&sv.name), '__try0)), &subs), '__try0)
                );
            }
            assign_field!(
                sv.variable_index = (match sv.variable_index.clone() {
                    Some(mut index) => {
                        Some(
                            index
                                + unwrap_break_err!(SimCodeUtilShared::getScalarElementIndex(&subs, &(unwrap_break_err!(List::map(sv.numArrayElement.clone(), &stringInt), '__try0))), '__try0)
                                - 1,
                        )
                    }
                    _ => {
                        sv.variable_index.clone()
                    }
                })
            );
            assign_field!(
                sv.fmi_index = (match sv.fmi_index.clone() {
                    Some(mut fmiIndex) => {
                        Some(
                            fmiIndex
                                + unwrap_break_err!(SimCodeUtilShared::getScalarElementIndex(&subs, &(unwrap_break_err!(List::map(sv.numArrayElement.clone(), &stringInt), '__try0))), '__try0)
                                - 1,
                        )
                    }
                    _ => {
                        sv.fmi_index.clone()
                    }
                })
            );
        }
        sv = (match sv.aliasvar.clone() {
            SimCodeVar::AliasVariable::NOALIAS { .. } => sv.clone(),
            SimCodeVar::AliasVariable::ALIAS {
                varName: ref __esc_cref,
            } => {
                cref = __esc_cref.clone();
                unwrap_break_err!(simVarFromHT(cref.clone(), crefToSimVarHT), '__try0)
            }
            SimCodeVar::AliasVariable::NEGATEDALIAS { .. } => sv.clone(),
        });
        Ok::<_, &'static str>((sv.clone(),))
    } {
        Ok((__try0_o0,)) => {
            sv = __try0_o0;
        }
        Err(_) => {
            badcref = ComponentReferenceBasics::makeCrefIdent(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("ERROR_simVarFromHT_failed "));
                    __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&inCref)?);
                    ArcStr::from(__mm_s)
                },
                DAE::T_REAL_DEFAULT().clone(),
                metamodelica::nil(),
            );
            sv = metamodelica::Ref::new(SimCodeVar::SimVar {
                name: badcref.clone(),
                varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE,
                comment: literal!(""),
                unit: literal!(""),
                displayUnit: literal!(""),
                index: -2,
                minValue: None,
                maxValue: None,
                initialValue: None,
                nominalValue: None,
                isFixed: false,
                type_: DAE::T_REAL_DEFAULT().clone(),
                isDiscrete: false,
                arrayCref: None,
                aliasvar: openmodelica_simcode_types::SimCodeVar::AliasVariable::NOALIAS,
                source: DAE::emptyElementSource().clone(),
                causality: Some(openmodelica_simcode_types::SimCodeVar::Causality::LOCAL),
                variable_index: None,
                fmi_index: None,
                numArrayElement: metamodelica::nil(),
                isValueChangeable: false,
                isProtected: true,
                hideResult: None,
                isEncrypted: false,
                inputIndex: None,
                initNonlinear: false,
                matrixName: None,
                variability: None,
                initial_: None,
                exportVar: Some(badcref.clone()),
                relativeQuantity: false,
                isConnectorFlow: false,
            });
        }
    }
    outSimVar = sv;
    Ok(outSimVar)
}

pub fn createJacContext(
    mut name: ArcStr,
    mut jacHT: Option<(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<SimCodeVar::SimVar>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<SimCodeVar::SimVar>) -> Result<ArcStr> + 'static>,
        ),
    )>,
) -> SimCodeFunction::Context {
    let mut outContext: SimCodeFunction::Context;
    outContext = SimCodeFunction::Context::JACOBIAN_CONTEXT {
        name: name,
        jacHT: jacHT,
    };
    outContext
}

pub fn localCref2SimVar(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inCrefToSimVarHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<SimCodeVar::SimVar>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<SimCodeVar::SimVar>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<metamodelica::Ref<SimCodeVar::SimVar>> {
    let mut outSimVar: metamodelica::Ref<SimCodeVar::SimVar>;
    outSimVar = 'mc: {
        let __mc_input = (inCref.clone(), inCrefToSimVarHT);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cref, crefToSimVarHT) => {
                    let mut sv: metamodelica::Ref<SimCodeVar::SimVar>;
                    sv = BaseHashTable::get(cref.clone(), &(crefToSimVarHT.clone()))?;
                    Ok(sv.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut badcref: metamodelica::Ref<DAE::ComponentRef>;
                    badcref = ComponentReferenceBasics::makeCrefIdent({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ERROR_localCref2SimVar_failed ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&inCref)?); ArcStr::from(__mm_s) }, DAE::T_REAL_DEFAULT().clone(), metamodelica::nil());
                    Ok(metamodelica::Ref::new(SimCodeVar::SimVar { name: badcref.clone(), varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE, comment: literal!(""), unit: literal!(""), displayUnit: literal!(""), index: -2, minValue: None, maxValue: None, initialValue: None, nominalValue: None, isFixed: false, type_: DAE::T_REAL_DEFAULT().clone(), isDiscrete: false, arrayCref: None, aliasvar: openmodelica_simcode_types::SimCodeVar::AliasVariable::NOALIAS, source: DAE::emptyElementSource().clone(), causality: Some(openmodelica_simcode_types::SimCodeVar::Causality::LOCAL), variable_index: None, fmi_index: None, numArrayElement: metamodelica::nil(), isValueChangeable: false, isProtected: true, hideResult: None, isEncrypted: false, inputIndex: None, initNonlinear: false, matrixName: None, variability: None, initial_: None, exportVar: Some(badcref.clone()), relativeQuantity: false, isConnectorFlow: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outSimVar)
}

pub(crate) fn localCref2Index(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inCrefToSimVarHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<SimCodeVar::SimVar>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<SimCodeVar::SimVar>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> ArcStr {
    let mut outIndex: ArcStr;
    outIndex = 'mc: {
        let __mc_input = (inCref, inCrefToSimVarHT);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cref, crefToSimVarHT) => {
                    let mut sv: metamodelica::Ref<SimCodeVar::SimVar>;
                    sv = BaseHashTable::get(cref.clone(), &(crefToSimVarHT.clone()))?;
                    Ok(ArcStr::from(::std::format!("{}", sv.index.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("-1"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outIndex
}

pub fn codegenExpSanityCheck(
    mut e: metamodelica::Ref<DAE::Exp>,
    mut context: &SimCodeFunction::Context,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut e: metamodelica::Ref<DAE::Exp> = e;
    e = unboxFunctionReferenceCall(e)?;
    if SimCodeFunctionUtil::inFunctionContext(context) {
        return Ok(e);
    }
    e = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, componentRef: __e_componentRef } => {
            let mut vars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
            let mut simCode: metamodelica::Ref<SimCode::SimCode>;
            let mut index: i32;
            let mut crf_lst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            simCode = getSimCode()?;
            crf_lst = ComponentReference::expandCref(metamodelica::AsArg::as_arg(&__e_componentRef), true)?;
            vars = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
        for mut cr in (crf_lst).into_iter().cloned() {
            let __x = cref2simvar(cr.clone(), &simCode)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            if !((vars).is_empty()) {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(vars) {
                    Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCodeVar::SimVar { index: __pa0, .. }, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                index = metamodelica::Own::own(__pa0);
                vars = metamodelica::Own::own(__pa1);
                for mut v in &*vars {
                    if v.index.clone() != index + 1 {
                        (e, _) = Expression::expandCrefs(e, false, 0)?;
                        break;
                    }
                    index = v.index.clone();
                }
            }
            e
        },
        _ => {
            e
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(e)
}

pub fn unboxFunctionReferenceCall(mut exp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut attr: metamodelica::Ref<DAE::CallAttributes>;
    if Config::acceptMetaModelicaGrammar()? {
        return Ok(exp);
    }
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::UNBOX { exp: __esc_e @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { isFunctionPointerCall: true, .. }, .. }, .. } => {
            e = (*__esc_e).clone();
            unboxFunctionReferenceCall(e.clone())?
        },
        Deref @ DAE::Exp::UNBOX { exp: __esc_e @ Deref @ DAE::Exp::TSUB { exp: Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { isFunctionPointerCall: true, .. }, .. }, .. }, .. } => {
            e = (*__esc_e).clone();
            unboxFunctionReferenceCall(e.clone())?
        },
        Deref @ DAE::Exp::TSUB { exp: __esc_e @ Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { isFunctionPointerCall: true, .. }, .. }, .. } => {
            e = (*__esc_e).clone();
            assign_variant_field!(exp => DAE::Exp::TSUB;
                exp = unboxFunctionReferenceCall(e.clone())?,
                ty = Types::unboxedType(var_field!((*exp).ty, DAE::Exp::TSUB).clone())?
            );
            exp
        },
        Deref @ DAE::Exp::CALL { attr: __esc_attr @ Deref @ DAE::CallAttributes { isFunctionPointerCall: true, .. }, expLst: __exp_expLst, .. } => {
            attr = (*__esc_attr).clone();
            assign_variant_field!(exp => DAE::Exp::CALL; expLst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut a in (__exp_expLst.clone()).into_iter().cloned() {
            let __x = unboxArgument(a.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            assign_field!(attr.ty = unboxResultType(attr.ty.clone())?);
            assign_variant_field!(exp => DAE::Exp::CALL; attr = attr.clone());
            exp
        },
        Deref @ DAE::Exp::PARTEVALFUNCTION { expList: __exp_expList, .. } => {
            assign_variant_field!(exp => DAE::Exp::PARTEVALFUNCTION;
                expList = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut a in (__exp_expList.clone()).into_iter().cloned() {
            let __x = unboxArgument(a.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
                ty = unboxFunctionReferenceType(var_field!((*exp).ty, DAE::Exp::PARTEVALFUNCTION).clone())?,
                origType = unboxFunctionReferenceType(var_field!((*exp).origType, DAE::Exp::PARTEVALFUNCTION).clone())?
            );
            exp
        },
        _ => exp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn unboxArgument(mut exp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    '__tco: loop {
        let mut e: metamodelica::Ref<DAE::Exp>;
        let mut args: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
        ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ DAE::Exp::BOX { exp: __exp_exp } => return Ok(unboxFunctionReferenceCall(__exp_exp.clone())?),
            Deref @ DAE::Exp::METARECORDCALL { index: (-1), args: __exp_args, fieldNames: __exp_fieldNames, path: __exp_path, .. } => {
                args = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
            for mut a in (__exp_args.clone()).into_iter().cloned() {
                let __x = unboxArgument(a.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                return Ok(metamodelica::Ref::new(DAE::Exp::RECORD { path: __exp_path.clone(), exps: args.clone(), comp: __exp_fieldNames.clone(), ty: metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: __exp_path.clone() }, varLst: ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
            let __thr_src0 = args;
            let mut __thr_it0 = (&__thr_src0).into_iter();
            let __thr_src1 = __exp_fieldNames.clone();
            let mut __thr_it1 = (&__thr_src1).into_iter();
            loop {
                match (__thr_it0.next(), __thr_it1.next()) {
                    (Some(a), Some(n)) => {
                        let __x = metamodelica::Ref::new(DAE::Var { name: n.clone(), attributes: DAE::dummyAttrVar().clone(), ty: Expression::r#typeof(a.clone())?, binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None });
                        __acc = cons(__x, __acc);
                    }
                    (None, None) => break,
                    _ => return Err("threaded for: ranges of unequal length"),
                }
            }
            __acc.reverse()
        }), equalityConstraint: None, usedExternally: false }) }))
            },
            Deref @ DAE::Exp::SHARED_LITERAL { exp: __esc_e @ Deref @ DAE::Exp::BOX { .. }, .. } => {
                e = (*__esc_e).clone();
                { exp = e.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::SHARED_LITERAL { exp: __esc_e @ Deref @ DAE::Exp::METARECORDCALL { index: (-1), .. }, .. } => {
                e = (*__esc_e).clone();
                { exp = e.clone(); continue '__tco; }
            },
            _ => return Ok(unboxFunctionReferenceCall(exp)?),
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn unboxResultType(mut ty: metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut ty: metamodelica::Ref<DAE::Type> = ty;
    ty = (match &*ty {
        DAE::Type::T_TUPLE { types: __ty_types, .. } => {
            assign_variant_field!(ty => DAE::Type::T_TUPLE; types = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
                for mut t in (__ty_types.clone()).into_iter().cloned() {
                    let __x = Types::unboxedType(t.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ty
        }
        _ => Types::unboxedType(ty)?,
    });
    Ok(ty)
}

fn unboxFunctionReferenceType(mut ty: metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut ty: metamodelica::Ref<DAE::Type> = ty;
    let mut fty: metamodelica::Ref<DAE::Type>;
    ty = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: __esc_fty @ Deref @ DAE::Type::T_FUNCTION { .. } } => {
            fty = (*__esc_fty).clone();
            assign_variant_field!(fty => DAE::Type::T_FUNCTION;
                funcArg = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::FuncArg>> = metamodelica::nil();
        for mut a in (var_field!((*fty).funcArg, DAE::Type::T_FUNCTION).clone()).into_iter().cloned() {
            let __x = unboxFuncArg(a.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
                funcResultType = unboxResultType(var_field!((*fty).funcResultType, DAE::Type::T_FUNCTION).clone())?
            );
            assign_variant_field!(ty => DAE::Type::T_FUNCTION_REFERENCE_VAR; functionType = fty.clone());
            ty
        },
        _ => ty,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ty)
}

fn unboxFuncArg(mut arg: metamodelica::Ref<DAE::FuncArg>) -> Result<metamodelica::Ref<DAE::FuncArg>> {
    let mut arg: metamodelica::Ref<DAE::FuncArg> = arg;
    arg = (match &*arg {
        DAE::FuncArg { .. } => {
            assign_field!(arg.ty = Types::unboxedType(arg.ty.clone())?);
            arg
        }
    });
    Ok(arg)
}

pub fn absoluteClockIdxForBaseClock(
    mut baseClockIdx: i32,
    mut allBaseClockPartitions: &metamodelica::List<SimCode::ClockedPartition>,
) -> Result<i32> {
    let mut absBaseClockIdx: i32;
    let mut i: i32 = 1;
    absBaseClockIdx = 1;
    while i < baseClockIdx {
        absBaseClockIdx = absBaseClockIdx + ((getSubPartition(&((allBaseClockPartitions).get(i)?))).len() as i32);
        i = i + 1;
    }
    Ok(absBaseClockIdx)
}

pub fn getClockedPartitions(
    mut simcode: &metamodelica::Ref<SimCode::SimCode>,
) -> metamodelica::List<SimCode::ClockedPartition> {
    let mut clockedPartitions: metamodelica::List<SimCode::ClockedPartition>;
    clockedPartitions = simcode.clockedPartitions.clone();
    clockedPartitions
}

pub(crate) fn isScalarLiteralAssignment(mut eq: &metamodelica::Ref<SimCode::SimEqSystem>) -> Result<bool> {
    let mut b: bool;
    b = (match &**eq {
        SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { exp: __eq_exp, .. } => {
            Expression::isSimpleLiteralValue(metamodelica::AsArg::as_arg(&__eq_exp), false)?
        }
        _ => false,
    });
    Ok(b)
}

pub fn selectScalarLiteralAssignments(
    mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
) -> Result<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>> {
    let mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>> = eqs;
    eqs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>> = metamodelica::nil();
        for mut e in (eqs).into_iter().cloned() {
            if !(isScalarLiteralAssignment(&(e.clone()))?) {
                continue;
            }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(eqs)
}

pub fn filterScalarLiteralAssignments(
    mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
) -> Result<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>> {
    let mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>> = eqs;
    eqs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>> = metamodelica::nil();
        for mut e in (eqs).into_iter().cloned() {
            if !(!(isScalarLiteralAssignment(&(e.clone()))?)) {
                continue;
            }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(eqs)
}

pub fn sortSimpleAssignmentBasedOnLhs(
    mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
) -> Result<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>> {
    let mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>> = eqs;
    let mut simCode: metamodelica::Ref<SimCode::SimCode> = getSimCode()?;
    eqs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>> = metamodelica::nil();
        for mut e in (List::sort(
            ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<SimCode::SimEqSystem>, Option<(i32, i32, i32)>)> =
                    metamodelica::nil();
                for mut eq in (eqs).into_iter().cloned() {
                    let __x = (eq.clone(), lhsSortKey(&(eq.clone()), &simCode)?);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            (std::sync::Arc::new(
                move |__a0: (metamodelica::Ref<SimCode::SimEqSystem>, Option<(i32, i32, i32)>),
                      __a1: (metamodelica::Ref<SimCode::SimEqSystem>, Option<(i32, i32, i32)>)|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(keyedLhsGreaterThan(&__a0, &__a1))
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            (metamodelica::Ref<SimCode::SimEqSystem>, Option<(i32, i32, i32)>),
                            (metamodelica::Ref<SimCode::SimEqSystem>, Option<(i32, i32, i32)>),
                        ) -> Result<bool>
                        + 'static,
                >),
        )?)
        .into_iter()
        .cloned()
        {
            let __x = Util::tuple21(e.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(eqs)
}

fn lhsSortKey(
    mut eq: &metamodelica::Ref<SimCode::SimEqSystem>,
    mut simCode: &metamodelica::Ref<SimCode::SimCode>,
) -> Result<Option<(i32, i32, i32)>> {
    let mut key: Option<(i32, i32, i32)>;
    key = (match &**eq {
        SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { cref: __eq_cref, .. } => {
            let mut v: metamodelica::Ref<SimCodeVar::SimVar>;
            v = cref2simvar(__eq_cref.clone(), simCode)?;
            Some((
                metamodelica::valueConstructor((&*v.type_.clone()))?,
                metamodelica::valueConstructor((&v.varKind.clone()))?,
                v.index.clone(),
            ))
        }
        _ => None,
    });
    Ok(key)
}

fn keyedLhsGreaterThan(
    mut e1: &(metamodelica::Ref<SimCode::SimEqSystem>, Option<(i32, i32, i32)>),
    mut e2: &(metamodelica::Ref<SimCode::SimEqSystem>, Option<(i32, i32, i32)>),
) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &((e1, e2)) {
        ((_, Some((t1, k1, i1))), (_, Some((t2, k2, i2)))) => {
            if (t1.clone() == t2.clone()) {if (k1.clone() == k2.clone()) {i1.clone() > i2.clone()} else {k1.clone() > k2.clone()}} else {t1.clone() > t2.clone()}
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn getNumContinuousEquations(
    mut eqns: &metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
    mut numStates: i32,
) -> i32 {
    let mut n: i32;
    let mut numEqns: i32 = 0;
    for mut eqn in &**eqns {
        numEqns = numEqns + getNumContinuousEquationsSingleEq(metamodelica::AsArg::as_arg(&eqn));
    }
    n = numEqns + numStates;
    n
}

fn getNumContinuousEquationsSingleEq<'__b>(mut eqn: &'__b metamodelica::Ref<SimCode::SimEqSystem>) -> i32 {
    '__tco: loop {
        ::match_deref::match_deref! { match eqn {
            Deref @ SimCode::SimEqSystem::SES_MIXED { .. } => {
                { eqn = var_field!((**eqn).cont, SimCode::SimEqSystem::SES_MIXED); continue '__tco; }
            },
            Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: ls @ Deref @ SimCode::LinearSystem { index: _, .. }, .. } => {
                return ((ls.vars).len() as i32)
            },
            Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: nls @ Deref @ SimCode::NonlinearSystem { index: _, .. }, .. } => {
                return ((nls.crefs).len() as i32)
            },
            _ => {
                return 1
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn lookupVR(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut simCode: &metamodelica::Ref<SimCode::SimCode>,
) -> Result<i32> {
    let mut vr: i32;
    vr = AvlTreeCRToInt::get(&simCode.valueReferences, cr)?;
    Ok(vr)
}

pub fn isFMUSimCode(mut simCode: &metamodelica::Ref<SimCode::SimCode>) -> bool {
    let mut isFMU: bool;
    isFMU = (match &*simCode.valueReferences.clone() {
        AvlTreeCRToInt::Tree::EMPTY { .. } => false,
        _ => true,
    });
    isFMU
}

pub fn lookupVRForRealOutputDerivative(
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut simCode: &metamodelica::Ref<SimCode::SimCode>,
    mut fmuType: &ArcStr,
) -> Result<i32> {
    let mut vr: i32;
    let mut outputRealDerivativeCref: metamodelica::Ref<DAE::ComponentRef>;
    if metamodelica::stringEq(&fmuType, &(literal!("cs"))) {
        outputRealDerivativeCref = ComponentReference::appendStringLastIdent(&(literal!("_der")), cr)?;
        outputRealDerivativeCref = ComponentReference::prependStringCref(literal!("$"), &outputRealDerivativeCref)?;
        vr = AvlTreeCRToInt::get(&simCode.valueReferences, outputRealDerivativeCref)?;
    } else {
        vr = -1;
    }
    Ok(vr)
}

pub fn fmi3ArrayView(mut simCode: metamodelica::Ref<SimCode::SimCode>) -> Result<metamodelica::Ref<SimCode::SimCode>> {
    let mut view: metamodelica::Ref<SimCode::SimCode> = simCode.clone();
    let mut ms: SimCode::FmiModelStructure;
    let mut mi: SimCode::ModelInfo;
    let mut vars: SimCodeVar::SimVars;
    let mut iu: SimCode::FmiInitialUnknowns;
    let mut firsts: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, i32>>;
    let mut rep: metamodelica::Ref<UnorderedMap::UnorderedMap<i32, i32>>;
    if (simCode.modelStructure).is_none() {
        return Ok(view);
    }
    let __pa0 = ::match_deref::match_deref! { match &(simCode.modelStructure.clone()) {
        Some(__pa0) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    ms = metamodelica::Own::own(__pa0);
    if (ms.fmiArrays).is_empty() {
        return Ok(view);
    }
    firsts = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
            ComponentReferenceBasics::hashComponentRef(&__a0)
        })
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| {
                ComponentReferenceBasics::crefEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    rep = UnorderedMap::new(
        std::sync::Arc::new(fnptr!(Util::id, _)),
        (std::sync::Arc::new(fnptr!(intEq, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        1,
    );
    for mut a in &*ms.fmiArrays.clone() {
        UnorderedMap::add(a.first.clone(), a.numElements.clone(), firsts.clone())?;
        for mut k in 1..=a.numElements.clone() - 1 {
            UnorderedMap::add(a.fmiIndex.clone() + k, a.fmiIndex.clone(), rep.clone())?;
        }
    }
    mi = simCode.modelInfo.clone();
    vars = mi.vars.clone();
    vars.stateVars = fmi3CollapseArrays(vars.stateVars.clone(), firsts.clone())?;
    vars.derivativeVars = fmi3CollapseArrays(vars.derivativeVars.clone(), firsts.clone())?;
    vars.algVars = fmi3CollapseArrays(vars.algVars.clone(), firsts.clone())?;
    vars.discreteAlgVars = fmi3CollapseArrays(vars.discreteAlgVars.clone(), firsts.clone())?;
    vars.paramVars = fmi3CollapseArrays(vars.paramVars.clone(), firsts.clone())?;
    vars.intAlgVars = fmi3CollapseArrays(vars.intAlgVars.clone(), firsts.clone())?;
    vars.intParamVars = fmi3CollapseArrays(vars.intParamVars.clone(), firsts.clone())?;
    vars.boolAlgVars = fmi3CollapseArrays(vars.boolAlgVars.clone(), firsts.clone())?;
    vars.boolParamVars = fmi3CollapseArrays(vars.boolParamVars.clone(), firsts.clone())?;
    vars.stringAlgVars = fmi3CollapseArrays(vars.stringAlgVars.clone(), firsts.clone())?;
    vars.stringParamVars = fmi3CollapseArrays(vars.stringParamVars.clone(), firsts)?;
    mi.vars = vars;
    assign_field!(view.modelInfo = mi);
    ms.fmiOutputs = SimCode::FmiOutputs {
        fmiUnknownsList: fmi3CollapseUnknowns(&ms.fmiOutputs.fmiUnknownsList, rep.clone())?,
    };
    ms.fmiDerivatives = SimCode::FmiDerivatives {
        fmiUnknownsList: fmi3CollapseUnknowns(&ms.fmiDerivatives.fmiUnknownsList, rep.clone())?,
    };
    ms.fmiDiscreteStates = SimCode::FmiDiscreteStates {
        fmiUnknownsList: fmi3CollapseUnknowns(&ms.fmiDiscreteStates.fmiUnknownsList, rep.clone())?,
    };
    iu = ms.fmiInitialUnknowns.clone();
    iu.fmiUnknownsList = fmi3CollapseUnknowns(&iu.fmiUnknownsList, rep)?;
    ms.fmiInitialUnknowns = iu;
    assign_field!(view.modelStructure = Some(ms));
    Ok(view)
}

fn fmi3CollapseArrays(
    mut vars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
    mut firsts: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, i32>>,
) -> Result<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>> {
    let mut outVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut rest: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = vars;
    let mut elements: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut v: metamodelica::Ref<SimCodeVar::SimVar>;
    let mut n: i32;
    while !((rest).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        v = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        n = UnorderedMap::getOrDefault(v.name.clone(), firsts.clone(), 0)?;
        if n > 1 {
            (elements, rest) = List::split(rest, n - 1)?;
            v = fmi3ArrayVar(v, elements)?;
        }
        outVars = metamodelica::cons(v, outVars);
    }
    outVars = outVars.reverse();
    Ok(outVars)
}

fn fmi3ArrayVar(
    mut first: metamodelica::Ref<SimCodeVar::SimVar>,
    mut others: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
) -> Result<metamodelica::Ref<SimCodeVar::SimVar>> {
    let mut var: metamodelica::Ref<SimCodeVar::SimVar> = first.clone();
    assign_field!(
        var.type_ = metamodelica::Ref::new(DAE::Type::T_ARRAY {
            ty: first.type_.clone(),
            dims: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Dimension>> = metamodelica::nil();
                for mut d in (first.numArrayElement.clone()).into_iter().cloned() {
                    let __x = metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER {
                        integer: stringInt(d.clone())?,
                    });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        }),
        var.exportVar = Some(ComponentReferenceBasics::crefStripLastSubs(
            &(first.exportVar.clone().ok_or("pattern mismatch")?)
        )?)
    );
    assign_field!(
        var.initialValue = (::match_deref::match_deref! { match &(first.initialValue.clone()) {
            Some(_) => Some(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: var.type_.clone(), scalar: true, array: ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
            for mut v in (metamodelica::cons(first, others)).into_iter().cloned() {
                let __x = v.initialValue.clone().ok_or("pattern mismatch")?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }) })),
            _ => None,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    );
    Ok(var)
}

fn fmi3CollapseUnknowns(
    mut unknowns: &metamodelica::List<SimCode::FmiUnknown>,
    mut rep: metamodelica::Ref<UnorderedMap::UnorderedMap<i32, i32>>,
) -> Result<metamodelica::List<SimCode::FmiUnknown>> {
    let mut outUnknowns: metamodelica::List<SimCode::FmiUnknown> = metamodelica::nil();
    let mut slot: metamodelica::Ref<UnorderedMap::UnorderedMap<i32, i32>> = UnorderedMap::new(
        std::sync::Arc::new(fnptr!(Util::id, _)),
        (std::sync::Arc::new(fnptr!(intEq, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        1,
    );
    let mut deps: metamodelica::Array<metamodelica::List<i32>> =
        arrayCreate(((unknowns).len() as i32), metamodelica::nil());
    let mut order: metamodelica::List<i32> = metamodelica::nil();
    let mut ds: metamodelica::List<i32>;
    let mut r: i32 = 0;
    let mut i: i32;
    let mut n: i32 = 0;
    for mut u in &**unknowns {
        r = UnorderedMap::getOrDefault(u.index.clone(), rep.clone(), u.index.clone())?;
        ds = ({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut d in (u.dependencies.clone()).into_iter().cloned() {
                let __x = UnorderedMap::getOrDefault(d.clone(), rep.clone(), d.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        i = UnorderedMap::getOrDefault(r, slot.clone(), 0)?;
        if i == 0 {
            n = n + 1;
            i = n;
            UnorderedMap::add(r, i, slot.clone())?;
            order = metamodelica::cons(r, order);
        }
        metamodelica::arrayUpdate(
            deps.clone(),
            i,
            listAppend(ds, metamodelica::arrayGet(deps.clone(), i)?),
        )?;
    }
    for mut r in &*order {
        let mut r = r.clone();
        ds = List::sortedUnique(
            List::sort(
                metamodelica::arrayGet(deps.clone(), UnorderedMap::getOrFail(r, slot.clone())?)?,
                (std::sync::Arc::new(fnptr!(intGt, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?,
            &fnptr!(intEq, i32, i32),
        )?;
        outUnknowns = metamodelica::cons(
            SimCode::FmiUnknown {
                index: r,
                dependencies: ds.clone(),
                dependenciesKind: List::fill(literal!("dependent"), ((ds).len() as i32)),
            },
            outUnknowns,
        );
    }
    Ok(outUnknowns)
}

pub fn fmi3ArrayDefines(mut simCode: &metamodelica::Ref<SimCode::SimCode>) -> Result<ArcStr> {
    let mut defines: ArcStr;
    let mut arrays: metamodelica::List<(i32, i32)> = metamodelica::nil();
    let mut ht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<SimCodeVar::SimVar>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCrefSimVar::FuncHashCref,
            HashTableCrefSimVar::FuncCrefEqual,
            HashTableCrefSimVar::FuncCrefStr,
            HashTableCrefSimVar::FuncExpStr,
        ),
    );
    let mut v: metamodelica::Ref<SimCodeVar::SimVar>;
    let _ = (match simCode.modelStructure.clone() {
        Some(mut ms) if (!((ms.fmiArrays).is_empty())) => {
            ht = createCrefToSimVarHT(&simCode.modelInfo)?;
            for mut a in &*ms.fmiArrays.clone() {
                v = BaseHashTable::get(a.first.clone(), &ht)?;
                arrays = metamodelica::cons(
                    (stringInt(getFMI3ValueReference(&v, simCode)?)?, a.numElements.clone()),
                    arrays,
                );
            }
            arrays = List::sort(arrays, std::sync::Arc::new(fnptr!(Util::compareTupleIntGt, _, _)))?;
            ()
        }
        _ => (),
    });
    defines = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("#define FMI3_NUMBER_OF_ARRAYS "));
        __mm_s.push_str(&*intString(((arrays).len() as i32)));
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*literal!("#define FMI3_ARRAY_VRS { "));
        __mm_s.push_str(&*stringDelimitList(
            ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut a in (arrays.clone()).into_iter().cloned() {
                    let __x = intString(Util::tuple21(a.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            literal!(", "),
        ));
        __mm_s.push_str(&*literal!(" }\n"));
        __mm_s.push_str(&*literal!("#define FMI3_ARRAY_LENGTHS { "));
        __mm_s.push_str(&*stringDelimitList(
            ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut a in (arrays).into_iter().cloned() {
                    let __x = intString(Util::tuple22(a.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            literal!(", "),
        ));
        __mm_s.push_str(&*literal!(" }"));
        ArcStr::from(__mm_s)
    };
    Ok(defines)
}

pub fn isFMI3NestableAlias(mut simVar: &metamodelica::Ref<SimCodeVar::SimVar>) -> bool {
    let mut nestable: bool;
    nestable = (match &**simVar {
        SimCodeVar::SimVar {
            aliasvar: SimCodeVar::AliasVariable::ALIAS { .. },
            ..
        } if ((simVar.exportVar).is_some()
            && !(Types::isArray(&simVar.type_))
            && (match simVar.causality.clone() {
                None => true,
                Some(SimCodeVar::Causality::LOCAL { .. }) => true,
                Some(SimCodeVar::Causality::NONECAUS { .. }) => true,
                _ => false,
            })) =>
        {
            true
        }
        _ => false,
    });
    nestable
}

fn fmi3AliasTargetValueReference(
    mut v: &metamodelica::Ref<SimCodeVar::SimVar>,
    mut simCode: &metamodelica::Ref<SimCode::SimCode>,
) -> Result<Option<i32>> {
    let mut vr: Option<i32>;
    vr = (match v.aliasvar.clone() {
        SimCodeVar::AliasVariable::ALIAS { varName: ref cr } => {
            let mut local_: i32;
            (match AvlTreeCRToInt::getOpt(&simCode.valueReferences, cr.clone())? {
                Some(mut __esc_local_) => {
                    local_ = __esc_local_.clone();
                    Some(getFMI3TypeOffset(&v.type_, &simCode.modelInfo) + local_)
                }
                _ => None,
            })
        }
        _ => None,
    });
    Ok(vr)
}

pub fn cacheFMI3VariableAliases(mut simCode: &metamodelica::Ref<SimCode::SimCode>) -> Result<ArcStr> {
    let mut dummy: ArcStr = literal!("");
    let mut vars: SimCodeVar::SimVars = simCode.modelInfo.vars.clone();
    let mut aliasLists: metamodelica::List<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>> = list![
        vars.aliasVars.clone(),
        vars.intAliasVars.clone(),
        vars.boolAliasVars.clone(),
        vars.stringAliasVars.clone()
    ];
    let mut table: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>;
    let mut n: i32 = 0;
    let mut vr: i32;
    for mut lst in &*aliasLists {
        for mut v in &*lst.clone() {
            if isFMI3NestableAlias(metamodelica::AsArg::as_arg(&v)) {
                n = (match fmi3AliasTargetValueReference(metamodelica::AsArg::as_arg(&v), simCode)? {
                    Some(mut __esc_vr) => {
                        vr = __esc_vr.clone();
                        intMax(n, vr + 1)
                    }
                    _ => n,
                });
            }
        }
    }
    table = arrayCreate(n, metamodelica::nil());
    for mut lst in &*aliasLists {
        for mut v in &*lst.clone() {
            if isFMI3NestableAlias(metamodelica::AsArg::as_arg(&v)) {
                let _ = (match fmi3AliasTargetValueReference(metamodelica::AsArg::as_arg(&v), simCode)? {
                    Some(mut __esc_vr) => {
                        vr = __esc_vr.clone();
                        metamodelica::arrayUpdate(
                            table.clone(),
                            vr + 1,
                            metamodelica::cons(v.clone(), metamodelica::arrayGet(table.clone(), vr + 1)?),
                        )?;
                        ()
                    }
                    _ => (),
                });
            }
        }
    }
    for mut i in 1..=n {
        metamodelica::arrayUpdate(table.clone(), i, metamodelica::arrayGet(table.clone(), i)?.reverse())?;
    }
    {
        let __v = Some(table.clone());
        crate::Globals::fmi3VariableAliasCache.with(|__root| *__root.borrow_mut() = __v)
    };
    Ok(dummy)
}

pub fn clearFMI3VariableAliases() -> ArcStr {
    let mut dummy: ArcStr = literal!("");
    {
        let __v = None;
        crate::Globals::fmi3VariableAliasCache.with(|__root| *__root.borrow_mut() = __v)
    };
    dummy
}

pub fn getFMI3VariableAliases(
    mut simCode: &metamodelica::Ref<SimCode::SimCode>,
    mut canonical: &metamodelica::Ref<SimCodeVar::SimVar>,
) -> Result<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>> {
    let mut aliases: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut vars: SimCodeVar::SimVars = simCode.modelInfo.vars.clone();
    let mut cached: Option<metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>>;
    let mut table: metamodelica::Array<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>>;
    let mut vr: i32;
    cached = crate::Globals::fmi3VariableAliasCache.with(|__root| __root.borrow().clone());
    if (cached).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(cached) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        table = metamodelica::Own::own(__pa0);
        vr = getFMI3TypeOffset(&canonical.type_, &simCode.modelInfo)
            + (match AvlTreeCRToInt::getOpt(&simCode.valueReferences, canonical.name.clone())? {
                Some(mut local_) => local_,
                _ => -1,
            });
        if vr >= 0 && vr < metamodelica::arrayLength(table.clone()) {
            aliases = metamodelica::arrayGet(table.clone(), vr + 1)?;
        }
        return Ok(aliases);
    }
    for mut lst in &*list![
        vars.aliasVars.clone(),
        vars.intAliasVars.clone(),
        vars.boolAliasVars.clone(),
        vars.stringAliasVars.clone()
    ] {
        for mut v in &*lst.clone() {
            if isFMI3NestableAlias(metamodelica::AsArg::as_arg(&v)) {
                let _ = (match v.aliasvar.clone() {
                    SimCodeVar::AliasVariable::ALIAS { varName: ref cr }
                        if (ComponentReferenceBasics::crefEqualNoStringCompare(
                            metamodelica::AsArg::as_arg(&cr),
                            &canonical.name,
                        )?) =>
                    {
                        aliases = metamodelica::cons(v.clone(), aliases);
                        ()
                    }
                    _ => (),
                });
            }
        }
    }
    aliases = aliases.reverse();
    Ok(aliases)
}

pub fn getFMI3Terminals(
    mut simCode: &metamodelica::Ref<SimCode::SimCode>,
) -> Result<metamodelica::List<SimCode::FmiTerminal>> {
    let mut terminals: metamodelica::List<SimCode::FmiTerminal> = metamodelica::nil();
    let mut vars: SimCodeVar::SimVars;
    let mut allVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    let mut flat: metamodelica::List<(ArcStr, ArcStr, bool, SimCode::FmiTerminalMember)> = metamodelica::nil();
    let mut names: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut memberNames: metamodelica::List<ArcStr>;
    let mut om: Option<(ArcStr, ArcStr, bool, SimCode::FmiTerminalMember)>;
    let mut tname: ArcStr;
    let mut tname2: ArcStr;
    let mut tkind: ArcStr;
    let mut tkind2: ArcStr;
    let mut texp: bool;
    let mut texp2: bool;
    let mut mem: SimCode::FmiTerminalMember;
    let mut mems: metamodelica::List<SimCode::FmiTerminalMember>;
    vars = simCode.modelInfo.vars.clone();
    allVars = List::flatten(list![
        vars.stateVars.clone(),
        vars.derivativeVars.clone(),
        vars.algVars.clone(),
        vars.discreteAlgVars.clone(),
        vars.paramVars.clone(),
        vars.intAlgVars.clone(),
        vars.intParamVars.clone(),
        vars.boolAlgVars.clone(),
        vars.boolParamVars.clone(),
        vars.stringAlgVars.clone(),
        vars.stringParamVars.clone(),
        vars.aliasVars.clone(),
        vars.intAliasVars.clone(),
        vars.boolAliasVars.clone(),
        vars.stringAliasVars.clone()
    ])?;
    for mut v in &*allVars {
        om = connectorMemberOf(metamodelica::AsArg::as_arg(&v));
        if (om).is_some() {
            flat = metamodelica::cons(om.ok_or("pattern mismatch")?, flat);
        }
    }
    flat = flat.reverse();
    for mut t in &*flat {
        (tname, _, _, _) = t.clone();
        if !(listMember(tname.clone(), names.clone())) {
            names = metamodelica::cons(tname, names);
        }
    }
    names = names.reverse();
    for mut nm in &*names {
        mems = metamodelica::nil();
        memberNames = metamodelica::nil();
        texp = false;
        tkind = literal!("");
        for mut t in &*flat {
            (tname2, tkind2, texp2, mem) = t.clone();
            if stringEq(&tname2, &nm) && !(listMember(mem.memberName.clone(), memberNames.clone())) {
                mems = metamodelica::cons(mem.clone(), mems);
                memberNames = metamodelica::cons(mem.memberName.clone(), memberNames);
                texp = texp2;
                tkind = tkind2;
            }
        }
        terminals = metamodelica::cons(
            SimCode::FmiTerminal {
                name: nm.clone(),
                terminalKind: tkind,
                isExpandable: texp,
                members: mems.reverse(),
            },
            terminals,
        );
    }
    terminals = listAppend(terminals.reverse(), simplePortTerminals(&vars, names));
    Ok(terminals)
}

fn simplePortTerminals(
    mut vars: &SimCodeVar::SimVars,
    mut taken: metamodelica::List<ArcStr>,
) -> metamodelica::List<SimCode::FmiTerminal> {
    let mut terminals: metamodelica::List<SimCode::FmiTerminal> = metamodelica::nil();
    let mut seen: metamodelica::List<ArcStr> = taken;
    for mut v in &*listAppend(vars.inputVars.clone(), vars.outputVars.clone()) {
        terminals = 'mc: {
            let __mc_input = v.clone();
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        if !(((v.exportVar).is_some())) { return Err("guard") }
                        let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                        let mut nm: ArcStr;
                        let mut seen: metamodelica::List<ArcStr> = seen.clone();
                        cr = v.exportVar.clone().ok_or("pattern mismatch")?;
                        let __pa0 = ::match_deref::match_deref! { match &(cr.clone()) {
                            Deref @ DAE::ComponentRef::CREF_IDENT { ident: __pa0, subscriptLst: Deref @ metamodelica::ListNode::Nil, .. } => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        nm = metamodelica::Own::own(__pa0);
                        if listMember(nm.clone(), seen.clone()) {
                            return Err("fail");
                        }
                        seen = metamodelica::cons(nm.clone(), seen.clone());
                        Ok((metamodelica::cons(SimCode::FmiTerminal { name: nm.clone(), terminalKind: literal!(""), isExpandable: false, members: list![SimCode::FmiTerminalMember { variable: cr.clone(), memberName: nm.clone(), variableKind: literal!("signal") }] }, terminals.clone()), seen.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                seen = __wb0;
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        Ok(terminals.clone())
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            panic!("matchcontinue: no arm matched")
        };
    }
    terminals = terminals.reverse();
    terminals
}

fn connectorMemberOf(
    mut var: &metamodelica::Ref<SimCodeVar::SimVar>,
) -> Option<(ArcStr, ArcStr, bool, SimCode::FmiTerminalMember)> {
    let mut result: Option<(ArcStr, ArcStr, bool, SimCode::FmiTerminalMember)>;
    result = 'mc: {
        let __mc_input = &**var;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if !(((var.exportVar).is_some())) { return Err("guard") }
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut tname: ArcStr;
                    let mut member: ArcStr;
                    let mut tkind: ArcStr;
                    let mut kind: ArcStr;
                    let mut isExp: bool;
                    cref = var.exportVar.clone().ok_or("pattern mismatch")?;
                    (tname, member, isExp, tkind) = crefConnectorSplit(&cref)?;
                    kind = if (var.isConnectorFlow.clone()) {literal!("inflow")} else {literal!("signal")};
                    Ok(Some((tname.clone(), tkind.clone(), isExp, SimCode::FmiTerminalMember { variable: cref.clone(), memberName: member.clone(), variableKind: kind.clone() })))
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
    result
}

pub fn getFMI3VisualizationResource(mut simCode: &metamodelica::Ref<SimCode::SimCode>) -> Result<ArcStr> {
    let mut resource: ArcStr = literal!("");
    let mut path: ArcStr;
    if Flags::isSet(Flags::VISUAL_XML.clone())? {
        path = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*simCode.fileNamePrefix);
            __mm_s.push_str(&*literal!("_visual.xml"));
            ArcStr::from(__mm_s)
        };
        if System::regularFileExists(path) {
            resource = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*simCode.fileNamePrefix);
                __mm_s.push_str(&*literal!("_visual.xml"));
                ArcStr::from(__mm_s)
            };
        }
    }
    Ok(resource)
}

fn crefConnectorSplit(mut cref: &metamodelica::Ref<DAE::ComponentRef>) -> Result<(ArcStr, ArcStr, bool, ArcStr)> {
    let mut terminalName: ArcStr;
    let mut memberName: ArcStr;
    let mut isExpandable: bool;
    let mut terminalKind: ArcStr;
    (terminalName, memberName, isExpandable, terminalKind) = (match &**cref {
        DAE::ComponentRef::CREF_QUAL {
            ident: id,
            identType: ity,
            componentRef: rest,
            ..
        } if (Types::isConnector(ity)) => (
            id.clone(),
            ComponentReference::crefStr(rest)?,
            connectorIsExpandable(ity),
            connectorTypePath(ity)?,
        ),
        DAE::ComponentRef::CREF_QUAL {
            ident: id,
            componentRef: rest,
            ..
        } => {
            let mut innerT: ArcStr;
            let mut innerM: ArcStr;
            let mut innerK: ArcStr;
            let mut isExp: bool;
            (innerT, innerM, isExp, innerK) = crefConnectorSplit(rest)?;
            (
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*id);
                    __mm_s.push_str(&*literal!("."));
                    __mm_s.push_str(&*innerT);
                    ArcStr::from(__mm_s)
                },
                innerM,
                isExp,
                innerK,
            )
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((terminalName, memberName, isExpandable, terminalKind))
}

fn connectorIsExpandable(mut ty: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut isExpandable: bool;
    isExpandable = (match &**ty {
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::CONNECTOR { isExpandable: b, .. },
            ..
        } => b.clone(),
        DAE::Type::T_SUBTYPE_BASIC {
            complexClassType: ClassInf::State::CONNECTOR { isExpandable: b, .. },
            ..
        } => b.clone(),
        _ => false,
    });
    isExpandable
}

fn connectorTypePath(mut ty: &metamodelica::Ref<DAE::Type>) -> Result<ArcStr> {
    let mut path: ArcStr;
    path = (match &**ty {
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::CONNECTOR { path: p, .. },
            ..
        } => AbsynUtil::pathString(p.clone(), literal!("."), true, false)?,
        DAE::Type::T_SUBTYPE_BASIC {
            complexClassType: ClassInf::State::CONNECTOR { path: p, .. },
            ..
        } => AbsynUtil::pathString(p.clone(), literal!("."), true, false)?,
        _ => {
            literal!("")
        }
    });
    Ok(path)
}

pub fn getFMI3Clocks(
    mut simCode: &metamodelica::Ref<SimCode::SimCode>,
) -> Result<metamodelica::List<SimCode::FmiClock>> {
    let mut clocks: metamodelica::List<SimCode::FmiClock> = metamodelica::nil();
    let mut offset: i32;
    let mut i: i32 = 0;
    offset = getFMI3ClockVROffset(&simCode.modelInfo)?;
    for mut p in &*simCode.clockedPartitions.clone() {
        clocks = metamodelica::cons(makeFmiClock(&p.baseClock, offset + i, i), clocks);
        i = i + 1;
    }
    clocks = clocks.reverse();
    Ok(clocks)
}

fn getFMI3ClockVROffset(mut modelInfo: &SimCode::ModelInfo) -> Result<i32> {
    let mut offset: i32;
    let mut vars: SimCodeVar::SimVars = modelInfo.vars.clone();
    offset = 2 * numScalarElems(&vars.stateVars)?
        + numScalarElems(&vars.algVars)?
        + numScalarElems(&vars.discreteAlgVars)?
        + numScalarElems(&vars.paramVars)?
        + numScalarElems(&vars.aliasVars)?
        + numScalarElems(&vars.intAlgVars)?
        + numScalarElems(&vars.intParamVars)?
        + numScalarElems(&vars.intAliasVars)?
        + numScalarElems(&vars.boolAlgVars)?
        + numScalarElems(&vars.boolParamVars)?
        + numScalarElems(&vars.boolAliasVars)?
        + numScalarElems(&vars.stringAlgVars)?
        + numScalarElems(&vars.stringParamVars)?
        + numScalarElems(&vars.stringAliasVars)?
        + numScalarElems(&vars.extObjVars)?;
    Ok(offset)
}

fn makeFmiClock(mut kind: &metamodelica::Ref<DAE::ClockKind>, mut vr: i32, mut idx: i32) -> SimCode::FmiClock {
    let mut clk: SimCode::FmiClock;
    let mut nm: ArcStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("$clock"));
        __mm_s.push_str(&*intString(idx + 1));
        ArcStr::from(__mm_s)
    };
    clk = (match &**kind {
        DAE::ClockKind::REAL_CLOCK { interval: e } => SimCode::FmiClock {
            valueReference: vr,
            name: nm,
            intervalVariability: if (stringEq(&(clockConstString(e)), &(literal!("")))) {
                literal!("fixed")
            } else {
                literal!("constant")
            },
            supportsFraction: false,
            intervalDecimal: clockConstString(e),
            intervalCounter: literal!(""),
            resolution: literal!(""),
        },
        DAE::ClockKind::RATIONAL_CLOCK {
            intervalCounter: ic,
            resolution: res,
        } => SimCode::FmiClock {
            valueReference: vr,
            name: nm,
            intervalVariability: literal!("constant"),
            supportsFraction: true,
            intervalDecimal: literal!(""),
            intervalCounter: clockConstString(ic),
            resolution: clockConstString(res),
        },
        DAE::ClockKind::EVENT_CLOCK { .. } => SimCode::FmiClock {
            valueReference: vr,
            name: nm,
            intervalVariability: literal!("triggered"),
            supportsFraction: false,
            intervalDecimal: literal!(""),
            intervalCounter: literal!(""),
            resolution: literal!(""),
        },
        _ => SimCode::FmiClock {
            valueReference: vr,
            name: nm,
            intervalVariability: literal!("fixed"),
            supportsFraction: false,
            intervalDecimal: literal!(""),
            intervalCounter: literal!(""),
            resolution: literal!(""),
        },
    });
    clk
}

fn clockConstString(mut e: &metamodelica::Ref<DAE::Exp>) -> ArcStr {
    let mut s: ArcStr;
    s = (match &**e {
        DAE::Exp::RCONST { real: r } => realString(r.clone()),
        DAE::Exp::ICONST { integer: i } => intString(i.clone()),
        _ => {
            literal!("")
        }
    });
    s
}

pub fn unbalancedEqSystemPartition(
    mut inList: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
    mut maxLength: i32,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>>> {
    let mut partitions: metamodelica::List<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>>;
    let mut length: i32;
    let mut eqLength: i32;
    let mut lst: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut cur: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut first: metamodelica::Ref<SimCode::SimEqSystem>;
    lst = inList;
    cur = metamodelica::nil();
    partitions = metamodelica::nil();
    length = 0;
    while !((lst).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lst) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        first = metamodelica::Own::own(__pa0);
        lst = metamodelica::Own::own(__pa1);
        eqLength = getNumContinuousEquationsSingleEq(&first);
        if length > 0 && length + eqLength > maxLength {
            partitions = metamodelica::cons(cur, partitions);
            length = 0;
            cur = metamodelica::nil();
        }
        length = eqLength + length;
        cur = metamodelica::cons(first, cur);
    }
    if !((cur).is_empty()) {
        partitions = metamodelica::cons(cur, partitions);
    }
    Ok(partitions)
}

pub fn selectNLEqSys(
    mut simEqSysIn: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>,
) -> Result<metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>> {
    let mut eqs: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>>;
    let mut e: metamodelica::Ref<SimCode::SimEqSystem>;
    eqs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<SimCode::SimEqSystem>> = metamodelica::nil();
        for mut eq in (simEqSysIn).into_iter().cloned() {
            if !(::match_deref::match_deref! { match &(eq.clone()) {
                Deref @ SimCode::SimEqSystem::SES_NONLINEAR { .. } => true,
                Deref @ SimCode::SimEqSystem::SES_MIXED { cont: Deref @ SimCode::SimEqSystem::SES_NONLINEAR { .. }, .. } => true,
                _ => false,
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } }) {
                continue;
            }
            let __x = (::match_deref::match_deref! { match &(eq.clone()) {
                Deref @ SimCode::SimEqSystem::SES_NONLINEAR { .. } => eq.clone(),
                Deref @ SimCode::SimEqSystem::SES_MIXED { cont: __esc_e @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { .. }, .. } => {
                    e = (*__esc_e).clone();
                    e.clone()
                },
                _ => return Err("match: no arm matched"),
            } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(eqs)
}

fn matrixFormatC(mut size: i32, mut nnz: Option<i32>, mut isLinear: bool) -> Result<ArcStr> {
    let mut format: ArcStr;
    let mut entries: i32 = nnz.clone().unwrap_or(size * size);
    format = if (useSparseSolver(size, entries, isLinear)?) {
        literal!("OMC_MATRIX_SPARSE")
    } else {
        literal!("OMC_MATRIX_DENSE")
    };
    Ok(format)
}

pub fn linearSystemMatrixFormat(mut ls: &metamodelica::Ref<SimCode::LinearSystem>) -> Result<ArcStr> {
    let mut format: ArcStr;
    let mut nnz: Option<i32>;
    nnz = sparsityNonzeros(ls.jacobianMatrix.clone());
    if (nnz).is_none() {
        nnz = simJacNonzeros(&ls.simJac);
    }
    format = matrixFormatC(((ls.vars).len() as i32), nnz, true)?;
    Ok(format)
}

pub fn nonlinearSystemMatrixFormat(mut nls: &metamodelica::Ref<SimCode::NonlinearSystem>) -> Result<ArcStr> {
    let mut format: ArcStr;
    format = (match &**nls {
        SimCode::NonlinearSystem { .. } => matrixFormatC(
            ((nls.crefs).len() as i32),
            sparsityNonzeros(nls.jacobianMatrix.clone()),
            false,
        )?,
    });
    Ok(format)
}

fn simJacNonzeros(mut simJac: &metamodelica::List<(i32, i32, metamodelica::Ref<SimCode::SimEqSystem>)>) -> Option<i32> {
    let mut nnz: Option<i32> = if ((simJac).is_empty()) {
        None
    } else {
        Some(((simJac).len() as i32))
    };
    nnz
}

fn sparsityNonzeros(mut ojac: Option<metamodelica::Ref<SimCode::JacobianMatrix>>) -> Option<i32> {
    let mut nnz: Option<i32>;
    let mut entries: i32 = 0;
    nnz = (::match_deref::match_deref! { match &(ojac) {
        Some(Deref @ SimCode::JacobianMatrix { sparsity, .. }) if (!((sparsity).is_empty())) => {
            for mut col in &*sparsity.clone() {
                entries = entries + (((Util::tuple22(col.clone()))).len() as i32);
            }
            Some(entries)
        },
        Some(Deref @ SimCode::JacobianMatrix { sparsityMatrix: SimCode::Sparsity::SPARSITY { rows }, .. }) => {
            for mut row in &*rows.clone() {
                entries = entries + ((row.dependencies).len() as i32);
            }
            Some(entries)
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    nnz
}

pub fn getExpNominal(mut expr: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(expr.clone()) {
            Deref @ DAE::Exp::ICONST { integer: __expr_integer } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: (intReal(__expr_integer.clone())).abs() }))
            },
            Deref @ DAE::Exp::RCONST { real: __expr_real } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: (__expr_real.clone()).abs() }))
            },
            Deref @ DAE::Exp::CREF { componentRef: cr, ty: t } => {
                let mut v: metamodelica::Ref<SimCodeVar::SimVar>;
                let mut r1: metamodelica::Real;
                let mut e1: metamodelica::Ref<DAE::Exp>;
                v = cref2simvar(cr.clone(), &(getSimCode()?))?;
                ::match_deref::match_deref! { match &(v.nominalValue.clone()) {
            Some(Deref @ DAE::Exp::RCONST { real: __esc_r1 }) => {
                r1 = (*__esc_r1).clone();
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: (r1.clone()).abs() }))
            },
            Some(__esc_e1) => {
                e1 = (*__esc_e1).clone();
                return Ok(Expression::makePureBuiltinCall(literal!("abs"), list![e1.clone()], t.clone()))
            },
            None => match v.varKind.clone() {
            BackendDAE::VarKind::PARAM { .. } => return Ok(Expression::makePureBuiltinCall(literal!("abs"), list![expr], t.clone())),
            _ => return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) })),
        },
            _ => return Err("match: no arm matched"),
        } }
            },
            Deref @ DAE::Exp::BINARY { operator: DAE::Operator::ADD { .. }, exp1: __expr_exp1, exp2: __expr_exp2 } => {
                let mut r1: metamodelica::Real;
                let mut r2: metamodelica::Real;
                let mut e1: metamodelica::Ref<DAE::Exp>;
                let mut e2: metamodelica::Ref<DAE::Exp>;
                ::match_deref::match_deref! { match &((getExpNominal(__expr_exp1.clone())?, getExpNominal(__expr_exp2.clone())?)) {
            (Deref @ DAE::Exp::RCONST { real: __esc_r1 }, Deref @ DAE::Exp::RCONST { real: __esc_r2 }) => {
                r1 = (*__esc_r1).clone();
                r2 = (*__esc_r2).clone();
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r1.clone() + r2.clone() }))
            },
            (__esc_e1, __esc_e2) => {
                e1 = (*__esc_e1).clone();
                e2 = (*__esc_e2).clone();
                return Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: var_field!((*expr).operator, DAE::Exp::BINARY).clone(), exp2: e2.clone() }))
            },
            _ => return Err("match: no arm matched"),
        } }
            },
            Deref @ DAE::Exp::BINARY { operator: DAE::Operator::SUB { ty: t }, exp1: __expr_exp1, exp2: __expr_exp2 } => {
                let mut r1: metamodelica::Real;
                let mut r2: metamodelica::Real;
                let mut e1: metamodelica::Ref<DAE::Exp>;
                let mut e2: metamodelica::Ref<DAE::Exp>;
                ::match_deref::match_deref! { match &((getExpNominal(__expr_exp1.clone())?, getExpNominal(__expr_exp2.clone())?)) {
            (Deref @ DAE::Exp::RCONST { real: __esc_r1 }, Deref @ DAE::Exp::RCONST { real: __esc_r2 }) => {
                r1 = (*__esc_r1).clone();
                r2 = (*__esc_r2).clone();
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r1.clone() + r2.clone() }))
            },
            (__esc_e1, __esc_e2) => {
                e1 = (*__esc_e1).clone();
                e2 = (*__esc_e2).clone();
                return Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::ADD { ty: t.clone() }, exp2: e2.clone() }))
            },
            _ => return Err("match: no arm matched"),
        } }
            },
            Deref @ DAE::Exp::BINARY { operator: DAE::Operator::MUL { .. }, exp1: __expr_exp1, exp2: __expr_exp2 } => {
                let mut r1: metamodelica::Real;
                let mut r2: metamodelica::Real;
                let mut e1: metamodelica::Ref<DAE::Exp>;
                let mut e2: metamodelica::Ref<DAE::Exp>;
                ::match_deref::match_deref! { match &((getExpNominal(__expr_exp1.clone())?, getExpNominal(__expr_exp2.clone())?)) {
            (Deref @ DAE::Exp::RCONST { real: __esc_r1 }, Deref @ DAE::Exp::RCONST { real: __esc_r2 }) => {
                r1 = (*__esc_r1).clone();
                r2 = (*__esc_r2).clone();
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r1.clone() * r2.clone() }))
            },
            (__esc_e1, __esc_e2) => {
                e1 = (*__esc_e1).clone();
                e2 = (*__esc_e2).clone();
                return Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: var_field!((*expr).operator, DAE::Exp::BINARY).clone(), exp2: e2.clone() }))
            },
            _ => return Err("match: no arm matched"),
        } }
            },
            Deref @ DAE::Exp::BINARY { operator: DAE::Operator::DIV { .. }, exp1: __expr_exp1, exp2: __expr_exp2 } => {
                let mut r1: metamodelica::Real;
                let mut r2: metamodelica::Real;
                let mut e1: metamodelica::Ref<DAE::Exp>;
                let mut e2: metamodelica::Ref<DAE::Exp>;
                ::match_deref::match_deref! { match &((getExpNominal(__expr_exp1.clone())?, getExpNominal(__expr_exp2.clone())?)) {
            (Deref @ DAE::Exp::RCONST { real: __esc_r1 }, Deref @ DAE::Exp::RCONST { real: __esc_r2 }) => {
                r1 = (*__esc_r1).clone();
                r2 = (*__esc_r2).clone();
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::real_div_checked(r1.clone(), r2.clone())? }))
            },
            (__esc_e1, __esc_e2) => {
                e1 = (*__esc_e1).clone();
                e2 = (*__esc_e2).clone();
                return Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: var_field!((*expr).operator, DAE::Exp::BINARY).clone(), exp2: e2.clone() }))
            },
            _ => return Err("match: no arm matched"),
        } }
            },
            Deref @ DAE::Exp::BINARY { operator: DAE::Operator::POW { .. }, exp1: __expr_exp1, exp2: __expr_exp2 } => {
                let mut r1: metamodelica::Real;
                let mut r2: metamodelica::Real;
                let mut e1: metamodelica::Ref<DAE::Exp>;
                let mut e2: metamodelica::Ref<DAE::Exp>;
                ::match_deref::match_deref! { match &((getExpNominal(__expr_exp1.clone())?, getExpNominal(__expr_exp2.clone())?)) {
            (Deref @ DAE::Exp::RCONST { real: __esc_r1 }, Deref @ DAE::Exp::RCONST { real: __esc_r2 }) => {
                r1 = (*__esc_r1).clone();
                r2 = (*__esc_r2).clone();
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: (r1.clone()).powf(r2.clone()) }))
            },
            (__esc_e1, __esc_e2) => {
                e1 = (*__esc_e1).clone();
                e2 = (*__esc_e2).clone();
                return Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: var_field!((*expr).operator, DAE::Exp::BINARY).clone(), exp2: e2.clone() }))
            },
            _ => return Err("match: no arm matched"),
        } }
            },
            Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: __expr_exp } => {
                { expr = __expr_exp.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::IFEXP { expCond: __expr_expCond, expElse: __expr_expElse, expThen: __expr_expThen } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: __expr_expCond.clone(), expThen: getExpNominal(__expr_expThen.clone())?, expElse: getExpNominal(__expr_expElse.clone())? }))
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { expr = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sign" }, .. } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty: t, .. } } => {
                let mut r1: metamodelica::Real;
                let mut e2: metamodelica::Ref<DAE::Exp>;
                ::match_deref::match_deref! { match &(getExpNominal(e1.clone())?) {
            Deref @ DAE::Exp::RCONST { real: __esc_r1 } => {
                r1 = (*__esc_r1).clone();
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: (r1.clone()).sqrt() }))
            },
            __esc_e2 => {
                e2 = (*__esc_e2).clone();
                return Ok(Expression::makePureBuiltinCall(literal!("sqrt"), list![e2.clone()], t.clone()))
            },
            _ => return Err("match: no arm matched"),
        } }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "div" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
                let mut r1: metamodelica::Real;
                let mut r2: metamodelica::Real;
                ::match_deref::match_deref! { match &((getExpNominal(e1.clone())?, getExpNominal(e2.clone())?)) {
            (Deref @ DAE::Exp::RCONST { real: __esc_r1 }, Deref @ DAE::Exp::RCONST { real: __esc_r2 }) => {
                r1 = (*__esc_r1).clone();
                r2 = (*__esc_r2).clone();
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: std::cmp::max(metamodelica::OrderedFloat(1.0_f64), (metamodelica::real_div_checked(r1.clone(), r2.clone())?).abs()) }))
            },
            _ => return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) })),
            _ => return Err("match: no arm matched"),
        } }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "mod" }, expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
                let mut r2: metamodelica::Real;
                match &*(getExpNominal(e2.clone())?) {
            DAE::Exp::RCONST { real: __esc_r2 } => {
                r2 = (*__esc_r2).clone();
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r2.clone() }))
            },
            _ => return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) })),
        }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "rem" }, expLst: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
                let mut r2: metamodelica::Real;
                match &*(getExpNominal(e2.clone())?) {
            DAE::Exp::RCONST { real: __esc_r2 } => {
                r2 = (*__esc_r2).clone();
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r2.clone() }))
            },
            _ => return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) })),
        }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "ceil" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { expr = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "floor" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { expr = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }, .. } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cos" }, .. } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tan" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { expr = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "asin" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { expr = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "acos" }, .. } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan" }, .. } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "atan" }, .. } => {
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty: t, .. } } => {
                let mut r1: metamodelica::Real;
                let mut e2: metamodelica::Ref<DAE::Exp>;
                ::match_deref::match_deref! { match &(getExpNominal(e1.clone())?) {
            Deref @ DAE::Exp::RCONST { real: __esc_r1 } => {
                r1 = (*__esc_r1).clone();
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: (r1.clone()).sinh() }))
            },
            __esc_e2 => {
                e2 = (*__esc_e2).clone();
                return Ok(Expression::makePureBuiltinCall(literal!("sinh"), list![e2.clone()], t.clone()))
            },
            _ => return Err("match: no arm matched"),
        } }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty: t, .. } } => {
                let mut r1: metamodelica::Real;
                let mut e2: metamodelica::Ref<DAE::Exp>;
                ::match_deref::match_deref! { match &(getExpNominal(e1.clone())?) {
            Deref @ DAE::Exp::RCONST { real: __esc_r1 } => {
                r1 = (*__esc_r1).clone();
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: (r1.clone()).cosh() }))
            },
            __esc_e2 => {
                e2 = (*__esc_e2).clone();
                return Ok(Expression::makePureBuiltinCall(literal!("cosh"), list![e2.clone()], t.clone()))
            },
            _ => return Err("match: no arm matched"),
        } }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty: t, .. } } => {
                let mut r1: metamodelica::Real;
                let mut e2: metamodelica::Ref<DAE::Exp>;
                ::match_deref::match_deref! { match &(getExpNominal(e1.clone())?) {
            Deref @ DAE::Exp::RCONST { real: __esc_r1 } => {
                r1 = (*__esc_r1).clone();
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: (r1.clone()).tanh() }))
            },
            __esc_e2 => {
                e2 = (*__esc_e2).clone();
                return Ok(Expression::makePureBuiltinCall(literal!("tanh"), list![e2.clone()], t.clone()))
            },
            _ => return Err("match: no arm matched"),
        } }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty: t, .. } } => {
                let mut r1: metamodelica::Real;
                let mut e2: metamodelica::Ref<DAE::Exp>;
                ::match_deref::match_deref! { match &(getExpNominal(e1.clone())?) {
            Deref @ DAE::Exp::RCONST { real: __esc_r1 } => {
                r1 = (*__esc_r1).clone();
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: (r1.clone()).exp() }))
            },
            __esc_e2 => {
                e2 = (*__esc_e2).clone();
                return Ok(Expression::makePureBuiltinCall(literal!("exp"), list![e2.clone()], t.clone()))
            },
            _ => return Err("match: no arm matched"),
        } }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty: t, .. } } => {
                let mut r1: metamodelica::Real;
                let mut e2: metamodelica::Ref<DAE::Exp>;
                ::match_deref::match_deref! { match &(getExpNominal(e1.clone())?) {
            Deref @ DAE::Exp::RCONST { real: __esc_r1 } => {
                r1 = (*__esc_r1).clone();
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: (r1.clone()).ln() }))
            },
            __esc_e2 => {
                e2 = (*__esc_e2).clone();
                return Ok(Expression::makePureBuiltinCall(literal!("log"), list![e2.clone()], t.clone()))
            },
            _ => return Err("match: no arm matched"),
        } }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log10" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, attr: Deref @ DAE::CallAttributes { ty: t, .. } } => {
                let mut r1: metamodelica::Real;
                let mut e2: metamodelica::Ref<DAE::Exp>;
                ::match_deref::match_deref! { match &(getExpNominal(e1.clone())?) {
            Deref @ DAE::Exp::RCONST { real: __esc_r1 } => {
                r1 = (*__esc_r1).clone();
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: (r1.clone()).log10() }))
            },
            __esc_e2 => {
                e2 = (*__esc_e2).clone();
                return Ok(Expression::makePureBuiltinCall(literal!("log10"), list![e2.clone()], t.clone()))
            },
            _ => return Err("match: no arm matched"),
        } }
            },
            _ => {
                return Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn simIteratorString(mut iter: &BackendDAE::SimIterator) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match iter.clone() {
        BackendDAE::SimIterator::SIM_ITERATOR_RANGE { .. } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(var_field!(
                iter.name,
                BackendDAE::SimIterator::SIM_ITERATOR_RANGE
            ))?);
            __mm_s.push_str(&*literal!(" in "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(
                var_field!(iter.start, BackendDAE::SimIterator::SIM_ITERATOR_RANGE).clone(),
            )?);
            __mm_s.push_str(&*literal!(":"));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(
                var_field!(iter.step, BackendDAE::SimIterator::SIM_ITERATOR_RANGE).clone(),
            )?);
            __mm_s.push_str(&*literal!(":"));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(
                var_field!(iter.stop, BackendDAE::SimIterator::SIM_ITERATOR_RANGE).clone(),
            )?);
            ArcStr::from(__mm_s)
        }
        BackendDAE::SimIterator::SIM_ITERATOR_LIST { .. } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(var_field!(
                iter.name,
                BackendDAE::SimIterator::SIM_ITERATOR_LIST
            ))?);
            __mm_s.push_str(&*literal!(" in "));
            __mm_s.push_str(&*List::toString(
                var_field!(iter.lst, BackendDAE::SimIterator::SIM_ITERATOR_LIST).clone(),
                &fnptr!(intString, i32),
                List::Style::FLAT_CURLY_SHORT.clone(),
            )?);
            ArcStr::from(__mm_s)
        }
    });
    Ok(r#str)
}

pub fn useSparseSolver(mut size: i32, mut nnz: i32, mut isLinear: bool) -> Result<bool> {
    let mut sparse: bool;
    let maxDensityLinear: metamodelica::Real = metamodelica::OrderedFloat(0.2_f64);
    let maxDensityNonlinear: metamodelica::Real = metamodelica::OrderedFloat(0.1_f64);
    let minSize: i32 = 1000;
    sparse = if (size <= 0) {
        false
    } else {
        metamodelica::real_div_checked(intReal(nnz), intReal(size * size))?
            < if (isLinear) {
                maxDensityLinear
            } else {
                maxDensityNonlinear
            }
            || size > minSize
    };
    Ok(sparse)
}
