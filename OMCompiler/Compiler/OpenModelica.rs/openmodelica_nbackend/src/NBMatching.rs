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
use crate::NBBackendUtil as BackendUtil;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBModule as Module;
use crate::NBPartition as Partition;
use crate::NBResolveSingularities as ResolveSingularities;
use crate::NBSlice as Slice;
use crate::NBSlice::IntLst;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use openmodelica_ast::Absyn::Path;
use openmodelica_backend_util::BackendDAEEXT;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util::Vector;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

/// file:        NBMatching.mo
/// package:     NBMatching
/// description: This file contains the functions which perform the matching process;
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct NBMatching {
    /// eqn := var_to_eqn[var]
    pub var_to_eqn: metamodelica::Array<i32>,
    /// var := eqn_to_var[eqn]
    pub eqn_to_var: metamodelica::Array<i32>,
}

impl metamodelica::gc::MMTrace for NBMatching {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.var_to_eqn, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.eqn_to_var, __mmv)?;
        Ok(())
    }
}
impl Default for NBMatching {
    fn default() -> Self {
        Self {
            var_to_eqn: Default::default(),
            eqn_to_var: Default::default(),
        }
    }
}

pub type MATCHING = NBMatching;

thread_local! { static __EMPTY_MATCHING_TLS: metamodelica::Ref<NBMatching> = metamodelica::Ref::new(NBMatching { var_to_eqn: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), eqn_to_var: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()) }); }
pub(crate) fn EMPTY_MATCHING() -> metamodelica::Ref<NBMatching> {
    __EMPTY_MATCHING_TLS.with(|__t| __t.clone())
}

pub(crate) fn toString(mut matching: &metamodelica::Ref<NBMatching>, mut r#str: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*StringUtil::headline_2(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("Scalar Matching"));
                ArcStr::from(__mm_s)
            }),
        )?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    };
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*toStringSingle(matching.var_to_eqn.clone(), false)?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    };
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*r#str);
        __mm_s.push_str(&*toStringSingle(matching.eqn_to_var.clone(), true)?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

pub(crate) fn trivial(mut n: i32) -> metamodelica::Ref<NBMatching> {
    let mut matching: metamodelica::Ref<NBMatching>;
    let mut arr: metamodelica::Array<i32> = Array::createIntRange(n);
    matching = metamodelica::Ref::new(NBMatching {
        var_to_eqn: arr.clone(),
        eqn_to_var: arr.clone(),
    });
    matching
}

pub(crate) fn regular(
    mut matching: metamodelica::Ref<NBMatching>,
    mut adj: &metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut transposed: bool,
    mut partially: bool,
    mut clear: bool,
) -> Result<metamodelica::Ref<NBMatching>> {
    let mut matching: metamodelica::Ref<NBMatching> = matching;
    let mut marked_eqns: metamodelica::List<metamodelica::List<i32>>;
    (matching, marked_eqns, _, _) = continue_(matching, adj, transposed, clear)?;
    if !(partially) && !((List::flatten(marked_eqns)?).is_empty()) {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NBMatching.regular"));
                __mm_s.push_str(&*literal!(" failed because the partition is structurally singular."));
                ArcStr::from(__mm_s)
            }],
        )?;
        return Err("fail");
    }
    Ok(matching)
}

pub(crate) const MAX_INDEX_REDUCTION_RESTARTS: i32 = 20;

pub(crate) fn singular(
    mut matching: metamodelica::Ref<NBMatching>,
    mut adj: metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut full: metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut vars: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut eqns: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut varData: metamodelica::Ref<VarData::VarData>,
    mut eqData: metamodelica::Ref<EqData::EqData>,
    mut kind: Partition::Kind,
    mut transposed: bool,
    mut clear: bool,
    mut restarts: i32,
) -> Result<(
    metamodelica::Ref<NBMatching>,
    metamodelica::Ref<Adjacency::Matrix::Matrix>,
    metamodelica::Ref<Adjacency::Matrix::Matrix>,
    metamodelica::Ref<VariablePointers::VariablePointers>,
    metamodelica::Ref<EquationPointers::EquationPointers>,
    metamodelica::Ref<VarData::VarData>,
    metamodelica::Ref<EqData::EqData>,
)> {
    let mut matching: metamodelica::Ref<NBMatching> = matching;
    let mut adj: metamodelica::Ref<Adjacency::Matrix::Matrix> = adj;
    let mut full: metamodelica::Ref<Adjacency::Matrix::Matrix> = full;
    let mut vars: metamodelica::Ref<VariablePointers::VariablePointers> = vars;
    let mut eqns: metamodelica::Ref<EquationPointers::EquationPointers> = eqns;
    let mut varData: metamodelica::Ref<VarData::VarData> = varData;
    let mut eqData: metamodelica::Ref<EqData::EqData> = eqData;
    let mut marked_eqns: metamodelica::List<metamodelica::List<i32>>;
    let mut mapping: Option<metamodelica::Ref<Adjacency::Mapping::Mapping>>;
    let mut matrixStrictness: Adjacency::MatrixStrictness;
    let mut changed: bool;
    if let Ok((__pa0, __pa1, __pa2, __pa3)) = continue_(matching.clone(), &adj, transposed, clear) {
        matching = metamodelica::Own::own(__pa0);
        marked_eqns = metamodelica::Own::own(__pa1);
        mapping = metamodelica::Own::own(__pa2);
        matrixStrictness = metamodelica::Own::own(__pa3);
    } else {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NBMatching.singular"));
                __mm_s.push_str(&*literal!(" failed to match partition:\n"));
                __mm_s.push_str(&*BVariable::VariablePointers::toString(
                    &vars,
                    literal!("partition vars"),
                    None,
                    true,
                )?);
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*EquationPointers::toString(
                    &eqns,
                    literal!("partition eqns"),
                    None,
                    true,
                    None,
                )?);
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*Adjacency::Matrix::toString(&adj, literal!(""))?);
                ArcStr::from(__mm_s)
            }],
        )?;
        return Err("fail");
    }
    if Partition::kindIsInitial(kind) {
        (adj, full, vars, eqns, varData, eqData, changed) = ResolveSingularities::balanceInitialization(
            adj,
            full,
            vars,
            eqns,
            varData,
            eqData,
            kind,
            funcMap.clone(),
            matching.clone(),
            mapping,
        )?;
    } else {
        (adj, full, vars, eqns, varData, eqData, changed) = ResolveSingularities::indexReduction(
            adj,
            full,
            vars,
            eqns,
            varData,
            eqData,
            kind,
            funcMap.clone(),
            matching.clone(),
            mapping,
        )?;
    }
    if changed {
        full = Adjacency::Matrix::createFull(&vars, &eqns, kind)?;
        adj = Adjacency::Matrix::fullToFinal(
            &full,
            vars.map.clone(),
            eqns.map.clone(),
            &(eqns.clone()),
            matrixStrictness,
            &(crate::NBEquation::Iterator::interned_EMPTY()),
        )?;
        if Partition::kindIsInitial(kind) {
            matching = regular(EMPTY_MATCHING().clone(), &adj, false, false, true)?;
        } else {
            if restarts >= MAX_INDEX_REDUCTION_RESTARTS.clone() {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBMatching.singular"));
                        __mm_s.push_str(&*literal!(" could not resolve the structural singularity after "));
                        __mm_s.push_str(&*intString(restarts));
                        __mm_s.push_str(&*literal!(
                            " index reduction steps. The system is probably over-determined or has a too high index."
                        ));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
            (matching, adj, full, vars, eqns, varData, eqData) = singular(
                EMPTY_MATCHING().clone(),
                adj,
                full,
                vars,
                eqns,
                funcMap,
                varData,
                eqData,
                kind,
                transposed,
                true,
                restarts + 1,
            )?;
        }
    }
    Ok((matching, adj, full, vars, eqns, varData, eqData))
}

pub(crate) fn fromSeed(
    mut seed: &metamodelica::Ref<Partition::Partition::Partition>,
    mut adj: &metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut vars: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
) -> Result<metamodelica::Ref<NBMatching>> {
    let mut matching: metamodelica::Ref<NBMatching> = EMPTY_MATCHING().clone();
    let mut seed_matching: metamodelica::Ref<NBMatching>;
    let mut seed_map: metamodelica::Ref<Adjacency::Mapping::Mapping>;
    let mut map: metamodelica::Ref<Adjacency::Mapping::Mapping>;
    let mut var_to_eqn: metamodelica::Array<i32>;
    let mut eqn_to_var: metamodelica::Array<i32>;
    let mut data: metamodelica::Array<i32>;
    let mut var_index: metamodelica::Array<i32>;
    let mut e: i32;
    let mut v: i32;
    let mut eqn: i32;
    let mut var: i32;
    let mut eqn_start: i32;
    let mut eqn_len: i32;
    let mut seed_start: i32;
    let mut seed_len: i32;
    let mut seed_var_start: i32;
    let mut var_start: i32;
    let mut var_len: i32;
    let mut offset: i32;
    let mut first: i32;
    if (seed.matching).is_none() || (seed.adjacencyMatrix).is_none() {
        return Ok(matching);
    }
    seed_matching = Util::getOption(seed.matching.clone())?;
    let () = (::match_deref::match_deref! { match &((Adjacency::Matrix::getMappingOpt(&(Util::getOption(seed.adjacencyMatrix.clone())?)), &**adj)) {
        (Some(__esc_seed_map), Deref @ Adjacency::Matrix::FINAL { mapping: __esc_map, .. }) => {
            seed_map = (*__esc_seed_map).clone();
            map = (*__esc_map).clone();
            var_index = arrayCreate(metamodelica::arrayLength(seed_map.var_AtS.clone()), -1);
            for mut i in 1..=metamodelica::arrayLength(var_index.clone()) {
                {
                    let __cell0 = BVariable::VariablePointers::getVarIndex(vars, BVariable::getVarName(BVariable::VariablePointers::getVarAt(&seed.unknowns, i)?))?;
                    let __idx0 = i;
                    *metamodelica::index_mut_checked(&mut var_index.clone().borrow_mut(), __idx0)? = __cell0;
                }
            }
            data = Adjacency::IntMatrix::entries(var_field!((**adj).m, Adjacency::Matrix::Matrix::FINAL));
            var_to_eqn = arrayCreate(metamodelica::arrayLength(map.var_StA.clone()), -1);
            eqn_to_var = arrayCreate(metamodelica::arrayLength(map.eqn_StA.clone()), -1);
            for mut i in 1..=metamodelica::arrayLength(seed_map.eqn_AtS.clone()) {
                e = EquationPointers::getEqnIndex(eqns, Equation::getEqnName(EquationPointers::getEqnAt(&seed.equations, i)?)?)?;
                if e < 1 {
                    continue;
                }
                (seed_start, seed_len) = ({let __elt = (*metamodelica::index_checked(&seed_map.eqn_AtS.borrow(), i)?).clone(); __elt});
                (eqn_start, eqn_len) = ({let __elt = (*metamodelica::index_checked(&map.eqn_AtS.borrow(), e)?).clone(); __elt});
                for mut k in 0..=std::cmp::min(seed_len, eqn_len) - 1 {
                    v = ({let __elt = (*metamodelica::index_checked(&seed_matching.eqn_to_var.borrow(), seed_start + k)?).clone(); __elt});
                    if v < 1 {
                        continue;
                    }
                    (seed_var_start, _) = ({let __elt = (*metamodelica::index_checked(&seed_map.var_AtS.borrow(), ({let __elt = (*metamodelica::index_checked(&seed_map.var_StA.borrow(), v)?).clone(); __elt}))?).clone(); __elt});
                    offset = v - seed_var_start;
                    v = ({let __elt = (*metamodelica::index_checked(&var_index.borrow(), ({let __elt = (*metamodelica::index_checked(&seed_map.var_StA.borrow(), v)?).clone(); __elt}))?).clone(); __elt});
                    if v < 1 {
                        continue;
                    }
                    (var_start, var_len) = ({let __elt = (*metamodelica::index_checked(&map.var_AtS.borrow(), v)?).clone(); __elt});
                    if offset >= var_len {
                        continue;
                    }
                    var = var_start + offset;
                    eqn = eqn_start + k;
                    first = ({let __elt = (*metamodelica::index_checked(&var_field!((**adj).m, Adjacency::Matrix::Matrix::FINAL).start.borrow(), eqn)?).clone(); __elt});
                    let __range1 = first..=first + ({let __elt = (*metamodelica::index_checked(&var_field!((**adj).m, Adjacency::Matrix::Matrix::FINAL).len.borrow(), eqn)?).clone(); __elt}) - 1;
                    for mut p in __range1 {
                        if ({let __elt = (*metamodelica::index_checked(&data.borrow(), p)?).clone(); __elt}) == var {
                            {
                                let __cell2 = var;
                                let __idx2 = eqn;
                                *metamodelica::index_mut_checked(&mut eqn_to_var.clone().borrow_mut(), __idx2)? = __cell2;
                            }
                            {
                                let __cell3 = eqn;
                                let __idx3 = var;
                                *metamodelica::index_mut_checked(&mut var_to_eqn.clone().borrow_mut(), __idx3)? = __cell3;
                            }
                            break;
                        }
                    }
                }
            }
            matching = metamodelica::Ref::new(NBMatching { var_to_eqn: var_to_eqn.clone(), eqn_to_var: eqn_to_var.clone() });
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matching)
}

pub(crate) fn continue_(
    mut matching: metamodelica::Ref<NBMatching>,
    mut adj: &metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut transposed: bool,
    mut clear: bool,
) -> Result<(
    metamodelica::Ref<NBMatching>,
    metamodelica::List<metamodelica::List<i32>>,
    Option<metamodelica::Ref<Adjacency::Mapping::Mapping>>,
    Adjacency::MatrixStrictness,
)> {
    let mut matching: metamodelica::Ref<NBMatching> = matching;
    let mut marked_eqns: metamodelica::List<metamodelica::List<i32>>;
    let mut mapping: Option<metamodelica::Ref<Adjacency::Mapping::Mapping>>;
    let mut matrixStrictness: Adjacency::MatrixStrictness;
    let mut var_to_eqn: metamodelica::Array<i32>;
    let mut eqn_to_var: metamodelica::Array<i32>;
    (matching, marked_eqns, mapping, matrixStrictness) = (match &**adj {
        Adjacency::Matrix::FINAL {
            m: __adj_m,
            mT: __adj_mT,
            mapping: __adj_mapping,
            st: __adj_st,
            ..
        } => {
            (var_to_eqn, eqn_to_var) = getAssignments(
                &matching,
                metamodelica::AsArg::as_arg(&__adj_m),
                metamodelica::AsArg::as_arg(&__adj_mT),
            )?;
            (var_to_eqn, eqn_to_var, marked_eqns) = PFPlusExternal(
                metamodelica::AsArg::as_arg(&__adj_m),
                var_to_eqn.clone(),
                eqn_to_var.clone(),
                clear,
            )?;
            matching = metamodelica::Ref::new(NBMatching {
                var_to_eqn: var_to_eqn.clone(),
                eqn_to_var: eqn_to_var.clone(),
            });
            (matching, marked_eqns, Some(__adj_mapping.clone()), __adj_st.clone())
        }
        Adjacency::Matrix::EMPTY { .. } => (
            EMPTY_MATCHING().clone(),
            metamodelica::nil(),
            None,
            Adjacency::MatrixStrictness::FULL.clone(),
        ),
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBMatching.continue_"));
                    __mm_s.push_str(&*literal!(" failed."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok((matching, marked_eqns, mapping, matrixStrictness))
}

pub(crate) fn isEmpty(mut matching: &metamodelica::Ref<NBMatching>) -> bool {
    let mut b: bool =
        matching.eqn_to_var.clone().borrow().is_empty() && matching.var_to_eqn.clone().borrow().is_empty();
    b
}

pub(crate) fn isPerfect(mut matching: &metamodelica::Ref<NBMatching>) -> Result<bool> {
    let mut b: bool;
    if metamodelica::arrayLength(matching.var_to_eqn.clone()) > metamodelica::arrayLength(matching.eqn_to_var.clone()) {
        b = Array::all(
            matching.eqn_to_var.clone(),
            &({
                let __pe_b1 = 0;
                move |__pe_a0| Ok(intGt(__pe_a0, __pe_b1.clone()))
            }),
        )?;
    } else {
        b = Array::all(
            matching.var_to_eqn.clone(),
            &({
                let __pe_b1 = 0;
                move |__pe_a0| Ok(intGt(__pe_a0, __pe_b1.clone()))
            }),
        )?;
    }
    Ok(b)
}

pub(crate) fn getAssignments(
    mut matching: &metamodelica::Ref<NBMatching>,
    mut m: &metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
    mut mT: &metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
) -> Result<(metamodelica::Array<i32>, metamodelica::Array<i32>)> {
    let mut var_to_eqn: metamodelica::Array<i32>;
    let mut eqn_to_var: metamodelica::Array<i32>;
    let mut nVars: i32 = Adjacency::IntMatrix::rows(mT);
    let mut nEqns: i32 = Adjacency::IntMatrix::rows(m);
    var_to_eqn = Array::expandToSize(nVars, matching.var_to_eqn.clone(), -1)?;
    eqn_to_var = Array::expandToSize(nEqns, matching.eqn_to_var.clone(), -1)?;
    Ok((var_to_eqn, eqn_to_var))
}

pub(crate) fn getMatches(
    mut matching: &metamodelica::Ref<NBMatching>,
    mut mapping_opt: Option<metamodelica::Ref<Adjacency::Mapping::Mapping>>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut equations: &metamodelica::Ref<EquationPointers::EquationPointers>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>>,
    metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>>,
    metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>>,
    metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>>,
)> {
    let mut matched_vars: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    > = metamodelica::nil();
    let mut unmatched_vars: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    > = metamodelica::nil();
    let mut matched_eqns: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    > = metamodelica::nil();
    let mut unmatched_eqns: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    > = metamodelica::nil();
    let mut mapping: metamodelica::Ref<Adjacency::Mapping::Mapping>;
    let mut var_map_matched: metamodelica::Ref<
        UnorderedMap::UnorderedMap<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>, metamodelica::List<i32>>,
    >;
    let mut var_map_unmatched: metamodelica::Ref<
        UnorderedMap::UnorderedMap<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>, metamodelica::List<i32>>,
    >;
    let mut eqn_map_matched: metamodelica::Ref<
        UnorderedMap::UnorderedMap<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>, metamodelica::List<i32>>,
    >;
    let mut eqn_map_unmatched: metamodelica::Ref<
        UnorderedMap::UnorderedMap<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>, metamodelica::List<i32>>,
    >;
    let mut arr_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut arr_eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut start_idx: i32;
    if (mapping_opt).is_some() {
        mapping = Util::getOption(mapping_opt)?;
        var_map_matched = UnorderedMap::new(
            (std::sync::Arc::new(BVariable::hash)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<i32>
                        + 'static,
                >),
            (std::sync::Arc::new(BVariable::equalName)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                            Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                        ) -> Result<bool>
                        + 'static,
                >),
            1,
        );
        var_map_unmatched = UnorderedMap::new(
            (std::sync::Arc::new(BVariable::hash)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<i32>
                        + 'static,
                >),
            (std::sync::Arc::new(BVariable::equalName)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                            Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                        ) -> Result<bool>
                        + 'static,
                >),
            1,
        );
        eqn_map_matched = UnorderedMap::new(
            (std::sync::Arc::new(Equation::hash)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<i32>
                        + 'static,
                >),
            (std::sync::Arc::new(Equation::equalName)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
                            Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
                        ) -> Result<bool>
                        + 'static,
                >),
            1,
        );
        eqn_map_unmatched = UnorderedMap::new(
            (std::sync::Arc::new(Equation::hash)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<i32>
                        + 'static,
                >),
            (std::sync::Arc::new(Equation::equalName)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
                            Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
                        ) -> Result<bool>
                        + 'static,
                >),
            1,
        );
        for mut var in 1..=metamodelica::arrayLength(matching.var_to_eqn.clone()) {
            arr_var = ExpandableArray::get(
                ({
                    let __elt = (*metamodelica::index_checked(&mapping.var_StA.borrow(), var)?).clone();
                    __elt
                }),
                variables.varArr.clone(),
            )?;
            (start_idx, _) = ({
                let __elt = (*metamodelica::index_checked(
                    &mapping.var_AtS.borrow(),
                    ({
                        let __elt = (*metamodelica::index_checked(&mapping.var_StA.borrow(), var)?).clone();
                        __elt
                    }),
                )?)
                .clone();
                __elt
            });
            if ({
                let __elt = (*metamodelica::index_checked(&matching.var_to_eqn.borrow(), var)?).clone();
                __elt
            }) > 0
            {
                Slice::addToSliceMap(arr_var, var - start_idx, var_map_matched.clone())?;
            } else {
                Slice::addToSliceMap(arr_var, var - start_idx, var_map_unmatched.clone())?;
            }
        }
        for mut eqn in 1..=metamodelica::arrayLength(matching.eqn_to_var.clone()) {
            arr_eqn = ExpandableArray::get(
                ({
                    let __elt = (*metamodelica::index_checked(&mapping.eqn_StA.borrow(), eqn)?).clone();
                    __elt
                }),
                equations.eqArr.clone(),
            )?;
            (start_idx, _) = ({
                let __elt = (*metamodelica::index_checked(
                    &mapping.eqn_AtS.borrow(),
                    ({
                        let __elt = (*metamodelica::index_checked(&mapping.eqn_StA.borrow(), eqn)?).clone();
                        __elt
                    }),
                )?)
                .clone();
                __elt
            });
            if ({
                let __elt = (*metamodelica::index_checked(&matching.eqn_to_var.borrow(), eqn)?).clone();
                __elt
            }) > 0
            {
                Slice::addToSliceMap(arr_eqn, eqn - start_idx, eqn_map_matched.clone())?;
            } else {
                Slice::addToSliceMap(arr_eqn, eqn - start_idx, eqn_map_unmatched.clone())?;
            }
        }
        matched_vars = ({
            let mut __acc: metamodelica::List<
                metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
            > = metamodelica::nil();
            for mut slice in (Slice::fromMap(var_map_matched)).into_iter().cloned() {
                let __x = Slice::simplify(
                    slice.clone(),
                    &({
                        let __pe_b1 = true;
                        move |__pe_a0| BVariable::size(__pe_a0, __pe_b1.clone())
                    }),
                )?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        unmatched_vars = ({
            let mut __acc: metamodelica::List<
                metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
            > = metamodelica::nil();
            for mut slice in (Slice::fromMap(var_map_unmatched)).into_iter().cloned() {
                let __x = Slice::simplify(
                    slice.clone(),
                    &({
                        let __pe_b1 = true;
                        move |__pe_a0| BVariable::size(__pe_a0, __pe_b1.clone())
                    }),
                )?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        matched_eqns = ({
            let mut __acc: metamodelica::List<
                metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
            > = metamodelica::nil();
            for mut slice in (Slice::fromMap(eqn_map_matched)).into_iter().cloned() {
                let __x = Slice::simplify(
                    slice.clone(),
                    &({
                        let __pe_b1 = true;
                        move |__pe_a0| Equation::size(__pe_a0, __pe_b1.clone())
                    }),
                )?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        unmatched_eqns = ({
            let mut __acc: metamodelica::List<
                metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
            > = metamodelica::nil();
            for mut slice in (Slice::fromMap(eqn_map_unmatched)).into_iter().cloned() {
                let __x = Slice::simplify(
                    slice.clone(),
                    &({
                        let __pe_b1 = true;
                        move |__pe_a0| Equation::size(__pe_a0, __pe_b1.clone())
                    }),
                )?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
    } else {
        for mut var in 1..=metamodelica::arrayLength(matching.var_to_eqn.clone()) {
            if ({
                let __elt = (*metamodelica::index_checked(&matching.var_to_eqn.borrow(), var)?).clone();
                __elt
            }) > 0
            {
                matched_vars = metamodelica::cons(
                    metamodelica::Ref::new(Slice::NBSlice {
                        t: ExpandableArray::get(var, variables.varArr.clone())?,
                        indices: metamodelica::nil(),
                    }),
                    matched_vars,
                );
            } else {
                unmatched_vars = metamodelica::cons(
                    metamodelica::Ref::new(Slice::NBSlice {
                        t: ExpandableArray::get(var, variables.varArr.clone())?,
                        indices: metamodelica::nil(),
                    }),
                    unmatched_vars,
                );
            }
        }
        for mut eqn in 1..=metamodelica::arrayLength(matching.eqn_to_var.clone()) {
            if ({
                let __elt = (*metamodelica::index_checked(&matching.eqn_to_var.borrow(), eqn)?).clone();
                __elt
            }) > 0
            {
                matched_eqns = metamodelica::cons(
                    metamodelica::Ref::new(Slice::NBSlice {
                        t: ExpandableArray::get(eqn, equations.eqArr.clone())?,
                        indices: metamodelica::nil(),
                    }),
                    matched_eqns,
                );
            } else {
                unmatched_eqns = metamodelica::cons(
                    metamodelica::Ref::new(Slice::NBSlice {
                        t: ExpandableArray::get(eqn, equations.eqArr.clone())?,
                        indices: metamodelica::nil(),
                    }),
                    unmatched_eqns,
                );
            }
        }
    }
    Ok((matched_vars, unmatched_vars, matched_eqns, unmatched_eqns))
}

pub(crate) fn getMatchedVars(
    mut matching: &metamodelica::Ref<NBMatching>,
    mut mapping_opt: Option<metamodelica::Ref<Adjacency::Mapping::Mapping>>,
    mut vars_map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
    let mut matched: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut start: i32;
    let mut size: i32;
    for mut arr_idx in &*UnorderedMap::valueList(vars_map) {
        (start, size) = (::match_deref::match_deref! { match &(&mapping_opt) {
            Some(mapping) => {
                ({let __elt = (*metamodelica::index_checked(&mapping.var_AtS.borrow(), arr_idx.clone())?).clone(); __elt})
            },
            _ => {
                (arr_idx.clone(), 1)
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        for mut scal_idx in start..=start + size - 1 {
            if scal_idx <= metamodelica::arrayLength(matching.var_to_eqn.clone())
                && ({
                    let __elt = (*metamodelica::index_checked(&matching.var_to_eqn.borrow(), scal_idx)?).clone();
                    __elt
                }) > 0
            {
                matched = metamodelica::cons(
                    ExpandableArray::get(arr_idx.clone(), variables.varArr.clone())?,
                    matched,
                );
                break;
            }
        }
    }
    Ok(matched)
}

fn toStringSingle(mut mapping: metamodelica::Array<i32>, mut inverse: bool) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut head: ArcStr = if (inverse) {
        literal!("equation to variable")
    } else {
        literal!("variable to equation")
    };
    let mut from: ArcStr = if (inverse) { literal!("eqn") } else { literal!("var") };
    let mut to: ArcStr = if (inverse) { literal!("var") } else { literal!("eqn") };
    r#str = StringUtil::headline_4(&head)?;
    for mut i in 1..=metamodelica::arrayLength(mapping.clone()) {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("\t"));
            __mm_s.push_str(&*from);
            __mm_s.push_str(&*literal!(" "));
            __mm_s.push_str(&*intString(i));
            __mm_s.push_str(&*literal!(" --> "));
            __mm_s.push_str(&*to);
            __mm_s.push_str(&*literal!(" "));
            __mm_s.push_str(&*intString(
                ({
                    let __elt = (*metamodelica::index_checked(&mapping.borrow(), i)?).clone();
                    __elt
                }),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
    }
    Ok(r#str)
}

fn scalarMatching(
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut transposed: bool,
    mut partially: bool,
) -> Result<(
    metamodelica::Ref<NBMatching>,
    metamodelica::List<metamodelica::List<i32>>,
)> {
    let mut matching: metamodelica::Ref<NBMatching>;
    let mut marked_eqns: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut nVars: i32 = metamodelica::arrayLength(mT.clone());
    let mut nEqns: i32 = metamodelica::arrayLength(m.clone());
    let mut var_to_eqn: metamodelica::Array<i32>;
    let mut eqn_to_var: metamodelica::Array<i32>;
    let mut var_marks: metamodelica::Array<bool> = arrayCreate(0, false);
    let mut eqn_marks: metamodelica::Array<bool> = arrayCreate(0, false);
    let mut pathFound: bool;
    var_to_eqn = arrayCreate(nVars, -1);
    for mut eqn in 1..=nEqns {
        var_marks = arrayCreate(nVars, false);
        eqn_marks = arrayCreate(nEqns, false);
        (var_to_eqn, var_marks, eqn_marks, pathFound) = augmentPath(
            eqn,
            m.clone(),
            mT.clone(),
            var_to_eqn.clone(),
            var_marks.clone(),
            eqn_marks.clone(),
        )?;
        if !(pathFound) {
            if !(partially) {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBMatching.scalarMatching"));
                        __mm_s.push_str(&*literal!(" failed because the partition is structurally singular. Index Reduction is not yet supported"));
                        ArcStr::from(__mm_s)
                    }],
                )?;
            } else if transposed {
                marked_eqns = metamodelica::cons(BackendUtil::findTrueIndices(var_marks.clone())?, marked_eqns);
            } else {
                marked_eqns = metamodelica::cons(BackendUtil::findTrueIndices(eqn_marks.clone())?, marked_eqns);
            }
        }
    }
    eqn_to_var = arrayCreate(nEqns, -1);
    for mut var in 1..=nVars {
        if ({
            let __elt = (*metamodelica::index_checked(&var_to_eqn.borrow(), var)?).clone();
            __elt
        }) > 0
        {
            {
                let __cell0 = var;
                let __idx0 = ({
                    let __elt = (*metamodelica::index_checked(&var_to_eqn.borrow(), var)?).clone();
                    __elt
                });
                *metamodelica::index_mut_checked(&mut eqn_to_var.clone().borrow_mut(), __idx0)? = __cell0;
            }
        }
    }
    if nEqns > 0 {
        GCExt::free(var_marks.clone());
        GCExt::free(eqn_marks.clone());
    }
    matching = if (transposed) {
        metamodelica::Ref::new(NBMatching {
            var_to_eqn: eqn_to_var.clone(),
            eqn_to_var: var_to_eqn.clone(),
        })
    } else {
        metamodelica::Ref::new(NBMatching {
            var_to_eqn: var_to_eqn.clone(),
            eqn_to_var: eqn_to_var.clone(),
        })
    };
    Ok((matching, marked_eqns))
}

fn augmentPath(
    mut eqn: i32,
    mut m: metamodelica::Array<metamodelica::List<i32>>,
    mut mT: metamodelica::Array<metamodelica::List<i32>>,
    mut var_to_eqn: metamodelica::Array<i32>,
    mut var_marks: metamodelica::Array<bool>,
    mut eqn_marks: metamodelica::Array<bool>,
) -> Result<(
    metamodelica::Array<i32>,
    metamodelica::Array<bool>,
    metamodelica::Array<bool>,
    bool,
)> {
    let mut var_to_eqn: metamodelica::Array<i32> = var_to_eqn;
    let mut var_marks: metamodelica::Array<bool> = var_marks;
    let mut eqn_marks: metamodelica::Array<bool> = eqn_marks;
    let mut pathFound: bool = false;
    {
        let __cell0 = true;
        let __idx0 = eqn;
        *metamodelica::index_mut_checked(&mut eqn_marks.clone().borrow_mut(), __idx0)? = __cell0;
    }
    let __range1 = &*({
        let __elt = (*metamodelica::index_checked(&m.borrow(), eqn)?).clone();
        __elt
    });
    for mut var in __range1 {
        if ({
            let __elt = (*metamodelica::index_checked(&var_to_eqn.borrow(), var.clone())?).clone();
            __elt
        }) <= 0
        {
            pathFound = true;
            {
                let __cell2 = eqn;
                let __idx2 = var.clone();
                *metamodelica::index_mut_checked(&mut var_to_eqn.clone().borrow_mut(), __idx2)? = __cell2;
            }
            return Ok((var_to_eqn, var_marks, eqn_marks, pathFound));
        }
    }
    let __range3 = &*({
        let __elt = (*metamodelica::index_checked(&m.borrow(), eqn)?).clone();
        __elt
    });
    for mut var in __range3 {
        if !({
            let __elt = (*metamodelica::index_checked(&var_marks.borrow(), var.clone())?).clone();
            __elt
        }) {
            {
                let __cell4 = true;
                let __idx4 = var.clone();
                *metamodelica::index_mut_checked(&mut var_marks.clone().borrow_mut(), __idx4)? = __cell4;
            }
            (var_to_eqn, var_marks, eqn_marks, pathFound) = augmentPath(
                ({
                    let __elt = (*metamodelica::index_checked(&var_to_eqn.borrow(), var.clone())?).clone();
                    __elt
                }),
                m.clone(),
                mT.clone(),
                var_to_eqn.clone(),
                var_marks.clone(),
                eqn_marks.clone(),
            )?;
            if pathFound {
                {
                    let __cell5 = eqn;
                    let __idx5 = var.clone();
                    *metamodelica::index_mut_checked(&mut var_to_eqn.clone().borrow_mut(), __idx5)? = __cell5;
                }
                return Ok((var_to_eqn, var_marks, eqn_marks, pathFound));
            }
        }
    }
    Ok((var_to_eqn, var_marks, eqn_marks, pathFound))
}

fn PFPlusExternal(
    mut m: &metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
    mut ass1: metamodelica::Array<i32>,
    mut ass2: metamodelica::Array<i32>,
    mut clear: bool,
) -> Result<(
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::List<metamodelica::List<i32>>,
)> {
    let mut ass1: metamodelica::Array<i32> = ass1;
    let mut ass2: metamodelica::Array<i32> = ass2;
    let mut marked_eqns: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut n1: i32 = metamodelica::arrayLength(ass1.clone());
    let mut n2: i32 = metamodelica::arrayLength(ass2.clone());
    let mut nonZero: i32 = Adjacency::IntMatrix::nonZeroCount(m);
    let mut cheap: i32 = 0;
    let mut algIndx: i32 = 5;
    BackendDAEEXT::setAssignment(n2, n1, ass2.clone(), ass1.clone());
    BackendDAEEXT::setAdjacencyMatrixFlat(
        n1,
        n2,
        nonZero,
        m.start.clone(),
        m.len.clone(),
        Vector::rawArray(m.data.clone()),
    );
    BackendDAEEXT::matching(
        n1,
        n2,
        algIndx,
        cheap,
        metamodelica::OrderedFloat(1.0_f64),
        if (clear) { 1 } else { 0 },
    );
    BackendDAEEXT::getAssignment(ass2.clone(), ass1.clone())?;
    Ok((ass1, ass2, marked_eqns))
}
