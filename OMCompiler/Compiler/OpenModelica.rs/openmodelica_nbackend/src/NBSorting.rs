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
use crate::NBAdjacency::Mode;
use crate::NBBackendUtil as BackendUtil;
use crate::NBEquation as BEquation;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointers;
use crate::NBMatching as Matching;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VariablePointers;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util::Vector;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// NB imports
// NF imports
// Util imports
// ############################################################
//                Pseudo Bucket Structures
// ############################################################
pub mod Value {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum Value {
        SINGLE_VAL {
            /// cref to solve for in this mode
            cref_to_solve: metamodelica::Ref<ComponentRef::NFComponentRef>,
            /// indices of all scalarized equations that have to be solved that way
            eqn_scal_indices: metamodelica::List<i32>,
        },
        MULTI_VAL {
            /// crefs to solve for in this mode
            crefs_to_solve: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            /// indices of all scalarized equations that have to be solved that way
            eqn_scal_indices: metamodelica::List<i32>,
        },
    }
    impl metamodelica::gc::MMTrace for Value {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                Value::SINGLE_VAL {
                    cref_to_solve,
                    eqn_scal_indices,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(cref_to_solve, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(eqn_scal_indices, __mmv)?;
                    Ok(())
                }
                Value::MULTI_VAL {
                    crefs_to_solve,
                    eqn_scal_indices,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(crefs_to_solve, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(eqn_scal_indices, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Default for Value {
        fn default() -> Self {
            Self::SINGLE_VAL {
                cref_to_solve: Default::default(),
                eqn_scal_indices: Default::default(),
            }
        }
    }
    pub use self::Value::{MULTI_VAL, SINGLE_VAL};
    pub(crate) fn toString(mut val: &metamodelica::Ref<Value>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = (match &**val {
            SINGLE_VAL {
                cref_to_solve: __val_cref_to_solve,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n\tval: ("));
                __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(
                    &__val_cref_to_solve,
                ))?);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
            MULTI_VAL {
                crefs_to_solve: __val_crefs_to_solve,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\n\tval: "));
                __mm_s.push_str(&*List::toString(
                    __val_crefs_to_solve.clone(),
                    &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                    List::Style::FLAT_CURLY.clone(),
                )?);
                ArcStr::from(__mm_s)
            }
        });
        Ok(r#str)
    }

    pub(crate) fn filter(
        mut val: metamodelica::Ref<Value>,
        mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>,
    ) -> Result<metamodelica::Ref<Value>> {
        let mut val: metamodelica::Ref<Value> = val;
        val = (match &*val {
            SINGLE_VAL {
                eqn_scal_indices: __val_eqn_scal_indices,
                ..
            } => {
                assign_variant_field!(val => Value::SINGLE_VAL; eqn_scal_indices = ({
                    let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                    for mut idx in (__val_eqn_scal_indices.clone()).into_iter().cloned() {
                        if !(!(UnorderedSet::contains(idx.clone(), set.clone())?)) { continue; }
                        let __x = idx.clone();
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }));
                val
            }
            MULTI_VAL {
                eqn_scal_indices: __val_eqn_scal_indices,
                ..
            } => {
                assign_variant_field!(val => Value::MULTI_VAL; eqn_scal_indices = ({
                    let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                    for mut idx in (__val_eqn_scal_indices.clone()).into_iter().cloned() {
                        if !(!(UnorderedSet::contains(idx.clone(), set.clone())?)) { continue; }
                        let __x = idx.clone();
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }));
                val
            }
        });
        Ok(val)
    }

    pub(crate) fn getEquations(mut val: &metamodelica::Ref<Value>) -> metamodelica::List<i32> {
        let mut eqn_scal_indices: metamodelica::List<i32>;
        eqn_scal_indices = (match &**val {
            SINGLE_VAL {
                eqn_scal_indices: __val_eqn_scal_indices,
                ..
            } => __val_eqn_scal_indices.clone(),
            MULTI_VAL {
                eqn_scal_indices: __val_eqn_scal_indices,
                ..
            } => __val_eqn_scal_indices.clone(),
        });
        eqn_scal_indices
    }
}

pub mod PseudoBucket {
    use super::*;
    // While collecting, a bucket accumulates its equations and crefs in pointers
    // so that adding one costs a single cons.
    pub type Bucket = (
        metamodelica::Ref<Mode::Mode>,
        bool,
        Pointer::Pointer<metamodelica::List<i32>>,
        Pointer::Pointer<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    );

    pub(crate) fn create(
        mut eqn_to_var: metamodelica::Array<i32>,
        mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
        mut mapping: &metamodelica::Ref<Adjacency::Mapping::Mapping>,
        mut m: &metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
        mut modes: metamodelica::Ref<Vector::Vector<metamodelica::Ref<Mode::Mode>>>,
    ) -> Result<metamodelica::List<(metamodelica::Ref<Mode::Mode>, metamodelica::Ref<Value::Value>)>> {
        let mut buckets: metamodelica::List<(metamodelica::Ref<Mode::Mode>, metamodelica::Ref<Value::Value>)> =
            metamodelica::nil();
        let mut data: metamodelica::Array<i32> = Adjacency::IntMatrix::entries(m);
        let mut ids: metamodelica::Array<i32> = Adjacency::IntMatrix::payload(m);
        let mut per_eqn: metamodelica::Array<
            metamodelica::List<(
                metamodelica::Ref<Mode::Mode>,
                bool,
                Pointer::Pointer<metamodelica::List<i32>>,
                Pointer::Pointer<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
            )>,
        > = arrayCreate(
            intMax(metamodelica::arrayLength(mapping.eqn_AtS.clone()), 1),
            metamodelica::nil(),
        );
        let mut order: metamodelica::List<(
            metamodelica::Ref<Mode::Mode>,
            bool,
            Pointer::Pointer<metamodelica::List<i32>>,
            Pointer::Pointer<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        )> = metamodelica::nil();
        let mut mode_opt: Option<metamodelica::Ref<Mode::Mode>>;
        let mut mode: metamodelica::Ref<Mode::Mode>;
        let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut eqn_arr_idx: i32;
        let mut multi: bool;
        let mut fresh: bool;
        let mut idx_ptr: Pointer::Pointer<metamodelica::List<i32>>;
        let mut cref_ptr: Pointer::Pointer<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
        let mut val: metamodelica::Ref<Value::Value>;
        for mut eqn_scal_idx in 1..=metamodelica::arrayLength(eqn_to_var.clone()) {
            mode_opt = Adjacency::Modes::get(
                modes.clone(),
                m,
                data.clone(),
                ids.clone(),
                eqn_scal_idx,
                ({
                    let __elt = (*metamodelica::index_checked(&eqn_to_var.borrow(), eqn_scal_idx)?).clone();
                    __elt
                }),
            )?;
            if (mode_opt).is_some() {
                mode = Util::getOption(mode_opt)?;
                eqn_arr_idx = ({
                    let __elt = (*metamodelica::index_checked(&mapping.eqn_StA.borrow(), eqn_scal_idx)?).clone();
                    __elt
                });
                multi = BEquation::Equation::isRecordOrTupleEquation(BEquation::EquationPointers::getEqnAt(
                    eqns,
                    eqn_arr_idx,
                )?)?;
                cref = openmodelica_nf_frontend::NFComponentRef::interned_EMPTY();
                if multi {
                    cref = (mode.crefs).head().cloned()?;
                    assign_field!(mode.crefs = metamodelica::nil());
                }
                (idx_ptr, cref_ptr, fresh) = getBucket(mode.clone(), multi, eqn_arr_idx, per_eqn.clone())?;
                if fresh {
                    order = metamodelica::cons((mode, multi, idx_ptr.clone(), cref_ptr), order);
                } else if multi {
                    Pointer::update(cref_ptr.clone(), metamodelica::cons(cref, Pointer::access(cref_ptr)));
                }
                Pointer::update(
                    idx_ptr.clone(),
                    metamodelica::cons(eqn_scal_idx, Pointer::access(idx_ptr)),
                );
            }
        }
        for mut bucket in &*order {
            (mode, multi, idx_ptr, cref_ptr) = bucket.clone();
            if multi {
                val = metamodelica::Ref::new(Value::Value::MULTI_VAL {
                    crefs_to_solve: Pointer::access(cref_ptr),
                    eqn_scal_indices: Pointer::access(idx_ptr),
                });
            } else {
                val = metamodelica::Ref::new(Value::Value::SINGLE_VAL {
                    cref_to_solve: (mode.crefs).head().cloned()?,
                    eqn_scal_indices: Pointer::access(idx_ptr),
                });
            }
            buckets = metamodelica::cons((mode, val), buckets);
        }
        if Flags::isSet(Flags::DUMP_SORTING.clone())? {
            for mut bucket_tpl in &*buckets {
                (mode, val) = bucket_tpl.clone();
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*Adjacency::Mode::toString(&mode)?);
                    __mm_s.push_str(&*Value::toString(&val)?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
        }
        Ok(buckets)
    }

    pub(crate) fn getBucket(
        mut mode: metamodelica::Ref<Mode::Mode>,
        mut multi: bool,
        mut eqn_arr_idx: i32,
        mut per_eqn: metamodelica::Array<
            metamodelica::List<(
                metamodelica::Ref<Mode::Mode>,
                bool,
                Pointer::Pointer<metamodelica::List<i32>>,
                Pointer::Pointer<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
            )>,
        >,
    ) -> Result<(
        Pointer::Pointer<metamodelica::List<i32>>,
        Pointer::Pointer<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        bool,
    )> {
        let mut idx_ptr: Pointer::Pointer<metamodelica::List<i32>>;
        let mut cref_ptr: Pointer::Pointer<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
        let mut fresh: bool = false;
        let mut m: metamodelica::Ref<Mode::Mode>;
        let mut mu: bool;
        let __range0 = &*({
            let __elt = (*metamodelica::index_checked(&per_eqn.borrow(), eqn_arr_idx)?).clone();
            __elt
        });
        for mut bucket in __range0 {
            (m, mu, idx_ptr, cref_ptr) = bucket.clone();
            if mu == multi && Adjacency::Mode::isEqual(&m, &mode)? {
                return Ok((idx_ptr, cref_ptr, fresh));
            }
        }
        idx_ptr = Pointer::create(metamodelica::nil());
        cref_ptr = Pointer::create(if (multi) {
            mode.crefs.clone()
        } else {
            metamodelica::nil()
        });
        fresh = true;
        metamodelica::arrayUpdate(
            per_eqn.clone(),
            eqn_arr_idx,
            metamodelica::cons(
                (mode, multi, idx_ptr.clone(), cref_ptr.clone()),
                ({
                    let __elt = (*metamodelica::index_checked(&per_eqn.borrow(), eqn_arr_idx)?).clone();
                    __elt
                }),
            ),
        )?;
        Ok((idx_ptr, cref_ptr, fresh))
    }

    pub(crate) fn filter(
        mut tpl: (metamodelica::Ref<Mode::Mode>, metamodelica::Ref<Value::Value>),
        mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>,
    ) -> Result<(metamodelica::Ref<Mode::Mode>, metamodelica::Ref<Value::Value>)> {
        let mut tpl: (metamodelica::Ref<Mode::Mode>, metamodelica::Ref<Value::Value>) = tpl;
        let mut mode: metamodelica::Ref<Mode::Mode>;
        let mut val: metamodelica::Ref<Value::Value>;
        (mode, val) = tpl;
        val = Value::filter(val, set)?;
        tpl = (mode, val);
        Ok(tpl)
    }

    pub(crate) fn relevant(mut tpl: &(metamodelica::Ref<Mode::Mode>, metamodelica::Ref<Value::Value>)) -> bool {
        let mut b: bool;
        let mut val: metamodelica::Ref<Value::Value>;
        (_, val) = tpl.clone();
        b = List::hasSeveralElements(&(Value::getEquations(&val)));
        b
    }
}

// ############################################################
//                      Main Functions
// ############################################################
pub(crate) fn tarjan(
    mut adj: metamodelica::Ref<Adjacency::Matrix::Matrix>,
    mut matching: metamodelica::Ref<Matching::NBMatching>,
    mut vars: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
) -> Result<metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>> {
    let mut comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> = metamodelica::nil();
    let mut mapping_opt: Option<metamodelica::Ref<Adjacency::Mapping::Mapping>>;
    let mut eqn_AtS: Option<metamodelica::Array<(i32, i32)>>;
    let mut var_AtS: Option<metamodelica::Array<(i32, i32)>>;
    match '__try0: {
        comps = (match &*adj {
            Adjacency::Matrix::FINAL {
                m: __adj_m,
                mapping: __adj_mapping,
                modes: __adj_modes,
                ..
            } => {
                let mut comps_indices: metamodelica::List<metamodelica::List<i32>>;
                let mut phase2_indices: metamodelica::List<metamodelica::List<i32>>;
                let mut phase2_adj: metamodelica::Ref<Adjacency::Matrix::Matrix>;
                let mut phase2_matching: metamodelica::Ref<Matching::NBMatching>;
                let mut super_nodes: metamodelica::Array<metamodelica::Ref<SuperNode::SuperNode>>;
                let mut var_loc: metamodelica::Array<i32>;
                let mut buckets: metamodelica::List<(metamodelica::Ref<Mode::Mode>, metamodelica::Ref<Value::Value>)>;
                if unwrap_break_err!(Flags::isSet(Flags::DUMP_SORTING.clone()), '__try0) {
                    metamodelica::print(unwrap_break_err!(StringUtil::headline_1(&(literal!("Sorting"))), '__try0));
                }
                buckets = unwrap_break_err!(PseudoBucket::create(matching.eqn_to_var.clone(), eqns, metamodelica::AsArg::as_arg(&__adj_mapping), metamodelica::AsArg::as_arg(&__adj_m), __adj_modes.clone()), '__try0);
                comps_indices =
                    unwrap_break_err!(tarjanScalar(metamodelica::AsArg::as_arg(&__adj_m), &matching), '__try0);
                (phase2_adj, phase2_matching, super_nodes) = unwrap_break_err!(SuperNode::create(adj.clone(), metamodelica::AsArg::as_arg(&__adj_mapping), matching.clone(), eqns.map.clone(), comps_indices.clone(), buckets.clone()), '__try0);
                let () = (match &*phase2_adj {
                    Adjacency::Matrix::FINAL { m: __phase2_adj_m, .. } => {
                        phase2_indices = unwrap_break_err!(tarjanScalar(metamodelica::AsArg::as_arg(&__phase2_adj_m), &phase2_matching), '__try0);
                        var_loc = arrayCreate(metamodelica::arrayLength(matching.var_to_eqn.clone()), 0);
                        comps = ({
                            let mut __acc: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> =
                                metamodelica::nil();
                            for mut comp in (phase2_indices.clone()).into_iter().cloned() {
                                let __x = unwrap_break_err!(SuperNode::collapse(comp.clone(), super_nodes.clone(), metamodelica::AsArg::as_arg(&__adj_m), metamodelica::AsArg::as_arg(&__adj_mapping), &matching, vars, eqns, var_loc.clone()), '__try0);
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        });
                        GCExt::free(var_loc.clone());
                        ()
                    }
                    _ => {
                        unwrap_break_err!(Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSorting.tarjan")); __mm_s.push_str(&*literal!(" failed because of unknown adjacency matrix or matching type.")); ArcStr::from(__mm_s) }]), '__try0);
                        break '__try0 Err::<_, _>("fail");
                    }
                });
                comps.clone()
            }
            Adjacency::Matrix::EMPTY { .. } => metamodelica::nil(),
            _ => {
                unwrap_break_err!(Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSorting.tarjan")); __mm_s.push_str(&*literal!(" failed because adjacency matrix has unknown type.")); ArcStr::from(__mm_s) }]), '__try0);
                break '__try0 Err::<_, _>("fail");
            }
        });
        Ok::<_, &'static str>((comps.clone(),))
    } {
        Ok((__try0_o0,)) => {
            comps = __try0_o0;
        }
        Err(__try0_err) => {
            mapping_opt = Adjacency::Matrix::getMappingOpt(&adj);
            (eqn_AtS, var_AtS) = (::match_deref::match_deref! { match &(&mapping_opt) {
                Some(mapping) => {
                    (Some(mapping.eqn_AtS.clone()), Some(mapping.var_AtS.clone()))
                },
                _ => {
                    (None, None)
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBSorting.tarjan"));
                    __mm_s.push_str(&*literal!(" failed to sort system:\n"));
                    __mm_s.push_str(&*BVariable::VariablePointers::toString(
                        vars,
                        literal!("System"),
                        var_AtS.clone(),
                        true,
                    )?);
                    __mm_s.push_str(&*literal!("\n"));
                    __mm_s.push_str(&*BEquation::EquationPointers::toString(
                        eqns,
                        literal!("System"),
                        eqn_AtS.clone(),
                        true,
                        None,
                    )?);
                    __mm_s.push_str(&*literal!("\n"));
                    __mm_s.push_str(&*Matching::toString(&matching, literal!(""))?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err(__try0_err);
        }
    }
    Ok(comps)
}

pub(crate) fn tarjanScalar(
    mut m: &metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
    mut matching: &metamodelica::Ref<Matching::NBMatching>,
) -> Result<metamodelica::List<metamodelica::List<i32>>> {
    let mut comps: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
    let mut index: i32 = 0;
    let mut stack: metamodelica::List<i32> = metamodelica::nil();
    let mut number: metamodelica::Array<i32>;
    let mut lowlink: metamodelica::Array<i32>;
    let mut onStack: metamodelica::Array<bool>;
    let mut data: metamodelica::Array<i32> = Adjacency::IntMatrix::entries(m);
    let mut N: i32 = metamodelica::arrayLength(matching.var_to_eqn.clone());
    let mut M: i32 = metamodelica::arrayLength(matching.eqn_to_var.clone());
    let mut call_eqn: metamodelica::Array<i32>;
    let mut call_pos: metamodelica::Array<i32>;
    let mut eqn: i32;
    number = arrayCreate(M, -1);
    lowlink = arrayCreate(M, -1);
    onStack = arrayCreate(M, false);
    call_eqn = arrayCreate(M, 0);
    call_pos = arrayCreate(M, 0);
    for mut var in 1..=N {
        eqn = ({
            let __elt = (*metamodelica::index_checked(&matching.var_to_eqn.borrow(), var)?).clone();
            __elt
        });
        if eqn > 0
            && ({
                let __elt = (*metamodelica::index_checked(&number.borrow(), eqn)?).clone();
                __elt
            }) == -1
        {
            (stack, index, comps) = strongConnect(
                m,
                data.clone(),
                matching.var_to_eqn.clone(),
                eqn,
                stack,
                index,
                number.clone(),
                lowlink.clone(),
                onStack.clone(),
                call_eqn.clone(),
                call_pos.clone(),
                comps,
            )?;
        }
    }
    GCExt::free(number.clone());
    GCExt::free(lowlink.clone());
    GCExt::free(onStack.clone());
    GCExt::free(call_eqn.clone());
    GCExt::free(call_pos.clone());
    comps = comps.reverse();
    Ok(comps)
}

pub type SCC = metamodelica::List<i32>;

pub mod LoopIdentifier {
    use super::*;
    /// used to identify algebraic loops that are structurally equal just differ in local indexing
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct LoopIdentifier {
        pub eqns: metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>,
        pub vars: metamodelica::Ref<UnorderedSet::UnorderedSet<i32>>,
    }

    impl metamodelica::gc::MMTrace for LoopIdentifier {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.eqns, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.vars, __mmv)?;
            Ok(())
        }
    }
    impl Default for LoopIdentifier {
        fn default() -> Self {
            Self {
                eqns: Default::default(),
                vars: Default::default(),
            }
        }
    }

    pub type LOOP_IDENTIFIER = LoopIdentifier;

    pub(crate) fn hash(mut li: &metamodelica::Ref<LoopIdentifier>) -> Result<i32> {
        let mut i: i32 = stringHashDjb2(&(toString(li)?));
        Ok(i)
    }

    pub(crate) fn isEqual(
        mut li1: &metamodelica::Ref<LoopIdentifier>,
        mut li2: &metamodelica::Ref<LoopIdentifier>,
    ) -> Result<bool> {
        let mut b: bool = UnorderedSet::isEqual(li1.eqns.clone(), li2.eqns.clone())?
            && UnorderedSet::isEqual(li1.vars.clone(), li2.vars.clone())?;
        Ok(b)
    }

    pub(crate) fn toString(mut li: &metamodelica::Ref<LoopIdentifier>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(" eqns: "));
            __mm_s.push_str(&*UnorderedSet::toString(
                li.eqns.clone(),
                &fnptr!(intString, i32),
                literal!("\n"),
            )?);
            __mm_s.push_str(&*literal!("\n vars:"));
            __mm_s.push_str(&*UnorderedSet::toString(
                li.vars.clone(),
                &fnptr!(intString, i32),
                literal!("\n"),
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn fromSCC(
        mut scc: metamodelica::List<i32>,
        mut mapping: &metamodelica::Ref<Adjacency::Mapping::Mapping>,
        mut matching: &metamodelica::Ref<Matching::NBMatching>,
    ) -> Result<metamodelica::Ref<LoopIdentifier>> {
        let mut li: metamodelica::Ref<LoopIdentifier>;
        li = metamodelica::Ref::new(LoopIdentifier {
            eqns: UnorderedSet::fromList(
                &({
                    let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                    for mut i in (scc.clone()).into_iter().cloned() {
                        let __x = ({
                            let __elt = (*metamodelica::index_checked(&mapping.eqn_StA.borrow(), i.clone())?).clone();
                            __elt
                        });
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                std::sync::Arc::new(fnptr!(Util::id, _)),
                (std::sync::Arc::new(fnptr!(intEq, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?,
            vars: UnorderedSet::fromList(
                &({
                    let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                    for mut i in (scc).into_iter().cloned() {
                        let __x = ({
                            let __elt = (*metamodelica::index_checked(
                                &mapping.var_StA.borrow(),
                                ({
                                    let __elt =
                                        (*metamodelica::index_checked(&matching.eqn_to_var.borrow(), i.clone())?)
                                            .clone();
                                    __elt
                                }),
                            )?)
                            .clone();
                            __elt
                        });
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                std::sync::Arc::new(fnptr!(Util::id, _)),
                (std::sync::Arc::new(fnptr!(intEq, i32, i32))
                    as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            )?,
        });
        Ok(li)
    }
}

pub mod SuperNode {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum SuperNode {
        /// does not belong to an algebraic loop or array
        SINGLE { index: i32 },
        /// is part of either an algebraic loop or array
        ELEMENT { index: i32, parent: i32 },
        /// an algebraic loop of equations
        ALGEBRAIC_LOOP {
            index: i32,
            eqn_indices: metamodelica::List<i32>,
        },
        /// a bucket of array equations solved for the same cref
        ARRAY_BUCKET {
            index: i32,
            cref_to_solve: metamodelica::Ref<ComponentRef::NFComponentRef>,
            eqn_indices: metamodelica::List<i32>,
            arr_idx: i32,
        },
    }
    impl metamodelica::gc::MMTrace for SuperNode {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                SuperNode::SINGLE { index } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    Ok(())
                }
                SuperNode::ELEMENT { index, parent } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(parent, __mmv)?;
                    Ok(())
                }
                SuperNode::ALGEBRAIC_LOOP { index, eqn_indices } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(eqn_indices, __mmv)?;
                    Ok(())
                }
                SuperNode::ARRAY_BUCKET {
                    index,
                    cref_to_solve,
                    eqn_indices,
                    arr_idx,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(cref_to_solve, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(eqn_indices, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(arr_idx, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    pub use self::SuperNode::{ALGEBRAIC_LOOP, ARRAY_BUCKET, ELEMENT, SINGLE};
    pub(crate) fn toString(mut node: &metamodelica::Ref<SuperNode>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = (match &**node {
            SINGLE { index: __node_index } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("["));
                __mm_s.push_str(&*intString(__node_index.clone() + 1));
                __mm_s.push_str(&*literal!("] single "));
                ArcStr::from(__mm_s)
            }
            ELEMENT {
                index: __node_index,
                parent: __node_parent,
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("["));
                __mm_s.push_str(&*intString(__node_index.clone() + 1));
                __mm_s.push_str(&*literal!("] scalar element of ("));
                __mm_s.push_str(&*intString(__node_parent.clone() + 1));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
            ALGEBRAIC_LOOP {
                eqn_indices: __node_eqn_indices,
                index: __node_index,
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("["));
                __mm_s.push_str(&*intString(__node_index.clone() + 1));
                __mm_s.push_str(&*literal!("] algebraic loop "));
                __mm_s.push_str(&*List::toString(
                    ({
                        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                        for mut i in (__node_eqn_indices.clone()).into_iter().cloned() {
                            let __x = i.clone() + 1;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    &fnptr!(intString, i32),
                    List::Style::FLAT_CURLY.clone(),
                )?);
                ArcStr::from(__mm_s)
            }
            ARRAY_BUCKET {
                eqn_indices: __node_eqn_indices,
                index: __node_index,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("["));
                __mm_s.push_str(&*intString(__node_index.clone() + 1));
                __mm_s.push_str(&*literal!("] array bucket "));
                __mm_s.push_str(&*List::toString(
                    ({
                        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                        for mut i in (__node_eqn_indices.clone()).into_iter().cloned() {
                            let __x = i.clone() + 1;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    &fnptr!(intString, i32),
                    List::Style::FLAT_CURLY.clone(),
                )?);
                ArcStr::from(__mm_s)
            }
            _ => literal!("ERROR"),
        });
        Ok(r#str)
    }

    pub(crate) fn isArrayBucket(mut node: &metamodelica::Ref<SuperNode>) -> bool {
        let mut b: bool;
        b = (match &**node {
            ARRAY_BUCKET { .. } => true,
            _ => false,
        });
        b
    }

    pub(crate) fn getEqnIndices(mut node: &metamodelica::Ref<SuperNode>) -> Result<metamodelica::List<i32>> {
        let mut eqn_indices: metamodelica::List<i32>;
        eqn_indices = (match &**node {
            SINGLE { index: __node_index } => list![__node_index.clone()],
            ALGEBRAIC_LOOP {
                eqn_indices: __node_eqn_indices,
                ..
            } => __node_eqn_indices.clone(),
            ARRAY_BUCKET {
                eqn_indices: __node_eqn_indices,
                ..
            } => __node_eqn_indices.clone(),
            ELEMENT { .. } => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBSorting.SuperNode.getEqnIndices"));
                        __mm_s.push_str(&*literal!(
                            " failed because elements should not be accessed, only their parents: "
                        ));
                        __mm_s.push_str(&*toString(node)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBSorting.SuperNode.getEqnIndices"));
                        __mm_s.push_str(&*literal!(" failed because of incorrect super node type."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(eqn_indices)
    }

    pub(crate) fn create(
        mut adj: metamodelica::Ref<Adjacency::Matrix::Matrix>,
        mut mapping: &metamodelica::Ref<Adjacency::Mapping::Mapping>,
        mut matching: metamodelica::Ref<Matching::NBMatching>,
        mut eqn_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>,
        >,
        mut scc_phase1: metamodelica::List<metamodelica::List<i32>>,
        mut buck: metamodelica::List<(metamodelica::Ref<Mode::Mode>, metamodelica::Ref<Value::Value>)>,
    ) -> Result<(
        metamodelica::Ref<Adjacency::Matrix::Matrix>,
        metamodelica::Ref<Matching::NBMatching>,
        metamodelica::Array<metamodelica::Ref<SuperNode>>,
    )> {
        let mut phase2_adj: metamodelica::Ref<Adjacency::Matrix::Matrix> = adj;
        let mut phase2_matching: metamodelica::Ref<Matching::NBMatching> = matching.clone();
        let mut super_nodes: metamodelica::Array<metamodelica::Ref<SuperNode>> = Default::default();
        let mut li: metamodelica::Ref<LoopIdentifier::LoopIdentifier>;
        let mut loop_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<LoopIdentifier::LoopIdentifier>, metamodelica::List<i32>>,
        > = UnorderedMap::new(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<LoopIdentifier::LoopIdentifier>| {
                LoopIdentifier::hash(&__a0)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<LoopIdentifier::LoopIdentifier>) -> Result<i32> + 'static,
                >),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<LoopIdentifier::LoopIdentifier>,
                      __a1: metamodelica::Ref<LoopIdentifier::LoopIdentifier>| {
                    LoopIdentifier::isEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<LoopIdentifier::LoopIdentifier>,
                            metamodelica::Ref<LoopIdentifier::LoopIdentifier>,
                        ) -> Result<bool>
                        + 'static,
                >),
            1,
        );
        let mut algebraic_loops: metamodelica::List<metamodelica::List<i32>> = ({
            let mut __acc: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
            for mut scc in (scc_phase1.clone()).into_iter().cloned() {
                if !(List::hasSeveralElements(&(scc.clone()))) {
                    continue;
                }
                let __x = scc.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        let mut buckets: metamodelica::List<(metamodelica::Ref<Mode::Mode>, metamodelica::Ref<Value::Value>)> = buck;
        let mut mode: metamodelica::Ref<Mode::Mode>;
        let mut val: metamodelica::Ref<Value::Value>;
        let mut index: i32;
        let mut shift: i32;
        let mut var_lst: metamodelica::List<i32>;
        let mut eqn_lst: metamodelica::List<i32>;
        let mut eqn_rows: metamodelica::List<metamodelica::List<i32>>;
        let mut var_rows: metamodelica::List<metamodelica::List<i32>>;
        let mut rest_var_rows: metamodelica::List<metamodelica::List<i32>>;
        let mut alg_loop_set: metamodelica::Ref<UnorderedSet::UnorderedSet<i32>> = UnorderedSet::new(
            std::sync::Arc::new(fnptr!(Util::id, _)),
            (std::sync::Arc::new(fnptr!(intEq, i32, i32))
                as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
            13,
        );
        let mut stamp: metamodelica::Array<i32>;
        let mut counts: metamodelica::Array<i32>;
        let mut uniq: metamodelica::Array<i32>;
        let mut sorted: metamodelica::Array<i32>;
        let mut mx: i32;
        phase2_adj = (match &*phase2_adj {
            Adjacency::Matrix::FINAL {
                m: __phase2_adj_m,
                mT: __phase2_adj_mT,
                ..
            } => {
                for mut scc in &*algebraic_loops {
                    li = LoopIdentifier::fromSCC(scc.clone(), mapping, &matching)?;
                    UnorderedMap::add(
                        li.clone(),
                        listAppend(
                            scc.clone(),
                            UnorderedMap::getOrDefault(li, loop_map.clone(), metamodelica::nil())?,
                        ),
                        loop_map.clone(),
                    )?;
                }
                algebraic_loops = UnorderedMap::valueList(loop_map);
                for mut scc in &*algebraic_loops {
                    for mut idx in &*scc.clone() {
                        UnorderedSet::add(idx.clone(), alg_loop_set.clone())?;
                    }
                }
                buckets = ({
                    let mut __acc: metamodelica::List<(
                        metamodelica::Ref<Mode::Mode>,
                        metamodelica::Ref<Value::Value>,
                    )> = metamodelica::nil();
                    for mut bucket_tpl in (buckets).into_iter().cloned() {
                        let __x = PseudoBucket::filter(bucket_tpl.clone(), alg_loop_set.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                buckets = ({
                    let mut __acc: metamodelica::List<(
                        metamodelica::Ref<Mode::Mode>,
                        metamodelica::Ref<Value::Value>,
                    )> = metamodelica::nil();
                    for mut bucket_tpl in (buckets).into_iter().cloned() {
                        if !(PseudoBucket::relevant(&(bucket_tpl.clone()))) {
                            continue;
                        }
                        let __x = bucket_tpl.clone();
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                shift = ((algebraic_loops).len() as i32) + ((buckets).len() as i32);
                super_nodes = arrayCreate(
                    Adjacency::IntMatrix::rows(metamodelica::AsArg::as_arg(&__phase2_adj_m)) + shift,
                    metamodelica::Ref::new(SuperNode::SINGLE { index: 0 }),
                );
                for mut i in 1..=metamodelica::arrayLength(super_nodes.clone()) {
                    metamodelica::arrayUpdate(
                        super_nodes.clone(),
                        i,
                        metamodelica::Ref::new(SuperNode::SINGLE { index: i }),
                    )?;
                }
                index = metamodelica::arrayLength(phase2_matching.eqn_to_var.clone());
                assign_field!(
                    phase2_matching.eqn_to_var = Array::expandToSize(
                        metamodelica::arrayLength(phase2_matching.eqn_to_var.clone()) + shift,
                        phase2_matching.eqn_to_var.clone(),
                        -1
                    )?
                );
                for mut i in index + 1..=index + shift {
                    {
                        let __cell0 = i;
                        let __idx0 = i;
                        *metamodelica::index_mut_checked(
                            &mut phase2_matching.eqn_to_var.clone().borrow_mut(),
                            __idx0,
                        )? = __cell0;
                    }
                }
                index = metamodelica::arrayLength(phase2_matching.var_to_eqn.clone());
                assign_field!(
                    phase2_matching.var_to_eqn = Array::expandToSize(
                        metamodelica::arrayLength(phase2_matching.var_to_eqn.clone()) + shift,
                        phase2_matching.var_to_eqn.clone(),
                        -1
                    )?
                );
                for mut i in index + 1..=index + shift {
                    {
                        let __cell1 = i;
                        let __idx1 = i;
                        *metamodelica::index_mut_checked(
                            &mut phase2_matching.var_to_eqn.clone().borrow_mut(),
                            __idx1,
                        )? = __cell1;
                    }
                }
                index = Adjacency::IntMatrix::rows(metamodelica::AsArg::as_arg(&__phase2_adj_mT)) + 1;
                stamp = arrayCreate(
                    intMax(
                        Adjacency::IntMatrix::rows(metamodelica::AsArg::as_arg(&__phase2_adj_m)),
                        Adjacency::IntMatrix::rows(metamodelica::AsArg::as_arg(&__phase2_adj_mT)),
                    ) + shift,
                    0,
                );
                assign_variant_field!(phase2_adj => Adjacency::Matrix::Matrix::FINAL; mT = Adjacency::IntMatrix::expandRows(__phase2_adj_mT.clone(), shift)?);
                eqn_rows = listAppend(
                    algebraic_loops.clone(),
                    ({
                        let mut __acc: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
                        for mut bucket in (buckets.clone()).into_iter().cloned() {
                            let __x = Value::getEquations(&(Util::tuple22(bucket.clone())));
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                );
                var_rows = ({
                    let mut __acc: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
                    for mut row in (eqn_rows.clone()).into_iter().cloned() {
                        let __x = ({
                            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                            for mut idx in (row.clone()).into_iter().cloned() {
                                let __x = ({
                                    let __elt = (*metamodelica::index_checked(
                                        &phase2_matching.eqn_to_var.borrow(),
                                        idx.clone(),
                                    )?)
                                    .clone();
                                    __elt
                                });
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        });
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                Adjacency::IntMatrix::reserveData(
                    &(var_field!((*phase2_adj).mT, Adjacency::Matrix::Matrix::FINAL).clone()),
                    mergedSize(
                        var_field!((*phase2_adj).mT, Adjacency::Matrix::Matrix::FINAL),
                        &var_rows,
                    )?,
                    false,
                );
                mx = maxMergedRow(
                    var_field!((*phase2_adj).mT, Adjacency::Matrix::Matrix::FINAL),
                    &var_rows,
                )?;
                counts = arrayCreate(Util::nextPrime(mx), 0);
                uniq = arrayCreate(mx, 0);
                sorted = arrayCreate(mx, 0);
                rest_var_rows = var_rows;
                for mut scc in &*algebraic_loops {
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_var_rows) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    var_lst = metamodelica::Own::own(__pa2);
                    rest_var_rows = metamodelica::Own::own(__pa3);
                    mergeLoopNodes(super_nodes.clone(), var_lst.clone(), index, false)?;
                    index = mergeRows(
                        var_field!((*phase2_adj).mT, Adjacency::Matrix::Matrix::FINAL),
                        phase2_matching.var_to_eqn.clone(),
                        super_nodes.clone(),
                        &var_lst,
                        index,
                        stamp.clone(),
                        counts.clone(),
                        uniq.clone(),
                        sorted.clone(),
                    )?;
                }
                for mut bucket in &*buckets {
                    (mode, val) = bucket.clone();
                    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(rest_var_rows) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    var_lst = metamodelica::Own::own(__pa4);
                    rest_var_rows = metamodelica::Own::own(__pa5);
                    let () = (match &*val {
                        Value::SINGLE_VAL {
                            cref_to_solve: __val_cref_to_solve,
                            ..
                        } => {
                            mergeArrayNodes(
                                super_nodes.clone(),
                                __val_cref_to_solve.clone(),
                                var_lst.clone(),
                                index,
                                UnorderedMap::getSafe(
                                    mode.eqn_name.clone(),
                                    eqn_map.clone(),
                                    metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBSorting.mo"),
                                )?,
                                false,
                            )?;
                            ()
                        }
                        Value::MULTI_VAL { .. } => {
                            mergeLoopNodes(super_nodes.clone(), var_lst.clone(), index, false)?;
                            ()
                        }
                    });
                    index = mergeRows(
                        var_field!((*phase2_adj).mT, Adjacency::Matrix::Matrix::FINAL),
                        phase2_matching.var_to_eqn.clone(),
                        super_nodes.clone(),
                        &var_lst,
                        index,
                        stamp.clone(),
                        counts.clone(),
                        uniq.clone(),
                        sorted.clone(),
                    )?;
                }
                index = Adjacency::IntMatrix::rows(var_field!((*phase2_adj).m, Adjacency::Matrix::Matrix::FINAL)) + 1;
                assign_variant_field!(phase2_adj => Adjacency::Matrix::Matrix::FINAL; m = Adjacency::IntMatrix::transpose(&(var_field!((*phase2_adj).mT, Adjacency::Matrix::Matrix::FINAL).clone()), Adjacency::IntMatrix::rows(var_field!((*phase2_adj).m, Adjacency::Matrix::Matrix::FINAL)) + shift, mergedSize(var_field!((*phase2_adj).m, Adjacency::Matrix::Matrix::FINAL), &eqn_rows)?)?);
                mx = maxMergedRow(var_field!((*phase2_adj).m, Adjacency::Matrix::Matrix::FINAL), &eqn_rows)?;
                counts = arrayCreate(Util::nextPrime(mx), 0);
                uniq = arrayCreate(mx, 0);
                sorted = arrayCreate(mx, 0);
                for mut scc in &*algebraic_loops {
                    mergeLoopNodes(super_nodes.clone(), scc.clone(), index, true)?;
                    index = mergeRows(
                        var_field!((*phase2_adj).m, Adjacency::Matrix::Matrix::FINAL),
                        phase2_matching.eqn_to_var.clone(),
                        super_nodes.clone(),
                        metamodelica::AsArg::as_arg(&scc),
                        index,
                        stamp.clone(),
                        counts.clone(),
                        uniq.clone(),
                        sorted.clone(),
                    )?;
                }
                for mut bucket in &*buckets {
                    (mode, val) = bucket.clone();
                    eqn_lst = Value::getEquations(&val);
                    let () = (match &*val {
                        Value::SINGLE_VAL {
                            cref_to_solve: __val_cref_to_solve,
                            ..
                        } => {
                            mergeArrayNodes(
                                super_nodes.clone(),
                                __val_cref_to_solve.clone(),
                                eqn_lst.clone(),
                                index,
                                UnorderedMap::getSafe(
                                    mode.eqn_name.clone(),
                                    eqn_map.clone(),
                                    metamodelica::sourceInfo!("NBackEnd/Modules/1_Main/NBSorting.mo"),
                                )?,
                                true,
                            )?;
                            ()
                        }
                        Value::MULTI_VAL { .. } => {
                            mergeLoopNodes(super_nodes.clone(), eqn_lst.clone(), index, true)?;
                            ()
                        }
                    });
                    index = mergeRows(
                        var_field!((*phase2_adj).m, Adjacency::Matrix::Matrix::FINAL),
                        phase2_matching.eqn_to_var.clone(),
                        super_nodes.clone(),
                        &eqn_lst,
                        index,
                        stamp.clone(),
                        counts.clone(),
                        uniq.clone(),
                        sorted.clone(),
                    )?;
                }
                phase2_adj
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBSorting.SuperNode.create"));
                        __mm_s.push_str(&*literal!(" failed because of unknown adjacency matrix type."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok((phase2_adj, phase2_matching, super_nodes))
    }

    pub(crate) fn collapse(
        mut comp_indices: metamodelica::List<i32>,
        mut super_nodes: metamodelica::Array<metamodelica::Ref<SuperNode>>,
        mut m: &metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
        mut mapping: &metamodelica::Ref<Adjacency::Mapping::Mapping>,
        mut matching: &metamodelica::Ref<Matching::NBMatching>,
        mut vars: &metamodelica::Ref<VariablePointers::VariablePointers>,
        mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
        mut var_loc: metamodelica::Array<i32>,
    ) -> Result<metamodelica::Ref<StrongComponent::NBStrongComponent>> {
        let __ab_super_nodes = super_nodes.borrow();
        let mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent>;
        let mut node_comp: metamodelica::List<metamodelica::Ref<SuperNode>> = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<SuperNode>> = metamodelica::nil();
            for mut i in (comp_indices.clone()).into_iter().cloned() {
                let __x = (*metamodelica::index_checked(&__ab_super_nodes, i.clone())?).clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        let mut sorted_body_components: metamodelica::List<metamodelica::List<i32>>;
        let mut sorted_body_indices: metamodelica::List<i32>;
        comp = ({
            let mut indep: bool = true;
            (::match_deref::match_deref! { match &(&*node_comp) {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ SINGLE { .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
                    StrongComponent::createPseudoScalar(&comp_indices, matching.eqn_to_var.clone(), mapping, vars, eqns)?
                },
                Deref @ metamodelica::ListNode::Cons { head: node @ Deref @ ALGEBRAIC_LOOP { .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
                    StrongComponent::createPseudoScalar(var_field!((**node).eqn_indices, SuperNode::ALGEBRAIC_LOOP), matching.eqn_to_var.clone(), mapping, vars, eqns)?
                },
                Deref @ metamodelica::ListNode::Cons { head: node @ Deref @ ARRAY_BUCKET { .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
                    let mut m_local: metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>;
                    let mut matching_local: metamodelica::Ref<Matching::NBMatching>;
                    let mut map_back: metamodelica::Array<i32>;
                    let mut eqn_arr_idx: i32;
                    let mut var_arr_idx: i32;
                    (m_local, matching_local, map_back) = getLocalSystem(m, matching, var_field!((**node).eqn_indices, SuperNode::ARRAY_BUCKET), var_loc.clone())?;
                    sorted_body_components = tarjanScalar(&m_local, &matching_local)?;
                    sorted_body_indices = mapFlatten(&sorted_body_components, map_back.clone())?;
                    if List::compareLength(sorted_body_components, sorted_body_indices.clone())? != 0 {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBSorting.SuperNode.collapse")); __mm_s.push_str(&*literal!(" crucially failed for the following Phase II strong component")); __mm_s.push_str(&*literal!(" because the body turned out to still have strong components:\n")); __mm_s.push_str(&*List::toString(node_comp.clone(), &move |__a0: metamodelica::Ref<SuperNode>| toString(&__a0), List::Style::NEWLINE_TAB.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }])?;
                    }
                    indep = Array::all(m_local.len.clone(), &({ let __pe_b1 = 1; move |__pe_a0| Ok(intEq(__pe_a0, __pe_b1.clone())) }))?;
                    eqn_arr_idx = ({let __elt = (*metamodelica::index_checked(&mapping.eqn_StA.borrow(), (var_field!((**node).eqn_indices, SuperNode::ARRAY_BUCKET)).head().cloned()?)?).clone(); __elt});
                    var_arr_idx = ({let __elt = (*metamodelica::index_checked(&mapping.var_StA.borrow(), ({let __elt = (*metamodelica::index_checked(&matching.eqn_to_var.borrow(), (var_field!((**node).eqn_indices, SuperNode::ARRAY_BUCKET)).head().cloned()?)?).clone(); __elt}))?).clone(); __elt});
                    StrongComponent::createPseudoSlice(var_arr_idx, eqn_arr_idx, var_field!((**node).cref_to_solve, SuperNode::ARRAY_BUCKET).clone(), sorted_body_indices, matching.eqn_to_var.clone(), eqns, mapping, indep)?
                },
                _ if (List::any(&node_comp, &move |__a0: metamodelica::Ref<SuperNode>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isArrayBucket(&__a0)) })?) => {
                    let mut m_local: metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>;
                    let mut matching_local: metamodelica::Ref<Matching::NBMatching>;
                    let mut map_back: metamodelica::Array<i32>;
                    (m_local, matching_local, map_back) = getLocalSystem(m, matching, &(List::flatten(({
                let mut __acc: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
                for mut n in (node_comp.clone()).into_iter().cloned() {
                    let __x = getEqnIndices(&(n.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }))?), var_loc.clone())?;
                    sorted_body_components = tarjanScalar(&m_local, &matching_local)?;
                    sorted_body_indices = mapFlatten(&sorted_body_components, map_back.clone())?;
                    comp = StrongComponent::createPseudoEntwined(sorted_body_indices, matching.eqn_to_var.clone(), mapping, vars, eqns, &node_comp)?;
                    comp
                },
                _ => {
                    sorted_body_indices = List::flatten(({
                let mut __acc: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
                for mut n in (node_comp.clone()).into_iter().cloned() {
                    let __x = getEqnIndices(&(n.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }))?;
                    StrongComponent::createPseudoScalar(&sorted_body_indices, matching.eqn_to_var.clone(), mapping, vars, eqns)?
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        });
        Ok(comp)
    }

    fn mapFlatten(
        mut components: &metamodelica::List<metamodelica::List<i32>>,
        mut map_back: metamodelica::Array<i32>,
    ) -> Result<metamodelica::List<i32>> {
        let __ab_map_back = map_back.borrow();
        let mut indices: metamodelica::List<i32> = metamodelica::nil();
        for mut comp in &**components {
            for mut i in &*comp.clone() {
                indices = metamodelica::cons(
                    (*metamodelica::index_checked(&__ab_map_back, i.clone())?).clone(),
                    indices,
                );
            }
        }
        indices = metamodelica::Dangerous::listReverseInPlace(indices);
        Ok(indices)
    }

    fn mergedSize(
        mut m: &metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
        mut rows: &metamodelica::List<metamodelica::List<i32>>,
    ) -> Result<i32> {
        let mut total: i32 = 0;
        for mut row in &**rows {
            for mut idx in &*row.clone() {
                total = total
                    + ({
                        let __elt = (*metamodelica::index_checked(&m.len.borrow(), idx.clone())?).clone();
                        __elt
                    });
            }
        }
        Ok(total)
    }

    fn maxMergedRow(
        mut m: &metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
        mut rows: &metamodelica::List<metamodelica::List<i32>>,
    ) -> Result<i32> {
        let mut mx: i32 = 0;
        let mut total: i32;
        for mut row in &**rows {
            total = 0;
            for mut idx in &*row.clone() {
                total = total
                    + ({
                        let __elt = (*metamodelica::index_checked(&m.len.borrow(), idx.clone())?).clone();
                        __elt
                    });
            }
            mx = intMax(mx, total);
        }
        Ok(mx)
    }

    fn mergeRows(
        mut m: &metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
        mut matching: metamodelica::Array<i32>,
        mut super_nodes: metamodelica::Array<metamodelica::Ref<SuperNode>>,
        mut rows_to_merge: &metamodelica::List<i32>,
        mut new_idx: i32,
        mut stamp: metamodelica::Array<i32>,
        mut counts: metamodelica::Array<i32>,
        mut uniq: metamodelica::Array<i32>,
        mut sorted: metamodelica::Array<i32>,
    ) -> Result<i32> {
        let mut new_idx: i32 = new_idx;
        let mut data: metamodelica::Array<i32> = Adjacency::IntMatrix::entries(m);
        let mut total: i32 = 0;
        let mut first: i32;
        let mut n: i32 = 0;
        let mut p: i32;
        let mut h: i32;
        let mut v: i32;
        let mut acc: i32;
        let mut c: i32;
        for mut idx in &**rows_to_merge {
            total = total
                + ({
                    let __elt = (*metamodelica::index_checked(&m.len.borrow(), idx.clone())?).clone();
                    __elt
                });
        }
        p = Util::nextPrime(total);
        for mut idx in &**rows_to_merge {
            first = ({
                let __elt = (*metamodelica::index_checked(&m.start.borrow(), idx.clone())?).clone();
                __elt
            });
            let __range0 = first
                ..=first
                    + ({
                        let __elt = (*metamodelica::index_checked(&m.len.borrow(), idx.clone())?).clone();
                        __elt
                    })
                    - 1;
            for mut k in __range0 {
                v = ({
                    let __elt = (*metamodelica::index_checked(&data.borrow(), k)?).clone();
                    __elt
                });
                if ({
                    let __elt = (*metamodelica::index_checked(&stamp.borrow(), v)?).clone();
                    __elt
                }) == 0
                {
                    metamodelica::arrayUpdate(stamp.clone(), v, 1)?;
                    n = n + 1;
                    metamodelica::arrayUpdate(uniq.clone(), n, v)?;
                }
            }
        }
        for mut i in 1..=p {
            metamodelica::arrayUpdate(counts.clone(), i, 0)?;
        }
        for mut i in 1..=n {
            h = intMod(
                ({
                    let __elt = (*metamodelica::index_checked(&uniq.borrow(), i)?).clone();
                    __elt
                }),
                p,
            ) + 1;
            metamodelica::arrayUpdate(
                counts.clone(),
                h,
                ({
                    let __elt = (*metamodelica::index_checked(&counts.borrow(), h)?).clone();
                    __elt
                }) + 1,
            )?;
        }
        acc = 0;
        for mut i in ({
            let __s = p;
            let __e = 1;
            (0i32..)
                .map(move |__k| __s + __k * (-1))
                .take_while(move |&__v| __v >= __e)
        }) {
            c = ({
                let __elt = (*metamodelica::index_checked(&counts.borrow(), i)?).clone();
                __elt
            });
            metamodelica::arrayUpdate(counts.clone(), i, acc)?;
            acc = acc + c;
        }
        for mut i in 1..=n {
            v = ({
                let __elt = (*metamodelica::index_checked(&uniq.borrow(), i)?).clone();
                __elt
            });
            h = intMod(v, p) + 1;
            metamodelica::arrayUpdate(
                sorted.clone(),
                ({
                    let __elt = (*metamodelica::index_checked(&counts.borrow(), h)?).clone();
                    __elt
                }) + 1,
                v,
            )?;
            metamodelica::arrayUpdate(
                counts.clone(),
                h,
                ({
                    let __elt = (*metamodelica::index_checked(&counts.borrow(), h)?).clone();
                    __elt
                }) + 1,
            )?;
            metamodelica::arrayUpdate(stamp.clone(), v, 0)?;
        }
        Adjacency::IntMatrix::setRowFromArray(m, new_idx, sorted.clone(), n)?;
        for mut idx in &**rows_to_merge {
            Adjacency::IntMatrix::clearRow(m, idx.clone())?;
            metamodelica::arrayUpdate(matching.clone(), idx.clone(), -1)?;
        }
        new_idx = new_idx + 1;
        Ok(new_idx)
    }

    fn mergeArrayNodes(
        mut super_nodes: metamodelica::Array<metamodelica::Ref<SuperNode>>,
        mut cref_to_solve: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut rows_to_merge: metamodelica::List<i32>,
        mut new_idx: i32,
        mut arr_idx: i32,
        mut update_scalar: bool,
    ) -> Result<i32> {
        let mut new_idx: i32 = new_idx;
        metamodelica::arrayUpdate(
            super_nodes.clone(),
            new_idx,
            metamodelica::Ref::new(SuperNode::ARRAY_BUCKET {
                index: new_idx,
                cref_to_solve: cref_to_solve,
                eqn_indices: rows_to_merge.clone(),
                arr_idx: arr_idx,
            }),
        )?;
        if update_scalar {
            for mut i in &*rows_to_merge {
                metamodelica::arrayUpdate(
                    super_nodes.clone(),
                    i.clone(),
                    metamodelica::Ref::new(SuperNode::ELEMENT {
                        index: i.clone(),
                        parent: new_idx,
                    }),
                )?;
            }
        }
        Ok(new_idx)
    }

    fn mergeLoopNodes(
        mut super_nodes: metamodelica::Array<metamodelica::Ref<SuperNode>>,
        mut rows_to_merge: metamodelica::List<i32>,
        mut new_idx: i32,
        mut update_scalar: bool,
    ) -> Result<i32> {
        let mut new_idx: i32 = new_idx;
        metamodelica::arrayUpdate(
            super_nodes.clone(),
            new_idx,
            metamodelica::Ref::new(SuperNode::ALGEBRAIC_LOOP {
                index: new_idx,
                eqn_indices: rows_to_merge.clone(),
            }),
        )?;
        if update_scalar {
            for mut i in &*rows_to_merge {
                metamodelica::arrayUpdate(
                    super_nodes.clone(),
                    i.clone(),
                    metamodelica::Ref::new(SuperNode::ELEMENT {
                        index: i.clone(),
                        parent: new_idx,
                    }),
                )?;
            }
        }
        Ok(new_idx)
    }
}

// ############################################################
//                Protected Functions and Types
// ############################################################
fn getLocalSystem(
    mut m: &metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
    mut matching: &metamodelica::Ref<Matching::NBMatching>,
    mut eqn_indices: &metamodelica::List<i32>,
    mut var_loc: metamodelica::Array<i32>,
) -> Result<(
    metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
    metamodelica::Ref<Matching::NBMatching>,
    metamodelica::Array<i32>,
)> {
    let mut m_loc: metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>;
    let mut matching_loc: metamodelica::Ref<Matching::NBMatching>;
    let mut map_back: metamodelica::Array<i32>;
    let N: i32 = ((eqn_indices).len() as i32);
    let mut var_to_eqn: metamodelica::Array<i32> = arrayCreate(N, -1);
    let mut eqn_to_var: metamodelica::Array<i32> = arrayCreate(N, -1);
    let mut data: metamodelica::Array<i32> = Adjacency::IntMatrix::entries(m);
    let mut builder: Adjacency::IntMatrix::Builder;
    let mut j: i32 = 1;
    let mut row: i32;
    let mut first: i32;
    let mut edges: i32 = 0;
    let mut loc: i32;
    let mut var: i32;
    map_back = arrayCreate(N, -1);
    for mut i in &**eqn_indices {
        {
            let __cell0 = i.clone();
            let __idx0 = j;
            *metamodelica::index_mut_checked(&mut map_back.clone().borrow_mut(), __idx0)? = __cell0;
        }
        var = ({
            let __elt = (*metamodelica::index_checked(&matching.eqn_to_var.borrow(), i.clone())?).clone();
            __elt
        });
        if var > 0 {
            metamodelica::arrayUpdate(var_loc.clone(), var, j)?;
        }
        {
            let __cell1 = j;
            let __idx1 = j;
            *metamodelica::index_mut_checked(&mut eqn_to_var.clone().borrow_mut(), __idx1)? = __cell1;
        }
        {
            let __cell2 = j;
            let __idx2 = j;
            *metamodelica::index_mut_checked(&mut var_to_eqn.clone().borrow_mut(), __idx2)? = __cell2;
        }
        j = j + 1;
    }
    matching_loc = metamodelica::Ref::new(Matching::NBMatching {
        var_to_eqn: var_to_eqn.clone(),
        eqn_to_var: eqn_to_var.clone(),
    });
    for mut j in 1..=N {
        edges = edges
            + ({
                let __elt = (*metamodelica::index_checked(
                    &m.len.borrow(),
                    ({
                        let __elt = (*metamodelica::index_checked(&map_back.borrow(), j)?).clone();
                        __elt
                    }),
                )?)
                .clone();
                __elt
            });
    }
    builder = Adjacency::IntMatrix::newBuilder(edges, false);
    for mut j in 1..=N {
        row = ({
            let __elt = (*metamodelica::index_checked(&map_back.borrow(), j)?).clone();
            __elt
        });
        first = ({
            let __elt = (*metamodelica::index_checked(&m.start.borrow(), row)?).clone();
            __elt
        });
        let __range3 = ({
            let __s = first
                + ({
                    let __elt = (*metamodelica::index_checked(&m.len.borrow(), row)?).clone();
                    __elt
                })
                - 1;
            let __e = first;
            (0i32..)
                .map(move |__k| __s + __k * (-1))
                .take_while(move |&__v| __v >= __e)
        });
        for mut k in __range3 {
            var = ({
                let __elt = (*metamodelica::index_checked(&data.borrow(), k)?).clone();
                __elt
            });
            loc = if (var > 0) {
                ({
                    let __elt = (*metamodelica::index_checked(&var_loc.borrow(), var)?).clone();
                    __elt
                })
            } else {
                0
            };
            if loc > 0 {
                Adjacency::IntMatrix::builderAdd(&builder, j, loc);
            }
        }
    }
    m_loc = Adjacency::IntMatrix::fromBuilder(&builder, N)?;
    for mut i in &**eqn_indices {
        var = ({
            let __elt = (*metamodelica::index_checked(&matching.eqn_to_var.borrow(), i.clone())?).clone();
            __elt
        });
        if var > 0 {
            metamodelica::arrayUpdate(var_loc.clone(), var, 0)?;
        }
    }
    Ok((m_loc, matching_loc, map_back))
}

fn strongConnect(
    mut m: &metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
    mut data: metamodelica::Array<i32>,
    mut var_to_eqn: metamodelica::Array<i32>,
    mut root: i32,
    mut stack: metamodelica::List<i32>,
    mut index: i32,
    mut number: metamodelica::Array<i32>,
    mut lowlink: metamodelica::Array<i32>,
    mut onStack: metamodelica::Array<bool>,
    mut call_eqn: metamodelica::Array<i32>,
    mut call_pos: metamodelica::Array<i32>,
    mut comps: metamodelica::List<metamodelica::List<i32>>,
) -> Result<(
    metamodelica::List<i32>,
    i32,
    metamodelica::List<metamodelica::List<i32>>,
)> {
    let __ab_data = data.borrow();
    let __ab_var_to_eqn = var_to_eqn.borrow();
    let mut stack: metamodelica::List<i32> = stack;
    let mut index: i32 = index;
    let mut comps: metamodelica::List<metamodelica::List<i32>> = comps;
    let mut SCC: metamodelica::List<i32>;
    let mut depth: i32;
    let mut eqn: i32;
    let mut eqn2: i32;
    let mut k: i32;
    let mut cand: i32;
    depth = 1;
    (stack, index) = strongConnectVisit(
        m,
        root,
        depth,
        stack,
        index,
        number.clone(),
        lowlink.clone(),
        onStack.clone(),
        call_eqn.clone(),
        call_pos.clone(),
    )?;
    while depth > 0 {
        eqn = ({
            let __elt = (*metamodelica::index_checked(&call_eqn.borrow(), depth)?).clone();
            __elt
        });
        k = ({
            let __elt = (*metamodelica::index_checked(&call_pos.borrow(), depth)?).clone();
            __elt
        });
        if k < ({
            let __elt = (*metamodelica::index_checked(&m.start.borrow(), eqn)?).clone();
            __elt
        }) + ({
            let __elt = (*metamodelica::index_checked(&m.len.borrow(), eqn)?).clone();
            __elt
        }) {
            metamodelica::arrayUpdate(call_pos.clone(), depth, k + 1)?;
            cand = (*metamodelica::index_checked(&__ab_data, k)?).clone();
            if cand > 0 {
                eqn2 = (*metamodelica::index_checked(&__ab_var_to_eqn, cand)?).clone();
                if eqn2 > 0 && eqn2 != eqn {
                    if ({
                        let __elt = (*metamodelica::index_checked(&number.borrow(), eqn2)?).clone();
                        __elt
                    }) == -1
                    {
                        depth = depth + 1;
                        (stack, index) = strongConnectVisit(
                            m,
                            eqn2,
                            depth,
                            stack,
                            index,
                            number.clone(),
                            lowlink.clone(),
                            onStack.clone(),
                            call_eqn.clone(),
                            call_pos.clone(),
                        )?;
                    } else if ({
                        let __elt = (*metamodelica::index_checked(&onStack.borrow(), eqn2)?).clone();
                        __elt
                    }) {
                        metamodelica::arrayUpdate(
                            lowlink.clone(),
                            eqn,
                            intMin(
                                ({
                                    let __elt = (*metamodelica::index_checked(&lowlink.borrow(), eqn)?).clone();
                                    __elt
                                }),
                                ({
                                    let __elt = (*metamodelica::index_checked(&number.borrow(), eqn2)?).clone();
                                    __elt
                                }),
                            ),
                        )?;
                    }
                }
            }
        } else {
            if ({
                let __elt = (*metamodelica::index_checked(&lowlink.borrow(), eqn)?).clone();
                __elt
            }) == ({
                let __elt = (*metamodelica::index_checked(&number.borrow(), eqn)?).clone();
                __elt
            }) {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(stack) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                eqn2 = metamodelica::Own::own(__pa0);
                stack = metamodelica::Own::own(__pa1);
                metamodelica::arrayUpdate(onStack.clone(), eqn2, false)?;
                SCC = list![eqn2];
                while eqn != eqn2 {
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(stack) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    eqn2 = metamodelica::Own::own(__pa2);
                    stack = metamodelica::Own::own(__pa3);
                    metamodelica::arrayUpdate(onStack.clone(), eqn2, false)?;
                    SCC = metamodelica::cons(eqn2, SCC);
                }
                comps = metamodelica::cons(metamodelica::Dangerous::listReverseInPlace(SCC), comps);
            }
            depth = depth - 1;
            if depth > 0 {
                eqn2 = ({
                    let __elt = (*metamodelica::index_checked(&call_eqn.borrow(), depth)?).clone();
                    __elt
                });
                metamodelica::arrayUpdate(
                    lowlink.clone(),
                    eqn2,
                    intMin(
                        ({
                            let __elt = (*metamodelica::index_checked(&lowlink.borrow(), eqn2)?).clone();
                            __elt
                        }),
                        ({
                            let __elt = (*metamodelica::index_checked(&lowlink.borrow(), eqn)?).clone();
                            __elt
                        }),
                    ),
                )?;
            }
        }
    }
    Ok((stack, index, comps))
}

fn strongConnectVisit(
    mut m: &metamodelica::Ref<Adjacency::IntMatrix::IntMatrix>,
    mut eqn: i32,
    mut depth: i32,
    mut stack: metamodelica::List<i32>,
    mut index: i32,
    mut number: metamodelica::Array<i32>,
    mut lowlink: metamodelica::Array<i32>,
    mut onStack: metamodelica::Array<bool>,
    mut call_eqn: metamodelica::Array<i32>,
    mut call_pos: metamodelica::Array<i32>,
) -> Result<(metamodelica::List<i32>, i32)> {
    let mut stack: metamodelica::List<i32> = stack;
    let mut index: i32 = index;
    metamodelica::arrayUpdate(number.clone(), eqn, index)?;
    metamodelica::arrayUpdate(lowlink.clone(), eqn, index)?;
    metamodelica::arrayUpdate(onStack.clone(), eqn, true)?;
    index = index + 1;
    stack = metamodelica::cons(eqn, stack);
    metamodelica::arrayUpdate(call_eqn.clone(), depth, eqn)?;
    metamodelica::arrayUpdate(
        call_pos.clone(),
        depth,
        ({
            let __elt = (*metamodelica::index_checked(&m.start.borrow(), eqn)?).clone();
            __elt
        }),
    )?;
    Ok((stack, index))
}
